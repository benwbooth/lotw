//! Replays an input fixture through the port and dumps per-frame work RAM
//! ($0000-$07FF) for the differential oracle (see tools/oracle.py).
//!
//! Usage: `replay_oracle <replay-file> [out.bin] [rom]`
//! The replay file is the `frame <count> <buttons>` text format written by
//! `play` (and parsed by `common::parse_replay`).
mod common;

use std::{env, error::Error, fs, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let replay = args.get(1).ok_or("usage: replay_oracle <replay> [out.bin] [rom]")?;
    let out_path = args.get(2).map(String::as_str).unwrap_or("/tmp/oracle_port.bin");
    let rom = args.get(3).map(String::as_str).unwrap_or("rom/lotw.nes");

    // parse_replay returns a 1-based vector (index 0 is a synthetic leading
    // frame); play the real frames in [1..].
    let input = common::parse_replay(replay)?;
    let engine = common::load_rom(rom, true)?;
    let mut runner = common::start_runner(engine)?;

    let mut out = fs::File::create(out_path)?;
    let mut frames = 0usize;
    for &buttons in input.iter().skip(1) {
        runner.engine_mut().ppu.buttons = buttons;
        if !common::step_frame(&mut runner) {
            break;
        }
        // Dump work RAM ($0000-$07FF); the harness compares only the curated
        // game-state offsets, masking the stack/APU/volatile regions.
        out.write_all(&runner.engine().state.ram_bytes()[0..0x800])?;
        frames += 1;
    }
    eprintln!("replay_oracle: {frames} frames -> {out_path}");
    Ok(())
}
