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

/// Serialize song `idx` into `out` (capacity `cap` bytes) for the player.
/// Layout (little-endian): `u32 n_channels=4`, then per channel
/// `u32 byte_len, bytes…, u32 n_sections, u32 section_start_token…`.
/// Returns bytes written, or 0 if the song is missing or the buffer is too small.
///
/// # Safety
/// `out` must be valid for `cap` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn song_blob(idx: u32, out: *mut u8, cap: usize) -> usize {
    let Some(song) = songs::get(idx as usize) else { return 0 };
    let mut buf = Vec::new();
    buf.extend_from_slice(&4u32.to_le_bytes());
    for ci in 0..4 {
        let bytes = lotw_music::assemble(&song.channels[ci].1);
        buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(&bytes);
        let starts = &song.section_starts[ci];
        buf.extend_from_slice(&(starts.len() as u32).to_le_bytes());
        for &s in starts {
            buf.extend_from_slice(&(s as u32).to_le_bytes());
        }
        // Loop target byte offset, or 0xFFFFFFFF for "no loop" (see loop_of).
        let code: u32 = match lotw_music::loop_of(&song.channels[ci].1) {
            lotw_music::Loop::To(n) => n as u32,
            lotw_music::Loop::None => u32::MAX,
        };
        buf.extend_from_slice(&code.to_le_bytes());
    }
    if buf.len() > cap {
        return 0;
    }
    unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), out, buf.len()) };
    buf.len()
}

/// Serialize SFX `idx` (a single pulse2 stream) into `out` as raw assembled
/// bytes. Returns bytes written, or 0 if missing / the buffer is too small.
///
/// # Safety
/// `out` must be valid for `cap` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfx_blob(idx: u32, out: *mut u8, cap: usize) -> usize {
    let Some(sfx) = songs::sfx(idx as usize) else { return 0 };
    let bytes = lotw_music::assemble(&sfx);
    if bytes.len() > cap {
        return 0;
    }
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len()) };
    bytes.len()
}
