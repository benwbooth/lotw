//! Generate the `music::note` const grid (pure note math, no ROM data) into
//! OUT_DIR, and make sure the ROM-derived `src/music/songs.rs` exists (writing a
//! stub when it doesn't) so `mod songs;` always compiles and rust-analyzer sees
//! it. `gen_music` overwrites the stub with the real songs.

use std::io::Write;

const SONGS_STUB: &str = "\
//! Stub — run `cargo run --bin gen_music` to generate the real songs/SFX.
use crate::audio::{Song, Tok};
pub fn get(_: usize) -> Option<Song> { None }
pub fn sfx(_: usize) -> Option<Vec<Tok>> { None }
";

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
    println!("cargo::rerun-if-changed=src/music/songs.rs");
    println!("cargo::rerun-if-changed=build.rs");
    // Ensure the module file exists (a stub until `gen_music` writes the real one).
    let songs = std::path::Path::new("src/music/songs.rs");
    if !songs.exists() {
        let _ = std::fs::write(songs, SONGS_STUB);
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
