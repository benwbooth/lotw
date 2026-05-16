use lotw_port::rom::InesRom;
use lotw_port::sha256;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TraceEntry {
    cpu_addr: u16,
    prg_offset: u32,
    count: u32,
    first_frame: u32,
    role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlockCandidate {
    cpu_addr: u16,
    prg_offset: u32,
    byte_count: u16,
    hit_count: u32,
    first_frame: u32,
    first_opcode: u8,
    control_flow_ops: u16,
    terminator_opcode: u8,
    role: String,
    stop_reason: String,
}

pub fn run(
    rom_path: &Path,
    trace_path: &Path,
    out_dir: &Path,
    expected_sha256: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read(rom_path)?;
    let actual_sha256 = sha256::digest_hex(&bytes);
    if let Some(expected) = expected_sha256 {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "blocks: ROM hash mismatch: got {actual_sha256}, expected {}",
                expected.to_ascii_lowercase()
            )
            .into());
        }
    }

    let rom = InesRom::parse(&bytes)?;
    let mut entries = read_trace(trace_path)?;
    if entries.is_empty() {
        return Err("blocks: trace has no known PRG offsets".into());
    }
    entries.sort_by_key(|entry| (entry.prg_offset, entry.cpu_addr));

    let blocks = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let next_offset = entries
                .get(index + 1)
                .filter(|next| next.prg_offset > entry.prg_offset)
                .map(|next| next.prg_offset);
            make_block(&rom, entry, next_offset)
        })
        .collect::<io::Result<Vec<_>>>()?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;
    write_manifest(
        &out_dir.join("manifest.txt"),
        &actual_sha256,
        trace_path,
        blocks.len(),
    )?;
    write_blocks_tsv(&out_dir.join("block_candidates.tsv"), &blocks)?;

    println!("blocks: wrote {}", out_dir.display());
    println!(
        "blocks: {} candidates from {} trace entries",
        blocks.len(),
        entries.len()
    );
    Ok(())
}

fn read_trace(path: &Path) -> io::Result<Vec<TraceEntry>> {
    let text = fs::read_to_string(path)?;
    let mut entries = Vec::new();
    for line in text.lines().skip(1) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 4 || fields.get(1) == Some(&"unknown") {
            continue;
        }

        entries.push(TraceEntry {
            cpu_addr: parse_hex_u16(fields[0]),
            prg_offset: parse_hex_u32(fields[1]),
            count: parse_dec_u32(fields[2]),
            first_frame: parse_dec_u32(fields[3]),
            role: fields.get(4).copied().unwrap_or("").to_string(),
        });
    }
    Ok(entries)
}

fn parse_hex_u16(value: &str) -> u16 {
    u16::from_str_radix(value, 16).unwrap_or(0)
}

fn parse_hex_u32(value: &str) -> u32 {
    u32::from_str_radix(value, 16).unwrap_or(0)
}

fn parse_dec_u32(value: &str) -> u32 {
    value.parse::<u32>().unwrap_or(0)
}

fn make_block(
    rom: &InesRom,
    entry: &TraceEntry,
    next_offset: Option<u32>,
) -> io::Result<BlockCandidate> {
    let prg = rom.prg_rom();
    let off = usize::try_from(entry.prg_offset).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("blocks: PRG offset is too large: {:05X}", entry.prg_offset),
        )
    })?;
    if off >= prg.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "blocks: PRG offset is outside ROM: {:05X} >= {:05X}",
                entry.prg_offset,
                prg.len()
            ),
        ));
    }

    let mut end = off;
    let mut limit = off.saturating_add(128);
    if let Some(next_offset) = next_offset {
        if next_offset > entry.prg_offset
            && usize::try_from(next_offset).unwrap_or(usize::MAX) < limit
        {
            limit = usize::try_from(next_offset).unwrap_or(limit);
        }
    }
    limit = limit.min(prg.len());

    let mut block = BlockCandidate {
        cpu_addr: entry.cpu_addr,
        prg_offset: entry.prg_offset,
        byte_count: 0,
        hit_count: entry.count,
        first_frame: entry.first_frame,
        first_opcode: prg[off],
        control_flow_ops: 0,
        terminator_opcode: 0,
        role: entry.role.clone(),
        stop_reason: "byte_limit".to_string(),
    };

    while end < limit {
        let opcode = prg[end];
        let len = opcode_len(opcode);

        if is_control_flow_opcode(opcode) {
            block.control_flow_ops = block.control_flow_ops.saturating_add(1);
        }

        if end.saturating_add(len) > prg.len() {
            block.stop_reason = "end_of_prg".to_string();
            break;
        }

        end += len;

        if is_stop_opcode(opcode) {
            block.terminator_opcode = opcode;
            block.stop_reason = format!("terminator_{opcode:02X}");
            break;
        }

        if let Some(next_offset) = next_offset {
            if next_offset > entry.prg_offset && end >= usize::try_from(next_offset).unwrap_or(end)
            {
                block.stop_reason = "next_trace_label".to_string();
                break;
            }
        }
    }

    if end <= off {
        end = off + 1;
    }
    let byte_count = end.saturating_sub(off).min(usize::from(u16::MAX));
    block.byte_count = u16::try_from(byte_count).expect("byte_count clamped to u16");
    Ok(block)
}

fn opcode_len(opcode: u8) -> usize {
    match opcode {
        0x00 | 0x08 | 0x0A | 0x18 | 0x28 | 0x2A | 0x38 | 0x40 | 0x48 | 0x4A | 0x58 | 0x60
        | 0x68 | 0x6A | 0x78 | 0x88 | 0x8A | 0x98 | 0x9A | 0xA8 | 0xAA | 0xB8 | 0xBA | 0xC8
        | 0xCA | 0xD8 | 0xE8 | 0xEA | 0xF8 => 1,
        0x01 | 0x05 | 0x06 | 0x09 | 0x10 | 0x11 | 0x15 | 0x16 | 0x21 | 0x24 | 0x25 | 0x26
        | 0x29 | 0x30 | 0x31 | 0x35 | 0x36 | 0x41 | 0x45 | 0x46 | 0x49 | 0x50 | 0x51 | 0x55
        | 0x56 | 0x61 | 0x65 | 0x66 | 0x69 | 0x70 | 0x71 | 0x75 | 0x76 | 0x81 | 0x84 | 0x85
        | 0x86 | 0x90 | 0x91 | 0x94 | 0x95 | 0x96 | 0xA0 | 0xA1 | 0xA2 | 0xA4 | 0xA5 | 0xA6
        | 0xA9 | 0xB0 | 0xB1 | 0xB4 | 0xB5 | 0xB6 | 0xC0 | 0xC1 | 0xC4 | 0xC5 | 0xC6 | 0xC9
        | 0xD0 | 0xD1 | 0xD5 | 0xD6 | 0xE0 | 0xE1 | 0xE4 | 0xE5 | 0xE6 | 0xE9 | 0xF0 | 0xF1
        | 0xF5 | 0xF6 => 2,
        0x0D | 0x0E | 0x19 | 0x1D | 0x1E | 0x20 | 0x2C | 0x2D | 0x2E | 0x39 | 0x3D | 0x3E
        | 0x4C | 0x4D | 0x4E | 0x59 | 0x5D | 0x5E | 0x6C | 0x6D | 0x6E | 0x79 | 0x7D | 0x7E
        | 0x8C | 0x8D | 0x8E | 0x99 | 0x9D | 0xAC | 0xAD | 0xAE | 0xB9 | 0xBC | 0xBD | 0xBE
        | 0xCC | 0xCD | 0xCE | 0xD9 | 0xDD | 0xDE | 0xEC | 0xED | 0xEE | 0xF9 | 0xFD | 0xFE => 3,
        _ => 1,
    }
}

fn is_stop_opcode(opcode: u8) -> bool {
    matches!(opcode, 0x00 | 0x40 | 0x4C | 0x60 | 0x6C)
}

fn is_branch_opcode(opcode: u8) -> bool {
    matches!(
        opcode,
        0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xB0 | 0xD0 | 0xF0
    )
}

fn is_control_flow_opcode(opcode: u8) -> bool {
    opcode == 0x20 || is_stop_opcode(opcode) || is_branch_opcode(opcode)
}

fn write_manifest(
    path: &Path,
    sha256_hex: &str,
    trace_path: &Path,
    block_count: usize,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "sha256={sha256_hex}")?;
    writeln!(file, "trace={}", trace_path.display())?;
    writeln!(file, "block_count={block_count}")?;
    writeln!(file, "scope=executed label basic-block candidates")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_blocks_tsv(path: &Path, blocks: &[BlockCandidate]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "id\tcpu_addr\tprg_offset\tbytes\thit_count\tfirst_frame\tfirst_opcode\tstop_reason\trole\tcontrol_flow_ops\tterminator_opcode"
    )?;
    for (index, block) in blocks.iter().enumerate() {
        writeln!(
            file,
            "{index}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{:02X}\t{}\t{}\t{}\t{:02X}",
            block.cpu_addr,
            block.prg_offset,
            block.byte_count,
            block.hit_count,
            block.first_frame,
            block.first_opcode,
            block.stop_reason,
            block.role,
            block.control_flow_ops,
            block.terminator_opcode
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::unique_temp_dir;

    const PRG_LEN: usize = 0x8000;
    const CHR_LEN: usize = 0x2000;

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + PRG_LEN + CHR_LEN];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        let prg = 16;
        bytes[prg..prg + PRG_LEN].fill(0xEA);
        bytes[prg] = 0x20;
        bytes[prg + 1] = 0x10;
        bytes[prg + 2] = 0x80;
        bytes[prg + 3] = 0xA9;
        bytes[prg + 4] = 0x01;
        bytes[prg + 5] = 0x60;
        bytes[prg + 0x10] = 0x4C;
        bytes[prg + 0x11] = 0x00;
        bytes[prg + 0x12] = 0x80;
        bytes[prg + 0x20] = 0xD0;
        bytes[prg + 0x21] = 0xFC;
        bytes
    }

    #[test]
    fn writes_block_candidates_without_generated_c() {
        let root = unique_temp_dir("blocks");
        let rom = root.join("fixture.nes");
        let trace = root.join("executed_labels.tsv");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::write(&rom, bytes).unwrap();
        fs::write(
            &trace,
            "cpu_addr\tprg_offset\tcount\tfirst_frame\trole\n\
             8003\t00003\t2\t1\timm\n\
             8010\t00010\t5\t2\tsub\n\
             8000\t00000\t1\t1\tentry\n\
             8020\t00020\t7\t9\t\n\
             9000\tunknown\t1\t1\tignored\n",
        )
        .unwrap();
        fs::create_dir_all(&out).unwrap();
        fs::write(out.join("lotw_generated_blocks.c"), "stale").unwrap();

        run(&rom, &trace, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("block_count=4\n"));
        assert!(manifest.contains("complete=1\n"));
        let tsv = fs::read_to_string(out.join("block_candidates.tsv")).unwrap();
        assert!(tsv.contains("0\t8000\t00000\t3\t1\t1\t20\tnext_trace_label\tentry\t1\t00\n"));
        assert!(tsv.contains("1\t8003\t00003\t3\t2\t1\tA9\tterminator_60\timm\t1\t60\n"));
        assert!(tsv.contains("2\t8010\t00010\t3\t5\t2\t4C\tterminator_4C\tsub\t1\t4C\n"));
        assert!(tsv.contains("3\t8020\t00020\t128\t7\t9\tD0\tbyte_limit\t\t1\t00\n"));
        assert!(!out.join("lotw_generated_blocks.c").exists());

        fs::remove_dir_all(root).unwrap();
    }
}
