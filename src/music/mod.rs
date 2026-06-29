//! The game's music: the DSL (re-exported from the `lotw_music` crate) plus the
//! generated songs/SFX.

pub use lotw_music::*;

// The generated songs/SFX. `src/music/songs.rs` is gitignored (ROM-derived);
// build.rs writes a `get`/`sfx` -> None stub there when it's absent, so this
// always compiles and rust-analyzer always sees the module.
mod songs;
pub use songs::*;
