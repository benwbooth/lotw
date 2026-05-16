use lotw_port::ppu_trace::{self, RenderInfo};
use lotw_port::rom::InesRom;
use lotw_port::sha256;
use lotw_port::video;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(
    rom_path: &Path,
    trace_dir: &Path,
    c_frame_path: &Path,
    out_dir: &Path,
    frame: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let frame = parse_positive_frame(frame)?;
    require_file(rom_path, "missing ROM")?;
    require_dir(trace_dir, "missing trace directory")?;
    require_file(c_frame_path, "missing C PPU trace render frame")?;

    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    let rom_bytes = fs::read(rom_path)?;
    let rom = InesRom::parse(&rom_bytes)?;
    let rendered = ppu_trace::render_trace_frame(&rom, trace_dir, Some(frame))?;
    let rust_ppm = video::frame_ppm(&rendered.frame);
    let c_ppm = fs::read(c_frame_path)?;
    let rust_frame_file = format!("rust_ppu_frame_{frame:06}.ppm");
    fs::write(out_dir.join(&rust_frame_file), &rust_ppm)?;

    let c_hash = sha256::digest_hex(&c_ppm);
    let rust_hash = sha256::digest_hex(&rust_ppm);
    let frames_match = c_ppm == rust_ppm;

    write_compare_report(&CompareReport {
        out_dir,
        rom_path,
        rom_sha256: &sha256::digest_hex(&rom_bytes),
        trace_dir,
        c_frame_path,
        rust_frame_file: &rust_frame_file,
        c_hash: &c_hash,
        rust_hash: &rust_hash,
        frames_match,
        info: &rendered.info,
    })?;

    println!(
        "rust-ppu-render-compare: wrote {}",
        out_dir.join("rust_ppu_render_compare.txt").display()
    );

    if !frames_match {
        return Err(format!(
            "Rust PPU trace render drifted from C renderer for frame {frame}: {rust_hash} != {c_hash}"
        )
        .into());
    }
    Ok(())
}

pub fn run_summary(
    compare_root: &Path,
    out_dir: &Path,
    specs: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if specs.is_empty() {
        return Err("rust-ppu-render-compare-summary: at least one replay spec is required".into());
    }
    fs::create_dir_all(out_dir)?;

    let summary_path = out_dir.join("replay_rust_ppu_render_compare.tsv");
    let mut summary = fs::File::create(&summary_path)?;
    writeln!(
        summary,
        "replay\tframe\tc_ppu_render_hash\trust_ppu_render_hash\tmatch\trender_trace_frame\trenderer\treference_renderer"
    )?;

    for spec in specs {
        let (name, expected_frame) = parse_replay_spec(spec)?;
        let report_path = compare_root.join(name).join("rust_ppu_render_compare.txt");
        let values = read_key_values(&report_path)?;
        require_value(&values, "complete", "1", &report_path)?;
        require_value(&values, "match", "1", &report_path)?;

        let frame = required(&values, "frame", &report_path)?;
        if frame != expected_frame {
            return Err(format!(
                "{}: frame {frame} does not match requested frame {expected_frame}",
                report_path.display()
            )
            .into());
        }

        writeln!(
            summary,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            name,
            frame,
            required(&values, "c_ppu_render_hash", &report_path)?,
            required(&values, "rust_ppu_render_hash", &report_path)?,
            required(&values, "match", &report_path)?,
            required(&values, "render_trace_frame", &report_path)?,
            required(&values, "renderer", &report_path)?,
            required(&values, "reference_renderer", &report_path)?,
        )?;
    }

    println!(
        "rust-ppu-render-compare-summary: wrote {}",
        summary_path.display()
    );
    Ok(())
}

struct CompareReport<'a> {
    out_dir: &'a Path,
    rom_path: &'a Path,
    rom_sha256: &'a str,
    trace_dir: &'a Path,
    c_frame_path: &'a Path,
    rust_frame_file: &'a str,
    c_hash: &'a str,
    rust_hash: &'a str,
    frames_match: bool,
    info: &'a RenderInfo,
}

fn write_compare_report(report: &CompareReport<'_>) -> io::Result<()> {
    let mut file = fs::File::create(report.out_dir.join("rust_ppu_render_compare.txt"))?;
    writeln!(file, "rom={}", report.rom_path.display())?;
    writeln!(file, "rom_sha256={}", report.rom_sha256)?;
    writeln!(file, "trace_dir={}", report.trace_dir.display())?;
    writeln!(file, "frame={}", report.info.frame)?;
    writeln!(file, "c_ppu_render_frame={}", report.c_frame_path.display())?;
    writeln!(file, "rust_ppu_render_frame={}", report.rust_frame_file)?;
    writeln!(file, "c_ppu_render_hash={}", report.c_hash)?;
    writeln!(file, "rust_ppu_render_hash={}", report.rust_hash)?;
    writeln!(file, "match={}", usize::from(report.frames_match))?;
    writeln!(file, "renderer=rust_ppu_trace_render")?;
    writeln!(file, "reference_renderer=c_ppu_trace_render")?;
    writeln!(file, "render_trace_frame={}", report.info.frame)?;
    writeln!(file, "ppu_ctrl={:02X}", report.info.ppu_ctrl)?;
    writeln!(file, "ppu_mask={:02X}", report.info.ppu_mask)?;
    writeln!(file, "chr_mode={}", report.info.chr_mode)?;
    writeln!(
        file,
        "scroll_valid={}",
        usize::from(report.info.scroll_valid)
    )?;
    writeln!(file, "scroll_v={:04X}", report.info.scroll_v)?;
    writeln!(file, "scroll_x={}", report.info.scroll_x)?;
    writeln!(file, "scroll_y={}", report.info.scroll_y)?;
    writeln!(
        file,
        "applied_mapper_writes={}",
        report.info.applied_mapper_writes
    )?;
    writeln!(
        file,
        "applied_ppu_register_writes={}",
        report.info.applied_ppu_register_writes
    )?;
    writeln!(
        file,
        "applied_ppu_scroll_writes={}",
        report.info.applied_ppu_scroll_writes
    )?;
    writeln!(
        file,
        "applied_ppu_vram_writes={}",
        report.info.applied_ppu_vram_writes
    )?;
    writeln!(
        file,
        "applied_oam_dma_writes={}",
        report.info.applied_oam_dma_writes
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn parse_positive_frame(value: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let frame = value.parse::<usize>().map_err(|err| {
        format!("rust-ppu-render-compare: frame must be a positive integer: {err}")
    })?;
    if frame == 0 {
        return Err("rust-ppu-render-compare: frame must be a positive integer".into());
    }
    Ok(frame)
}

fn parse_replay_spec(spec: &str) -> Result<(&str, &str), Box<dyn std::error::Error>> {
    let (name, frame) = spec
        .split_once(':')
        .ok_or_else(|| format!("invalid replay spec {spec}: expected name:frame"))?;
    if name.is_empty() || frame.is_empty() {
        return Err(format!("invalid replay spec {spec}: expected name:frame").into());
    }
    parse_positive_frame(frame)?;
    Ok((name, frame))
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("rust-ppu-render-compare: {message}: {}", path.display()),
        ))
    }
}

fn require_dir(path: &Path, message: &str) -> io::Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("rust-ppu-render-compare: {message}: {}", path.display()),
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

fn required<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("{}: missing {key}", path.display()).into())
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
            "{}: expected {key}={expected}, got {actual}",
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
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_rust_ppu_render_compare_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn write_empty_trace(root: &Path) {
        fs::create_dir_all(root).unwrap();
        fs::write(root.join("trace_summary.txt"), "frames=1\n").unwrap();
        fs::write(
            root.join("mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n",
        )
        .unwrap();
        fs::write(
            root.join("ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n",
        )
        .unwrap();
        fs::write(
            root.join("ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n",
        )
        .unwrap();
        fs::write(
            root.join("oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n",
        )
        .unwrap();
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + 0x4000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 1;
        bytes[5] = 1;
        bytes[6] = 0x40;
        bytes
    }

    #[test]
    fn compares_rust_trace_renderer_to_existing_ppm() {
        let root = temp_dir();
        let trace_dir = root.join("trace");
        let out_dir = root.join("compare");
        let rom_path = root.join("game.nes");
        let c_frame_path = root.join("c.ppm");
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom_path, ines_fixture()).unwrap();
        write_empty_trace(&trace_dir);

        let rom = InesRom::parse(&fs::read(&rom_path).unwrap()).unwrap();
        let rendered = ppu_trace::render_trace_frame(&rom, &trace_dir, Some(1)).unwrap();
        fs::write(&c_frame_path, video::frame_ppm(&rendered.frame)).unwrap();

        run(&rom_path, &trace_dir, &c_frame_path, &out_dir, "1").unwrap();

        let report = fs::read_to_string(out_dir.join("rust_ppu_render_compare.txt")).unwrap();
        assert!(report.contains("match=1\n"));
        assert!(report.contains("renderer=rust_ppu_trace_render\n"));
        assert!(out_dir.join("rust_ppu_frame_000001.ppm").is_file());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn summarizes_matching_reports() {
        let root = temp_dir();
        let compare_root = root.join("reports");
        let out_dir = root.join("summary");
        let report_dir = compare_root.join("title");
        fs::create_dir_all(&report_dir).unwrap();
        fs::write(
            report_dir.join("rust_ppu_render_compare.txt"),
            concat!(
                "frame=7\n",
                "c_ppu_render_hash=aaaa\n",
                "rust_ppu_render_hash=aaaa\n",
                "match=1\n",
                "render_trace_frame=7\n",
                "renderer=rust_ppu_trace_render\n",
                "reference_renderer=c_ppu_trace_render\n",
                "complete=1\n",
            ),
        )
        .unwrap();

        run_summary(&compare_root, &out_dir, &["title:7".to_string()]).unwrap();

        let summary =
            fs::read_to_string(out_dir.join("replay_rust_ppu_render_compare.tsv")).unwrap();
        assert!(summary
            .contains("title\t7\taaaa\taaaa\t1\t7\trust_ppu_trace_render\tc_ppu_trace_render\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
