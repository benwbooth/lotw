use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const FRAME_HEADER: &str = "replay\tframe\tframe_sha256";
const RAM_HEADER: &str = "replay\tframe\tram_sha256";
const INPUT_HEADER: &str =
    "replay\trom_sha256\treplay_sha256\tframes\tframe_count\tram_dump_count\tsource_summary";

#[derive(Debug, Clone)]
struct CaptureSummary {
    replay: String,
    rom_sha256: String,
    replay_sha256: String,
    frames: String,
    frame_count: usize,
    ram_dump_count: usize,
    summary_path: PathBuf,
    frame_hashes: BTreeMap<u64, String>,
    ram_hashes: BTreeMap<u64, String>,
}

pub fn run(
    capture_root: &Path,
    out_dir: &Path,
    replay_filter: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    require_dir(capture_root)?;
    let replays = if replay_filter.is_empty() {
        discover_replays(capture_root)?
    } else {
        replay_filter.to_vec()
    };
    if replays.is_empty() {
        return Err("reference_hash_report: no replay captures found".into());
    }

    let mut summaries = Vec::new();
    for replay in replays {
        summaries.push(read_capture(capture_root, &replay)?);
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;
    write_outputs(capture_root, out_dir, &summaries)?;

    println!("reference_hash_report: wrote {}", out_dir.display());
    Ok(())
}

fn require_dir(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "reference_hash_report: missing input dir: {}",
                path.display()
            ),
        ))
    }
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("reference_hash_report: missing input: {}", path.display()),
        ))
    }
}

fn discover_replays(capture_root: &Path) -> io::Result<Vec<String>> {
    let mut replays = Vec::new();
    for entry in fs::read_dir(capture_root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if entry.path().join("reference_summary.txt").is_file() {
                replays.push(name);
            }
        }
    }
    replays.sort();
    Ok(replays)
}

fn read_capture(
    capture_root: &Path,
    replay: &str,
) -> Result<CaptureSummary, Box<dyn std::error::Error>> {
    let dir = capture_root.join(replay);
    require_dir(&dir)?;
    let manifest_path = dir.join("capture_manifest.txt");
    let summary_path = dir.join("reference_summary.txt");
    let frame_hash_path = dir.join("frame_hashes.sha256");
    let ram_hash_path = dir.join("ram_hashes.sha256");
    for path in [
        &manifest_path,
        &summary_path,
        &frame_hash_path,
        &ram_hash_path,
    ] {
        require_file(path)?;
    }

    let manifest = read_key_values(&manifest_path)?;
    let summary = read_key_values(&summary_path)?;
    require_value(&manifest, "complete", "1", &manifest_path)?;
    require_value(&summary, "complete", "1", &summary_path)?;

    let frame_hashes = read_hash_file(&frame_hash_path, "frame_", ".ppm")?;
    let ram_hashes = read_hash_file(&ram_hash_path, "ram_", ".bin")?;
    let frame_count = parse_usize(
        required(&summary, "frame_count", &summary_path)?,
        &summary_path,
        "frame_count",
    )?;
    let ram_dump_count = parse_usize(
        required(&summary, "ram_dump_count", &summary_path)?,
        &summary_path,
        "ram_dump_count",
    )?;
    if frame_count == 0 || ram_dump_count == 0 {
        return Err(format!("reference_hash_report: empty capture in {}", dir.display()).into());
    }
    if frame_hashes.len() != frame_count {
        return Err(format!(
            "reference_hash_report: {} frame hash count {} != summary {}",
            dir.display(),
            frame_hashes.len(),
            frame_count
        )
        .into());
    }
    if ram_hashes.len() != ram_dump_count {
        return Err(format!(
            "reference_hash_report: {} RAM hash count {} != summary {}",
            dir.display(),
            ram_hashes.len(),
            ram_dump_count
        )
        .into());
    }
    if frame_hashes.keys().collect::<BTreeSet<_>>() != ram_hashes.keys().collect::<BTreeSet<_>>() {
        return Err(format!(
            "reference_hash_report: frame/RAM hash frame sets differ in {}",
            dir.display()
        )
        .into());
    }

    Ok(CaptureSummary {
        replay: replay.to_string(),
        rom_sha256: required(&summary, "rom_sha256", &summary_path)?.to_string(),
        replay_sha256: required(&summary, "replay_sha256", &summary_path)?.to_string(),
        frames: required(&summary, "frames", &summary_path)?.to_string(),
        frame_count,
        ram_dump_count,
        summary_path,
        frame_hashes,
        ram_hashes,
    })
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
        .ok_or_else(|| format!("reference_hash_report: missing {key} in {}", path.display()).into())
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
            "reference_hash_report: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn parse_usize(value: &str, path: &Path, key: &str) -> Result<usize, Box<dyn std::error::Error>> {
    value.parse::<usize>().map_err(|err| {
        format!(
            "reference_hash_report: {} invalid {key}={value}: {err}",
            path.display()
        )
        .into()
    })
}

fn read_hash_file(path: &Path, prefix: &str, suffix: &str) -> io::Result<BTreeMap<u64, String>> {
    let text = fs::read_to_string(path)?;
    let mut hashes = BTreeMap::new();
    for (line_no, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid hash line", path.display(), line_no + 1),
            ));
        }
        if !is_sha256(fields[0]) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid sha256", path.display(), line_no + 1),
            ));
        }
        let frame = parse_numbered_file(fields[1], prefix, suffix).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} {err}", path.display(), line_no + 1),
            )
        })?;
        if hashes.insert(frame, fields[0].to_string()).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} duplicate frame {frame}", path.display(), line_no + 1),
            ));
        }
    }
    Ok(hashes)
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn parse_numbered_file(name: &str, prefix: &str, suffix: &str) -> Result<u64, String> {
    let number = name
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
        .ok_or_else(|| format!("unexpected filename: {name}"))?;
    number
        .parse::<u64>()
        .map_err(|err| format!("invalid frame in {name}: {err}"))
}

fn write_outputs(
    capture_root: &Path,
    out_dir: &Path,
    summaries: &[CaptureSummary],
) -> io::Result<()> {
    let mut frame_file = fs::File::create(out_dir.join("reference_frame_hashes.tsv"))?;
    let mut ram_file = fs::File::create(out_dir.join("reference_ram_hashes.tsv"))?;
    let mut input_file = fs::File::create(out_dir.join("reference_inputs.tsv"))?;
    writeln!(frame_file, "{FRAME_HEADER}")?;
    writeln!(ram_file, "{RAM_HEADER}")?;
    writeln!(input_file, "{INPUT_HEADER}")?;

    let mut rom_hashes = BTreeSet::new();
    let mut frame_hash_count = 0usize;
    let mut ram_hash_count = 0usize;
    for summary in summaries {
        rom_hashes.insert(summary.rom_sha256.clone());
        writeln!(
            input_file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            summary.replay,
            summary.rom_sha256,
            summary.replay_sha256,
            summary.frames,
            summary.frame_count,
            summary.ram_dump_count,
            summary.summary_path.display()
        )?;
        for (frame, hash) in &summary.frame_hashes {
            writeln!(frame_file, "{}\t{}\t{}", summary.replay, frame, hash)?;
            frame_hash_count += 1;
        }
        for (frame, hash) in &summary.ram_hashes {
            writeln!(ram_file, "{}\t{}\t{}", summary.replay, frame, hash)?;
            ram_hash_count += 1;
        }
    }

    let mut manifest = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(manifest, "runtime=reference_hash_report")?;
    writeln!(manifest, "source_capture_root={}", capture_root.display())?;
    writeln!(manifest, "frame_hashes=reference_frame_hashes.tsv")?;
    writeln!(manifest, "ram_hashes=reference_ram_hashes.tsv")?;
    writeln!(manifest, "inputs=reference_inputs.tsv")?;
    writeln!(manifest, "replay_count={}", summaries.len())?;
    writeln!(manifest, "frame_hash_count={frame_hash_count}")?;
    writeln!(manifest, "ram_hash_count={ram_hash_count}")?;
    writeln!(manifest, "rom_sha256_count={}", rom_hashes.len())?;
    if rom_hashes.len() == 1 {
        writeln!(manifest, "rom_sha256={}", rom_hashes.iter().next().unwrap())?;
    }
    writeln!(
        manifest,
        "scope=hash-only manifest for repeatable FCEUX reference frame/RAM captures"
    )?;
    writeln!(
        manifest,
        "complete={}",
        u8::from(
            !summaries.is_empty() && frame_hash_count > 0 && frame_hash_count == ram_hash_count
        )
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_hash_only_reference_report() {
        let root = unique_temp_dir("reference-hash-report");
        let capture = root.join("captures");
        let out = root.join("out");
        write_capture(
            &capture,
            "title_idle",
            "1,60",
            &[(
                1,
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            )],
            &[(
                1,
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            )],
        );
        write_capture(
            &capture,
            "start_game",
            "120",
            &[(
                120,
                "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            )],
            &[(
                120,
                "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            )],
        );

        run(
            &capture,
            &out,
            &["title_idle".to_string(), "start_game".to_string()],
        )
        .unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("runtime=reference_hash_report\n"));
        assert!(manifest.contains("replay_count=2\n"));
        assert!(manifest.contains("frame_hash_count=2\n"));
        assert!(manifest.contains("ram_hash_count=2\n"));
        assert!(manifest.contains("rom_sha256_count=1\n"));
        assert!(manifest.contains("complete=1\n"));

        let frame_hashes = fs::read_to_string(out.join("reference_frame_hashes.tsv")).unwrap();
        assert_eq!(
            frame_hashes,
            "replay\tframe\tframe_sha256\n\
title_idle\t1\taaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\
start_game\t120\tcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\n"
        );
        let inputs = fs::read_to_string(out.join("reference_inputs.tsv")).unwrap();
        assert!(inputs.contains("title_idle\t"));
        assert!(inputs.contains("start_game\t"));
    }

    fn write_capture(
        root: &Path,
        replay: &str,
        frames: &str,
        frame_hashes: &[(u64, &str)],
        ram_hashes: &[(u64, &str)],
    ) {
        let dir = root.join(replay);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("capture_manifest.txt"),
            format!(
                "emulator=fceux\nscript=tools/fceux_capture.lua\nreplay={replay}\nframes={frames}\ncomplete=1\n"
            ),
        )
        .unwrap();
        fs::write(
            dir.join("reference_summary.txt"),
            format!(
                "rom=/tmp/game.nes\nrom_sha256=079f648d669966357fe4414a986573eacd7ecadf5c4f289c288427b8c5f491f1\nreplay={replay}\nreplay_sha256=eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\nframes={frames}\nout_dir={}\nframe_count={}\nram_dump_count={}\ncomplete=1\n",
                dir.display(),
                frame_hashes.len(),
                ram_hashes.len()
            ),
        )
        .unwrap();
        let frame_text = frame_hashes
            .iter()
            .map(|(frame, hash)| format!("{hash}  frame_{frame:06}.ppm\n"))
            .collect::<String>();
        fs::write(dir.join("frame_hashes.sha256"), frame_text).unwrap();
        let ram_text = ram_hashes
            .iter()
            .map(|(frame, hash)| format!("{hash}  ram_{frame:06}.bin\n"))
            .collect::<String>();
        fs::write(dir.join("ram_hashes.sha256"), ram_text).unwrap();
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "lotw-tools-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }
}
