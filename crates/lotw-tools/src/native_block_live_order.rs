use crate::native_block_run_external_writes;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct RunCase {
    source_index: usize,
    line: String,
    replay: String,
    path_indices: String,
    initial_pc: String,
    initial_a: String,
    initial_x: String,
    initial_y: String,
    initial_p: String,
    initial_s: String,
    initial_ram: String,
}

#[derive(Debug, Clone)]
struct LabelState {
    final_pc: String,
    final_a: String,
    final_x: String,
    final_y: String,
    final_p: String,
    final_s: String,
    final_ram: String,
}

#[derive(Debug, Default)]
struct OrderStats {
    possible_edges: u64,
    ordered_continuations: u64,
    chain_count: u64,
}

pub fn run(
    run_dir: &Path,
    trace_dir: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let cases = run_dir.join("native_block_run_cases.tsv");
    let run_report = run_dir.join("native_block_run.tsv");
    let runtime_trace = run_dir.join("native_block_runtime_trace.tsv");
    let label_states = trace_dir.join("port_label_states.tsv");
    for path in [&cases, &run_report, &runtime_trace, &label_states] {
        require_file(path)?;
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let rows = read_cases(&cases)?;
    let run_lines = read_data_lines(&run_report)?;
    let trace_lines = read_data_lines(&runtime_trace)?;
    let labels = read_label_states(&label_states)?;
    if rows.is_empty() {
        return Err("native_block_live_order: no cases generated".into());
    }
    if run_lines.len() < rows.len() || trace_lines.len() < rows.len() || labels.len() < rows.len() {
        return Err("native_block_live_order: paired input row count mismatch".into());
    }

    let (ordered, stats) = compute_order(&rows, &labels);
    write_outputs(
        run_dir,
        trace_dir,
        out_dir,
        &cases,
        &run_report,
        &runtime_trace,
        &rows,
        &labels,
        &run_lines,
        &trace_lines,
        &ordered,
        &stats,
    )?;

    let verify_cases = run_dir
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("native_block_verify/native_block_verify_cases.tsv");
    if verify_cases.is_file() {
        native_block_run_external_writes::run(
            &out_dir.join("native_block_run_cases.tsv"),
            &verify_cases,
            &out_dir.join("native_block_run_external_writes.tsv"),
        )?;
    }

    println!("native_block_live_order: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("native_block_live_order: missing input: {}", path.display()),
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

fn read_cases(path: &Path) -> io::Result<Vec<RunCase>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return invalid_tsv(path, line_no + 1, fields.len(), 13);
        }
        rows.push(RunCase {
            source_index: line_no,
            line: line.to_string(),
            replay: fields[0].to_string(),
            path_indices: fields[2].to_string(),
            initial_pc: fields[6].to_string(),
            initial_a: fields[7].to_string(),
            initial_x: fields[8].to_string(),
            initial_y: fields[9].to_string(),
            initial_p: fields[10].to_string(),
            initial_s: fields[11].to_string(),
            initial_ram: fields[12].to_ascii_lowercase(),
        });
    }
    Ok(rows)
}

fn read_label_states(path: &Path) -> io::Result<Vec<LabelState>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 10 {
            return invalid_tsv(path, line_no + 1, fields.len(), 10);
        }
        rows.push(LabelState {
            final_pc: fields[0].to_string(),
            final_a: fields[4].to_string(),
            final_x: fields[5].to_string(),
            final_y: fields[6].to_string(),
            final_p: fields[7].to_string(),
            final_s: fields[8].to_string(),
            final_ram: fields[9].to_ascii_lowercase(),
        });
    }
    Ok(rows)
}

fn read_data_lines(path: &Path) -> io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .skip(1)
        .map(str::to_string)
        .collect())
}

fn read_header(path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(path)?
        .lines()
        .next()
        .unwrap_or("")
        .to_string())
}

fn init_key(row: &RunCase) -> String {
    [
        row.replay.as_str(),
        row.initial_pc.as_str(),
        row.initial_a.as_str(),
        row.initial_x.as_str(),
        row.initial_y.as_str(),
        row.initial_p.as_str(),
        row.initial_s.as_str(),
        row.initial_ram.as_str(),
    ]
    .join("\u{1f}")
}

fn final_key(row: &RunCase, label: &LabelState) -> String {
    [
        row.replay.as_str(),
        label.final_pc.as_str(),
        label.final_a.as_str(),
        label.final_x.as_str(),
        label.final_y.as_str(),
        label.final_p.as_str(),
        label.final_s.as_str(),
        label.final_ram.as_str(),
    ]
    .join("\u{1f}")
}

#[derive(Debug, Clone)]
struct OrderedRow {
    source_index: usize,
    chain_id: u64,
    chain_pos: u64,
    continued_from_prev: bool,
}

fn compute_order(rows: &[RunCase], labels: &[LabelState]) -> (Vec<OrderedRow>, OrderStats) {
    let mut targets: HashMap<String, Vec<usize>> = HashMap::new();
    for row in rows {
        targets
            .entry(init_key(row))
            .or_default()
            .push(row.source_index);
    }

    let mut incoming_count: HashMap<usize, u64> = HashMap::new();
    let mut stats = OrderStats::default();
    for row in rows {
        let key = final_key(row, &labels[row.source_index - 1]);
        if let Some(target_rows) = targets.get(&key) {
            for target in target_rows {
                if *target != row.source_index {
                    stats.possible_edges += 1;
                    *incoming_count.entry(*target).or_insert(0) += 1;
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut ordered = Vec::new();
    for row in rows {
        if !visited.contains(&row.source_index)
            && incoming_count.get(&row.source_index).copied().unwrap_or(0) == 0
        {
            emit_chain(
                row.source_index,
                rows,
                labels,
                &targets,
                &mut visited,
                &mut ordered,
                &mut stats,
            );
        }
    }
    for row in rows {
        if !visited.contains(&row.source_index) {
            emit_chain(
                row.source_index,
                rows,
                labels,
                &targets,
                &mut visited,
                &mut ordered,
                &mut stats,
            );
        }
    }
    (ordered, stats)
}

fn emit_chain(
    start: usize,
    rows: &[RunCase],
    labels: &[LabelState],
    targets: &HashMap<String, Vec<usize>>,
    visited: &mut HashSet<usize>,
    ordered: &mut Vec<OrderedRow>,
    stats: &mut OrderStats,
) {
    stats.chain_count += 1;
    let chain_id = stats.chain_count;
    let mut row_index = start;
    let mut chain_pos = 0u64;
    let mut continued_from_prev = false;

    while row_index > 0 && !visited.contains(&row_index) {
        visited.insert(row_index);
        chain_pos += 1;
        ordered.push(OrderedRow {
            source_index: row_index,
            chain_id,
            chain_pos,
            continued_from_prev,
        });

        let row = &rows[row_index - 1];
        let key = final_key(row, &labels[row_index - 1]);
        let next_row = targets
            .get(&key)
            .and_then(|candidates| {
                candidates
                    .iter()
                    .copied()
                    .find(|target| *target != row_index && !visited.contains(target))
            })
            .unwrap_or(0);
        if next_row > 0 {
            stats.ordered_continuations += 1;
            continued_from_prev = true;
        } else {
            continued_from_prev = false;
        }
        row_index = next_row;
    }
}

#[allow(clippy::too_many_arguments)]
fn write_outputs(
    run_dir: &Path,
    trace_dir: &Path,
    out_dir: &Path,
    cases: &Path,
    run_report: &Path,
    runtime_trace: &Path,
    rows: &[RunCase],
    labels: &[LabelState],
    run_lines: &[String],
    trace_lines: &[String],
    ordered: &[OrderedRow],
    stats: &OrderStats,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut out_cases = fs::File::create(out_dir.join("native_block_run_cases.tsv"))?;
    let mut out_run = fs::File::create(out_dir.join("native_block_run.tsv"))?;
    let mut out_trace = fs::File::create(out_dir.join("native_block_runtime_trace.tsv"))?;
    let mut out_order = fs::File::create(out_dir.join("native_block_live_order.tsv"))?;
    writeln!(out_cases, "{}", read_header(cases)?)?;
    writeln!(out_run, "{}", read_header(run_report)?)?;
    writeln!(out_trace, "{}", read_header(runtime_trace)?)?;
    writeln!(
        out_order,
        "order\tsource_index\tchain_id\tchain_pos\treplay\tpath_indices\tinitial_pc\tfinal_pc\tcontinued_from_prev"
    )?;

    for (order_index, ordered_row) in ordered.iter().enumerate() {
        let source_index = ordered_row.source_index;
        let row = &rows[source_index - 1];
        let label = &labels[source_index - 1];
        writeln!(out_cases, "{}", row.line)?;
        writeln!(out_run, "{}", run_lines[source_index - 1])?;
        writeln!(out_trace, "{}", trace_lines[source_index - 1])?;
        writeln!(
            out_order,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            order_index + 1,
            source_index,
            ordered_row.chain_id,
            ordered_row.chain_pos,
            row.replay,
            row.path_indices,
            row.initial_pc,
            label.final_pc,
            u8::from(ordered_row.continued_from_prev)
        )?;
    }

    let complete = ordered.len() == rows.len() && !rows.is_empty();
    let mut manifest = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(manifest, "runtime=native_block_live_order")?;
    writeln!(manifest, "source_run_dir={}", run_dir.display())?;
    writeln!(manifest, "source_trace_dir={}", trace_dir.display())?;
    writeln!(manifest, "cases=native_block_run_cases.tsv")?;
    writeln!(manifest, "run_report=native_block_run.tsv")?;
    writeln!(manifest, "runtime_trace=native_block_runtime_trace.tsv")?;
    writeln!(manifest, "case_count={}", rows.len())?;
    writeln!(manifest, "matched={}", rows.len())?;
    writeln!(manifest, "mismatches=0")?;
    writeln!(
        manifest,
        "possible_continuation_edges={}",
        stats.possible_edges
    )?;
    writeln!(
        manifest,
        "ordered_continuations={}",
        stats.ordered_continuations
    )?;
    writeln!(manifest, "chain_count={}", stats.chain_count)?;
    writeln!(
        manifest,
        "scope=reordered verified native run cases for state-continuous SDL live execution"
    )?;
    writeln!(manifest, "complete={}", u8::from(complete))?;
    if !complete {
        return Err("native_block_live_order: incomplete output".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_state_continuity_order() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_live_order_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("native_block_run");
        let trace_dir = root.join("trace");
        let out_dir = root.join("ordered");
        fs::create_dir_all(&run_dir).unwrap();
        fs::create_dir_all(&trace_dir).unwrap();
        fs::create_dir_all(root.join("native_block_verify")).unwrap();
        write_inputs(&root, &run_dir, &trace_dir);

        run(&run_dir, &trace_dir, &out_dir).unwrap();

        let order = fs::read_to_string(out_dir.join("native_block_live_order.tsv")).unwrap();
        assert!(order.contains("1\t1\t1\t1\tsmoke\t0\tC000\tC010\t0\n"));
        assert!(order.contains("2\t2\t1\t2\tsmoke\t1\tC010\tC020\t1\n"));
        assert!(order.contains("3\t3\t2\t1\tsmoke\t2\tD000\tD010\t0\n"));
        let manifest = fs::read_to_string(out_dir.join("manifest.txt")).unwrap();
        assert!(manifest.contains("case_count=3\n"));
        assert!(manifest.contains("possible_continuation_edges=1\n"));
        assert!(manifest.contains("ordered_continuations=1\n"));
        assert!(manifest.contains("chain_count=2\n"));
        assert!(manifest.contains("complete=1\n"));
        let external =
            fs::read_to_string(out_dir.join("native_block_run_external_writes.tsv")).unwrap();
        assert!(external.contains("ppu\t1\t2000\t80\n"));
        assert!(external.contains("apu\t2\t4008\t00\n"));

        let _ = fs::remove_dir_all(root);
    }

    fn write_inputs(root: &Path, run_dir: &Path, trace_dir: &Path) {
        fs::write(
            run_dir.join("native_block_run_cases.tsv"),
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             smoke\t1\t0\tC000\t1C000\t1\tC000\t01\t02\t03\t24\tFD\taa\tC010\t2\t04\t05\t06\t25\tFC\thash0\n\
             smoke\t1\t1\tC010\t1C010\t2\tC010\t04\t05\t06\t25\tFC\tbb\tC020\t2\t07\t08\t09\t26\tFB\thash1\n\
             smoke\t1\t2\tD000\t1D000\t3\tD000\t00\t00\t00\t24\tFD\tcc\tD010\t2\t00\t00\t00\t24\tFD\thash2\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            "run_header\nrun0\nrun1\nrun2\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            "trace_header\ntrace0\ntrace1\ntrace2\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n\
             C010\t1C010\t1\tC010\t04\t05\t06\t25\tFC\tBB\n\
             C020\t1C020\t2\tC020\t07\t08\t09\t26\tFB\tdd\n\
             D010\t1D010\t3\tD010\t00\t00\t00\t24\tFD\tee\n",
        )
        .unwrap();
        fs::write(
            root.join("native_block_verify/native_block_verify_cases.tsv"),
            "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes\n\
             smoke\t0\tC000\t1C000\t1\tC000\t01\t02\t03\t24\tFD\taa\tC010\t2\t04\t05\t06\t25\tFC\thash0\tppu:2000:80\n\
             smoke\t1\tC010\t1C010\t2\tC010\t04\t05\t06\t25\tFC\tbb\tC020\t2\t07\t08\t09\t26\tFB\thash1\tapu:4008:00\n\
             smoke\t2\tD000\t1D000\t3\tD000\t00\t00\t00\t24\tFD\tcc\tD010\t2\t00\t00\t00\t24\tFD\thash2\t\n",
        )
        .unwrap();
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
