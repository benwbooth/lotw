//! Hot-reloadable cdylib for live-edit playback: it `include!`s the song source
//! under edit and exposes a C-ABI entry that renders every song/SFX to its
//! assembled channel bytes. Recompiling this tiny crate against a prebuilt
//! `lotw-music` is the per-edit reload cost the spike measures.

mod songs;

/// Render all songs + SFX and return the total number of assembled channel
/// bytes — a cheap proof we actually ran the compiled song code.
#[unsafe(no_mangle)]
pub extern "C" fn render_total_bytes() -> u64 {
    let mut total = 0u64;
    let mut i = 0;
    while let Some(song) = songs::get(i) {
        for (_, ch) in &song.channels {
            total += lotw_music::assemble(ch).len() as u64;
        }
        i += 1;
    }
    let mut j = 0;
    while let Some(sfx) = songs::sfx(j) {
        total += lotw_music::assemble(&sfx).len() as u64;
        j += 1;
    }
    total
}
