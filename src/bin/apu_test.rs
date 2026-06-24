//! Headless APU driver that renders a single in-game song to a WAV file.
//!
//! Initializes the engine just far enough to start the sound engine, kicks off
//! the requested song, then ticks the sound engine and APU for a fixed duration,
//! accumulating samples into a buffer that is written to `build/song.wav`. Used
//! to audition / verify the ported audio engine without the SDL front-end.
//!
//! Usage: `apu_test [rom] [song] [secs]`
//! - `rom`  ROM path (default `rom/lotw.nes`)
//! - `song` song index to play (default 0)
//! - `secs` duration in seconds to render (default 6)

mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

/// Initialize sound, render the requested song for `secs` seconds, and write the
/// result to `build/song.wav`. Returns an error on I/O failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments with defaults.
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let song: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let secs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(6);

    // Minimal engine init: load ROM (no power-on RAM pattern), seed RAM/banks.
    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    game::ram_state_init(&mut engine, &mut r);
    game::farcall_bank_0C0D_seed(&mut engine, &mut r);
    // Select the requested song and unpause sound, then start it playing.
    engine.state.song = (song as u8);
    engine.state.sound_paused = 0;
    eprintln!("song_init({song})...");
    game::song_init(&mut engine, &mut r);

    // Allocate one big buffer sized for the whole render (frames * samples/frame).
    let frames = secs * common::FPS;
    let mut buf = vec![0i16; frames * common::SPF];
    // For each frame: tick the music engine, advance the APU, render its samples.
    for fr in 0..frames {
        game::sound_tick(&mut engine, &mut r);
        engine.apu.frame();
        engine
            .apu
            .generate(&mut buf[fr * common::SPF..(fr + 1) * common::SPF]);
    }
    // Write the WAV and report a quick non-silence sanity summary.
    common::write_wav("build/song.wav", &buf)?;
    let nonzero = buf.iter().filter(|sample| **sample != 0).count();
    let peak = buf
        .iter()
        .map(|sample| sample.abs() as i32)
        .max()
        .unwrap_or(0);
    eprintln!(
        "wrote build/song.wav: {} samples, {nonzero} non-zero, peak={peak}",
        buf.len()
    );
    Ok(())
}
