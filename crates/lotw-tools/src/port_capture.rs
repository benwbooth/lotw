use lotw_port::sha256;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

const TRACE_FILES: &[(&str, &str)] = &[
    ("port_input_trace.tsv", "input_trace_sha256"),
    ("expected_input_trace.tsv", "expected_input_trace_sha256"),
    ("port_mapper_writes.tsv", "mapper_trace_sha256"),
    ("port_apu_writes.tsv", "apu_trace_sha256"),
    ("port_oam_dma.tsv", "oam_dma_trace_sha256"),
    ("port_ppu_writes.tsv", "ppu_trace_sha256"),
    ("port_ppu_vram_writes.tsv", "ppu_vram_trace_sha256"),
    ("port_label_states.tsv", "label_state_trace_sha256"),
    ("port_trace_summary.txt", "trace_summary_sha256"),
];

const REQUIRED_RUNTIME_TRACE_FILES: &[&str] = &[
    "port_mapper_writes.tsv",
    "port_apu_writes.tsv",
    "port_oam_dma.tsv",
    "port_ppu_writes.tsv",
    "port_ppu_vram_writes.tsv",
    "port_label_states.tsv",
    "port_trace_summary.txt",
];

struct PortSummary<'a> {
    out_dir: &'a Path,
    rom: &'a Path,
    rom_sha256: &'a str,
    replay: &'a Path,
    replay_sha256: &'a str,
    frame: u64,
    port_frame_file: &'a str,
    port_frame_hash: &'a str,
    trace_hashes: &'a HashMap<&'a str, String>,
    runtime: &'a str,
}

pub fn run(
    lotw: &Path,
    replay_dump: &Path,
    rom: &Path,
    out_dir: &Path,
    replay: &Path,
    frame: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in [lotw, replay_dump, rom, replay] {
        require_exists(path)?;
    }
    let frame = parse_positive_frame(frame)?;
    let frame_file = format!("frame_{frame:06}.ppm");
    let port_frame_file = format!("port_{frame_file}");
    let rom_sha256 = file_hash(rom)?;
    let replay_sha256 = file_hash(replay)?;

    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    run_lotw(lotw, rom, out_dir, replay, frame, &port_frame_file)?;
    require_file(&out_dir.join(&port_frame_file), "missing dumped frame")?;
    require_file(&out_dir.join("port_input_trace.tsv"), "missing input trace")?;
    for file in REQUIRED_RUNTIME_TRACE_FILES {
        require_file(&out_dir.join(file), &format!("missing trace file: {file}"))?;
    }

    let expected_input = replay_input_trace(replay_dump, replay, frame)?;
    fs::write(out_dir.join("expected_input_trace.tsv"), &expected_input)?;
    let port_input = fs::read(out_dir.join("port_input_trace.tsv"))?;
    if expected_input != port_input {
        return Err("port_capture: port input trace differs from replay parser output".into());
    }

    let port_frame_hash = file_hash(&out_dir.join(&port_frame_file))?;
    write_hash_file(
        &out_dir.join("port_frame_hashes.sha256"),
        &[(&port_frame_file, port_frame_hash.as_str())],
    )?;

    let mut trace_hashes = HashMap::new();
    let mut trace_hash_rows = Vec::new();
    for (file, _) in TRACE_FILES {
        let hash = file_hash(&out_dir.join(file))?;
        trace_hash_rows.push((*file, hash.clone()));
        trace_hashes.insert(*file, hash);
    }
    let trace_hash_refs = trace_hash_rows
        .iter()
        .map(|(file, hash)| (*file, hash.as_str()))
        .collect::<Vec<_>>();
    write_hash_file(&out_dir.join("port_trace_hashes.sha256"), &trace_hash_refs)?;

    let trace_summary = read_key_values(&out_dir.join("port_trace_summary.txt"))?;
    let runtime = required(
        &trace_summary,
        "runtime",
        &out_dir.join("port_trace_summary.txt"),
    )?;
    require_value(
        &trace_summary,
        "complete",
        "1",
        &out_dir.join("port_trace_summary.txt"),
    )?;

    write_summary(&PortSummary {
        out_dir,
        rom,
        rom_sha256: &rom_sha256,
        replay,
        replay_sha256: &replay_sha256,
        frame,
        port_frame_file: &port_frame_file,
        port_frame_hash: &port_frame_hash,
        trace_hashes: &trace_hashes,
        runtime,
    })?;
    let summary = read_key_values(&out_dir.join("port_summary.txt"))?;
    require_value(&summary, "complete", "1", &out_dir.join("port_summary.txt"))?;
    require_value(
        &summary,
        "input_trace_matches_replay",
        "1",
        &out_dir.join("port_summary.txt"),
    )?;

    println!("port_capture: wrote {}", out_dir.display());
    Ok(())
}

fn run_lotw(
    lotw: &Path,
    rom: &Path,
    out_dir: &Path,
    replay: &Path,
    frame: u64,
    port_frame_file: &str,
) -> io::Result<()> {
    let log_path = out_dir.join("port_run.log");
    let log_file = fs::File::create(&log_path)?;
    let err_file = log_file.try_clone()?;
    let mut command = Command::new(lotw);
    command
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .arg("--frames")
        .arg(frame.to_string())
        .arg("--dump-frame")
        .arg(out_dir.join(port_frame_file))
        .arg("--dump-frame-number")
        .arg(frame.to_string())
        .arg("--dump-input-trace")
        .arg(out_dir.join("port_input_trace.tsv"))
        .arg("--dump-trace-dir")
        .arg(out_dir)
        .arg("--replay")
        .arg(replay)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));

    if let Ok(path) = env::var("LOTW_PORT_TRACE_SEED_DIR") {
        if !path.is_empty() {
            command.arg("--trace-seed-dir").arg(path);
        }
    }
    if let Ok(path) = env::var("LOTW_PORT_RENDER_TRACE_DIR") {
        if !path.is_empty() {
            command.arg("--render-trace-dir").arg(path);
        }
    }
    if let Ok(frame) = env::var("LOTW_PORT_RENDER_TRACE_FRAME") {
        if !frame.is_empty() {
            command.arg("--render-trace-frame").arg(frame);
        }
    }
    command.arg(rom);

    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "port_capture: lotw failed with {status}; see {}",
            log_path.display()
        )))
    }
}

fn replay_input_trace(replay_dump: &Path, replay: &Path, frame: u64) -> io::Result<Vec<u8>> {
    let output = Command::new(replay_dump)
        .arg(replay)
        .arg("--frames")
        .arg(frame.to_string())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "port_capture: replay dump failed with {}",
            output.status
        )));
    }

    let mut out = Vec::new();
    let mut emit = false;
    for line in output.stdout.split_inclusive(|byte| *byte == b'\n') {
        let without_newline = line.strip_suffix(b"\n").unwrap_or(line);
        if without_newline == b"frame\tmask\tbuttons" {
            emit = true;
        }
        if emit {
            out.extend_from_slice(line);
        }
    }
    if out.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "port_capture: replay dump did not emit input trace header",
        ));
    }
    Ok(out)
}

fn write_summary(summary: &PortSummary<'_>) -> io::Result<()> {
    let mut file = fs::File::create(summary.out_dir.join("port_summary.txt"))?;
    writeln!(file, "rom={}", summary.rom.display())?;
    writeln!(file, "rom_sha256={}", summary.rom_sha256)?;
    writeln!(file, "replay={}", summary.replay.display())?;
    writeln!(file, "replay_sha256={}", summary.replay_sha256)?;
    writeln!(file, "frame={}", summary.frame)?;
    writeln!(file, "port_frame={}", summary.port_frame_file)?;
    writeln!(file, "port_frame_hash={}", summary.port_frame_hash)?;
    writeln!(file, "input_trace=port_input_trace.tsv")?;
    writeln!(
        file,
        "input_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_input_trace.tsv")?
    )?;
    writeln!(file, "expected_input_trace=expected_input_trace.tsv")?;
    writeln!(
        file,
        "expected_input_trace_sha256={}",
        required_hash(summary.trace_hashes, "expected_input_trace.tsv")?
    )?;
    writeln!(file, "input_trace_matches_replay=1")?;
    writeln!(file, "mapper_trace=port_mapper_writes.tsv")?;
    writeln!(
        file,
        "mapper_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_mapper_writes.tsv")?
    )?;
    writeln!(file, "apu_trace=port_apu_writes.tsv")?;
    writeln!(
        file,
        "apu_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_apu_writes.tsv")?
    )?;
    writeln!(file, "oam_dma_trace=port_oam_dma.tsv")?;
    writeln!(
        file,
        "oam_dma_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_oam_dma.tsv")?
    )?;
    writeln!(file, "ppu_trace=port_ppu_writes.tsv")?;
    writeln!(
        file,
        "ppu_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_ppu_writes.tsv")?
    )?;
    writeln!(file, "ppu_vram_trace=port_ppu_vram_writes.tsv")?;
    writeln!(
        file,
        "ppu_vram_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_ppu_vram_writes.tsv")?
    )?;
    writeln!(file, "label_state_trace=port_label_states.tsv")?;
    writeln!(
        file,
        "label_state_trace_sha256={}",
        required_hash(summary.trace_hashes, "port_label_states.tsv")?
    )?;
    writeln!(file, "trace_summary=port_trace_summary.txt")?;
    writeln!(
        file,
        "trace_summary_sha256={}",
        required_hash(summary.trace_hashes, "port_trace_summary.txt")?
    )?;
    writeln!(file, "runtime={}", summary.runtime)?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn required_hash<'a>(hashes: &'a HashMap<&str, String>, file: &str) -> io::Result<&'a str> {
    hashes
        .get(file)
        .map(String::as_str)
        .ok_or_else(|| io::Error::other(format!("port_capture: missing hash for {file}")))
}

fn require_exists(path: &Path) -> io::Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("port_capture: file not found: {}", path.display()),
        ))
    }
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("port_capture: {message}: {}", path.display()),
        ))
    }
}

fn parse_positive_frame(value: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let frame = value
        .parse::<u64>()
        .map_err(|err| format!("port_capture: frame must be a positive integer: {err}"))?;
    if frame == 0 {
        return Err("port_capture: frame must be a positive integer".into());
    }
    Ok(frame)
}

fn file_hash(path: &Path) -> io::Result<String> {
    Ok(sha256::digest_hex(&fs::read(path)?))
}

fn write_hash_file(path: &Path, rows: &[(&str, &str)]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    for (name, hash) in rows {
        writeln!(file, "{hash}  {name}")?;
    }
    Ok(())
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
        .ok_or_else(|| format!("port_capture: missing {key} in {}", path.display()).into())
}

fn require_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = required(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "port_capture: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn remove_path(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, unique_temp_dir};

    #[test]
    fn captures_port_outputs_and_writes_summary() {
        let root = unique_temp_dir("port-capture");
        let lotw = root.join("fake_lotw");
        let replay_dump = root.join("fake_replay_dump");
        let rom = root.join("game.nes");
        let replay = root.join("input.replay");
        let out = root.join("out");
        write_fake_lotw(&lotw);
        write_fake_replay_dump(&replay_dump);
        fs::write(&rom, b"rom").unwrap();
        fs::write(&replay, b"replay").unwrap();

        run(&lotw, &replay_dump, &rom, &out, &replay, "3").unwrap();

        let summary = fs::read_to_string(out.join("port_summary.txt")).unwrap();
        assert!(summary.contains("frame=3\n"));
        assert!(summary.contains("port_frame=port_frame_000003.ppm\n"));
        assert!(summary.contains("input_trace_matches_replay=1\n"));
        assert!(summary.contains("runtime=chr_preview\n"));
        assert!(summary.contains("complete=1\n"));
        let frame_hashes = fs::read_to_string(out.join("port_frame_hashes.sha256")).unwrap();
        assert!(frame_hashes.contains("  port_frame_000003.ppm\n"));
        let trace_hashes = fs::read_to_string(out.join("port_trace_hashes.sha256")).unwrap();
        assert!(trace_hashes.contains("  port_input_trace.tsv\n"));
        assert!(trace_hashes.contains("  expected_input_trace.tsv\n"));
    }

    fn write_fake_lotw(path: &Path) {
        compile(
            path,
            r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut out_dir = None;
    let mut frame_path = None;
    let mut input_trace = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dump-frame" => {
                frame_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--dump-input-trace" => {
                input_trace = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--dump-trace-dir" => {
                out_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }
    let out_dir = out_dir.expect("missing trace dir");
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(frame_path.expect("missing frame path"), b"P6\n1 1\n255\nabc").unwrap();
    fs::write(
        input_trace.expect("missing input trace"),
        "frame\tmask\tbuttons\n1\t01\tA\n3\t02\tB\n",
    )
    .unwrap();
    fs::write(
        out_dir.join("port_trace_summary.txt"),
        "runtime=chr_preview\nframes=3\nmapper_write_count=0\napu_write_count=0\noam_dma_count=0\nppu_write_count=0\nppu_vram_write_count=0\nlabel_state_count=0\ncomplete=1\n",
    )
    .unwrap();
    fs::write(out_dir.join("port_mapper_writes.tsv"), "frame\taddr\tvalue\tstate\n").unwrap();
    fs::write(out_dir.join("port_apu_writes.tsv"), "frame\tcycle\taddr\tvalue\n").unwrap();
    fs::write(
        out_dir.join("port_oam_dma.tsv"),
        "frame\tcycle\tpage\tbytes_0000_00ff\n",
    )
    .unwrap();
    fs::write(
        out_dir.join("port_ppu_writes.tsv"),
        "frame\tcycle\taddr\tregister\tvalue\n",
    )
    .unwrap();
    fs::write(
        out_dir.join("port_ppu_vram_writes.tsv"),
        "frame\tcycle\taddr\tregion\tvalue\n",
    )
    .unwrap();
    fs::write(
        out_dir.join("port_label_states.tsv"),
        "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n",
    )
    .unwrap();
}
"#,
        );
    }

    fn write_fake_replay_dump(path: &Path) {
        compile(
            path,
            r#"
fn main() {
    print!("replay=input.replay\nframes=3\nframe\tmask\tbuttons\n1\t01\tA\n3\t02\tB\n");
}
"#,
        );
    }
}
