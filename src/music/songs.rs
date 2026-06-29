//! Legacy of the Wizard songs + SFX as the music DSL — generated from the ROM
//! by `gen_music` (deterministic, byte-exact). Refine the notation freely; it
//! must still assemble to the same bytes (see `tests/audio_dsl.rs`).

#![allow(clippy::all)]
use lotw_music::note::*;
use lotw_music::{duty, env, flags, line, pitch, section, song, sweep, volume, Song, Tok};

// ===== songs =====

pub fn area_north() -> Song {
    song(96, &[
        section(
            &[duty(11), volume(255), c3ed, c4ed, g4t, f4t, g4t, c4ed],
            &[duty(11), pitch(4), volume(255), c3ed, c4ed, c4id, g3ed],
            &[c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x],
            &[duty(2), volume(254), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t, volume(254), duty(2), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t],
        ),
        section(
            &[c5id, g4ed, d4ed],
            &[c5id, as4ed, as3ed],
            &[as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x, as3x, as3x, as4x, as3x],
            &[volume(254), duty(2), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t, volume(254), duty(2), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t],
        ),
        section(
            &[f4id, c4id, ds4id, f4id],
            &[c4id, f3id, gs3id, as3id],
            &[ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x, gs3x, gs3x, gs4x, gs3x, c4x, gs3x, ds4x, gs3x, f4x, gs3x, gs4x, gs3x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x],
            &[volume(254), duty(2), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t, volume(254), duty(2), g11t, g11t, g11t, g11t, g11t, g11t, g11t, volume(0), duty(3), g11t],
        ),
        section(
            &[g4q, rid, raw(41, 1), raw(42, 1), raw(43, 1), raw(44, 1), raw(48, 1), raw(49, 1), raw(50, 1), raw(51, 1), raw(52, 1), raw(54, 1), raw(55, 1), raw(56, 1), volume(0), g5q],
            &[c4q, rid, raw(33, 1), raw(34, 1), raw(35, 1), raw(36, 1), raw(38, 1), raw(39, 1), raw(40, 1), raw(41, 1), raw(42, 1), raw(43, 1), raw(44, 1), raw(48, 1), volume(0), c5q],
            &[c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x],
            &[volume(254), duty(2), g11t, volume(0), duty(3), g11t, volume!(0 g11t, 0 g11t)],
        ),
        section(
            &[re, volume(0), duty(16), c4t, d4t, ds4t, f4t, g4t, c4i, d4t],
            &[re, volume(252), duty(16), c7t, d7t, ds7t, f7t, g7t, c7i, d7t],
            &[as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x],
            &[],
        ),
        section(
            &[ds4t, f4t, g4t, c4t, d4t, ds4t, f4t, d4t, c4t, as3i, d4t, ds4t, f4t, ds4t, f4t],
            &[ds7t, f7t, g7t, c7t, d7t, ds7t, f7t, d7t, c7t, as6i, d7t, ds7t, f7t, ds7t, f7t],
            &[f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x],
            &[],
        ),
        section(
            &[duty(32), volume(0), c5t, d5t, ds5t, f5t, g5t, c5i, d5t, ds5t, f5t, g5t, c5t, d5t, ds5t, f5t, d5t],
            &[duty(32), pitch(8), rt, c5t, d5t, ds5t, f5t, g5t, c5i, d5t, ds5t, f5t, g5t, c5t, d5t, ds5t, f5t],
            &[c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x],
            &[],
        ),
        section(
            &[c5t, as4i, d5t, ds5t, f5t, ds5t, d5t, duty(27), c5t, gs4t, ds4t, gs4t, ds4t, c4t, as4t, f4t],
            &[d5t, c5t, as4i, d5t, ds5t, f5t, ds5t, duty(27), volume(0), c5t, gs4t, ds4t, gs4t, ds4t, c4t, as4t, f4t],
            &[gs4x, as3x, as4x, as3x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x, gs3x, gs3x, gs4x, gs3x, cs4x, gs3x, ds4x, gs3x, f4x, gs3x, gs4x, gs3x, as3x, as3x, as4x, as3x],
            &[],
        ),
        section(
            &[d4t, f4t, d4t, as3t, c5t, gs4t, ds4t, gs4t, ds4t, c4t, as3t, d4t, f4t, d4t, f4t, as4t],
            &[d4t, f4t, d4t, as3t, c5t, gs4t, ds4t, gs4t, ds4t, c4t, as3t, d4t, f4t, d4t, f4t, as4t],
            &[ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x, gs3x, gs3x, gs4x, gs3x, cs4x, gs3x, ds4x, gs3x, f4x, gs3x, gs4x, gs3x, as3x, as3x, as4x, as3x, ds4x, as3x, f4x, as3x, gs4x, as3x, as4x, as3x],
            &[],
        ),
        section(
            &[c5t, g4t, f4t, raw(36, 108)],
            &[c5t, g4t, f4t, raw(36, 108)],
            &[c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x, c4x, c4x, c5x, c4x, f4x, c4x, g4x, c4x, as4x, c4x, c5x, c4x],
            &[],
        ),
    ])
}

pub fn area_west() -> Song {
    song(96, &[
        section(
            &[duty(27), volume(0), c4id, c5id, d5t, ds5t, d5id, as4id, g4i],
            &[duty(27), volume(0), pitch!(8 c4id, 0 ds4id f4t g4t f4id d4id as3i)],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx],
            &[duty(34), volume(254), b2t, duty(35), raw(255, 12)],
        ),
        section(
            &[c5q, c5id, b4id, g4i],
            &[g4q, d4id, raw(34, 60)],
            &[gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, g3x, rx, g3x, rx, g4x, rx, g3x, rx, g3x, rx, g3x, rx, g4x, rx, g3x, rx],
            &[],
        ),
        section(
            &[c5id, ds5id, f5t, g5t, f5id, d5id, as5i],
            &[g4id, c5id, d5t, ds5t, d5id, as4id, f5i],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx],
            &[],
        ),
        section(
            &[g5q, g5e, volume(254), d5i, f5i],
            &[as4q, b4e, pitch(8), volume(254), d5i, f5i],
            &[ds4x, rx, ds4x, rx, ds5x, rx, ds4x, rx, ds4x, rx, ds4x, rx, ds5x, rx, ds4x, rx, d4x, rx, d4x, rx, d5x, rx, d4x, rx, d4x, rx, d4x, rx, d5x, rx, d4x, rx],
            &[],
        ),
        section(
            &[f5t, ds5t, d5t, ds5i, raw(48, 60), c6i, g5i, ds5i],
            &[f5t, ds5t, d5t, ds5i, c5ed, volume(251), c6i, g5i, ds5t],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx],
            &[],
        ),
        section(
            &[f5t, ds5t, d5t, ds5i, raw(48, 60), c6i, g5i, ds5i],
            &[volume!(254 f5t ds5t d5t ds5i c5ed, 251 c6i g5i ds5t)],
            &[gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx],
            &[],
        ),
        section(
            &[volume(0), d5ed, d5t, ds5t, f5e, ds5i, f5i],
            &[volume(255), f4q, g4q],
            &[as3x, rx, as3x, rx, as4x, rx, as3x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx, b3x, rx, b3x, rx, b4x, rx, b3x, rx, b3x, rx, b3x, rx, b4x, rx, b3x, rx],
            &[],
        ),
        section(
            &[volume(254), g5q, re, d5i, f5i],
            &[c5ed, d5i, b4e, volume(254), d5i, f5i],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, g3x, rx, g3x, rx, g3x, g4x, g3x, rx, g4x, rx, g3x, rx, a3x, rx, b3x, rx],
            &[],
        ),
        section(
            &[f5t, ds5t, d5t, ds5i, raw(48, 60), c6i, g5i, ds5i],
            &[f5t, ds5t, d5t, ds5i, c5ed, volume(251), c6i, g5i, ds5t],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx],
            &[],
        ),
        section(
            &[f5t, ds5t, d5t, ds5i, raw(48, 60), c6i, g5i, ds5i],
            &[volume!(254 f5t ds5t d5t ds5i c5ed, 251 c6i g5i ds5t)],
            &[gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx, gs3x, rx, gs3x, rx, gs4x, rx, gs3x, rx],
            &[],
        ),
        section(
            &[volume(0), d5ed, d5t, ds5t, f5e, raw(51, 16), raw(50, 16), raw(43, 16)],
            &[pitch(0), volume(255), f4q, g4q],
            &[as3x, rx, as3x, rx, as4x, rx, as3x, rx, as3x, rx, as3x, rx, as4x, rx, as3x, rx, g3x, rx, g3x, rx, g4x, rx, g3x, rx, g3x, rx, g3x, rx, g4x, rx, g3x, rx],
            &[],
        ),
        section(
            &[c5edd, raw(64, 108)],
            &[g4edd, raw(56, 108)],
            &[c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx, c4x, rx, c4x, rx, c5x, rx, c4x, rx],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn home_theme() -> Song {
    song(32, &[
        section(
            &[duty(32), volume(255), c5q, d5e, ds5e, d5q, as4q, gs4q, as4e, c5e],
            &[duty(32), pitch(8), volume(255), g4q, as4e, c5e, as4q, f4q, ds4q, f4e, gs4e],
            &[c4i, c4i, c5i, c4i, ri, c4i, ri, c5i, as3i, as3i, as4i, as3i, ri, as3i, ri, as4i, gs3i, gs3i, gs4i, gs3i, ri, gs3i, ri, gs4i],
            &[volume(254), duty(34), g2i, duty(35), raw(255, 8)],
        ),
        section(
            &[b4e, c5i, d5e, g4ed, c5q, d5e, ds5e, d5q, as4q],
            &[g4e, g4i, g4e, volume!(0 g4x, 252 fs4x, 250 f4x, 248 e4x), ri, volume!(0 ds4x, 252 d4x, 250 cs4x, 248 cs4x, 255 g4q as4e c5e as4q f4q)],
            &[g3i, g3i, g4i, g3i, ri, g3i, a3i, b3i, c4i, c4i, c5i, c4i, ri, c4i, ri, c5i, as3i, as3i, as4i, as3i, ri, as3i, ri, as4i],
            &[],
        ),
        section(
            &[c5q, d5e, c5e, b4e, d5i, g5q, ri, duty(32), volume(0), g5h],
            &[gs4q, as4e, gs4e, g4ed, d5e, volume!(0 g4x, 252 fs4x, 250 f4x, 248 e4x, 0 ds4x, 252 d4x, 250 cs4x, 248 c4x, 0 g4x, 252 fs4x, 250 f4x, 248 e4x, 253 g5i ds5i as4i g5i ds5i as4i g5i ds5i)],
            &[gs3i, gs3i, gs4i, gs3i, ri, gs3i, ri, gs4i, g3i, g3i, g4i, g3i, ri, g3i, g4i, g3i, ds4i, ds4i, ds5i, ds4i, ri, ds4i, ri, ds5i],
            &[],
        ),
        section(
            &[f5q, as5q, ds5h, d5q, as4q],
            &[as4i, f5i, d5i, as4i, f5i, d5i, as4i, f5i, ds5i, as4i, g4i, ds5i, as4i, g4i, ds5i, as4i, g4i, d5i, as4i, f4i, d5i, as4i, f4i, d5i],
            &[d4i, d4i, ri, d4i, ri, d4i, ri, d5i, c4i, c4i, c5i, c4i, ri, c4i, ri, c5i, as3i, as3i, as4i, as3i, ri, as3i, as4i, as3i],
            &[],
        ),
        section(
            &[c5q, d5e, c5e, as4ed, ds5ed, g5e, g5ed, f5qdd],
            &[c5i, gs4i, ds4i, c5i, gs4i, ds4i, c5i, gs4i, ds4i, as4i, g4i, ds4i, as4i, g4i, ds4i, as4i, as4i, f4i, d4i, as4i, f4i, d4i, as4i, f4i],
            &[gs3i, gs3i, gs4i, gs3i, ri, gs3i, ri, gs4i, g3i, g3i, g4i, g3i, ri, g3i, ri, g4i, as3i, as3i, as4i, as3i, ri, as3i, ri, as4i],
            &[],
        ),
        section(
            &[as4e, ds5e, f5e, duty(32), volume(0), g5h, f5q, as5q],
            &[d4i, as4i, f4i, d4i, as4i, f4i, as4i, f4i, g5i, ds5i, as4i, g5i, ds5i, as4i, g5i, ds5i, as4i, f5i, d5i, as4i, f5i, d5i, as4i, f5i],
            &[as3i, as3i, as4i, as3i, ri, c4i, ri, d4i, ds4i, ds4i, ds5i, ds4i, ri, ds4i, ri, ds5i, d4i, d4i, ri, d4i, ri, d4i, ri, d5i],
            &[],
        ),
        section(
            &[ds5h, d5q, g5q, c5q, d5e, c5e],
            &[ds5i, as4i, g4i, ds5i, as4i, g4i, ds5i, as4i, g4i, d5i, as4i, f4i, d5i, as4i, f4i, d5i, c5i, gs4i, ds4i, c5i, gs4i, ds4i, c5i, gs4i],
            &[c4i, c4i, c5i, c4i, ri, c4i, ri, c5i, as3i, as3i, as4i, as3i, ri, as3i, as4i, as3i, gs3i, gs3i, gs4i, gs3i, ri, gs3i, ri, gs4i],
            &[],
        ),
        section(
            &[ds5e, f5e, g5e, ds5e, d5qd, c5e, raw(44, 40), duty(32), g4i, a4i, b4i],
            &[ds4i, as4i, g4i, ds4i, as4i, g4i, ds4i, as4i, b4i, g4i, d4i, b4i, g4i, d4i, b4i, g4i, d4i, b4i, g4i, d4i, volume!(0 g4x, 252 fs4x, 0 g4x, 252 fs4x, 0 ds4x, 252 d4x, 250 cs4x, 248 cs4x, 0 ds4x, 252 d4x, 0 ds4x, 252 d4x, 0 cs4x, 252 c4x, 250 b3x, 248 as3x)],
            &[g3i, g3i, g4i, g3i, ri, g3i, ri, g4i, g3i, g3i, g4i, g3i, ri, g3i, ri, g4i, g3i, g3i, g4i, g3i, ri, g3i, a3i, b3i],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn area_east() -> Song {
    song(16, &[
        section(
            &[duty(37), volume(0), d5id, rid, d5id, rid, g5ed, d5id, raw(54, 36), rid, volume(253), duty(32), d7x, e7x, d7x, e7x, rx, rx, d7x, e7x, d7x, e7x, rx, rx, volume(0), duty(37), d5id, rid, d5id, rid, g5ed, d5id, raw(54, 36), rid, volume(255), duty(32), d7x, e7x, d7x, e7x, rx, rx, d7x, e7x, d7x, e7x, rx, rx],
            &[duty(37), volume(253), g3id, rid, g3id, rid, g4id, rid, g3id, g3id, rid, c4id, cs4id, d4id, rid, f4id, fs4id, g4id, g3id, rid, g3id, rid, g4id, rid, g3id, g3id, rid, c4id, cs4id, d4id, rid, f4id, fs4id, g4id],
            &[g3id, rid, g3id, rid, g4id, rid, g3id, g3id, rid, c4id, cs4id, d4id, rid, f4id, fs4id, g4id, g3id, rid, g3id, rid, g4id, rid, g3id, g3id, rid, c4id, cs4id, d4id, rid, f4id, fs4id, g4id],
            &[duty(34), volume(254), f2id, f2id, f2id, f2id, f2id, f2id, raw(255, 6)],
        ),
        section(
            &[volume(0), duty(37), raw(56, 18), raw(50, 18), g5ed, raw(54, 18), raw(48, 18), f5ed, raw(51, 18), raw(43, 18), ds5ed, raw(54, 18), raw(48, 18), volume(253), duty(32), d7x, e7x, d7x, e7x, rx, rx, d7x, e7x, d7x, e7x, rx, rx],
            &[raw(50, 18), raw(40, 18), d5ed, raw(48, 18), raw(38, 18), c5ed, raw(43, 18), raw(35, 18), as4ed, raw(48, 18), f4qd, rid],
            &[g3id, rid, g3id, rid, g4id, g3id, rid, rid, f3id, rid, f3id, rid, f4id, f3id, rid, rid, ds3id, rid, ds3id, rid, ds4id, ds3id, rid, rid, f3id, rid, f3id, rid, f4id, f3id, rid, rid],
            &[],
        ),
        section(
            &[volume(0), duty(37), raw(56, 18), raw(50, 18), g5ed, raw(54, 18), raw(48, 18), f5ed, raw(51, 18), raw(43, 18), ds5ed, raw(50, 18), raw(42, 18), f4ed],
            &[pitch(0), raw(50, 18), raw(40, 18), d5ed, raw(48, 18), raw(38, 18), c5ed, raw(43, 18), raw(35, 18), as4ed, raw(42, 18), raw(34, 18), a3ed],
            &[g3id, rid, g3id, rid, g4id, g3id, rid, rid, f3id, rid, f3id, rid, f4id, f3id, rid, rid, ds3id, rid, ds3id, rid, ds4id, ds3id, rid, rid, f3id, rid, f3id, rid, f4id, f3id, f3id, f4id],
            &[],
        ),
        section(
            &[duty(43), raw(40, 72), d4ed, g4ed, a4hd, f4qd, c4qd],
            &[duty(43), raw(27, 72), as3ed, as3ed, c4hd, a3qd, f3qd],
            &[g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id],
            &[],
        ),
        section(
            &[raw(34, 84), raw(34, 60), d5hd],
            &[raw(24, 84), raw(23, 60), fs4hd],
            &[ds3id, rid, ds3id, ds3id, ds3id, rid, ds3id, ds3id, ds3id, rid, ds3id, ds3id, ds3id, rid, ds3id, ds3id, d3id, rid, d3id, d3id, d3id, rid, d3id, d3id, d3id, rid, d3id, d3id, d3id, rid, d3id, d3id],
            &[],
        ),
        section(
            &[raw(56, 72), a5ed, as5ed, c6hd, as5qd, c6qd],
            &[raw(43, 72), f4ed, g4ed, a4hd, f4qd, a4qd],
            &[g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, g3id, rid, g3id, g3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id, f3id, rid, f3id, f3id],
            &[],
        ),
        section(
            &[raw(66, 96), d6hd, d5hd],
            &[raw(43, 96), volume(254), b2id, d3id, g3id, d3id, g3id, b3id, d4id, b3id, d4id, g4id, b4id, g4id, b4id, d5id, g5id, b5id],
            &[as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, b3id, rid, b3id, b3id, b3id, rid, b3id, b3id, b3id, rid, b3id, b3id, b3id, rid, b3id, b3id],
            &[],
        ),
        section(
            &[duty(37), ds5qd, c5ed, g4qd, ds5ed, f5ed, ds5ed, d5qd, as4ed, g4hd, red],
            &[g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id],
            &[c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id],
            &[],
        ),
        section(
            &[a4qd, fs4ed, d4qd, a4ed, g4ed, a4ed, as4qd, c5ed, raw(50, 60)],
            &[fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id, g4id, b4id, d5id, b4id, g4id, b4id, d5id, b4id],
            &[a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, b3id, rid, b3id, b3id, b3id, rid, b3id, b3id],
            &[],
        ),
        section(
            &[ds5qd, c5ed, g4qd, ds5ed, f5ed, ds5ed, d5qd, as4ed, g4hd, red],
            &[g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, c5id, ds5id, c5id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id, g4id, as4id, d5id, as4id],
            &[c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, c4id, rid, c4id, c4id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id, as3id, rid, as3id, as3id],
            &[],
        ),
        section(
            &[a4qd, fs4ed, d4qd, a4ed, as4ed, c5ed, raw(42, 18), raw(40, 30), rqd, volume(253), duty(32), d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x, d7x, e7x],
            &[fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, fs4id, a4id, d5id, a4id, g4id, d4id, as3id, g3id, rqd, rid, pitch(10), d2td, rtd, f2td, rtd, g2td, rtd, a2td, rtd, c3td, rtd, f3td, rtd, a3td, rtd],
            &[a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, a3id, rid, a3id, a3id, g3id, g3id, g3id, g3id, rid, g3id, g3id, g3id, rid, d3id, f3id, g3id, a3id, c4id, f4id, a4id],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn area_central() -> Song {
    song(64, &[
        section(
            &[duty(37), volume!(250 d3t d4t d3t, 251 ds3t ds4t ds3t, 252 f3t f4t f3t, 253 ds3t ds4t ds3t, 254 d3t d4t d3t, 255 ds3t ds4t ds3t, 0 f3t f4t f3t ds3t ds4t ds3t)],
            &[duty(37), pitch(8), volume!(250 d3t d4t d3t, 251 ds3t ds4t ds3t, 252 f3t f4t f3t, 253 ds3t ds4t ds3t, 254 d3t d4t d3t, 255 ds3t ds4t ds3t, 0 f3t f4t f3t ds3t ds4t ds3t)],
            &[g3t, g4t, g3t, gs3t, gs4t, gs3t, as3t, as4t, as3t, gs3t, gs4t, gs3t, g3t, g4t, g3t, gs3t, gs4t, gs3t, as3t, as4t, as3t, gs3t, gs4t, gs3t],
            &[volume(254), duty(0), sweep(23), raw(255, 8)],
        ),
        section(
            &[d3t, d4t, d3t, ds3t, ds4t, ds3t, f3t, f4t, f3t, ds3t, ds4t, ds3t, d3t, d4t, d3t, ds3t, ds4t, ds3t, f3t, f4t, f3t, g3t, g4t, g3t],
            &[d3t, d4t, d3t, ds3t, ds4t, ds3t, f3t, f4t, f3t, ds3t, ds4t, ds3t, d3t, d4t, d3t, ds3t, ds4t, ds3t, f3t, f4t, f3t, g3t, g4t, g3t],
            &[g3t, g4t, g3t, gs3t, gs4t, gs3t, as3t, as4t, as3t, gs3t, gs4t, gs3t, g3t, g4t, g3t, gs3t, gs4t, gs3t, as3t, as4t, as3t, b3t, b4t, b3t],
            &[],
        ),
        section(
            &[volume(254), c3t, g3t, c4t, d4t, c4t, g3t, ds3t, as3t, ds4t, f4t, ds4t, as3t, f3t, c4t, f4t, g4t, f4t, c4t, g3t, d4t, g4t, g3t, a3t, b3t],
            &[g3e, g3t, g3t, as3e, as3t, as3t, c4e, c4t, c4t, d4e, d4t, d4t],
            &[c4i, ri, c4x, rx, c4x, rx, ds4i, ri, ds4x, rx, ds4x, rx, f4i, ri, f4x, rx, f4x, rx, g4i, ri, g4x, rx, g4x, rx],
            &[],
        ),
        section(
            &[c4t, g4t, c5t, d5t, c5t, g4t, ds4t, as4t, ds5t, f5t, ds5t, as4t, f4t, c5t, f5t, g5t, f5t, c5t, g4t, d5t, g5t, d5t, c5t, g4t],
            &[g3e, g3t, g3t, as3e, as3t, as3t, c4e, c4t, c4t, d4e, d4t, d4t],
            &[c4i, ri, c4x, rx, c4x, rx, ds4i, ri, ds4x, rx, ds4x, rx, f4i, ri, f4x, rx, f4x, rx, g4i, ri, g4x, rx, g4x, rx],
            &[],
        ),
        section(
            &[volume!(253 g3t g3t g3t f3t f3t f3t g3t g3t g3t f3t f3t f3t, 0 g3t g3t g3t), volume(253), duty!(36 g5x c6x f6x g6x f6x c6x, 37 g4t g4t g4t f4t f4t f4t)],
            &[volume!(253 c4id c4id c4id c4id, 0 d4t d4t d4t, 255 d3t d4t d3t, 253 c4id c4id)],
            &[g4i, rt, g4i, rt, g4i, rt, g4i, rt, g4x, rx, g4x, rx, g4x, rx, g3x, rx, g4x, rx, g3x, rx, g4i, rt, g4i, rt],
            &[],
        ),
        section(
            &[g4t, g4t, g4t, f4t, f4t, f4t, volume(0), g4t, g4t, g4t, volume(253), duty(36), g5x, c6x, f6x, g6x, f6x, c6x, duty(37), volume(250), g4t, rt, g4t, rt, g4t, volume(252), g4t, g4t, rt, g4t, rt, g4t, g4t],
            &[c4id, c4id, volume!(0 d4t d4t d4t, 255 d3t d4t d3t, 250 c4t), rt, c4t, rt, c4t, c4t, volume(252), d4t, rt, d4t, rt, d4t, d4t],
            &[g4i, rt, g4i, rt, g4x, rx, g4x, rx, g4x, rx, g3x, rx, g4x, rx, g3x, rx, g3t, rt, g3t, rt, g3t, g3t, g3t, rt, g3t, rt, g3t, g3t],
            &[],
        ),
        section(
            &[volume(254), g5t, rt, g5t, rt, g5t, g5t, volume(0), g5t, rt, g5t, rt, g5t, g5t],
            &[volume(254), c5t, rt, c5t, rt, c5t, c5t, volume(0), d5t, rt, d5t, rt, d5t, d5t],
            &[g3t, rt, g3t, rt, g3t, g3t, g3t, rt, g3t, rt, g3t, g3t],
            &[],
        ),
    ])
}

pub fn song_05() -> Song {
    song(48, &[
        section(
            &[duty(39), volume(253), g4i, c5i, ds5i, d5i, fs5h, g5e, d5e],
            &[duty(37), pitch(6), volume(0), c3i, ds3i, g3i, fs3i, as3i, a3i, re, d4i, cs4i, re, a3i, as3i, re],
            &[c3i, ds3i, g3i, fs3i, as3i, a3i, re, d4i, cs4i, re, a3i, as3i, re],
            &[volume(0), duty!(34 f2t f2t f2t f2t, 35 b2i, 34 f2t f2t, 35 b2i, 34 f2t f2t, 35 b2i, 34 f2t), duty(35), raw(255, 12)],
        ),
        section(
            &[ds5h, re, as4e, fs4e, g4e],
            &[c3i, ds3i, g3i, fs3i, as3i, a3i, re, d4i, cs4i, re, a3i, as3i, re],
            &[c3i, ds3i, g3i, fs3i, as3i, a3i, re, d4i, cs4i, re, a3i, as3i, re],
            &[],
        ),
        section(
            &[duty(37), volume(0), raw(35, 60), fs4e, g4i, a4ed, as4ed, fs4e],
            &[c4h, fs4ed, g4ed, d4e],
            &[c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t],
            &[],
        ),
        section(
            &[g4ed, ds4ed, fs4e, a4ed, as4ed, c5e],
            &[ds4ed, raw(32, 60), fs4ed, g4ed, a4e],
            &[c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t, c4t, rt, c4t, c5t],
            &[],
        ),
        section(
            &[cs5ed, as4ed, f4q, volume(253), pitch(8), cs4e, ds4e, f4e],
            &[as4qd, cs4q, cs4e, ds4e, f4e],
            &[as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t],
            &[],
        ),
        section(
            &[volume(0), pitch(0), cs4ed, ds4ed, f4q, volume(253), pitch(8), cs5e, f5e, as5e],
            &[as3ed, c4ed, cs4q, cs5e, f5e, as5e],
            &[as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t, as3t, rt, as3t, as4t],
            &[],
        ),
        section(
            &[volume(0), pitch(0), c5i, c5i, ds5i, g5i, fs5i, g5i, fs5i, ds5i, c5i, ds5i, g5i, as5e, a5ed],
            &[volume(253), ri, c5i, c5i, ds5i, g5i, fs5i, g5i, fs5i, ds5i, c5i, ds5i, g5i, as5e, a5ed],
            &[c3td, rx, c3td, rx, re, c3td, rx, c3td, rx, re, c3i, c4i, ds4i, g4i, fs4i, g4i, fs4i, ds4i],
            &[],
        ),
        section(
            &[c5i, c5i, ds5i, g5i, fs5i, g5i, fs5i, ds5i, c5i, ds5i, g5i, as5e, a5ed],
            &[c5i, c5i, ds5i, g5i, fs5i, g5i, fs5i, ds5i, c5i, ds5i, g5i, as5e, a5e],
            &[c3td, rx, c3td, rx, re, c3td, rx, c3td, rx, re, c3i, c4i, ds4i, g4i, fs4i, g4i, fs4i, ds4i],
            &[],
        ),
        section(
            &[c4e, fs4i, g4e, as4i, a4i, fs4i, g4e, d4i, ds4e, as3i, b3e],
            &[volume(0), c3e, fs3i, g3e, as3i, a3i, fs3i, g3e, d3i, ds3e, as2i, b2e],
            &[c3id, rt, fs3t, rt, g3e, as3t, rt, a3t, rt, fs3t, rt, g3id, rt, d3t, rt, ds3id, rt, as2t, rt, b2id, rt],
            &[],
        ),
        section(
            &[c4e, fs4i, g4e, as4i, a4i, fs4i, g4e, d5i, ds5e, as4i, b4e],
            &[c3e, fs3i, g3e, as3i, a3i, fs3i, g3e, d4i, ds4e, as3i, b3e],
            &[c3id, rt, fs3t, rt, g3e, as3t, rt, a3t, rt, fs3t, rt, g3id, rt, d4t, rt, ds4id, rt, as3t, rt, b3id, rt],
            &[],
        ),
        section(
            &[duty(0)],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_06() -> Song {
    song(96, &[
        section(
            &[duty(32), volume(0), e4t, rt, g4t, rx, c5t, rx, as4t, f4t, raw(40, 54), rx, f4td, rx, d4td, rx],
            &[volume(255), pitch(8), duty(32), e4t, rt, g4t, rx, c5t, rx, as4t, f4t, raw(40, 54), rx, f4td, rx, d4td, rx],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, as3t, as3t, as4t, as3t, rt, as3t, as4t, as3t],
            &[volume(254), duty(34), b2t, duty(35), raw(255, 12)],
        ),
        section(
            &[e4t, rt, g4t, rx, c5t, rx, e5t, f5t, raw(56, 54), rx, f5td, rx, d5td, rx],
            &[e4t, rt, g4t, rx, c5t, rx, e5t, f5t, raw(56, 54), rx, f5td, rx, d5td, rx],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, gs3t, gs3t, gs4t, gs3t, rt, as3t, as4t, as3t],
            &[],
        ),
        section(
            &[e4t, rt, g4t, rx, c5t, rx, as4t, f4t, raw(40, 54), rx, f4td, rx, d4td, rx],
            &[e4t, rt, g4t, rx, c5t, rx, as4t, f4t, raw(40, 54), rx, f4td, rx, d4td, rx],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, as3t, as3t, as4t, as3t, rt, as3t, as4t, as3t],
            &[],
        ),
        section(
            &[e4t, rt, g4t, rx, c5t, rx, e5t, f5t, raw(56, 54), rx, c5td, rx, d5td, rx],
            &[e4t, rt, g4t, rx, c5t, rx, e5t, f5t, raw(56, 54), rx, c5td, rx, d5td, rx],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, gs3t, gs3t, gs4t, gs3t, rt, as3t, as4t, as3t],
            &[],
        ),
        section(
            &[ds5q, d5e, f5e],
            &[ds5q, d5e, f5e],
            &[gs3t, gs3t, gs4t, gs3t, rt, gs3t, gs4t, gs3t, as3t, as3t, as4t, as3t, rt, as3t, as4t, as3t],
            &[],
        ),
        section(
            &[f5t, e5t, f5t, e5i, rt, f4t, e4t, f4t, e4i, rt, c4t, g4t, c5t, d5t],
            &[f5t, e5t, f5t, e5i, rt, f4t, e4t, f4t, e4i, rt, c4t, g4t, c5t, d5t],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t],
            &[],
        ),
        section(
            &[ds5q, d5e, f5e],
            &[ds5q, d5e, f5e],
            &[gs3t, gs3t, gs4t, gs3t, rt, gs3t, gs4t, gs3t, as3t, as3t, as4t, as3t, rt, as3t, as4t, as3t],
            &[],
        ),
        section(
            &[f5t, c5t, d5t, raw(52, 108), re],
            &[f5t, c5t, d5t, raw(52, 108), re],
            &[c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t, c4t, c4t, c5t, c4t, rt, c4t, c5t, c4t],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_07() -> Song {
    song(16, &[
        section(
            &[rhd, volume(0), duty(40), b4hd, raw(48, 18), raw(44, 18), g4ed, a4hd],
            &[rhd, volume(252), duty(40), pitch(8), rest(18), b4hd, raw(48, 18), raw(44, 18), g4ed, a4hd],
            &[rhd, g3id, rid, g3id, rid, g4ed, rid, g3ed, rid, g3id, rid, g4ed, rid, g3id, fs3id, rid, fs3id, rid, fs4ed, rid, fs3ed],
            &[volume(0), duty(34), raw(255, 6)],
        ),
        section(
            &[raw(44, 18), raw(42, 30), a4hd, raw(44, 18), raw(42, 18), fs4ed, g4hd],
            &[raw(44, 18), raw(42, 30), a4hd, raw(44, 18), raw(42, 18), fs4ed, g4hd],
            &[rid, fs3id, rid, fs4ed, rid, fs3id, f3id, rid, f3id, rid, f4ed, rid, f3ed, rid, f3id, rid, f4ed, rid, f3id, e3id, rid, e3id, rid, e4ed, rid, e3ed],
            &[],
        ),
        section(
            &[raw(42, 18), raw(40, 30), g4hd, raw(42, 18), raw(40, 18), a4ed, raw(44, 36), raw(34, 36)],
            &[raw(42, 18), raw(40, 30), g4hd, raw(42, 18), raw(40, 18), a4ed, raw(44, 36)],
            &[rid, e3id, rid, e4ed, rid, e3id, ds3id, rid, ds3id, rid, ds4ed, rid, ds3ed, rid, ds3id, rid, ds4ed, rid, ds3id, d3id, rid, d3id, rid, d4ed, rid, d3ed],
            &[],
        ),
        section(
            &[b4qd, raw(42, 36), raw(36, 36), g4qd, g4ed, fs4ed, e4ed, d4qd],
            &[raw(34, 36), b4qd, raw(42, 36), raw(36, 36), g4qd, g4ed, fs4ed, e4ed],
            &[rid, d3id, rid, d4ed, rid, d3id, cs3id, rid, cs3id, rid, cs4ed, rid, cs3ed, rid, cs3id, rid, cs4ed, rid, cs3id, d3id, rid, d3id, rid, d4ed, rid, d3ed],
            &[],
        ),
        section(
            &[raw(50, 36), b4hd, raw(48, 18), raw(44, 18), c5ed, d5hd],
            &[d4qd, raw(50, 36), b4hd, raw(48, 18), raw(44, 18), c5ed, d5hd],
            &[rid, d3id, d4id, e3id, e4id, fs3id, fs4id, g3id, rid, g3id, rid, g4ed, rid, g3ed, rid, g3id, rid, g4ed, rid, g3id, fs3id, rid, fs3id, rid, fs4ed, rid, fs3ed],
            &[],
        ),
        section(
            &[raw(52, 18), raw(55, 30), g5hd, raw(58, 18), raw(56, 30), raw(56, 36), raw(52, 60)],
            &[raw(52, 18), raw(55, 30), g5hd, raw(58, 18), raw(56, 30), raw(56, 36)],
            &[rid, fs3id, rid, fs4ed, rid, fs3id, f3id, rid, f3id, rid, f4ed, rid, f3ed, rid, f3id, rid, f4ed, rid, f3id, e3id, rid, e3id, rid, e4ed, rid, e3ed],
            &[],
        ),
        section(
            &[g5ed, fs5ed, e5ed, d5ed, red, e5ed, red, d5qd, red, d5ed, red],
            &[raw(52, 42), pitch(0), volume(254), e5ed, d5ed, c5ed, b4ed, red, c5ed, red, b4qd, red, b4ed, red],
            &[rid, e3id, rid, e4ed, rid, e3id, c3id, rid, c3id, rid, c4ed, rid, c3ed, rid, c3id, rid, c4ed, rid, c3id, d3id, rid, d3id, rid, d4ed, rid, d3ed],
            &[],
        ),
        section(
            &[c5ed, b4ed, c5ed, raw(44, 72), a4ed, g4qd, d5hd],
            &[a4ed, g4ed, a4ed, raw(40, 96), red, fs4hd],
            &[rid, d3id, rid, d4ed, rid, d3id, g3id, rid, g3id, rid, g4ed, rid, g3ed, rid, g3id, rid, g4ed, rid, g3id, fs3id, rid, fs3id, rid, fs4ed, rid, fs3ed],
            &[],
        ),
        section(
            &[e5qd, fs5qd, raw(56, 96), red, g5ed, red, g5ed],
            &[b4hd, raw(52, 96), red, pitch(8), g5ed, red, g5ed],
            &[rid, fs3id, rid, fs4ed, rid, fs3id, e3id, rid, e3id, rid, e4ed, rid, e3ed, rid, e3id, rid, e4ed, rid, e3id, c3id, rid, c3id, rid, c4ed, rid, c3id],
            &[],
        ),
        section(
            &[fs5ed, e5ed, fs5ed, raw(56, 60), b4ed, a4ed, g4ed, raw(42, 60)],
            &[fs5ed, e5ed, fs5ed, raw(56, 96), pitch(0), raw(39, 60)],
            &[d3id, rid, d3id, rid, d4ed, rid, d3id, g3id, rid, g3id, rid, g4ed, rid, g3ed, rid, g3id, rid, g4ed, rid, g3id, fs3id, rid, fs3id, rid, fs4ed, rid, fs3ed],
            &[],
        ),
        section(
            &[d5hd, raw(40, 96), red, g5ed, red, g5ed],
            &[b4hd, pitch(8), raw(40, 96), red, g5ed, red, g5ed],
            &[rid, fs3id, rid, fs4ed, rid, fs3id, e3id, rid, e3id, rid, e4ed, rid, e3ed, rid, e3id, rid, e4ed, rid, e3id, c3id, rid, c3id, rid, c4ed, rid, c3id],
            &[],
        ),
        section(
            &[fs5ed, e5ed, fs5ed, raw(56, 96), duty(36), volume(254), d7x, e7x, d7x, e7x, rt, d7x, e7x, d7x, e7x, rt, volume(0), duty(40), red, c6id, rid, c6id, raw(60, 18)],
            &[fs5ed, e5ed, fs5ed, g5qd, pitch(0), c5id, rid, c5id, raw(44, 18), c5id, rid, c5id, raw(44, 30), red, g5id, rid, g5id, raw(56, 18)],
            &[d3id, rid, d3id, rid, d4ed, rid, d3id, g3id, d4id, b4id, d4id, c5id, g4id, d4id, b4id, g4id, d4id, c5id, g4id, d4id, b4id, g4id, d4id, g3id, d4id, g4id, d4id, c5id, g4id, d4id, b4id],
            &[],
        ),
        section(
            &[c6id, rid, c6id, raw(60, 30), red, c5id, rid, c5id, raw(44, 18), c5id, rid, c5id, raw(44, 30), red, c6id, rid, c6id, raw(60, 18)],
            &[g5id, rid, g5id, raw(56, 18), duty(36), a6x, b6x, a6x, b6x, rt, a6x, b6x, a6x, b6x, rt, duty(40), red, g5id, rid, g5id, raw(56, 18), g5id, rid, g5id, raw(56, 18), duty(36), d7x, e7x, d7x, e7x, rt, d7x, e7x, d7x, e7x, rt, duty(40), red, g5id, rid, g5id, raw(56, 18)],
            &[g4id, d4id, c5id, g4id, d4id, b4id, g4id, d4id, g3id, d4id, b4id, d4id, c5id, g4id, d4id, b4id, g4id, d4id, c5id, g4id, d4id, b4id, g4id, d4id, g3id, d4id, g4id, d4id, c5id, g4id, d4id, b4id],
            &[],
        ),
        section(
            &[volume(254), d5id, fs5id, a5id, fs5id, a5id, d6id, fs6id, a6id],
            &[duty(36), d7x, e7x, d7x, e7x, rt, d7x, e7x, d7x, e7x, rt, rid, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, a6x, b6x, rt],
            &[d4id, a4id, d5id, d4id, a4id, d5id, d4id, a4id],
            &[],
        ),
    ])
}

pub fn death_jingle() -> Song {
    song(64, &[
        section(
            &[duty(32), volume(0), rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t, a4id, d4id, g4i, rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t],
            &[volume(252), pitch(5), duty(32), rid, a3t, e4t, a4t, b4t, c5t, b4t, g4t, a4id, d4id, g4i, rt, a3t, e4t, a4t, b4t, c5t],
            &[a3t, e4t, a4ed, as3t, f4t, as4ed, a3t, e4t, a4ed],
            &[volume(0), duty(34), c11i, c11i, c11i, duty(35), raw(255, 32)],
        ),
        section(
            &[e5id, d5id, as4i, rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t, a4id, d4id, g4i],
            &[b4t, g4t, e5id, d5id, as4i, rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t, a4id, d4id],
            &[as3t, f4t, as4ed, a3t, e4t, a4ed, as3t, f4t, as4ed],
            &[],
        ),
        section(
            &[rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t, e5id, d5id, g5i, e5t, a4t, rt, volume(253), e5t, a4t, rt, volume(250), e5t, a4t],
            &[g4i, rt, a3t, e4t, a4t, b4t, c5t, b4t, g4t, e5id, d5id, g5i, e5t, a4t, rt, volume(250), e5t, a4t, rt],
            &[a3t, e4t, a4ed, as3t, f4t, as4ed, a3t, rt, c4t, rt, d4t, e4t, g4t, a4t],
            &[],
        ),
        section(
            &[rt, volume(247), e5t, a4t, rt, volume(244), e5t, a4t, ri],
            &[volume(247), e5t, a4t, rt, volume(244), e5t, a4t, rt, ri],
            &[rt, e4t, g4t, d4t, rt, c4t, b3t, g3t],
            &[],
        ),
    ])
}

pub fn title_theme() -> Song {
    song(32, &[
        section(
            &[duty(37), pitch(1), volume(254), a3qd, raw(36, 72), cs4ed, d4ed, e4ed],
            &[duty(37), pitch(8), volume(254), a3qd, raw(36, 72), cs4ed, d4ed, e4ed],
            &[sweep(255), a4hd, g4hd],
            &[rhd, rhd],
        ),
        section(
            &[d4qd, raw(50, 72), a4ed, f4e, c4e, d4e],
            &[d4qd, raw(50, 72), a4ed, f4e, c4e, d4e],
            &[fs4hd, f4hd],
            &[rhd, rhd],
        ),
        section(
            &[e4hd, e5hd],
            &[e4hd, volume(253), e4td, gs4td, b4td, e5td, gs4td, b4td, e5td, gs5td, b4td, e5td, gs5td, b5td, e5td, gs5td, b5td, e6td],
            &[sweep(19), e4id, e4id, e4id, e4id, e4id, e4id, e4id, e4id, e3id, e3id, e3id, e3id, e3id, e3id, e3id, e3id],
            &[volume(254), duty(34), b10id, b10id, b10id, b10id, b2id, f2td, duty(35), raw(255, 12)],
        ),
        section(
            &[duty(32), volume(0), a4qd, raw(36, 72), cs4ed, raw(26, 18), raw(33, 18), e4id],
            &[duty(32), pitch(8), volume(254), a4qd, e4ed, a6ed, e6ed, a4i, e4i, cs4i, raw(36, 18), raw(42, 18), cs5id],
            &[a3id, a3id, a4id, a3id, a3id, a3id, a4id, a3id, g3id, g3id, g4id, g3id, g3id, g3id, g4id, g3id],
            &[],
        ),
        section(
            &[fs4qd, raw(34, 72), a4ed, raw(50, 18), raw(52, 18), d5id],
            &[d5qd, fs4ed, a6ed, d6ed, a4i, f4i, d4i, raw(38, 18), raw(40, 18), f4id],
            &[fs3id, fs3id, fs4id, fs3id, fs3id, fs3id, fs4id, fs3id, f3id, f3id, f4id, f3id, f3id, f3id, f4id, f3id],
            &[],
        ),
        section(
            &[duty(32), cs5qd, a4qd, b4ed, e4ed, b4ed, cs5ed],
            &[volume(253), cs5i, a4i, e4i, a4i, e4i, cs4i, e4i, cs4i, a3i, cs4i, e4i, a4i, e5i, b4i, gs4i, b4i, gs4i, e4i, gs4i, e4i, b3i, e4i, gs4i, b4i],
            &[e3id, e3id, e4id, e3id, e3id, e3id, e4id, e3id, e3id, e3id, e4id, e3id, e3id, e3id, e4id, e3id],
            &[],
        ),
        section(
            &[d5qd, fs4qd, gs4qd, e4qd],
            &[fs5i, d5i, a4i, d5i, a4i, fs4i, a4i, fs4i, d4i, a3i, d4i, fs4i, e4i, gs4i, b4i, gs4i, b4i, e5i, b4i, e5i, gs5i, e5td, gs5td, b5td, e6td],
            &[d3id, d3id, d4id, d3id, d3id, d3id, d4id, d3id, e3id, e3id, e4id, e3id, e3id, e3id, e4id, e3id],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn ending_theme() -> Song {
    song(48, &[
        section(
            &[duty(32), volume(0), as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t],
            &[pitch(5), duty(32), volume(252), ri, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[volume(0), duty(34), f10t, f2t, duty(35), raw(255, 12)],
        ),
        section(
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5i, a5t, g5t, a5i, g5t, fs5t, a5id, d5id, c6i],
            &[as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, as5i, a5t, g5t, a5i, g5t, fs5t, a5id, d5id, c6i],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, d3i, rt, d3i, rt, d3id, rt, a3t, rt, d4t, rt, a3t, rt],
            &[],
        ),
        section(
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t],
            &[rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5i, a5t, g5t, c6i, as5t, a5t, d6id, f6id, fs6i],
            &[as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, as5i, a5t, g5t, c6i, as5t, a5t, d6id, f6id, fs6i],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, d3i, rt, d3i, rt, d3id, rt, a3t, rt, d4t, rt, a3t, rt],
            &[],
        ),
        section(
            &[duty(41), g6q, g5i, a5i, b5i, c6i, b5i, g5i, d5i, d6i, b5i, g5i, f6i, d6i],
            &[duty(41), ri, g6q, g5i, a5i, b5i, c6i, b5i, g5i, d5i, d6i, b5i, g5i, f6i],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, g3i, rt, g3i, rt, g3id, rt, b3t, rt, g4t, rt, b3t, rt],
            &[],
        ),
        section(
            &[ds6ed, c6ed, g5q, g6e, f6e, ds6e],
            &[d6i, ds6ed, c6ed, g5q, g6e, f6e, ds6e],
            &[c4i, rt, c4i, rt, c4id, rt, g4t, rt, c5t, rt, g4t, rt, c4i, rt, c4i, rt, c4id, rt, g4t, rt, c5t, rt, g4t, rt],
            &[],
        ),
        section(
            &[d6qdd, a5t, as5t, c6q, as5e, a5e],
            &[d6qdd, a5t, as5t, c6q, as5e, a5e],
            &[d4i, rt, d4i, rt, d4id, rt, a4t, rt, d5t, rt, a4t, rt, d4i, rt, d4i, rt, d4id, rt, a4t, rt, d5t, rt, a4t, rt],
            &[],
        ),
        section(
            &[as5t, a5t, g5qdd, ri, duty(32), d4t, d4t, d4t, d4t, ri, d4id, f4id, fs4i],
            &[as5t, a5t, raw(56, 78), duty(32), a3t, a3t, a3t, a3t, ri, a3id, d4id, d4i],
            &[g3i, rt, g3i, rt, g3id, rt, d3t, rt, f3t, rt, fs3t, rt, g3i, d4x, rx, d4x, rx, d4x, rx, d4x, rx, ri, d4i, rt, f4i, rt, fs4i],
            &[],
        ),
        section(
            &[duty(42), d6q, g5i, a5i, as5i, c6i, d6qdd, c6t, as5t],
            &[duty(42), volume(253), ri, d6q, g5i, a5i, as5i, c6i, d6qdd],
            &[volume(0), g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(56, 16), raw(54, 16)],
            &[c6t, as5t, c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(56, 16), raw(54, 16)],
            &[e3i, rt, e3i, rt, e3id, rt, c4t, rt, e4t, rt, c4t, rt, ds3i, rt, ds4i, rt, ds3t, rt, f3i, rt, f4i, rt, f3t, rt],
            &[],
        ),
        section(
            &[d6q, g5i, a5i, as5i, c6i, d6qdd, c6t, as5t],
            &[d6q, g5i, a5i, as5i, c6i, d6qdd],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(59, 16), raw(64, 16)],
            &[c6t, as5t, c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(59, 16), raw(64, 16)],
            &[e3i, rt, e3i, rt, e3id, rt, c4t, rt, e4t, rt, c4t, rt, ds3i, rt, ds4i, rt, ds3t, rt, f3i, rt, f4i, rt, f3t, rt],
            &[],
        ),
        section(
            &[duty(40), raw(66, 60), as5i, d6i, raw(64, 60), a5e, c6e],
            &[volume(0), pitch(0), duty(40), ri, g5ed, g5i, g5i, raw(58, 60), f5e, a5e],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[raw(66, 60), as5i, d6i, raw(64, 60), raw(58, 16), raw(59, 16), raw(64, 16)],
            &[re, g5ed, g5i, g5i, raw(58, 60), raw(54, 16), raw(56, 16), raw(58, 16)],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[as5t, a5t, g5qdd, ri, duty(41), volume(253), d4i, g4i, a4i, as4i, c5i, d5i, f5i],
            &[pitch(5), g5q, volume(252), as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t],
            &[g3i, ri, g3i, ri, g3i, ri, g3i, ri, f3i, ri, f3i, ri, f3i, ri, f3i, ri],
            &[],
        ),
        section(
            &[g5ed, raw(40, 60), volume(0), duty(32), g5e, fs5e, e5e, fs5e],
            &[as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, pitch(0), volume(0), d5e, d5e, d5e, d5e],
            &[ds3i, ri, ds3i, ri, ds3i, ri, ds3i, ri, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt],
            &[],
        ),
        section(
            &[g5q, as4i, ds5i, f5i, g5i, a5q, raw(54, 16), raw(56, 16), raw(58, 16)],
            &[ds5q, pitch(5), g4i, as4i, as4i, as4i, c5q, raw(48, 16), raw(48, 16), raw(48, 16)],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
            &[],
        ),
        section(
            &[b5h, d3i, rt, e3i, rt, f3t, rt, g3i, rt, a3i, rt, c4t, rt],
            &[d5h, g3i, rt, a3i, rt, as3t, rt, c4i, rt, d4i, rt, f4t, rt],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, g3i, rt, a3i, rt, as3t, rt, c4i, rt, d4i, rt, f4t, rt],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_11() -> Song {
    song(64, &[
        section(
            &[volume(255), duty(32), d4t, d4t, f4t, a4t, f4t, d4t, f4t, a4t, c5id, b4id, as4i, d4t, d4t, f4t, a4t, f4t, d4t, f4t, a4t],
            &[volume(254), duty(32), a3t, a3t, d4t, f4t, d4t, a3t, d4t, f4t, gs4id, g4id, fs4i, a3t, a3t, d4t, f4t, d4t, a3t, d4t, f4t],
            &[d3x, rx, d3x, rx, red, f3id, e3id, ds3i, d3x, rx, d3x, rx, red],
            &[volume(254), duty!(35 g2t, 34 g2t g2t g2t), duty(35), raw(255, 8)],
        ),
        section(
            &[c5id, b4id, as4i, a4e, d4t, e4t, f4t, g4t, a4i, g4t, f4t, rt, c5id],
            &[gs4id, g4id, fs4i, pitch(8), a4e, d4t, e4t, f4t, g4t, a4i, g4t, f4t, rt, c5id],
            &[f3id, e3id, ds3i, d3t, d4t, d3t, d4t, d3t, d4t, d3t, d4t, e3t, e4t, e3t, e4t, e3t, e4t, e3t, e4t],
            &[],
        ),
        section(
            &[a4q, rt, duty!(0 as5t a5t gs5t g5t fs5t f5t e5t, 32 a4e d4t e4t f4t g4t)],
            &[a4q, rt, rt, volume(252), pitch(0), duty(0), as5t, a5t, gs5t, g5t, fs5t, f5t, duty(32), pitch(8), volume(254), a4e, d4t, e4t, f4t, g4t],
            &[f3t, f4t, f3t, f4t, f3t, f4t, f3t, f4t, e3t, e4t, e3t, e4t, e3t, e4t, e3t, e4t, d3t, d4t, d3t, d4t, d3t, d4t, d3t, d4t],
            &[],
        ),
        section(
            &[a4i, g4t, f4t, rt, c5id, a4q, rt, duty(0), as5t, a5t, gs5t, g5t, fs5t, f5t, e5t],
            &[a4i, g4t, f4t, rt, c5id, a4q, rt, rt, duty(0), pitch(0), volume(252), as5t, a5t, gs5t, g5t, fs5t, f5t],
            &[e3t, e4t, e3t, e4t, e3t, e4t, e3t, e4t, f3t, f4t, f3t, f4t, f3t, f4t, f3t, f4t, e3t, e4t, e3t, e4t, e3t, e4t, e3t, e4t],
            &[],
        ),
        section(
            &[duty(32), as4t, f4t, d4t, f4t, rt, d4id, ri, duty(0), d5t, d5t, c5t, d5i, rt, duty(32), d4e, e4e],
            &[duty(32), pitch(8), volume(254), as4t, f4t, d4t, f4t, rt, d4id, ri, pitch(0), duty(0), g4t, g4t, f4t, g4i, rt, duty(32), a3e, b3e],
            &[ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, ds3t, ds4t, f3t, f4t, f3t, f4t, g3t, g4t, g3t, g4t],
            &[],
        ),
        section(
            &[f4e, g4e, a4q, rq],
            &[c4e, d4e, e4q, rt, a4t, c5t, e5t, a5t, c5t, e5t, a5t],
            &[gs3t, gs4t, gs3t, gs4t, as3t, as4t, as3t, as4t, c4t, c5t, c4t, c5t, c4t, c5t, c4t, c5t, c4t, c5t, c4t, c5t, c4t, c5t, c4t, c5t],
            &[],
        ),
        section(
            &[a4q, rq],
            &[e4q, rt, a4t, cs5t, e5t, a5t, cs5t, e5t, a5t],
            &[cs4t, cs5t, cs4t, cs5t, cs4t, cs5t, cs4t, cs5t, cs4t, cs5t, cs4t, cs5t, cs4t, cs5t, cs4t, cs5t],
            &[],
        ),
    ])
}

pub fn song_12() -> Song {
    song(64, &[
        section(
            &[duty(41), volume!(250 a5t fs5t, 251 d5t b5t, 252 g5t d5t, 253 a5t fs5t d5t b5t g5t d5t a5t fs5t, 254 d5t b5t g5t d5t a5t fs5t d5t b5t g5t d5t)],
            &[duty(41), pitch(8), rt, a5t, fs5t, volume!(248 d5t b5t, 249 g5t d5t, 250 a5t fs5t d5t b5t g5t d5t, 251 a5t fs5t d5t b5t g5t, 252 d5t a5t fs5t d5t b5t g5t)],
            &[sweep(255), d4qd, c4qd],
            &[volume(0), duty(34), raw(192, 8), rt, g2t, raw(192, 8), rt, g2t, raw(192, 8), rt, e2x, e2x, g10t, g2t, rt, e2t, g2x, e2t, e2x, e2x, g2x, ri, c11t, c11t, g10t, e2x, c11t, c11t, c11t],
        ),
        section(
            &[a5t, fs5t, d5t, b5t, g5t, d5t, a5t, fs5t, d5t, b5t, g5t, d5t, a5t, f5t, c5t, d5t, c5t, g4t, a4id, g4id],
            &[d5t, a5t, fs5t, d5t, b5t, g5t, d5t, a5t, fs5t, d5t, b5t, g5t, d5t, a5t, f5t, c5t, d5t, c5t, g4t, a4id, g4id],
            &[raw(28, 80), g3t, a3t, as3i, rt, as3i, rt, sweep(26), c4t, c4t, c4t, c4t, c4t, c4t],
            &[g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t],
        ),
        section(
            &[volume(0), fs4ed, g4id, a4id, e4ed, fs4ed],
            &[rt, fs4ed, g4id, a4id, e4ed, fs4ed],
            &[d4t, ri, d4t, ri, d4t, d4t, d4t, d4t, ri, c4t, ri, c4t, ri, c4t, c4t, c4t, c4t, ri],
            &[e2x, c11t, c11t, g10t, g10t, g2t, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t],
        ),
        section(
            &[g4ed, d4id, e4id, f4e, e4t, f4t, c4t, g3t, c4t, d4t, e4t, g4t],
            &[g4ed, d4id, e4id, f4e, e4t, f4t, c4t, g3t, c4t, d4t],
            &[b3t, ri, b3t, ri, b3t, b3t, b3t, b3t, ri, as3t, ri, as3t, ri, c4t, c4t, c4t, c4t, c4t, c4t],
            &[c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g10t, e2x, c11t, c11t, c11t, g2t, c2t, volume(0), duty(40), a4e, e5i],
        ),
        section(
            &[fs4ed, g4id, a4id, e4ed, fs4ed],
            &[e4t, g4t, fs4ed, g4id, a4id, e4ed, fs4ed],
            &[d4t, ri, d4t, ri, d4t, d4t, d4t, d4t, ri, c4t, ri, c4t, ri, c4t, c4t, c4t, c4t, ri],
            &[d5e, g5ed, f5ed, c5e, d5ed],
        ),
        section(
            &[g4ed, d4id, g4id, d5id, c5id, g4id, a4id],
            &[g4ed, d4id, g4id, d5id, c5id, g4id, a4t],
            &[b3t, ri, b3t, ri, b3t, b3t, b3t, b3t, ri, as3t, ri, as3t, ri, c4t, c4t, c4t, c4t, c4t, c4t],
            &[a4e, e5i, d5e, g5ed, f5ed],
        ),
        section(
            &[raw(43, 40), ds5e, as5ed, g5id, gs5id, as5id],
            &[raw(35, 40), ds5e, as5ed, g5id, gs5id, as5id],
            &[ds4t, ri, ds4t, ri, ds4t, ds4t, ds4t, ds4t, ri, cs4t, ri, cs4t, ri, cs4t, cs4t, cs4t, cs4t, ri],
            &[as5e, d6ed, duty(0), b7i, g7i, a7i, e7i, g7i, d7i, e7i],
        ),
        section(
            &[raw(51, 40), ds5e, ds5i, ds5ed, ds5edd],
            &[raw(51, 40), as4e, ds5i, as5ed, gs5ed, rt],
            &[c4t, ri, c4t, ri, c4t, c4t, c4t, c4t, ri, b3t, ri, b3t, ri, b3t, b3t, b3t, b3t, ri],
            &[b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i],
        ),
        section(
            &[raw(51, 40), ds5edd, raw(48, 40), c5i, c5t, a4t, f4t, c4t, f4t],
            &[raw(57, 40), g5edd, raw(56, 40), f5id, c5t, a4t, f4t, c4t],
            &[as3t, ri, as3t, ri, as3t, as3t, as3t, as3t, ri, gs3t, ri, gs3t, ri, gs3t, gs3t, gs3t, gs3t, ri],
            &[e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i],
        ),
        section(
            &[g4ed, d5ed, raw(56, 80), d4t, e4t],
            &[f4t, g4ed, g4ed, g5edd, d4i, e4id],
            &[g3t, ri, g3t, ri, g3t, g3t, g3t, g3t, ri, g3t, ri, g3t, ri, g3t, g3t, g3t, g3t, g3t, g3t],
            &[b6i, duty(40), e5i, d5i, a4i, e5i, d5i, a4ed, e5i, d5i, g4i],
        ),
        section(
            &[],
            &[],
            &[],
            &[e5i, d5i, g4ed, f4ed, as4ed, d5e],
        ),
        section(
            &[],
            &[],
            &[],
            &[f5i, e5i, c5i, volume!(253 f5i e5i c5i, 250 f5i e5i)],
        ),
    ])
}

pub fn song_13() -> Song {
    song(64, &[
        section(
            &[volume(0), duty(40), a4e, e5i, d5e, g5ed, f5ed, c5e],
            &[volume(251), pitch(5), duty(40), rid, a4e, e5i, d5e, g5ed, f5ed],
            &[a3q, a3q, as3q],
            &[volume(254), duty(34), c3i, duty(35), raw(255, 16)],
        ),
        section(
            &[d5ed, a4e, e5i, d5e, g5ed],
            &[c5e, d5ed, a4e, e5i, d5e, g5ed],
            &[ri, as3t, rt, as3t, d4id, a3q, a3q],
            &[],
        ),
        section(
            &[f5ed, as5e, d6ed, duty(0), b7i, g7i, a7i, e7i],
            &[f5ed, as5e, d6ed, duty(0), pitch(0), b7i, g7i, a7i],
            &[as3q, d4e, f4e, a4q],
            &[],
        ),
        section(
            &[g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i],
            &[e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i],
            &[a4q, g4q, g4q],
            &[],
        ),
        section(
            &[b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i],
            &[b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i],
            &[fs4q, fs4q, f4q],
            &[],
        ),
        section(
            &[g7i, d7i, e7i, b6i, duty(40), e5i, d5i, a4i, e5i, d5i, a4ed],
            &[e7i, g7i, d7i, e7i, b6i, duty(40), pitch(5), e5i, d5i, a4i, e5i, d5i, a4ed],
            &[f4q, as3q, as3q],
            &[],
        ),
        section(
            &[e5i, d5i, g4i, e5i, d5i, g4ed, f4ed, as4ed],
            &[e5i, d5i, g4i, e5i, d5i, g4ed, f4ed],
            &[c4q, e4e, g4e, as3q],
            &[],
        ),
        section(
            &[d5e, f5i, e5i, c5i, volume!(253 f5i e5i c5i, 250 f5i e5i)],
            &[as4ed, d5e, f5i, e5i, c5i, volume!(248 f5i e5i c5i, 245 f5t)],
            &[as3q, c4q, c4q],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn door_unlock_jingle() -> Song {
    song(96, &[
        section(
            &[duty(41), volume!(0 a3x e4x a4x b4x cs5x e5x, 254 a4x e5x a5x b5x cs6x e6x, 252 a3x e4x a4x b4x cs5x e5x, 250 a4x e5x a5x b5x cs6x e6x)],
            &[duty(41), volume(254), rx, a3x, e4x, a4x, b4x, cs5x, e5x, volume!(252 a4x e5x a5x b5x cs6x e6x, 250 a3x e4x a4x b4x cs5x e5x, 249 a4x e5x a5x b5x cs6x)],
            &[a3ed, red],
            &[],
        ),
    ])
}

pub fn song_15() -> Song {
    song(48, &[
        section(
            &[duty(32), volume(0), a4id, raw(40, 30), a4id, c5id, g4i, a4id, raw(40, 30), a4id, c5id, d5i],
            &[duty(32), pitch(8), volume(255), e4id, raw(34, 30), e4id, g4id, d4i, e4id, raw(34, 30), e4id, g4id, a4i],
            &[a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t],
            &[volume(254), duty(34), f2t, duty(35), raw(255, 6)],
        ),
        section(
            &[d5id, e5i, c5id, d5id, e5i, c5id, d5id, e5i, c5id, d5id, c5i, d5id],
            &[a4id, a4i, a4id, a4id, a4i, a4id, a4id, a4i, a4id, a4id, a4i, g4id],
            &[fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, f3t, f4t, g3t, g4t],
            &[],
        ),
        section(
            &[a4id, raw(40, 30), a4id, c5id, g4i, a4id, raw(40, 30), a4id, c5id, d5i],
            &[e4id, raw(34, 30), e4id, g4id, d4i, e4id, raw(34, 30), e4id, g4id, a4i],
            &[a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t],
            &[],
        ),
        section(
            &[d5id, e5i, c5id, d5id, e5i, c5id, d5id, e5i, c5id, d5t, c5t, g4t, a4t, e4t, d4t, c4t, b3t],
            &[a4id, a4i, a4id, a4id, a4i, a4id, a4id, a4i, a4id, a4t, g4t, d4t, e4t, c4t, b3t, a3t, g3t],
            &[fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, f3t, f4t, g3t, g4t],
            &[],
        ),
        section(
            &[a3id, e3id, a4id, e4edd, a3id, e3id, a4id, e4edd],
            &[volume(254), rt, a3id, e3id, a4id, e4edd, a3id, e3id, a4id, e4edd],
            &[a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t],
            &[],
        ),
        section(
            &[a3id, e3id, a4id, e4id, a4i, b4i, c5id, b4id, a4i, d5id, c5id, b4i],
            &[a3id, e3id, a4id, e4i, volume(255), e4i, fs4i, a4id, g4id, a4i, g4id, d4id, g4id],
            &[fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t],
            &[],
        ),
        section(
            &[a4id, e4id, a5id, e5edd, a4id, e4id, a5id, e5edd],
            &[volume(254), a4id, e4id, a5id, e5edd, a4id, e4id, a5id, e5edd],
            &[a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, a3t, rt, a4t, a4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t],
            &[],
        ),
        section(
            &[a4id, e4id, a5id, e5id, a5i, g5i, f5id, g5i, a5i, f5t, g5id, d5id, b4i],
            &[a4id, e4id, a5id, e5i, volume!(255 e5i d5i c5id d5i e5id, 254 f5t g5t d5t b4t d5t b4t g4t d4t)],
            &[fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, fs3t, rt, fs4t, fs4t, f3t, rt, f4t, f4t, f3t, rt, f4t, f4t, g3t, rt, g4t, g4t, g3t, rt, g4t, g4t],
            &[],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_16() -> Song {
    song(48, &[
        section(
            &[raw(128, 4), rest(117), rid, rest(117)],
            &[duty(32), volume(0), as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t],
            &[pitch(5), duty(32), volume(252), ri, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[rid, rest(58)],
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5i, a5t, g5t, a5i, g5t, fs5t, a5id, d5id, c6i],
            &[as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, as5i, a5t, g5t, a5i, g5t, fs5t, a5id, d5id, c6i],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, d3i, rt, d3i, rt, d3id, rt, a3t, rt, d4t, rt, a3t, rt],
        ),
        section(
            &[],
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t],
            &[rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, c6t, as5i, a5t, g5t, c6i, as5t, a5t, d6id, f6id, fs6i],
            &[as5t, c6t, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, as5t, g5t, rt, g5t, a5t, as5t, as5i, a5t, g5t, c6i, as5t, a5t, d6id, f6id, fs6i],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, d3i, rt, d3i, rt, d3id, rt, a3t, rt, d4t, rt, a3t, rt],
        ),
        section(
            &[],
            &[duty(41), g6q, g5i, a5i, b5i, c6i, b5i, g5i, d5i, d6i, b5i, g5i, f6i, d6i],
            &[duty(41), ri, g6q, g5i, a5i, b5i, c6i, b5i, g5i, d5i, d6i, b5i, g5i, f6i],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, g3i, rt, g3i, rt, g3id, rt, b3t, rt, g4t, rt, b3t, rt],
        ),
        section(
            &[],
            &[ds6ed, c6ed, g5q, g6e, f6e, ds6e],
            &[d6i, ds6ed, c6ed, g5q, g6e, f6e, ds6e],
            &[c4i, rt, c4i, rt, c4id, rt, g4t, rt, c5t, rt, g4t, rt, c4i, rt, c4i, rt, c4id, rt, g4t, rt, c5t, rt, g4t, rt],
        ),
        section(
            &[],
            &[d6qdd, a5t, as5t, c6q, as5e, a5e],
            &[d6qdd, a5t, as5t, c6q, as5e, a5e],
            &[d4i, rt, d4i, rt, d4id, rt, a4t, rt, d5t, rt, a4t, rt, d4i, rt, d4i, rt, d4id, rt, a4t, rt, d5t, rt, a4t, rt],
        ),
        section(
            &[],
            &[as5t, a5t, g5qdd, ri, duty(32), d4t, d4t, d4t, d4t, ri, d4id, f4id, fs4i],
            &[as5t, a5t, raw(56, 78), duty(32), a3t, a3t, a3t, a3t, ri, a3id, d4id, d4i],
            &[g3i, rt, g3i, rt, g3id, rt, d3t, rt, f3t, rt, fs3t, rt, g3i, d4x, rx, d4x, rx, d4x, rx, d4x, rx, ri, d4i, rt, f4i, rt, fs4i],
        ),
        section(
            &[],
            &[duty(42), d6q, g5i, a5i, as5i, c6i, d6qdd, c6t, as5t],
            &[duty(42), volume(253), ri, d6q, g5i, a5i, as5i, c6i, d6qdd],
            &[volume(0), g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(56, 16), raw(54, 16)],
            &[c6t, as5t, c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(56, 16), raw(54, 16)],
            &[e3i, rt, e3i, rt, e3id, rt, c4t, rt, e4t, rt, c4t, rt, ds3i, rt, ds4i, rt, ds3t, rt, f3i, rt, f4i, rt, f3t, rt],
        ),
        section(
            &[],
            &[d6q, g5i, a5i, as5i, c6i, d6qdd, c6t, as5t],
            &[d6q, g5i, a5i, as5i, c6i, d6qdd],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(59, 16), raw(64, 16)],
            &[c6t, as5t, c6q, g5i, a5i, as5i, c6i, as5ed, c6t, as5t, raw(58, 16), raw(59, 16), raw(64, 16)],
            &[e3i, rt, e3i, rt, e3id, rt, c4t, rt, e4t, rt, c4t, rt, ds3i, rt, ds4i, rt, ds3t, rt, f3i, rt, f4i, rt, f3t, rt],
        ),
        section(
            &[],
            &[duty(40), raw(66, 60), as5i, d6i, raw(64, 60), a5e, c6e],
            &[volume(0), pitch(0), duty(40), ri, g5ed, g5i, g5i, raw(58, 60), f5e, a5e],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[raw(66, 60), as5i, d6i, raw(64, 60), raw(58, 16), raw(59, 16), raw(64, 16)],
            &[re, g5ed, g5i, g5i, raw(58, 60), raw(54, 16), raw(56, 16), raw(58, 16)],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[as5t, a5t, g5qdd, ri, duty(41), volume(253), d4i, g4i, a4i, as4i, c5i, d5i, f5i],
            &[pitch(5), g5q, volume(252), as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t],
            &[g3i, ri, g3i, ri, g3i, ri, g3i, ri, f3i, ri, f3i, ri, f3i, ri, f3i, ri],
        ),
        section(
            &[],
            &[g5ed, raw(40, 60), volume(0), duty(32), g5e, fs5e, e5e, fs5e],
            &[as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, as5t, a5t, g5t, d5t, pitch(0), volume(0), d5e, d5e, d5e, d5e],
            &[ds3i, ri, ds3i, ri, ds3i, ri, ds3i, ri, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt, d3t, rt],
        ),
        section(
            &[],
            &[g5q, as4i, ds5i, f5i, g5i, a5q, raw(54, 16), raw(56, 16), raw(58, 16)],
            &[ds5q, pitch(5), g4i, as4i, as4i, as4i, c5q, raw(48, 16), raw(48, 16), raw(48, 16)],
            &[ds3i, rt, ds3i, rt, ds3id, rt, as3t, rt, ds4t, rt, as3t, rt, f3i, rt, f3i, rt, f3id, rt, c4t, rt, f4t, rt, c4t, rt],
        ),
        section(
            &[],
            &[b5h, d3i, rt, e3i, rt, f3t, rt, g3i, rt, a3i, rt, c4t, rt],
            &[d5h, g3i, rt, a3i, rt, as3t, rt, c4i, rt, d4i, rt, f4t, rt],
            &[g3i, rt, g3i, rt, g3id, rt, d4t, rt, g4t, rt, d4t, rt, g3i, rt, a3i, rt, as3t, rt, c4i, rt, d4i, rt, f4t, rt],
        ),
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_17() -> Song {
    song(96, &[
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_18() -> Song {
    song(96, &[
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

/// All songs by ROM index.
pub fn get(i: usize) -> Option<Song> {
    Some(match i {
        0 => area_north(),
        1 => area_west(),
        2 => home_theme(),
        3 => area_east(),
        4 => area_central(),
        5 => song_05(),
        6 => song_06(),
        7 => song_07(),
        8 => death_jingle(),
        9 => title_theme(),
        10 => ending_theme(),
        11 => song_11(),
        12 => song_12(),
        13 => song_13(),
        14 => door_unlock_jingle(),
        15 => song_15(),
        16 => song_16(),
        17 => song_17(),
        18 => song_18(),
        _ => return None,
    })
}

// ===== sound effects (one pulse2 stream each) =====

pub fn sfx_00() -> Vec<Tok> {
    line(64, &[c10x, raw(162, 124), raw(162, 125), rest(58)])
}

pub fn sfx_01() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), f7x, b6x, rt, c8x, raw(101, 1)])
}

pub fn sfx_02() -> Vec<Tok> {
    line(16, &[duty(4), volume(0), c5x, b4x, as4x, gs4x, fs4x, ds4x, c4x, fs3x, e3x])
}

pub fn sfx_char_select_open() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), raw(69, 1), as5x, g6x, ds6x, c7x, as6x])
}

pub fn sfx_char_select_close() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), a6x, b6x, cs7x, fs7x, e7x, a7x, e7x, fs7x])
}

pub fn sfx_05() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), f3x, a7x, fs3x, gs7x, g3x, g7x, gs3x, fs7x])
}

pub fn sfx_blocked() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), g2x, d3x, as2x, g3x, d4x, a3x, as3x])
}

pub fn sfx_07() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), e8x, ds8x, b2x, a2x, as2x])
}

pub fn sfx_08() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), d7x, fs7x, as7x, d8x, fs8x])
}

pub fn sfx_09() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), c5x, ds5x, fs5x, a5x, c6x, ds6x])
}

pub fn sfx_damage_bounce() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), c6t, rx, f6x, fs6x, f6x])
}

pub fn sfx_11() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), d5x, e5x, f5x, g5x, a5x, b5x, b4x, a4x, g4x, f4x, e4x, d4x, c4x])
}

pub fn sfx_cursor_select() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), c7x])
}

pub fn sfx_13() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), as9x, as8x])
}

pub fn sfx_14() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), g6x, c6x, g5x, c5x, g4x, c4x, g3x, c3x, g2x, c2x])
}

pub fn sfx_15() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), b7x, g7x, e7x, c7x, b6x, g6x, e6x, c6x])
}

pub fn sfx_16() -> Vec<Tok> {
    line(16, &[duty(37), volume(0), g4x, e5x, c6x, c5x, c7ed, rqd])
}

pub fn sfx_magic_pickup() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 gs6x d6x cs6x b5x c6x d6x fs6x b6x d7x fs7x b7x, 252 gs6x d6x cs6x b5x c6x d6x fs6x b6x d7x fs7x b7x a7x b7x)])
}

pub fn sfx_18() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 d5x fs5x a5x b5x fs5x a5x b5x d6x, 255 a5x b5x d6x g6x, 254 g5x b5x d6x f6x, 253 fs5x a5x b5x d6x, 252 g5x b5x d6x f6x, 251 f5x a5x d6x g6x, 250 f5x a5x d6x g6x, 249 f5x a5x d6x g6x, 248 f5x a5x d6x g6x, 247 f5x a5x d6x g6x, 246 f5x a5x d6x g6x, 245 f5x a5x d6x g6x)])
}

pub fn sfx_got_item() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 d6x fs6x a6x b6x b4x, 255 d5x fs5x d5x fs5x a5x b5x d6x b4x, 254 d5x fs5x d6x fs6x a6x as6x b6x)])
}

pub fn sfx_20() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 b2x g2x e2x c2x, 254 fs3t, 253 d3t cs3t b2t as2t a2t gs2t g2t, 252 fs3t, 251 d3t cs3t b2t as2t a2t gs2t g2t)])
}

pub fn sfx_key_pickup() -> Vec<Tok> {
    line(32, &[duty(36), volume!(0 fs6x b5x d6x a6x a7i, 252 fs6x b5x d6x a6x e7x a7e)])
}

pub fn sfx_22() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), g6x, d6x])
}

pub fn sfx_23() -> Vec<Tok> {
    line(32, &[duty(36), volume(0), g6x, d6x, f6x, a6x, b6x, d7x, g7x])
}

pub fn sfx_24() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 b7x as7x g7x e7x c7x g5x c5x b5x b7x g7x c4x d4x g3x e3x d3x g2x, 254 g5x c5x b5x b7x g7x c4x d4x g3x e3x d3x g2x, 252 g5x c5x b5x b7x g7x c4x d4x g3x e3x d3x g2x, 250 g5x c5x b5x b7x g7x c4x d4x g3x e3x d3x g2x, 248 g5x c5x b5x b7x g7x c4x d4x g3x e3x d3x g2x)])
}

pub fn sfx_fire() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 b5x as5x b5x a5x, 255 as5x gs5x a5x g5x, 254 gs5x fs5x g5x f5x, 253 fs5x e5x f5x ds5x, 252 e5x d5x ds5x cs5x, 251 d5x c5x cs5x b4x, 250 c5x as4x b4x a4x, 249 as4x gs4x a4x g4x, 248 gs4x fs4x g4x f4x, 247 fs4x e4x f4x, 246 ds4x e4x c4x ds4x c4x)])
}

pub fn sfx_low_magic() -> Vec<Tok> {
    line(64, &[duty(36), volume(0), e4x, cs5x, e4x, cs5x, e4x, cs5x, e4x, cs5x])
}

pub fn sfx_jump() -> Vec<Tok> {
    line(32, &[duty(36), volume!(0 fs4x g4x gs4x a4x, 255 as4x b4x c5x cs5x)])
}

pub fn sfx_password_error() -> Vec<Tok> {
    line(64, &[duty(36), volume(0), c6x, d6x, cs6x, ds6x, d6x, e6x, ds6x, f6x, e6x, fs6x, ds6x, f6x, d6x, e6x, cs6x, ds6x, c6x, d6x])
}

pub fn sfx_inventory_full() -> Vec<Tok> {
    line(32, &[duty(36), volume(0), ds5x, f5x, d5x, e5x, cs5x, ds5x, c5x, d5x, b4x, cs5x, as4x, c5x, cs5x, d5x, ds5x, e5x])
}

pub fn sfx_health_pickup() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), g6x, g5x, d5x, c5x, re, b6x, c5x, e5x, g5x, b5x])
}

pub fn sfx_31() -> Vec<Tok> {
    line(64, &[duty(36), volume!(0 e7x b6x, 254 e7x b6x, 252 e7x b6x, 250 e7x b6x, 248 e7x b6x, 246 e7x b6x, 244 e7x b6x)])
}

pub fn sfx_32() -> Vec<Tok> {
    line(16, &[duty(52), volume!(0 a3x as3x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x, 255 c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x c4x b3x, 254 c4x as3t b3t, 253 a3t as3t, 252 g3td gs3td, 251 f3t fs3t e3x), pitch(8), volume(250), e3x, pitch(16), e3x, pitch(32), volume(249), e3x, pitch!(40 e3x, 50 e3x, 0 f3x e3x ds3x d3x cs3x)])
}

pub fn sfx_hurt() -> Vec<Tok> {
    line(16, &[volume!(0 a4x a3x a2x a3x f3x b4x b4x d4x f5x b5x d6x f6x b5x d6x a6x, 253 f7x b6x d7x a7x, 251 f6x b5x d6x a6x, 249 f7x b6x d7x a7x f6x b6x d7x a7x e8x)])
}

pub fn sfx_fire_char0() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 c3x ds3x f3x fs3x g3x g4x d3x c3x b2x, 252 c3x ds3x f3x fs3x g3x g4x d3x c3x b2x, 249 c3x ds3x f3x fs3x g3x g4x d3x c3x b2x)])
}

pub fn sfx_fire_char1() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 b4x fs5x a5x a5x e5x fs5x a5x b5x fs5x fs5x a5x a5x fs5x, 255 f5x e5x, 254 ds5x, 253 g5x, 252 gs5x d5x d5x fs5x b5x b5x b5x a5x b5x, 250 e5x fs5x a5x b5x b5x b5x)])
}

pub fn sfx_fire_char2() -> Vec<Tok> {
    line(16, &[duty(36), volume(0), d6x, f6x, b6x, d7x, a7x, e8x])
}

pub fn sfx_fire_char3() -> Vec<Tok> {
    line(32, &[duty(36), volume!(0 b7x g6x g5x g4x g3x, 252 b7x g6x g5x g4x g3x, 249 b7x g6x g5x g4x g3x)])
}

pub fn sfx_fire_char4() -> Vec<Tok> {
    line(16, &[duty(36), volume!(0 b5x as5x b5x a5x, 255 as5x gs5x a5x g5x, 254 gs5x fs5x g5x f5x, 253 fs5x e5x f5x ds5x, 252 e5x d5x ds5x cs5x, 251 d5x c5x cs5x b4x, 250 c5x as4x b4x a4x, 249 as4x gs4x a4x g4x, 248 gs4x fs4x g4x f4x, 247 fs4x e4x f4x, 246 ds4x e4x c4x ds4x c4x)])
}

/// All sound effects by ROM index.
pub fn sfx(i: usize) -> Option<Vec<Tok>> {
    Some(match i {
        0 => sfx_00(),
        1 => sfx_01(),
        2 => sfx_02(),
        3 => sfx_char_select_open(),
        4 => sfx_char_select_close(),
        5 => sfx_05(),
        6 => sfx_blocked(),
        7 => sfx_07(),
        8 => sfx_08(),
        9 => sfx_09(),
        10 => sfx_damage_bounce(),
        11 => sfx_11(),
        12 => sfx_cursor_select(),
        13 => sfx_13(),
        14 => sfx_14(),
        15 => sfx_15(),
        16 => sfx_16(),
        17 => sfx_magic_pickup(),
        18 => sfx_18(),
        19 => sfx_got_item(),
        20 => sfx_20(),
        21 => sfx_key_pickup(),
        22 => sfx_22(),
        23 => sfx_23(),
        24 => sfx_24(),
        25 => sfx_fire(),
        26 => sfx_low_magic(),
        27 => sfx_jump(),
        28 => sfx_password_error(),
        29 => sfx_inventory_full(),
        30 => sfx_health_pickup(),
        31 => sfx_31(),
        32 => sfx_32(),
        33 => sfx_hurt(),
        34 => sfx_fire_char0(),
        35 => sfx_fire_char1(),
        36 => sfx_fire_char2(),
        37 => sfx_fire_char3(),
        38 => sfx_fire_char4(),
        _ => return None,
    })
}
