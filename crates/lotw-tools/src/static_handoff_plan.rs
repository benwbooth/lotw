use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Default, Clone)]
struct EntryContext {
    source_edge_count: String,
    source_edge_types: String,
    outgoing_edge_types: String,
}

#[derive(Debug, Default)]
struct Summary {
    candidate_count: u64,
    leaf_return_or_interrupt_count: u64,
    calls_subroutine_count: u64,
    control_flow_count: u64,
    straight_line_or_data_count: u64,
    not_in_current_entry_plan_count: u64,
    other_shape_count: u64,
    reason_counts: BTreeMap<String, u64>,
    action_counts: BTreeMap<String, u64>,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if limit == 0 {
        return Err("static_handoff_plan: limit must be a positive integer".into());
    }

    let frontier = build_dir
        .join("static_rom_audit")
        .join("static_rom_frontier.tsv");
    let entry_plan = build_dir
        .join("static_entry_plan")
        .join("static_entry_plan.tsv");
    let leaf_skipped = build_dir
        .join("static_leaf_verify")
        .join("static_leaf_skipped.tsv");
    require_file(&frontier)?;
    require_file(&entry_plan)?;
    require_file(&leaf_skipped)?;

    fs::create_dir_all(out_dir)?;

    let entry_context = read_entry_context(&entry_plan)?;
    let skipped = read_leaf_skipped(&leaf_skipped)?;
    let summary = write_plan(
        &out_dir.join("static_handoff_plan.tsv"),
        &frontier,
        &entry_context,
        &skipped,
        limit,
    )?;
    write_summary(&out_dir.join("static_handoff_plan_summary.txt"), &summary)?;
    write_manifest(
        &out_dir.join("manifest.txt"),
        &frontier,
        &entry_plan,
        &leaf_skipped,
        limit,
    )?;

    println!("static_handoff_plan: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_handoff_plan: missing input: {}", path.display()),
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

fn read_entry_context(path: &Path) -> io::Result<HashMap<String, EntryContext>> {
    let text = fs::read_to_string(path)?;
    let mut context = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 19 {
            return invalid_tsv(path, line_no + 1, fields.len(), 19);
        }
        context.insert(
            fields[6].to_uppercase(),
            EntryContext {
                source_edge_count: fields[14].to_string(),
                source_edge_types: fields[17].to_string(),
                outgoing_edge_types: fields[18].to_string(),
            },
        );
    }
    Ok(context)
}

fn read_leaf_skipped(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut skipped = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 6 {
            return invalid_tsv(path, line_no + 1, fields.len(), 6);
        }
        skipped.insert(fields[2].to_uppercase(), fields[5].to_string());
    }
    Ok(skipped)
}

fn blocking_reason(shape: &str, skipped_reason: Option<&str>) -> String {
    if let Some(reason) = skipped_reason {
        if !reason.is_empty() {
            return reason.to_string();
        }
    }
    match shape {
        "leaf_return_or_interrupt" => "leaf_not_selected_or_alias_pending",
        "calls_subroutine" => "needs_jsr_handoff_model",
        "control_flow" => "needs_branch_handoff_cases",
        "straight_line_or_data" => "needs_synthetic_handoff_state",
        "not_in_current_entry_plan" => "expand_static_entry_plan",
        _ => "needs_manual_classification",
    }
    .to_string()
}

fn next_action(shape: &str, reason: &str) -> &'static str {
    match reason {
        "call_like_leaf_deferred" => "add_call_handoff_synthetic_cases",
        "unsupported_native_opcode" => "add_native_opcode_support",
        "missing_byte_count" => "improve_static_block_bounds",
        _ => match shape {
            "calls_subroutine" => "translate_callee_or_split_at_call_return",
            "control_flow" => "generate_branch_taken_and_fallthrough_cases",
            "straight_line_or_data" => "generate_linear_handoff_case",
            "leaf_return_or_interrupt" => "inspect_leaf_alias_or_raise_leaf_limit",
            "not_in_current_entry_plan" => "raise_static_entry_plan_limit",
            _ => "inspect_static_candidate",
        },
    }
}

fn count_shape(summary: &mut Summary, shape: &str) {
    match shape {
        "leaf_return_or_interrupt" => summary.leaf_return_or_interrupt_count += 1,
        "calls_subroutine" => summary.calls_subroutine_count += 1,
        "control_flow" => summary.control_flow_count += 1,
        "straight_line_or_data" => summary.straight_line_or_data_count += 1,
        "not_in_current_entry_plan" => summary.not_in_current_entry_plan_count += 1,
        _ => summary.other_shape_count += 1,
    }
}

fn write_plan(
    out_path: &Path,
    frontier_path: &Path,
    entry_context: &HashMap<String, EntryContext>,
    skipped: &HashMap<String, String>,
    limit: usize,
) -> io::Result<Summary> {
    let text = fs::read_to_string(frontier_path)?;
    let mut file = fs::File::create(out_path)?;
    writeln!(
        file,
        "rank\tfrontier_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tstatic_shape\tblocking_reason\tnext_action\tsource_edge_count\tsource_edge_types\toutgoing_edge_types\tmapped_in_edges\treachable_in_edges\treachable_from\tfile"
    )?;

    let mut summary = Summary::default();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if summary.candidate_count as usize >= limit {
            break;
        }
        let fields = split_tsv(line);
        if fields.len() < 18 {
            return invalid_tsv(frontier_path, line_no + 1, fields.len(), 18);
        }
        if fields[15] == "1" {
            continue;
        }

        let key = fields[5].to_uppercase();
        let shape = fields[13];
        let reason = blocking_reason(shape, skipped.get(&key).map(String::as_str));
        let action = next_action(shape, &reason);
        let empty = EntryContext::default();
        let context = entry_context.get(&key).unwrap_or(&empty);

        summary.candidate_count += 1;
        count_shape(&mut summary, shape);
        *summary.reason_counts.entry(reason.clone()).or_default() += 1;
        *summary.action_counts.entry(action.to_string()).or_default() += 1;

        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            summary.candidate_count,
            fields[0],
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7],
            fields[8],
            shape,
            reason,
            action,
            context.source_edge_count.parse::<u64>().unwrap_or(0),
            context.source_edge_types,
            context.outgoing_edge_types,
            fields[10],
            fields[11],
            fields[12],
            fields[17]
        )?;
    }

    Ok(summary)
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
        "calls_subroutine_count={}",
        summary.calls_subroutine_count
    )?;
    writeln!(file, "control_flow_count={}", summary.control_flow_count)?;
    writeln!(
        file,
        "straight_line_or_data_count={}",
        summary.straight_line_or_data_count
    )?;
    writeln!(
        file,
        "not_in_current_entry_plan_count={}",
        summary.not_in_current_entry_plan_count
    )?;
    writeln!(file, "other_shape_count={}", summary.other_shape_count)?;
    for (reason, count) in &summary.reason_counts {
        writeln!(file, "blocking_reason_{reason}={count}")?;
    }
    for (action, count) in &summary.action_counts {
        writeln!(file, "next_action_{action}={count}")?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(
    path: &Path,
    frontier: &Path,
    entry_plan: &Path,
    leaf_skipped: &Path,
    limit: usize,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=static_handoff_plan")?;
    writeln!(file, "frontier={}", frontier.display())?;
    writeln!(file, "entry_plan={}", entry_plan.display())?;
    writeln!(file, "leaf_skipped={}", leaf_skipped.display())?;
    writeln!(file, "limit={limit}")?;
    writeln!(file, "plan=static_handoff_plan.tsv")?;
    writeln!(file, "summary=static_handoff_plan_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_static_handoff_plan() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_handoff_plan_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build = root.join("build");
        let out = root.join("out");
        fs::create_dir_all(build.join("static_rom_audit")).unwrap();
        fs::create_dir_all(build.join("static_entry_plan")).unwrap();
        fs::create_dir_all(build.join("static_leaf_verify")).unwrap();
        fs::write(
            build.join("static_rom_audit/static_rom_frontier.tsv"),
            "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tstatic_shape\trecommended_next_step\tsynthetic_verified\tnative_static_reason\tfile\n\
             1\t900\tfixed\t1\tC010\t04010\tL_C010\t20\tJSR\t1\t3\t2\tdynamic\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\t0\t\tfixture.asm\n\
             2\t800\tfixed\t1\tC020\t04020\tL_C020\tD0\tBNE\t1\t2\t2\tdynamic\tcontrol_flow\ttarget_replay_or_synthetic_handoff_state\t0\t\tfixture.asm\n\
             3\t600\tfixed\t1\tC040\t04040\tL_C040\t60\tRTS\t1\t1\t1\tvector_reset\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\t1\tstatic_leaf_synthetic\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            build.join("static_entry_plan/static_entry_plan.tsv"),
            "rank\tgap_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tsource_edge_count\tsource_labels\tsource_instruction_cpu_addrs\tsource_edge_types\toutgoing_edge_types\tstatic_shape\trecommended_next_step\tfile\n\
             1\t1\t900\tfixed\t1\tC010\t04010\tL_C010\t20\tJSR\t1\t3\t2\tdynamic\t2\tL_C000\tC000\tcall_target\tcall_target,call_return\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\tfixture.asm\n\
             2\t2\t800\tfixed\t1\tC020\t04020\tL_C020\tD0\tBNE\t1\t2\t2\tdynamic\t1\tL_C000\tC002\tbranch_target\tbranch_target,branch_fallthrough\tcontrol_flow\ttarget_replay_or_synthetic_handoff_state\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            build.join("static_leaf_verify/static_leaf_skipped.tsv"),
            "plan_rank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\treason\n\
             1\tC010\t04010\tL_C010\t20\tcall_like_leaf_deferred\n",
        )
        .unwrap();

        run(&build, &out, 8).unwrap();

        let summary = fs::read_to_string(out.join("static_handoff_plan_summary.txt")).unwrap();
        assert!(summary.contains("candidate_count=2\n"));
        assert!(summary.contains("leaf_return_or_interrupt_count=1\n"));
        assert!(summary.contains("control_flow_count=1\n"));
        assert!(summary.contains("blocking_reason_call_like_leaf_deferred=1\n"));
        let plan = fs::read_to_string(out.join("static_handoff_plan.tsv")).unwrap();
        assert!(plan.contains("1\t1\t900\tfixed\t1\tC010\t04010\tL_C010\t20\tJSR\tleaf_return_or_interrupt\tcall_like_leaf_deferred\tadd_call_handoff_synthetic_cases\t2\tcall_target\tcall_target,call_return\t3\t2\tdynamic\tfixture.asm\n"));
        assert!(plan.contains("2\t2\t800\tfixed\t1\tC020\t04020\tL_C020\tD0\tBNE\tcontrol_flow\tneeds_branch_handoff_cases\tgenerate_branch_taken_and_fallthrough_cases\t1\tbranch_target\tbranch_target,branch_fallthrough\t2\t2\tdynamic\tfixture.asm\n"));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
