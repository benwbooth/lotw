//! Verify the music DSL round-trips every reachable channel stream in the ROM
//! byte-exactly: bytes -> tokens -> DSL text -> tokens -> bytes.

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

        // play! DSL: render_play -> push_tok per token -> assemble matches.
        let play = audio::render_play(&toks);
        let mut toks3 = Vec::new();
        for ptok in play_tokens(&play) {
            audio::push_tok(&mut toks3, &ptok).unwrap_or_else(|e| panic!("{where_}: push {e}\n{play}"));
        }
        assert_eq!(toks, toks3, "{where_}: play round-trip\n{play}");
    }
}

/// Split a `render_play` string into tokens, keeping `[..]` groups whole.
fn play_tokens(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '[' => {
                depth += 1;
                buf.push(c);
            }
            ']' => {
                depth -= 1;
                buf.push(c);
            }
            c if c.is_whitespace() && depth == 0 => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
            }
            c => buf.push(c),
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
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
fn play_macro_assembles_exact_bytes() {
    let song = lotw::play! {
        pulse1 { [duty:0x0b] c2hd b2q [b4:30] rh [~ff:e] | }
    };
    let (_, stream) = &song.channels[0];
    assert_eq!(
        lotw::audio::assemble(stream),
        vec![0xff, 0, 0x0b, 72, 0x00, 24, 0x0c, 30, 0x2c, 0x80 | 48, 12, 0xff, 0x00]
    );
}
