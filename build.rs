//! Generate the `music::note` const grid (pure note math, no ROM data) into
//! OUT_DIR, and detect whether the ROM-derived `src/music/songs.rs` is present
//! so the `music` module compiles to the real songs when it is and to a stub
//! otherwise (keeping clean checkouts building).

use std::io::Write;

// note_idx -> name (idx 5 is the unused gap; 0..=12 = C..B). Keep in sync with
// audio.rs / src/music.
const NOTE_NAMES: [&str; 13] = ["c", "cs", "d", "ds", "e", "", "f", "fs", "g", "gs", "a", "as", "b"];
// (name, num, den): a note value is num/den of a quarter note.
const VALS: &[(&str, u16, u16)] = &[
    ("w", 4, 1), ("hdd", 7, 2), ("hd", 3, 1), ("h", 2, 1), ("qdd", 7, 4), ("qd", 3, 2),
    ("q", 1, 1), ("edd", 7, 8), ("ed", 3, 4), ("e", 1, 2), ("id", 3, 8), ("i", 1, 4),
    ("td", 3, 16), ("t", 1, 8), ("x", 1, 16),
];
const BASE_OCTAVE: i32 = 2;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_music)");
    println!("cargo::rerun-if-changed=src/music/songs.rs");
    println!("cargo::rerun-if-changed=build.rs");
    if std::path::Path::new("src/music/songs.rs").exists() {
        println!("cargo::rustc-cfg=has_music");
    }

    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("note_consts.rs");
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    // Pitched consts for octaves 2..=11 (octave nibble 0..=9).
    for nib in 0u8..=9 {
        let oct = nib as i32 + BASE_OCTAVE;
        for (idx, nm) in NOTE_NAMES.iter().enumerate() {
            if nm.is_empty() {
                continue;
            }
            let pitch = (nib << 4) | idx as u8;
            for (vn, num, den) in VALS {
                writeln!(f, "pub const {nm}{oct}{vn}: Note = Note::Pitched {{ pitch: {pitch}, val: Val {{ num: {num}, den: {den} }} }};").unwrap();
            }
        }
    }
    // Rest consts.
    for (vn, num, den) in VALS {
        writeln!(f, "pub const r{vn}: Note = Note::Rest {{ val: Val {{ num: {num}, den: {den} }} }};").unwrap();
    }
}
