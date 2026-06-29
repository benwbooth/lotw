//! Detect whether the generated `src/music.rs` (produced by `gen_music` from the
//! ROM) is present, so the `music` module can compile to the real songs when it
//! is and to an empty stub when it isn't (keeping clean checkouts building).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_music)");
    println!("cargo::rerun-if-changed=src/music.rs");
    if std::path::Path::new("src/music.rs").exists() {
        println!("cargo::rustc-cfg=has_music");
    }
}
