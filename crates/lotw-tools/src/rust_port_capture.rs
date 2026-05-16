use lotw_port::replay::{input_trace_tsv, Replay};
use lotw_port::sha256;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

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

const REQUIRED_TRACE_FILES: &[&str] = &[
    "port_mapper_writes.tsv",
    "port_apu_writes.tsv",
    "port_oam_dma.tsv",
    "port_ppu_writes.tsv",
    "port_ppu_vram_writes.tsv",
    "port_label_states.tsv",
    "port_trace_summary.txt",
];

struct CaptureSummary<'a> {
    capture_dir: &'a Path,
    rom: &'a Path,
    rom_sha256: &'a str,
    replay: &'a Path,
    replay_sha256: &'a str,
    frame: usize,
    port_frame_file: &'a str,
    port_frame_hash: &'a str,
    trace_hashes: &'a HashMap<&'a str, String>,
    runtime: &'a str,
}

pub fn run(
    capture_dir: &Path,
    rom: &Path,
    replay_path: &Path,
    frame: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    require_exists(capture_dir)?;
    require_exists(rom)?;
    require_exists(replay_path)?;

    let frame = parse_positive_frame(frame)?;
    let port_frame_file = format!("port_frame_{frame:06}.ppm");
    require_file(
        &capture_dir.join(&port_frame_file),
        "missing dumped Rust runtime frame",
    )?;
    require_file(
        &capture_dir.join("port_input_trace.tsv"),
        "missing Rust runtime input trace",
    )?;
    for file in REQUIRED_TRACE_FILES {
        require_file(
            &capture_dir.join(file),
            &format!("missing Rust runtime trace file: {file}"),
        )?;
    }

    let rom_sha256 = file_hash(rom)?;
    let replay_bytes = fs::read(replay_path)?;
    let replay_sha256 = sha256::digest_hex(&replay_bytes);
    let replay_text = std::str::from_utf8(&replay_bytes)?;
    let replay = Replay::parse(replay_text)?;
    let expected_input = input_trace_tsv(Some(&replay), frame);
    fs::write(
        capture_dir.join("expected_input_trace.tsv"),
        expected_input.as_bytes(),
    )?;
    let port_input = fs::read(capture_dir.join("port_input_trace.tsv"))?;
    if expected_input.as_bytes() != port_input {
        return Err(
            "rust-port-capture: runtime input trace differs from replay parser output".into(),
        );
    }

    let port_frame_hash = file_hash(&capture_dir.join(&port_frame_file))?;
    write_hash_file(
        &capture_dir.join("port_frame_hashes.sha256"),
        &[(&port_frame_file, port_frame_hash.as_str())],
    )?;

    let mut trace_hashes = HashMap::new();
    let mut trace_hash_rows = Vec::new();
    for (file, _) in TRACE_FILES {
        let hash = file_hash(&capture_dir.join(file))?;
        trace_hash_rows.push((*file, hash.clone()));
        trace_hashes.insert(*file, hash);
    }
    let trace_hash_refs = trace_hash_rows
        .iter()
        .map(|(file, hash)| (*file, hash.as_str()))
        .collect::<Vec<_>>();
    write_hash_file(
        &capture_dir.join("port_trace_hashes.sha256"),
        &trace_hash_refs,
    )?;

    let trace_summary = read_key_values(&capture_dir.join("port_trace_summary.txt"))?;
    let runtime = required(
        &trace_summary,
        "runtime",
        &capture_dir.join("port_trace_summary.txt"),
    )?;
    require_value(
        &trace_summary,
        "complete",
        "1",
        &capture_dir.join("port_trace_summary.txt"),
    )?;

    write_summary(&CaptureSummary {
        capture_dir,
        rom,
        rom_sha256: &rom_sha256,
        replay: replay_path,
        replay_sha256: &replay_sha256,
        frame,
        port_frame_file: &port_frame_file,
        port_frame_hash: &port_frame_hash,
        trace_hashes: &trace_hashes,
        runtime,
    })?;

    println!(
        "rust-port-capture-report: wrote {}",
        capture_dir.join("port_summary.txt").display()
    );
    Ok(())
}

fn write_summary(summary: &CaptureSummary<'_>) -> io::Result<()> {
    let mut file = fs::File::create(summary.capture_dir.join("port_summary.txt"))?;
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
        .ok_or_else(|| io::Error::other(format!("rust-port-capture: missing hash for {file}")))
}

fn require_exists(path: &Path) -> io::Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("rust-port-capture: path not found: {}", path.display()),
        ))
    }
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("rust-port-capture: {message}: {}", path.display()),
        ))
    }
}

fn parse_positive_frame(value: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let frame = value
        .parse::<usize>()
        .map_err(|err| format!("rust-port-capture: frame must be a positive integer: {err}"))?;
    if frame == 0 {
        return Err("rust-port-capture: frame must be a positive integer".into());
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
) -> io::Result<&'a str> {
    values.get(key).map(String::as_str).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("rust-port-capture: missing {key} in {}", path.display()),
        )
    })
}

fn require_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> io::Result<()> {
    let actual = required(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "rust-port-capture: expected {key}={expected} in {}, got {actual}",
                path.display()
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_rust_port_capture_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_rust_port_capture_summary() {
        let root = temp_dir();
        let capture = root.join("capture");
        let rom = root.join("fixture.nes");
        let replay = root.join("fixture.replay");
        fs::create_dir_all(&capture).unwrap();
        fs::write(&rom, b"rom bytes").unwrap();
        fs::write(&replay, "frame 2\nframe 2 start\n").unwrap();
        fs::write(
            capture.join("port_frame_000004.ppm"),
            b"P6\n1 1\n255\n\0\0\0",
        )
        .unwrap();
        fs::write(
            capture.join("port_input_trace.tsv"),
            input_trace_tsv(Some(&Replay::parse("frame 2\nframe 2 start\n").unwrap()), 4),
        )
        .unwrap();
        write_empty_trace_files(&capture, 4).unwrap();

        run(&capture, &rom, &replay, "4").unwrap();

        let summary = fs::read_to_string(capture.join("port_summary.txt")).unwrap();
        assert!(summary.contains("port_frame=port_frame_000004.ppm\n"));
        assert!(summary.contains("input_trace_matches_replay=1\n"));
        assert!(summary.contains("runtime=rust_native_port_headless\n"));
        assert!(summary.contains("complete=1\n"));
        assert!(capture.join("expected_input_trace.tsv").is_file());
        assert!(capture.join("port_trace_hashes.sha256").is_file());

        fs::remove_dir_all(root).unwrap();
    }

    fn write_empty_trace_files(dir: &Path, frames: usize) -> io::Result<()> {
        fs::write(
            dir.join("port_mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n",
        )?;
        fs::write(
            dir.join("port_apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n",
        )?;
        fs::write(
            dir.join("port_oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n",
        )?;
        fs::write(
            dir.join("port_ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n",
        )?;
        fs::write(
            dir.join("port_ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n",
        )?;
        fs::write(
            dir.join("port_label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n",
        )?;
        fs::write(
            dir.join("port_trace_summary.txt"),
            format!(
                "runtime=rust_native_port_headless\nframes={frames}\nmapper_write_count=0\napu_write_count=0\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=0\ncomplete=1\n"
            ),
        )
    }
}
