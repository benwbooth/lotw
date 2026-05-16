use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const RUN_HEADER: &str = "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tdispatch_match\tpath_match\tmetadata_match\tregister_match\tcycles_match\tram_match\tmatch\tfinal_pc\tcycles";
const TRACE_HEADER: &str = "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match";

#[derive(Debug, Clone)]
struct RunRow {
    replay: String,
    expected_steps: String,
    executed_steps: String,
    executed_path_indices: String,
    final_pc: String,
    cycles: String,
}

#[derive(Debug, Default)]
struct Stats {
    run_bad_header: u64,
    trace_bad_header: u64,
    run_rows: u64,
    trace_rows: u64,
    run_match_count: u64,
    run_mismatch_rows: u64,
    state_match_count: u64,
    state_mismatch_rows: u64,
    step_mismatch_rows: u64,
    missing_run_rows: u64,
    run_trace_mismatch_rows: u64,
}

pub fn run(run_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = run_dir.join("manifest.txt");
    let run_report = run_dir.join("native_block_run.tsv");
    let runtime_trace = run_dir.join("native_block_runtime_trace.tsv");
    for path in [&manifest, &run_report, &runtime_trace] {
        require_file(path)?;
    }

    fs::create_dir_all(out_dir)?;
    let report = out_dir.join("native_block_runtime_trace_verify.txt");
    let manifest_values = read_key_values(&manifest)?;
    let manifest_cases = require_manifest_value(&manifest_values, "case_count", &manifest)?;
    let manifest_matched = require_manifest_value(&manifest_values, "matched", &manifest)?;
    let manifest_mismatches = require_manifest_value(&manifest_values, "mismatches", &manifest)?;
    let manifest_runtime_trace =
        require_manifest_value(&manifest_values, "runtime_trace", &manifest)?;

    let stats = collect_stats(&run_report, &runtime_trace)?;
    write_report(
        &report,
        run_dir,
        &manifest,
        &run_report,
        &runtime_trace,
        manifest_cases,
        manifest_matched,
        manifest_mismatches,
        manifest_runtime_trace,
        &stats,
    )?;

    validate(
        manifest_cases,
        manifest_matched,
        manifest_mismatches,
        manifest_runtime_trace,
        &stats,
    )?;

    println!(
        "native_block_runtime_trace_verify: wrote {}",
        report.display()
    );
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "native_block_runtime_trace_verify: missing input: {}",
                path.display()
            ),
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

fn require_manifest_value<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values.get(key).map(String::as_str).ok_or_else(|| {
        format!(
            "native_block_runtime_trace_verify: missing {key} in {}",
            path.display()
        )
        .into()
    })
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

fn collect_stats(run_report: &Path, runtime_trace: &Path) -> io::Result<Stats> {
    let mut stats = Stats::default();
    let run_rows = read_run_rows(run_report, &mut stats)?;
    read_trace_rows(runtime_trace, &run_rows, &mut stats)?;
    Ok(stats)
}

fn read_run_rows(run_report: &Path, stats: &mut Stats) -> io::Result<Vec<RunRow>> {
    let text = fs::read_to_string(run_report)?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if header != RUN_HEADER {
        stats.run_bad_header = 1;
    }

    let mut rows = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return invalid_tsv(run_report, line_no + 2, fields.len(), 13);
        }
        stats.run_rows += 1;
        stats.run_match_count += parse_count_field(run_report, line_no + 2, fields[10])?;
        if fields[4] != "1"
            || fields[5] != "1"
            || fields[6] != "1"
            || fields[7] != "1"
            || fields[8] != "1"
            || fields[9] != "1"
            || fields[10] != "1"
        {
            stats.run_mismatch_rows += 1;
        }
        rows.push(RunRow {
            replay: fields[0].to_string(),
            expected_steps: fields[1].to_string(),
            executed_steps: fields[2].to_string(),
            executed_path_indices: fields[3].to_string(),
            final_pc: fields[11].to_string(),
            cycles: fields[12].to_string(),
        });
    }
    Ok(rows)
}

fn read_trace_rows(runtime_trace: &Path, run_rows: &[RunRow], stats: &mut Stats) -> io::Result<()> {
    let text = fs::read_to_string(runtime_trace)?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if header != TRACE_HEADER {
        stats.trace_bad_header = 1;
    }

    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 23 {
            return invalid_tsv(runtime_trace, line_no + 2, fields.len(), 23);
        }
        stats.trace_rows += 1;
        stats.state_match_count += parse_count_field(runtime_trace, line_no + 2, fields[22])?;
        if fields[22] != "1" {
            stats.state_mismatch_rows += 1;
        }
        if fields[1] != fields[2] {
            stats.step_mismatch_rows += 1;
        }
        match run_rows.get(line_no) {
            Some(run_row) => {
                if fields[0] != run_row.replay
                    || fields[1] != run_row.expected_steps
                    || fields[2] != run_row.executed_steps
                    || fields[3] != run_row.executed_path_indices
                    || fields[6] != run_row.final_pc
                    || fields[7] != run_row.cycles
                {
                    stats.run_trace_mismatch_rows += 1;
                }
            }
            None => {
                stats.missing_run_rows += 1;
            }
        }
    }
    Ok(())
}

fn parse_count_field(path: &Path, line_no: usize, value: &str) -> io::Result<u64> {
    value.parse::<u64>().map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{}:{line_no} invalid numeric field {value:?}: {err}",
                path.display()
            ),
        )
    })
}

#[allow(clippy::too_many_arguments)]
fn write_report(
    report: &Path,
    run_dir: &Path,
    manifest: &Path,
    run_report: &Path,
    runtime_trace: &Path,
    manifest_cases: &str,
    manifest_matched: &str,
    manifest_mismatches: &str,
    manifest_runtime_trace: &str,
    stats: &Stats,
) -> io::Result<()> {
    let mut file = fs::File::create(report)?;
    writeln!(file, "native_block_run={}", run_dir.display())?;
    writeln!(file, "manifest={}", manifest.display())?;
    writeln!(file, "run_report={}", run_report.display())?;
    writeln!(file, "runtime_trace={}", runtime_trace.display())?;
    writeln!(file, "manifest_case_count={manifest_cases}")?;
    writeln!(file, "manifest_matched={manifest_matched}")?;
    writeln!(file, "manifest_mismatches={manifest_mismatches}")?;
    writeln!(file, "manifest_runtime_trace={manifest_runtime_trace}")?;
    writeln!(file, "run_bad_header={}", stats.run_bad_header)?;
    writeln!(file, "trace_bad_header={}", stats.trace_bad_header)?;
    writeln!(file, "run_rows={}", stats.run_rows)?;
    writeln!(file, "trace_rows={}", stats.trace_rows)?;
    writeln!(file, "run_match_count={}", stats.run_match_count)?;
    writeln!(file, "run_mismatch_rows={}", stats.run_mismatch_rows)?;
    writeln!(file, "state_match_count={}", stats.state_match_count)?;
    writeln!(file, "state_mismatch_rows={}", stats.state_mismatch_rows)?;
    writeln!(file, "step_mismatch_rows={}", stats.step_mismatch_rows)?;
    writeln!(file, "missing_run_rows={}", stats.missing_run_rows)?;
    writeln!(
        file,
        "run_trace_mismatch_rows={}",
        stats.run_trace_mismatch_rows
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn validate(
    manifest_cases: &str,
    manifest_matched: &str,
    manifest_mismatches: &str,
    manifest_runtime_trace: &str,
    stats: &Stats,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_cases = manifest_cases.parse::<u64>().map_err(|err| {
        format!("native_block_runtime_trace_verify: invalid manifest case_count: {err}")
    })?;
    let manifest_matched = manifest_matched.parse::<u64>().map_err(|err| {
        format!("native_block_runtime_trace_verify: invalid manifest matched: {err}")
    })?;
    let manifest_mismatches = manifest_mismatches.parse::<u64>().map_err(|err| {
        format!("native_block_runtime_trace_verify: invalid manifest mismatches: {err}")
    })?;

    let mut failures = Vec::new();
    if manifest_runtime_trace != "native_block_runtime_trace.tsv" {
        failures.push("manifest runtime_trace mismatch");
    }
    if stats.run_bad_header != 0 || stats.trace_bad_header != 0 {
        failures.push("bad header");
    }
    if stats.run_rows != manifest_cases || stats.trace_rows != manifest_cases {
        failures.push("row count mismatch");
    }
    if stats.run_match_count != manifest_matched || manifest_mismatches != 0 {
        failures.push("manifest match count mismatch");
    }
    if stats.run_mismatch_rows != 0 {
        failures.push("run report contains mismatches");
    }
    if stats.state_match_count != manifest_cases || stats.state_mismatch_rows != 0 {
        failures.push("runtime trace state mismatch");
    }
    if stats.step_mismatch_rows != 0
        || stats.missing_run_rows != 0
        || stats.run_trace_mismatch_rows != 0
    {
        failures.push("runtime trace does not align with run report");
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("native_block_runtime_trace_verify: {}", failures.join("; ")).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_runtime_trace_report() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_runtime_trace_verify_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("native_block_run");
        let verify_dir = root.join("verify");
        fs::create_dir_all(&run_dir).unwrap();

        fs::write(
            run_dir.join("manifest.txt"),
            "cases=/tmp/native_block_run_cases.tsv\n\
             run_report=native_block_run.tsv\n\
             runtime_trace=native_block_runtime_trace.tsv\n\
             case_count=2\n\
             matched=2\n\
             mismatches=0\n\
             scope=pc-dispatched generated native block runs and translated-native final RAM/register trace versus block-exec oracle\n\
             complete=1\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            format!(
                "{RUN_HEADER}\n\
                 smoke\t2\t2\t0,1\t1\t1\t1\t1\t1\t1\t1\tD415\t9\n\
                 smoke\t3\t3\t0,1,2\t1\t1\t1\t1\t1\t1\t1\tD40F\t14\n"
            ),
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            format!(
                "{TRACE_HEADER}\n\
                 smoke\t2\t2\t0,1\t1\tD40D\tD415\t9\t00\t07\t00\t02\tFD\te5a00aa9991ac8a5ee3109844d84a55583bd20572ad3ffcd42792f3c36b183ad\tD415\t9\t00\t07\t00\t02\tFD\te5a00aa9991ac8a5ee3109844d84a55583bd20572ad3ffcd42792f3c36b183ad\t1\n\
                 smoke\t3\t3\t0,1,2\t1\tD40D\tD40F\t14\t00\t06\t00\t00\tFD\te5a00aa9991ac8a5ee3109844d84a55583bd20572ad3ffcd42792f3c36b183ad\tD40F\t14\t00\t06\t00\t00\tFD\te5a00aa9991ac8a5ee3109844d84a55583bd20572ad3ffcd42792f3c36b183ad\t1\n"
            ),
        )
        .unwrap();

        run(&run_dir, &verify_dir).unwrap();
        let report =
            fs::read_to_string(verify_dir.join("native_block_runtime_trace_verify.txt")).unwrap();
        assert!(report.contains("complete=1\n"));
        assert!(report.contains("run_rows=2\n"));
        assert!(report.contains("trace_rows=2\n"));
        assert!(report.contains("run_match_count=2\n"));
        assert!(report.contains("state_match_count=2\n"));
        assert!(report.contains("run_trace_mismatch_rows=0\n"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_misaligned_runtime_trace() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_runtime_trace_verify_bad_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("native_block_run");
        let verify_dir = root.join("verify");
        fs::create_dir_all(&run_dir).unwrap();

        fs::write(
            run_dir.join("manifest.txt"),
            "runtime_trace=native_block_runtime_trace.tsv\ncase_count=1\nmatched=1\nmismatches=0\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            format!("{RUN_HEADER}\nsmoke\t2\t2\t0,1\t1\t1\t1\t1\t1\t1\t1\tD415\t9\n"),
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            format!(
                "{TRACE_HEADER}\nsmoke\t2\t2\t0,1\t1\tD40D\tD416\t9\t00\t07\t00\t02\tFD\thash\tD415\t9\t00\t07\t00\t02\tFD\thash\t1\n"
            ),
        )
        .unwrap();

        let err = run(&run_dir, &verify_dir).unwrap_err();
        assert!(err.to_string().contains("does not align"));
        assert!(verify_dir
            .join("native_block_runtime_trace_verify.txt")
            .is_file());

        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
