//! Render a song from the DSL (`music::get`) through the real sound engine to a
//! WAV, and trace the now-playing position per channel.
//!
//! The song's channel streams are assembled from the DSL and patched into the
//! ROM's PRG image, then the actual ported sound engine plays them (so the audio
//! is game-accurate, not a re-synthesis). Each frame we read every channel's
//! live stream pointer and map it back to a token index — the foundation for
//! the editor's "highlight the playing note" feature.
//!
//! Usage: `music_render [rom] [song] [secs]`

mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, audio, game, music};

// Songs 0-9 live in PRG banks 10/11 ($8000/$A000), 10-19 in banks 12/13.
fn cpu_to_prg(cpu: usize, song: usize) -> Option<usize> {
    let (lo, hi) = if song < 10 { (0x14000, 0x16000) } else { (0x18000, 0x1A000) };
    match cpu {
        0x8000..0xA000 => Some(lo + cpu - 0x8000),
        0xA000..0xC000 => Some(hi + cpu - 0xA000),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let song: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let secs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(6);

    // --- Assemble the DSL song and patch it into a copy of the ROM ---
    let mut rom = std::fs::read(rom_path)?;
    let prg = 16..16 + rom[4] as usize * 16_384;
    let chans = audio::song_channels(&rom[prg.clone()]).into_iter().find(|(i, _)| *i == song).map(|(_, c)| c).ok_or("song not found")?;
    let dsl = music::get(song).ok_or("music::get returned None (regenerate songs.rs)")?;

    // Debug: `SOLO_CH=2` plays only that channel (others filled with rest bytes
    // so they stay silent regardless of their header loop pointer).
    let solo: Option<usize> = std::env::var("SOLO_CH").ok().and_then(|s| s.parse().ok());

    // Per channel: a token byte-offset table (for mapping live pointer -> token).
    let mut tok_at = [(); 4].map(|_| Vec::<(usize, usize)>::new()); // (prg_offset, token_index)
    for (ci, off) in chans.iter().enumerate() {
        let Some(off) = off else { continue };
        let bytes = audio::assemble(&dsl.channels[ci].1);
        if solo.is_some_and(|s| s != ci) {
            rom[prg.start + off..prg.start + off + bytes.len()].fill(0xFE); // rest 126, no End
            continue;
        }
        rom[prg.start + off..prg.start + off + bytes.len()].copy_from_slice(&bytes);
        // Walk the tokens to record where each starts.
        let mut o = *off;
        for (ti, t) in dsl.channels[ci].1.iter().enumerate() {
            tok_at[ci].push((o, ti));
            o += match t {
                audio::Tok::Note { .. } => 2,
                audio::Tok::Hit { .. } => 1,
                audio::Tok::Rest { .. } => 1,
                audio::Tok::Cmd { .. } => 3,
                audio::Tok::End => 1,
            };
        }
    }
    let patched = "/tmp/ben/scratch/music_render.nes";
    std::fs::write(patched, &rom)?;

    // --- Drive the sound engine (same setup as apu_test) ---
    let mut engine = common::load_rom(patched, false)?;
    let mut r = RoutineContext::default();
    game::ram_state_init(&mut engine, &mut r);
    game::farcall_bank_0C0D_seed(&mut engine, &mut r);
    engine.state.song = song as u8;
    engine.state.sound_paused = 0;
    game::song_init(&mut engine, &mut r);
    // Enable pulse1/pulse2/triangle/noise on the APU ($4015) — the minimal
    // headless init never powers the channels on, so they'd stay silent.
    engine.device_write(lotw::engine::reg::APU_STATUS, 0x0F);

    let frames = secs * common::FPS;
    let mut buf = vec![0i16; frames * common::SPF];
    let live_cpu = |engine: &lotw::Engine, c: usize| -> usize {
        (engine.state.sound_channel_byte(2, (c * 16) as i32) | engine.state.sound_channel_byte(3, (c * 16) as i32) << 8) as usize
    };

    println!("now-playing token index per channel (pulse1 pulse2 tri noise), every 0.5s:");
    for fr in 0..frames {
        game::sound_tick(&mut engine, &mut r);
        engine.apu.frame();
        engine.apu.generate(&mut buf[fr * common::SPF..(fr + 1) * common::SPF]);

        if fr.is_multiple_of(common::FPS / 2) {
            let toks: Vec<String> = (0..4usize)
                .map(|c| match cpu_to_prg(live_cpu(&engine, c), song) {
                    // Token whose start offset is the largest <= the live offset.
                    Some(po) => tok_at[c].iter().take_while(|t| t.0 <= po).last().map(|t| t.1.to_string()).unwrap_or_else(|| "-".into()),
                    None => "-".into(),
                })
                .collect();
            println!("  t={:4.1}s  {}", fr as f32 / common::FPS as f32, toks.join(" "));
            // DEBUG_CH=c dumps that channel's 8 shadow bytes (0:dur 1:enable 2/3:ptr
            // 4/5:loop 6:duty 7:linear/sweep/period) so we can see why it's silent.
            if let Ok(c) = std::env::var("DEBUG_CH").map(|s| s.parse::<usize>().unwrap_or(0)) {
                let b: Vec<i32> = (0..16).map(|n| engine.state.sound_channel_byte(n, (c * 16) as i32)).collect();
                println!("    ch{c} shadow: en={:#04x} ptr={:#06x} loop={:#06x} duty={:#04x} lin/sw={:#04x} vol13={:#04x} env={:#04x} all={:02x?}",
                    b[1], (b[2] | b[3] << 8), (b[4] | b[5] << 8), b[6], b[7], b[13], b[12], b);
            }
        }
    }

    common::write_wav("build/music_render.wav", &buf)?;
    let nonzero = buf.iter().filter(|s| **s != 0).count();
    let peak = buf.iter().map(|s| s.unsigned_abs() as i32).max().unwrap_or(0);
    eprintln!("wrote build/music_render.wav: {} samples, {nonzero} non-zero, peak={peak}", buf.len());
    Ok(())
}
