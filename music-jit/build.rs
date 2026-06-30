//! Pick the song source compiled into the JIT cdylib. By default it's the tracked
//! `src/songs.rs`; the music-server sets `LOTW_JIT_SONGS` to the live editor buffer
//! so live-editing never rewrites the tracked file. The chosen source is copied to
//! OUT_DIR (with its `//!` header demoted to `//`, which `include!` requires).

use std::io::Write;

fn main() {
    println!("cargo::rerun-if-env-changed=LOTW_JIT_SONGS");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = std::env::var("LOTW_JIT_SONGS").unwrap_or_else(|_| format!("{manifest}/src/songs.rs"));
    println!("cargo::rerun-if-changed={src}");

    let content = std::fs::read_to_string(&src).unwrap_or_else(|e| panic!("read {src}: {e}"));
    let content = content.replace("//!", "//"); // include! rejects inner doc comments
    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("songs.rs");
    std::fs::File::create(out).unwrap().write_all(content.as_bytes()).unwrap();
}
