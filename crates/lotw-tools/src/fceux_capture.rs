use lotw_port::sha256;
use std::env;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

struct FceuxRun<'a> {
    name: &'a str,
    out_dir: &'a Path,
    rom: &'a Path,
    lua_script: &'a Path,
    done_marker: &'a Path,
    display_start: u16,
    display_end: u16,
    timeout: Duration,
    envs: Vec<(&'a str, OsString)>,
}

struct ChildGuard {
    child: Child,
}

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self { child }
    }

    fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        self.child.try_wait()
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.child.try_wait() {
            Ok(Some(_)) => {
                let _ = self.child.wait();
            }
            Ok(None) => {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
            Err(_) => {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
        }
    }
}

pub fn reference_run(
    rom: &Path,
    out_dir: &Path,
    replay: Option<&Path>,
    frames: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    require_tool("fceux", "reference_capture")?;
    require_tool("Xvfb", "reference_capture")?;

    let repo_root = repo_root();
    let rom = absolutize(rom)?;
    let out_dir = absolutize(out_dir)?;
    let replay = match replay {
        Some(path) => absolutize(path)?,
        None => repo_root.join("fixtures/reference/title_idle.replay"),
    };
    let frames = frames
        .map(str::to_string)
        .or_else(|| env::var("LOTW_REFERENCE_FRAMES").ok())
        .unwrap_or_else(|| "1,60,120,180".to_string());
    let timeout = env_u64("LOTW_REFERENCE_TIMEOUT", 30)?;

    require_file(&rom, "reference_capture: ROM not found")?;
    require_file(&replay, "reference_capture: replay not found")?;
    let rom_sha256 = file_hash(&rom)?;
    let replay_sha256 = file_hash(&replay)?;

    remove_path(&out_dir)?;
    fs::create_dir_all(out_dir.join("home"))?;
    fs::create_dir_all(out_dir.join("tmp"))?;

    let done_marker = out_dir.join(".capture.done");
    let lua_script = repo_root.join("tools/fceux_capture.lua");
    run_fceux(FceuxRun {
        name: "reference_capture",
        out_dir: &out_dir,
        rom: &rom,
        lua_script: &lua_script,
        done_marker: &done_marker,
        display_start: 99,
        display_end: 140,
        timeout: Duration::from_secs(timeout),
        envs: vec![
            ("LOTW_REFERENCE_OUT_DIR", out_dir.as_os_str().to_os_string()),
            ("LOTW_REFERENCE_REPLAY", replay.as_os_str().to_os_string()),
            ("LOTW_REFERENCE_FRAMES", OsString::from(&frames)),
            (
                "LOTW_REFERENCE_DONE",
                done_marker.as_os_str().to_os_string(),
            ),
        ],
    })?;

    let manifest = out_dir.join("capture_manifest.txt");
    require_key_value(&manifest, "complete", "1", "reference_capture")?;
    append_lines(
        &manifest,
        &[
            format!("rom_sha256={rom_sha256}"),
            format!("replay_sha256={replay_sha256}"),
        ],
    )?;

    let frame_files = collect_matching_files(&out_dir, "frame_", ".ppm")?;
    let ram_files = collect_matching_files(&out_dir, "ram_", ".bin")?;
    if frame_files.is_empty() {
        return Err("reference_capture: no frames captured".into());
    }
    if ram_files.is_empty() {
        return Err("reference_capture: no RAM dumps captured".into());
    }
    write_hashes(&out_dir, &frame_files, &out_dir.join("frame_hashes.sha256"))?;
    write_hashes(&out_dir, &ram_files, &out_dir.join("ram_hashes.sha256"))?;

    let mut summary = fs::File::create(out_dir.join("reference_summary.txt"))?;
    writeln!(summary, "rom={}", rom.display())?;
    writeln!(summary, "rom_sha256={rom_sha256}")?;
    writeln!(summary, "replay={}", replay.display())?;
    writeln!(summary, "replay_sha256={replay_sha256}")?;
    writeln!(summary, "frames={frames}")?;
    writeln!(summary, "out_dir={}", out_dir.display())?;
    writeln!(summary, "frame_count={}", frame_files.len())?;
    writeln!(summary, "ram_dump_count={}", ram_files.len())?;
    writeln!(summary, "complete=1")?;

    println!("reference_capture: wrote {}", out_dir.display());
    Ok(())
}

pub fn trace_run(
    rom: &Path,
    out_dir: &Path,
    labels: &Path,
    replay: Option<&Path>,
    frames: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    require_tool("fceux", "trace_capture")?;
    require_tool("Xvfb", "trace_capture")?;

    let repo_root = repo_root();
    let rom = absolutize(rom)?;
    let out_dir = absolutize(out_dir)?;
    let labels = absolutize(labels)?;
    let replay = match replay {
        Some(path) => absolutize(path)?,
        None => repo_root.join("fixtures/reference/title_idle.replay"),
    };
    let frames = frames
        .map(str::to_string)
        .or_else(|| env::var("LOTW_TRACE_FRAMES").ok())
        .unwrap_or_else(|| "180".to_string());
    let timeout = env_u64("LOTW_TRACE_TIMEOUT", 120)?;
    let hit_limit =
        env::var("LOTW_TRACE_LABEL_STATE_HIT_LIMIT").unwrap_or_else(|_| "32".to_string());

    for path in [&rom, &labels, &replay] {
        require_file(path, "trace_capture: file not found")?;
    }
    let rom_sha256 = file_hash(&rom)?;
    let labels_sha256 = file_hash(&labels)?;
    let replay_sha256 = file_hash(&replay)?;

    remove_path(&out_dir)?;
    fs::create_dir_all(out_dir.join("home"))?;
    fs::create_dir_all(out_dir.join("tmp"))?;

    let done_marker = out_dir.join(".trace.done");
    let lua_script = repo_root.join("tools/fceux_trace.lua");
    run_fceux(FceuxRun {
        name: "trace_capture",
        out_dir: &out_dir,
        rom: &rom,
        lua_script: &lua_script,
        done_marker: &done_marker,
        display_start: 141,
        display_end: 190,
        timeout: Duration::from_secs(timeout),
        envs: vec![
            ("LOTW_TRACE_OUT_DIR", out_dir.as_os_str().to_os_string()),
            ("LOTW_TRACE_LABELS", labels.as_os_str().to_os_string()),
            ("LOTW_TRACE_REPLAY", replay.as_os_str().to_os_string()),
            ("LOTW_TRACE_FRAMES", OsString::from(&frames)),
            (
                "LOTW_TRACE_LABEL_STATE_HIT_LIMIT",
                OsString::from(hit_limit),
            ),
            ("LOTW_TRACE_DONE", done_marker.as_os_str().to_os_string()),
        ],
    })?;

    let summary = out_dir.join("trace_summary.txt");
    require_key_value(&summary, "complete", "1", "trace_capture")?;
    for (file, prefix) in [
        ("label_states.tsv", "cpu_addr"),
        ("label_state_hits.tsv", "cpu_addr"),
        ("apu_writes.tsv", "frame"),
        ("ppu_writes.tsv", "frame"),
        ("ppu_vram_writes.tsv", "frame"),
        ("oam_dma.tsv", "frame"),
    ] {
        require_header_prefix(&out_dir.join(file), prefix, "trace_capture")?;
    }

    append_lines(
        &summary,
        &[
            format!("rom_sha256={rom_sha256}"),
            format!("labels_sha256={labels_sha256}"),
            format!("replay_sha256={replay_sha256}"),
        ],
    )?;
    for key in ["rom_sha256", "labels_sha256", "replay_sha256"] {
        require_key(&summary, key, "trace_capture")?;
    }

    write_hashes(
        &out_dir,
        &[
            "apu_writes.tsv".to_string(),
            "executed_labels.tsv".to_string(),
            "label_state_hits.tsv".to_string(),
            "label_states.tsv".to_string(),
            "mapper_writes.tsv".to_string(),
            "oam_dma.tsv".to_string(),
            "ppu_writes.tsv".to_string(),
            "ppu_vram_writes.tsv".to_string(),
            "trace_summary.txt".to_string(),
        ],
        &out_dir.join("trace_hashes.sha256"),
    )?;

    println!("trace_capture: wrote {}", out_dir.display());
    Ok(())
}

fn run_fceux(config: FceuxRun<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let display_num = find_free_display(config.display_start, config.display_end)
        .ok_or_else(|| format!("{}: could not find a free Xvfb display", config.name))?;
    let display = format!(":{display_num}");
    let socket = PathBuf::from(format!("/tmp/.X11-unix/X{display_num}"));

    let xvfb_log = fs::File::create(config.out_dir.join("xvfb.log"))?;
    let xvfb_err = xvfb_log.try_clone()?;
    let mut xvfb = ChildGuard::new(
        Command::new("Xvfb")
            .arg(&display)
            .arg("-screen")
            .arg("0")
            .arg("640x480x24")
            .arg("-nolisten")
            .arg("tcp")
            .stdout(Stdio::from(xvfb_log))
            .stderr(Stdio::from(xvfb_err))
            .spawn()?,
    );

    let mut ready = false;
    for _ in 0..50 {
        if socket.exists() {
            ready = true;
            break;
        }
        if let Some(status) = xvfb.try_wait()? {
            return Err(format!("{}: Xvfb exited early with {status}", config.name).into());
        }
        thread::sleep(Duration::from_millis(100));
    }
    if !ready {
        return Err(format!("{}: Xvfb display did not become ready", config.name).into());
    }

    let fceux_log = fs::File::create(config.out_dir.join("fceux.log"))?;
    let fceux_err = fceux_log.try_clone()?;
    let mut command = Command::new("fceux");
    command
        .env("DISPLAY", &display)
        .env("HOME", config.out_dir.join("home"))
        .env("TMPDIR", config.out_dir.join("tmp"))
        .env("QT_QPA_PLATFORM", "xcb")
        .env("SDL_AUDIODRIVER", "dummy");
    for (key, value) in config.envs {
        command.env(key, value);
    }
    let mut fceux = ChildGuard::new(
        command
            .arg("--no-config")
            .arg("1")
            .arg("--sound")
            .arg("0")
            .arg("--frameskip")
            .arg("0")
            .arg("--xscale")
            .arg("1")
            .arg("--yscale")
            .arg("1")
            .arg("--loadlua")
            .arg(config.lua_script)
            .arg(config.rom)
            .stdout(Stdio::from(fceux_log))
            .stderr(Stdio::from(fceux_err))
            .spawn()?,
    );

    let deadline = Instant::now() + config.timeout;
    loop {
        if config.done_marker.is_file() {
            break;
        }
        if let Some(status) = fceux.try_wait()? {
            return Err(format!(
                "{}: FCEUX exited before capture completed with {status}; see {}",
                config.name,
                config.out_dir.join("fceux.log").display()
            )
            .into());
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "{}: timed out after {}s; see {}",
                config.name,
                config.timeout.as_secs(),
                config.out_dir.join("fceux.log").display()
            )
            .into());
        }
        thread::sleep(Duration::from_millis(100));
    }

    drop(fceux);
    drop(xvfb);
    Ok(())
}

fn find_free_display(start: u16, end: u16) -> Option<u16> {
    (start..=end).find(|num| {
        let socket = PathBuf::from(format!("/tmp/.X11-unix/X{num}"));
        let lock = PathBuf::from(format!("/tmp/.X{num}-lock"));
        display_slot_available(&lock, &socket)
    })
}

fn display_slot_available(lock: &Path, socket: &Path) -> bool {
    if lock.exists() {
        match read_display_lock_pid(lock) {
            Ok(Some(pid)) if process_exists(pid) => return false,
            Ok(_) => {
                let _ = fs::remove_file(lock);
                let _ = fs::remove_file(socket);
            }
            Err(_) => return false,
        }
    } else if socket.exists() {
        let _ = fs::remove_file(socket);
    }

    !lock.exists() && !socket.exists()
}

fn read_display_lock_pid(path: &Path) -> io::Result<Option<u32>> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().parse::<u32>().ok())
}

#[cfg(target_os = "linux")]
fn process_exists(pid: u32) -> bool {
    Path::new("/proc").join(pid.to_string()).exists()
}

#[cfg(not(target_os = "linux"))]
fn process_exists(_pid: u32) -> bool {
    true
}

fn require_tool(tool: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if tool_in_path(tool) {
        Ok(())
    } else {
        Err(format!("{name}: missing required tool: {tool}").into())
    }
}

fn tool_in_path(tool: &str) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    env::split_paths(&path).any(|dir| dir.join(tool).is_file())
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{message}: {}", path.display()),
        ))
    }
}

fn require_key_value(
    path: &Path,
    key: &str,
    expected: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let value = read_key(path, key, name)?;
    if value == expected {
        Ok(())
    } else {
        Err(format!(
            "{name}: {} expected {key}={expected}, got {value}",
            path.display()
        )
        .into())
    }
}

fn require_key(path: &Path, key: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    read_key(path, key, name).map(|_| ())
}

fn read_key(path: &Path, key: &str, name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if let Some((actual, value)) = line.split_once('=') {
            if actual == key {
                return Ok(value.to_string());
            }
        }
    }
    Err(format!("{name}: missing {key} in {}", path.display()).into())
}

fn require_header_prefix(path: &Path, prefix: &str, name: &str) -> io::Result<()> {
    let text = fs::read_to_string(path)?;
    let first = text.lines().next().unwrap_or("");
    if first.starts_with(prefix) {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{name}: {} missing {prefix} header", path.display()),
        ))
    }
}

fn append_lines(path: &Path, lines: &[String]) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    for line in lines {
        writeln!(file, "{line}")?;
    }
    Ok(())
}

fn collect_matching_files(dir: &Path, prefix: &str, suffix: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(prefix) && name.ends_with(suffix) {
            files.push(name);
        }
    }
    files.sort();
    Ok(files)
}

fn write_hashes(dir: &Path, files: &[String], out: &Path) -> io::Result<()> {
    let mut output = fs::File::create(out)?;
    for file in files {
        writeln!(output, "{}  {}", file_hash(&dir.join(file))?, file)?;
    }
    Ok(())
}

fn file_hash(path: &Path) -> io::Result<String> {
    Ok(sha256::digest_hex(&fs::read(path)?))
}

fn absolutize(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

fn env_u64(key: &str, default: u64) -> Result<u64, Box<dyn std::error::Error>> {
    match env::var(key) {
        Ok(value) => Ok(value
            .parse::<u64>()
            .map_err(|err| format!("{key} must be an integer: {err}"))?),
        Err(_) => Ok(default),
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

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("lotw-tools is under crates/lotw-tools")
        .to_path_buf()
}
