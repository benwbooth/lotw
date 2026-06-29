//! Verify the music DSL round-trips every reachable channel stream in the ROM
//! byte-exactly, and that the generated `music.rs` proc-macro DSL assembles to
//! the exact ROM bytes.

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

        // Low level: re-emitting the tokens reproduces the original bytes.
        let bytes = audio::assemble(&toks);
        assert_eq!(&prg[off..off + bytes.len()], &bytes[..], "{where_}: assemble");

        // Text DSL: render -> parse -> assemble reproduces the same tokens/bytes.
        let text = audio::render(&toks);
        let toks2 = audio::parse(&text).unwrap_or_else(|e| panic!("{where_}: parse {e}\n{text}"));
        assert_eq!(toks, toks2, "{where_}: token round-trip\n{text}");
        assert_eq!(bytes, audio::assemble(&toks2), "{where_}: byte round-trip");
    }
}

#[test]
fn music_rs_matches_rom() {
    let Ok(rom) = std::fs::read("rom/lotw.nes") else { return };
    if lotw::music::song(0).is_none() {
        eprintln!("src/music.rs not generated — skipping (run `cargo run --bin gen_music`)");
        return;
    }
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    for (idx, chans) in audio::song_channels(prg) {
        let song = lotw::music::song(idx).unwrap_or_else(|| panic!("music::song({idx}) missing"));
        for (ci, off) in chans.iter().enumerate() {
            let Some(off) = off else { continue };
            let bytes = audio::assemble(&song.channels[ci].1);
            assert_eq!(&prg[*off..*off + bytes.len()], &bytes[..], "song{idx} {}", audio::CHANNEL_NAMES[ci]);
        }
    }
}

#[test]
fn proc_macros_assemble_exact_bytes() {
    use lotw::audio::{Tok, assemble};
    use lotw::{noise, param, pulse1, raw, ser, song, triangle};

    // A reusable section spliced into a channel.
    let lick = ser![c2hd, b2q];

    let s = song! {
        pulse1![ param!(duty=0x0b, volume=0xff), lick, raw!(0x9f, e), b4 30, rh, | ],
        triangle![ c4i, c4i, c5i ],
        noise![ ds2q ],
    };

    // pulse1: duty cmd, vol cmd, c2(hd), b2(q), raw 0x9f(e), b4(30), rest(h), end
    assert_eq!(
        assemble(&s.channels[0].1),
        vec![0xff, 0, 0x0b, 0xff, 1, 0xff, 72, 0x00, 24, 0x0c, 12, 0x9f, 30, 0x2c, 0x80 | 48, 0x00]
    );
    // triangle arpeggio: c4 c4 c5 as sixteenths (c2 = nibble 0, so c4 = 0x20)
    assert_eq!(s.channels[2].1, vec![Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x20 }, Tok::Note { dur: 6, pitch: 0x30 }]);
    // noise channel parses melodically for now: ds2 quarter -> pitch idx 3
    assert_eq!(s.channels[3].1, vec![Tok::Note { dur: 24, pitch: 0x03 }]);
}
