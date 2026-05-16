use crate::runtime_native_trace_verify;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
struct CaseState {
    replay: String,
    pc: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram: String,
}

#[derive(Debug, Clone)]
struct LabelState {
    pc: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram: String,
}

#[derive(Debug, Default)]
struct RuntimeSummary {
    cases: u64,
    executed: u64,
    matched: u64,
    seeded: u64,
    continued: u64,
}

pub fn run(
    lotw: &Path,
    run_dir: &Path,
    verify_command: &Path,
    out_dir: &Path,
    rom: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let cases = run_dir.join("native_block_run_cases.tsv");
    let expected_external = run_dir.join("native_block_run_external_writes.tsv");
    for path in [lotw, &cases, verify_command, rom] {
        require_file(path)?;
    }

    let case_count = count_data_rows(&cases)?;
    if case_count == 0 {
        return Err(format!(
            "runtime_native_live: no native block cases in {}",
            cases.display()
        )
        .into());
    }

    let trace_dir = out_dir.join("trace");
    let trace_verify_dir = out_dir.join("trace_verify");
    let log = out_dir.join("runtime_native_live.log");
    let summary_report = out_dir.join("runtime_native_live_summary.txt");

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(&trace_dir)?;

    run_lotw(lotw, case_count, &trace_dir, &cases, rom, &log)?;
    require_log_line(
        &log,
        &format!("Native live blocks: {case_count}/{case_count} cases matched"),
    )?;

    let expected_external = if has_data_rows(&expected_external)? {
        Some(expected_external.as_path())
    } else {
        None
    };
    runtime_native_trace_verify::run(
        run_dir,
        &trace_dir,
        &trace_verify_dir,
        "lotw_runtime_native_live",
        expected_external,
    )?;
    require_key_value(
        &read_key_values(&trace_verify_dir.join("runtime_native_trace_verify.txt"))?,
        "complete",
        "1",
        &trace_verify_dir.join("runtime_native_trace_verify.txt"),
    )?;

    let actual = read_runtime_summary(&log)?;
    let expected_continued =
        expected_continuations(&cases, &trace_dir.join("port_label_states.tsv"))?;
    validate_summary(case_count, expected_continued, &actual)?;
    write_summary(&summary_report, case_count, expected_continued, &actual)?;

    println!("runtime_native_live: wrote {}", trace_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("runtime_native_live: missing input: {}", path.display()),
        ))
    }
}

fn count_data_rows(path: &Path) -> io::Result<u64> {
    Ok(fs::read_to_string(path)?
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .count() as u64)
}

fn has_data_rows(path: &Path) -> io::Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    Ok(fs::read_to_string(path)?
        .lines()
        .skip(1)
        .any(|line| !line.is_empty()))
}

fn run_lotw(
    lotw: &Path,
    case_count: u64,
    trace_dir: &Path,
    cases: &Path,
    rom: &Path,
    log: &Path,
) -> io::Result<()> {
    let log_file = fs::File::create(log)?;
    let log_stderr = log_file.try_clone()?;
    let status = Command::new(lotw)
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .arg("--frames")
        .arg(case_count.to_string())
        .arg("--dump-trace-dir")
        .arg(trace_dir)
        .arg("--native-live-cases")
        .arg(cases)
        .arg(rom)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_stderr))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "runtime_native_live: lotw failed: {status}"
        )))
    }
}

fn require_log_line(path: &Path, needle: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text.lines().any(|line| line.contains(needle)) {
        Ok(())
    } else {
        Err(format!("runtime_native_live: missing log line: {needle}").into())
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

fn require_key_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = values
        .get(key)
        .ok_or_else(|| format!("runtime_native_live: missing {key} in {}", path.display()))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_live: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn read_runtime_summary(path: &Path) -> Result<RuntimeSummary, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("Native live summary:") {
            for token in rest.split_whitespace() {
                if let Some((key, value)) = token.split_once('=') {
                    values.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    Ok(RuntimeSummary {
        cases: parse_u64(&values, "cases", path)?,
        executed: parse_u64(&values, "executed", path)?,
        matched: parse_u64(&values, "matched", path)?,
        seeded: parse_u64(&values, "seeded", path)?,
        continued: parse_u64(&values, "continued", path)?,
    })
}

fn parse_u64(
    values: &HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<u64, Box<dyn std::error::Error>> {
    values
        .get(key)
        .ok_or_else(|| {
            format!(
                "runtime_native_live: missing summary {key} in {}",
                path.display()
            )
        })?
        .parse::<u64>()
        .map_err(|err| {
            format!(
                "runtime_native_live: invalid summary {key} in {}: {err}",
                path.display()
            )
            .into()
        })
}

fn expected_continuations(cases: &Path, labels: &Path) -> io::Result<u64> {
    let cases = read_cases(cases)?;
    let labels = read_labels(labels)?;
    if cases.len() != labels.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "runtime_native_live: case/label row count mismatch: {} != {}",
                cases.len(),
                labels.len()
            ),
        ));
    }

    let mut continued = 0;
    for index in 0..cases.len().saturating_sub(1) {
        let final_state = &labels[index];
        let next = &cases[index + 1];
        if cases[index].replay == next.replay
            && final_state.pc == next.pc
            && final_state.a == next.a
            && final_state.x == next.x
            && final_state.y == next.y
            && final_state.p == next.p
            && final_state.s == next.s
            && final_state.ram == next.ram
        {
            continued += 1;
        }
    }
    Ok(continued)
}

fn read_cases(path: &Path) -> io::Result<Vec<CaseState>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 13 {
            return invalid_tsv(path, line_no + 1, fields.len(), 13);
        }
        rows.push(CaseState {
            replay: fields[0].to_string(),
            pc: fields[6].to_string(),
            a: fields[7].to_string(),
            x: fields[8].to_string(),
            y: fields[9].to_string(),
            p: fields[10].to_string(),
            s: fields[11].to_string(),
            ram: fields[12].to_ascii_lowercase(),
        });
    }
    Ok(rows)
}

fn read_labels(path: &Path) -> io::Result<Vec<LabelState>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 10 {
            return invalid_tsv(path, line_no + 1, fields.len(), 10);
        }
        rows.push(LabelState {
            pc: fields[0].to_string(),
            a: fields[4].to_string(),
            x: fields[5].to_string(),
            y: fields[6].to_string(),
            p: fields[7].to_string(),
            s: fields[8].to_string(),
            ram: fields[9].to_ascii_lowercase(),
        });
    }
    Ok(rows)
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

fn validate_summary(
    case_count: u64,
    expected_continued: u64,
    actual: &RuntimeSummary,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected_seeded = case_count - expected_continued;
    if actual.cases == case_count
        && actual.executed == case_count
        && actual.matched == case_count
        && actual.seeded == expected_seeded
        && actual.continued == expected_continued
    {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_live: native live summary mismatch: cases={} executed={} matched={} seeded={} continued={}, expected cases={case_count} seeded={expected_seeded} continued={expected_continued}",
            actual.cases, actual.executed, actual.matched, actual.seeded, actual.continued
        )
        .into())
    }
}

fn write_summary(
    path: &Path,
    case_count: u64,
    expected_continued: u64,
    actual: &RuntimeSummary,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=lotw_runtime_native_live")?;
    writeln!(file, "case_count={case_count}")?;
    writeln!(file, "executed={}", actual.executed)?;
    writeln!(file, "matched={}", actual.matched)?;
    writeln!(file, "seeded={}", actual.seeded)?;
    writeln!(file, "continued={}", actual.continued)?;
    writeln!(file, "expected_continued={expected_continued}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, compile_noop, unique_temp_dir};
    use lotw_port::sha256;

    #[test]
    fn runs_live_runtime_and_verifies_continuations() {
        let root = unique_temp_dir("runtime-native-live");
        let run_dir = root.join("native_run");
        let out = root.join("out");
        let lotw = root.join("fake_lotw");
        let verify_command = root.join("runtime_native_trace_verify");
        let rom = root.join("game.nes");
        fs::create_dir_all(&run_dir).unwrap();
        fs::write(&rom, "rom").unwrap();
        compile_noop(&verify_command);

        let zero_ram = "00".repeat(2048);
        let zero_sha = sha256::digest_hex(&vec![0u8; 2048]);
        write_run_dir(&run_dir, &zero_ram, &zero_sha);
        write_fake_lotw(&lotw, &zero_ram);

        run(&lotw, &run_dir, &verify_command, &out, &rom).unwrap();

        let summary = fs::read_to_string(out.join("runtime_native_live_summary.txt")).unwrap();
        assert!(summary.contains("case_count=2\n"));
        assert!(summary.contains("seeded=1\n"));
        assert!(summary.contains("continued=1\n"));
        assert!(summary.contains("expected_continued=1\n"));
        assert!(summary.contains("complete=1\n"));
        let trace_verify =
            fs::read_to_string(out.join("trace_verify/runtime_native_trace_verify.txt")).unwrap();
        assert!(trace_verify.contains("complete=1\n"));
    }

    fn write_run_dir(run_dir: &Path, zero_ram: &str, zero_sha: &str) {
        fs::write(
            run_dir.join("manifest.txt"),
            "runtime=native_block_run\ncases=native_block_run_cases.tsv\nrun_report=native_block_run.tsv\nruntime_trace=native_block_runtime_trace.tsv\ncase_count=2\nmatched=2\nmismatches=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run_cases.tsv"),
            format!(
                "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
smoke\t1\t0\tC000\t1C000\t7\tC000\t00\t00\t00\t24\tFD\t{zero_ram}\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\n\
smoke\t1\t1\tC010\t1C010\t8\tC010\t01\t02\t03\t24\tF8\t{zero_ram}\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\n"
            ),
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tdispatch_match\tpath_match\tmetadata_match\tregister_match\tcycles_match\tram_match\tmatch\tfinal_pc\tcycles\n\
smoke\t1\t1\t0\t1\t1\t1\t1\t1\t1\t1\tC010\t5\n\
smoke\t1\t1\t1\t1\t1\t1\t1\t1\t1\t1\tC020\t9\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            format!(
                "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match\n\
smoke\t1\t1\t0\t7\tC000\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\t1\n\
smoke\t1\t1\t1\t8\tC010\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\t1\n"
            ),
        )
        .unwrap();
    }

    fn write_fake_lotw(path: &Path, zero_ram: &str) {
        let source = r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut trace_dir = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dump-trace-dir" => {
                trace_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }
    let trace_dir = trace_dir.expect("missing trace dir");
    fs::create_dir_all(&trace_dir).unwrap();
    println!("Native live blocks: 2/2 cases matched");
    println!("Native live summary: cases=2 executed=2 matched=2 seeded=1 continued=1");
    fs::write(
        trace_dir.join("port_trace_summary.txt"),
        "runtime=lotw_runtime_native_live\nframes=2\nmapper_write_count=0\napu_write_count=0\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=2\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_label_states.tsv"),
        "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC010\t1C010\t7\tC010\t01\t02\t03\t24\tF8\t{ZERO_RAM}\nC020\t1C020\t8\tC020\t04\t05\t06\t25\tF7\t{ZERO_RAM}\n",
    )
    .unwrap();
    fs::write(trace_dir.join("port_mapper_writes.tsv"), "frame\taddr\tvalue\tstate\n").unwrap();
    fs::write(trace_dir.join("port_apu_writes.tsv"), "frame\tcycle\taddr\tvalue\n").unwrap();
    fs::write(
        trace_dir.join("port_oam_dma.tsv"),
        "frame\tcycle\tpage\tbytes_0000_00ff\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_ppu_writes.tsv"),
        "frame\tcycle\taddr\tregister\tvalue\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_ppu_vram_writes.tsv"),
        "frame\tcycle\taddr\tregion\tvalue\n",
    )
    .unwrap();
}
"#
        .replace("{ZERO_RAM}", zero_ram);
        compile(path, &source);
    }
}
