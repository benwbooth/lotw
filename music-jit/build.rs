//! Ensure `src/songs.rs` exists (a stub until the bench / live-edit loop writes
//! the real song source) so the crate always compiles.

const STUB: &str = "\
#![allow(clippy::all)]
use lotw_music::{Song, Tok};
pub fn get(_: usize) -> Option<Song> { None }
pub fn sfx(_: usize) -> Option<Vec<Tok>> { None }
";

fn main() {
    println!("cargo::rerun-if-changed=src/songs.rs");
    let songs = std::path::Path::new("src/songs.rs");
    if !songs.exists() {
        let _ = std::fs::write(songs, STUB);
    }
}
