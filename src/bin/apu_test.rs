mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let song: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let secs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(6);

    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    game::ram_state_init(&mut engine, &mut r);
    game::farcall_bank_0C0D_seed(&mut engine, &mut r);
    engine.state.set_song(song);
    engine.state.set_sound_paused(0x00);
    eprintln!("song_init({song})...");
    game::song_init(&mut engine, &mut r);

    let frames = secs * common::FPS;
    let mut buf = vec![0i16; frames * common::SPF];
    for fr in 0..frames {
        game::sound_tick(&mut engine, &mut r);
        engine.apu.frame();
        engine
            .apu
            .generate(&mut buf[fr * common::SPF..(fr + 1) * common::SPF]);
    }
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
