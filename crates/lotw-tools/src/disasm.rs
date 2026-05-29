use lotw_port::rom::InesRom;
use lotw_port::sha256;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const BANK_16K: usize = 0x4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AddrMode {
    Imp,
    Acc,
    Imm,
    Zp,
    Zpx,
    Zpy,
    Rel,
    Abs,
    Absx,
    Absy,
    Ind,
    Indx,
    Indy,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OpInfo {
    pub(crate) mnemonic: Option<&'static str>,
    pub(crate) mode: AddrMode,
    pub(crate) len: usize,
    pub(crate) is_branch: bool,
    pub(crate) is_jump: bool,
    pub(crate) is_jsr: bool,
}

impl OpInfo {
    const fn op(mnemonic: &'static str, mode: AddrMode, len: usize) -> Self {
        Self {
            mnemonic: Some(mnemonic),
            mode,
            len,
            is_branch: false,
            is_jump: false,
            is_jsr: false,
        }
    }

    const fn branch(mnemonic: &'static str) -> Self {
        Self {
            mnemonic: Some(mnemonic),
            mode: AddrMode::Rel,
            len: 2,
            is_branch: true,
            is_jump: false,
            is_jsr: false,
        }
    }

    const fn jump(mode: AddrMode) -> Self {
        Self {
            mnemonic: Some("JMP"),
            mode,
            len: 3,
            is_branch: false,
            is_jump: true,
            is_jsr: false,
        }
    }

    const fn jsr() -> Self {
        Self {
            mnemonic: Some("JSR"),
            mode: AddrMode::Abs,
            len: 3,
            is_branch: false,
            is_jump: false,
            is_jsr: true,
        }
    }

    const fn stop(mnemonic: &'static str) -> Self {
        Self::op(mnemonic, AddrMode::Imp, 1)
    }

    const fn unknown() -> Self {
        Self {
            mnemonic: None,
            mode: AddrMode::Imp,
            len: 1,
            is_branch: false,
            is_jump: false,
            is_jsr: false,
        }
    }
}

pub fn run(
    rom_path: &Path,
    out_dir: &Path,
    expected_sha256: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read(rom_path)?;
    let actual_sha256 = sha256::digest_hex(&bytes);
    if let Some(expected) = expected_sha256 {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "disasm: ROM hash mismatch: got {actual_sha256}, expected {}",
                expected.to_ascii_lowercase()
            )
            .into());
        }
    }

    let rom = InesRom::parse(&bytes)?;
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let nmi = vector_at(&rom, 0)?;
    let reset = vector_at(&rom, 1)?;
    let irq = vector_at(&rom, 2)?;
    let mut labels = vec![false; 65536];
    labels[usize::from(nmi)] = true;
    labels[usize::from(reset)] = true;
    labels[usize::from(irq)] = true;

    let bank_count = rom.header().prg_rom_size / BANK_16K;
    for bank in 0..bank_count {
        collect_labels_for_region(
            &rom.prg_rom()[bank * BANK_16K..(bank + 1) * BANK_16K],
            0x8000,
            &mut labels,
        );
    }
    if bank_count > 0 {
        collect_labels_for_region(
            &rom.prg_rom()[(bank_count - 1) * BANK_16K..bank_count * BANK_16K],
            0xC000,
            &mut labels,
        );
    }

    write_manifest(
        &out_dir.join("manifest.txt"),
        &actual_sha256,
        &rom,
        nmi,
        reset,
        irq,
    )?;
    write_vectors_asm(&out_dir.join("vectors.asm"), nmi, reset, irq)?;
    write_labels(&out_dir.join("labels.txt"), &labels, nmi, reset, irq)?;
    write_disasm_files(out_dir, &rom, &labels)?;
    write_translation_plan(&out_dir.join("translation_plan.txt"))?;

    println!("disasm: wrote {}", out_dir.display());
    println!("vectors: nmi=${nmi:04X} reset=${reset:04X} irq=${irq:04X}");
    Ok(())
}

fn vector_at(rom: &InesRom, vector_index: usize) -> io::Result<u16> {
    let offset = rom
        .header()
        .prg_rom_size
        .checked_sub(6)
        .and_then(|base| base.checked_add(vector_index * 2))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "ROM PRG is too small"))?;
    Ok(read16(&rom.prg_rom()[offset..]))
}

fn read16(bytes: &[u8]) -> u16 {
    u16::from(bytes[0]) | (u16::from(bytes[1]) << 8)
}

pub(crate) fn op_info(opcode: u8) -> OpInfo {
    use AddrMode::*;
    match opcode {
        0x00 => OpInfo::stop("BRK"),
        0x01 => OpInfo::op("ORA", Indx, 2),
        0x05 => OpInfo::op("ORA", Zp, 2),
        0x06 => OpInfo::op("ASL", Zp, 2),
        0x08 => OpInfo::op("PHP", Imp, 1),
        0x09 => OpInfo::op("ORA", Imm, 2),
        0x0A => OpInfo::op("ASL", Acc, 1),
        0x0D => OpInfo::op("ORA", Abs, 3),
        0x0E => OpInfo::op("ASL", Abs, 3),
        0x10 => OpInfo::branch("BPL"),
        0x11 => OpInfo::op("ORA", Indy, 2),
        0x15 => OpInfo::op("ORA", Zpx, 2),
        0x16 => OpInfo::op("ASL", Zpx, 2),
        0x18 => OpInfo::op("CLC", Imp, 1),
        0x19 => OpInfo::op("ORA", Absy, 3),
        0x1D => OpInfo::op("ORA", Absx, 3),
        0x1E => OpInfo::op("ASL", Absx, 3),
        0x20 => OpInfo::jsr(),
        0x21 => OpInfo::op("AND", Indx, 2),
        0x24 => OpInfo::op("BIT", Zp, 2),
        0x25 => OpInfo::op("AND", Zp, 2),
        0x26 => OpInfo::op("ROL", Zp, 2),
        0x28 => OpInfo::op("PLP", Imp, 1),
        0x29 => OpInfo::op("AND", Imm, 2),
        0x2A => OpInfo::op("ROL", Acc, 1),
        0x2C => OpInfo::op("BIT", Abs, 3),
        0x2D => OpInfo::op("AND", Abs, 3),
        0x2E => OpInfo::op("ROL", Abs, 3),
        0x30 => OpInfo::branch("BMI"),
        0x31 => OpInfo::op("AND", Indy, 2),
        0x35 => OpInfo::op("AND", Zpx, 2),
        0x36 => OpInfo::op("ROL", Zpx, 2),
        0x38 => OpInfo::op("SEC", Imp, 1),
        0x39 => OpInfo::op("AND", Absy, 3),
        0x3D => OpInfo::op("AND", Absx, 3),
        0x3E => OpInfo::op("ROL", Absx, 3),
        0x40 => OpInfo::stop("RTI"),
        0x41 => OpInfo::op("EOR", Indx, 2),
        0x45 => OpInfo::op("EOR", Zp, 2),
        0x46 => OpInfo::op("LSR", Zp, 2),
        0x48 => OpInfo::op("PHA", Imp, 1),
        0x49 => OpInfo::op("EOR", Imm, 2),
        0x4A => OpInfo::op("LSR", Acc, 1),
        0x4C => OpInfo::jump(Abs),
        0x4D => OpInfo::op("EOR", Abs, 3),
        0x4E => OpInfo::op("LSR", Abs, 3),
        0x50 => OpInfo::branch("BVC"),
        0x51 => OpInfo::op("EOR", Indy, 2),
        0x55 => OpInfo::op("EOR", Zpx, 2),
        0x56 => OpInfo::op("LSR", Zpx, 2),
        0x58 => OpInfo::op("CLI", Imp, 1),
        0x59 => OpInfo::op("EOR", Absy, 3),
        0x5D => OpInfo::op("EOR", Absx, 3),
        0x5E => OpInfo::op("LSR", Absx, 3),
        0x60 => OpInfo::stop("RTS"),
        0x61 => OpInfo::op("ADC", Indx, 2),
        0x65 => OpInfo::op("ADC", Zp, 2),
        0x66 => OpInfo::op("ROR", Zp, 2),
        0x68 => OpInfo::op("PLA", Imp, 1),
        0x69 => OpInfo::op("ADC", Imm, 2),
        0x6A => OpInfo::op("ROR", Acc, 1),
        0x6C => OpInfo::jump(Ind),
        0x6D => OpInfo::op("ADC", Abs, 3),
        0x6E => OpInfo::op("ROR", Abs, 3),
        0x70 => OpInfo::branch("BVS"),
        0x71 => OpInfo::op("ADC", Indy, 2),
        0x75 => OpInfo::op("ADC", Zpx, 2),
        0x76 => OpInfo::op("ROR", Zpx, 2),
        0x78 => OpInfo::op("SEI", Imp, 1),
        0x79 => OpInfo::op("ADC", Absy, 3),
        0x7D => OpInfo::op("ADC", Absx, 3),
        0x7E => OpInfo::op("ROR", Absx, 3),
        0x81 => OpInfo::op("STA", Indx, 2),
        0x84 => OpInfo::op("STY", Zp, 2),
        0x85 => OpInfo::op("STA", Zp, 2),
        0x86 => OpInfo::op("STX", Zp, 2),
        0x88 => OpInfo::op("DEY", Imp, 1),
        0x8A => OpInfo::op("TXA", Imp, 1),
        0x8C => OpInfo::op("STY", Abs, 3),
        0x8D => OpInfo::op("STA", Abs, 3),
        0x8E => OpInfo::op("STX", Abs, 3),
        0x90 => OpInfo::branch("BCC"),
        0x91 => OpInfo::op("STA", Indy, 2),
        0x94 => OpInfo::op("STY", Zpx, 2),
        0x95 => OpInfo::op("STA", Zpx, 2),
        0x96 => OpInfo::op("STX", Zpy, 2),
        0x98 => OpInfo::op("TYA", Imp, 1),
        0x99 => OpInfo::op("STA", Absy, 3),
        0x9A => OpInfo::op("TXS", Imp, 1),
        0x9D => OpInfo::op("STA", Absx, 3),
        0xA0 => OpInfo::op("LDY", Imm, 2),
        0xA1 => OpInfo::op("LDA", Indx, 2),
        0xA2 => OpInfo::op("LDX", Imm, 2),
        0xA4 => OpInfo::op("LDY", Zp, 2),
        0xA5 => OpInfo::op("LDA", Zp, 2),
        0xA6 => OpInfo::op("LDX", Zp, 2),
        0xA8 => OpInfo::op("TAY", Imp, 1),
        0xA9 => OpInfo::op("LDA", Imm, 2),
        0xAA => OpInfo::op("TAX", Imp, 1),
        0xAC => OpInfo::op("LDY", Abs, 3),
        0xAD => OpInfo::op("LDA", Abs, 3),
        0xAE => OpInfo::op("LDX", Abs, 3),
        0xB0 => OpInfo::branch("BCS"),
        0xB1 => OpInfo::op("LDA", Indy, 2),
        0xB4 => OpInfo::op("LDY", Zpx, 2),
        0xB5 => OpInfo::op("LDA", Zpx, 2),
        0xB6 => OpInfo::op("LDX", Zpy, 2),
        0xB8 => OpInfo::op("CLV", Imp, 1),
        0xB9 => OpInfo::op("LDA", Absy, 3),
        0xBA => OpInfo::op("TSX", Imp, 1),
        0xBC => OpInfo::op("LDY", Absx, 3),
        0xBD => OpInfo::op("LDA", Absx, 3),
        0xBE => OpInfo::op("LDX", Absy, 3),
        0xC0 => OpInfo::op("CPY", Imm, 2),
        0xC1 => OpInfo::op("CMP", Indx, 2),
        0xC4 => OpInfo::op("CPY", Zp, 2),
        0xC5 => OpInfo::op("CMP", Zp, 2),
        0xC6 => OpInfo::op("DEC", Zp, 2),
        0xC8 => OpInfo::op("INY", Imp, 1),
        0xC9 => OpInfo::op("CMP", Imm, 2),
        0xCA => OpInfo::op("DEX", Imp, 1),
        0xCC => OpInfo::op("CPY", Abs, 3),
        0xCD => OpInfo::op("CMP", Abs, 3),
        0xCE => OpInfo::op("DEC", Abs, 3),
        0xD0 => OpInfo::branch("BNE"),
        0xD1 => OpInfo::op("CMP", Indy, 2),
        0xD5 => OpInfo::op("CMP", Zpx, 2),
        0xD6 => OpInfo::op("DEC", Zpx, 2),
        0xD8 => OpInfo::op("CLD", Imp, 1),
        0xD9 => OpInfo::op("CMP", Absy, 3),
        0xDD => OpInfo::op("CMP", Absx, 3),
        0xDE => OpInfo::op("DEC", Absx, 3),
        0xE0 => OpInfo::op("CPX", Imm, 2),
        0xE1 => OpInfo::op("SBC", Indx, 2),
        0xE4 => OpInfo::op("CPX", Zp, 2),
        0xE5 => OpInfo::op("SBC", Zp, 2),
        0xE6 => OpInfo::op("INC", Zp, 2),
        0xE8 => OpInfo::op("INX", Imp, 1),
        0xE9 => OpInfo::op("SBC", Imm, 2),
        0xEA => OpInfo::op("NOP", Imp, 1),
        0xEC => OpInfo::op("CPX", Abs, 3),
        0xED => OpInfo::op("SBC", Abs, 3),
        0xEE => OpInfo::op("INC", Abs, 3),
        0xF0 => OpInfo::branch("BEQ"),
        0xF1 => OpInfo::op("SBC", Indy, 2),
        0xF5 => OpInfo::op("SBC", Zpx, 2),
        0xF6 => OpInfo::op("INC", Zpx, 2),
        0xF8 => OpInfo::op("SED", Imp, 1),
        0xF9 => OpInfo::op("SBC", Absy, 3),
        0xFD => OpInfo::op("SBC", Absx, 3),
        0xFE => OpInfo::op("INC", Absx, 3),
        _ => OpInfo::unknown(),
    }
}

pub(crate) fn format_operand(op: OpInfo, operand_data: &[u8], off: usize, addr: u16) -> String {
    let b1 = operand_data.get(off + 1).copied().unwrap_or(0xFF);
    let b2 = operand_data.get(off + 2).copied().unwrap_or(0xFF);
    let w = u16::from(b1) | (u16::from(b2) << 8);
    let rel = (i32::from(addr) + op.len as i32 + i32::from(b1 as i8)) as u16;
    match op.mode {
        AddrMode::Imp => String::new(),
        AddrMode::Acc => "A".to_string(),
        AddrMode::Imm => format!("#${b1:02X}"),
        AddrMode::Zp => format!("${b1:02X}"),
        AddrMode::Zpx => format!("${b1:02X},X"),
        AddrMode::Zpy => format!("${b1:02X},Y"),
        AddrMode::Rel => format!("L_{rel:04X}"),
        AddrMode::Abs => format!("${w:04X}"),
        AddrMode::Absx => format!("${w:04X},X"),
        AddrMode::Absy => format!("${w:04X},Y"),
        AddrMode::Ind => format!("(${w:04X})"),
        AddrMode::Indx => format!("(${b1:02X},X)"),
        AddrMode::Indy => format!("(${b1:02X}),Y"),
    }
}

pub(crate) fn absolute_operand(op: OpInfo, bytes: &[u8]) -> Option<u16> {
    matches!(
        op.mode,
        AddrMode::Abs | AddrMode::Absx | AddrMode::Absy | AddrMode::Ind
    )
    .then(|| read16(&bytes[1..]))
}

fn should_label_jsr_return(call_addr: u16, target: u16) -> bool {
    if matches!(target, 0xC1C7 | 0xC1D8 | 0xE660) {
        return true;
    }
    (call_addr == 0xE5C7 && target == 0xCD2C) || (call_addr == 0xE5CA && target == 0xD8B6)
}

fn collect_labels_for_region(data: &[u8], base_addr: u16, labels: &mut [bool]) {
    let mut off = 0usize;
    while off < data.len() {
        let addr = base_addr.wrapping_add(off as u16);
        let opcode = data[off];
        let op = op_info(opcode);
        let mut len = op.len;
        if off + len > data.len() {
            len = 1;
        }
        let bytes = &data[off..off + len];

        if op.is_branch && len >= 2 {
            let target = (i32::from(addr) + op.len as i32 + i32::from(bytes[1] as i8)) as u16;
            labels[usize::from(target)] = true;
        } else if (op.is_jump || op.is_jsr) && len >= 3 {
            if let Some(target) = absolute_operand(op, bytes) {
                if target >= 0x8000 {
                    labels[usize::from(target)] = true;
                }
                if op.is_jsr && should_label_jsr_return(addr, target) {
                    let continuation = addr.wrapping_add(len as u16);
                    if continuation >= 0x8000 {
                        labels[usize::from(continuation)] = true;
                    }
                }
            }
        }

        off += len;
    }
}

fn write_disasm_region(
    file: &mut fs::File,
    region_data: &[u8],
    operand_data: &[u8],
    base_addr: u16,
    labels: &[bool],
) -> io::Result<()> {
    let mut off = 0usize;
    while off < region_data.len() {
        let addr = base_addr.wrapping_add(off as u16);
        let opcode = region_data[off];
        let op = op_info(opcode);
        let mut len = op.len;
        if off + len > region_data.len() {
            len = 1;
        }
        let bytes = &region_data[off..off + len];

        if labels[usize::from(addr)] {
            writeln!(file, "L_{addr:04X}:")?;
        }

        let bytes_text = bytes
            .iter()
            .map(|value| format!("{value:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        if let Some(mnemonic) = op.mnemonic {
            let operand = format_operand(op, operand_data, off, addr);
            writeln!(
                file,
                "  ; {addr:04X}  {bytes_text:<8}  {mnemonic:<3} {operand}"
            )?;
        } else {
            writeln!(file, "  ; {addr:04X}  {bytes_text:<8}  .db ${opcode:02X}")?;
        }

        off += len;
    }
    Ok(())
}

fn write_manifest(
    path: &Path,
    sha256_hex: &str,
    rom: &InesRom,
    nmi: u16,
    reset: u16,
    irq: u16,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let header = rom.header();
    writeln!(file, "sha256={sha256_hex}")?;
    writeln!(file, "mapper={}", header.mapper)?;
    writeln!(file, "prg_size={}", header.prg_rom_size)?;
    writeln!(file, "chr_size={}", header.chr_rom_size)?;
    writeln!(file, "prg_8k_pages={}", header.prg_rom_size / 0x2000)?;
    writeln!(file, "prg_16k_banks={}", header.prg_rom_size / BANK_16K)?;
    writeln!(
        file,
        "mmc3_note=CPU $8000-$BFFF is bank-switched; CPU $C000-$FFFF is treated as the fixed final 16 KiB bank for vector seeds."
    )?;
    writeln!(file, "vector_nmi=${nmi:04X}")?;
    writeln!(file, "vector_reset=${reset:04X}")?;
    writeln!(file, "vector_irq=${irq:04X}")?;
    if let Some(offset) = rom.fixed_bank_prg_offset(nmi) {
        writeln!(file, "vector_nmi_prg_offset=0x{offset:05X}")?;
    }
    if let Some(offset) = rom.fixed_bank_prg_offset(reset) {
        writeln!(file, "vector_reset_prg_offset=0x{offset:05X}")?;
    }
    if let Some(offset) = rom.fixed_bank_prg_offset(irq) {
        writeln!(file, "vector_irq_prg_offset=0x{offset:05X}")?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_vectors_asm(path: &Path, nmi: u16, reset: u16, irq: u16) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "; Legacy of the Wizard vector seed labels")?;
    writeln!(file, "VECTOR_NMI   = ${nmi:04X}")?;
    writeln!(file, "VECTOR_RESET = ${reset:04X}")?;
    writeln!(file, "VECTOR_IRQ   = ${irq:04X}")?;
    writeln!(file)?;
    for addr in 0x8000u16..=0xFFFF {
        let mut names = Vec::new();
        if addr == nmi {
            names.push("NMI");
        }
        if addr == reset {
            names.push("RESET");
        }
        if addr == irq {
            names.push("IRQ/BRK");
        }
        if !names.is_empty() {
            writeln!(file, "L_{addr:04X}: ; {}", names.join(" "))?;
        }
    }
    Ok(())
}

fn write_labels(path: &Path, labels: &[bool], nmi: u16, reset: u16, irq: u16) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "vector_nmi=L_{nmi:04X}")?;
    writeln!(file, "vector_reset=L_{reset:04X}")?;
    writeln!(file, "vector_irq=L_{irq:04X}")?;
    for (addr, is_label) in labels.iter().enumerate().take(0x10000).skip(0x8000) {
        if *is_label {
            writeln!(file, "L_{addr:04X} target=control_flow")?;
        }
    }
    Ok(())
}

fn write_disasm_files(out_dir: &Path, rom: &InesRom, labels: &[bool]) -> io::Result<()> {
    let bank_count = rom.header().prg_rom_size / BANK_16K;
    for bank in 0..bank_count {
        let path = out_dir.join(format!("prg_bank_{bank:02}_8000.asm"));
        let mut file = fs::File::create(path)?;
        writeln!(file, "; PRG 16 KiB bank {bank} mapped at CPU $8000-$BFFF")?;
        writeln!(file, "; Mapper 4/MMC3 can switch this window at runtime.")?;
        writeln!(file)?;
        write_disasm_region(
            &mut file,
            &rom.prg_rom()[bank * BANK_16K..(bank + 1) * BANK_16K],
            &rom.prg_rom()[bank * BANK_16K..],
            0x8000,
            labels,
        )?;
    }

    if bank_count > 0 {
        let path = out_dir.join(format!("fixed_bank_{:02}_c000.asm", bank_count - 1));
        let mut file = fs::File::create(path)?;
        writeln!(
            file,
            "; Final PRG 16 KiB bank mapped at CPU $C000-$FFFF for reset vectors"
        )?;
        writeln!(file)?;
        write_disasm_region(
            &mut file,
            &rom.prg_rom()[(bank_count - 1) * BANK_16K..bank_count * BANK_16K],
            &rom.prg_rom()[(bank_count - 1) * BANK_16K..bank_count * BANK_16K],
            0xC000,
            labels,
        )?;
    }
    Ok(())
}

fn write_translation_plan(path: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "status=seed")?;
    writeln!(
        file,
        "scope=vector entry stubs and mapper-aware linear disassembly"
    )?;
    writeln!(
        file,
        "next=trace reset/NMI/IRQ in FCEUX, identify code/data ranges, then generate per-routine Rust only for proven routine bounds"
    )?;
    writeln!(
        file,
        "copyright=all files in this directory are generated from the local ROM and must remain ignored"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_disasm_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + BANK_16K * 2 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 0x41;
        let prg = 16;
        bytes[prg] = 0x20;
        bytes[prg + 1] = 0x10;
        bytes[prg + 2] = 0x80;
        bytes[prg + 3] = 0xF0;
        bytes[prg + 4] = 0x02;
        bytes[prg + 5] = 0x60;
        bytes[prg + 0x4000] = 0x4C;
        bytes[prg + 0x4001] = 0x00;
        bytes[prg + 0x4002] = 0xC0;
        let vectors = prg + BANK_16K * 2 - 6;
        bytes[vectors] = 0x00;
        bytes[vectors + 1] = 0xC0;
        bytes[vectors + 2] = 0x00;
        bytes[vectors + 3] = 0xC0;
        bytes[vectors + 4] = 0x00;
        bytes[vectors + 5] = 0xC0;
        bytes
    }

    #[test]
    fn writes_mapper_aware_disasm() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, bytes).unwrap();

        run(&rom, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("mapper=4\n"));
        assert!(manifest.contains("vector_reset=$C000\n"));
        assert!(manifest.contains("complete=1\n"));
        let labels = fs::read_to_string(out.join("labels.txt")).unwrap();
        assert!(labels.contains("vector_reset=L_C000\n"));
        assert!(labels.contains("L_8010 target=control_flow\n"));
        let bank = fs::read_to_string(out.join("prg_bank_00_8000.asm")).unwrap();
        assert!(bank.contains("  ; 8000  20 10 80  JSR $8010\n"));
        assert!(bank.contains("  ; 8003  F0 02     BEQ L_8007\n"));
        let fixed = fs::read_to_string(out.join("fixed_bank_01_c000.asm")).unwrap();
        assert!(fixed.contains("L_C000:\n"));
        assert!(fixed.contains("  ; C000  4C 00 C0  JMP $C000\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
