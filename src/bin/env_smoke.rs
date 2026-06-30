//! Smoke test for the agent environment (`lotw::env::Env`): drive the game with a
//! `.replay` fixture (or hold a button), render, print player/game state and a
//! lit-pixel count, then confirm `reset_replay` reproduces the same RAM bit-for-bit
//! (determinism — the property the replay-checkpoint / reverse-curriculum scheme
//! relies on).
//!
//! Usage: `env_smoke [rom] [replay.replay]`
//!   with a replay → runs it; without → holds Right for 300 frames.

use lotw::env::Env;

/// Hardware controller bits (what `Env`/`ppu.buttons` take).
fn button_bit(name: &str) -> u8 {
    match name {
        "A" => 1,
        "B" => 2,
        "select" => 4,
        "start" => 8,
        "up" => 16,
        "down" => 32,
        "left" => 64,
        "right" => 128,
        _ => 0,
    }
}

/// Expand a `frame <count> <buttons…>` replay file into one byte per frame.
fn parse_replay(path: &str) -> Result<Vec<u8>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut p = line.split_whitespace();
        if p.next() != Some("frame") {
            return Err(format!("bad directive: {line}"));
        }
        let count: usize = p.next().ok_or("missing count")?.parse().map_err(|_| "bad count")?;
        let buttons = p.fold(0u8, |b, name| b | button_bit(name));
        out.extend(std::iter::repeat(buttons).take(count));
    }
    Ok(out)
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let rom = args.next().unwrap_or_else(|| "rom/lotw.nes".into());
    let inputs = match args.next() {
        Some(replay) => parse_replay(&replay)?,
        None => vec![128u8; 300], // hold Right
    };

    let mut env = Env::from_path(&rom, true)?;
    // Advance-only (no per-frame render) for a clean determinism check; sample the
    // brightest frame seen so we can tell rendering reached a real screen.
    let mut peak_lit = 0usize;
    for &a in &inputs {
        if env.advance(a) {
            break;
        }
        if env.frame_count() % 30 == 0 {
            let fb = env.render();
            peak_lit = peak_lit.max(fb.chunks_exact(3).filter(|p| p.iter().any(|c| *c != 0)).count());
        }
    }
    let s = env.state();
    println!(
        "ran {} frames: peak_lit_pixels={peak_lit}/{} char={:02X} map=({:02X},{:02X}) player_y={:02X} done={}",
        env.frame_count(),
        lotw::env::FRAME_W * lotw::env::FRAME_H,
        s.character_index,
        s.map_screen_x,
        s.map_screen_y,
        s.player_y,
        env.is_done(),
    );
    let ram_a = env.ram().to_vec();

    env.reset_replay(&inputs)?;
    let ram_b = env.ram().to_vec();
    println!("reset_replay deterministic: {}", ram_a == ram_b);
    if ram_a != ram_b {
        return Err("non-deterministic replay — save-state scheme is unsound".into());
    }
    Ok(())
}
