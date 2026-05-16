use lotw_port::replay::{input_trace_tsv, Replay, ReplayStats};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(
    replay_path: &Path,
    out_dir: &Path,
    frame_limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(replay_path)?;
    let replay = Replay::parse(&text)?;
    let stats = replay.stats();

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    write_manifest(&out_dir.join("manifest.txt"), replay_path, &stats)?;
    write_frames(
        &out_dir.join("frames.tsv"),
        &replay,
        frame_limit.unwrap_or_else(|| replay.frame_count()),
    )?;

    println!("replay_dump: wrote {}", out_dir.display());
    Ok(())
}

fn write_manifest(path: &Path, replay_path: &Path, stats: &ReplayStats) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=rust_replay_dump")?;
    writeln!(file, "replay={}", replay_path.display())?;
    writeln!(file, "frame_count={}", stats.frame_count)?;
    writeln!(file, "pressed_frame_count={}", stats.pressed_frame_count)?;
    writeln!(file, "first_pressed_frame={}", stats.first_pressed_frame)?;
    writeln!(file, "last_pressed_frame={}", stats.last_pressed_frame)?;
    writeln!(file, "frames=frames.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_frames(path: &Path, replay: &Replay, frame_limit: usize) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(input_trace_tsv(Some(replay), frame_limit).as_bytes())?;
    Ok(())
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
            "lotw_tools_replay_dump_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_replay_manifest_and_frames() {
        let root = temp_dir();
        let replay = root.join("fixture.replay");
        let out = root.join("out");
        fs::create_dir_all(&root).unwrap();
        fs::write(&replay, "frame 2\nframe 3 start\nframe 1 right A\n").unwrap();

        run(&replay, &out, Some(6)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        let frames = fs::read_to_string(out.join("frames.tsv")).unwrap();
        assert!(manifest.contains("runtime=rust_replay_dump\n"));
        assert!(manifest.contains("frame_count=6\n"));
        assert!(manifest.contains("pressed_frame_count=4\n"));
        assert!(manifest.contains("first_pressed_frame=3\n"));
        assert!(manifest.contains("last_pressed_frame=6\n"));
        assert!(frames.contains("3\t0040\tstart\n"));
        assert!(frames.contains("6\t0018\tright A\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
