use lotw_port::sha256;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SUMMARY_HEADER: &str = "replay\tframe\tapplied_mapper_writes\tapplied_ppu_register_writes\tapplied_ppu_vram_writes\tapplied_oam_dma_writes\tppu_render_hash\truntime_hash\tmatch\tpixel_match\tmismatch_pixels\ttotal_pixels";

#[derive(Debug, Clone)]
struct ReplayFrame {
    replay: String,
    frame: u64,
}

#[derive(Debug, Clone)]
struct ReplayResult {
    replay: String,
    frame: u64,
    applied_mapper: u64,
    applied_ppu_register: u64,
    applied_ppu_vram: u64,
    applied_oam_dma: u64,
    render_hash: String,
    runtime_hash: String,
    hash_match: bool,
    pixel_match: bool,
    mismatch_pixels: u64,
    total_pixels: u64,
}

pub fn run(
    lotw: &Path,
    ppu_render: &Path,
    ppm_compare: &Path,
    live_dir: &Path,
    out_dir: &Path,
    rom: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_summary = source_summary_path(live_dir)?;
    for path in [lotw, ppu_render, ppm_compare, &source_summary, rom] {
        require_file(path)?;
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let replay_frames = read_replay_frames(&source_summary)?;
    let summary_path = out_dir.join("replay_runtime_native_live_frame_ppu.tsv");
    let mut summary = fs::File::create(&summary_path)?;
    writeln!(summary, "{SUMMARY_HEADER}")?;

    let mut results = Vec::new();
    for replay_frame in replay_frames {
        let result = process_replay(
            lotw,
            ppu_render,
            ppm_compare,
            live_dir,
            out_dir,
            rom,
            &replay_frame,
        )?;
        writeln!(
            summary,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            result.replay,
            result.frame,
            result.applied_mapper,
            result.applied_ppu_register,
            result.applied_ppu_vram,
            result.applied_oam_dma,
            result.render_hash,
            result.runtime_hash,
            u8::from(result.hash_match),
            u8::from(result.pixel_match),
            result.mismatch_pixels,
            result.total_pixels
        )?;
        if !result.hash_match || !result.pixel_match {
            return Err(format!(
                "runtime_native_live_frame_ppu: {} frame {} did not match",
                result.replay, result.frame
            )
            .into());
        }
        results.push(result);
    }

    write_manifest(out_dir, &source_summary, &results)?;
    println!(
        "runtime_native_live_frame_ppu: wrote {}",
        summary_path.display()
    );
    Ok(())
}

fn source_summary_path(live_dir: &Path) -> io::Result<PathBuf> {
    let maximal = live_dir.join("replay_runtime_native_live_frame_maximal.tsv");
    if maximal.is_file() {
        return Ok(maximal);
    }
    let normal = live_dir.join("replay_runtime_native_live_frame.tsv");
    if normal.is_file() {
        return Ok(normal);
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "runtime_native_live_frame_ppu: missing replay runtime summary in {}",
            live_dir.display()
        ),
    ))
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "runtime_native_live_frame_ppu: missing input: {}",
                path.display()
            ),
        ))
    }
}

fn require_non_empty_file(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() && metadata.len() > 0 {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "runtime_native_live_frame_ppu: empty output: {}",
                path.display()
            ),
        ))
    }
}

fn read_replay_frames(path: &Path) -> io::Result<Vec<ReplayFrame>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} has {} fields, expected at least 3",
                    path.display(),
                    line_no + 1,
                    fields.len()
                ),
            ));
        }
        if fields[0].is_empty() || fields[2].is_empty() {
            continue;
        }
        let frame = fields[2].parse::<u64>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid frame: {err}", path.display(), line_no + 1),
            )
        })?;
        rows.push(ReplayFrame {
            replay: fields[0].to_string(),
            frame,
        });
    }
    Ok(rows)
}

fn process_replay(
    lotw: &Path,
    ppu_render: &Path,
    ppm_compare: &Path,
    live_dir: &Path,
    out_dir: &Path,
    rom: &Path,
    replay_frame: &ReplayFrame,
) -> Result<ReplayResult, Box<dyn std::error::Error>> {
    let trace_dir = live_dir.join(&replay_frame.replay).join("trace");
    for path in [
        trace_dir.join("port_trace_summary.txt"),
        trace_dir.join("port_mapper_writes.tsv"),
        trace_dir.join("port_ppu_writes.tsv"),
        trace_dir.join("port_ppu_vram_writes.tsv"),
        trace_dir.join("port_oam_dma.tsv"),
    ] {
        if !path.is_file() {
            return Err(format!(
                "runtime_native_live_frame_ppu: missing runtime trace input: {}",
                path.display()
            )
            .into());
        }
    }

    let replay_out = out_dir.join(&replay_frame.replay);
    let render_dir = replay_out.join("render");
    let runtime_dir = replay_out.join("runtime");
    let compare_dir = replay_out.join("compare");
    fs::create_dir_all(&render_dir)?;
    fs::create_dir_all(&runtime_dir)?;
    fs::create_dir_all(&compare_dir)?;

    run_status(
        Command::new(ppu_render)
            .arg(rom)
            .arg(&trace_dir)
            .arg(&render_dir)
            .arg(replay_frame.frame.to_string()),
        "runtime_native_live_frame_ppu: ppu render failed",
    )?;

    let render_frame = render_dir.join(format!("ppu_frame_{:06}.ppm", replay_frame.frame));
    let runtime_frame =
        runtime_dir.join(format!("runtime_ppu_frame_{:06}.ppm", replay_frame.frame));
    let pixel_report = compare_dir.join("runtime_native_live_frame_ppu_pixels.txt");
    let pixel_diff = compare_dir.join("runtime_native_live_frame_ppu_diff.ppm");
    let log = replay_out.join("runtime_native_live_frame_ppu.log");

    run_lotw(
        lotw,
        &trace_dir,
        replay_frame.frame,
        &runtime_frame,
        rom,
        &log,
    )?;

    require_non_empty_file(&render_frame)?;
    require_non_empty_file(&runtime_frame)?;
    run_status(
        Command::new(ppm_compare)
            .arg(&render_frame)
            .arg(&runtime_frame)
            .arg(&pixel_report)
            .arg(&pixel_diff),
        "runtime_native_live_frame_ppu: ppm compare failed",
    )?;

    let render_hash = sha256::digest_hex(&fs::read(&render_frame)?);
    let runtime_hash = sha256::digest_hex(&fs::read(&runtime_frame)?);
    let pixel_values = read_key_values(&pixel_report)?;
    let ppu_values = read_key_values(&render_dir.join("ppu_state_summary.txt"))?;
    let pixel_match = require_u64(&pixel_values, "match", &pixel_report)? == 1;
    let mismatch_pixels = require_u64(&pixel_values, "mismatch_pixels", &pixel_report)?;
    let total_pixels = require_u64(&pixel_values, "total_pixels", &pixel_report)?;
    let applied_mapper = require_u64(
        &ppu_values,
        "applied_mapper_writes",
        &render_dir.join("ppu_state_summary.txt"),
    )?;
    let applied_ppu_register = require_u64(
        &ppu_values,
        "applied_ppu_register_writes",
        &render_dir.join("ppu_state_summary.txt"),
    )?;
    let applied_ppu_vram = require_u64(
        &ppu_values,
        "applied_ppu_vram_writes",
        &render_dir.join("ppu_state_summary.txt"),
    )?;
    let applied_oam_dma = require_u64(
        &ppu_values,
        "applied_oam_dma_writes",
        &render_dir.join("ppu_state_summary.txt"),
    )?;
    let hash_match = render_hash == runtime_hash;

    let result = ReplayResult {
        replay: replay_frame.replay.clone(),
        frame: replay_frame.frame,
        applied_mapper,
        applied_ppu_register,
        applied_ppu_vram,
        applied_oam_dma,
        render_hash,
        runtime_hash,
        hash_match,
        pixel_match,
        mismatch_pixels,
        total_pixels,
    };
    write_replay_report(
        &replay_out.join("runtime_native_live_frame_ppu_compare.txt"),
        &trace_dir,
        &render_frame,
        &runtime_frame,
        &result,
    )?;
    Ok(result)
}

fn run_status(command: &mut Command, message: &str) -> io::Result<()> {
    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("{message}: {status}")))
    }
}

fn run_lotw(
    lotw: &Path,
    trace_dir: &Path,
    frame: u64,
    runtime_frame: &Path,
    rom: &Path,
    log: &Path,
) -> io::Result<()> {
    let log_file = fs::File::create(log)?;
    let log_stderr = log_file.try_clone()?;
    let status = Command::new(lotw)
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .arg("--frames")
        .arg("1")
        .arg("--render-trace-dir")
        .arg(trace_dir)
        .arg("--render-trace-frame")
        .arg(frame.to_string())
        .arg("--dump-frame")
        .arg(runtime_frame)
        .arg(rom)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_stderr))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "runtime_native_live_frame_ppu: lotw failed: {status}"
        )))
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

fn require_u64(values: &HashMap<String, String>, key: &str, path: &Path) -> io::Result<u64> {
    values
        .get(key)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} missing key: {key}", path.display()),
            )
        })?
        .parse::<u64>()
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} invalid {key}: {err}", path.display()),
            )
        })
}

fn write_replay_report(
    path: &Path,
    trace_dir: &Path,
    render_frame: &Path,
    runtime_frame: &Path,
    result: &ReplayResult,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "replay={}", result.replay)?;
    writeln!(file, "frame={}", result.frame)?;
    writeln!(file, "trace_dir={}", trace_dir.display())?;
    writeln!(file, "ppu_render_frame={}", render_frame.display())?;
    writeln!(file, "runtime_frame={}", runtime_frame.display())?;
    writeln!(file, "ppu_render_hash={}", result.render_hash)?;
    writeln!(file, "runtime_hash={}", result.runtime_hash)?;
    writeln!(file, "match={}", u8::from(result.hash_match))?;
    writeln!(
        file,
        "pixel_compare=compare/runtime_native_live_frame_ppu_pixels.txt"
    )?;
    writeln!(
        file,
        "pixel_diff=compare/runtime_native_live_frame_ppu_diff.ppm"
    )?;
    writeln!(file, "pixel_match={}", u8::from(result.pixel_match))?;
    writeln!(file, "mismatch_pixels={}", result.mismatch_pixels)?;
    writeln!(file, "total_pixels={}", result.total_pixels)?;
    writeln!(file, "applied_mapper_writes={}", result.applied_mapper)?;
    writeln!(
        file,
        "applied_ppu_register_writes={}",
        result.applied_ppu_register
    )?;
    writeln!(file, "applied_ppu_vram_writes={}", result.applied_ppu_vram)?;
    writeln!(file, "applied_oam_dma_writes={}", result.applied_oam_dma)?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(
    out_dir: &Path,
    source_summary: &Path,
    results: &[ReplayResult],
) -> io::Result<()> {
    let replay_count = results.len() as u64;
    let mapper_count = results.iter().map(|row| row.applied_mapper).sum::<u64>();
    let ppu_count = results
        .iter()
        .map(|row| row.applied_ppu_register)
        .sum::<u64>();
    let ppu_vram_count = results.iter().map(|row| row.applied_ppu_vram).sum::<u64>();
    let oam_dma_count = results.iter().map(|row| row.applied_oam_dma).sum::<u64>();
    let bad_rows = results
        .iter()
        .filter(|row| !row.hash_match || !row.pixel_match)
        .count();
    if replay_count == 0 || bad_rows != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "runtime_native_live_frame_ppu: summary verification failed",
        ));
    }

    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=runtime_native_live_frame_ppu")?;
    writeln!(file, "source_summary={}", source_summary.display())?;
    writeln!(file, "summary=replay_runtime_native_live_frame_ppu.tsv")?;
    writeln!(file, "replay_count={replay_count}")?;
    writeln!(file, "applied_mapper_writes={mapper_count}")?;
    writeln!(file, "applied_ppu_register_writes={ppu_count}")?;
    writeln!(file, "applied_ppu_vram_writes={ppu_vram_count}")?;
    writeln!(file, "applied_oam_dma_writes={oam_dma_count}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, unique_temp_dir};

    #[test]
    fn runs_ppu_frame_compare_stage() {
        let root = unique_temp_dir("runtime-native-live-frame-ppu");
        let tools = root.join("tools");
        let live = root.join("live");
        let out = root.join("out");
        fs::create_dir_all(&tools).unwrap();
        fs::create_dir_all(live.join("demo/trace")).unwrap();

        let rom = root.join("game.nes");
        fs::write(&rom, "rom").unwrap();
        write_ppu_render(&tools.join("ppu_render"));
        write_lotw(&tools.join("lotw"));
        write_ppm_compare(&tools.join("ppm_compare"));

        fs::write(
            live.join("replay_runtime_native_live_frame.tsv"),
            "replay\tcase_count\tscheduled_frame_count\tactive_frame_count\tidle_frame_count\tmatched\tcontinued\taudio_apu_write_count\tinput_trace_matches_replay\n\
demo\t1\t7\t1\t6\t1\t0\t0\t1\n",
        )
        .unwrap();
        for name in [
            "port_trace_summary.txt",
            "port_mapper_writes.tsv",
            "port_ppu_writes.tsv",
            "port_ppu_vram_writes.tsv",
            "port_oam_dma.tsv",
        ] {
            fs::write(live.join("demo/trace").join(name), "header\n").unwrap();
        }

        run(
            &tools.join("lotw"),
            &tools.join("ppu_render"),
            &tools.join("ppm_compare"),
            &live,
            &out,
            &rom,
        )
        .unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("runtime=runtime_native_live_frame_ppu\n"));
        assert!(manifest.contains("replay_count=1\n"));
        assert!(manifest.contains("applied_mapper_writes=2\n"));
        assert!(manifest.contains("applied_ppu_register_writes=3\n"));
        assert!(manifest.contains("applied_ppu_vram_writes=4\n"));
        assert!(manifest.contains("applied_oam_dma_writes=5\n"));
        assert!(manifest.contains("complete=1\n"));
        let summary =
            fs::read_to_string(out.join("replay_runtime_native_live_frame_ppu.tsv")).unwrap();
        assert!(summary.contains("\ndemo\t7\t2\t3\t4\t5\t"));
        assert!(summary.contains("\t1\t1\t0\t1\n"));
        let report =
            fs::read_to_string(out.join("demo/runtime_native_live_frame_ppu_compare.txt")).unwrap();
        assert!(report.contains("match=1\n"));
        assert!(report.contains("pixel_match=1\n"));
    }

    fn write_ppu_render(path: &Path) {
        compile(
            path,
            r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let out = PathBuf::from(&args[3]);
    let frame = args[4].parse::<u64>().unwrap();
    fs::create_dir_all(&out).unwrap();
    fs::write(
        out.join(format!("ppu_frame_{frame:06}.ppm")),
        "P3\n1 1\n255\n0 0 0\n",
    )
    .unwrap();
    fs::write(
        out.join("ppu_state_summary.txt"),
        "applied_mapper_writes=2\napplied_ppu_register_writes=3\napplied_ppu_vram_writes=4\napplied_oam_dma_writes=5\n",
    )
    .unwrap();
}
"#,
        );
    }

    fn write_lotw(path: &Path) {
        compile(
            path,
            r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut dump = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dump-frame" => {
                dump = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }
    let dump = dump.expect("missing dump path");
    fs::create_dir_all(dump.parent().unwrap()).unwrap();
    fs::write(&dump, "P3\n1 1\n255\n0 0 0\n").unwrap();
    println!("runtime ok");
}
"#,
        );
    }

    fn write_ppm_compare(path: &Path) {
        compile(
            path,
            r#"
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    fs::write(&args[3], "match=1\nmismatch_pixels=0\ntotal_pixels=1\n").unwrap();
    fs::copy(&args[1], &args[4]).unwrap();
}
"#,
        );
    }
}
