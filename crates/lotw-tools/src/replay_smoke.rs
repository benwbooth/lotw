use crate::fceux_capture;
use lotw_port::sha256;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

struct CaptureSpec {
    name: &'static str,
    replay: &'static str,
    frames: &'static str,
}

struct FrameCheck {
    summary_key: &'static str,
    capture: &'static str,
    frame: u64,
}

const CAPTURES: &[CaptureSpec] = &[
    CaptureSpec {
        name: "title_idle",
        replay: "title_idle.replay",
        frames: "180",
    },
    CaptureSpec {
        name: "start_game",
        replay: "start_game.replay",
        frames: "360",
    },
    CaptureSpec {
        name: "gameplay_walk",
        replay: "gameplay_walk.replay",
        frames: "492,672",
    },
    CaptureSpec {
        name: "gameplay_climb",
        replay: "gameplay_climb.replay",
        frames: "492,840",
    },
    CaptureSpec {
        name: "pochi_move",
        replay: "pochi_move.replay",
        frames: "666,906",
    },
    CaptureSpec {
        name: "password_prompt",
        replay: "password_prompt.replay",
        frames: "552,624",
    },
    CaptureSpec {
        name: "room_transition",
        replay: "room_transition.replay",
        frames: "924,1044",
    },
    CaptureSpec {
        name: "outside_walk",
        replay: "outside_walk.replay",
        frames: "1261,1830",
    },
    CaptureSpec {
        name: "door_return",
        replay: "door_return.replay",
        frames: "1261,1380",
    },
];

const FRAME_CHECKS: &[FrameCheck] = &[
    FrameCheck {
        summary_key: "title_idle_180",
        capture: "title_idle",
        frame: 180,
    },
    FrameCheck {
        summary_key: "start_game_360",
        capture: "start_game",
        frame: 360,
    },
    FrameCheck {
        summary_key: "gameplay_walk_492",
        capture: "gameplay_walk",
        frame: 492,
    },
    FrameCheck {
        summary_key: "gameplay_walk_672",
        capture: "gameplay_walk",
        frame: 672,
    },
    FrameCheck {
        summary_key: "gameplay_climb_492",
        capture: "gameplay_climb",
        frame: 492,
    },
    FrameCheck {
        summary_key: "gameplay_climb_840",
        capture: "gameplay_climb",
        frame: 840,
    },
    FrameCheck {
        summary_key: "pochi_move_666",
        capture: "pochi_move",
        frame: 666,
    },
    FrameCheck {
        summary_key: "pochi_move_906",
        capture: "pochi_move",
        frame: 906,
    },
    FrameCheck {
        summary_key: "password_prompt_552",
        capture: "password_prompt",
        frame: 552,
    },
    FrameCheck {
        summary_key: "password_prompt_624",
        capture: "password_prompt",
        frame: 624,
    },
    FrameCheck {
        summary_key: "room_transition_924",
        capture: "room_transition",
        frame: 924,
    },
    FrameCheck {
        summary_key: "room_transition_1044",
        capture: "room_transition",
        frame: 1044,
    },
    FrameCheck {
        summary_key: "outside_walk_1261",
        capture: "outside_walk",
        frame: 1261,
    },
    FrameCheck {
        summary_key: "outside_walk_1830",
        capture: "outside_walk",
        frame: 1830,
    },
    FrameCheck {
        summary_key: "door_return_1261",
        capture: "door_return",
        frame: 1261,
    },
    FrameCheck {
        summary_key: "door_return_1380",
        capture: "door_return",
        frame: 1380,
    },
];

const DIVERGENCE_CHECKS: &[(&str, &str, &str)] = &[
    (
        "title_idle_180",
        "start_game_360",
        "replay_smoke: start_game frame still matches title frame",
    ),
    (
        "gameplay_walk_492",
        "gameplay_walk_672",
        "replay_smoke: gameplay_walk did not change across movement frames",
    ),
    (
        "gameplay_climb_492",
        "gameplay_climb_840",
        "replay_smoke: gameplay_climb did not change across movement frames",
    ),
    (
        "pochi_move_666",
        "pochi_move_906",
        "replay_smoke: pochi_move did not change across movement frames",
    ),
    (
        "password_prompt_552",
        "password_prompt_624",
        "replay_smoke: password_prompt did not open the prompt",
    ),
    (
        "room_transition_924",
        "room_transition_1044",
        "replay_smoke: room_transition did not leave the house",
    ),
    (
        "outside_walk_1261",
        "outside_walk_1830",
        "replay_smoke: outside_walk did not change after outdoor movement",
    ),
    (
        "door_return_1261",
        "door_return_1380",
        "replay_smoke: door_return did not re-enter the house",
    ),
];

pub fn run(rom: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let repo_root = repo_root();
    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    for capture in CAPTURES {
        let replay = repo_root.join("fixtures/reference").join(capture.replay);
        fceux_capture::reference_run(
            rom,
            &out_dir.join(capture.name),
            Some(&replay),
            Some(capture.frames),
        )?;
    }

    let mut frame_hashes = BTreeMap::new();
    for check in FRAME_CHECKS {
        let hash = hash_for_frame(&out_dir.join(check.capture), check.frame)?;
        frame_hashes.insert(check.summary_key, hash);
    }
    for (left, right, message) in DIVERGENCE_CHECKS {
        if frame_hashes.get(left) == frame_hashes.get(right) {
            return Err((*message).into());
        }
    }

    write_summary(&repo_root, out_dir, &frame_hashes)?;
    println!(
        "replay_smoke: wrote {}",
        out_dir.join("replay_smoke_summary.txt").display()
    );
    Ok(())
}

fn hash_for_frame(dir: &Path, frame: u64) -> Result<String, Box<dyn std::error::Error>> {
    let frame_file = format!("frame_{frame:06}.ppm");
    let hash_file = dir.join("frame_hashes.sha256");
    let text = fs::read_to_string(&hash_file)?;
    for line in text.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() == 2 && fields[1] == frame_file {
            return Ok(fields[0].to_string());
        }
    }
    Err(format!("replay_smoke: missing frame hash: {}", hash_file.display()).into())
}

fn write_summary(
    repo_root: &Path,
    out_dir: &Path,
    frame_hashes: &BTreeMap<&'static str, String>,
) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("replay_smoke_summary.txt"))?;
    for check in FRAME_CHECKS {
        let hash = frame_hashes.get(check.summary_key).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("replay_smoke: missing summary key {}", check.summary_key),
            )
        })?;
        writeln!(file, "{}={}", check.summary_key, hash)?;
    }
    for capture in CAPTURES {
        let replay = repo_root.join("fixtures/reference").join(capture.replay);
        let key = format!("{}_replay_sha256", capture.name);
        writeln!(file, "{}={}", key, sha256::digest_hex(&fs::read(replay)?))?;
    }
    writeln!(file, "complete=1")?;
    Ok(())
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
