//! Generate the `note` const grid (pitch×value consts, pure note math, no ROM
//! data) into OUT_DIR; `note.rs` includes it.

use std::io::Write;

// note_idx -> name (idx 5 is the unused gap; 0..=12 = C..B).
const NOTE_NAMES: [&str; 13] = ["c", "cs", "d", "ds", "e", "", "f", "fs", "g", "gs", "a", "as", "b"];
// (name, num, den): a note value is num/den of a quarter note. The `N3` names
// are triplets (e3 = an eighth-note triplet = 1/3 of a quarter).
const VALS: &[(&str, u16, u16)] = &[
    ("w", 4, 1), ("hdd", 7, 2), ("hd", 3, 1), ("h", 2, 1), ("qdd", 7, 4), ("qd", 3, 2),
    ("q", 1, 1), ("edd", 7, 8), ("ed", 3, 4), ("e", 1, 2), ("id", 3, 8), ("i", 1, 4),
    ("td", 3, 16), ("t", 1, 8), ("x", 1, 16),
    ("h3", 4, 3), ("q3", 2, 3), ("e3", 1, 3), ("i3", 1, 6), ("t3", 1, 12),
];
const BASE_OCTAVE: i32 = 2;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("note_consts.rs");
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
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
    for (vn, num, den) in VALS {
        writeln!(f, "pub const r{vn}: Note = Note::Rest {{ val: Val {{ num: {num}, den: {den} }} }};").unwrap();
    }
}
