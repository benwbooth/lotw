use crate::block_exec;
use lotw_port::rom::InesRom;
use lotw_port::sha256;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const CASE_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tbytes\tfirst_frame\thit_ordinal\tpc\ta\tx\ty\tp\ts\tram_0000_07ff";
const BLOCK_HEADER: &str = "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason";
const VERIFY_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\texecuted\tmetadata_match\tregister_match\tcycles_match\tram_match\texternal_write_match\texpected_external_writes\tactual_external_writes\tmatch";
const FINAL_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tcycles\tram_0000_07ff\tfinal_ram_sha256";
const VERIFY_CASE_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes";

#[derive(Debug, Clone)]
struct PlanRow {
    plan_rank: String,
    cpu_addr: u16,
    prg_offset: usize,
    label: String,
    first_opcode: u8,
    file: PathBuf,
}

#[derive(Debug, Clone)]
struct Block {
    native_index: usize,
    plan: PlanRow,
    byte_count: u16,
}

#[derive(Debug, Clone)]
struct Case {
    replay: &'static str,
    native_index: usize,
    cpu_addr: u16,
    prg_offset: usize,
    byte_count: u16,
    first_frame: u32,
    hit_ordinal: u32,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    ram: [u8; 0x800],
}

#[derive(Debug, Clone, Default)]
struct ExternalWrite {
    kind: String,
    addr: String,
    value: String,
}

#[derive(Debug, Clone)]
struct OracleRow {
    replay: String,
    native_index: usize,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_frame: String,
    hit_ordinal: u32,
    status: String,
    final_pc: String,
    cycles: String,
    writes: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    unmapped_reads: u64,
    final_a: String,
    final_x: String,
    final_y: String,
    final_p: String,
    final_s: String,
    final_ram_sha256: String,
}

#[derive(Debug, Clone)]
struct NativeResult {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    cycles: u64,
    ram: [u8; 0x800],
    external_writes: Vec<ExternalWrite>,
    unsupported_opcode: Option<u8>,
}

#[derive(Debug, Default)]
struct Summary {
    selected_count: u64,
    synthetic_case_count: u64,
    matched: u64,
    mismatches: u64,
    external_write_matched: u64,
    external_write_mismatches: u64,
    skipped_unsupported: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    rom_path: &Path,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if limit == 0 {
        return Err("static_handoff_verify: limit must be positive".into());
    }
    let plan_path = build_dir.join("static_handoff_plan/static_handoff_plan.tsv");
    require_file(&plan_path)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir.join("oracle"))?;
    fs::create_dir_all(out_dir.join("native_verify"))?;

    let rom = InesRom::parse(&fs::read(rom_path)?)?;
    let existing = read_existing_block_keys(build_dir)?;
    let blocks = select_blocks(&plan_path, limit, &existing)?;
    let cases = build_cases(&blocks);
    write_cases(&out_dir.join("static_handoff_state_cases.tsv"), &cases)?;

    if cases.is_empty() {
        write_empty_outputs(out_dir)?;
        return Ok(());
    }

    block_exec::run_case_states(
        rom_path,
        &out_dir.join("static_handoff_state_cases.tsv"),
        &out_dir.join("oracle"),
        64,
    )?;

    let oracle_rows = read_oracle_rows(&out_dir.join("oracle/block_state_exec.tsv"))?;
    let external_writes =
        read_external_writes(&out_dir.join("oracle/block_state_external_writes.tsv"))?;
    let mut summary = Summary {
        selected_count: blocks.len() as u64,
        synthetic_case_count: cases.len() as u64,
        ..Summary::default()
    };

    let native_results = run_native_cases(&rom, &cases);
    write_native_verify(
        out_dir,
        &cases,
        &native_results,
        &oracle_rows,
        &external_writes,
        &mut summary,
    )?;
    write_verified_blocks(
        &out_dir.join("static_handoff_native_blocks.tsv"),
        &blocks,
        &oracle_rows,
        &native_results,
    )?;
    write_skipped(
        &out_dir.join("static_handoff_skipped.tsv"),
        &blocks,
        &native_results,
        &mut summary,
    )?;
    write_summary(out_dir, &summary)?;
    write_manifest(out_dir, &summary)?;

    if summary.mismatches != 0 || summary.external_write_mismatches != 0 {
        return Err("static_handoff_verify: native verification mismatches remain".into());
    }
    if summary.skipped_unsupported != 0 {
        return Err("static_handoff_verify: unsupported native opcodes remain".into());
    }

    println!("static_handoff_verify: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_handoff_verify: missing input: {}", path.display()),
        ))
    }
}

fn read_existing_block_keys(build_dir: &Path) -> io::Result<HashSet<String>> {
    let path = build_dir.join("static_handoff_verify/static_handoff_native_blocks.tsv");
    if !path.is_file() {
        return Ok(HashSet::new());
    }

    let text = fs::read_to_string(&path)?;
    let mut keys = HashSet::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 3 {
            return invalid_tsv(&path, line_no + 1, fields.len(), 3);
        }
        keys.insert(block_key(fields[1], fields[2]));
    }
    Ok(keys)
}

fn block_key(cpu_addr: &str, prg_offset: &str) -> String {
    format!(
        "{}\t{}",
        cpu_addr.to_ascii_uppercase(),
        prg_offset.to_ascii_uppercase()
    )
}

fn select_blocks(path: &Path, limit: usize, existing: &HashSet<String>) -> io::Result<Vec<Block>> {
    let mut rows = Vec::new();
    let text = fs::read_to_string(path)?;
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 20 {
            return invalid_tsv(path, line_no + 1, fields.len(), 20);
        }
        let next_action = fields[12].to_string();
        if next_action != "generate_linear_handoff_case" {
            continue;
        }
        if existing.contains(&block_key(fields[5], fields[6])) {
            continue;
        }
        let row = PlanRow {
            plan_rank: fields[0].to_string(),
            cpu_addr: parse_hex_u16(fields[5])
                .ok_or_else(|| invalid_data(path, line_no + 1, "cpu_addr"))?,
            prg_offset: parse_hex_usize(fields[6])
                .ok_or_else(|| invalid_data(path, line_no + 1, "prg_offset"))?,
            label: fields[7].to_string(),
            first_opcode: parse_hex_u8(fields[8])
                .ok_or_else(|| invalid_data(path, line_no + 1, "first_opcode"))?,
            file: PathBuf::from(fields[19]),
        };
        let byte_count = disasm_label_byte_count(&row.file, &row.label)? as u16;
        rows.push(Block {
            native_index: rows.len(),
            plan: row,
            byte_count,
        });
        if rows.len() >= limit {
            break;
        }
    }
    Ok(rows)
}

fn invalid_tsv<T>(path: &Path, line_no: usize, actual: usize, expected: usize) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "{}:{line_no} has {actual} fields, expected at least {expected}",
            path.display()
        ),
    ))
}

fn invalid_data(path: &Path, line_no: usize, field: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{}:{line_no} invalid {field}", path.display()),
    )
}

fn disasm_label_byte_count(path: &Path, label: &str) -> io::Result<usize> {
    let text = fs::read_to_string(path)?;
    let mut in_label = false;
    let mut bytes = 0usize;
    for line in text.lines() {
        if line.trim_end() == format!("{label}:") {
            in_label = true;
            continue;
        }
        if !in_label {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("L_") && trimmed.ends_with(':') {
            break;
        }
        if let Some(count) = disasm_instruction_byte_count(trimmed) {
            bytes += count;
        }
    }
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "static_handoff_verify: no disassembly bytes for {label} in {}",
                path.display()
            ),
        ));
    }
    Ok(bytes)
}

fn disasm_instruction_byte_count(line: &str) -> Option<usize> {
    let rest = line.strip_prefix(';')?.trim();
    let mut fields = rest.split_whitespace();
    let _addr = fields.next()?;
    let mut count = 0usize;
    for field in fields {
        if field.len() == 2 && u8::from_str_radix(field, 16).is_ok() {
            count += 1;
        } else {
            break;
        }
    }
    (count > 0).then_some(count)
}

fn build_cases(blocks: &[Block]) -> Vec<Case> {
    let regs = [
        (0x55, 0x12, 0x00, 0x24, 0xfb),
        (0x00, 0x00, 0x01, 0x25, 0xfb),
        (0xff, 0x80, 0x01, 0xa4, 0xfb),
        (0x30, 0x7f, 0x01, 0x65, 0xfb),
    ];
    let mut cases = Vec::new();
    for block in blocks {
        for (case_index, (a, x, y, p, s)) in regs.iter().copied().enumerate() {
            cases.push(Case {
                replay: "static_handoff",
                native_index: block.native_index,
                cpu_addr: block.plan.cpu_addr,
                prg_offset: block.plan.prg_offset,
                byte_count: block.byte_count,
                first_frame: 0,
                hit_ordinal: (case_index + 1) as u32,
                pc: block.plan.cpu_addr,
                a,
                x,
                y,
                p,
                s,
                ram: synthetic_ram(block.native_index, case_index),
            });
        }
    }
    cases
}

fn synthetic_ram(block_index: usize, case_index: usize) -> [u8; 0x800] {
    let mut ram = [0u8; 0x800];
    let seed = (block_index as u32 * 17) + (case_index as u32 * 29);
    for (index, byte) in ram.iter_mut().enumerate() {
        *byte = ((index as u32 * 19 + seed + 0x2a) & 0xff) as u8;
    }
    ram
}

fn write_cases(path: &Path, cases: &[Case]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{CASE_HEADER}")?;
    for case in cases {
        writeln!(
            file,
            "{}\t{}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{:04X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}",
            case.replay,
            case.native_index,
            case.cpu_addr,
            case.prg_offset,
            case.byte_count,
            case.first_frame,
            case.hit_ordinal,
            case.pc,
            case.a,
            case.x,
            case.y,
            case.p,
            case.s,
            hex_bytes(&case.ram)
        )?;
    }
    Ok(())
}

fn run_native_cases(rom: &InesRom, cases: &[Case]) -> Vec<NativeResult> {
    cases
        .iter()
        .map(|case| run_native_case(rom.prg_rom(), case))
        .collect()
}

fn run_native_case(prg: &[u8], case: &Case) -> NativeResult {
    let mut result = NativeResult {
        pc: case.pc,
        a: case.a,
        x: case.x,
        y: case.y,
        p: case.p,
        s: case.s,
        cycles: 0,
        ram: case.ram,
        external_writes: Vec::new(),
        unsupported_opcode: None,
    };
    while in_block_range(case.cpu_addr, case.byte_count, result.pc) {
        let offset = case.prg_offset + usize::from(result.pc - case.cpu_addr);
        if offset >= prg.len() {
            result.unsupported_opcode = Some(0xff);
            break;
        }
        let opcode = prg[offset];
        match opcode {
            0xa5 => {
                let zp = read_operand_u8(prg, offset + 1, &mut result) as u16;
                result.a = read_native(&result, zp);
                set_nz(&mut result.p, result.a);
                result.pc = result.pc.wrapping_add(2);
                result.cycles += 3;
            }
            0xad => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                result.a = read_native(&result, addr);
                set_nz(&mut result.p, result.a);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 4;
            }
            0xae => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                result.x = read_native(&result, addr);
                set_nz(&mut result.p, result.x);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 4;
            }
            0x8c => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                let value = result.y;
                write_native(&mut result, addr, value);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 4;
            }
            0x8d => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                let value = result.a;
                write_native(&mut result, addr, value);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 4;
            }
            0x8e => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                let value = result.x;
                write_native(&mut result, addr, value);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 4;
            }
            0xce => {
                let addr = read_operand_u16(prg, offset + 1, &mut result);
                let value = read_native(&result, addr).wrapping_sub(1);
                write_native(&mut result, addr, value);
                set_nz(&mut result.p, value);
                result.pc = result.pc.wrapping_add(3);
                result.cycles += 6;
            }
            unsupported => {
                result.unsupported_opcode = Some(unsupported);
                break;
            }
        }
    }
    result
}

fn read_operand_u8(prg: &[u8], offset: usize, result: &mut NativeResult) -> u8 {
    match prg.get(offset) {
        Some(value) => *value,
        None => {
            result.unsupported_opcode = Some(0xff);
            0
        }
    }
}

fn read_operand_u16(prg: &[u8], offset: usize, result: &mut NativeResult) -> u16 {
    let lo = read_operand_u8(prg, offset, result);
    let hi = read_operand_u8(prg, offset + 1, result);
    u16::from_le_bytes([lo, hi])
}

fn in_block_range(cpu_addr: u16, byte_count: u16, pc: u16) -> bool {
    let start = u32::from(cpu_addr);
    let end = start + u32::from(byte_count);
    let pc = u32::from(pc);
    pc >= start && pc < end
}

fn read_native(result: &NativeResult, addr: u16) -> u8 {
    if addr < 0x2000 {
        result.ram[usize::from(addr & 0x07ff)]
    } else {
        0
    }
}

fn write_native(result: &mut NativeResult, addr: u16, value: u8) {
    if addr < 0x2000 {
        result.ram[usize::from(addr & 0x07ff)] = value;
    } else {
        result.external_writes.push(ExternalWrite {
            kind: external_write_kind(addr).to_string(),
            addr: format!("{addr:04X}"),
            value: format!("{value:02X}"),
        });
    }
}

fn external_write_kind(addr: u16) -> &'static str {
    if (0x2000..0x4000).contains(&addr) {
        "ppu"
    } else if (0x4000..=0x4017).contains(&addr) {
        "apu"
    } else if addr >= 0x8000 {
        "mapper"
    } else {
        "unknown"
    }
}

fn set_nz(p: &mut u8, value: u8) {
    *p &= !(0x02 | 0x80);
    if value == 0 {
        *p |= 0x02;
    }
    if value & 0x80 != 0 {
        *p |= 0x80;
    }
}

fn write_native_verify(
    out_dir: &Path,
    cases: &[Case],
    native_results: &[NativeResult],
    oracle_rows: &[OracleRow],
    oracle_external_writes: &HashMap<(usize, u32), Vec<ExternalWrite>>,
    summary: &mut Summary,
) -> io::Result<()> {
    let mut verify = fs::File::create(out_dir.join("native_verify/native_block_verify.tsv"))?;
    let mut final_states =
        fs::File::create(out_dir.join("native_verify/native_block_final_states.tsv"))?;
    let mut verify_cases =
        fs::File::create(out_dir.join("native_verify/static_handoff_native_verify_cases.tsv"))?;
    writeln!(verify, "{VERIFY_HEADER}")?;
    writeln!(final_states, "{FINAL_HEADER}")?;
    writeln!(verify_cases, "{VERIFY_CASE_HEADER}")?;

    if cases.len() != native_results.len() || cases.len() != oracle_rows.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "static_handoff_verify: case/native/oracle row count mismatch: cases={} native={} oracle={}",
                cases.len(),
                native_results.len(),
                oracle_rows.len()
            ),
        ));
    }

    for ((case, native), oracle) in cases.iter().zip(native_results).zip(oracle_rows) {
        let actual_hash = sha256::digest_hex(&native.ram);
        let expected_external = oracle_external_writes
            .get(&(case.native_index, case.hit_ordinal))
            .cloned()
            .unwrap_or_default();
        let actual_external = native.external_writes.clone();
        let expected_external_text = format_external_writes(&expected_external);
        let actual_external_text = format_external_writes(&actual_external);
        let executed = native.unsupported_opcode.is_none();
        let metadata_match = oracle.replay == case.replay
            && oracle.native_index == case.native_index
            && oracle.cpu_addr == format!("{:04X}", case.cpu_addr)
            && oracle.prg_offset == format!("{:05X}", case.prg_offset)
            && oracle.bytes == case.byte_count.to_string()
            && oracle.first_frame == case.first_frame.to_string()
            && oracle.hit_ordinal == case.hit_ordinal;
        let register_match = oracle.status == "left_block"
            && oracle.final_pc == format!("{:04X}", native.pc)
            && oracle.final_a == format!("{:02X}", native.a)
            && oracle.final_x == format!("{:02X}", native.x)
            && oracle.final_y == format!("{:02X}", native.y)
            && oracle.final_p == format!("{:02X}", native.p)
            && oracle.final_s == format!("{:02X}", native.s);
        let cycles_match = oracle.cycles == native.cycles.to_string();
        let ram_match = oracle.final_ram_sha256 == actual_hash;
        let external_write_match = expected_external_text == actual_external_text;
        let matched = executed
            && metadata_match
            && register_match
            && cycles_match
            && ram_match
            && external_write_match;

        if matched {
            summary.matched += 1;
        } else {
            summary.mismatches += 1;
        }
        if external_write_match {
            summary.external_write_matched += 1;
        } else {
            summary.external_write_mismatches += 1;
        }

        writeln!(
            verify,
            "{}\t{}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            case.replay,
            case.native_index,
            case.cpu_addr,
            case.prg_offset,
            bit(executed),
            bit(metadata_match),
            bit(register_match),
            bit(cycles_match),
            bit(ram_match),
            bit(external_write_match),
            expected_external_text,
            actual_external_text,
            bit(matched)
        )?;
        writeln!(
            final_states,
            "{}\t{}\t{:04X}\t{:05X}\t{}\t{:04X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}\t{}\t{}",
            case.replay,
            case.native_index,
            case.cpu_addr,
            case.prg_offset,
            case.first_frame,
            native.pc,
            native.a,
            native.x,
            native.y,
            native.p,
            native.s,
            native.cycles,
            hex_bytes(&native.ram),
            actual_hash
        )?;
        writeln!(
            verify_cases,
            "{}\t{}\t{:04X}\t{:05X}\t{}\t{:04X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}\t{:04X}\t{}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}\t{}",
            case.replay,
            case.native_index,
            case.cpu_addr,
            case.prg_offset,
            case.first_frame,
            case.pc,
            case.a,
            case.x,
            case.y,
            case.p,
            case.s,
            hex_bytes(&case.ram),
            native.pc,
            native.cycles,
            native.a,
            native.x,
            native.y,
            native.p,
            native.s,
            actual_hash,
            actual_external_text
        )?;
    }

    write_native_manifest(out_dir, summary)
}

fn write_verified_blocks(
    path: &Path,
    blocks: &[Block],
    oracle_rows: &[OracleRow],
    native_results: &[NativeResult],
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{BLOCK_HEADER}")?;
    for block in blocks {
        let mut observations = 0u64;
        let mut writes = 0u64;
        let mut ppu = 0u64;
        let mut apu = 0u64;
        let mut mapper = 0u64;
        let mut final_hashes = 0u64;
        let mut unsupported = false;
        for (oracle, native) in oracle_rows.iter().zip(native_results) {
            if oracle.native_index != block.native_index {
                continue;
            }
            observations += 1;
            writes += oracle.writes;
            ppu += oracle.ppu_writes;
            apu += oracle.apu_writes;
            mapper += oracle.mapper_writes;
            if !oracle.final_ram_sha256.is_empty() {
                final_hashes += 1;
            }
            unsupported |= native.unsupported_opcode.is_some();
        }
        if unsupported {
            continue;
        }
        let reason = if ppu + apu + mapper > 0 {
            "static_handoff_external_writes"
        } else if writes > 0 {
            "static_handoff_linear_ram_writes"
        } else {
            "static_handoff_linear"
        };
        writeln!(
            file,
            "{}\t{:04X}\t{:05X}\t{}\t{:02X}\t0\tstatic_handoff_plan\t{}\t0\t{}\t{}\t{}\t{}\t{}\t{}",
            block.native_index + 1,
            block.plan.cpu_addr,
            block.plan.prg_offset,
            block.byte_count,
            block.plan.first_opcode,
            observations,
            writes,
            ppu,
            apu,
            mapper,
            final_hashes,
            reason
        )?;
    }
    Ok(())
}

fn write_skipped(
    path: &Path,
    blocks: &[Block],
    native_results: &[NativeResult],
    summary: &mut Summary,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "plan_rank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\treason"
    )?;
    for block in blocks {
        let block_results = native_results
            .iter()
            .enumerate()
            .filter(|(index, _)| index / 4 == block.native_index);
        if block_results
            .into_iter()
            .any(|(_, result)| result.unsupported_opcode.is_some())
        {
            summary.skipped_unsupported += 1;
            writeln!(
                file,
                "{}\t{:04X}\t{:05X}\t{}\t{:02X}\tunsupported_native_opcode",
                block.plan.plan_rank,
                block.plan.cpu_addr,
                block.plan.prg_offset,
                block.plan.label,
                block.plan.first_opcode
            )?;
        }
    }
    Ok(())
}

fn read_oracle_rows(path: &Path) -> io::Result<Vec<OracleRow>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 31 {
            return invalid_tsv(path, line_no + 1, fields.len(), 31);
        }
        rows.push(OracleRow {
            replay: fields[0].to_string(),
            native_index: fields[1]
                .parse()
                .map_err(|_| invalid_data(path, line_no + 1, "native_index"))?,
            cpu_addr: fields[2].to_string(),
            prg_offset: fields[3].to_string(),
            bytes: fields[4].to_string(),
            first_frame: fields[5].to_string(),
            hit_ordinal: fields[6]
                .parse()
                .map_err(|_| invalid_data(path, line_no + 1, "hit_ordinal"))?,
            status: fields[14].to_string(),
            final_pc: fields[17].to_string(),
            cycles: fields[18].to_string(),
            writes: fields[19].parse().unwrap_or(0),
            ppu_writes: fields[20].parse().unwrap_or(0),
            apu_writes: fields[21].parse().unwrap_or(0),
            mapper_writes: fields[22].parse().unwrap_or(0),
            unmapped_reads: fields[23].parse().unwrap_or(0),
            final_a: fields[25].to_string(),
            final_x: fields[26].to_string(),
            final_y: fields[27].to_string(),
            final_p: fields[28].to_string(),
            final_s: fields[29].to_string(),
            final_ram_sha256: fields[30].to_string(),
        });
    }
    Ok(rows)
}

fn read_external_writes(path: &Path) -> io::Result<HashMap<(usize, u32), Vec<ExternalWrite>>> {
    let text = fs::read_to_string(path)?;
    let mut rows = HashMap::<(usize, u32), Vec<ExternalWrite>>::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 11 {
            return invalid_tsv(path, line_no + 1, fields.len(), 11);
        }
        let native_index = fields[1]
            .parse::<usize>()
            .map_err(|_| invalid_data(path, line_no + 1, "native_index"))?;
        let hit_ordinal = fields[5]
            .parse::<u32>()
            .map_err(|_| invalid_data(path, line_no + 1, "hit_ordinal"))?;
        rows.entry((native_index, hit_ordinal))
            .or_default()
            .push(ExternalWrite {
                kind: fields[8].to_string(),
                addr: fields[9].to_string(),
                value: fields[10].to_string(),
            });
    }
    Ok(rows)
}

fn format_external_writes(writes: &[ExternalWrite]) -> String {
    writes
        .iter()
        .map(|write| format!("{}:{}:{}", write.kind, write.addr, write.value))
        .collect::<Vec<_>>()
        .join(",")
}

fn write_native_manifest(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("native_verify/manifest.txt"))?;
    writeln!(
        file,
        "cases=native_verify/static_handoff_native_verify_cases.tsv"
    )?;
    writeln!(file, "final_states=native_block_final_states.tsv")?;
    writeln!(file, "case_count={}", summary.synthetic_case_count)?;
    writeln!(file, "matched={}", summary.matched)?;
    writeln!(file, "mismatches={}", summary.mismatches)?;
    writeln!(
        file,
        "external_write_matched={}",
        summary.external_write_matched
    )?;
    writeln!(
        file,
        "external_write_mismatches={}",
        summary.external_write_mismatches
    )?;
    writeln!(
        file,
        "scope=rust native linear block output, external writes, and final state versus block-exec oracle"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_summary(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    let oracle = read_oracle_rows(&out_dir.join("oracle/block_state_exec.tsv"))?;
    let mut ram_write_rows = 0u64;
    let mut ram_writes_total = 0u64;
    let mut external_writes_total = 0u64;
    let mut unmapped_reads_total = 0u64;
    for row in &oracle {
        if row.writes > 0 {
            ram_write_rows += 1;
        }
        ram_writes_total += row.writes;
        external_writes_total += row.ppu_writes + row.apu_writes + row.mapper_writes;
        unmapped_reads_total += row.unmapped_reads;
    }
    let mut file = fs::File::create(out_dir.join("static_handoff_verify_summary.txt"))?;
    writeln!(file, "runtime=static_handoff_verify_rust")?;
    writeln!(file, "selected_count={}", summary.selected_count)?;
    writeln!(
        file,
        "synthetic_case_count={}",
        summary.synthetic_case_count
    )?;
    writeln!(file, "ram_write_rows={ram_write_rows}")?;
    writeln!(file, "ram_writes_total={ram_writes_total}")?;
    writeln!(file, "external_writes_total={external_writes_total}")?;
    writeln!(file, "unmapped_reads_total={unmapped_reads_total}")?;
    writeln!(file, "native_mismatches={}", summary.mismatches)?;
    writeln!(
        file,
        "native_external_write_mismatches={}",
        summary.external_write_mismatches
    )?;
    writeln!(file, "skipped_unsupported={}", summary.skipped_unsupported)?;
    writeln!(file, "native_blocks=static_handoff_native_blocks.tsv")?;
    writeln!(file, "cases=static_handoff_state_cases.tsv")?;
    writeln!(file, "skipped=static_handoff_skipped.tsv")?;
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=static_handoff_verify_rust")?;
    writeln!(file, "kind=handoff")?;
    writeln!(file, "selected_count={}", summary.selected_count)?;
    writeln!(
        file,
        "synthetic_case_count={}",
        summary.synthetic_case_count
    )?;
    writeln!(file, "skipped_unsupported={}", summary.skipped_unsupported)?;
    writeln!(file, "native_blocks=static_handoff_native_blocks.tsv")?;
    writeln!(file, "cases=static_handoff_state_cases.tsv")?;
    writeln!(file, "skipped=static_handoff_skipped.tsv")?;
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "summary=static_handoff_verify_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_empty_outputs(out_dir: &Path) -> io::Result<()> {
    fs::write(
        out_dir.join("static_handoff_native_blocks.tsv"),
        format!("{BLOCK_HEADER}\n"),
    )?;
    fs::write(
        out_dir.join("static_handoff_skipped.tsv"),
        "plan_rank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\treason\n",
    )?;
    fs::write(out_dir.join("oracle/block_state_exec.tsv"), "replay\tnative_index\tcpu_addr\tprg_offset\tbytes\tfirst_frame\thit_ordinal\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n")?;
    fs::write(out_dir.join("oracle/block_state_external_writes.tsv"), "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\thit_ordinal\twrite_index\texternal_index\tkind\taddr\tvalue\n")?;
    fs::write(
        out_dir.join("oracle/unsupported_opcodes.tsv"),
        "opcode\tcount\n",
    )?;
    fs::write(
        out_dir.join("native_verify/native_block_verify.tsv"),
        format!("{VERIFY_HEADER}\n"),
    )?;
    fs::write(
        out_dir.join("native_verify/native_block_final_states.tsv"),
        format!("{FINAL_HEADER}\n"),
    )?;
    fs::write(
        out_dir.join("native_verify/static_handoff_native_verify_cases.tsv"),
        format!("{VERIFY_CASE_HEADER}\n"),
    )?;
    let summary = Summary::default();
    write_oracle_manifest_empty(out_dir)?;
    write_native_manifest(out_dir, &summary)?;
    write_summary(out_dir, &summary)?;
    write_manifest(out_dir, &summary)
}

fn write_oracle_manifest_empty(out_dir: &Path) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("oracle/manifest.txt"))?;
    writeln!(file, "cases=static_handoff_state_cases.tsv")?;
    writeln!(file, "case_count=0")?;
    writeln!(file, "left_block=0")?;
    writeln!(file, "stopped=0")?;
    writeln!(file, "unsupported_opcode=0")?;
    writeln!(file, "step_limit=0")?;
    writeln!(file, "invalid_block=0")?;
    writeln!(file, "external_write_rows=0")?;
    writeln!(file, "external_write_alloc_failed=0")?;
    writeln!(file, "scope=rust static handoff semantic block execution")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn parse_hex_u8(value: &str) -> Option<u8> {
    u8::from_str_radix(value, 16).ok()
}

fn parse_hex_u16(value: &str) -> Option<u16> {
    u16::from_str_radix(value, 16).ok()
}

fn parse_hex_usize(value: &str) -> Option<usize> {
    usize::from_str_radix(value, 16).ok()
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect()
}

fn bit(value: bool) -> u8 {
    u8::from(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_disasm_bytes_until_next_label() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_handoff_verify_disasm_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let asm = root.join("bank.asm");
        fs::write(
            &asm,
            "L_9819:\n  ; 9819  CE 17 02  DEC $0217\n  ; 981C  A5 4E     LDA $4E\nL_981E:\n  ; 981E  05 4F     ORA $4F\n",
        )
        .unwrap();

        assert_eq!(disasm_label_byte_count(&asm, "L_9819").unwrap(), 5);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn native_linear_runner_matches_expected_state_shape() {
        let mut prg = vec![0; 0x4000];
        prg[0] = 0xce;
        prg[1] = 0x17;
        prg[2] = 0x02;
        prg[3] = 0xa5;
        prg[4] = 0x4e;
        let mut ram = [0u8; 0x800];
        ram[0x217] = 1;
        ram[0x4e] = 0x80;
        let case = Case {
            replay: "static_handoff",
            native_index: 0,
            cpu_addr: 0x8000,
            prg_offset: 0,
            byte_count: 5,
            first_frame: 0,
            hit_ordinal: 1,
            pc: 0x8000,
            a: 0,
            x: 0,
            y: 0,
            p: 0x24,
            s: 0xfb,
            ram,
        };

        let result = run_native_case(&prg, &case);
        assert_eq!(result.pc, 0x8005);
        assert_eq!(result.a, 0x80);
        assert_eq!(result.ram[0x217], 0);
        assert_eq!(result.p & 0x82, 0x80);
        assert_eq!(result.cycles, 9);
        assert!(result.unsupported_opcode.is_none());
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
