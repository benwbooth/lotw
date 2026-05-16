use crate::runtime_native_trace_verify;
use std::collections::{HashMap, HashSet};
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
struct CaseStats {
    case_count: u64,
    max_frame: u64,
    active_frames: u64,
}

#[derive(Debug, Default)]
struct RuntimeSummary {
    cases: u64,
    executed: u64,
    matched: u64,
    seeded: u64,
    continued: u64,
}

#[derive(Debug, Clone, Copy)]
struct ReplayArgs<'a> {
    replay: &'a Path,
    replay_dump: &'a Path,
}

pub fn run(
    lotw: &Path,
    run_dir: &Path,
    verify_command: &Path,
    out_dir: &Path,
    rom: &Path,
    replay: Option<&Path>,
    replay_dump: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cases = run_dir.join("native_block_run_cases.tsv");
    let expected_external = run_dir.join("native_block_run_external_writes.tsv");
    let run_manifest = run_dir.join("manifest.txt");
    for path in [lotw, &cases, verify_command, rom, &run_manifest] {
        require_file(path)?;
    }
    let replay_args =
        match (replay, replay_dump) {
            (Some(replay), Some(replay_dump)) => {
                require_file(replay)?;
                require_file(replay_dump)?;
                Some(ReplayArgs {
                    replay,
                    replay_dump,
                })
            }
            (None, None) => None,
            _ => return Err(
                "runtime_native_live_frame: replay and lotw_replay_dump must be provided together"
                    .into(),
            ),
        };

    let stats = read_case_stats(&cases)?;
    if stats.case_count == 0 {
        return Err(format!(
            "runtime_native_live_frame: no native block cases in {}",
            cases.display()
        )
        .into());
    }
    if stats.max_frame == 0 {
        return Err(
            "runtime_native_live_frame: cases are not sorted into a valid frame schedule".into(),
        );
    }

    let trace_dir = out_dir.join("trace");
    let trace_verify_dir = out_dir.join("trace_verify");
    let log = out_dir.join("runtime_native_live_frame.log");
    let summary_report = out_dir.join("runtime_native_live_frame_summary.txt");
    let audio_summary = out_dir.join("runtime_native_live_frame_audio_summary.txt");
    let input_trace = out_dir.join("port_input_trace.tsv");
    let expected_input_trace = out_dir.join("expected_input_trace.tsv");

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(&trace_dir)?;

    run_lotw(
        lotw,
        &stats,
        &trace_dir,
        &audio_summary,
        &cases,
        rom,
        replay_args,
        &input_trace,
        &log,
    )?;
    require_log_line(
        &log,
        &format!(
            "Native live blocks: {}/{} cases matched",
            stats.case_count, stats.case_count
        ),
    )?;

    let expected_external_arg = if has_data_rows(&expected_external)? {
        Some(expected_external.as_path())
    } else {
        None
    };
    runtime_native_trace_verify::run(
        run_dir,
        &trace_dir,
        &trace_verify_dir,
        "lotw_runtime_native_live",
        expected_external_arg,
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
    let trace_values = read_key_values(&trace_dir.join("port_trace_summary.txt"))?;
    let trace_frames = parse_required_u64(
        &trace_values,
        "frames",
        &trace_dir.join("port_trace_summary.txt"),
    )?;
    let trace_labels = parse_required_u64(
        &trace_values,
        "label_state_count",
        &trace_dir.join("port_trace_summary.txt"),
    )?;
    let expected_apu_writes = expected_apu_writes(&expected_external)?;
    let audio_values = read_key_values(&audio_summary)?;
    let audio_apu_writes =
        parse_required_u64(&audio_values, "audio_apu_write_count", &audio_summary)?;

    let input_trace_matches_replay = if let Some(args) = replay_args {
        write_expected_input_trace(
            args.replay_dump,
            args.replay,
            stats.max_frame,
            &expected_input_trace,
        )?;
        if !files_equal(&expected_input_trace, &input_trace)? {
            return Err(
                "runtime_native_live_frame: port input trace differs from replay parser output"
                    .into(),
            );
        }
        true
    } else {
        false
    };

    validate_summary(
        &stats,
        expected_continued,
        trace_frames,
        trace_labels,
        expected_apu_writes,
        audio_apu_writes,
        &actual,
    )?;
    write_summary(
        &summary_report,
        &stats,
        expected_continued,
        expected_apu_writes,
        audio_apu_writes,
        replay_args.map(|args| args.replay),
        input_trace_matches_replay,
        &actual,
    )?;

    println!("runtime_native_live_frame: wrote {}", trace_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "runtime_native_live_frame: missing input: {}",
                path.display()
            ),
        ))
    }
}

fn read_case_stats(path: &Path) -> io::Result<CaseStats> {
    let text = fs::read_to_string(path)?;
    let mut stats = CaseStats::default();
    let mut prev_frame = 0;
    let mut seen_frames = HashSet::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 6 {
            return invalid_tsv(path, line_no + 1, fields.len(), 6);
        }
        let frame = fields[5].parse::<u64>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid frame: {err}", path.display(), line_no + 1),
            )
        })?;
        if stats.case_count > 0 && frame < prev_frame {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime_native_live_frame: cases are not sorted into a valid frame schedule",
            ));
        }
        prev_frame = frame;
        stats.case_count += 1;
        stats.max_frame = stats.max_frame.max(frame);
        seen_frames.insert(frame);
    }
    stats.active_frames = seen_frames.len() as u64;
    Ok(stats)
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

#[allow(clippy::too_many_arguments)]
fn run_lotw(
    lotw: &Path,
    stats: &CaseStats,
    trace_dir: &Path,
    audio_summary: &Path,
    cases: &Path,
    rom: &Path,
    replay_args: Option<ReplayArgs<'_>>,
    input_trace: &Path,
    log: &Path,
) -> io::Result<()> {
    let log_file = fs::File::create(log)?;
    let log_stderr = log_file.try_clone()?;
    let mut command = Command::new(lotw);
    command
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .arg("--frames")
        .arg(stats.max_frame.to_string())
        .arg("--dump-trace-dir")
        .arg(trace_dir)
        .arg("--dump-audio-summary")
        .arg(audio_summary)
        .arg("--native-live-frame-cases")
        .arg(cases);
    if let Some(args) = replay_args {
        command
            .arg("--dump-input-trace")
            .arg(input_trace)
            .arg("--replay")
            .arg(args.replay);
    }
    let status = command
        .arg(rom)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_stderr))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "runtime_native_live_frame: lotw failed: {status}"
        )))
    }
}

fn require_log_line(path: &Path, needle: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text.lines().any(|line| line.contains(needle)) {
        Ok(())
    } else {
        Err(format!("runtime_native_live_frame: missing log line: {needle}").into())
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
    let actual = values.get(key).ok_or_else(|| {
        format!(
            "runtime_native_live_frame: missing {key} in {}",
            path.display()
        )
    })?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_live_frame: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn parse_required_u64(
    values: &HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<u64, Box<dyn std::error::Error>> {
    values
        .get(key)
        .ok_or_else(|| {
            format!(
                "runtime_native_live_frame: missing {key} in {}",
                path.display()
            )
        })?
        .parse::<u64>()
        .map_err(|err| {
            format!(
                "runtime_native_live_frame: invalid {key} in {}: {err}",
                path.display()
            )
            .into()
        })
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
        cases: parse_required_u64(&values, "cases", path)?,
        executed: parse_required_u64(&values, "executed", path)?,
        matched: parse_required_u64(&values, "matched", path)?,
        seeded: parse_required_u64(&values, "seeded", path)?,
        continued: parse_required_u64(&values, "continued", path)?,
    })
}

fn expected_continuations(cases: &Path, labels: &Path) -> io::Result<u64> {
    let cases = read_cases(cases)?;
    let labels = read_labels(labels)?;
    if cases.len() != labels.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "runtime_native_live_frame: case/label row count mismatch: {} != {}",
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

fn expected_apu_writes(path: &Path) -> io::Result<u64> {
    if !path.is_file() {
        return Ok(0);
    }
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .skip(1)
        .filter(|line| line.split('\t').next() == Some("apu"))
        .count() as u64)
}

fn write_expected_input_trace(
    replay_dump: &Path,
    replay: &Path,
    max_frame: u64,
    out: &Path,
) -> io::Result<()> {
    let output = Command::new(replay_dump)
        .arg(replay)
        .arg("--frames")
        .arg(max_frame.to_string())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "runtime_native_live_frame: replay dump failed: {}",
            output.status
        )));
    }
    let text = String::from_utf8(output.stdout).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("runtime_native_live_frame: replay dump output is not UTF-8: {err}"),
        )
    })?;
    let mut emit = false;
    let mut file = fs::File::create(out)?;
    for line in text.lines() {
        if line == "frame\tmask\tbuttons" {
            emit = true;
        }
        if emit {
            writeln!(file, "{line}")?;
        }
    }
    if !emit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "runtime_native_live_frame: replay dump output missing input trace header",
        ));
    }
    Ok(())
}

fn files_equal(left: &Path, right: &Path) -> io::Result<bool> {
    Ok(fs::read(left)? == fs::read(right)?)
}

#[allow(clippy::too_many_arguments)]
fn validate_summary(
    stats: &CaseStats,
    expected_continued: u64,
    trace_frames: u64,
    trace_labels: u64,
    expected_apu_writes: u64,
    audio_apu_writes: u64,
    actual: &RuntimeSummary,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected_seeded = stats.case_count - expected_continued;
    if actual.cases == stats.case_count
        && actual.executed == stats.case_count
        && actual.matched == stats.case_count
        && actual.seeded == expected_seeded
        && actual.continued == expected_continued
        && trace_frames == stats.max_frame
        && trace_labels == stats.case_count
        && audio_apu_writes == expected_apu_writes
    {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_live_frame: native live frame summary mismatch: cases={} executed={} matched={} seeded={} continued={} trace_frames={trace_frames} trace_labels={trace_labels} audio_apu_writes={audio_apu_writes}; expected cases={} seeded={expected_seeded} continued={expected_continued} frames={} labels={} audio_apu_writes={expected_apu_writes}",
            actual.cases,
            actual.executed,
            actual.matched,
            actual.seeded,
            actual.continued,
            stats.case_count,
            stats.max_frame,
            stats.case_count
        )
        .into())
    }
}

#[allow(clippy::too_many_arguments)]
fn write_summary(
    path: &Path,
    stats: &CaseStats,
    expected_continued: u64,
    expected_apu_writes: u64,
    audio_apu_writes: u64,
    replay: Option<&Path>,
    input_trace_matches_replay: bool,
    actual: &RuntimeSummary,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=lotw_runtime_native_live")?;
    writeln!(file, "case_count={}", stats.case_count)?;
    writeln!(file, "scheduled_frame_count={}", stats.max_frame)?;
    writeln!(file, "active_frame_count={}", stats.active_frames)?;
    writeln!(
        file,
        "idle_frame_count={}",
        stats.max_frame - stats.active_frames
    )?;
    writeln!(file, "executed={}", actual.executed)?;
    writeln!(file, "matched={}", actual.matched)?;
    writeln!(file, "seeded={}", actual.seeded)?;
    writeln!(file, "continued={}", actual.continued)?;
    writeln!(file, "expected_continued={expected_continued}")?;
    writeln!(file, "audio_apu_write_count={audio_apu_writes}")?;
    writeln!(file, "expected_audio_apu_write_count={expected_apu_writes}")?;
    if let Some(replay) = replay {
        writeln!(file, "replay={}", replay.display())?;
        writeln!(file, "input_trace=port_input_trace.tsv")?;
        writeln!(file, "expected_input_trace=expected_input_trace.tsv")?;
        writeln!(
            file,
            "input_trace_matches_replay={}",
            u8::from(input_trace_matches_replay)
        )?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, compile_noop, unique_temp_dir};
    use lotw_port::sha256;

    #[test]
    fn runs_frame_scheduled_runtime_with_replay_input_check() {
        let root = unique_temp_dir("runtime-native-live-frame");
        let run_dir = root.join("native_run");
        let out = root.join("out");
        let lotw = root.join("fake_lotw");
        let verify_command = root.join("runtime_native_trace_verify");
        let replay_dump = root.join("fake_replay_dump");
        let rom = root.join("game.nes");
        let replay = root.join("fixture.replay");
        fs::create_dir_all(&run_dir).unwrap();
        fs::write(&rom, "rom").unwrap();
        fs::write(&replay, "frame 2 start\n").unwrap();
        compile_noop(&verify_command);

        let zero_ram = "00".repeat(2048);
        let zero_sha = sha256::digest_hex(&vec![0u8; 2048]);
        write_run_dir(&run_dir, &zero_ram, &zero_sha);
        write_fake_lotw(&lotw, &zero_ram);
        write_fake_replay_dump(&replay_dump);

        run(
            &lotw,
            &run_dir,
            &verify_command,
            &out,
            &rom,
            Some(&replay),
            Some(&replay_dump),
        )
        .unwrap();

        let summary =
            fs::read_to_string(out.join("runtime_native_live_frame_summary.txt")).unwrap();
        assert!(summary.contains("case_count=2\n"));
        assert!(summary.contains("scheduled_frame_count=4\n"));
        assert!(summary.contains("active_frame_count=2\n"));
        assert!(summary.contains("idle_frame_count=2\n"));
        assert!(summary.contains("seeded=1\n"));
        assert!(summary.contains("continued=1\n"));
        assert!(summary.contains("audio_apu_write_count=1\n"));
        assert!(summary.contains("input_trace_matches_replay=1\n"));
        assert!(summary.contains("complete=1\n"));
    }

    fn write_run_dir(run_dir: &Path, zero_ram: &str, zero_sha: &str) {
        fs::write(
            run_dir.join("manifest.txt"),
            "runtime=native_block_live_frame_schedule\ncases=native_block_run_cases.tsv\nrun_report=native_block_run.tsv\nruntime_trace=native_block_runtime_trace.tsv\ncase_count=2\nmatched=2\nmismatches=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run_cases.tsv"),
            format!(
                "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
smoke\t1\t0\tC000\t1C000\t2\tC000\t00\t00\t00\t24\tFD\t{zero_ram}\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\n\
smoke\t1\t1\tC010\t1C010\t4\tC010\t01\t02\t03\t24\tF8\t{zero_ram}\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\n"
            ),
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            format!(
                "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match\n\
smoke\t1\t1\t0\t2\tC000\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\tC010\t5\t01\t02\t03\t24\tF8\t{zero_sha}\t1\n\
smoke\t1\t1\t1\t4\tC010\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\tC020\t9\t04\t05\t06\t25\tF7\t{zero_sha}\t1\n"
            ),
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run_external_writes.tsv"),
            "kind\tframe\taddr\tvalue\napu\t4\t4008\t00\n",
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
    let mut audio_summary = None;
    let mut input_trace = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dump-trace-dir" => {
                trace_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--dump-audio-summary" => {
                audio_summary = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--dump-input-trace" => {
                input_trace = Some(PathBuf::from(&args[i + 1]));
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
        "runtime=lotw_runtime_native_live\nframes=4\nmapper_write_count=0\napu_write_count=1\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=2\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_label_states.tsv"),
        "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC010\t1C010\t2\tC010\t01\t02\t03\t24\tF8\t{ZERO_RAM}\nC020\t1C020\t4\tC020\t04\t05\t06\t25\tF7\t{ZERO_RAM}\n",
    )
    .unwrap();
    fs::write(trace_dir.join("port_mapper_writes.tsv"), "frame\taddr\tvalue\tstate\n").unwrap();
    fs::write(
        trace_dir.join("port_apu_writes.tsv"),
        "frame\tcycle\taddr\tvalue\n4\tnative\t4008\t00\n",
    )
    .unwrap();
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
    fs::write(
        audio_summary.expect("missing audio summary"),
        "runtime=chr_preview\naudio_apu_write_count=1\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        input_trace.expect("missing input trace"),
        "frame\tmask\tbuttons\n1\t00\t\n2\t08\tstart\n3\t00\t\n4\t00\t\n",
    )
    .unwrap();
}
"#
        .replace("{ZERO_RAM}", zero_ram);
        compile(path, &source);
    }

    fn write_fake_replay_dump(path: &Path) {
        compile(
            path,
            r#"
fn main() {
    print!("runtime=lotw_replay_dump\nframe\tmask\tbuttons\n1\t00\t\n2\t08\tstart\n3\t00\t\n4\t00\t\n");
}
"#,
        );
    }
}
