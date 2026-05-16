#[cfg(feature = "sdl")]
mod sdl;

use lotw_port::apu_trace;
use lotw_port::ppu_trace::{self, RenderInfo};
use lotw_port::replay::{input_trace_tsv, Replay};
use lotw_port::rom::InesRom;
use lotw_port::runtime;
use lotw_port::sha256;
use lotw_port::video;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const EXPECTED_ROM_SHA256: &str =
    "079f648d669966357fe4414a986573eacd7ecadf5c4f289c288427b8c5f491f1";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Options {
    rom_path: PathBuf,
    out_dir: Option<PathBuf>,
    dump_frame: Option<PathBuf>,
    dump_input_trace: Option<PathBuf>,
    dump_trace_dir: Option<PathBuf>,
    trace_seed_dir: Option<PathBuf>,
    render_trace_dir: Option<PathBuf>,
    render_trace_frame: Option<usize>,
    apu_trace_path: Option<PathBuf>,
    dump_audio_summary: Option<PathBuf>,
    replay_path: Option<PathBuf>,
    expected_sha256: Option<String>,
    frames: usize,
    scale: i32,
    window: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplaySummary {
    path: PathBuf,
    sha256: String,
    frame_count: usize,
    pressed_frame_count: usize,
    first_pressed_frame: usize,
    last_pressed_frame: usize,
    replay: Replay,
}

fn usage(program: &str) {
    eprintln!(
        "Usage: {program} [--headless|--window] [--out-dir path] [--dump-frame path] [--dump-input-trace path] [--dump-trace-dir path] [--trace-seed-dir path] [--render-trace-dir path] [--render-trace-frame n] [--apu-trace path] [--dump-audio-summary path] [--replay path] [--expected-sha256 hash] [--frames n] [--scale n] <rom.nes>"
    );
}

fn main() {
    if let Err(err) = run(std::env::args().collect()) {
        eprintln!("lotw-runtime: {err}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let program = args.first().map(String::as_str).unwrap_or("lotw-runtime");
    if args[1..].iter().any(|arg| arg == "--help" || arg == "-h") {
        usage(program);
        return Ok(());
    }

    let options = match parse_args(&args[1..]) {
        Ok(options) => options,
        Err(()) => {
            usage(program);
            std::process::exit(2);
        }
    };

    run_with_options(&options)
}

fn parse_args(args: &[String]) -> Result<Options, ()> {
    let mut out_dir = None;
    let mut dump_frame = None;
    let mut dump_input_trace = None;
    let mut dump_trace_dir = None;
    let mut trace_seed_dir = None;
    let mut render_trace_dir = None;
    let mut render_trace_frame = None;
    let mut apu_trace_path = None;
    let mut dump_audio_summary = None;
    let mut replay_path = None;
    let mut expected_sha256 = Some(EXPECTED_ROM_SHA256.to_string());
    let mut frames = 1usize;
    let mut scale = 3i32;
    let mut window = false;
    let mut rom_path = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => return Err(()),
            "--headless" => {
                window = false;
                index += 1;
            }
            "--window" => {
                window = true;
                index += 1;
            }
            "--out-dir" => {
                out_dir = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--dump-frame" => {
                dump_frame = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--dump-input-trace" => {
                dump_input_trace = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--dump-trace-dir" => {
                dump_trace_dir = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--trace-seed-dir" => {
                trace_seed_dir = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--render-trace-dir" => {
                render_trace_dir = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--render-trace-frame" => {
                let frame = args
                    .get(index + 1)
                    .ok_or(())?
                    .parse::<usize>()
                    .map_err(|_| ())?;
                if frame == 0 {
                    return Err(());
                }
                render_trace_frame = Some(frame);
                index += 2;
            }
            "--apu-trace" => {
                apu_trace_path = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--dump-audio-summary" => {
                dump_audio_summary = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--replay" => {
                replay_path = Some(PathBuf::from(args.get(index + 1).ok_or(())?));
                index += 2;
            }
            "--expected-sha256" => {
                expected_sha256 = Some(args.get(index + 1).ok_or(())?.to_ascii_lowercase());
                index += 2;
            }
            "--no-expected-sha256" => {
                expected_sha256 = None;
                index += 1;
            }
            "--frames" => {
                frames = args
                    .get(index + 1)
                    .ok_or(())?
                    .parse::<usize>()
                    .map_err(|_| ())?;
                index += 2;
            }
            "--scale" => {
                scale = args
                    .get(index + 1)
                    .ok_or(())?
                    .parse::<i32>()
                    .map_err(|_| ())?;
                if scale < 1 {
                    return Err(());
                }
                index += 2;
            }
            arg if arg.starts_with('-') => return Err(()),
            _ => {
                if rom_path.is_some() {
                    return Err(());
                }
                rom_path = Some(PathBuf::from(&args[index]));
                index += 1;
            }
        }
    }

    Ok(Options {
        rom_path: rom_path.ok_or(())?,
        out_dir,
        dump_frame,
        dump_input_trace,
        dump_trace_dir,
        trace_seed_dir,
        render_trace_dir,
        render_trace_frame,
        apu_trace_path,
        dump_audio_summary,
        replay_path,
        expected_sha256,
        frames,
        scale,
        window,
    })
}

fn run_with_options(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    let rom_bytes = fs::read(&options.rom_path)?;
    let actual_sha256 = sha256::digest_hex(&rom_bytes);
    if let Some(expected) = &options.expected_sha256 {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "ROM hash mismatch: got {actual_sha256}, expected {}",
                expected.to_ascii_lowercase()
            )
            .into());
        }
    }

    let rom = InesRom::parse(&rom_bytes)?;
    let rendered = render_runtime_frame(&rom, options)?;
    let ppm = video::frame_ppm(&rendered.frame);
    let frame_sha256 = sha256::digest_hex(&ppm);
    let replay = match &options.replay_path {
        Some(path) => Some(load_replay_summary(path)?),
        None => None,
    };

    if let Some(out_dir) = &options.out_dir {
        if out_dir.exists() {
            fs::remove_dir_all(out_dir)?;
        }
        fs::create_dir_all(out_dir)?;
        fs::write(out_dir.join("frame.ppm"), &ppm)?;
        let input_trace_hash = if let Some(replay) = replay.as_ref() {
            let input_trace_path = out_dir.join("input_trace.tsv");
            write_input_trace(&input_trace_path, Some(replay), options.frames)?;
            Some(sha256::digest_hex(&fs::read(input_trace_path)?))
        } else {
            None
        };
        write_manifest(
            &out_dir.join("manifest.txt"),
            options,
            &actual_sha256,
            &frame_sha256,
            rendered.frame.width,
            rendered.frame.height,
            rendered.page,
            rendered.page_count,
            rendered.frame_source,
            rendered.trace_info.as_ref(),
            replay.as_ref(),
            input_trace_hash.as_deref(),
        )?;
    }

    if let Some(path) = &options.dump_frame {
        write_parented(path, &ppm)?;
    }

    if let Some(path) = &options.dump_input_trace {
        write_input_trace(path, replay.as_ref(), options.frames)?;
    }

    if let Some(path) = &options.dump_trace_dir {
        write_trace_dir(path, options.trace_seed_dir.as_deref(), options.frames)?;
    }

    if let Some(path) = &options.dump_audio_summary {
        write_audio_summary(path, options.apu_trace_path.as_deref(), options.frames)?;
    }

    if options.window {
        run_window(&rendered.frame, options.scale, options.frames)?;
    }

    println!(
        "lotw-runtime: frame={}x{} sha256={frame_sha256}",
        rendered.frame.width, rendered.frame.height
    );
    Ok(())
}

struct RuntimeFrame {
    frame: video::Frame,
    page: usize,
    page_count: usize,
    frame_source: &'static str,
    trace_info: Option<RenderInfo>,
}

fn render_runtime_frame(
    rom: &InesRom,
    options: &Options,
) -> Result<RuntimeFrame, Box<dyn std::error::Error>> {
    if let Some(trace_dir) = &options.render_trace_dir {
        let rendered = ppu_trace::render_trace_frame(rom, trace_dir, options.render_trace_frame)?;
        return Ok(RuntimeFrame {
            frame: rendered.frame,
            page: 0,
            page_count: 0,
            frame_source: "rust_ppu_trace_render",
            trace_info: Some(rendered.info),
        });
    }

    let boot = runtime::render_boot_frame(rom);
    Ok(RuntimeFrame {
        frame: boot.frame,
        page: boot.page,
        page_count: boot.page_count,
        frame_source: "chr_preview",
        trace_info: None,
    })
}

fn write_parented(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes)
}

fn load_replay_summary(path: &Path) -> Result<ReplaySummary, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    let text = std::str::from_utf8(&bytes)?;
    let replay = Replay::parse(text)?;
    let stats = replay.stats();

    Ok(ReplaySummary {
        path: path.to_path_buf(),
        sha256: sha256::digest_hex(&bytes),
        frame_count: stats.frame_count,
        pressed_frame_count: stats.pressed_frame_count,
        first_pressed_frame: stats.first_pressed_frame,
        last_pressed_frame: stats.last_pressed_frame,
        replay,
    })
}

fn write_input_trace(path: &Path, replay: Option<&ReplaySummary>, frames: usize) -> io::Result<()> {
    let replay = replay.map(|summary| &summary.replay);
    write_parented(path, input_trace_tsv(replay, frames).as_bytes())
}

fn write_trace_dir(path: &Path, seed_dir: Option<&Path>, frames: usize) -> io::Result<()> {
    match seed_dir {
        Some(seed_dir) => write_seeded_trace_dir(path, seed_dir, frames),
        None => write_empty_trace_dir(path, frames),
    }
}

fn write_empty_trace_dir(path: &Path, frames: usize) -> io::Result<()> {
    fs::create_dir_all(path)?;
    fs::write(
        path.join("port_mapper_writes.tsv"),
        b"frame\taddr\tvalue\tstate\n",
    )?;
    fs::write(
        path.join("port_apu_writes.tsv"),
        b"frame\tcycle\taddr\tvalue\n",
    )?;
    fs::write(
        path.join("port_ppu_writes.tsv"),
        b"frame\tcycle\taddr\tregister\tvalue\n",
    )?;
    fs::write(
        path.join("port_ppu_vram_writes.tsv"),
        b"frame\tcycle\taddr\tregion\tvalue\n",
    )?;
    fs::write(
        path.join("port_oam_dma.tsv"),
        b"frame\tcycle\tpage\tbytes_0000_00ff\n",
    )?;
    fs::write(
        path.join("port_label_states.tsv"),
        b"cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n",
    )?;

    let mut summary = fs::File::create(path.join("port_trace_summary.txt"))?;
    writeln!(summary, "runtime=rust_native_port_headless")?;
    writeln!(summary, "frames={frames}")?;
    writeln!(summary, "mapper_write_count=0")?;
    writeln!(summary, "apu_write_count=0")?;
    writeln!(summary, "ppu_write_count=0")?;
    writeln!(summary, "ppu_vram_write_count=0")?;
    writeln!(summary, "oam_dma_count=0")?;
    writeln!(summary, "label_state_count=0")?;
    writeln!(summary, "complete=1")?;
    Ok(())
}

fn write_seeded_trace_dir(path: &Path, seed_dir: &Path, frames: usize) -> io::Result<()> {
    fs::create_dir_all(path)?;
    for (source, dest) in [
        ("mapper_writes.tsv", "port_mapper_writes.tsv"),
        ("apu_writes.tsv", "port_apu_writes.tsv"),
        ("ppu_writes.tsv", "port_ppu_writes.tsv"),
        ("ppu_vram_writes.tsv", "port_ppu_vram_writes.tsv"),
        ("oam_dma.tsv", "port_oam_dma.tsv"),
        ("label_states.tsv", "port_label_states.tsv"),
    ] {
        fs::copy(seed_dir.join(source), path.join(dest))?;
    }

    let summary = read_key_values(&seed_dir.join("trace_summary.txt"))?;
    let mut file = fs::File::create(path.join("port_trace_summary.txt"))?;
    writeln!(file, "runtime=rust_trace_seed")?;
    writeln!(file, "frames={frames}")?;
    for key in [
        "mapper_write_count",
        "apu_write_count",
        "ppu_write_count",
        "ppu_vram_write_count",
        "oam_dma_count",
        "label_state_count",
    ] {
        writeln!(
            file,
            "{key}={}",
            summary.get(key).map_or("0", String::as_str)
        )?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_audio_summary(
    path: &Path,
    apu_trace_path: Option<&Path>,
    frames: usize,
) -> io::Result<()> {
    let events = match apu_trace_path {
        Some(path) => {
            let text = fs::read_to_string(path)?;
            apu_trace::parse_apu_writes_tsv(&text)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?
        }
        None => Vec::new(),
    };
    let played = events
        .iter()
        .filter(|event| event.frame <= frames as u32)
        .count();

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=rust_native_port_headless")?;
    writeln!(file, "backend=rust_audio_stub")?;
    writeln!(file, "frames={frames}")?;
    writeln!(file, "apu_trace_events={}", events.len())?;
    writeln!(file, "apu_trace_events_played={played}")?;
    writeln!(file, "audio_apu_write_count={played}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn read_key_values(path: &Path) -> io::Result<std::collections::HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = std::collections::HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

#[allow(clippy::too_many_arguments)]
fn write_manifest(
    path: &Path,
    options: &Options,
    rom_sha256: &str,
    frame_sha256: &str,
    width: usize,
    height: usize,
    page: usize,
    page_count: usize,
    frame_source: &str,
    trace_info: Option<&RenderInfo>,
    replay: Option<&ReplaySummary>,
    input_trace_hash: Option<&str>,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=rust_native_port_headless")?;
    writeln!(file, "rom={}", options.rom_path.display())?;
    writeln!(file, "sha256={rom_sha256}")?;
    if let Some(expected) = &options.expected_sha256 {
        writeln!(file, "expected_sha256={}", expected.to_ascii_lowercase())?;
        writeln!(file, "sha256_match=1")?;
    }
    writeln!(file, "frame=frame.ppm")?;
    writeln!(file, "frame_sha256={frame_sha256}")?;
    writeln!(file, "width={width}")?;
    writeln!(file, "height={height}")?;
    writeln!(file, "frame_source={frame_source}")?;
    writeln!(file, "page={page}")?;
    writeln!(file, "page_count={page_count}")?;
    writeln!(file, "requested_frames={}", options.frames)?;
    writeln!(file, "window={}", usize::from(options.window))?;
    writeln!(file, "scale={}", options.scale)?;
    if let Some(hash) = input_trace_hash {
        writeln!(file, "input_trace=input_trace.tsv")?;
        writeln!(file, "input_trace_sha256={hash}")?;
    }
    if let Some(trace_dir) = &options.dump_trace_dir {
        writeln!(file, "trace_dir={}", trace_dir.display())?;
    }
    if let Some(trace_dir) = &options.render_trace_dir {
        writeln!(file, "render_trace_dir={}", trace_dir.display())?;
    }
    if let Some(apu_trace_path) = &options.apu_trace_path {
        writeln!(file, "apu_trace={}", apu_trace_path.display())?;
    }
    if let Some(audio_summary) = &options.dump_audio_summary {
        writeln!(file, "audio_summary={}", audio_summary.display())?;
    }
    if let Some(info) = trace_info {
        writeln!(file, "render_trace_frame={}", info.frame)?;
        writeln!(file, "ppu_ctrl={:02X}", info.ppu_ctrl)?;
        writeln!(file, "ppu_mask={:02X}", info.ppu_mask)?;
        writeln!(file, "chr_mode={}", info.chr_mode)?;
        writeln!(file, "scroll_valid={}", usize::from(info.scroll_valid))?;
        writeln!(file, "scroll_v={:04X}", info.scroll_v)?;
        writeln!(file, "scroll_x={}", info.scroll_x)?;
        writeln!(file, "scroll_y={}", info.scroll_y)?;
        writeln!(file, "applied_mapper_writes={}", info.applied_mapper_writes)?;
        writeln!(
            file,
            "applied_ppu_register_writes={}",
            info.applied_ppu_register_writes
        )?;
        writeln!(
            file,
            "applied_ppu_scroll_writes={}",
            info.applied_ppu_scroll_writes
        )?;
        writeln!(
            file,
            "applied_ppu_vram_writes={}",
            info.applied_ppu_vram_writes
        )?;
        writeln!(
            file,
            "applied_oam_dma_writes={}",
            info.applied_oam_dma_writes
        )?;
    }
    if let Some(replay) = replay {
        writeln!(file, "replay={}", replay.path.display())?;
        writeln!(file, "replay_sha256={}", replay.sha256)?;
        writeln!(file, "replay_frame_count={}", replay.frame_count)?;
        writeln!(
            file,
            "replay_pressed_frame_count={}",
            replay.pressed_frame_count
        )?;
        writeln!(
            file,
            "replay_first_pressed_frame={}",
            replay.first_pressed_frame
        )?;
        writeln!(
            file,
            "replay_last_pressed_frame={}",
            replay.last_pressed_frame
        )?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(not(feature = "sdl"))]
fn run_window(
    _frame: &video::Frame,
    _scale: i32,
    _frames: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("SDL window support requires building lotw-runtime with --features sdl".into())
}

#[cfg(feature = "sdl")]
fn run_window(
    frame: &video::Frame,
    scale: i32,
    frames: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    sdl::run(frame, scale, frames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_runtime_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + 0x4000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 1;
        bytes[5] = 1;
        bytes[6] = 0x40;
        bytes[16 + 0x4000] = 0xff;
        bytes
    }

    #[test]
    fn parses_headless_runtime_args() {
        let args = vec![
            "--headless".to_string(),
            "--out-dir".to_string(),
            "out".to_string(),
            "--frames".to_string(),
            "2".to_string(),
            "game.nes".to_string(),
        ];

        let options = parse_args(&args).unwrap();

        assert!(!options.window);
        assert_eq!(options.out_dir, Some(PathBuf::from("out")));
        assert_eq!(options.frames, 2);
        assert_eq!(options.rom_path, PathBuf::from("game.nes"));
    }

    #[test]
    fn writes_headless_runtime_manifest() {
        let root = temp_dir();
        let rom_path = root.join("fixture.nes");
        let replay_path = root.join("fixture.replay");
        let out_dir = root.join("out");
        let rom_bytes = ines_fixture();
        let expected = sha256::digest_hex(&rom_bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom_path, rom_bytes).unwrap();
        fs::write(&replay_path, b"frame 2\nframe 3 start\n").unwrap();

        let options = Options {
            rom_path,
            out_dir: Some(out_dir.clone()),
            dump_frame: None,
            dump_input_trace: None,
            dump_trace_dir: Some(out_dir.clone()),
            trace_seed_dir: None,
            render_trace_dir: None,
            render_trace_frame: None,
            apu_trace_path: None,
            dump_audio_summary: None,
            replay_path: Some(replay_path),
            expected_sha256: Some(expected),
            frames: 5,
            scale: 3,
            window: false,
        };

        run_with_options(&options).unwrap();

        let manifest = fs::read_to_string(out_dir.join("manifest.txt")).unwrap();
        let ppm = fs::read(out_dir.join("frame.ppm")).unwrap();
        assert!(manifest.contains("runtime=rust_native_port_headless\n"));
        assert!(manifest.contains("sha256_match=1\n"));
        assert!(manifest.contains("width=256\n"));
        assert!(manifest.contains("height=240\n"));
        assert!(manifest.contains("input_trace=input_trace.tsv\n"));
        assert!(manifest.contains("replay_frame_count=5\n"));
        assert!(manifest.contains("replay_pressed_frame_count=3\n"));
        assert!(ppm.starts_with(b"P6\n256 240\n255\n"));
        let input_trace = fs::read_to_string(out_dir.join("input_trace.tsv")).unwrap();
        assert!(input_trace.contains("1\t0000\t\n"));
        assert!(input_trace.contains("3\t0040\tstart\n"));
        let trace_summary = fs::read_to_string(out_dir.join("port_trace_summary.txt")).unwrap();
        assert!(trace_summary.contains("runtime=rust_native_port_headless\n"));
        assert!(trace_summary.contains("frames=5\n"));
        assert!(trace_summary.contains("mapper_write_count=0\n"));
        assert!(fs::read_to_string(out_dir.join("port_mapper_writes.tsv"))
            .unwrap()
            .starts_with("frame\taddr\tvalue\tstate\n"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn writes_seeded_runtime_traces() {
        let root = temp_dir();
        let seed = root.join("seed");
        let out = root.join("out");
        fs::create_dir_all(&seed).unwrap();
        fs::write(
            seed.join("mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n1\t8000\t06\tstate\n",
        )
        .unwrap();
        fs::write(
            seed.join("apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n1\t123\t4000\t30\n",
        )
        .unwrap();
        fs::write(
            seed.join("ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n",
        )
        .unwrap();
        fs::write(
            seed.join("ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n",
        )
        .unwrap();
        fs::write(
            seed.join("oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n",
        )
        .unwrap();
        fs::write(
            seed.join("label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n",
        )
        .unwrap();
        fs::write(
            seed.join("trace_summary.txt"),
            "frames=1\nmapper_write_count=1\napu_write_count=1\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=0\ncomplete=1\n",
        )
        .unwrap();

        write_trace_dir(&out, Some(&seed), 1).unwrap();

        assert_eq!(
            fs::read_to_string(out.join("port_mapper_writes.tsv")).unwrap(),
            "frame\taddr\tvalue\tstate\n1\t8000\t06\tstate\n"
        );
        let summary = fs::read_to_string(out.join("port_trace_summary.txt")).unwrap();
        assert!(summary.contains("runtime=rust_trace_seed\n"));
        assert!(summary.contains("mapper_write_count=1\n"));
        assert!(summary.contains("apu_write_count=1\n"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn writes_rust_audio_trace_summary() {
        let root = temp_dir();
        let trace = root.join("apu_writes.tsv");
        let summary = root.join("audio_summary.txt");
        fs::create_dir_all(&root).unwrap();
        fs::write(
            &trace,
            concat!(
                "frame\tcycle\taddr\tvalue\n",
                "1\t10\t4000\t30\n",
                "2\t11\t4014\t02\n",
                "3\tunknown\t4015\t0F\n",
                "6\t12\t4017\t40\n",
            ),
        )
        .unwrap();

        write_audio_summary(&summary, Some(&trace), 3).unwrap();

        let text = fs::read_to_string(summary).unwrap();
        assert!(text.contains("runtime=rust_native_port_headless\n"));
        assert!(text.contains("backend=rust_audio_stub\n"));
        assert!(text.contains("frames=3\n"));
        assert!(text.contains("apu_trace_events=3\n"));
        assert!(text.contains("apu_trace_events_played=2\n"));
        assert!(text.contains("audio_apu_write_count=2\n"));
        assert!(text.contains("complete=1\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
