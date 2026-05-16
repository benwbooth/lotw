use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
struct CoverageInfo {
    replays: Vec<String>,
    seen: HashSet<String>,
}

#[derive(Debug, Clone)]
struct FileContext {
    kind: String,
    bank: u64,
    prg_base: u64,
}

#[derive(Debug, Clone)]
struct StaticLabel {
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    file: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    covered: u64,
    replay_count: u64,
    replays: String,
}

#[derive(Debug, Clone)]
struct StaticEdge {
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    opcode: String,
    mnemonic: String,
    edge_type: String,
    target_cpu_addr: String,
    target_prg_offset: String,
    certainty: String,
}

#[derive(Debug, Clone)]
struct LabelEdge {
    source_prg_offset: String,
    source_label: String,
    source_cpu_addr: String,
    edge_type: String,
    target_prg_offset: String,
    target_cpu_addr: String,
    certainty: String,
    opcode: String,
    mnemonic: String,
    instruction_prg_offset: String,
    instruction_cpu_addr: String,
    bank_kind: String,
    bank: u64,
}

#[derive(Debug, Clone)]
struct CoverageGap {
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    file: String,
}

#[derive(Debug, Default)]
struct StaticCfgData {
    labels: Vec<StaticLabel>,
    edges: Vec<StaticEdge>,
    label_edges: Vec<LabelEdge>,
    gaps: Vec<CoverageGap>,
    gap_mnemonic_count: BTreeMap<(String, String), u64>,
    summary: StaticSummary,
}

#[derive(Debug, Default)]
struct StaticSummary {
    label_count: u64,
    banked_label_count: u64,
    fixed_label_count: u64,
    known_opcode_label_count: u64,
    data_or_unknown_label_count: u64,
    covered_label_count: u64,
    uncovered_label_count: u64,
    covered_known_opcode_label_count: u64,
    uncovered_known_opcode_label_count: u64,
    covered_data_or_unknown_label_count: u64,
    uncovered_data_or_unknown_label_count: u64,
    instruction_count: u64,
    unknown_opcode_or_data_count: u64,
    edge_count: u64,
    static_reachable_label_count: u64,
    static_reachable_uncovered_label_count: u64,
    static_reachable_uncovered_known_opcode_label_count: u64,
}

#[derive(Debug, Default)]
struct ReachInfo {
    sources: Vec<String>,
    source_seen: HashSet<String>,
    reasons: Vec<String>,
    reason_seen: HashSet<String>,
}

#[derive(Debug, Clone)]
struct GraphEdge {
    src: String,
    dst: String,
    edge_type: String,
}

#[derive(Debug, Clone)]
struct ReachableGap {
    priority: u64,
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    known_opcode: u64,
    mapped_in_edges: u64,
    reachable_in_edges: u64,
    reachable_from: String,
    file: String,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    replays: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if replays.is_empty() {
        return Err("static_cfg_gap: at least one replay is required".into());
    }

    let disasm_dir = build_dir.join("disasm");
    let plan = build_dir
        .join("block_translation_plan")
        .join("block_translation_plan.tsv");
    let manifest_in = disasm_dir.join("manifest.txt");
    require_path(&disasm_dir)?;
    require_path(&plan)?;
    require_path(&manifest_in)?;

    let fixed_bank = read_fixed_bank(&manifest_in)?;
    let asm_files = find_asm_files(&disasm_dir)?;
    if asm_files.is_empty() {
        return Err(format!(
            "static_cfg_gap: no disassembly files under {}",
            disasm_dir.display()
        )
        .into());
    }

    fs::create_dir_all(out_dir)?;

    let coverage = read_coverage(&plan)?;
    let mut data = parse_disassembly(&asm_files, &coverage, fixed_bank)?;
    let ranked_gaps = rank_coverage_gaps(&data.gaps, &data.edges);
    let (reachable_labels, reachable_gaps, reach_counts) =
        compute_reachability(&manifest_in, &data.labels, &data.label_edges)?;
    data.summary.static_reachable_label_count = reach_counts.0;
    data.summary.static_reachable_uncovered_label_count = reach_counts.1;
    data.summary
        .static_reachable_uncovered_known_opcode_label_count = reach_counts.2;

    write_static_labels(&out_dir.join("static_labels.tsv"), &data.labels)?;
    write_static_edges(&out_dir.join("static_edges.tsv"), &data.edges)?;
    write_static_label_edges(&out_dir.join("static_label_edges.tsv"), &data.label_edges)?;
    write_coverage_gaps(&out_dir.join("coverage_gap.tsv"), &data.gaps)?;
    write_gap_summary(
        &out_dir.join("coverage_gap_summary.tsv"),
        &data.gap_mnemonic_count,
    )?;
    write_ranked_gaps(&out_dir.join("coverage_gap_ranked.tsv"), &ranked_gaps)?;
    write_reachable_labels(
        &out_dir.join("static_reachable_labels.tsv"),
        &reachable_labels,
    )?;
    write_reachable_gaps(&out_dir.join("coverage_gap_reachable.tsv"), &reachable_gaps)?;
    write_summary(
        &out_dir.join("static_cfg_summary.txt"),
        replays,
        &data.summary,
    )?;
    write_manifest(&out_dir.join("manifest.txt"), &disasm_dir, &plan)?;

    println!("static_cfg_gap: wrote {}", out_dir.display());
    Ok(())
}

fn require_path(path: &Path) -> io::Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_cfg_gap: missing input: {}", path.display()),
        ))
    }
}

fn read_fixed_bank(path: &Path) -> io::Result<u64> {
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("prg_16k_banks=") {
            let banks = value.parse::<u64>().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}: invalid prg_16k_banks: {err}", path.display()),
                )
            })?;
            return banks.checked_sub(1).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}: prg_16k_banks must be positive", path.display()),
                )
            });
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "static_cfg_gap: missing prg_16k_banks in {}",
            path.display()
        ),
    ))
}

fn find_asm_files(disasm_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut banked = Vec::new();
    let mut fixed = Vec::new();
    for entry in fs::read_dir(disasm_dir)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if parse_file_context_from_name(name).is_some() {
            if name.starts_with("prg_bank_") {
                banked.push(path);
            } else {
                fixed.push(path);
            }
        }
    }
    banked.sort_by_key(|path| path.file_name().map(|name| name.to_os_string()));
    fixed.sort_by_key(|path| path.file_name().map(|name| name.to_os_string()));
    banked.extend(fixed);
    Ok(banked)
}

fn parse_file_context(path: &Path) -> FileContext {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    parse_file_context_from_name(name).unwrap_or_else(|| FileContext {
        kind: "unknown".to_string(),
        bank: 0,
        prg_base: 0,
    })
}

fn parse_file_context_from_name(name: &str) -> Option<FileContext> {
    if let Some(bank) = name
        .strip_prefix("prg_bank_")
        .and_then(|value| value.strip_suffix("_8000.asm"))
        .and_then(|value| value.parse::<u64>().ok())
    {
        return Some(FileContext {
            kind: "banked".to_string(),
            bank,
            prg_base: bank * 0x4000,
        });
    }
    if let Some(bank) = name
        .strip_prefix("fixed_bank_")
        .and_then(|value| value.strip_suffix("_c000.asm"))
        .and_then(|value| value.parse::<u64>().ok())
    {
        return Some(FileContext {
            kind: "fixed".to_string(),
            bank,
            prg_base: bank * 0x4000,
        });
    }
    None
}

fn read_coverage(path: &Path) -> io::Result<HashMap<String, CoverageInfo>> {
    let text = fs::read_to_string(path)?;
    let mut coverage = HashMap::<String, CoverageInfo>::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 4 {
            return invalid_tsv(path, line_no + 1, fields.len(), 4);
        }
        let key = fields[3].to_uppercase();
        let replay = fields[0].to_string();
        let info = coverage.entry(key).or_default();
        if info.seen.insert(replay.clone()) {
            info.replays.push(replay);
        }
    }
    Ok(coverage)
}

fn parse_disassembly(
    asm_files: &[PathBuf],
    coverage: &HashMap<String, CoverageInfo>,
    fixed_bank: u64,
) -> io::Result<StaticCfgData> {
    let mut data = StaticCfgData::default();

    for path in asm_files {
        let context = parse_file_context(path);
        let text = fs::read_to_string(path)?;
        let mut pending_label = String::new();
        let mut active_label = String::new();
        let mut active_label_offset = String::new();
        let mut active_label_cpu = String::new();
        let file_name = path.display().to_string();

        for line in text.lines() {
            if let Some(label) = parse_label_line(line) {
                pending_label = label;
                continue;
            }
            let Some(instruction) = parse_instruction_line(line) else {
                continue;
            };

            handle_instruction(
                &mut data,
                &context,
                fixed_bank,
                coverage,
                &file_name,
                &mut pending_label,
                &mut active_label,
                &mut active_label_offset,
                &mut active_label_cpu,
                &instruction,
            );
        }
    }

    Ok(data)
}

#[derive(Debug)]
struct Instruction {
    addr: u64,
    opcode: String,
    byte_count: u64,
    mnemonic: String,
    operand: String,
}

fn parse_label_line(line: &str) -> Option<String> {
    let token = line.split_whitespace().next()?;
    let label = token.strip_suffix(':')?;
    if label.len() == 6 && label.starts_with("L_") && is_hex(&label[2..]) {
        Some(label.to_string())
    } else {
        None
    }
}

fn parse_instruction_line(line: &str) -> Option<Instruction> {
    let line = line.strip_prefix("  ; ")?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 3 || parts[0].len() != 4 || !is_hex(parts[0]) {
        return None;
    }
    let addr = hex_to_u64(parts[0])?;
    let opcode = parts[1].to_uppercase();
    let byte_count = parts
        .iter()
        .skip(1)
        .take_while(|part| part.len() == 2 && is_hex(part))
        .count();
    if byte_count == 0 {
        return None;
    }
    let mnemonic_index = 1 + byte_count;
    let mnemonic = parts.get(mnemonic_index)?.to_string();
    let operand = parts
        .iter()
        .skip(mnemonic_index + 1)
        .copied()
        .collect::<Vec<_>>()
        .join(" ");
    Some(Instruction {
        addr,
        opcode,
        byte_count: byte_count as u64,
        mnemonic,
        operand,
    })
}

#[allow(clippy::too_many_arguments)]
fn handle_instruction(
    data: &mut StaticCfgData,
    context: &FileContext,
    fixed_bank: u64,
    coverage: &HashMap<String, CoverageInfo>,
    file_name: &str,
    pending_label: &mut String,
    active_label: &mut String,
    active_label_offset: &mut String,
    active_label_cpu: &mut String,
    instruction: &Instruction,
) {
    let Some(prg) = current_prg_offset(context, instruction.addr) else {
        return;
    };
    data.summary.instruction_count += 1;
    if instruction.mnemonic == ".db" {
        data.summary.unknown_opcode_or_data_count += 1;
    }

    if !pending_label.is_empty() {
        let key = upper_hex(prg, 5);
        let replay_info = coverage.get(&key);
        let covered = u64::from(replay_info.is_some());
        let replay_count = replay_info
            .map(|info| info.replays.len() as u64)
            .unwrap_or(0);
        let replays = replay_info
            .map(|info| info.replays.join(","))
            .unwrap_or_default();

        data.labels.push(StaticLabel {
            bank_kind: context.kind.clone(),
            bank: context.bank,
            cpu_addr: upper_hex(instruction.addr, 4),
            prg_offset: key.clone(),
            file: file_name.to_string(),
            label: pending_label.clone(),
            first_opcode: instruction.opcode.clone(),
            mnemonic: instruction.mnemonic.clone(),
            covered,
            replay_count,
            replays,
        });

        data.summary.label_count += 1;
        if instruction.mnemonic == ".db" {
            data.summary.data_or_unknown_label_count += 1;
        } else {
            data.summary.known_opcode_label_count += 1;
        }
        match context.kind.as_str() {
            "banked" => data.summary.banked_label_count += 1,
            "fixed" => data.summary.fixed_label_count += 1,
            _ => {}
        }
        if covered == 1 {
            data.summary.covered_label_count += 1;
            if instruction.mnemonic == ".db" {
                data.summary.covered_data_or_unknown_label_count += 1;
            } else {
                data.summary.covered_known_opcode_label_count += 1;
            }
        } else {
            data.summary.uncovered_label_count += 1;
            if instruction.mnemonic == ".db" {
                data.summary.uncovered_data_or_unknown_label_count += 1;
            } else {
                data.summary.uncovered_known_opcode_label_count += 1;
            }
            *data
                .gap_mnemonic_count
                .entry((instruction.mnemonic.clone(), instruction.opcode.clone()))
                .or_default() += 1;
            data.gaps.push(CoverageGap {
                bank_kind: context.kind.clone(),
                bank: context.bank,
                cpu_addr: upper_hex(instruction.addr, 4),
                prg_offset: key.clone(),
                label: pending_label.clone(),
                first_opcode: instruction.opcode.clone(),
                mnemonic: instruction.mnemonic.clone(),
                file: file_name.to_string(),
            });
        }

        *active_label = pending_label.clone();
        *active_label_offset = key;
        *active_label_cpu = upper_hex(instruction.addr, 4);
        pending_label.clear();
    }

    emit_instruction_edges(
        data,
        context,
        fixed_bank,
        active_label,
        active_label_offset,
        active_label_cpu,
        instruction,
    );
}

fn emit_instruction_edges(
    data: &mut StaticCfgData,
    context: &FileContext,
    fixed_bank: u64,
    active_label: &str,
    active_label_offset: &str,
    active_label_cpu: &str,
    instruction: &Instruction,
) {
    if is_branch_mnemonic(&instruction.mnemonic) {
        let target = parse_target(&instruction.operand);
        let target_offset = target
            .and_then(|target| map_target(context, fixed_bank, target))
            .unwrap_or_default();
        let certainty = target
            .map(|target| target_certainty(context, target, &instruction.operand))
            .unwrap_or_else(|| "unresolved".to_string());
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            "branch_target",
            target,
            target_offset,
            certainty,
        );
        let fallthrough = instruction.addr + instruction.byte_count;
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            "branch_fallthrough",
            Some(fallthrough),
            map_target(context, fixed_bank, fallthrough).unwrap_or_default(),
            target_certainty(context, fallthrough, &instruction.operand),
        );
    } else if instruction.mnemonic == "JMP" {
        let target = parse_target(&instruction.operand);
        let target_offset = target
            .and_then(|target| map_target(context, fixed_bank, target))
            .unwrap_or_default();
        let certainty = target
            .map(|target| target_certainty(context, target, &instruction.operand))
            .unwrap_or_else(|| "unresolved".to_string());
        let edge_type = if instruction.operand.starts_with('(') {
            "indirect_jump"
        } else {
            "jump_target"
        };
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            edge_type,
            target,
            target_offset,
            certainty,
        );
    } else if instruction.mnemonic == "JSR" {
        let target = parse_target(&instruction.operand);
        let target_offset = target
            .and_then(|target| map_target(context, fixed_bank, target))
            .unwrap_or_default();
        let certainty = target
            .map(|target| target_certainty(context, target, &instruction.operand))
            .unwrap_or_else(|| "unresolved".to_string());
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            "call_target",
            target,
            target_offset,
            certainty,
        );
        let return_addr = instruction.addr + instruction.byte_count;
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            "call_return",
            Some(return_addr),
            map_target(context, fixed_bank, return_addr).unwrap_or_default(),
            target_certainty(context, return_addr, &instruction.operand),
        );
    } else if matches!(instruction.mnemonic.as_str(), "RTS" | "RTI" | "BRK") {
        emit_edge(
            data,
            context,
            active_label,
            active_label_offset,
            active_label_cpu,
            instruction,
            "stop",
            None,
            String::new(),
            "terminator".to_string(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_edge(
    data: &mut StaticCfgData,
    context: &FileContext,
    active_label: &str,
    active_label_offset: &str,
    active_label_cpu: &str,
    instruction: &Instruction,
    edge_type: &str,
    target_addr: Option<u64>,
    target_prg_offset: String,
    certainty: String,
) {
    let instruction_prg = current_prg_offset(context, instruction.addr).unwrap_or(0);
    let target_cpu_addr = target_addr
        .map(|addr| upper_hex(addr, 4))
        .unwrap_or_default();
    data.edges.push(StaticEdge {
        bank_kind: context.kind.clone(),
        bank: context.bank,
        cpu_addr: upper_hex(instruction.addr, 4),
        prg_offset: upper_hex(instruction_prg, 5),
        opcode: instruction.opcode.clone(),
        mnemonic: instruction.mnemonic.clone(),
        edge_type: edge_type.to_string(),
        target_cpu_addr: target_cpu_addr.clone(),
        target_prg_offset: target_prg_offset.clone(),
        certainty: certainty.clone(),
    });
    data.label_edges.push(LabelEdge {
        source_prg_offset: active_label_offset.to_string(),
        source_label: active_label.to_string(),
        source_cpu_addr: active_label_cpu.to_string(),
        edge_type: edge_type.to_string(),
        target_prg_offset,
        target_cpu_addr,
        certainty,
        opcode: instruction.opcode.clone(),
        mnemonic: instruction.mnemonic.clone(),
        instruction_prg_offset: upper_hex(instruction_prg, 5),
        instruction_cpu_addr: upper_hex(instruction.addr, 4),
        bank_kind: context.kind.clone(),
        bank: context.bank,
    });
    data.summary.edge_count += 1;
}

fn rank_coverage_gaps(gaps: &[CoverageGap], edges: &[StaticEdge]) -> Vec<RankedGap> {
    let mut incoming = HashMap::<String, EdgeCounts>::new();
    for edge in edges {
        if edge.target_prg_offset.is_empty() || edge.certainty != "mapped" {
            continue;
        }
        let counts = incoming.entry(edge.target_prg_offset.clone()).or_default();
        counts.incoming += 1;
        match edge.edge_type.as_str() {
            "call_target" | "call_return" => counts.calls += 1,
            "branch_target" | "branch_fallthrough" => counts.branches += 1,
            "jump_target" => counts.jumps += 1,
            _ => {}
        }
    }

    let mut ranked = gaps
        .iter()
        .map(|gap| {
            let counts = incoming.get(&gap.prg_offset).cloned().unwrap_or_default();
            let known = u64::from(gap.mnemonic != ".db");
            let ordinary = u64::from(!matches!(gap.mnemonic.as_str(), "BRK" | "RTI"));
            let priority = counts.incoming * 1_000_000 + known * 1_000 + ordinary * 100;
            RankedGap {
                priority,
                bank_kind: gap.bank_kind.clone(),
                bank: gap.bank,
                cpu_addr: gap.cpu_addr.clone(),
                prg_offset: gap.prg_offset.clone(),
                label: gap.label.clone(),
                first_opcode: gap.first_opcode.clone(),
                mnemonic: gap.mnemonic.clone(),
                known_opcode: known,
                mapped_in_edges: counts.incoming,
                call_in_edges: counts.calls,
                branch_in_edges: counts.branches,
                jump_in_edges: counts.jumps,
                file: gap.file.clone(),
            }
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|lhs, rhs| {
        rhs.priority
            .cmp(&lhs.priority)
            .then_with(|| rhs.mapped_in_edges.cmp(&lhs.mapped_in_edges))
            .then_with(|| lhs.mnemonic.cmp(&rhs.mnemonic))
            .then_with(|| lhs.prg_offset.cmp(&rhs.prg_offset))
    });
    ranked
}

#[derive(Debug, Default, Clone)]
struct EdgeCounts {
    incoming: u64,
    calls: u64,
    branches: u64,
    jumps: u64,
}

#[derive(Debug, Clone)]
struct RankedGap {
    priority: u64,
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    known_opcode: u64,
    mapped_in_edges: u64,
    call_in_edges: u64,
    branch_in_edges: u64,
    jump_in_edges: u64,
    file: String,
}

type ReachabilityOutput = (Vec<ReachableLabel>, Vec<ReachableGap>, (u64, u64, u64));

fn compute_reachability(
    manifest: &Path,
    labels: &[StaticLabel],
    label_edges: &[LabelEdge],
) -> io::Result<ReachabilityOutput> {
    let mut reach = HashMap::<String, ReachInfo>::new();
    read_vector_sources(manifest, &mut reach)?;

    let mut label_exists = HashSet::<String>::new();
    let mut linear_edges = Vec::<(String, String)>::new();
    let mut last_file = String::new();
    let mut last_key = String::new();
    for row in labels {
        let key = row.prg_offset.to_uppercase();
        label_exists.insert(key.clone());
        if row.covered == 1 {
            add_source(&mut reach, &key, "dynamic");
        }
        if row.file == last_file && !last_key.is_empty() {
            linear_edges.push((last_key.clone(), key.clone()));
        }
        last_file = row.file.clone();
        last_key = key;
    }

    let mut graph = Vec::<GraphEdge>::new();
    let mut hard_stop = HashSet::<String>::new();
    let mut mapped_in = HashMap::<String, u64>::new();
    let mut mapped_out = HashMap::<String, u64>::new();

    for edge in label_edges {
        let src = edge.source_prg_offset.to_uppercase();
        let dst = edge.target_prg_offset.to_uppercase();
        if src.is_empty() {
            continue;
        }
        if matches!(
            edge.edge_type.as_str(),
            "stop" | "jump_target" | "indirect_jump"
        ) {
            hard_stop.insert(src.clone());
        }
        if edge.certainty == "mapped" && !dst.is_empty() {
            add_graph_edge(
                &mut graph,
                &mut mapped_in,
                &mut mapped_out,
                src,
                dst,
                &edge.edge_type,
            );
        }
    }

    for (src, dst) in linear_edges {
        if !hard_stop.contains(&src) {
            add_graph_edge(
                &mut graph,
                &mut mapped_in,
                &mut mapped_out,
                src,
                dst,
                "linear_next_label",
            );
        }
    }

    let mut reachable = reach
        .keys()
        .filter(|key| label_exists.contains(*key))
        .cloned()
        .collect::<HashSet<_>>();

    let mut changed = true;
    while changed {
        changed = false;
        for edge in &graph {
            if !reachable.contains(&edge.src) || !label_exists.contains(&edge.dst) {
                continue;
            }
            if reachable.insert(edge.dst.clone()) {
                changed = true;
            }
            let sources = reach
                .get(&edge.src)
                .map(|info| info.sources.clone())
                .unwrap_or_default();
            for source in sources {
                if add_source(&mut reach, &edge.dst, &source) {
                    changed = true;
                }
            }
            add_reason(&mut reach, &edge.dst, &edge.edge_type);
        }
    }

    let mut reachable_in = HashMap::<String, u64>::new();
    for edge in &graph {
        if reachable.contains(&edge.src) && reachable.contains(&edge.dst) {
            *reachable_in.entry(edge.dst.clone()).or_default() += 1;
        }
    }

    let mut reachable_label_count = 0;
    let mut reachable_uncovered_count = 0;
    let mut reachable_uncovered_known_count = 0;
    let mut reachable_labels = Vec::new();
    let mut reachable_gaps = Vec::new();

    for row in labels {
        let key = row.prg_offset.to_uppercase();
        let is_reachable = u64::from(reachable.contains(&key));
        if is_reachable == 1 {
            reachable_label_count += 1;
            if row.covered == 0 {
                reachable_uncovered_count += 1;
                let known = u64::from(row.mnemonic != ".db");
                let ordinary = u64::from(!matches!(row.mnemonic.as_str(), "BRK" | "RTI"));
                if known == 1 {
                    reachable_uncovered_known_count += 1;
                }
                let reachable_in_count = *reachable_in.get(&key).unwrap_or(&0);
                let mapped_in_count = *mapped_in.get(&key).unwrap_or(&0);
                let priority = reachable_in_count * 1_000_000
                    + mapped_in_count * 10_000
                    + known * 1_000
                    + ordinary * 100;
                reachable_gaps.push(ReachableGap {
                    priority,
                    bank_kind: row.bank_kind.clone(),
                    bank: row.bank,
                    cpu_addr: row.cpu_addr.clone(),
                    prg_offset: key.clone(),
                    label: row.label.clone(),
                    first_opcode: row.first_opcode.clone(),
                    mnemonic: row.mnemonic.clone(),
                    known_opcode: known,
                    mapped_in_edges: mapped_in_count,
                    reachable_in_edges: reachable_in_count,
                    reachable_from: reach_sources(&reach, &key),
                    file: row.file.clone(),
                });
            }
        }

        reachable_labels.push(ReachableLabel {
            bank_kind: row.bank_kind.clone(),
            bank: row.bank,
            cpu_addr: row.cpu_addr.clone(),
            prg_offset: key.clone(),
            label: row.label.clone(),
            first_opcode: row.first_opcode.clone(),
            mnemonic: row.mnemonic.clone(),
            covered: row.covered,
            replay_count: row.replay_count,
            replays: row.replays.clone(),
            static_reachable: is_reachable,
            reachable_from: reach_sources(&reach, &key),
            reachable_via: reach_reasons(&reach, &key),
            mapped_in_edges: *mapped_in.get(&key).unwrap_or(&0),
            reachable_in_edges: *reachable_in.get(&key).unwrap_or(&0),
            mapped_out_edges: *mapped_out.get(&key).unwrap_or(&0),
            file: row.file.clone(),
        });
    }

    reachable_gaps.sort_by(|lhs, rhs| {
        rhs.priority
            .cmp(&lhs.priority)
            .then_with(|| rhs.reachable_in_edges.cmp(&lhs.reachable_in_edges))
            .then_with(|| lhs.mnemonic.cmp(&rhs.mnemonic))
            .then_with(|| lhs.prg_offset.cmp(&rhs.prg_offset))
    });

    Ok((
        reachable_labels,
        reachable_gaps,
        (
            reachable_label_count,
            reachable_uncovered_count,
            reachable_uncovered_known_count,
        ),
    ))
}

#[derive(Debug, Clone)]
struct ReachableLabel {
    bank_kind: String,
    bank: u64,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    covered: u64,
    replay_count: u64,
    replays: String,
    static_reachable: u64,
    reachable_from: String,
    reachable_via: String,
    mapped_in_edges: u64,
    reachable_in_edges: u64,
    mapped_out_edges: u64,
    file: String,
}

fn read_vector_sources(manifest: &Path, reach: &mut HashMap<String, ReachInfo>) -> io::Result<()> {
    let text = fs::read_to_string(manifest)?;
    for line in text.lines() {
        let Some((source, value)) = line.split_once('=') else {
            continue;
        };
        if source.starts_with("vector_") && source.ends_with("_prg_offset") {
            if let Some(key) = normalize_offset(value) {
                let source = source.trim_end_matches("_prg_offset");
                add_source(reach, &key, source);
            }
        }
    }
    Ok(())
}

fn add_graph_edge(
    graph: &mut Vec<GraphEdge>,
    mapped_in: &mut HashMap<String, u64>,
    mapped_out: &mut HashMap<String, u64>,
    src: String,
    dst: String,
    edge_type: &str,
) {
    if src.is_empty() || dst.is_empty() {
        return;
    }
    *mapped_out.entry(src.clone()).or_default() += 1;
    *mapped_in.entry(dst.clone()).or_default() += 1;
    graph.push(GraphEdge {
        src,
        dst,
        edge_type: edge_type.to_string(),
    });
}

fn add_source(reach: &mut HashMap<String, ReachInfo>, key: &str, source: &str) -> bool {
    if key.is_empty() || source.is_empty() {
        return false;
    }
    let info = reach.entry(key.to_string()).or_default();
    if info.source_seen.insert(source.to_string()) {
        info.sources.push(source.to_string());
        true
    } else {
        false
    }
}

fn add_reason(reach: &mut HashMap<String, ReachInfo>, key: &str, reason: &str) {
    if key.is_empty() || reason.is_empty() {
        return;
    }
    let info = reach.entry(key.to_string()).or_default();
    if info.reason_seen.insert(reason.to_string()) {
        info.reasons.push(reason.to_string());
    }
}

fn reach_sources(reach: &HashMap<String, ReachInfo>, key: &str) -> String {
    reach
        .get(key)
        .map(|info| info.sources.join(","))
        .unwrap_or_default()
}

fn reach_reasons(reach: &HashMap<String, ReachInfo>, key: &str) -> String {
    reach
        .get(key)
        .map(|info| info.reasons.join(","))
        .unwrap_or_default()
}

fn current_prg_offset(context: &FileContext, addr: u64) -> Option<u64> {
    if context.kind == "banked" && (0x8000..=0xBFFF).contains(&addr) {
        return Some(context.prg_base + addr - 0x8000);
    }
    if context.kind == "fixed" && (0xC000..=0xFFFF).contains(&addr) {
        return Some(context.prg_base + addr - 0xC000);
    }
    None
}

fn map_target(context: &FileContext, fixed_bank: u64, addr: u64) -> Option<String> {
    if (0xC000..=0xFFFF).contains(&addr) {
        return Some(upper_hex(fixed_bank * 0x4000 + addr - 0xC000, 5));
    }
    if (0x8000..=0xBFFF).contains(&addr) && context.kind == "banked" {
        return Some(upper_hex(context.bank * 0x4000 + addr - 0x8000, 5));
    }
    None
}

fn target_certainty(context: &FileContext, addr: u64, operand: &str) -> String {
    if operand.starts_with('(') {
        return "indirect_operand".to_string();
    }
    if (0x8000..=0xBFFF).contains(&addr) && context.kind != "banked" {
        return "banked_window_unknown".to_string();
    }
    if (0x8000..=0xFFFF).contains(&addr) {
        return "mapped".to_string();
    }
    "non_prg_target".to_string()
}

fn parse_target(operand: &str) -> Option<u64> {
    let cleaned = operand
        .chars()
        .filter(|ch| !matches!(ch, ',' | '(' | ')' | '$'))
        .collect::<String>();
    if cleaned.len() == 6 && cleaned.starts_with("L_") && is_hex(&cleaned[2..]) {
        return hex_to_u64(&cleaned[2..]);
    }
    if cleaned.len() == 4 && is_hex(&cleaned) {
        return hex_to_u64(&cleaned);
    }
    None
}

fn normalize_offset(value: &str) -> Option<String> {
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    hex_to_u64(value).map(|value| upper_hex(value, 5))
}

fn is_branch_mnemonic(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "BCC" | "BCS" | "BEQ" | "BMI" | "BNE" | "BPL" | "BVC" | "BVS"
    )
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
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

fn is_hex(value: &str) -> bool {
    value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn hex_to_u64(value: &str) -> Option<u64> {
    u64::from_str_radix(value, 16).ok()
}

fn upper_hex(value: u64, width: usize) -> String {
    format!("{value:0width$X}")
}

fn write_static_labels(path: &Path, rows: &[StaticLabel]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\tfile\tlabel\tfirst_opcode\tmnemonic\tcovered\treplay_count\treplays"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.file,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.covered,
            row.replay_count,
            row.replays
        )?;
    }
    Ok(())
}

fn write_static_edges(path: &Path, rows: &[StaticEdge]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\topcode\tmnemonic\tedge_type\ttarget_cpu_addr\ttarget_prg_offset\tcertainty"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.opcode,
            row.mnemonic,
            row.edge_type,
            row.target_cpu_addr,
            row.target_prg_offset,
            row.certainty
        )?;
    }
    Ok(())
}

fn write_static_label_edges(path: &Path, rows: &[LabelEdge]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "source_prg_offset\tsource_label\tsource_cpu_addr\tedge_type\ttarget_prg_offset\ttarget_cpu_addr\tcertainty\topcode\tmnemonic\tinstruction_prg_offset\tinstruction_cpu_addr\tbank_kind\tbank"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.source_prg_offset,
            row.source_label,
            row.source_cpu_addr,
            row.edge_type,
            row.target_prg_offset,
            row.target_cpu_addr,
            row.certainty,
            row.opcode,
            row.mnemonic,
            row.instruction_prg_offset,
            row.instruction_cpu_addr,
            row.bank_kind,
            row.bank
        )?;
    }
    Ok(())
}

fn write_coverage_gaps(path: &Path, rows: &[CoverageGap]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tfile"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.file
        )?;
    }
    Ok(())
}

fn write_gap_summary(path: &Path, counts: &BTreeMap<(String, String), u64>) -> io::Result<()> {
    let mut rows = counts
        .iter()
        .map(|((mnemonic, opcode), count)| (mnemonic.clone(), opcode.clone(), *count))
        .collect::<Vec<_>>();
    rows.sort_by(|lhs, rhs| {
        rhs.2
            .cmp(&lhs.2)
            .then_with(|| lhs.0.cmp(&rhs.0))
            .then_with(|| lhs.1.cmp(&rhs.1))
    });
    let mut file = fs::File::create(path)?;
    writeln!(file, "mnemonic\tfirst_opcode\tuncovered_label_count")?;
    for (mnemonic, opcode, count) in rows {
        writeln!(file, "{mnemonic}\t{opcode}\t{count}")?;
    }
    Ok(())
}

fn write_ranked_gaps(path: &Path, rows: &[RankedGap]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\tcall_in_edges\tbranch_in_edges\tjump_in_edges\tfile"
    )?;
    for (index, row) in rows.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            row.priority,
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.known_opcode,
            row.mapped_in_edges,
            row.call_in_edges,
            row.branch_in_edges,
            row.jump_in_edges,
            row.file
        )?;
    }
    Ok(())
}

fn write_reachable_labels(path: &Path, rows: &[ReachableLabel]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tcovered\treplay_count\treplays\tstatic_reachable\treachable_from\treachable_via\tmapped_in_edges\treachable_in_edges\tmapped_out_edges\tfile"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.covered,
            row.replay_count,
            row.replays,
            row.static_reachable,
            row.reachable_from,
            row.reachable_via,
            row.mapped_in_edges,
            row.reachable_in_edges,
            row.mapped_out_edges,
            row.file
        )?;
    }
    Ok(())
}

fn write_reachable_gaps(path: &Path, rows: &[ReachableGap]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tfile"
    )?;
    for (index, row) in rows.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            row.priority,
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.known_opcode,
            row.mapped_in_edges,
            row.reachable_in_edges,
            row.reachable_from,
            row.file
        )?;
    }
    Ok(())
}

fn write_summary(path: &Path, replays: &[String], summary: &StaticSummary) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "replays={}", replays.join(" "))?;
    writeln!(file, "label_count={}", summary.label_count)?;
    writeln!(file, "banked_label_count={}", summary.banked_label_count)?;
    writeln!(file, "fixed_label_count={}", summary.fixed_label_count)?;
    writeln!(
        file,
        "known_opcode_label_count={}",
        summary.known_opcode_label_count
    )?;
    writeln!(
        file,
        "data_or_unknown_label_count={}",
        summary.data_or_unknown_label_count
    )?;
    writeln!(file, "covered_label_count={}", summary.covered_label_count)?;
    writeln!(
        file,
        "uncovered_label_count={}",
        summary.uncovered_label_count
    )?;
    writeln!(
        file,
        "covered_known_opcode_label_count={}",
        summary.covered_known_opcode_label_count
    )?;
    writeln!(
        file,
        "uncovered_known_opcode_label_count={}",
        summary.uncovered_known_opcode_label_count
    )?;
    writeln!(
        file,
        "covered_data_or_unknown_label_count={}",
        summary.covered_data_or_unknown_label_count
    )?;
    writeln!(
        file,
        "uncovered_data_or_unknown_label_count={}",
        summary.uncovered_data_or_unknown_label_count
    )?;
    writeln!(file, "instruction_count={}", summary.instruction_count)?;
    writeln!(
        file,
        "unknown_opcode_or_data_count={}",
        summary.unknown_opcode_or_data_count
    )?;
    writeln!(file, "edge_count={}", summary.edge_count)?;
    writeln!(file, "complete=1")?;
    writeln!(
        file,
        "static_reachable_label_count={}",
        summary.static_reachable_label_count
    )?;
    writeln!(
        file,
        "static_reachable_uncovered_label_count={}",
        summary.static_reachable_uncovered_label_count
    )?;
    writeln!(
        file,
        "static_reachable_uncovered_known_opcode_label_count={}",
        summary.static_reachable_uncovered_known_opcode_label_count
    )?;
    Ok(())
}

fn write_manifest(path: &Path, disasm_dir: &Path, plan: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=static_cfg_gap")?;
    writeln!(file, "disasm_dir={}", disasm_dir.display())?;
    writeln!(file, "dynamic_plan={}", plan.display())?;
    writeln!(file, "static_labels=static_labels.tsv")?;
    writeln!(file, "static_edges=static_edges.tsv")?;
    writeln!(file, "static_label_edges=static_label_edges.tsv")?;
    writeln!(file, "static_reachable_labels=static_reachable_labels.tsv")?;
    writeln!(file, "coverage_gap=coverage_gap.tsv")?;
    writeln!(file, "coverage_gap_summary=coverage_gap_summary.tsv")?;
    writeln!(file, "coverage_gap_ranked=coverage_gap_ranked.tsv")?;
    writeln!(file, "coverage_gap_reachable=coverage_gap_reachable.tsv")?;
    writeln!(file, "summary=static_cfg_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_static_cfg_gap_report() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_cfg_gap_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build = root.join("build");
        let disasm = build.join("disasm");
        let plan_dir = build.join("block_translation_plan");
        let out = root.join("out");
        fs::create_dir_all(&disasm).unwrap();
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(
            disasm.join("manifest.txt"),
            "prg_16k_banks=2\nvector_nmi_prg_offset=0x00006\nvector_reset_prg_offset=0x04000\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            disasm.join("prg_bank_00_8000.asm"),
            "; PRG 16 KiB bank 0 mapped at CPU $8000-$BFFF\n\n\
             L_8000:\n  ; 8000  A9 01     LDA #$01\n  ; 8002  F0 02     BEQ L_8006\n  ; 8004  20 00 C0  JSR $C000\n\
             L_8006:\n  ; 8006  EA        NOP\n\
             L_8007:\n  ; 8007  60        RTS\n",
        )
        .unwrap();
        fs::write(
            disasm.join("fixed_bank_01_c000.asm"),
            "; Final PRG 16 KiB bank mapped at CPU $C000-$FFFF for reset vectors\n\n\
             L_C000:\n  ; C000  4C 07 80  JMP $8007\n  ; C003  60        RTS\n",
        )
        .unwrap();
        fs::write(
            plan_dir.join("block_translation_plan.tsv"),
            "replay\tid\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstop_reason\tstatus\thit_count\tsteps\twrites\tppu_writes\tapu_writes\tmapper_writes\tstate_applied\tfinal_ram_sha256\tclass\tpriority\n\
             title\t0\t8000\t00000\t2\tA9\tnext_trace_label\tleft_block\t1\t1\t0\t0\t0\t0\t1\taaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\tstraight_line\t1\n\
             title\t1\tC000\t04000\t3\t4C\tterminator_4C\tleft_block\t1\t1\t0\t0\t0\t0\t1\tbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\tcall_or_jump\t1\n",
        )
        .unwrap();

        run(&build, &out, &["title".to_string()]).unwrap();

        let summary = fs::read_to_string(out.join("static_cfg_summary.txt")).unwrap();
        assert!(summary.contains("label_count=4\n"));
        assert!(summary.contains("static_reachable_label_count=4\n"));
        let edges = fs::read_to_string(out.join("static_edges.tsv")).unwrap();
        assert!(
            edges.contains("banked\t0\t8002\t00002\tF0\tBEQ\tbranch_target\t8006\t00006\tmapped\n")
        );
        assert!(edges.contains(
            "fixed\t1\tC000\t04000\t4C\tJMP\tjump_target\t8007\t\tbanked_window_unknown\n"
        ));
        let reachable = fs::read_to_string(out.join("coverage_gap_reachable.tsv")).unwrap();
        assert!(reachable.contains(
            "\t2021100\tbanked\t0\t8006\t00006\tL_8006\tEA\tNOP\t1\t2\t2\tvector_nmi,dynamic\t"
        ));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
