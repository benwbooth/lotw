use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const SUPPORTED_NATIVE_OPCODES: &[u8] = &[
    0x05, 0x08, 0x09, 0x0a, 0x10, 0x11, 0x18, 0x1d, 0x20, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2a, 0x2c,
    0x30, 0x38, 0x3d, 0x3e, 0x46, 0x48, 0x49, 0x4a, 0x4c, 0x5d, 0x5e, 0x60, 0x65, 0x68, 0x69, 0x6a,
    0x6c, 0x70, 0x7d, 0x84, 0x85, 0x86, 0x88, 0x8a, 0x8c, 0x8d, 0x8e, 0x90, 0x91, 0x95, 0x96, 0x98,
    0x99, 0x9d, 0xa0, 0xa2, 0xa4, 0xa5, 0xa6, 0xa8, 0xa9, 0xaa, 0xad, 0xae, 0xb0, 0xb1, 0xb5, 0xb6,
    0xb9, 0xbc, 0xbd, 0xc0, 0xc5, 0xc6, 0xc8, 0xc9, 0xca, 0xcd, 0xce, 0xd0, 0xd5, 0xd6, 0xe0, 0xe5,
    0xe6, 0xe8, 0xe9, 0xed, 0xf0, 0xf6, 0xfd,
];

#[derive(Debug, Clone)]
struct Unit {
    bank_kind: String,
    bank: String,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    replay_covered: String,
    replay_count: String,
    native_generated_c: bool,
    semantic_verified: bool,
    proof_kind: String,
    generated_bytes: String,
    remaining_action: String,
}

#[derive(Debug, Default, Clone)]
struct GeneratedBlock {
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    proof_kind: String,
}

#[derive(Debug, Default, Clone)]
struct EntryPlanInfo {
    static_shape: String,
    recommended_next_step: String,
    source_edge_count: String,
    source_edge_types: String,
    outgoing_edge_types: String,
}

#[derive(Debug, Default)]
struct RemainingSummary {
    total: u64,
    replay_covered_needs_split: u64,
    inside_verified_native_span: u64,
    entry_plan_leaf_return_or_interrupt: u64,
    entry_plan_control_flow: u64,
    entry_plan_calls_subroutine: u64,
    entry_plan_straight_line_or_data: u64,
    entry_plan_other: u64,
    not_in_static_entry_plan: u64,
    class_counts: BTreeMap<String, u64>,
}

#[derive(Debug, Default, Clone)]
struct OpcodeStats {
    mnemonic: String,
    rows: u64,
    supported: bool,
}

struct SummaryInputs<'a> {
    disasm: &'a HashMap<String, String>,
    static_summary: &'a HashMap<String, String>,
    codegen: &'a HashMap<String, String>,
    verify: &'a HashMap<String, String>,
    semantic: &'a HashMap<String, String>,
}

pub fn run(build_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let disasm_manifest = build_dir.join("disasm/manifest.txt");
    let static_summary = build_dir.join("static_cfg/static_cfg_summary.txt");
    let reachable_labels = build_dir.join("static_cfg/static_reachable_labels.tsv");
    let entry_plan = build_dir.join("static_entry_plan/static_entry_plan.tsv");
    let handoff_plan = build_dir.join("static_handoff_plan/static_handoff_plan.tsv");
    let merged_blocks = build_dir.join("native_block_plan_static/native_blocks.tsv");
    let merge_summary =
        build_dir.join("native_block_plan_static/native_block_static_merge_summary.txt");
    let codegen_manifest = build_dir.join("native_block_codegen_static/manifest.txt");
    let static_verify_manifest = build_dir.join("native_block_static_verify/manifest.txt");
    let semantic_summary = build_dir.join("semantic_match_report/semantic_match_summary.txt");

    for path in [
        &disasm_manifest,
        &static_summary,
        &reachable_labels,
        &entry_plan,
        &handoff_plan,
        &merged_blocks,
        &merge_summary,
        &codegen_manifest,
        &static_verify_manifest,
        &semantic_summary,
    ] {
        require_file(path)?;
    }

    let disasm_kv = read_key_values(&disasm_manifest)?;
    let static_kv = read_key_values(&static_summary)?;
    let merge_kv = read_key_values(&merge_summary)?;
    let codegen_kv = read_key_values(&codegen_manifest)?;
    let verify_kv = read_key_values(&static_verify_manifest)?;
    let semantic_kv = read_key_values(&semantic_summary)?;

    ensure_eq(&disasm_kv, "complete", "1", &disasm_manifest)?;
    ensure_eq(&static_kv, "complete", "1", &static_summary)?;
    ensure_eq(&merge_kv, "complete", "1", &merge_summary)?;
    ensure_eq(&codegen_kv, "complete", "1", &codegen_manifest)?;
    ensure_eq(&codegen_kv, "unsupported_count", "0", &codegen_manifest)?;
    ensure_eq(&verify_kv, "complete", "1", &static_verify_manifest)?;
    ensure_eq(&verify_kv, "mismatches", "0", &static_verify_manifest)?;
    ensure_eq(
        &verify_kv,
        "external_write_mismatches",
        "0",
        &static_verify_manifest,
    )?;
    ensure_eq(&semantic_kv, "complete", "1", &semantic_summary)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let generated = read_generated_blocks(&merged_blocks)?;
    let entry_info = read_entry_plan(&entry_plan)?;
    let planned_actions = read_planned_actions(&handoff_plan)?;
    let units = read_units(&reachable_labels, &generated, &planned_actions)?;
    let opcode_rows = read_disasm_opcode_rows(&build_dir.join("disasm"))?;
    let opcode_stats = summarize_opcode_support(&opcode_rows);
    let remaining_summary = write_remaining_units(
        &out_dir.join("whole_program_remaining_units.tsv"),
        &units,
        &generated,
        &entry_info,
    )?;

    write_supported_opcodes(&out_dir.join("native_codegen_supported_opcodes.txt"))?;
    write_opcode_rows(&out_dir.join("linear_disasm_opcode_rows.tsv"), &opcode_rows)?;
    write_opcode_support(
        &out_dir.join("whole_program_opcode_support.tsv"),
        &opcode_stats,
    )?;
    write_units(&out_dir.join("whole_program_translation_units.tsv"), &units)?;
    write_summary(
        &out_dir.join("whole_program_summary.txt"),
        SummaryInputs {
            disasm: &disasm_kv,
            static_summary: &static_kv,
            codegen: &codegen_kv,
            verify: &verify_kv,
            semantic: &semantic_kv,
        },
        &units,
        &opcode_stats,
        &remaining_summary,
    )?;
    write_manifest(&out_dir.join("manifest.txt"))?;

    println!("whole_program_report: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("missing input: {}", path.display()),
        ))
    }
}

fn read_key_values(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn required<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing {key} in {}", path.display()).into())
}

fn ensure_eq(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = required(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{} has {key}={actual}, expected {expected}", path.display()).into())
    }
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn key(cpu_addr: &str, prg_offset: &str) -> String {
    format!(
        "{}\t{}",
        cpu_addr.to_ascii_uppercase(),
        prg_offset.to_ascii_uppercase()
    )
}

fn proof_kind(reason: &str) -> &'static str {
    match reason {
        "static_verified_leaf" => "static_leaf",
        "static_verified_handoff" => "static_handoff",
        "static_verified_branch" => "static_branch",
        "static_verified_jsr" => "static_jsr",
        "static_verified_return" => "static_return",
        _ => "replay",
    }
}

fn read_generated_blocks(path: &Path) -> io::Result<HashMap<String, GeneratedBlock>> {
    let text = fs::read_to_string(path)?;
    let mut blocks = HashMap::new();
    for line in text.lines().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 15 {
            continue;
        }
        blocks.insert(
            key(fields[1], fields[2]),
            GeneratedBlock {
                cpu_addr: fields[1].to_string(),
                prg_offset: fields[2].to_string(),
                bytes: fields[3].to_string(),
                proof_kind: proof_kind(fields[14]).to_string(),
            },
        );
    }
    Ok(blocks)
}

fn read_entry_plan(path: &Path) -> io::Result<HashMap<String, EntryPlanInfo>> {
    let text = fs::read_to_string(path)?;
    let mut entries = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 22 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} has {} fields, expected at least 22",
                    path.display(),
                    line_no + 1,
                    fields.len()
                ),
            ));
        }
        entries.insert(
            key(fields[5], fields[6]),
            EntryPlanInfo {
                source_edge_count: fields[14].to_string(),
                source_edge_types: fields[17].to_string(),
                outgoing_edge_types: fields[18].to_string(),
                static_shape: fields[19].to_string(),
                recommended_next_step: fields[20].to_string(),
            },
        );
    }
    Ok(entries)
}

fn read_planned_actions(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut actions = HashMap::new();
    for line in text.lines().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            continue;
        }
        actions.insert(key(fields[5], fields[6]), fields[12].to_string());
    }
    Ok(actions)
}

fn read_units(
    path: &Path,
    generated: &HashMap<String, GeneratedBlock>,
    planned_actions: &HashMap<String, String>,
) -> io::Result<Vec<Unit>> {
    let text = fs::read_to_string(path)?;
    let mut units = Vec::new();
    for line in text.lines().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 17 || fields[10] != "1" || fields[6] == ".db" {
            continue;
        }
        let unit_key = key(fields[2], fields[3]);
        let generated_block = generated.get(&unit_key);
        let remaining_action = match generated_block {
            Some(_) => "translated_c_block_verified".to_string(),
            None if planned_actions.contains_key(&unit_key) => planned_actions[&unit_key].clone(),
            None if fields[7] == "1" => "select_or_split_replay_covered_block".to_string(),
            None => "expand_static_entry_plan_or_confirm_data".to_string(),
        };

        units.push(Unit {
            bank_kind: fields[0].to_string(),
            bank: fields[1].to_string(),
            cpu_addr: fields[2].to_string(),
            prg_offset: fields[3].to_string(),
            label: fields[4].to_string(),
            first_opcode: fields[5].to_string(),
            mnemonic: fields[6].to_string(),
            replay_covered: fields[7].to_string(),
            replay_count: fields[8].to_string(),
            native_generated_c: generated_block.is_some(),
            semantic_verified: generated_block.is_some(),
            proof_kind: generated_block
                .map(|block| block.proof_kind.clone())
                .unwrap_or_default(),
            generated_bytes: generated_block
                .map(|block| block.bytes.clone())
                .unwrap_or_default(),
            remaining_action,
        });
    }
    Ok(units)
}

fn disasm_files(disasm_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(disasm_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_disasm = (name.starts_with("prg_bank_") && name.ends_with("_8000.asm"))
            || (name.starts_with("fixed_bank_") && name.ends_with("_c000.asm"));
        if is_disasm {
            files.push(entry.path());
        }
    }
    files.sort();
    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("no disassembly files under {}", disasm_dir.display()),
        ));
    }
    Ok(files)
}

fn is_hex_byte(token: &str) -> bool {
    token.len() == 2 && token.bytes().all(|b| b.is_ascii_hexdigit())
}

fn parse_disasm_opcode_line(line: &str) -> Option<(String, String)> {
    let rest = line.trim_start().strip_prefix(';')?.trim_start();
    let mut fields = rest.split_whitespace();
    let addr = fields.next()?;
    if addr.len() != 4 || !addr.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }

    let opcode = fields.next()?;
    if !is_hex_byte(opcode) {
        return None;
    }

    let mut mnemonic = None;
    for field in fields {
        if is_hex_byte(field) {
            continue;
        }
        mnemonic = Some(field);
        break;
    }

    Some((opcode.to_ascii_uppercase(), mnemonic?.to_string()))
}

fn read_disasm_opcode_rows(disasm_dir: &Path) -> io::Result<Vec<(String, String)>> {
    let mut rows = Vec::new();
    for path in disasm_files(disasm_dir)? {
        for line in fs::read_to_string(path)?.lines() {
            if let Some(row) = parse_disasm_opcode_line(line) {
                rows.push(row);
            }
        }
    }
    Ok(rows)
}

fn supported_opcode_strings() -> Vec<String> {
    SUPPORTED_NATIVE_OPCODES
        .iter()
        .map(|opcode| format!("{opcode:02X}"))
        .collect()
}

fn summarize_opcode_support(rows: &[(String, String)]) -> BTreeMap<String, OpcodeStats> {
    let supported: Vec<String> = supported_opcode_strings();
    let mut stats = BTreeMap::<String, OpcodeStats>::new();
    for (opcode, mnemonic) in rows {
        let entry = stats.entry(opcode.clone()).or_insert_with(|| OpcodeStats {
            mnemonic: mnemonic.clone(),
            rows: 0,
            supported: supported.iter().any(|item| item == opcode),
        });
        entry.rows += 1;
    }
    stats
}

fn write_supported_opcodes(path: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    for opcode in supported_opcode_strings() {
        writeln!(file, "{opcode}")?;
    }
    Ok(())
}

fn write_opcode_rows(path: &Path, rows: &[(String, String)]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "opcode\tmnemonic")?;
    for (opcode, mnemonic) in rows {
        writeln!(file, "{opcode}\t{mnemonic}")?;
    }
    Ok(())
}

fn write_opcode_support(path: &Path, stats: &BTreeMap<String, OpcodeStats>) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "opcode\tmnemonic\tlinear_disasm_rows\tnative_codegen_supported"
    )?;
    for (opcode, stat) in stats {
        writeln!(
            file,
            "{opcode}\t{}\t{}\t{}",
            stat.mnemonic,
            stat.rows,
            u8::from(stat.supported)
        )?;
    }
    Ok(())
}

fn write_units(path: &Path, units: &[Unit]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\treplay_covered\treplay_count\tnative_generated_c\tsemantic_verified\tproof_kind\tgenerated_bytes\tremaining_action"
    )?;
    for unit in units {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            unit.bank_kind,
            unit.bank,
            unit.cpu_addr,
            unit.prg_offset,
            unit.label,
            unit.first_opcode,
            unit.mnemonic,
            unit.replay_covered,
            unit.replay_count,
            u8::from(unit.native_generated_c),
            u8::from(unit.semantic_verified),
            unit.proof_kind,
            unit.generated_bytes,
            unit.remaining_action
        )?;
    }
    Ok(())
}

fn parse_hex_u64(value: &str) -> Option<u64> {
    u64::from_str_radix(value.trim_start_matches('$'), 16).ok()
}

fn covering_block<'a>(
    unit: &Unit,
    generated: &'a HashMap<String, GeneratedBlock>,
) -> Option<(&'a GeneratedBlock, u64)> {
    let unit_offset = parse_hex_u64(&unit.prg_offset)?;
    generated
        .values()
        .filter_map(|block| {
            let start = parse_hex_u64(&block.prg_offset)?;
            let bytes = block.bytes.parse::<u64>().ok()?;
            let end = start.checked_add(bytes)?;
            if unit_offset > start && unit_offset < end {
                Some((block, unit_offset - start))
            } else {
                None
            }
        })
        .min_by_key(|(_, offset)| *offset)
}

fn classify_remaining_unit(
    unit: &Unit,
    generated: &HashMap<String, GeneratedBlock>,
    entry_info: &HashMap<String, EntryPlanInfo>,
) -> String {
    if unit.replay_covered == "1" {
        return "replay_covered_needs_block_split".to_string();
    }
    if covering_block(unit, generated).is_some() {
        return "inside_verified_native_block_span".to_string();
    }
    let unit_key = key(&unit.cpu_addr, &unit.prg_offset);
    if let Some(entry) = entry_info.get(&unit_key) {
        return match entry.static_shape.as_str() {
            "leaf_return_or_interrupt" => "entry_plan_leaf_return_or_interrupt",
            "control_flow" => "entry_plan_control_flow",
            "calls_subroutine" => "entry_plan_calls_subroutine",
            "straight_line_or_data" => "entry_plan_straight_line_or_data",
            _ => "entry_plan_other",
        }
        .to_string();
    }
    "not_in_static_entry_plan".to_string()
}

fn count_remaining_class(summary: &mut RemainingSummary, class: &str) {
    summary.total += 1;
    *summary.class_counts.entry(class.to_string()).or_default() += 1;
    match class {
        "replay_covered_needs_block_split" => summary.replay_covered_needs_split += 1,
        "inside_verified_native_block_span" => summary.inside_verified_native_span += 1,
        "entry_plan_leaf_return_or_interrupt" => {
            summary.entry_plan_leaf_return_or_interrupt += 1;
        }
        "entry_plan_control_flow" => summary.entry_plan_control_flow += 1,
        "entry_plan_calls_subroutine" => summary.entry_plan_calls_subroutine += 1,
        "entry_plan_straight_line_or_data" => summary.entry_plan_straight_line_or_data += 1,
        "entry_plan_other" => summary.entry_plan_other += 1,
        "not_in_static_entry_plan" => summary.not_in_static_entry_plan += 1,
        _ => {}
    }
}

fn write_remaining_units(
    path: &Path,
    units: &[Unit],
    generated: &HashMap<String, GeneratedBlock>,
    entry_info: &HashMap<String, EntryPlanInfo>,
) -> io::Result<RemainingSummary> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\treplay_covered\treplay_count\tremaining_action\tremaining_class\tcovering_cpu_addr\tcovering_prg_offset\tcovering_offset_bytes\tcovering_bytes\tcovering_proof_kind\tentry_static_shape\tentry_recommended_next_step\tentry_source_edge_count\tentry_source_edge_types\tentry_outgoing_edge_types"
    )?;

    let mut summary = RemainingSummary::default();
    for unit in units.iter().filter(|unit| !unit.native_generated_c) {
        let class = classify_remaining_unit(unit, generated, entry_info);
        count_remaining_class(&mut summary, &class);
        let cover = covering_block(unit, generated);
        let unit_key = key(&unit.cpu_addr, &unit.prg_offset);
        let entry = entry_info.get(&unit_key);
        let empty_entry = EntryPlanInfo::default();
        let entry = entry.unwrap_or(&empty_entry);

        let (cover_cpu, cover_prg, cover_offset, cover_bytes, cover_kind) = cover
            .map(|(block, offset)| {
                (
                    block.cpu_addr.as_str(),
                    block.prg_offset.as_str(),
                    offset.to_string(),
                    block.bytes.as_str(),
                    block.proof_kind.as_str(),
                )
            })
            .unwrap_or(("", "", String::new(), "", ""));

        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            unit.bank_kind,
            unit.bank,
            unit.cpu_addr,
            unit.prg_offset,
            unit.label,
            unit.first_opcode,
            unit.mnemonic,
            unit.replay_covered,
            unit.replay_count,
            unit.remaining_action,
            class,
            cover_cpu,
            cover_prg,
            cover_offset,
            cover_bytes,
            cover_kind,
            entry.static_shape,
            entry.recommended_next_step,
            entry.source_edge_count,
            entry.source_edge_types,
            entry.outgoing_edge_types
        )?;
    }

    Ok(summary)
}

fn parse_u64(value: &str) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(value.parse::<u64>()?)
}

fn ratio_per_10000(numerator: u64, denominator: u64) -> u64 {
    if denominator == 0 {
        0
    } else {
        numerator * 10_000 / denominator
    }
}

fn write_summary(
    path: &Path,
    inputs: SummaryInputs<'_>,
    units: &[Unit],
    opcode_stats: &BTreeMap<String, OpcodeStats>,
    remaining: &RemainingSummary,
) -> Result<(), Box<dyn std::error::Error>> {
    let disasm = inputs.disasm;
    let static_summary = inputs.static_summary;
    let codegen = inputs.codegen;
    let verify = inputs.verify;
    let semantic = inputs.semantic;
    let generated_units = units.iter().filter(|unit| unit.native_generated_c).count() as u64;
    let remaining_units = units.len() as u64 - generated_units;
    let replay_unselected = units
        .iter()
        .filter(|unit| unit.remaining_action == "select_or_split_replay_covered_block")
        .count() as u64;
    let unplanned = units
        .iter()
        .filter(|unit| unit.remaining_action == "expand_static_entry_plan_or_confirm_data")
        .count() as u64;
    let planned = remaining_units - replay_unselected - unplanned;

    let opcode_rows_supported: u64 = opcode_stats
        .values()
        .filter(|stat| stat.supported)
        .map(|stat| stat.rows)
        .sum();
    let opcode_rows_unsupported: u64 = opcode_stats
        .values()
        .filter(|stat| !stat.supported)
        .map(|stat| stat.rows)
        .sum();

    let case_count = parse_u64(required(
        verify,
        "case_count",
        Path::new("native_block_static_verify/manifest.txt"),
    )?)?;
    let cases_matched = parse_u64(required(
        verify,
        "matched",
        Path::new("native_block_static_verify/manifest.txt"),
    )?)?;

    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=whole_program_report")?;
    writeln!(file, "analysis_kind=whole_program_rust_translation_report")?;
    writeln!(
        file,
        "translation_strategy=static_cfg_to_native_units_with_rust_tooling_and_oracle_verification"
    )?;
    writeln!(file, "runtime_6502_interpreter_for_translated_units=0")?;
    writeln!(file, "source_rom_sha256={}", disasm["sha256"])?;
    writeln!(file, "mapper={}", disasm["mapper"])?;
    writeln!(file, "prg_size={}", disasm["prg_size"])?;
    writeln!(
        file,
        "static_label_candidates={}",
        static_summary["label_count"]
    )?;
    writeln!(
        file,
        "known_opcode_label_candidates={}",
        static_summary["known_opcode_label_count"]
    )?;
    writeln!(
        file,
        "data_or_unknown_label_candidates={}",
        static_summary["data_or_unknown_label_count"]
    )?;
    writeln!(
        file,
        "replay_covered_labels={}",
        static_summary["covered_label_count"]
    )?;
    writeln!(
        file,
        "replay_covered_known_opcode_labels={}",
        static_summary
            .get("covered_known_opcode_label_count")
            .unwrap_or(&static_summary["covered_label_count"])
    )?;
    writeln!(
        file,
        "static_reachable_label_count={}",
        static_summary["static_reachable_label_count"]
    )?;
    writeln!(
        file,
        "static_reachable_uncovered_known_opcode_label_count={}",
        static_summary["static_reachable_uncovered_known_opcode_label_count"]
    )?;
    writeln!(file, "whole_program_known_reachable_units={}", units.len())?;
    writeln!(file, "oracle_verified_native_units={generated_units}")?;
    writeln!(file, "remaining_known_reachable_units={remaining_units}")?;
    writeln!(
        file,
        "remaining_replay_covered_unselected_units={replay_unselected}"
    )?;
    writeln!(file, "remaining_planned_frontier_units={planned}")?;
    writeln!(file, "remaining_unplanned_or_data_split_units={unplanned}")?;
    writeln!(file, "remaining_classified_units={}", remaining.total)?;
    writeln!(
        file,
        "remaining_replay_covered_needs_block_split={}",
        remaining.replay_covered_needs_split
    )?;
    writeln!(
        file,
        "remaining_inside_verified_native_block_span={}",
        remaining.inside_verified_native_span
    )?;
    writeln!(
        file,
        "remaining_entry_plan_leaf_return_or_interrupt={}",
        remaining.entry_plan_leaf_return_or_interrupt
    )?;
    writeln!(
        file,
        "remaining_entry_plan_control_flow={}",
        remaining.entry_plan_control_flow
    )?;
    writeln!(
        file,
        "remaining_entry_plan_calls_subroutine={}",
        remaining.entry_plan_calls_subroutine
    )?;
    writeln!(
        file,
        "remaining_entry_plan_straight_line_or_data={}",
        remaining.entry_plan_straight_line_or_data
    )?;
    writeln!(
        file,
        "remaining_entry_plan_other={}",
        remaining.entry_plan_other
    )?;
    writeln!(
        file,
        "remaining_not_in_static_entry_plan={}",
        remaining.not_in_static_entry_plan
    )?;
    writeln!(
        file,
        "native_units_per_10000_known_reachable={}",
        ratio_per_10000(generated_units, units.len() as u64)
    )?;
    writeln!(file, "native_block_count={}", codegen["block_count"])?;
    writeln!(file, "semantic_matched_total={}", semantic["matched_total"])?;
    writeln!(
        file,
        "semantic_replay_matched={}",
        semantic["replay_matched"]
    )?;
    writeln!(
        file,
        "semantic_static_leaf_matched={}",
        semantic["static_leaf_matched"]
    )?;
    writeln!(
        file,
        "semantic_static_handoff_matched={}",
        semantic["static_handoff_matched"]
    )?;
    writeln!(
        file,
        "semantic_static_branch_matched={}",
        semantic["static_branch_matched"]
    )?;
    writeln!(
        file,
        "semantic_static_jsr_matched={}",
        semantic["static_jsr_matched"]
    )?;
    writeln!(
        file,
        "semantic_static_return_matched={}",
        semantic["static_return_matched"]
    )?;
    writeln!(file, "oracle_case_count={case_count}")?;
    writeln!(file, "oracle_cases_matched={cases_matched}")?;
    writeln!(file, "oracle_mismatches={}", verify["mismatches"])?;
    writeln!(
        file,
        "oracle_external_write_mismatches={}",
        verify["external_write_mismatches"]
    )?;
    writeln!(
        file,
        "oracle_match_rate_per_10000={}",
        ratio_per_10000(cases_matched, case_count)
    )?;
    writeln!(
        file,
        "native_codegen_supported_opcode_definitions={}",
        SUPPORTED_NATIVE_OPCODES.len()
    )?;
    writeln!(file, "linear_disasm_opcode_count={}", opcode_stats.len())?;
    writeln!(
        file,
        "linear_disasm_rows_with_native_codegen_support={opcode_rows_supported}"
    )?;
    writeln!(
        file,
        "linear_disasm_rows_without_native_codegen_support={opcode_rows_unsupported}"
    )?;
    writeln!(
        file,
        "translation_units=whole_program_translation_units.tsv"
    )?;
    writeln!(file, "remaining_units=whole_program_remaining_units.tsv")?;
    writeln!(file, "opcode_support=whole_program_opcode_support.tsv")?;
    writeln!(file, "opcode_rows=linear_disasm_opcode_rows.tsv")?;
    writeln!(
        file,
        "supported_opcodes=native_codegen_supported_opcodes.txt"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=whole_program_report")?;
    writeln!(file, "summary=whole_program_summary.txt")?;
    writeln!(
        file,
        "translation_units=whole_program_translation_units.tsv"
    )?;
    writeln!(file, "remaining_units=whole_program_remaining_units.tsv")?;
    writeln!(file, "opcode_support=whole_program_opcode_support.tsv")?;
    writeln!(file, "opcode_rows=linear_disasm_opcode_rows.tsv")?;
    writeln!(
        file,
        "supported_opcodes=native_codegen_supported_opcodes.txt"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_report_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn parses_disasm_rows_without_regex() {
        assert_eq!(
            parse_disasm_opcode_line("  ; FCB7  18        CLC "),
            Some(("18".to_string(), "CLC".to_string()))
        );
        assert_eq!(
            parse_disasm_opcode_line("  ; BEEF  A5 10     LDA $10"),
            Some(("A5".to_string(), "LDA".to_string()))
        );
        assert_eq!(
            parse_disasm_opcode_line("  ; 8015  6F        .db $6F"),
            Some(("6F".to_string(), ".db".to_string()))
        );
    }

    #[test]
    fn writes_whole_program_report() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("report");

        write(
            &build.join("disasm/manifest.txt"),
            "sha256=smoke\nmapper=4\nprg_size=131072\ncomplete=1\n",
        );
        write(
            &build.join("disasm/prg_bank_00_8000.asm"),
            "; smoke\n  ; FCB7  18        CLC \n  ; FD6A  60        RTS \n  ; BEEF  A5 10     LDA $10\n",
        );
        write(
            &build.join("static_cfg/static_cfg_summary.txt"),
            "label_count=5\nknown_opcode_label_count=5\ndata_or_unknown_label_count=0\ncovered_label_count=1\ncovered_known_opcode_label_count=1\nstatic_reachable_label_count=5\nstatic_reachable_uncovered_known_opcode_label_count=4\ncomplete=1\n",
        );
        write(
            &build.join("static_cfg/static_reachable_labels.tsv"),
            "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tcovered\treplay_count\treplays\tstatic_reachable\treachable_from\treachable_via\tmapped_in_edges\treachable_in_edges\tmapped_out_edges\tfile\nfixed\t7\tFCB7\t1FCB7\tL_FCB7\t18\tCLC\t1\t1\ttitle_idle\t1\tvector_reset\tstatic\t1\t1\t1\tfixed.asm\nfixed\t7\tFD6A\t1FD6A\tL_FD6A\t60\tRTS\t0\t0\t\t1\tvector_reset\tstatic\t1\t1\t0\tfixed.asm\nfixed\t7\tFD6C\t1FD6C\tL_FD6C\tA5\tLDA\t0\t0\t\t1\tvector_reset\tstatic\t1\t1\t0\tfixed.asm\nfixed\t7\tBEEF\t1BEEF\tL_BEEF\tA5\tLDA\t0\t0\t\t1\tvector_reset\tstatic\t1\t1\t1\tfixed.asm\nfixed\t7\tCAFE\t1CAFE\tL_CAFE\t85\tSTA\t0\t0\t\t1\tvector_reset\tstatic\t1\t1\t0\tfixed.asm\n",
        );
        write(
            &build.join("static_entry_plan/static_entry_plan.tsv"),
            "rank\tgap_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tsource_edge_count\tsource_labels\tsource_instruction_cpu_addrs\tsource_edge_types\toutgoing_edge_types\tstatic_shape\trecommended_next_step\tfile\n1\t1\t1\tfixed\t7\tBEEF\t1BEEF\tL_BEEF\tA5\tLDA\t1\t1\t1\tvector_reset\t1\tL_FCB7\tFCB7\tfallthrough\tfallthrough\tstraight_line_or_data\ttarget_replay_or_synthetic_handoff_state\tfixed.asm\n",
        );
        write(
            &build.join("static_handoff_plan/static_handoff_plan.tsv"),
            "rank\tfrontier_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tstatic_shape\tblocking_reason\tnext_action\tsource_edge_count\tsource_edge_types\toutgoing_edge_types\tmapped_in_edges\treachable_in_edges\treachable_from\tfile\n1\t3\t1\tfixed\t7\tBEEF\t1BEEF\tL_BEEF\tA5\tLDA\tstraight_line_or_data\tneeds_synthetic_handoff_state\tgenerate_linear_handoff_case\t1\tfallthrough\tfallthrough\t1\t1\tvector_reset\tfixed.asm\n",
        );
        write(
            &build.join("native_block_plan_static/native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n1\tFCB7\t1FCB7\t1\t18\t1\ttitle_idle\t1\t1\t0\t0\t0\t0\t1\treplay\n2\tFD6A\t1FD6A\t4\t60\t0\tstatic_entry_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_leaf\n",
        );
        write(
            &build.join("native_block_plan_static/native_block_static_merge_summary.txt"),
            "merged_block_count=2\ncomplete=1\n",
        );
        write(
            &build.join("native_block_codegen_static/manifest.txt"),
            "block_count=2\nunsupported_count=0\ncomplete=1\n",
        );
        write(
            &build.join("native_block_static_verify/manifest.txt"),
            "case_count=8\nmatched=8\nmismatches=0\nexternal_write_mismatches=0\ncomplete=1\n",
        );
        write(
            &build.join("semantic_match_report/semantic_match_summary.txt"),
            "matched_total=2\nreplay_matched=1\nstatic_leaf_matched=1\nstatic_handoff_matched=0\nstatic_branch_matched=0\nstatic_jsr_matched=0\nstatic_return_matched=0\ncomplete=1\n",
        );

        run(&build, &out).unwrap();

        let summary = fs::read_to_string(out.join("whole_program_summary.txt")).unwrap();
        assert!(summary.contains("analysis_kind=whole_program_rust_translation_report\n"));
        assert!(summary.contains("whole_program_known_reachable_units=5\n"));
        assert!(summary.contains("oracle_verified_native_units=2\n"));
        assert!(summary.contains("remaining_planned_frontier_units=1\n"));
        assert!(summary.contains("remaining_unplanned_or_data_split_units=2\n"));
        assert!(summary.contains("remaining_classified_units=3\n"));
        assert!(summary.contains("remaining_inside_verified_native_block_span=1\n"));
        assert!(summary.contains("remaining_entry_plan_straight_line_or_data=1\n"));
        assert!(summary.contains("remaining_not_in_static_entry_plan=1\n"));
        assert!(summary.contains("oracle_mismatches=0\n"));
        assert!(summary.contains("linear_disasm_opcode_count=3\n"));

        let units = fs::read_to_string(out.join("whole_program_translation_units.tsv")).unwrap();
        assert!(units.contains(
            "\tBEEF\t1BEEF\tL_BEEF\tA5\tLDA\t0\t0\t0\t0\t\t\tgenerate_linear_handoff_case\n"
        ));
        let remaining = fs::read_to_string(out.join("whole_program_remaining_units.tsv")).unwrap();
        assert!(remaining.contains(
            "FD6C\t1FD6C\tL_FD6C\tA5\tLDA\t0\t0\texpand_static_entry_plan_or_confirm_data\tinside_verified_native_block_span\tFD6A\t1FD6A\t2\t4\tstatic_leaf"
        ));
        assert!(remaining.contains(
            "BEEF\t1BEEF\tL_BEEF\tA5\tLDA\t0\t0\tgenerate_linear_handoff_case\tentry_plan_straight_line_or_data"
        ));
        assert!(remaining.contains(
            "CAFE\t1CAFE\tL_CAFE\t85\tSTA\t0\t0\texpand_static_entry_plan_or_confirm_data\tnot_in_static_entry_plan"
        ));

        let opcodes = fs::read_to_string(out.join("whole_program_opcode_support.tsv")).unwrap();
        assert!(opcodes.contains("A5\tLDA\t1\t1\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
