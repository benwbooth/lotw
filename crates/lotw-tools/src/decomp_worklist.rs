use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct ReplaySplit {
    bank_kind: String,
    bank: String,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    replay_count: u64,
    remaining_action: String,
}

#[derive(Debug, Clone)]
struct FrontierBlocker {
    plan_rank: u64,
    label: String,
    cpu_addr: String,
    prg_offset: String,
    static_shape: String,
    next_action: String,
    blocking_reason: String,
}

pub fn run(build_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let remaining_units = build_dir
        .join("whole_program_report")
        .join("whole_program_remaining_units.tsv");
    let frontier_status = build_dir
        .join("semantic_match_report")
        .join("static_frontier_match_status.tsv");

    require_file(&remaining_units)?;
    require_file(&frontier_status)?;

    let mut replay_splits = read_replay_splits(&remaining_units)?;
    replay_splits.sort_by(|lhs, rhs| {
        rhs.replay_count
            .cmp(&lhs.replay_count)
            .then_with(|| lhs.prg_offset.cmp(&rhs.prg_offset))
    });

    let mut frontier_blockers = read_frontier_blockers(&frontier_status)?;
    frontier_blockers.sort_by_key(|row| row.plan_rank);

    recreate_dir(out_dir)?;
    write_replay_split_queue(
        &out_dir.join("replay_block_split_queue.tsv"),
        &replay_splits,
    )?;
    write_static_frontier_queue(
        &out_dir.join("static_frontier_blocker_queue.tsv"),
        &frontier_blockers,
    )?;
    write_summary(
        &out_dir.join("decomp_worklist_summary.txt"),
        &replay_splits,
        &frontier_blockers,
    )?;
    write_manifest(&out_dir.join("manifest.txt"))?;

    println!("decomp_worklist: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("decomp_worklist: missing input: {}", path.display()),
        ))
    }
}

fn recreate_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

fn read_replay_splits(path: &Path) -> io::Result<Vec<ReplaySplit>> {
    let table = TsvTable::read(path)?;
    let bank_kind = table.required("bank_kind")?;
    let bank = table.required("bank")?;
    let cpu_addr = table.required("cpu_addr")?;
    let prg_offset = table.required("prg_offset")?;
    let label = table.required("label")?;
    let first_opcode = table.required("first_opcode")?;
    let mnemonic = table.required("mnemonic")?;
    let replay_count = table.required("replay_count")?;
    let remaining_action = table.required("remaining_action")?;
    let remaining_class = table.required("remaining_class")?;

    Ok(table
        .rows
        .iter()
        .filter(|row| row[remaining_class] == "replay_covered_needs_block_split")
        .map(|row| ReplaySplit {
            bank_kind: row[bank_kind].clone(),
            bank: row[bank].clone(),
            cpu_addr: row[cpu_addr].clone(),
            prg_offset: row[prg_offset].clone(),
            label: row[label].clone(),
            first_opcode: row[first_opcode].clone(),
            mnemonic: row[mnemonic].clone(),
            replay_count: parse_u64(&row[replay_count]),
            remaining_action: row[remaining_action].clone(),
        })
        .collect())
}

fn read_frontier_blockers(path: &Path) -> io::Result<Vec<FrontierBlocker>> {
    let table = TsvTable::read(path)?;
    let plan_rank = table.required("plan_rank")?;
    let label = table.required("label")?;
    let cpu_addr = table.required("cpu_addr")?;
    let prg_offset = table.required("prg_offset")?;
    let static_shape = table.required("static_shape")?;
    let next_action = table.required("next_action")?;
    let match_status = table.required("match_status")?;
    let blocking_reason = table.required("blocking_reason")?;

    Ok(table
        .rows
        .iter()
        .filter(|row| row[match_status] == "unverified")
        .map(|row| FrontierBlocker {
            plan_rank: parse_u64(&row[plan_rank]),
            label: row[label].clone(),
            cpu_addr: row[cpu_addr].clone(),
            prg_offset: row[prg_offset].clone(),
            static_shape: row[static_shape].clone(),
            next_action: row[next_action].clone(),
            blocking_reason: row[blocking_reason].clone(),
        })
        .collect())
}

fn write_replay_split_queue(path: &Path, rows: &[ReplaySplit]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "queue_rank\tlabel\tcpu_addr\tprg_offset\tbank_kind\tbank\tfirst_opcode\tmnemonic\treplay_count\tremaining_action"
    )?;
    for (index, row) in rows.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            row.label,
            row.cpu_addr,
            row.prg_offset,
            row.bank_kind,
            row.bank,
            row.first_opcode,
            row.mnemonic,
            row.replay_count,
            row.remaining_action
        )?;
    }
    Ok(())
}

fn write_static_frontier_queue(path: &Path, rows: &[FrontierBlocker]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "queue_rank\tplan_rank\tlabel\tcpu_addr\tprg_offset\tstatic_shape\tnext_action\tblocking_reason"
    )?;
    for (index, row) in rows.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            row.plan_rank,
            row.label,
            row.cpu_addr,
            row.prg_offset,
            row.static_shape,
            row.next_action,
            row.blocking_reason
        )?;
    }
    Ok(())
}

fn write_summary(
    path: &Path,
    replay_splits: &[ReplaySplit],
    frontier_blockers: &[FrontierBlocker],
) -> io::Result<()> {
    let mut shape_counts = BTreeMap::<String, u64>::new();
    let mut reason_counts = BTreeMap::<String, u64>::new();
    for row in frontier_blockers {
        *shape_counts.entry(row.static_shape.clone()).or_default() += 1;
        *reason_counts
            .entry(row.blocking_reason.clone())
            .or_default() += 1;
    }

    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=decomp_worklist")?;
    writeln!(file, "replay_block_split_count={}", replay_splits.len())?;
    writeln!(
        file,
        "static_frontier_unverified_count={}",
        frontier_blockers.len()
    )?;
    writeln!(
        file,
        "static_frontier_unverified_linear={}",
        shape_counts
            .get("straight_line_or_data")
            .copied()
            .unwrap_or(0)
    )?;
    writeln!(
        file,
        "static_frontier_unverified_branch={}",
        shape_counts.get("control_flow").copied().unwrap_or(0)
    )?;
    writeln!(
        file,
        "static_frontier_unverified_call_like_leaf={}",
        reason_counts
            .get("call_like_leaf_deferred")
            .copied()
            .unwrap_or(0)
    )?;
    writeln!(
        file,
        "static_frontier_unverified_jsr={}",
        shape_counts.get("calls_subroutine").copied().unwrap_or(0)
    )?;
    writeln!(
        file,
        "static_frontier_unverified_opcode_support={}",
        reason_counts
            .get("unsupported_native_opcode")
            .copied()
            .unwrap_or(0)
    )?;
    for (shape, count) in &shape_counts {
        writeln!(
            file,
            "static_frontier_shape_{}={count}",
            sanitize_key(shape)
        )?;
    }
    for (reason, count) in &reason_counts {
        writeln!(
            file,
            "static_frontier_blocking_{}={count}",
            sanitize_key(reason)
        )?;
    }
    if let Some(row) = replay_splits.first() {
        writeln!(
            file,
            "next_replay_block_split={}:{}:{}:{}",
            row.label, row.cpu_addr, row.prg_offset, row.replay_count
        )?;
    }
    if let Some(row) = frontier_blockers.first() {
        writeln!(
            file,
            "next_static_frontier_blocker={}:{}:{}:{}",
            row.label, row.cpu_addr, row.prg_offset, row.blocking_reason
        )?;
    }
    writeln!(
        file,
        "replay_block_split_queue=replay_block_split_queue.tsv"
    )?;
    writeln!(
        file,
        "static_frontier_blocker_queue=static_frontier_blocker_queue.tsv"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "summary=decomp_worklist_summary.txt")?;
    writeln!(
        file,
        "replay_block_split_queue=replay_block_split_queue.tsv"
    )?;
    writeln!(
        file,
        "static_frontier_blocker_queue=static_frontier_blocker_queue.tsv"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn sanitize_key(value: &str) -> String {
    let mut key = String::new();
    let mut last_was_separator = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            key.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator {
            key.push('_');
            last_was_separator = true;
        }
    }
    key.trim_matches('_').to_string()
}

fn parse_u64(value: &str) -> u64 {
    value.parse().unwrap_or(0)
}

#[derive(Debug)]
struct TsvTable {
    headers: HashMap<String, usize>,
    rows: Vec<Vec<String>>,
}

impl TsvTable {
    fn read(path: &Path) -> io::Result<Self> {
        let text = fs::read_to_string(path)?;
        let mut lines = text.lines();
        let header = lines.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} is empty", path.display()),
            )
        })?;
        let headers = header
            .split('\t')
            .enumerate()
            .map(|(index, name)| (name.to_string(), index))
            .collect::<HashMap<_, _>>();
        let width = headers.len();
        let mut rows = Vec::new();
        for (line_index, line) in lines.enumerate() {
            let fields = line.split('\t').map(str::to_string).collect::<Vec<_>>();
            if fields.len() != width {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "{}:{} has {} fields, expected {width}",
                        path.display(),
                        line_index + 2,
                        fields.len()
                    ),
                ));
            }
            rows.push(fields);
        }
        Ok(Self { headers, rows })
    }

    fn required(&self, name: &str) -> io::Result<usize> {
        self.headers.get(name).copied().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("missing required TSV column {name:?}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::unique_temp_dir;

    #[test]
    fn writes_replay_and_frontier_queues() {
        let root = unique_temp_dir("decomp-worklist");
        let build = root.join("build");
        let whole = build.join("whole_program_report");
        let semantic = build.join("semantic_match_report");
        let out = root.join("out");
        fs::create_dir_all(&whole).unwrap();
        fs::create_dir_all(&semantic).unwrap();
        fs::write(
            whole.join("whole_program_remaining_units.tsv"),
            "bank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\treplay_covered\treplay_count\tremaining_action\tremaining_class\n\
             banked\t6\tAE64\t1AE64\tL_AE64\t20\tJSR\t1\t9\tselect_or_split_replay_covered_block\treplay_covered_needs_block_split\n\
             banked\t6\tB365\t1B365\tL_B365\tB5\tLDA\t0\t0\ttranslate_callee_or_split_at_call_return\tentry_plan_calls_subroutine\n\
             banked\t7\t8093\t1C093\tL_8093\tA9\tLDA\t1\t3\tselect_or_split_replay_covered_block\treplay_covered_needs_block_split\n",
        )
        .unwrap();
        fs::write(
            semantic.join("static_frontier_match_status.tsv"),
            "plan_rank\tlabel\tcpu_addr\tprg_offset\tstatic_shape\tnext_action\tmatch_status\tmatched_kind\tblocking_reason\n\
             1\tL_E842\tE842\t1E842\tcalls_subroutine\ttranslate_callee_or_split_at_call_return\tunverified\t\tneeds_jsr_handoff_model\n\
             2\tL_B036\tB036\t1F036\tleaf_return_or_interrupt\tadd_native_opcode_support\tunverified\t\tunsupported_native_opcode\n\
             3\tL_B3F5\tB3F5\t1B3F5\tcalls_subroutine\ttranslate_callee_or_split_at_call_return\tsemantics_matched\tstatic_jsr\tneeds_jsr_handoff_model\n",
        )
        .unwrap();

        run(&build, &out).unwrap();

        let summary = fs::read_to_string(out.join("decomp_worklist_summary.txt")).unwrap();
        assert!(summary.contains("replay_block_split_count=2\n"));
        assert!(summary.contains("static_frontier_unverified_count=2\n"));
        assert!(summary.contains("static_frontier_unverified_jsr=1\n"));
        assert!(summary.contains("static_frontier_unverified_opcode_support=1\n"));
        assert!(summary.contains("next_replay_block_split=L_AE64:AE64:1AE64:9\n"));
        assert!(summary
            .contains("next_static_frontier_blocker=L_E842:E842:1E842:needs_jsr_handoff_model\n"));

        let replay_queue = fs::read_to_string(out.join("replay_block_split_queue.tsv")).unwrap();
        assert!(replay_queue.lines().nth(1).unwrap().contains("L_AE64"));
        assert!(replay_queue.lines().nth(2).unwrap().contains("L_8093"));

        fs::remove_dir_all(root).unwrap();
    }
}
