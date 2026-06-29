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

        // Full DSL: render -> parse -> assemble reproduces the same tokens/bytes.
        let text = audio::render(&toks);
        let toks2 = audio::parse(&text).unwrap_or_else(|e| panic!("{where_}: parse {e}\n{text}"));
        assert_eq!(toks, toks2, "{where_}: token round-trip\n{text}");
        assert_eq!(bytes, audio::assemble(&toks2), "{where_}: byte round-trip");
    }
}
