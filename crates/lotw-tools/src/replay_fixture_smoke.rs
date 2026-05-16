use lotw_port::replay::{Replay, ReplayError};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct ExpectedStats {
    frame_count: usize,
    pressed_frame_count: usize,
    first_pressed_frame: usize,
    last_pressed_frame: usize,
}

const EXPECTED_STATS: &[(&str, ExpectedStats)] = &[
    (
        "title_idle",
        ExpectedStats {
            frame_count: 180,
            pressed_frame_count: 0,
            first_pressed_frame: 0,
            last_pressed_frame: 0,
        },
    ),
    (
        "start_game",
        ExpectedStats {
            frame_count: 420,
            pressed_frame_count: 12,
            first_pressed_frame: 301,
            last_pressed_frame: 312,
        },
    ),
    (
        "gameplay_walk",
        ExpectedStats {
            frame_count: 732,
            pressed_frame_count: 192,
            first_pressed_frame: 301,
            last_pressed_frame: 672,
        },
    ),
    (
        "gameplay_climb",
        ExpectedStats {
            frame_count: 972,
            pressed_frame_count: 372,
            first_pressed_frame: 301,
            last_pressed_frame: 852,
        },
    ),
    (
        "pochi_move",
        ExpectedStats {
            frame_count: 906,
            pressed_frame_count: 306,
            first_pressed_frame: 301,
            last_pressed_frame: 786,
        },
    ),
    (
        "password_prompt",
        ExpectedStats {
            frame_count: 804,
            pressed_frame_count: 84,
            first_pressed_frame: 301,
            last_pressed_frame: 564,
        },
    ),
    (
        "room_transition",
        ExpectedStats {
            frame_count: 1261,
            pressed_frame_count: 301,
            first_pressed_frame: 301,
            last_pressed_frame: 1021,
        },
    ),
    (
        "outside_walk",
        ExpectedStats {
            frame_count: 1831,
            pressed_frame_count: 751,
            first_pressed_frame: 301,
            last_pressed_frame: 1711,
        },
    ),
    (
        "door_return",
        ExpectedStats {
            frame_count: 1711,
            pressed_frame_count: 631,
            first_pressed_frame: 301,
            last_pressed_frame: 1591,
        },
    ),
];

pub fn run(replay_paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if replay_paths.is_empty() {
        return Err("replay-fixture-smoke: at least one replay fixture is required".into());
    }

    let expected = EXPECTED_STATS.iter().copied().collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();

    for replay_path in replay_paths {
        let path = Path::new(replay_path);
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| {
                format!(
                    "replay-fixture-smoke: replay path has no UTF-8 file stem: {}",
                    path.display()
                )
            })?;
        let expected_stats = expected.get(name).ok_or_else(|| {
            format!("replay-fixture-smoke: no expected stats for replay fixture {name}")
        })?;
        let text = fs::read_to_string(path)?;
        let replay = Replay::parse(&text)?;
        let stats = replay.stats();

        require_eq(
            name,
            "frame_count",
            stats.frame_count,
            expected_stats.frame_count,
        )?;
        require_eq(
            name,
            "pressed_frame_count",
            stats.pressed_frame_count,
            expected_stats.pressed_frame_count,
        )?;
        require_eq(
            name,
            "first_pressed_frame",
            stats.first_pressed_frame,
            expected_stats.first_pressed_frame,
        )?;
        require_eq(
            name,
            "last_pressed_frame",
            stats.last_pressed_frame,
            expected_stats.last_pressed_frame,
        )?;
        seen.insert(name.to_string());
    }

    verify_bad_button_rejection()?;

    println!(
        "replay-fixture-smoke: {} fixtures parsed, invalid button rejected",
        seen.len()
    );
    Ok(())
}

fn require_eq(
    name: &str,
    key: &str,
    actual: usize,
    expected: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if actual != expected {
        return Err(format!(
            "replay-fixture-smoke: {name} expected {key}={expected}, got {actual}"
        )
        .into());
    }
    Ok(())
}

fn verify_bad_button_rejection() -> Result<(), Box<dyn std::error::Error>> {
    match Replay::parse("frame 1 fire\n") {
        Err(ReplayError::UnknownButton { name, .. }) if name == "fire" => Ok(()),
        Err(err) => {
            Err(format!("replay-fixture-smoke: invalid button produced wrong error: {err}").into())
        }
        Ok(_) => Err("replay-fixture-smoke: invalid button was accepted".into()),
    }
}
