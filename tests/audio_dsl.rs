//! Verify the music DSL round-trips every reachable channel stream in the ROM
//! byte-exactly, and that the generated songs/SFX assemble to the exact bytes.

use lotw::audio;

#[test]
fn rom_streams_round_trip() {
    let Ok(rom) = std::fs::read("rom/lotw.nes") else {
        eprintln!("rom/lotw.nes absent — skipping");
        return;
    };
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];

    let streams = audio::song_streams(prg);
    assert!(!streams.is_empty(), "no song streams found");
    for (song, ch, off) in streams {
        let where_ = format!("song{song} ch{ch} @{off:#x}");
        let toks = audio::disasm_channel(prg, off, ch == 3).unwrap_or_else(|| panic!("{where_}: disasm failed"));
        let bytes = audio::assemble(&toks);
        assert_eq!(&prg[off..off + bytes.len()], &bytes[..], "{where_}: assemble");

        // Text DSL round-trip.
        let text = audio::render(&toks);
        let toks2 = audio::parse(&text).unwrap_or_else(|e| panic!("{where_}: parse {e}\n{text}"));
        assert_eq!(toks, toks2, "{where_}: token round-trip\n{text}");
    }
}

#[test]
fn songs_match_rom() {
    let Ok(rom) = std::fs::read("rom/lotw.nes") else { return };
    if lotw::music::get(0).is_none() {
        eprintln!("src/music/songs.rs not generated — skipping (cargo run --bin gen_music)");
        return;
    }
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    for (idx, chans) in audio::song_channels(prg) {
        let song = lotw::music::get(idx).unwrap_or_else(|| panic!("music::get({idx}) missing"));
        for (ci, off) in chans.iter().enumerate() {
            let Some(off) = off else { continue };
            let bytes = audio::assemble(&song.channels[ci].1);
            assert_eq!(&prg[*off..*off + bytes.len()], &bytes[..], "song{idx} {}", audio::CHANNEL_NAMES[ci]);
        }
    }
}

#[test]
fn loops_match_rom() {
    let Ok(rom) = std::fs::read("rom/lotw.nes") else { return };
    if lotw::music::get(0).is_none() {
        return;
    }
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    // The LoopStart/NoLoop markers in the DSL must re-derive each channel's exact
    // loop target from the ROM header (bytes 4/5), per channel.
    for (idx, loops) in audio::song_loops(prg) {
        let song = lotw::music::get(idx).unwrap_or_else(|| panic!("music::get({idx}) missing"));
        for (ci, want) in loops.iter().enumerate() {
            let got = lotw_music::loop_of(&song.channels[ci].1);
            assert_eq!(got, *want, "song{idx} {} loop", audio::CHANNEL_NAMES[ci]);
        }
    }
}

#[test]
fn sfx_match_rom() {
    let Ok(rom) = std::fs::read("rom/lotw.nes") else { return };
    if lotw::music::sfx(0).is_none() {
        return;
    }
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    for (i, off) in audio::sfx_streams(prg) {
        let stream = lotw::music::sfx(i).unwrap_or_else(|| panic!("music::sfx({i}) missing"));
        let bytes = audio::assemble(&stream);
        assert_eq!(&prg[off..off + bytes.len()], &bytes[..], "sfx{i}");
    }
}

#[test]
fn note_dsl_assembles_exact_bytes() {
    use lotw::audio::{Tok, assemble};
    use lotw::music::note::*;
    use lotw::music::{section, song};

    // A reusable phrase spliced into a channel via array nesting.
    let arp: &[lotw::music::Note] = &[c4i, c4i, c5i];

    let s = song(24, &[
        // tempo 24: i=6, e=12, q=24. No terminator written — it's implicit.
        section(
            &[duty(11), volume(255), c2q, b2e, raw(159, 30)],
            &[],
            arp,
            &[ds2q],
        ),
    ]);

    // The builder appends the 0x00 terminator to each non-empty channel.
    assert_eq!(
        assemble(&s.channels[0].1),
        vec![0xff, 0, 11, 0xff, 1, 255, 24, 0x00, 12, 0x0c, 30, 159, 0x00]
    );
    // c4 = octave nibble 2 -> 0x20; c5 -> 0x30; i = 6 ticks at tempo 24; + End.
    assert_eq!(
        s.channels[2].1,
        vec![Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x30 }, Tok::End]
    );
    // noise ds2 quarter -> idx 3, 24 ticks; + End. Empty channel 1 stays empty.
    assert_eq!(s.channels[3].1, vec![Tok::Note { dur: 24, pitch: 0x03 }, Tok::End]);
    assert!(s.channels[1].1.is_empty());
}

#[test]
fn env_macro_expands() {
    use lotw::audio::Tok;
    use lotw::env;
    use lotw::music::line;
    use lotw::music::note::*;

    // Pitch bend on e3 (64th notes): absolute 8, then +8, +8, -4, then absolute 0
    // on f3. e3 = idx4 octave3 -> 0x14; f3 = idx6 -> 0x16; x at tempo 32 = 2 ticks.
    let bend = line(32, &[env!(pitch, 8 e3x, +8 e3x, +8 e3x, -4 e3x, 0 f3x)]);
    assert_eq!(
        bend,
        vec![
            Tok::Cmd { id: 3, arg: 8 }, Tok::Note { dur: 2, pitch: 0x14 },
            Tok::Cmd { id: 3, arg: 16 }, Tok::Note { dur: 2, pitch: 0x14 },
            Tok::Cmd { id: 3, arg: 24 }, Tok::Note { dur: 2, pitch: 0x14 },
            Tok::Cmd { id: 3, arg: 20 }, Tok::Note { dur: 2, pitch: 0x14 },
            Tok::Cmd { id: 3, arg: 0 }, Tok::Note { dur: 2, pitch: 0x16 },
            Tok::End,
        ]
    );

    // A value held across several notes (coarse sweep), with a bare `+` step.
    let sweep = line(24, &[env!(volume, 200 c4q d4q, + e4q)]);
    assert_eq!(
        sweep,
        vec![
            Tok::Cmd { id: 1, arg: 200 }, Tok::Note { dur: 24, pitch: 0x20 }, Tok::Note { dur: 24, pitch: 0x22 },
            Tok::Cmd { id: 1, arg: 201 }, Tok::Note { dur: 24, pitch: 0x24 },
            Tok::End,
        ]
    );
}
