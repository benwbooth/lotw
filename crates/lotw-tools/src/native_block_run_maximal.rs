use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct RunRow {
    source_index: usize,
    line: String,
    replay: String,
    expected_steps: usize,
    path_indices: String,
    first_frame: String,
}

#[derive(Debug, Default)]
struct MaximalReport {
    source_max_steps: usize,
    selected_max_steps: usize,
    run_mismatch_rows: u64,
    trace_mismatch_rows: u64,
    dropped_prefix_cases: u64,
    source_step_count: HashMap<usize, u64>,
    selected_step_count: HashMap<usize, u64>,
    dropped_step_count: HashMap<usize, u64>,
}

pub fn run(run_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cases = run_dir.join("native_block_run_cases.tsv");
    let run_report = run_dir.join("native_block_run.tsv");
    let runtime_trace = run_dir.join("native_block_runtime_trace.tsv");
    for path in [&cases, &run_report, &runtime_trace] {
        require_file(path)?;
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let verify_cases = run_dir
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("native_block_verify/native_block_verify_cases.tsv");

    let selected = write_maximal_outputs(run_dir, out_dir, &cases, &run_report, &runtime_trace)?;
    if verify_cases.is_file() {
        write_external_writes(
            &verify_cases,
            &selected,
            &out_dir.join("native_block_run_external_writes.tsv"),
        )?;
    } else {
        let mut file = fs::File::create(out_dir.join("native_block_run_external_writes.tsv"))?;
        writeln!(file, "kind\tframe\taddr\tvalue")?;
    }

    println!("native_block_run_maximal: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "native_block_run_maximal: missing input: {}",
                path.display()
            ),
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

fn write_maximal_outputs(
    run_dir: &Path,
    out_dir: &Path,
    cases: &Path,
    run_report: &Path,
    runtime_trace: &Path,
) -> Result<Vec<RunRow>, Box<dyn std::error::Error>> {
    let (case_header, rows, mut report) = read_cases(cases)?;
    let (run_header, run_lines, run_mismatch_rows) = read_lines_with_match(run_report, 10)?;
    let (trace_header, trace_lines, trace_mismatch_rows) =
        read_lines_with_match(runtime_trace, 22)?;
    report.run_mismatch_rows = run_mismatch_rows;
    report.trace_mismatch_rows = trace_mismatch_rows;

    let mut selected_rows = Vec::new();
    let mut selected = Vec::new();

    let mut out_cases = fs::File::create(out_dir.join("native_block_run_cases.tsv"))?;
    let mut out_run = fs::File::create(out_dir.join("native_block_run.tsv"))?;
    let mut out_trace = fs::File::create(out_dir.join("native_block_runtime_trace.tsv"))?;
    let mut out_selection = fs::File::create(out_dir.join("native_block_run_maximal.tsv"))?;
    writeln!(out_cases, "{case_header}")?;
    writeln!(out_run, "{run_header}")?;
    writeln!(out_trace, "{trace_header}")?;
    writeln!(
        out_selection,
        "source_index\tselected_index\tselected\treason\treplay\texpected_steps\tpath_indices"
    )?;

    for row in &rows {
        let dropped = rows.iter().any(|other| {
            row.source_index != other.source_index
                && row.replay == other.replay
                && other.expected_steps > row.expected_steps
                && is_prefix(&row.path_indices, &other.path_indices)
        });
        if dropped {
            report.dropped_prefix_cases += 1;
            *report
                .dropped_step_count
                .entry(row.expected_steps)
                .or_insert(0) += 1;
            writeln!(
                out_selection,
                "{}\t\t0\tprefix_of_longer_path\t{}\t{}\t{}",
                row.source_index, row.replay, row.expected_steps, row.path_indices
            )?;
            continue;
        }

        selected.push(row.clone());
        selected_rows.push(row.source_index);
        *report
            .selected_step_count
            .entry(row.expected_steps)
            .or_insert(0) += 1;
        report.selected_max_steps = report.selected_max_steps.max(row.expected_steps);
        let selected_index = selected.len();
        writeln!(out_cases, "{}", row.line)?;
        writeln!(
            out_run,
            "{}",
            run_lines.get(row.source_index - 1).ok_or_else(|| {
                format!(
                    "native_block_run_maximal: missing run report row {}",
                    row.source_index
                )
            })?
        )?;
        writeln!(
            out_trace,
            "{}",
            trace_lines.get(row.source_index - 1).ok_or_else(|| {
                format!(
                    "native_block_run_maximal: missing runtime trace row {}",
                    row.source_index
                )
            })?
        )?;
        writeln!(
            out_selection,
            "{}\t{}\t1\tmaximal_path\t{}\t{}\t{}",
            row.source_index, selected_index, row.replay, row.expected_steps, row.path_indices
        )?;
    }

    write_summary(out_dir, &report)?;
    write_manifest(
        out_dir,
        run_dir,
        rows.len(),
        selected.len(),
        run_lines.len(),
        trace_lines.len(),
        &report,
    )?;

    if selected.is_empty()
        || rows.len() != run_lines.len()
        || rows.len() != trace_lines.len()
        || report.run_mismatch_rows != 0
        || report.trace_mismatch_rows != 0
    {
        return Err("native_block_run_maximal: incomplete maximal filter output".into());
    }

    Ok(selected)
}

fn is_prefix(prefix: &str, value: &str) -> bool {
    let mut with_comma = String::with_capacity(value.len() + 1);
    with_comma.push_str(value);
    with_comma.push(',');
    with_comma.starts_with(&format!("{prefix},"))
}

fn read_cases(cases: &Path) -> io::Result<(String, Vec<RunRow>, MaximalReport)> {
    let text = fs::read_to_string(cases)?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("").to_string();
    let mut rows = Vec::new();
    let mut report = MaximalReport::default();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 6 {
            return invalid_tsv(cases, line_no + 2, fields.len(), 6);
        }
        let expected_steps = fields[1].parse::<usize>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} invalid expected_steps: {err}",
                    cases.display(),
                    line_no + 2
                ),
            )
        })?;
        report.source_max_steps = report.source_max_steps.max(expected_steps);
        *report.source_step_count.entry(expected_steps).or_insert(0) += 1;
        rows.push(RunRow {
            source_index: line_no + 1,
            line: line.to_string(),
            replay: fields[0].to_string(),
            expected_steps,
            path_indices: fields[2].to_string(),
            first_frame: fields[5].to_string(),
        });
    }
    Ok((header, rows, report))
}

fn read_lines_with_match(
    path: &Path,
    match_field: usize,
) -> io::Result<(String, Vec<String>, u64)> {
    let text = fs::read_to_string(path)?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("").to_string();
    let mut rows = Vec::new();
    let mut mismatch_rows = 0;
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() <= match_field {
            return invalid_tsv(path, line_no + 2, fields.len(), match_field + 1);
        }
        if fields[match_field] != "1" {
            mismatch_rows += 1;
        }
        rows.push(line.to_string());
    }
    Ok((header, rows, mismatch_rows))
}

fn write_summary(out_dir: &Path, report: &MaximalReport) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("native_block_run_maximal_summary.tsv"))?;
    writeln!(file, "kind\texpected_steps\tcount")?;
    for step in 1..=report.source_max_steps {
        if let Some(count) = report.source_step_count.get(&step) {
            writeln!(file, "source\t{step}\t{count}")?;
        }
    }
    for step in 1..=report.source_max_steps {
        if let Some(count) = report.selected_step_count.get(&step) {
            writeln!(file, "selected\t{step}\t{count}")?;
        }
    }
    for step in 1..=report.source_max_steps {
        if let Some(count) = report.dropped_step_count.get(&step) {
            writeln!(file, "dropped_prefix\t{step}\t{count}")?;
        }
    }
    Ok(())
}

fn write_manifest(
    out_dir: &Path,
    run_dir: &Path,
    source_count: usize,
    selected_count: usize,
    run_count: usize,
    trace_count: usize,
    report: &MaximalReport,
) -> io::Result<()> {
    let complete = selected_count > 0
        && source_count == run_count
        && source_count == trace_count
        && report.run_mismatch_rows == 0
        && report.trace_mismatch_rows == 0;
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=native_block_run_maximal")?;
    writeln!(file, "source_run_dir={}", run_dir.display())?;
    writeln!(file, "source_case_count={source_count}")?;
    writeln!(file, "source_max_steps={}", report.source_max_steps)?;
    writeln!(file, "cases=native_block_run_cases.tsv")?;
    writeln!(file, "run_report=native_block_run.tsv")?;
    writeln!(file, "runtime_trace=native_block_runtime_trace.tsv")?;
    writeln!(file, "selection=native_block_run_maximal.tsv")?;
    writeln!(file, "summary=native_block_run_maximal_summary.tsv")?;
    writeln!(file, "case_count={selected_count}")?;
    writeln!(file, "matched={selected_count}")?;
    writeln!(
        file,
        "mismatches={}",
        report.run_mismatch_rows + report.trace_mismatch_rows
    )?;
    writeln!(file, "dropped_prefix_cases={}", report.dropped_prefix_cases)?;
    writeln!(file, "max_selected_steps={}", report.selected_max_steps)?;
    writeln!(
        file,
        "scope=maximal non-prefix PC-dispatched native block runs for lower-reseed live execution"
    )?;
    writeln!(file, "complete={}", u8::from(complete))?;
    Ok(())
}

fn row_key(replay: &str, index: &str) -> String {
    format!("{replay}\t{index}")
}

fn write_external_writes(
    verify_cases: &Path,
    run_cases: &[RunRow],
    out: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(verify_cases)?;
    let mut external = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 2 {
            return invalid_tsv(verify_cases, line_no + 1, fields.len(), 2)?;
        }
        external.insert(
            row_key(fields[0], fields[1]),
            fields.get(20).copied().unwrap_or("").to_string(),
        );
    }

    let mut file = fs::File::create(out)?;
    writeln!(file, "kind\tframe\taddr\tvalue")?;
    for case in run_cases {
        for index in case.path_indices.split(',') {
            let writes = external
                .get(&row_key(&case.replay, index))
                .map_or("", String::as_str);
            if writes.is_empty() {
                continue;
            }
            for write in writes.split(',').filter(|value| !value.is_empty()) {
                let fields = write.split(':').collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err(format!(
                        "native_block_run_external_writes: malformed row: {write}"
                    )
                    .into());
                }
                writeln!(
                    file,
                    "{}\t{}\t{}\t{}",
                    fields[0],
                    case.first_frame,
                    fields[1].to_ascii_uppercase(),
                    fields[2].to_ascii_uppercase()
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_maximal_run_cases() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_run_maximal_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("native_block_run");
        let out_dir = root.join("maximal");
        fs::create_dir_all(&run_dir).unwrap();
        fs::create_dir_all(&out_dir).unwrap();
        fs::write(
            run_dir.join("native_block_run_cases.tsv"),
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             smoke\t2\t0,1\t8000,8002\t18000,18002\t1\t8000\t00\t00\t00\t00\tFD\tram\t8004\t4\t00\t00\t00\t00\tFD\thash\n\
             smoke\t3\t0,1,2\t8000,8002,8004\t18000,18002,18004\t1\t8000\t00\t00\t00\t00\tFD\tram\t8006\t6\t00\t00\t00\t00\tFD\thash\n\
             smoke\t4\t0,1,2,3\t8000,8002,8004,8006\t18000,18002,18004,18006\t1\t8000\t00\t00\t00\t00\tFD\tram\t8008\t8\t00\t00\t00\t00\tFD\thash\n\
             smoke\t2\t4,5\t8010,8012\t18010,18012\t2\t8010\t00\t00\t00\t00\tFD\tram\t8014\t4\t00\t00\t00\t00\tFD\thash\n\
             other\t2\t0,1\t9000,9002\t19000,19002\t3\t9000\t00\t00\t00\t00\tFD\tram\t9004\t4\t00\t00\t00\t00\tFD\thash\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tdispatch_match\tpath_match\tmetadata_match\tregister_match\tcycles_match\tram_match\tmatch\tfinal_pc\tcycles\n\
             smoke\t2\t2\t0,1\t1\t1\t1\t1\t1\t1\t1\t8004\t4\n\
             smoke\t3\t3\t0,1,2\t1\t1\t1\t1\t1\t1\t1\t8006\t6\n\
             smoke\t4\t4\t0,1,2,3\t1\t1\t1\t1\t1\t1\t1\t8008\t8\n\
             smoke\t2\t2\t4,5\t1\t1\t1\t1\t1\t1\t1\t8014\t4\n\
             other\t2\t2\t0,1\t1\t1\t1\t1\t1\t1\t1\t9004\t4\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match\n\
             smoke\t2\t2\t0,1\t1\t8000\t8004\t4\t00\t00\t00\t00\tFD\thash\t8004\t4\t00\t00\t00\t00\tFD\thash\t1\n\
             smoke\t3\t3\t0,1,2\t1\t8000\t8006\t6\t00\t00\t00\t00\tFD\thash\t8006\t6\t00\t00\t00\t00\tFD\thash\t1\n\
             smoke\t4\t4\t0,1,2,3\t1\t8000\t8008\t8\t00\t00\t00\t00\tFD\thash\t8008\t8\t00\t00\t00\t00\tFD\thash\t1\n\
             smoke\t2\t2\t4,5\t2\t8010\t8014\t4\t00\t00\t00\t00\tFD\thash\t8014\t4\t00\t00\t00\t00\tFD\thash\t1\n\
             other\t2\t2\t0,1\t3\t9000\t9004\t4\t00\t00\t00\t00\tFD\thash\t9004\t4\t00\t00\t00\t00\tFD\thash\t1\n",
        )
        .unwrap();

        let selected = write_maximal_outputs(
            &run_dir,
            &out_dir,
            &run_dir.join("native_block_run_cases.tsv"),
            &run_dir.join("native_block_run.tsv"),
            &run_dir.join("native_block_runtime_trace.tsv"),
        )
        .unwrap();
        assert_eq!(selected.len(), 3);
        let manifest = fs::read_to_string(out_dir.join("manifest.txt")).unwrap();
        assert!(manifest.contains("source_case_count=5\n"));
        assert!(manifest.contains("case_count=3\n"));
        assert!(manifest.contains("dropped_prefix_cases=2\n"));
        let selection = fs::read_to_string(out_dir.join("native_block_run_maximal.tsv")).unwrap();
        assert!(selection.contains("1\t\t0\tprefix_of_longer_path\tsmoke\t2\t0,1\n"));
        assert!(selection.contains("3\t1\t1\tmaximal_path\tsmoke\t4\t0,1,2,3\n"));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
