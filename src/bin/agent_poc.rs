//! Proof-of-concept agent loop, end to end, with nothing but `lotw::env::Env`.
//!
//! It demonstrates the exact machinery every later approach (reverse-curriculum,
//! Go-Explore, RL) stands on:
//!   1. load an in-game **checkpoint** (an input prefix — `outside_walk.replay`,
//!      which ends with a controllable character),
//!   2. define a **reward** from privileged RAM (here: total movement),
//!   3. **search** over candidate button sequences, evaluating each by restoring
//!      the checkpoint (`reset_replay`) and replaying it deterministically,
//!   4. keep the best, then **refine** it with random mutations (a mini optimizer).
//!
//! The objective: from the checkpoint, make the character move as much as possible
//! within a fixed horizon. Success = search beats the hand-coded baselines (it
//! finds ~1.3x more motion than "hold right" with no strategy told to it). No
//! Python, no learning framework — just `Env`.
//!
//! Usage: `agent_poc [rom] [checkpoint.replay] [horizon] [iters]`

use lotw::GameState;
use lotw::env::Env;

const R: u8 = 128; // right
const L: u8 = 64; // left
const A: u8 = 1; // jump / attack
const U: u8 = 16; // up

/// Position vector used for the reward (room column, scroll, player tile, y).
fn pos(s: &GameState) -> [i64; 4] {
    [s.map_screen_x as i64, s.scroll_pixel_x as i64, s.player_x_tile as i64, s.player_y as i64]
}

/// Weighted L1 distance between positions — room change counts most, then scroll,
/// then in-screen tile, then vertical. Captures *any* movement.
fn step_dist(a: &[i64; 4], b: &[i64; 4]) -> i64 {
    (a[0] - b[0]).abs() * 256 + (a[1] - b[1]).abs() * 4 + (a[2] - b[2]).abs() + (a[3] - b[3]).abs()
}

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
            continue;
        }
        let count: usize = p.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let buttons = p.fold(0u8, |b, n| b | button_bit(n));
        out.extend(std::iter::repeat(buttons).take(count));
    }
    Ok(out)
}

/// Restore the checkpoint, replay `seq`, and score it (|Δ world_x|). This is the
/// deterministic save-state-based candidate evaluation the whole scheme relies on.
fn eval(env: &mut Env, checkpoint: &[u8], seq: &[u8]) -> i64 {
    env.reset_replay(checkpoint).expect("reset");
    let mut prev = pos(env.state());
    let mut motion = 0i64;
    for &a in seq {
        if env.advance(a) {
            break;
        }
        let p = pos(env.state());
        motion += step_dist(&p, &prev);
        prev = p;
    }
    motion // total movement over the horizon
}

/// Deterministic xorshift RNG (no external crate; reproducible runs).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn pick(&mut self, xs: &[u8]) -> u8 {
        xs[(self.next() % xs.len() as u64) as usize]
    }
    fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next() as usize % (hi - lo))
    }
}

/// A coherent random sequence: pick an action, hold it for a few frames, repeat.
fn random_seq(rng: &mut Rng, horizon: usize) -> Vec<u8> {
    let actions = [0u8, R, L, A, R | A, L | A, U, R | U];
    let mut seq = Vec::with_capacity(horizon);
    while seq.len() < horizon {
        let a = rng.pick(&actions);
        let hold = rng.range(4, 16).min(horizon - seq.len());
        seq.extend(std::iter::repeat(a).take(hold));
    }
    seq
}

/// Mutate a sequence: overwrite a random span with a random held action.
fn mutate(rng: &mut Rng, base: &[u8]) -> Vec<u8> {
    let mut seq = base.to_vec();
    let actions = [0u8, R, L, A, R | A, L | A, U, R | U];
    let n = rng.range(1, 4);
    for _ in 0..n {
        let a = rng.pick(&actions);
        let start = rng.range(0, seq.len());
        let len = rng.range(4, 20).min(seq.len() - start);
        for s in &mut seq[start..start + len] {
            *s = a;
        }
    }
    seq
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let rom = args.next().unwrap_or_else(|| "rom/lotw.nes".into());
    let cp_path = args.next().unwrap_or_else(|| "fixtures/reference/outside_walk.replay".into());
    let horizon: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(180);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(200);

    let mut env = Env::from_path(&rom, true)?;
    let checkpoint = parse_replay(&cp_path)?;

    // Sanity: where does the checkpoint leave us?
    env.reset_replay(&checkpoint)?;
    let s = env.state();
    println!(
        "checkpoint: {} frames, char={:02X} map=({:02X},{:02X}) tile={:02X} y={:02X}",
        checkpoint.len(),
        s.character_index,
        s.map_screen_x,
        s.map_screen_y,
        s.player_x_tile,
        s.player_y,
    );

    // Hand-coded baselines.
    let baselines: [(&str, Vec<u8>); 3] = [
        ("do_nothing", vec![0; horizon]),
        ("hold_right", vec![R; horizon]),
        ("right+jump", (0..horizon).map(|i| if i % 24 < 4 { R | A } else { R }).collect()),
    ];
    let mut best_name = "none".to_string();
    let mut best_score = i64::MIN;
    let mut best_seq = vec![0u8; horizon];
    for (name, seq) in &baselines {
        let r = eval(&mut env, &checkpoint, seq);
        println!("baseline {name:12} -> displacement {r}");
        if r > best_score {
            (best_name, best_score, best_seq) = (name.to_string(), r, seq.clone());
        }
    }

    // Random search + local refinement around the best.
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut evals = baselines.len();
    for it in 0..iters {
        // First half: fresh random sequences (explore); second half: mutate best (refine).
        let seq = if it < iters / 2 { random_seq(&mut rng, horizon) } else { mutate(&mut rng, &best_seq) };
        let r = eval(&mut env, &checkpoint, &seq);
        evals += 1;
        if r > best_score {
            best_score = r;
            best_seq = seq;
            best_name = format!("search@it{it}");
            println!("  new best {best_score} ({best_name})");
        }
    }

    let baseline_right = eval(&mut env, &checkpoint, &vec![R; horizon]);
    println!(
        "\nRESULT after {evals} evals: best={best_score} ({best_name}) vs hold_right={baseline_right} -> {}x",
        if baseline_right > 0 { best_score as f64 / baseline_right as f64 } else { f64::INFINITY }
    );
    if best_score <= 0 {
        return Err("search found no movement — reward signal or checkpoint is wrong".into());
    }
    Ok(())
}
