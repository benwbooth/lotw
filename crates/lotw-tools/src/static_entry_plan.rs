use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

type SourceEdgeRows = Vec<Vec<String>>;
type EdgeReadResult = (HashMap<String, EdgeInfo>, SourceEdgeRows);

#[derive(Debug, Clone)]
struct Gap {
    rank: String,
    priority: String,
    bank_kind: String,
    bank: String,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    known_opcode: String,
    mapped_in_edges: String,
    reachable_in_edges: String,
    reachable_from: String,
    file: String,
}

#[derive(Debug, Default)]
struct EdgeInfo {
    source_edge_count: u64,
    source_labels: Vec<String>,
    source_instruction_cpu_addrs: Vec<String>,
    source_edge_types: Vec<String>,
    outgoing_edge_types: Vec<String>,
    seen: HashSet<String>,
}

#[derive(Debug, Default)]
struct Summary {
    candidate_count: u64,
    leaf_return_or_interrupt_count: u64,
    control_flow_or_call_count: u64,
    unresolved_indirect_count: u64,
    data_or_unknown_count: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if limit == 0 {
        return Err("static_entry_plan: limit must be a positive integer".into());
    }

    let static_dir = build_dir.join("static_cfg");
    let reachable_gaps = static_dir.join("coverage_gap_reachable.tsv");
    let reachable_labels = static_dir.join("static_reachable_labels.tsv");
    let label_edges = static_dir.join("static_label_edges.tsv");
    require_file(&reachable_gaps)?;
    require_file(&reachable_labels)?;
    require_file(&label_edges)?;

    fs::create_dir_all(out_dir)?;

    let reachable = read_reachable_labels(&reachable_labels)?;
    let (gap_order, gaps) = read_gaps(&reachable_gaps, limit)?;
    let (edge_info, source_edges) = read_edges(&label_edges, &reachable, &gaps)?;
    let summary = write_plan(
        &out_dir.join("static_entry_plan.tsv"),
        &gap_order,
        &gaps,
        &edge_info,
    )?;
    write_source_edges(
        &out_dir.join("static_entry_source_edges.tsv"),
        &source_edges,
    )?;
    write_summary(&out_dir.join("static_entry_plan_summary.txt"), &summary)?;
    write_manifest(&out_dir.join("manifest.txt"), &static_dir, limit)?;

    println!("static_entry_plan: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_entry_plan: missing input: {}", path.display()),
        ))
    }
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

fn read_reachable_labels(path: &Path) -> io::Result<HashMap<String, bool>> {
    let text = fs::read_to_string(path)?;
    let mut reachable = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 11 {
            return invalid_tsv(path, line_no + 1, fields.len(), 11);
        }
        reachable.insert(fields[3].to_string(), fields[10] == "1");
    }
    Ok(reachable)
}

fn read_gaps(path: &Path, limit: usize) -> io::Result<(Vec<String>, HashMap<String, Gap>)> {
    let text = fs::read_to_string(path)?;
    let mut order = Vec::new();
    let mut gaps = HashMap::<String, Gap>::new();

    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 14 {
            return invalid_tsv(path, line_no + 1, fields.len(), 14);
        }
        let key = fields[5].to_string();
        if let Some(existing) = gaps.get(&key) {
            if fields[2] == "fixed" && existing.bank_kind != "fixed" {
                gaps.insert(key, gap_from_fields(&fields));
            }
            continue;
        }
        if order.len() >= limit {
            continue;
        }
        order.push(key.clone());
        gaps.insert(key, gap_from_fields(&fields));
    }

    Ok((order, gaps))
}

fn gap_from_fields(fields: &[&str]) -> Gap {
    Gap {
        rank: fields[0].to_string(),
        priority: fields[1].to_string(),
        bank_kind: fields[2].to_string(),
        bank: fields[3].to_string(),
        cpu_addr: fields[4].to_string(),
        prg_offset: fields[5].to_string(),
        label: fields[6].to_string(),
        first_opcode: fields[7].to_string(),
        mnemonic: fields[8].to_string(),
        known_opcode: fields[9].to_string(),
        mapped_in_edges: fields[10].to_string(),
        reachable_in_edges: fields[11].to_string(),
        reachable_from: fields[12].to_string(),
        file: fields[13].to_string(),
    }
}

fn read_edges(
    path: &Path,
    reachable: &HashMap<String, bool>,
    gaps: &HashMap<String, Gap>,
) -> io::Result<EdgeReadResult> {
    let text = fs::read_to_string(path)?;
    let mut edge_info = HashMap::<String, EdgeInfo>::new();
    let mut source_edges = Vec::new();

    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return invalid_tsv(path, line_no + 1, fields.len(), 13);
        }
        let source = fields[0];
        let target = fields[4];
        let edge_type = fields[3];
        let certainty = fields[6];

        if gaps.contains_key(source) {
            add_unique(
                edge_info.entry(source.to_string()).or_default(),
                "outgoing_edge_types",
                edge_type,
            );
        }

        if !gaps.contains_key(target) || certainty != "mapped" {
            continue;
        }
        if !reachable.get(source).copied().unwrap_or(false) {
            continue;
        }

        let info = edge_info.entry(target.to_string()).or_default();
        info.source_edge_count += 1;
        add_unique(info, "source_labels", fields[1]);
        add_unique(info, "source_instruction_cpu_addrs", fields[10]);
        add_unique(info, "source_edge_types", edge_type);

        let gap = gaps
            .get(target)
            .expect("target gap exists after contains_key check");
        source_edges.push(vec![
            gap.rank.clone(),
            gap.label.clone(),
            target.to_string(),
            source.to_string(),
            fields[1].to_string(),
            fields[2].to_string(),
            edge_type.to_string(),
            fields[10].to_string(),
            fields[9].to_string(),
            fields[7].to_string(),
            fields[8].to_string(),
            fields[11].to_string(),
            fields[12].to_string(),
        ]);
    }

    Ok((edge_info, source_edges))
}

fn add_unique(info: &mut EdgeInfo, list_name: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    let seen_key = format!("{list_name}\t{value}");
    if !info.seen.insert(seen_key) {
        return;
    }
    match list_name {
        "source_labels" => info.source_labels.push(value.to_string()),
        "source_instruction_cpu_addrs" => {
            info.source_instruction_cpu_addrs.push(value.to_string());
        }
        "source_edge_types" => info.source_edge_types.push(value.to_string()),
        "outgoing_edge_types" => info.outgoing_edge_types.push(value.to_string()),
        _ => {}
    }
}

fn classify_shape(gap: &Gap, info: Option<&EdgeInfo>) -> &'static str {
    let outgoing = info
        .map(|info| info.outgoing_edge_types.join(","))
        .unwrap_or_default();
    if gap.known_opcode.parse::<u64>().unwrap_or(0) == 0 {
        return "data_or_unknown";
    }
    if outgoing.contains("indirect_jump") {
        return "unresolved_indirect";
    }
    if outgoing.contains("call_target") {
        return "calls_subroutine";
    }
    if outgoing.contains("branch_target") || outgoing.contains("jump_target") {
        return "control_flow";
    }
    if outgoing.contains("stop") {
        return "leaf_return_or_interrupt";
    }
    if matches!(gap.mnemonic.as_str(), "RTS" | "RTI" | "BRK") {
        return "leaf_return_or_interrupt";
    }
    "straight_line_or_data"
}

fn next_step(shape: &str) -> &'static str {
    match shape {
        "data_or_unknown" => "confirm_code_before_translation",
        "unresolved_indirect" => "resolve_indirect_target_or_trace",
        "leaf_return_or_interrupt" => "add_synthetic_first_hit_state_then_native_leaf_test",
        _ => "target_replay_or_synthetic_handoff_state",
    }
}

fn join(values: &[String]) -> String {
    values.join(",")
}

fn write_plan(
    path: &Path,
    order: &[String],
    gaps: &HashMap<String, Gap>,
    edge_info: &HashMap<String, EdgeInfo>,
) -> io::Result<Summary> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tgap_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tsource_edge_count\tsource_labels\tsource_instruction_cpu_addrs\tsource_edge_types\toutgoing_edge_types\tstatic_shape\trecommended_next_step\tfile"
    )?;

    let mut summary = Summary {
        candidate_count: order.len() as u64,
        ..Summary::default()
    };

    for (index, key) in order.iter().enumerate() {
        let gap = gaps.get(key).expect("ordered gap exists");
        let info = edge_info.get(key);
        let empty = EdgeInfo::default();
        let info = info.unwrap_or(&empty);
        let shape = classify_shape(gap, Some(info));
        match shape {
            "leaf_return_or_interrupt" => summary.leaf_return_or_interrupt_count += 1,
            "data_or_unknown" => summary.data_or_unknown_count += 1,
            "unresolved_indirect" => summary.unresolved_indirect_count += 1,
            _ => summary.control_flow_or_call_count += 1,
        }
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            gap.rank,
            gap.priority,
            gap.bank_kind,
            gap.bank,
            gap.cpu_addr,
            gap.prg_offset,
            gap.label,
            gap.first_opcode,
            gap.mnemonic,
            gap.known_opcode,
            gap.mapped_in_edges,
            gap.reachable_in_edges,
            gap.reachable_from,
            info.source_edge_count,
            join(&info.source_labels),
            join(&info.source_instruction_cpu_addrs),
            join(&info.source_edge_types),
            join(&info.outgoing_edge_types),
            shape,
            next_step(shape),
            gap.file
        )?;
    }

    Ok(summary)
}

fn write_source_edges(path: &Path, source_edges: &[Vec<String>]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "gap_rank\tgap_label\tgap_prg_offset\tsource_prg_offset\tsource_label\tsource_cpu_addr\tedge_type\tinstruction_cpu_addr\tinstruction_prg_offset\topcode\tmnemonic\tbank_kind\tbank"
    )?;
    for row in source_edges {
        writeln!(file, "{}", row.join("\t"))?;
    }
    Ok(())
}

fn write_summary(path: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "candidate_count={}", summary.candidate_count)?;
    writeln!(
        file,
        "leaf_return_or_interrupt_count={}",
        summary.leaf_return_or_interrupt_count
    )?;
    writeln!(
        file,
        "control_flow_or_call_count={}",
        summary.control_flow_or_call_count
    )?;
    writeln!(
        file,
        "unresolved_indirect_count={}",
        summary.unresolved_indirect_count
    )?;
    writeln!(
        file,
        "data_or_unknown_count={}",
        summary.data_or_unknown_count
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path, static_dir: &Path, limit: usize) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=static_entry_plan")?;
    writeln!(file, "static_cfg_dir={}", static_dir.display())?;
    writeln!(file, "limit={limit}")?;
    writeln!(file, "plan=static_entry_plan.tsv")?;
    writeln!(file, "source_edges=static_entry_source_edges.tsv")?;
    writeln!(file, "summary=static_entry_plan_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_static_entry_plan() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_entry_plan_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build = root.join("build");
        let static_dir = build.join("static_cfg");
        let out = root.join("out");
        fs::create_dir_all(&static_dir).unwrap();
        fs::write(
            static_dir.join("static_reachable_labels.tsv"),
            "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tcovered\treplay_count\treplays\tstatic_reachable\treachable_from\treachable_via\tmapped_in_edges\treachable_in_edges\tmapped_out_edges\tfile\n\
             banked\t0\t8000\t00000\tL_8000\tA9\tLDA\t1\t1\ttitle\t1\tdynamic\t\t0\t0\t3\tfixture.asm\n\
             banked\t0\t8006\t00006\tL_8006\tEA\tNOP\t0\t0\t\t1\tdynamic,vector_nmi\tbranch_target,linear_next_label\t2\t2\t1\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            static_dir.join("coverage_gap_reachable.tsv"),
            "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tfile\n\
             1\t2021100\tbanked\t0\t8006\t00006\tL_8006\tEA\tNOP\t1\t2\t2\tdynamic,vector_nmi\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            static_dir.join("static_label_edges.tsv"),
            "source_prg_offset\tsource_label\tsource_cpu_addr\tedge_type\ttarget_prg_offset\ttarget_cpu_addr\tcertainty\topcode\tmnemonic\tinstruction_prg_offset\tinstruction_cpu_addr\tbank_kind\tbank\n\
             00000\tL_8000\t8000\tbranch_target\t00006\t8006\tmapped\tF0\tBEQ\t00002\t8002\tbanked\t0\n\
             00006\tL_8006\t8006\tstop\t\t\tterminator\t60\tRTS\t00007\t8007\tbanked\t0\n",
        )
        .unwrap();

        run(&build, &out, 2).unwrap();

        let summary = fs::read_to_string(out.join("static_entry_plan_summary.txt")).unwrap();
        assert!(summary.contains("candidate_count=1\n"));
        assert!(summary.contains("leaf_return_or_interrupt_count=1\n"));
        let plan = fs::read_to_string(out.join("static_entry_plan.tsv")).unwrap();
        assert!(plan.contains("1\t1\t2021100\tbanked\t0\t8006\t00006\tL_8006\tEA\tNOP\t1\t2\t2\tdynamic,vector_nmi\t1\tL_8000\t8002\tbranch_target\tstop\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\tfixture.asm\n"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn branch_edges_take_precedence_over_stop_edges() {
        let gap = Gap {
            rank: "1".to_string(),
            priority: "1".to_string(),
            bank_kind: "banked".to_string(),
            bank: "7".to_string(),
            cpu_addr: "B036".to_string(),
            prg_offset: "1F036".to_string(),
            label: "L_B036".to_string(),
            first_opcode: "F0".to_string(),
            mnemonic: "BEQ".to_string(),
            known_opcode: "1".to_string(),
            mapped_in_edges: "3".to_string(),
            reachable_in_edges: "1".to_string(),
            reachable_from: "dynamic,vector_reset".to_string(),
            file: "fixture.asm".to_string(),
        };
        let info = EdgeInfo {
            outgoing_edge_types: vec![
                "branch_target".to_string(),
                "branch_fallthrough".to_string(),
                "stop".to_string(),
            ],
            ..EdgeInfo::default()
        };

        assert_eq!(classify_shape(&gap, Some(&info)), "control_flow");
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
