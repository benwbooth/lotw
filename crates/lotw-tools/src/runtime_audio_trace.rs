use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::Duration;

pub fn run(
    lotw: &Path,
    apu_trace: &Path,
    out_dir: &Path,
    frames: &str,
    rom: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    require_executable(lotw)?;
    require_file(apu_trace, "missing APU trace")?;
    let frames = parse_positive_frames(frames)?;
    if let Some(rom) = rom {
        require_file(rom, "missing ROM")?;
    }

    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    let audio_summary = out_dir.join("audio_summary.txt");
    let log = out_dir.join("runtime_audio.log");
    run_lotw(lotw, apu_trace, &audio_summary, &log, frames, rom)?;
    require_file(&audio_summary, "missing audio summary")?;

    let events = count_apu_events(apu_trace)?;
    let summary = read_key_values(&audio_summary)?;
    let played = parse_required_u64(&summary, "apu_trace_events_played", &audio_summary)?;
    let writes = parse_required_u64(&summary, "audio_apu_write_count", &audio_summary)?;
    if played != events {
        return Err(format!(
            "runtime_audio_trace: played event count mismatch: expected {events} got {played}"
        )
        .into());
    }
    if writes != events {
        return Err(format!(
            "runtime_audio_trace: audio write count mismatch: expected {events} got {writes}"
        )
        .into());
    }
    required(&summary, "backend", &audio_summary)?;
    require_value(&summary, "complete", "1", &audio_summary)?;

    println!("runtime_audio_trace: wrote {}", audio_summary.display());
    Ok(())
}

fn run_lotw(
    lotw: &Path,
    apu_trace: &Path,
    audio_summary: &Path,
    log: &Path,
    frames: u64,
    rom: Option<&Path>,
) -> io::Result<()> {
    let mut last_error = None;
    for attempt in 0..3 {
        match run_lotw_once(lotw, apu_trace, audio_summary, log, frames, rom) {
            Ok(status) if status.success() => return Ok(()),
            Ok(status) => {
                return Err(io::Error::other(format!(
                    "runtime_audio_trace: lotw failed with {status}; see {}",
                    log.display()
                )));
            }
            Err(err) if err.raw_os_error() == Some(26) && attempt < 2 => {
                last_error = Some(err);
                thread::sleep(Duration::from_millis(25));
            }
            Err(err) => return Err(err),
        }
    }
    Err(last_error.unwrap_or_else(|| io::Error::other("runtime_audio_trace: lotw failed")))
}

fn run_lotw_once(
    lotw: &Path,
    apu_trace: &Path,
    audio_summary: &Path,
    log: &Path,
    frames: u64,
    rom: Option<&Path>,
) -> io::Result<ExitStatus> {
    let log_file = fs::File::create(log)?;
    let err_file = log_file.try_clone()?;
    let mut command = Command::new(lotw);
    command
        .env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy")
        .arg("--frames")
        .arg(frames.to_string())
        .arg("--apu-trace")
        .arg(apu_trace)
        .arg("--dump-audio-summary")
        .arg(audio_summary)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    if let Some(rom) = rom {
        command.arg(rom);
    }

    command.status()
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("runtime_audio_trace: {message}: {}", path.display()),
        ))
    }
}

fn require_executable(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!(
                "runtime_audio_trace: missing executable: {}",
                path.display()
            ),
        )
    })?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "runtime_audio_trace: missing executable: {}",
                path.display()
            ),
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o111 == 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "runtime_audio_trace: missing executable: {}",
                    path.display()
                ),
            ));
        }
    }
    Ok(())
}

fn parse_positive_frames(value: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let frames = value
        .parse::<u64>()
        .map_err(|err| format!("runtime_audio_trace: frames must be a positive integer: {err}"))?;
    if frames == 0 {
        return Err("runtime_audio_trace: frames must be a positive integer".into());
    }
    Ok(frames)
}

fn count_apu_events(path: &Path) -> io::Result<u64> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .count() as u64)
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
        .ok_or_else(|| format!("runtime_audio_trace: missing {key} in {}", path.display()).into())
}

fn parse_required_u64(
    values: &HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<u64, Box<dyn std::error::Error>> {
    let value = required(values, key, path)?;
    value.parse::<u64>().map_err(|err| {
        format!(
            "runtime_audio_trace: {} invalid {key}={value}: {err}",
            path.display()
        )
        .into()
    })
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
            "runtime_audio_trace: {} expected {key}={expected}, got {actual}",
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
    fn runs_port_and_checks_audio_summary_counts() {
        let root = unique_temp_dir("runtime-audio-trace");
        let lotw = root.join("fake_lotw");
        let trace = root.join("apu_writes.tsv");
        let out = root.join("out");
        write_fake_lotw(&lotw);
        fs::write(
            &trace,
            "frame\tcycle\taddr\tvalue\n1\t0\t4015\t0F\n3\t59561\t4000\t30\n",
        )
        .unwrap();

        run(&lotw, &trace, &out, "3", None).unwrap();

        let summary = fs::read_to_string(out.join("audio_summary.txt")).unwrap();
        assert!(summary.contains("frames=3\n"));
        assert!(summary.contains("apu_trace_events_played=2\n"));
        assert!(summary.contains("audio_apu_write_count=2\n"));
        assert!(summary.contains("complete=1\n"));
        let log = fs::read_to_string(out.join("runtime_audio.log")).unwrap();
        assert!(log.contains("fake lotw\n"));
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
    let mut frames = String::new();
    let mut summary = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--frames" => {
                frames = args[i + 1].clone();
                i += 2;
            }
            "--dump-audio-summary" => {
                summary = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }
    fs::write(
        summary.expect("missing audio summary"),
        format!(
            "backend=fake\nframes={frames}\napu_trace_events=2\napu_trace_events_played=2\naudio_apu_write_count=2\ncomplete=1\n"
        ),
    )
    .unwrap();
    println!("fake lotw");
}
"#,
        );
    }
}
