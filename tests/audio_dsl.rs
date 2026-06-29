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
        let toks = audio::disasm(prg, off).unwrap_or_else(|| panic!("{where_}: disasm failed"));
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
        // tempo 24: i=6, e=12, q=24
        section(
            &[duty(0x0b), volume(0xff), c2q, b2e, raw(0x9f, 30), bar],
            &[],
            arp,
            &[ds2q],
        ),
    ]);

    assert_eq!(
        assemble(&s.channels[0].1),
        vec![0xff, 0, 0x0b, 0xff, 1, 0xff, 24, 0x00, 12, 0x0c, 30, 0x9f, 0x00]
    );
    // c4 = octave nibble 2 -> 0x20; c5 -> 0x30; i = 6 ticks at tempo 24.
    assert_eq!(
        s.channels[2].1,
        vec![Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x30 }]
    );
    // noise ds2 quarter -> idx 3, 24 ticks.
    assert_eq!(s.channels[3].1, vec![Tok::Note { dur: 24, pitch: 0x03 }]);
}
