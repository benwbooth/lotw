use crate::block_exec;
use lotw_port::rom::InesRom;
use lotw_port::sha256;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const FLAG_C: u8 = 0x01;
const FLAG_Z: u8 = 0x02;
const FLAG_V: u8 = 0x40;
const FLAG_N: u8 = 0x80;

const BRANCH_OPCODES: &[u8] = &[0x10, 0x30, 0x50, 0x70, 0x90, 0xb0, 0xd0, 0xf0];

const CASE_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tbytes\tfirst_frame\thit_ordinal\tpc\ta\tx\ty\tp\ts\tram_0000_07ff";
const BLOCK_HEADER: &str = "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason";
const TARGET_HEADER: &str = "native_index\tplan_rank\tlabel\tcpu_addr\tprg_offset\tbyte_count\tbranch_cpu_addr\tbranch_prg_offset\tbranch_opcode\tbranch_mnemonic\ttarget_cpu_addr\ttarget_prg_offset\tfallthrough_cpu_addr\tfallthrough_prg_offset";
const OUTCOME_HEADER: &str = "native_index\tplan_rank\tlabel\tcpu_addr\tprg_offset\tbranch_cpu_addr\tbranch_opcode\tbranch_mnemonic\ttarget_cpu_addr\tfallthrough_cpu_addr\tcase_count\ttarget_cases\tfallthrough_cases\tother_final_pc_cases";
const SKIP_HEADER: &str = "plan_rank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\treason";
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
struct BranchTarget {
    byte_count: u16,
    branch_cpu_addr: u16,
    branch_prg_offset: usize,
    branch_opcode: u8,
    branch_mnemonic: &'static str,
    target_cpu_addr: u16,
    target_prg_offset: usize,
    fallthrough_cpu_addr: u16,
    fallthrough_prg_offset: usize,
}

#[derive(Debug, Clone)]
struct Block {
    native_index: usize,
    plan: PlanRow,
    target: BranchTarget,
}

#[derive(Debug, Clone)]
struct SkippedBlock {
    plan: PlanRow,
    reason: &'static str,
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
    ppu_regs: [u8; 8],
    apu_regs: [u8; 0x18],
    external_writes: Vec<ExternalWrite>,
    unmapped_reads: u64,
    unsupported_opcode: Option<u8>,
}

#[derive(Debug, Clone)]
struct Instruction {
    cpu_addr: u16,
    bytes: Vec<u8>,
}

#[derive(Debug, Default)]
struct Summary {
    selected_count: u64,
    synthetic_case_count: u64,
    matched: u64,
    mismatches: u64,
    external_write_matched: u64,
    external_write_mismatches: u64,
    skipped_missing_target: u64,
    skipped_missing_fallthrough: u64,
    skipped_missing_branch_info: u64,
    skipped_unsupported: u64,
    skipped_unmapped: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    rom_path: &Path,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if limit == 0 {
        return Err("static_branch_verify: limit must be positive".into());
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
    let (blocks, mut skipped) = select_blocks(&plan_path, &rom, limit, &existing)?;
    let (blocks, cases, case_skipped) = select_branch_cases(&rom, blocks);
    skipped.extend(case_skipped);

    let mut summary = Summary {
        selected_count: blocks.len() as u64,
        synthetic_case_count: cases.len() as u64,
        ..Summary::default()
    };
    for skipped in &skipped {
        match skipped.reason {
            "missing_target_case" => summary.skipped_missing_target += 1,
            "missing_fallthrough_case" => summary.skipped_missing_fallthrough += 1,
            "missing_branch_info" => summary.skipped_missing_branch_info += 1,
            "unsupported_native_opcode" => summary.skipped_unsupported += 1,
            "oracle_unmapped_read" => summary.skipped_unmapped += 1,
            _ => {}
        }
    }

    write_cases(&out_dir.join("static_branch_state_cases.tsv"), &cases)?;
    write_targets(&out_dir.join("static_branch_targets.tsv"), &blocks)?;
    write_skipped(&out_dir.join("static_branch_skipped.tsv"), &skipped)?;

    if cases.is_empty() {
        write_empty_outputs(out_dir, &summary)?;
        println!("static_branch_verify: wrote {}", out_dir.display());
        return Ok(());
    }

    block_exec::run_case_states(
        rom_path,
        &out_dir.join("static_branch_state_cases.tsv"),
        &out_dir.join("oracle"),
        64,
    )?;

    let oracle_rows = read_oracle_rows(&out_dir.join("oracle/block_state_exec.tsv"))?;
    let external_writes =
        read_external_writes(&out_dir.join("oracle/block_state_external_writes.tsv"))?;
    let native_results = run_native_cases(&rom, &cases);

    write_native_verify(
        out_dir,
        &cases,
        &native_results,
        &oracle_rows,
        &external_writes,
        &mut summary,
    )?;
    write_outcomes(
        &out_dir.join("static_branch_outcomes.tsv"),
        &blocks,
        &oracle_rows,
    )?;
    write_verified_blocks(
        &out_dir.join("static_branch_native_blocks.tsv"),
        &blocks,
        &oracle_rows,
    )?;
    write_summary(out_dir, &summary)?;
    write_manifest(out_dir, &summary)?;

    if summary.mismatches != 0 || summary.external_write_mismatches != 0 {
        return Err("static_branch_verify: native verification mismatches remain".into());
    }
    if oracle_rows.iter().any(|row| row.status != "left_block")
        || oracle_rows.iter().any(|row| row.unmapped_reads != 0)
    {
        return Err("static_branch_verify: oracle rows did not cleanly leave block".into());
    }

    println!("static_branch_verify: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_branch_verify: missing input: {}", path.display()),
        ))
    }
}

fn read_existing_block_keys(build_dir: &Path) -> io::Result<HashSet<String>> {
    let path = build_dir.join("static_branch_verify/static_branch_native_blocks.tsv");
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

fn select_blocks(
    path: &Path,
    rom: &InesRom,
    limit: usize,
    existing: &HashSet<String>,
) -> io::Result<(Vec<Block>, Vec<SkippedBlock>)> {
    let mut blocks = Vec::new();
    let mut skipped = Vec::new();
    let text = fs::read_to_string(path)?;
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 20 {
            return invalid_tsv(path, line_no + 1, fields.len(), 20);
        }
        if fields[12] != "generate_branch_taken_and_fallthrough_cases" {
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

        match find_first_branch_target(&row, rom.prg_rom())? {
            Some(target) => blocks.push(Block {
                native_index: blocks.len(),
                plan: row,
                target,
            }),
            None => skipped.push(SkippedBlock {
                plan: row,
                reason: "missing_branch_info",
            }),
        }

        if blocks.len() >= limit {
            break;
        }
    }
    Ok((blocks, skipped))
}

fn find_first_branch_target(row: &PlanRow, prg: &[u8]) -> io::Result<Option<BranchTarget>> {
    let instructions = disasm_label_instructions(&row.file, &row.label)?;
    for instruction in instructions {
        let Some(opcode) = instruction.bytes.first().copied() else {
            continue;
        };
        if !BRANCH_OPCODES.contains(&opcode) || instruction.bytes.len() < 2 {
            continue;
        }
        let branch_cpu_addr = instruction.cpu_addr;
        let fallthrough_cpu_addr = branch_cpu_addr.wrapping_add(2);
        let target_cpu_addr =
            fallthrough_cpu_addr.wrapping_add_signed(i16::from(instruction.bytes[1] as i8));
        let Some(branch_prg_offset) = map_cpu_to_prg(row, branch_cpu_addr, prg.len()) else {
            continue;
        };
        let Some(target_prg_offset) = map_cpu_to_prg(row, target_cpu_addr, prg.len()) else {
            continue;
        };
        let Some(fallthrough_prg_offset) = map_cpu_to_prg(row, fallthrough_cpu_addr, prg.len())
        else {
            continue;
        };
        let byte_count = fallthrough_cpu_addr.wrapping_sub(row.cpu_addr);
        if byte_count == 0 {
            continue;
        }
        return Ok(Some(BranchTarget {
            byte_count,
            branch_cpu_addr,
            branch_prg_offset,
            branch_opcode: opcode,
            branch_mnemonic: branch_mnemonic(opcode),
            target_cpu_addr,
            target_prg_offset,
            fallthrough_cpu_addr,
            fallthrough_prg_offset,
        }));
    }
    Ok(None)
}

fn branch_mnemonic(opcode: u8) -> &'static str {
    match opcode {
        0x10 => "BPL",
        0x30 => "BMI",
        0x50 => "BVC",
        0x70 => "BVS",
        0x90 => "BCC",
        0xb0 => "BCS",
        0xd0 => "BNE",
        0xf0 => "BEQ",
        _ => "B??",
    }
}

fn disasm_label_instructions(path: &Path, label: &str) -> io::Result<Vec<Instruction>> {
    let text = fs::read_to_string(path)?;
    let mut in_label = false;
    let mut instructions = Vec::new();
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
        if let Some(instruction) = parse_disasm_instruction(trimmed) {
            instructions.push(instruction);
        }
    }
    Ok(instructions)
}

fn parse_disasm_instruction(line: &str) -> Option<Instruction> {
    let rest = line.strip_prefix(';')?.trim();
    let mut fields = rest.split_whitespace();
    let cpu_addr = parse_hex_u16(fields.next()?)?;
    let mut bytes = Vec::new();
    for field in fields {
        if field.len() == 2 {
            if let Ok(byte) = u8::from_str_radix(field, 16) {
                bytes.push(byte);
                continue;
            }
        }
        break;
    }
    (!bytes.is_empty()).then_some(Instruction { cpu_addr, bytes })
}

fn map_cpu_to_prg(row: &PlanRow, addr: u16, prg_len: usize) -> Option<usize> {
    let window_base_addr = row.cpu_addr & 0xe000;
    let window_delta = usize::from(row.cpu_addr - window_base_addr);
    let window_base_offset = row.prg_offset.checked_sub(window_delta);

    if addr >= window_base_addr && u32::from(addr) < u32::from(window_base_addr) + 0x2000 {
        if let Some(base) = window_base_offset {
            let off = base + usize::from(addr - window_base_addr);
            if off < prg_len {
                return Some(off);
            }
        }
    }

    if addr >= 0xc000 && prg_len >= 0x4000 {
        let off = prg_len - 0x4000 + usize::from(addr - 0xc000);
        if off < prg_len {
            return Some(off);
        }
    }

    None
}

fn select_branch_cases(
    rom: &InesRom,
    blocks: Vec<Block>,
) -> (Vec<Block>, Vec<Case>, Vec<SkippedBlock>) {
    let mut selected_blocks = Vec::new();
    let mut selected_cases = Vec::new();
    let mut skipped = Vec::new();

    for mut block in blocks {
        let mut target_cases = Vec::new();
        let mut fallthrough_cases = Vec::new();
        let mut unsupported_seen = false;
        let mut unmapped_seen = false;
        let target_only = branch_fallthrough_is_impossible(&block, rom.prg_rom());
        let target_limit = if target_only { 4 } else { 2 };

        for mut case in build_candidate_cases(&block) {
            let native = run_native_case(rom.prg_rom(), &case);
            unsupported_seen |= native.unsupported_opcode.is_some();
            unmapped_seen |= native.unmapped_reads != 0;
            if native.unsupported_opcode.is_some() || native.unmapped_reads != 0 {
                continue;
            }
            if native.pc == block.target.target_cpu_addr {
                if target_cases.len() < target_limit {
                    target_cases.push(case);
                }
            } else if native.pc == block.target.fallthrough_cpu_addr && fallthrough_cases.len() < 2
            {
                case.hit_ordinal = (fallthrough_cases.len() + 1) as u32;
                fallthrough_cases.push(case);
            }
            if (target_only && target_cases.len() >= 4)
                || (target_cases.len() >= 2 && fallthrough_cases.len() >= 2)
            {
                break;
            }
        }

        if target_cases.is_empty() {
            skipped.push(SkippedBlock {
                plan: block.plan,
                reason: if unsupported_seen {
                    "unsupported_native_opcode"
                } else if unmapped_seen {
                    "oracle_unmapped_read"
                } else {
                    "missing_target_case"
                },
            });
            continue;
        }
        if fallthrough_cases.is_empty() && !target_only {
            skipped.push(SkippedBlock {
                plan: block.plan,
                reason: if unsupported_seen {
                    "unsupported_native_opcode"
                } else if unmapped_seen {
                    "oracle_unmapped_read"
                } else {
                    "missing_fallthrough_case"
                },
            });
            continue;
        }

        block.native_index = selected_blocks.len();
        let mut cases = target_cases;
        cases.extend(fallthrough_cases);
        while cases.len() > 4 {
            cases.pop();
        }
        for (index, case) in cases.iter_mut().enumerate() {
            case.native_index = block.native_index;
            case.hit_ordinal = (index + 1) as u32;
        }
        selected_cases.extend(cases);
        selected_blocks.push(block);
    }

    (selected_blocks, selected_cases, skipped)
}

fn build_candidate_cases(block: &Block) -> Vec<Case> {
    let regs: [(u8, u8, u8, u8, u8); 16] = [
        (0x00, 0x00, 0x00, 0x26, 0xfb),
        (0x01, 0x01, 0x01, 0x24, 0xfb),
        (0x05, 0x02, 0x02, 0x25, 0xfb),
        (0x08, 0x07, 0x03, 0x20, 0xfb),
        (0x0a, 0x08, 0x04, 0x21, 0xfb),
        (0x0f, 0x0f, 0x05, 0xa0, 0xfb),
        (0x10, 0x10, 0x06, 0x64, 0xf7),
        (0x20, 0x20, 0x07, 0x65, 0xf7),
        (0x40, 0x40, 0x08, 0x24, 0xf7),
        (0x7f, 0x7f, 0x09, 0x25, 0xf7),
        (0x80, 0x80, 0x0a, 0xa4, 0xf7),
        (0xe1, 0x81, 0x0b, 0x24, 0xf7),
        (0xe5, 0xfe, 0x0c, 0x25, 0xf7),
        (0xef, 0xff, 0x7f, 0xa4, 0xf7),
        (0xf0, 0x00, 0x80, 0x24, 0xfb),
        (0xff, 0x01, 0xff, 0xa5, 0xfb),
    ];
    let mut cases = Vec::new();
    for round in 0..64 {
        for (case_index, (a, x, y, p, s)) in regs.iter().copied().enumerate() {
            cases.push(Case {
                replay: "static_branch",
                native_index: block.native_index,
                cpu_addr: block.plan.cpu_addr,
                prg_offset: block.plan.prg_offset,
                byte_count: block.target.byte_count,
                first_frame: 0,
                hit_ordinal: (cases.len() + 1) as u32,
                pc: block.plan.cpu_addr,
                a: a.wrapping_add((round * 3) as u8),
                x: x.wrapping_add(round as u8),
                y: y.wrapping_add((round % 16) as u8),
                p,
                s,
                ram: synthetic_ram(block.plan.cpu_addr, case_index, round),
            });
        }
    }
    cases
}

fn synthetic_ram(cpu_addr: u16, case_index: usize, round: usize) -> [u8; 0x800] {
    let mut ram = [0u8; 0x800];
    let seed = u32::from(cpu_addr) + (case_index as u32 * 29) + (round as u32 * 31);
    for (index, byte) in ram.iter_mut().enumerate() {
        *byte = ((index as u32 * 19 + seed + 0x31) & 0xff) as u8;
    }

    ram[0x0008] = 0;
    ram[0x000a] = 0x20;
    ram[0x000b] = if round.is_multiple_of(2) { 1 } else { 3 };
    ram[0x000c] = 0x00;
    ram[0x000d] = 0x00;
    ram[0x000e] = if round.is_multiple_of(2) { 0x30 } else { 0x40 };
    ram[0x000f] = if round.is_multiple_of(3) { 0x05 } else { 0x08 };
    ram[0x0016] = 0x00;
    ram[0x0017] = 0x20;
    ram[0x0018] = 0xff;
    ram[0x0019] = 0x20;
    ram[0x0037] = if round.is_multiple_of(2) { 0x80 } else { 0 };
    ram[0x0038] = 0x80;
    ram[0x0039] = 0;
    ram[0x003a] = 0;
    ram[0x003b] = 0;
    ram[0x0040] = 0x04;
    ram[0x0042] = 0x01;
    ram[0x0043] = 0x30;
    ram[0x0049] = if round.is_multiple_of(2) { 0 } else { 1 };
    ram[0x004b] = if round.is_multiple_of(3) { 0 } else { 1 };
    ram[0x004e] = if round.is_multiple_of(2) { 0 } else { 1 };
    ram[0x004f] = if round.is_multiple_of(3) { 0 } else { 1 };
    ram[0x005a] = 0xf0;
    ram[0x0060 + (round % 16)] = (round % 12) as u8;
    ram[0x0079] = 0x00;
    ram[0x007a] = 0x00;
    ram[0x007b] = 0x10;
    ram[0x007c] = if round.is_multiple_of(2) { 0x00 } else { 0x20 };
    ram[0x008e] = 0;
    ram[0x008f] = 0;
    ram[0x00ef] = 0;
    ram[0x00f3] = if round.is_multiple_of(2) { 0 } else { 1 };
    ram[0x00fe] = 0x80;
    for index in 0..0x100 {
        ram[0x0100 + index] = if (round + index).is_multiple_of(2) {
            0x04
        } else {
            0x09
        };
    }
    for index in 0..0x80 {
        ram[0x0400 + index] = ((index + round + case_index) & 0xff) as u8;
    }
    ram[0x040d + (round % 16)] = if round.is_multiple_of(2) { 0x30 } else { 0x08 };
    ram[0x040f + (round % 16)] = if round.is_multiple_of(2) { 0 } else { 1 };
    ram
}

fn branch_fallthrough_is_impossible(block: &Block, prg: &[u8]) -> bool {
    if block.target.branch_opcode != 0xd0 {
        return false;
    }
    let branch_offset = usize::from(block.target.branch_cpu_addr - block.plan.cpu_addr);
    if branch_offset < 7 || block.plan.prg_offset + branch_offset > prg.len() {
        return false;
    }
    let prefix_start = block.plan.prg_offset + branch_offset - 7;
    let prefix_end = block.plan.prg_offset + branch_offset;
    prg.get(prefix_start..prefix_end) == Some(&[0x98, 0x0a, 0xa8, 0x8a, 0x2a, 0xaa, 0xc8])
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
        ppu_regs: [0; 8],
        apu_regs: [0; 0x18],
        external_writes: Vec::new(),
        unmapped_reads: 0,
        unsupported_opcode: None,
    };

    for _ in 0..64 {
        if !in_block_range(case.cpu_addr, case.byte_count, result.pc) {
            break;
        }
        let opcode = fetch8(prg, case, &mut result);
        match opcode {
            0x05 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                result.a |= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 3;
            }
            0x09 => {
                let value = fetch8(prg, case, &mut result);
                result.a |= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x0a => {
                set_carry(&mut result.p, result.a & 0x80 != 0);
                result.a <<= 1;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x10 => {
                let condition = result.p & FLAG_N == 0;
                branch_if(prg, case, &mut result, condition);
            }
            0x18 => {
                set_carry(&mut result.p, false);
                result.cycles += 2;
            }
            0x1d => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.x));
                let value = read_native(prg, case, &mut result, addr);
                result.a |= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 4 + u64::from(page_crossed(base, addr));
            }
            0x20 => {
                let target = fetch16(prg, case, &mut result);
                let return_addr = result.pc.wrapping_sub(1);
                push8(&mut result, (return_addr >> 8) as u8);
                push8(&mut result, return_addr as u8);
                result.pc = target;
                result.cycles += 6;
            }
            0x24 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                bit(&mut result, value);
                result.cycles += 3;
            }
            0x25 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                result.a &= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 3;
            }
            0x29 => {
                let value = fetch8(prg, case, &mut result);
                result.a &= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x2a => {
                let value = result.a;
                result.a = (value << 1) | u8::from(result.p & FLAG_C != 0);
                set_carry(&mut result.p, value & 0x80 != 0);
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x30 => {
                let condition = result.p & FLAG_N != 0;
                branch_if(prg, case, &mut result, condition);
            }
            0x38 => {
                set_carry(&mut result.p, true);
                result.cycles += 2;
            }
            0x46 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                set_carry(&mut result.p, value & 0x01 != 0);
                let shifted = value >> 1;
                write_native(&mut result, zp, shifted);
                set_nz(&mut result.p, shifted);
                result.cycles += 5;
            }
            0x48 => {
                let value = result.a;
                push8(&mut result, value);
                result.cycles += 3;
            }
            0x49 => {
                let value = fetch8(prg, case, &mut result);
                result.a ^= value;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x4a => {
                set_carry(&mut result.p, result.a & 0x01 != 0);
                result.a >>= 1;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x4c => {
                result.pc = fetch16(prg, case, &mut result);
                result.cycles += 3;
            }
            0x60 => {
                let lo = pull8(&mut result);
                let hi = pull8(&mut result);
                result.pc = u16::from_le_bytes([lo, hi]).wrapping_add(1);
                result.cycles += 6;
            }
            0x65 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                adc(&mut result, value);
                result.cycles += 3;
            }
            0x68 => {
                result.a = pull8(&mut result);
                set_nz(&mut result.p, result.a);
                result.cycles += 4;
            }
            0x69 => {
                let value = fetch8(prg, case, &mut result);
                adc(&mut result, value);
                result.cycles += 2;
            }
            0x70 => {
                let condition = result.p & FLAG_V != 0;
                branch_if(prg, case, &mut result, condition);
            }
            0x84 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = result.y;
                write_native(&mut result, zp, value);
                result.cycles += 3;
            }
            0x85 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = result.a;
                write_native(&mut result, zp, value);
                result.cycles += 3;
            }
            0x86 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = result.x;
                write_native(&mut result, zp, value);
                result.cycles += 3;
            }
            0x88 => {
                result.y = result.y.wrapping_sub(1);
                set_nz(&mut result.p, result.y);
                result.cycles += 2;
            }
            0x8a => {
                result.a = result.x;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x8c => {
                let addr = fetch16(prg, case, &mut result);
                let value = result.y;
                write_native(&mut result, addr, value);
                result.cycles += 4;
            }
            0x8d => {
                let addr = fetch16(prg, case, &mut result);
                let value = result.a;
                write_native(&mut result, addr, value);
                result.cycles += 4;
            }
            0x90 => {
                let condition = result.p & FLAG_C == 0;
                branch_if(prg, case, &mut result, condition);
            }
            0x98 => {
                result.a = result.y;
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0x99 => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.y));
                let value = result.a;
                write_native(&mut result, addr, value);
                result.cycles += 5;
            }
            0x9d => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.x));
                let value = result.a;
                write_native(&mut result, addr, value);
                result.cycles += 5;
            }
            0xa0 => {
                result.y = fetch8(prg, case, &mut result);
                set_nz(&mut result.p, result.y);
                result.cycles += 2;
            }
            0xa2 => {
                result.x = fetch8(prg, case, &mut result);
                set_nz(&mut result.p, result.x);
                result.cycles += 2;
            }
            0xa4 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                result.y = read_native(prg, case, &mut result, zp);
                set_nz(&mut result.p, result.y);
                result.cycles += 3;
            }
            0xa5 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                result.a = read_native(prg, case, &mut result, zp);
                set_nz(&mut result.p, result.a);
                result.cycles += 3;
            }
            0xa6 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                result.x = read_native(prg, case, &mut result, zp);
                set_nz(&mut result.p, result.x);
                result.cycles += 3;
            }
            0xa8 => {
                result.y = result.a;
                set_nz(&mut result.p, result.y);
                result.cycles += 2;
            }
            0xa9 => {
                result.a = fetch8(prg, case, &mut result);
                set_nz(&mut result.p, result.a);
                result.cycles += 2;
            }
            0xaa => {
                result.x = result.a;
                set_nz(&mut result.p, result.x);
                result.cycles += 2;
            }
            0xad => {
                let addr = fetch16(prg, case, &mut result);
                result.a = read_native(prg, case, &mut result, addr);
                set_nz(&mut result.p, result.a);
                result.cycles += 4;
            }
            0xb0 => {
                let condition = result.p & FLAG_C != 0;
                branch_if(prg, case, &mut result, condition);
            }
            0xb1 => {
                let zp = fetch8(prg, case, &mut result);
                let base = read_zp16(&result, zp);
                let addr = base.wrapping_add(u16::from(result.y));
                result.a = read_native(prg, case, &mut result, addr);
                set_nz(&mut result.p, result.a);
                result.cycles += 5 + u64::from(page_crossed(base, addr));
            }
            0xb5 => {
                let zp = fetch8(prg, case, &mut result).wrapping_add(result.x) as u16;
                result.a = read_native(prg, case, &mut result, zp);
                set_nz(&mut result.p, result.a);
                result.cycles += 4;
            }
            0xb9 => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.y));
                result.a = read_native(prg, case, &mut result, addr);
                set_nz(&mut result.p, result.a);
                result.cycles += 4 + u64::from(page_crossed(base, addr));
            }
            0xbd => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.x));
                result.a = read_native(prg, case, &mut result, addr);
                set_nz(&mut result.p, result.a);
                result.cycles += 4 + u64::from(page_crossed(base, addr));
            }
            0xc5 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                let register = result.a;
                cmp(&mut result, register, value);
                result.cycles += 3;
            }
            0xc6 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp).wrapping_sub(1);
                write_native(&mut result, zp, value);
                set_nz(&mut result.p, value);
                result.cycles += 5;
            }
            0xc8 => {
                result.y = result.y.wrapping_add(1);
                set_nz(&mut result.p, result.y);
                result.cycles += 2;
            }
            0xc9 => {
                let value = fetch8(prg, case, &mut result);
                let register = result.a;
                cmp(&mut result, register, value);
                result.cycles += 2;
            }
            0xca => {
                result.x = result.x.wrapping_sub(1);
                set_nz(&mut result.p, result.x);
                result.cycles += 2;
            }
            0xd0 => {
                let condition = result.p & FLAG_Z == 0;
                branch_if(prg, case, &mut result, condition);
            }
            0xe0 => {
                let value = fetch8(prg, case, &mut result);
                let register = result.x;
                cmp(&mut result, register, value);
                result.cycles += 2;
            }
            0xe5 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp);
                sbc(&mut result, value);
                result.cycles += 3;
            }
            0xe6 => {
                let zp = fetch8(prg, case, &mut result) as u16;
                let value = read_native(prg, case, &mut result, zp).wrapping_add(1);
                write_native(&mut result, zp, value);
                set_nz(&mut result.p, value);
                result.cycles += 5;
            }
            0xe8 => {
                result.x = result.x.wrapping_add(1);
                set_nz(&mut result.p, result.x);
                result.cycles += 2;
            }
            0xe9 => {
                let value = fetch8(prg, case, &mut result);
                sbc(&mut result, value);
                result.cycles += 2;
            }
            0xf0 => {
                let condition = result.p & FLAG_Z != 0;
                branch_if(prg, case, &mut result, condition);
            }
            0xfd => {
                let base = fetch16(prg, case, &mut result);
                let addr = base.wrapping_add(u16::from(result.x));
                let value = read_native(prg, case, &mut result, addr);
                sbc(&mut result, value);
                result.cycles += 4 + u64::from(page_crossed(base, addr));
            }
            unsupported => {
                result.unsupported_opcode = Some(unsupported);
                break;
            }
        }
        if result.unsupported_opcode.is_some() {
            break;
        }
    }

    result
}

fn fetch8(prg: &[u8], case: &Case, result: &mut NativeResult) -> u8 {
    let value = read_prg(prg, case, result, result.pc);
    result.pc = result.pc.wrapping_add(1);
    value
}

fn fetch16(prg: &[u8], case: &Case, result: &mut NativeResult) -> u16 {
    let lo = fetch8(prg, case, result);
    let hi = fetch8(prg, case, result);
    u16::from_le_bytes([lo, hi])
}

fn read_prg(prg: &[u8], case: &Case, result: &mut NativeResult, addr: u16) -> u8 {
    if let Some(offset) = map_case_cpu_to_prg(case, addr, prg.len()) {
        return prg[offset];
    }
    result.unmapped_reads += 1;
    0
}

fn map_case_cpu_to_prg(case: &Case, addr: u16, prg_len: usize) -> Option<usize> {
    let window_base_addr = case.cpu_addr & 0xe000;
    let window_delta = usize::from(case.cpu_addr - window_base_addr);
    let window_base_offset = case.prg_offset.checked_sub(window_delta);

    if in_block_range(case.cpu_addr, case.byte_count, addr) {
        let off = case.prg_offset + usize::from(addr - case.cpu_addr);
        if off < prg_len {
            return Some(off);
        }
    }

    if addr >= window_base_addr && u32::from(addr) < u32::from(window_base_addr) + 0x2000 {
        if let Some(base) = window_base_offset {
            let off = base + usize::from(addr - window_base_addr);
            if off < prg_len {
                return Some(off);
            }
        }
    }

    if addr >= 0xc000 && prg_len >= 0x4000 {
        let off = prg_len - 0x4000 + usize::from(addr - 0xc000);
        if off < prg_len {
            return Some(off);
        }
    }

    None
}

fn in_block_range(cpu_addr: u16, byte_count: u16, pc: u16) -> bool {
    let start = u32::from(cpu_addr);
    let end = start + u32::from(byte_count);
    let pc = u32::from(pc);
    pc >= start && pc < end
}

fn read_native(prg: &[u8], case: &Case, result: &mut NativeResult, addr: u16) -> u8 {
    if addr < 0x2000 {
        result.ram[usize::from(addr & 0x07ff)]
    } else if (0x2000..0x4000).contains(&addr) {
        result.ppu_regs[usize::from(addr & 0x0007)]
    } else if (0x4000..=0x4017).contains(&addr) {
        result.apu_regs[usize::from(addr - 0x4000)]
    } else if addr >= 0x8000 {
        read_prg(prg, case, result, addr)
    } else {
        result.unmapped_reads += 1;
        0
    }
}

fn write_native(result: &mut NativeResult, addr: u16, value: u8) {
    if addr < 0x2000 {
        result.ram[usize::from(addr & 0x07ff)] = value;
    } else if (0x2000..0x4000).contains(&addr) {
        result.ppu_regs[usize::from(addr & 0x0007)] = value;
        result.external_writes.push(ExternalWrite {
            kind: "ppu".to_string(),
            addr: format!("{addr:04X}"),
            value: format!("{value:02X}"),
        });
    } else if (0x4000..=0x4017).contains(&addr) {
        result.apu_regs[usize::from(addr - 0x4000)] = value;
        result.external_writes.push(ExternalWrite {
            kind: "apu".to_string(),
            addr: format!("{addr:04X}"),
            value: format!("{value:02X}"),
        });
    } else if addr >= 0x8000 {
        result.external_writes.push(ExternalWrite {
            kind: "mapper".to_string(),
            addr: format!("{addr:04X}"),
            value: format!("{value:02X}"),
        });
    }
}

fn push8(result: &mut NativeResult, value: u8) {
    let addr = 0x0100 | u16::from(result.s);
    write_native(result, addr, value);
    result.s = result.s.wrapping_sub(1);
}

fn pull8(result: &mut NativeResult) -> u8 {
    result.s = result.s.wrapping_add(1);
    result.ram[usize::from(0x0100 | u16::from(result.s))]
}

fn read_zp16(result: &NativeResult, zp: u8) -> u16 {
    let lo = result.ram[usize::from(zp)] as u16;
    let hi = result.ram[usize::from(zp.wrapping_add(1))] as u16;
    lo | (hi << 8)
}

fn branch_if(prg: &[u8], case: &Case, result: &mut NativeResult, condition: bool) {
    let offset = fetch8(prg, case, result) as i8;
    let old_pc = result.pc;
    result.cycles += 2;
    if condition {
        result.pc = result.pc.wrapping_add_signed(i16::from(offset));
        result.cycles += 1;
        if page_crossed(old_pc, result.pc) {
            result.cycles += 1;
        }
    }
}

fn page_crossed(a: u16, b: u16) -> bool {
    (a & 0xff00) != (b & 0xff00)
}

fn bit(result: &mut NativeResult, value: u8) {
    let masked = result.a & value;
    set_flag(&mut result.p, FLAG_Z, masked == 0);
    set_flag(&mut result.p, FLAG_V, value & FLAG_V != 0);
    set_flag(&mut result.p, FLAG_N, value & FLAG_N != 0);
}

fn adc(result: &mut NativeResult, value: u8) {
    let carry = u16::from(result.p & FLAG_C != 0);
    let sum = u16::from(result.a) + u16::from(value) + carry;
    let next = sum as u8;
    set_carry(&mut result.p, sum > 0xff);
    set_flag(
        &mut result.p,
        FLAG_V,
        ((result.a ^ next) & (value ^ next) & 0x80) != 0,
    );
    result.a = next;
    set_nz(&mut result.p, result.a);
}

fn sbc(result: &mut NativeResult, value: u8) {
    adc(result, !value);
}

fn cmp(result: &mut NativeResult, register: u8, value: u8) {
    let diff = register.wrapping_sub(value);
    set_carry(&mut result.p, register >= value);
    set_nz(&mut result.p, diff);
}

fn set_nz(p: &mut u8, value: u8) {
    set_flag(p, FLAG_Z, value == 0);
    set_flag(p, FLAG_N, value & 0x80 != 0);
}

fn set_carry(p: &mut u8, value: bool) {
    set_flag(p, FLAG_C, value);
}

fn set_flag(p: &mut u8, flag: u8, value: bool) {
    if value {
        *p |= flag;
    } else {
        *p &= !flag;
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
    if cases.len() != native_results.len() || cases.len() != oracle_rows.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "static_branch_verify: case/native/oracle row count mismatch: cases={} native={} oracle={}",
                cases.len(),
                native_results.len(),
                oracle_rows.len()
            ),
        ));
    }

    let mut verify = fs::File::create(out_dir.join("native_verify/native_block_verify.tsv"))?;
    let mut final_states =
        fs::File::create(out_dir.join("native_verify/native_block_final_states.tsv"))?;
    let mut verify_cases =
        fs::File::create(out_dir.join("native_verify/static_branch_native_verify_cases.tsv"))?;
    writeln!(verify, "{VERIFY_HEADER}")?;
    writeln!(final_states, "{FINAL_HEADER}")?;
    writeln!(verify_cases, "{VERIFY_CASE_HEADER}")?;

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
            bit_bool(executed),
            bit_bool(metadata_match),
            bit_bool(register_match),
            bit_bool(cycles_match),
            bit_bool(ram_match),
            bit_bool(external_write_match),
            expected_external_text,
            actual_external_text,
            bit_bool(matched)
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
        for oracle in oracle_rows {
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
        }
        let reason = if ppu + apu + mapper > 0 {
            "static_branch_synthetic_external_writes"
        } else if writes > 0 {
            "static_branch_synthetic_ram_writes"
        } else {
            "static_branch_synthetic"
        };
        writeln!(
            file,
            "{}\t{:04X}\t{:05X}\t{}\t{:02X}\t0\tstatic_handoff_plan\t{}\t0\t{}\t{}\t{}\t{}\t{}\t{}",
            block.native_index + 1,
            block.plan.cpu_addr,
            block.plan.prg_offset,
            block.target.byte_count,
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

fn write_targets(path: &Path, blocks: &[Block]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{TARGET_HEADER}")?;
    for block in blocks {
        writeln!(
            file,
            "{}\t{}\t{}\t{:04X}\t{:05X}\t{}\t{:04X}\t{:05X}\t{:02X}\t{}\t{:04X}\t{:05X}\t{:04X}\t{:05X}",
            block.native_index,
            block.plan.plan_rank,
            block.plan.label,
            block.plan.cpu_addr,
            block.plan.prg_offset,
            block.target.byte_count,
            block.target.branch_cpu_addr,
            block.target.branch_prg_offset,
            block.target.branch_opcode,
            block.target.branch_mnemonic,
            block.target.target_cpu_addr,
            block.target.target_prg_offset,
            block.target.fallthrough_cpu_addr,
            block.target.fallthrough_prg_offset
        )?;
    }
    Ok(())
}

fn write_outcomes(path: &Path, blocks: &[Block], oracle_rows: &[OracleRow]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{OUTCOME_HEADER}")?;
    for block in blocks {
        let block_rows = oracle_rows
            .iter()
            .filter(|row| row.native_index == block.native_index)
            .collect::<Vec<_>>();
        let target_cases = block_rows
            .iter()
            .filter(|row| row.final_pc == format!("{:04X}", block.target.target_cpu_addr))
            .count();
        let fallthrough_cases = block_rows
            .iter()
            .filter(|row| row.final_pc == format!("{:04X}", block.target.fallthrough_cpu_addr))
            .count();
        writeln!(
            file,
            "{}\t{}\t{}\t{:04X}\t{:05X}\t{:04X}\t{:02X}\t{}\t{:04X}\t{:04X}\t{}\t{}\t{}\t{}",
            block.native_index,
            block.plan.plan_rank,
            block.plan.label,
            block.plan.cpu_addr,
            block.plan.prg_offset,
            block.target.branch_cpu_addr,
            block.target.branch_opcode,
            block.target.branch_mnemonic,
            block.target.target_cpu_addr,
            block.target.fallthrough_cpu_addr,
            block_rows.len(),
            target_cases,
            fallthrough_cases,
            block_rows
                .len()
                .saturating_sub(target_cases + fallthrough_cases)
        )?;
    }
    Ok(())
}

fn write_skipped(path: &Path, skipped: &[SkippedBlock]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "{SKIP_HEADER}")?;
    for skipped in skipped {
        writeln!(
            file,
            "{}\t{:04X}\t{:05X}\t{}\t{:02X}\t{}",
            skipped.plan.plan_rank,
            skipped.plan.cpu_addr,
            skipped.plan.prg_offset,
            skipped.plan.label,
            skipped.plan.first_opcode,
            skipped.reason
        )?;
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
        "cases=native_verify/static_branch_native_verify_cases.tsv"
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
        "scope=rust native branch handoff output, external writes, and final state versus block-exec oracle"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_summary(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    let oracle_path = out_dir.join("oracle/block_state_exec.tsv");
    let oracle = if oracle_path.is_file() {
        read_oracle_rows(&oracle_path)?
    } else {
        Vec::new()
    };
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

    let mut file = fs::File::create(out_dir.join("static_branch_verify_summary.txt"))?;
    writeln!(file, "runtime=static_branch_verify_rust")?;
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
    writeln!(
        file,
        "skipped_missing_target={}",
        summary.skipped_missing_target
    )?;
    writeln!(
        file,
        "skipped_missing_fallthrough={}",
        summary.skipped_missing_fallthrough
    )?;
    writeln!(
        file,
        "skipped_missing_branch_info={}",
        summary.skipped_missing_branch_info
    )?;
    writeln!(file, "skipped_unsupported={}", summary.skipped_unsupported)?;
    writeln!(file, "skipped_unmapped={}", summary.skipped_unmapped)?;
    writeln!(file, "native_blocks=static_branch_native_blocks.tsv")?;
    writeln!(file, "targets=static_branch_targets.tsv")?;
    writeln!(file, "outcomes=static_branch_outcomes.tsv")?;
    writeln!(file, "cases=static_branch_state_cases.tsv")?;
    writeln!(file, "skipped=static_branch_skipped.tsv")?;
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=static_branch_verify_rust")?;
    writeln!(file, "kind=branch")?;
    writeln!(file, "selected_count={}", summary.selected_count)?;
    writeln!(
        file,
        "synthetic_case_count={}",
        summary.synthetic_case_count
    )?;
    writeln!(file, "skipped_unsupported={}", summary.skipped_unsupported)?;
    writeln!(file, "native_blocks=static_branch_native_blocks.tsv")?;
    writeln!(file, "targets=static_branch_targets.tsv")?;
    writeln!(file, "outcomes=static_branch_outcomes.tsv")?;
    writeln!(file, "cases=static_branch_state_cases.tsv")?;
    writeln!(file, "skipped=static_branch_skipped.tsv")?;
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "summary=static_branch_verify_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_empty_outputs(out_dir: &Path, summary: &Summary) -> io::Result<()> {
    fs::write(
        out_dir.join("static_branch_native_blocks.tsv"),
        format!("{BLOCK_HEADER}\n"),
    )?;
    fs::write(
        out_dir.join("static_branch_outcomes.tsv"),
        format!("{OUTCOME_HEADER}\n"),
    )?;
    fs::write(
        out_dir.join("oracle/block_state_exec.tsv"),
        "replay\tnative_index\tcpu_addr\tprg_offset\tbytes\tfirst_frame\thit_ordinal\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n",
    )?;
    fs::write(
        out_dir.join("oracle/block_state_external_writes.tsv"),
        "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\thit_ordinal\twrite_index\texternal_index\tkind\taddr\tvalue\n",
    )?;
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
        out_dir.join("native_verify/static_branch_native_verify_cases.tsv"),
        format!("{VERIFY_CASE_HEADER}\n"),
    )?;
    write_oracle_manifest_empty(out_dir)?;
    write_native_manifest(out_dir, summary)?;
    write_summary(out_dir, summary)?;
    write_manifest(out_dir, summary)
}

fn write_oracle_manifest_empty(out_dir: &Path) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("oracle/manifest.txt"))?;
    writeln!(file, "cases=static_branch_state_cases.tsv")?;
    writeln!(file, "case_count=0")?;
    writeln!(file, "left_block=0")?;
    writeln!(file, "stopped=0")?;
    writeln!(file, "unsupported_opcode=0")?;
    writeln!(file, "step_limit=0")?;
    writeln!(file, "invalid_block=0")?;
    writeln!(file, "external_write_rows=0")?;
    writeln!(file, "external_write_alloc_failed=0")?;
    writeln!(file, "scope=rust static branch semantic block execution")?;
    writeln!(file, "complete=1")?;
    Ok(())
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

fn bit_bool(value: bool) -> u8 {
    u8::from(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_conditional_branch_in_disasm_label() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_branch_verify_disasm_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let asm = root.join("bank.asm");
        fs::write(
            &asm,
            "L_B036:\n  ; B036  A5 FE     LDA $FE\n  ; B038  F0 04     BEQ L_B03E\n  ; B03A  C6 42     DEC $42\nL_B03E:\n  ; B03E  60        RTS\n",
        )
        .unwrap();
        let row = PlanRow {
            plan_rank: "3".to_string(),
            cpu_addr: 0xb036,
            prg_offset: 0x1f036,
            label: "L_B036".to_string(),
            first_opcode: 0xa5,
            file: asm,
        };

        let target = find_first_branch_target(&row, &vec![0; 0x20000])
            .unwrap()
            .unwrap();
        assert_eq!(target.byte_count, 4);
        assert_eq!(target.branch_cpu_addr, 0xb038);
        assert_eq!(target.branch_opcode, 0xf0);
        assert_eq!(target.target_cpu_addr, 0xb03e);
        assert_eq!(target.fallthrough_cpu_addr, 0xb03a);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn native_branch_runner_takes_and_falls_through() {
        let mut prg = vec![0; 0x4000];
        prg[0] = 0xa5;
        prg[1] = 0xfe;
        prg[2] = 0xf0;
        prg[3] = 0x04;
        let mut ram = [0; 0x800];
        ram[0xfe] = 0;
        let mut case = Case {
            replay: "static_branch",
            native_index: 0,
            cpu_addr: 0x8000,
            prg_offset: 0,
            byte_count: 4,
            first_frame: 0,
            hit_ordinal: 1,
            pc: 0x8000,
            a: 0x44,
            x: 0,
            y: 0,
            p: 0x24,
            s: 0xfb,
            ram,
        };
        let taken = run_native_case(&prg, &case);
        assert_eq!(taken.pc, 0x8008);
        assert_eq!(taken.cycles, 6);

        case.ram[0xfe] = 1;
        let fallthrough = run_native_case(&prg, &case);
        assert_eq!(fallthrough.pc, 0x8004);
        assert_eq!(fallthrough.cycles, 5);
        assert!(fallthrough.unsupported_opcode.is_none());
    }

    #[test]
    fn native_branch_runner_records_external_writes() {
        let mut prg = vec![0; 0x4000];
        prg[0] = 0x8d;
        prg[1] = 0x07;
        prg[2] = 0x20;
        prg[3] = 0xca;
        prg[4] = 0x10;
        prg[5] = 0xfa;
        let case = Case {
            replay: "static_branch",
            native_index: 0,
            cpu_addr: 0x9000,
            prg_offset: 0,
            byte_count: 6,
            first_frame: 0,
            hit_ordinal: 1,
            pc: 0x9000,
            a: 0x55,
            x: 0,
            y: 0,
            p: 0x24,
            s: 0xfb,
            ram: [0; 0x800],
        };
        let result = run_native_case(&prg, &case);
        assert_eq!(result.pc, 0x9006);
        assert_eq!(result.external_writes.len(), 1);
        assert_eq!(result.external_writes[0].kind, "ppu");
        assert_eq!(result.external_writes[0].addr, "2007");
        assert_eq!(result.external_writes[0].value, "55");
    }

    #[test]
    fn recognizes_tya_asl_tay_iny_bne_as_target_only() {
        let block = Block {
            native_index: 0,
            plan: PlanRow {
                plan_rank: "37".to_string(),
                cpu_addr: 0x8c6e,
                prg_offset: 0,
                label: "L_8C6E".to_string(),
                first_opcode: 0x98,
                file: PathBuf::new(),
            },
            target: BranchTarget {
                byte_count: 9,
                branch_cpu_addr: 0x8c75,
                branch_prg_offset: 7,
                branch_opcode: 0xd0,
                branch_mnemonic: "BNE",
                target_cpu_addr: 0x8c78,
                target_prg_offset: 10,
                fallthrough_cpu_addr: 0x8c77,
                fallthrough_prg_offset: 9,
            },
        };
        let prg = vec![0x98, 0x0a, 0xa8, 0x8a, 0x2a, 0xaa, 0xc8, 0xd0, 0x01];

        assert!(branch_fallthrough_is_impossible(&block, &prg));
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
