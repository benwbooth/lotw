//! Legacy of the Wizard songs + SFX as the music DSL — generated from the ROM
//! by `gen_music` (deterministic, byte-exact). Refine the notation freely; it
//! must still assemble to the same bytes (see `tests/audio_dsl.rs`).

use lotw_music::music::*;

pub fn area_north() -> Song {
    song(24, &[
        section(
            &[duty(11), volume(255), c3hd, c4hd, g4e, f4e, g4e, c4hd, c5qd, g4hd],
            &[duty(11), pitch(4), volume(255), c3hd, c4hd, c4qd, g3hd, c5qd, as4hd],
            &[c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i],
            &[duty(2), volume(254), hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq, hite, rq],
        ),
        section(
            &[d4hd, f4qd, c4qd, ds4qd, f4qd, g4w, rqd, raw(41, 1), raw(42, 1), raw(43, 1), raw(44, 1), raw(48, 1), raw(49, 1), raw(50, 1), raw(51, 1), raw(52, 1), raw(54, 1), raw(55, 1), raw(56, 1)],
            &[as3hd, c4qd, f3qd, gs3qd, as3qd, c4w, rqd, raw(33, 1), raw(34, 1), raw(35, 1), raw(36, 1), raw(38, 1), raw(39, 1), raw(40, 1), raw(41, 1), raw(42, 1), raw(43, 1), raw(44, 1), raw(48, 1)],
            &[as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, gs3i, gs3i, gs4i, gs3i, c4i, gs3i, ds4i, gs3i, f4i, gs3i, gs4i, gs3i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i],
            &[hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq, hite, rq, hite, rq, hite, rq],
        ),
        section(
            &[volume(0), g5w, rh, volume(0), duty(16), c4e, d4e, ds4e, f4e, g4e, c4q, d4e, ds4e, f4e, g4e, c4e, d4e, ds4e, f4e, d4e, c4e, as3q],
            &[volume(0), c5w, rh, volume(252), duty(16), c7e, d7e, ds7e, f7e, g7e, c7q, d7e, ds7e, f7e, g7e, c7e, d7e, ds7e, f7e, d7e, c7e, as6q],
            &[c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, as3i, as3i],
            &[hite, rq, hite, rq, hite, rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite],
        ),
        section(
            &[d4e, ds4e, f4e, ds4e, f4e, duty(32), volume(0), c5e, d5e, ds5e, f5e, g5e, c5q, d5e, ds5e, f5e, g5e, c5e, d5e, ds5e, f5e, d5e, c5e, as4q, d5e, ds5e, f5e, ds5e, d5e, duty(27), c5e, gs4e, ds4e],
            &[d7e, ds7e, f7e, ds7e, f7e, duty(32), pitch(8), re, c5e, d5e, ds5e, f5e, g5e, c5q, d5e, ds5e, f5e, g5e, c5e, d5e, ds5e, f5e, d5e, c5e, as4q, d5e, ds5e, f5e, ds5e, duty(27), volume(0), c5e, gs4e, ds4e],
            &[as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, gs3i, gs3i, gs4i, gs3i, cs4i, gs3i],
            &[rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq],
        ),
        section(
            &[gs4e, ds4e, c4e, as4e, f4e, d4e, f4e, d4e, as3e, c5e, gs4e, ds4e, gs4e, ds4e, c4e, as3e, d4e, f4e, d4e, f4e, as4e, c5e, g4e, f4e, e4w + e4e],
            &[gs4e, ds4e, c4e, as4e, f4e, d4e, f4e, d4e, as3e, c5e, gs4e, ds4e, gs4e, ds4e, c4e, as3e, d4e, f4e, d4e, f4e, as4e, c5e, g4e, f4e, e4w + e4e],
            &[ds4i, gs3i, f4i, gs3i, gs4i, gs3i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, gs3i, gs3i, gs4i, gs3i, cs4i, gs3i, ds4i, gs3i, f4i, gs3i, gs4i, gs3i, as3i, as3i, as4i, as3i, ds4i, as3i, f4i, as3i, gs4i, as3i, as4i, as3i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i, c4i, c4i, c5i, c4i, f4i, c4i, g4i, c4i, as4i, c4i, c5i, c4i],
            &[hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, hite, rq, volume(0), duty(3), hite, rq, volume(254), duty(2), hite, rq, volume(0), duty(3), hite, rq, volume(0), hite, rq, volume(0), hite, rq],
        ),
    ])
}

pub fn area_west() -> Song {
    song(24, &[
        section(
            &[duty(27), volume(0), c4qd, c5qd, d5e, ds5e, d5qd, as4qd, g4q, c5w, c5qd, b4qd, g4q],
            &[duty(27), volume(0), pitch!(8 c4qd, 0 ds4qd f4e g4e f4qd d4qd as3q g4w d4qd), d4h + d4e],
            &[c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, g3i, ri, g3i, ri, g4i, ri, g3i, ri, g3i, ri, g3i, ri, g4i, ri, g3i, ri],
            &[duty(34), volume(254), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hiti, hiti, hite, duty(35), hite, hite, hite],
        ),
        section(
            &[c5qd, ds5qd, f5e, g5e, f5qd, d5qd, as5q, g5w, g5h, volume(254), d5q, f5q],
            &[g4qd, c5qd, d5e, ds5e, d5qd, as4qd, f5q, as4w, b4h, pitch(8), volume(254), d5q, f5q],
            &[c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, ds4i, ri, ds4i, ri, ds5i, ri, ds4i, ri, ds4i, ri, ds4i, ri, ds5i, ri, ds4i, ri, d4i, ri, d4i, ri, d5i, ri, d4i, ri, d4i, ri, d4i, ri, d5i, ri, d4i, ri],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hiti, hiti, hite, duty(35), hite, hiti, hiti, hite],
        ),
        section(
            &[f5e, ds5e, d5e, ds5q, c5h + c5e, c6q, g5q, ds5q, f5e, ds5e, d5e, ds5q, c5h + c5e, c6q, g5q, ds5q],
            &[f5e, ds5e, d5e, ds5q, c5hd, volume!(251 c6q g5q ds5e, 254 f5e ds5e d5e ds5q c5hd, 251 c6q g5q ds5e)],
            &[c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[volume!(0 d5hd d5e ds5e f5h ds5q f5q, 254 g5w), rh, d5q, f5q],
            &[volume!(255 f4w g4w c5hd d5q b4h, 254 d5q f5q)],
            &[as3i, ri, as3i, ri, as4i, ri, as3i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, b3i, ri, b3i, ri, b4i, ri, b3i, ri, b3i, ri, b3i, ri, b4i, ri, b3i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, g3i, ri, g3i, ri, g3i, g4i, g3i, ri, g4i, ri, g3i, ri, a3i, ri, b3i, ri],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, hite, hite, hite, hite, hite, hite],
        ),
        section(
            &[f5e, ds5e, d5e, ds5q, c5h + c5e, c6q, g5q, ds5q, f5e, ds5e, d5e, ds5q, c5h + c5e, c6q, g5q, ds5q],
            &[f5e, ds5e, d5e, ds5q, c5hd, volume!(251 c6q g5q ds5e, 254 f5e ds5e d5e ds5q c5hd, 251 c6q g5q ds5e)],
            &[c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri, gs3i, ri, gs3i, ri, gs4i, ri, gs3i, ri],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[volume(0), d5hd, d5e, ds5e, f5h, ds5q3, d5q3, as4q3, c5hdd, c6w + c6e],
            &[pitch(0), volume(255), f4w, g4w, g4hdd, g5w + g5e],
            &[as3i, ri, as3i, ri, as4i, ri, as3i, ri, as3i, ri, as3i, ri, as4i, ri, as3i, ri, g3i, ri, g3i, ri, g4i, ri, g3i, ri, g3i, ri, g3i, ri, g4i, ri, g3i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri, c4i, ri, c4i, ri, c5i, ri, c4i, ri],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, duty(35), hite, hite, hite, hite],
        ),
    ])
}

pub fn song_02() -> Song {
    song(8, &[
        section(
            &[duty(32), volume(255), c5w, d5h, ds5h, d5w, as4w],
            &[duty(32), pitch(8), volume(255), g4w, as4h, c5h, as4w, f4w],
            &[c4q, c4q, c5q, c4q, rq, c4q, rq, c5q, as3q, as3q, as4q, as3q, rq, as3q, rq, as4q],
            &[volume(254), duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[gs4w, as4h, c5h, b4h, c5q, d5h, g4hd],
            &[ds4w, f4h, gs4h, g4h, g4q, g4h, volume!(0 g4i, 252 fs4i, 250 f4i, 248 e4i), rq, volume!(0 ds4i, 252 d4i, 250 cs4i, 248 cs4i)],
            &[gs3q, gs3q, gs4q, gs3q, rq, gs3q, rq, gs4q, g3q, g3q, g4q, g3q, rq, g3q, a3q, b3q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, duty(35), hitq, hitq, hitq],
        ),
        section(
            &[c5w, d5h, ds5h, d5w, as4w],
            &[volume(255), g4w, as4h, c5h, as4w, f4w],
            &[c4q, c4q, c5q, c4q, rq, c4q, rq, c5q, as3q, as3q, as4q, as3q, rq, as3q, rq, as4q],
            &[duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[c5w, d5h, c5h, b4h, d5q, g5w, rq],
            &[gs4w, as4h, gs4h, g4hd, d5h, volume!(0 g4i, 252 fs4i, 250 f4i, 248 e4i, 0 ds4i, 252 d4i, 250 cs4i, 248 c4i, 0 g4i, 252 fs4i, 250 f4i, 248 e4i)],
            &[gs3q, gs3q, gs4q, gs3q, rq, gs3q, rq, gs4q, g3q, g3q, g4q, g3q, rq, g3q, g4q, g3q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, duty(35), hitq, hitq, hitq],
        ),
        section(
            &[duty(32), volume(0), g5w + g5w, f5w, as5w],
            &[volume(253), g5q, ds5q, as4q, g5q, ds5q, as4q, g5q, ds5q, as4q, f5q, d5q, as4q, f5q, d5q, as4q, f5q],
            &[ds4q, ds4q, ds5q, ds4q, rq, ds4q, rq, ds5q, d4q, d4q, rq, d4q, rq, d4q, rq, d5q],
            &[duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[ds5w + ds5w, d5w, as4w],
            &[ds5q, as4q, g4q, ds5q, as4q, g4q, ds5q, as4q, g4q, d5q, as4q, f4q, d5q, as4q, f4q, d5q],
            &[c4q, c4q, c5q, c4q, rq, c4q, rq, c5q, as3q, as3q, as4q, as3q, rq, as3q, as4q, as3q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[c5w, d5h, c5h, as4hd, ds5hd, g5h],
            &[c5q, gs4q, ds4q, c5q, gs4q, ds4q, c5q, gs4q, ds4q, as4q, g4q, ds4q, as4q, g4q, ds4q, as4q],
            &[gs3q, gs3q, gs4q, gs3q, rq, gs3q, rq, gs4q, g3q, g3q, g4q, g3q, rq, g3q, rq, g4q],
            &[duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[g5hd, f5w + f5hd, as4h, ds5h, f5h],
            &[as4q, f4q, d4q, as4q, f4q, d4q, as4q, f4q, d4q, as4q, f4q, d4q, as4q, f4q, as4q, f4q],
            &[as3q, as3q, as4q, as3q, rq, as3q, rq, as4q, as3q, as3q, as4q, as3q, rq, c4q, rq, d4q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hite, hite, hite, hite],
        ),
        section(
            &[duty(32), volume(0), g5w + g5w, f5w, as5w],
            &[g5q, ds5q, as4q, g5q, ds5q, as4q, g5q, ds5q, as4q, f5q, d5q, as4q, f5q, d5q, as4q, f5q],
            &[ds4q, ds4q, ds5q, ds4q, rq, ds4q, rq, ds5q, d4q, d4q, rq, d4q, rq, d4q, rq, d5q],
            &[duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[ds5w + ds5w, d5w, g5w],
            &[ds5q, as4q, g4q, ds5q, as4q, g4q, ds5q, as4q, g4q, d5q, as4q, f4q, d5q, as4q, f4q, d5q],
            &[c4q, c4q, c5q, c4q, rq, c4q, rq, c5q, as3q, as3q, as4q, as3q, rq, as3q, as4q, as3q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[c5w, d5h, c5h, ds5h, f5h, g5h, ds5h],
            &[c5q, gs4q, ds4q, c5q, gs4q, ds4q, c5q, gs4q, ds4q, as4q, g4q, ds4q, as4q, g4q, ds4q, as4q],
            &[gs3q, gs3q, gs4q, gs3q, rq, gs3q, rq, gs4q, g3q, g3q, g4q, g3q, rq, g3q, rq, g4q],
            &[duty(34), hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[d5w + d5h, c5h, b4w + b4q, duty(32), g4q, a4q, b4q],
            &[b4q, g4q, d4q, b4q, g4q, d4q, b4q, g4q, d4q, b4q, g4q, d4q, volume!(0 g4i, 252 fs4i, 0 g4i, 252 fs4i, 0 ds4i, 252 d4i, 250 cs4i, 248 cs4i, 0 ds4i, 252 d4i, 0 ds4i, 252 d4i, 0 cs4i, 252 c4i, 250 b3i, 248 as3i)],
            &[g3q, g3q, g4q, g3q, rq, g3q, rq, g4q, g3q, g3q, g4q, g3q, rq, g3q, a3q, b3q],
            &[hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, duty(35), hitq, hitq, hitq, hite, hite, hitq, hite, hite, hitq],
        ),
    ])
}

pub fn area_east() -> Song {
    song(6, &[
        section(
            &[duty(37), volume(0), d5q, rq, d5q, rq, g5h, d5q, f5w + f5h, rq, volume(253), duty(32), d7i3, e7i3, d7i3, e7i3, ri3, ri3, d7i3, e7i3, d7i3, e7i3, ri3, ri3],
            &[duty(37), volume(253), g3q, rq, g3q, rq, g4q, rq, g3q, g3q, rq, c4q, cs4q, d4q, rq, f4q, fs4q, g4q],
            &[g3q, rq, g3q, rq, g4q, rq, g3q, g3q, rq, c4q, cs4q, d4q, rq, f4q, fs4q, g4q],
            &[duty(34), volume(254), hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, hitq, hitq],
        ),
        section(
            &[volume(0), duty(37), d5q, rq, d5q, rq, g5h, d5q, f5w + f5h, rq, volume(255), duty(32), d7i3, e7i3, d7i3, e7i3, ri3, ri3, d7i3, e7i3, d7i3, e7i3, ri3, ri3],
            &[g3q, rq, g3q, rq, g4q, rq, g3q, g3q, rq, c4q, cs4q, d4q, rq, f4q, fs4q, g4q],
            &[g3q, rq, g3q, rq, g4q, rq, g3q, g3q, rq, c4q, cs4q, d4q, rq, f4q, fs4q, g4q],
            &[duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, hitq, hitq],
        ),
        section(
            &[volume(0), duty(37), g5hd, d5hd, g5h, f5hd, c5hd, f5h],
            &[d5hd, g4hd, d5h, c5hd, f4hd, c5h],
            &[g3q, rq, g3q, rq, g4q, g3q, rq, rq, f3q, rq, f3q, rq, f4q, f3q, rq, rq],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[ds5hd, as4hd, ds5h, f5hd, c5hd, volume(253), duty(32), d7i3, e7i3, d7i3, e7i3, ri3, ri3, d7i3, e7i3, d7i3, e7i3, ri3, ri3],
            &[as4hd, ds4hd, as4h, c5hd, f4w, rq],
            &[ds3q, rq, ds3q, rq, ds4q, ds3q, rq, rq, f3q, rq, f3q, rq, f4q, f3q, rq, rq],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[volume(0), duty(37), g5hd, d5hd, g5h, f5hd, c5hd, f5h],
            &[pitch(0), d5hd, g4hd, d5h, c5hd, f4hd, c5h],
            &[g3q, rq, g3q, rq, g4q, g3q, rq, rq, f3q, rq, f3q, rq, f4q, f3q, rq, rq],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[ds5hd, as4hd, ds5h, d5hd, a4hd, f4h],
            &[as4hd, ds4hd, as4h, a4hd, d4hd, a3h],
            &[ds3q, rq, ds3q, rq, ds4q, ds3q, rq, rq, f3q, rq, f3q, rq, f4q, f3q, f3q, f4q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, duty(35), hite, hite, hite, hite, hitq, hitq, hitq, hitq, hitq, hitq],
        ),
        section(
            &[duty(43), g4w + g4w + g4w, d4h, g4h],
            &[duty(43), as3w + as3w + as3w, as3h, as3h],
            &[g3q, rq, g3q, g3q, g3q, rq, g3q, g3q, g3q, rq, g3q, g3q, g3q, rq, g3q, g3q],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[a4w + a4w, f4w, c4w],
            &[c4w + c4w, a3w, f3w],
            &[f3q, rq, f3q, f3q, f3q, rq, f3q, f3q, f3q, rq, f3q, f3q, f3q, rq, f3q, f3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[d4w + d4w + d4w + d4h],
            &[g3w + g3w + g3w + g3h],
            &[ds3q, rq, ds3q, ds3q, ds3q, rq, ds3q, ds3q, ds3q, rq, ds3q, ds3q, ds3q, rq],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[d4w + d4w + d4h, d5w + d5w],
            &[fs3w + fs3w + fs3h, fs4w + fs4w],
            &[ds3q, ds3q, d3q, rq, d3q, d3q, d3q, rq, d3q, d3q, d3q, rq, d3q, d3q, d3q, rq, d3q, d3q],
            &[hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[g5w + g5w + g5w, a5h, as5h],
            &[as4w + as4w + as4w, f4h, g4h],
            &[g3q, rq, g3q, g3q, g3q, rq, g3q, g3q, g3q, rq, g3q, g3q, g3q, rq, g3q, g3q],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[c6w + c6w, as5w, c6w],
            &[a4w + a4w, f4w, a4w],
            &[f3q, rq, f3q, f3q, f3q, rq, f3q, f3q, f3q, rq, f3q, f3q, f3q, rq, f3q, f3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[d6w + d6w + d6w + d6w],
            &[as4w + as4w + as4w + as4w],
            &[as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[d6w + d6w, d5w + d5w],
            &[volume(254), b2q, d3q, g3q, d3q, g3q, b3q, d4q, b3q, d4q, g4q, b4q, g4q, b4q, d5q, g5q, b5q],
            &[b3q, rq, b3q, b3q, b3q, rq, b3q, b3q, b3q, rq, b3q, b3q, b3q, rq, b3q, b3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[duty(37), ds5w, c5h, g4w, ds5h, f5h, ds5h],
            &[g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q],
            &[c4q, rq, c4q, c4q, c4q, rq, c4q, c4q, c4q, rq, c4q, c4q, c4q, rq, c4q, c4q],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[d5w, as4h, g4w + g4w, rh],
            &[g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q],
            &[as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[a4w, fs4h, d4w, a4h, g4h, a4h],
            &[fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q],
            &[a3q, rq, a3q, a3q, a3q, rq, a3q, a3q, a3q, rq, a3q, a3q, a3q, rq, a3q, a3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[as4w, c5h, d5w + d5w + d5h],
            &[g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q, g4q, b4q, d5q, b4q, g4q, b4q, d5q, b4q],
            &[as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, b3q, rq, b3q, b3q, b3q, rq, b3q, b3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, duty(35), hitq, duty(34), hitq],
        ),
        section(
            &[ds5w, c5h, g4w, ds5h, f5h, ds5h],
            &[g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q, g4q, c5q, ds5q, c5q],
            &[c4q, rq, c4q, c4q, c4q, rq, c4q, c4q, c4q, rq, c4q, c4q, c4q, rq, c4q, c4q],
            &[duty(34), hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[d5w, as4h, g4w + g4w, rh],
            &[g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q, g4q, as4q, d5q, as4q],
            &[as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q, as3q, rq, as3q, as3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[a4w, fs4h, d4w, a4h, as4h, c5h],
            &[fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q, fs4q, a4q, d5q, a4q],
            &[a3q, rq, a3q, a3q, a3q, rq, a3q, a3q, a3q, rq, a3q, a3q, a3q, rq, a3q, a3q],
            &[hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq, hitq, hitq, hitq, hitq, duty(35), hitq, duty(34), hitq, hitq, hitq],
        ),
        section(
            &[a4hd, g4w + g4q, rw, volume(253), duty(32), d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3, d7i3, e7i3],
            &[g4q, d4q, as3q, g3q, rw, rq, pitch(10), d2e, re, f2e, re, g2e, re, a2e, re, c3e, re, f3e, re, a3e, re],
            &[g3q, g3q, g3q, g3q, rq, g3q, g3q, g3q, rq, d3q, f3q, g3q, a3q, c4q, f4q, a4q],
            &[duty(35), hitq, hitq, hitq, hitq, rq, hitq, hitq, hitq, rw, duty(34), hitq, rhd],
        ),
    ])
}

pub fn area_central() -> Song {
    song(16, &[
        section(
            &[duty(37), volume!(250 d3e d4e d3e, 251 ds3e ds4e ds3e, 252 f3e f4e f3e, 253 ds3e ds4e ds3e, 254 d3e d4e d3e, 255 ds3e ds4e ds3e, 0 f3e f4e f3e ds3e ds4e ds3e d3e d4e d3e ds3e ds4e ds3e f3e f4e)],
            &[duty(37), pitch(8), volume!(250 d3e d4e d3e, 251 ds3e ds4e ds3e, 252 f3e f4e f3e, 253 ds3e ds4e ds3e, 254 d3e d4e d3e, 255 ds3e ds4e ds3e, 0 f3e f4e f3e ds3e ds4e ds3e d3e d4e d3e ds3e ds4e ds3e f3e f4e)],
            &[g3e, g4e, g3e, gs3e, gs4e, gs3e, as3e, as4e, as3e, gs3e, gs4e, gs3e, g3e, g4e, g3e, gs3e, gs4e, gs3e, as3e, as4e, as3e, gs3e, gs4e, gs3e, g3e, g4e, g3e, gs3e, gs4e, gs3e, as3e, as4e],
            &[volume(254), duty(0), sweep(23), hite, duty(34), volume(246), hite, hite, volume(247), hite, hite, volume(248), hite, hite, volume(249), hite, hite, volume(250), hite, hite, volume(254), duty(35), hiti, hiti, hite, duty(34), volume(250), hite, hite, volume(251), hite, hite, volume(252), hite, hite, volume(253), hite, hite, duty(35), volume(254), hiti, hiti, hiti, hiti, hiti, hiti, hite, duty(34), volume(252), hite, hite, hite, hite, duty(35), volume(254), hiti, hiti, duty(34), volume(252), hite, hite],
        ),
        section(
            &[f3e, ds3e, ds4e, ds3e, d3e, d4e, d3e, ds3e, ds4e, ds3e, f3e, f4e, f3e, g3e, g4e, g3e, volume(254), c3e, g3e, c4e, d4e, c4e, g3e, ds3e, as3e, ds4e, f4e, ds4e, as3e, f3e, c4e, f4e, g4e],
            &[f3e, ds3e, ds4e, ds3e, d3e, d4e, d3e, ds3e, ds4e, ds3e, f3e, f4e, f3e, g3e, g4e, g3e, g3h, g3e, g3e, as3h, as3e, as3e, c4h],
            &[as3e, gs3e, gs4e, gs3e, g3e, g4e, g3e, gs3e, gs4e, gs3e, as3e, as4e, as3e, b3e, b4e, b3e, c4q, rq, c4i, ri, c4i, ri, ds4q, rq, ds4i, ri, ds4i, ri, f4q, rq],
            &[hite, hite, hite, duty(35), volume(254), hiti, hiti, hite, duty(34), volume(252), hite, hite, volume(253), hite, hite, volume(254), hite, hite, volume(255), hite, hite, duty(35), volume(254), hiti, hiti, hiti, hiti, hiti, hiti, hite, re, duty(34), volume(250), hite, re, hite, hite, duty(35), volume(254), hite, re, duty(34), volume(250), hite, re, hite, hite, duty(35), volume(254), hite, re, duty(34), volume(250), hite, re],
        ),
        section(
            &[f4e, c4e, g3e, d4e, g4e, g3e, a3e, b3e, c4e, g4e, c5e, d5e, c5e, g4e, ds4e, as4e, ds5e, f5e, ds5e, as4e, f4e, c5e, f5e, g5e, f5e, c5e, g4e, d5e, g5e, d5e, c5e, g4e],
            &[c4e, c4e, d4h, d4e, d4e, g3h, g3e, g3e, as3h, as3e, as3e, c4h, c4e, c4e, d4h, d4e, d4e],
            &[f4i, ri, f4i, ri, g4q, rq, g4i, ri, g4i, ri, c4q, rq, c4i, ri, c4i, ri, ds4q, rq, ds4i, ri, ds4i, ri, f4q, rq, f4i, ri, f4i, ri, g4q, rq, g4i, ri, g4i, ri],
            &[hite, hite, duty(35), volume(254), hite, re, duty(34), volume(250), hite, re, duty(35), volume(254), hiti, hiti, hiti, hiti, hite, re, duty(34), volume(250), hite, re, hite, hite, duty(35), volume(254), hite, re, hite, re, duty(34), volume(250), hite, hite, duty(35), volume(254), hite, re, duty(34), volume(250), hite, re, hite, hite, duty(35), volume(254), hite, re, hite, hite, re, hite],
        ),
        section(
            &[volume!(253 g3e g3e g3e f3e f3e f3e g3e g3e g3e f3e f3e f3e, 0 g3e g3e g3e), volume(253), duty!(36 g5i c6i f6i g6i f6i c6i, 37 g4e g4e g4e f4e f4e f4e g4e g4e g4e f4e f4e f4e), volume(0), g4e, g4e],
            &[volume!(253 c4qd c4qd c4qd c4qd, 0 d4e d4e d4e, 255 d3e d4e d3e, 253 c4qd c4qd c4qd c4qd, 0 d4e d4e)],
            &[g4q, re, g4q, re, g4q, re, g4q, re, g4i, ri, g4i, ri, g4i, ri, g3i, ri, g4i, ri, g3i, ri, g4q, re, g4q, re, g4q, re, g4q, re, g4i, ri, g4i, ri],
            &[duty(34), volume(250), hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, duty(35), volume(254), hite, hite, hite, re, duty(34), volume(250), hite, duty(35), volume(254), hiti, hiti, duty(34), volume(250), hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, hite, duty(35), volume(254), hite, hite],
        ),
        section(
            &[g4e, volume(253), duty(36), g5i, c6i, f6i, g6i, f6i, c6i, duty(37), volume(250), g4e, re, g4e, re, g4e, volume(252), g4e, g4e, re, g4e, re, g4e, g4e, volume(254), g5e, re, g5e, re, g5e, g5e, volume(0), g5e, re, g5e, re, g5e, g5e],
            &[d4e, volume!(255 d3e d4e d3e, 250 c4e), re, c4e, re, c4e, c4e, volume(252), d4e, re, d4e, re, d4e, d4e, volume(254), c5e, re, c5e, re, c5e, c5e, volume(0), d5e, re, d5e, re, d5e, d5e],
            &[g4i, ri, g3i, ri, g4i, ri, g3i, ri, g3e, re, g3e, re, g3e, g3e, g3e, re, g3e, re, g3e, g3e, g3e, re, g3e, re, g3e, g3e, g3e, re, g3e, re, g3e, g3e],
            &[hite, re, duty(34), volume(250), hite, duty(35), volume(254), hiti, hiti, duty(35), volume(250), hite, re, hite, re, hite, hite, volume(252), hite, re, hite, re, hite, hite, volume(254), hite, re, hite, re, hite, hite, volume(0), hite, re, hite, re, hite, hite],
        ),
    ])
}

pub fn song_05() -> Song {
    song(24, &[
        section(
            &[duty(39), volume(253), g4e, c5e, ds5e, d5e, fs5w, g5q, d5q, ds5w, rq, as4q, fs4q, g4q],
            &[duty(37), pitch(6), volume(0), c3e, ds3e, g3e, fs3e, as3e, a3e, rq, d4e, cs4e, rq, a3e, as3e, rq, c3e, ds3e, g3e, fs3e, as3e, a3e, rq, d4e, cs4e, rq, a3e, as3e, rq],
            &[c3e, ds3e, g3e, fs3e, as3e, a3e, rq, d4e, cs4e, rq, a3e, as3e, rq, c3e, ds3e, g3e, fs3e, as3e, a3e, rq, d4e, cs4e, rq, a3e, as3e, rq],
            &[volume(0), duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hite, hite, duty(34), hiti, hiti, hiti, hiti, duty(35), hite, hite, duty(34), hiti, hiti, hiti, hiti, duty(35), hite, hite, duty(34), hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hite, hite, duty(34), hiti, hiti, hiti, hiti, duty(35), hite, hite, duty(34), hiti, hiti, hiti, hiti, duty(35), hite, hite, hiti, hiti, hiti, hiti],
        ),
        section(
            &[duty(37), volume(0), ds4h + ds4e, fs4q, g4e, a4qd, as4qd, fs4q, g4qd, ds4qd, fs4q, a4qd, as4qd, c5q],
            &[c4w, fs4qd, g4qd, d4q, ds4qd, c4h + c4e, fs4qd, g4qd, a4q],
            &[c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i, c4i, ri, c4i, c5i],
            &[duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[cs5qd, as4qd, f4h, volume(253), pitch(8), cs4q, ds4q, f4q, volume(0), pitch(0), cs4qd, ds4qd, f4h, volume(253), pitch(8), cs5q, f5q, as5q],
            &[as4hd, cs4h, cs4q, ds4q, f4q, as3qd, c4qd, cs4h, cs5q, f5q, as5q],
            &[as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i, as3i, ri, as3i, as4i],
            &[duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[volume(0), pitch(0), c5e, c5e, ds5e, g5e, fs5e, g5e, fs5e, ds5e, c5e, ds5e, g5e, as5q, a5qd, c5e, c5e, ds5e, g5e, fs5e, g5e, fs5e, ds5e, c5e, ds5e, g5e, as5q, a5qd],
            &[volume(253), re, c5e, c5e, ds5e, g5e, fs5e, g5e, fs5e, ds5e, c5e, ds5e, g5e, as5q, a5qd, c5e, c5e, ds5e, g5e, fs5e, g5e, fs5e, ds5e, c5e, ds5e, g5e, as5q, a5q],
            &[c3id, rt, c3id, rt, rq, c3id, rt, c3id, rt, rq, c3e, c4e, ds4e, g4e, fs4e, g4e, fs4e, ds4e, c3id, rt, c3id, rt, rq, c3id, rt, c3id, rt, rq, c3e, c4e, ds4e, g4e, fs4e, g4e, fs4e, ds4e],
            &[duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti],
        ),
        section(
            &[c4q, fs4e, g4q, as4e, a4e, fs4e, g4q, d4e, ds4q, as3e, b3q, c4q, fs4e, g4q, as4e, a4e, fs4e, g4q, d5e, ds5q, as4e, b4q],
            &[volume(0), c3q, fs3e, g3q, as3e, a3e, fs3e, g3q, d3e, ds3q, as2e, b2q, c3q, fs3e, g3q, as3e, a3e, fs3e, g3q, d4e, ds4q, as3e, b3q],
            &[c3ed, ri, fs3i, ri, g3q, as3i, ri, a3i, ri, fs3i, ri, g3ed, ri, d3i, ri, ds3ed, ri, as2i, ri, b2ed, ri, c3ed, ri, fs3i, ri, g3q, as3i, ri, a3i, ri, fs3i, ri, g3ed, ri, d4i, ri, ds4ed, ri, as3i, ri, b3ed, ri],
            &[duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hiti, hiti, hiti, hiti, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hiti, ri, hiti, hiti],
        ),
        section(
            &[duty(0)],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn home_theme() -> Song {
    song(24, &[
        section(
            &[duty(32), volume(0), e4e, re, g4e, ri, c5e, ri, as4e, f4e, g4h + g4i, ri, f4ed, ri, d4ed, ri, e4e, re, g4e, ri, c5e, ri, e5e, f5e, g5h + g5i, ri, f5ed, ri, d5ed, ri],
            &[volume(255), pitch(8), duty(32), e4e, re, g4e, ri, c5e, ri, as4e, f4e, g4h + g4i, ri, f4ed, ri, d4ed, ri, e4e, re, g4e, ri, c5e, ri, e5e, f5e, g5h + g5i, ri, f5ed, ri, d5ed, ri],
            &[c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, as3e, as3e, as4e, as3e, re, as3e, as4e, as3e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, gs3e, gs3e, gs4e, gs3e, re, as3e, as4e, as3e],
            &[volume(254), duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[e4e, re, g4e, ri, c5e, ri, as4e, f4e, g4h + g4i, ri, f4ed, ri, d4ed, ri, e4e, re, g4e, ri, c5e, ri, e5e, f5e, g5h + g5i, ri, c5ed, ri, d5ed, ri],
            &[e4e, re, g4e, ri, c5e, ri, as4e, f4e, g4h + g4i, ri, f4ed, ri, d4ed, ri, e4e, re, g4e, ri, c5e, ri, e5e, f5e, g5h + g5i, ri, c5ed, ri, d5ed, ri],
            &[c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, as3e, as3e, as4e, as3e, re, as3e, as4e, as3e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, gs3e, gs3e, gs4e, gs3e, re, as3e, as4e, as3e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hiti, hiti, hite, duty(35), hite, duty(34), hite, hiti, hiti, duty(35), hite, duty(34), hite, duty(35), hite],
        ),
        section(
            &[ds5w, d5h, f5h, f5e, e5e, f5e, e5q, re, f4e, e4e, f4e, e4q, re, c4e, g4e, c5e, d5e],
            &[ds5w, d5h, f5h, f5e, e5e, f5e, e5q, re, f4e, e4e, f4e, e4q, re, c4e, g4e, c5e, d5e],
            &[gs3e, gs3e, gs4e, gs3e, re, gs3e, gs4e, gs3e, as3e, as3e, as4e, as3e, re, as3e, as4e, as3e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[ds5w, d5h, f5h, f5e, c5e, d5e, e5w + e5e, rh],
            &[ds5w, d5h, f5h, f5e, c5e, d5e, e5w + e5e, rh],
            &[gs3e, gs3e, gs4e, gs3e, re, gs3e, gs4e, gs3e, as3e, as3e, as4e, as3e, re, as3e, as4e, as3e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e, c4e, c4e, c5e, c4e, re, c4e, c5e, c4e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, hite, hite, hite],
        ),
    ])
}

pub fn song_07() -> Song {
    song(24, &[
        section(
            &[rh, volume(0), duty(40), b4h, c5ed, b4ed, g4e, a4h, b4ed, a4q + a4i, a4h, b4ed, a4ed, fs4e, g4h, a4ed],
            &[rh, volume(252), duty(40), pitch(8), red, b4h, c5ed, b4ed, g4e, a4h, b4ed, a4q + a4i, a4h, b4ed, a4ed, fs4e, g4h],
            &[rh, g3i, ri, g3i, ri, g4e, ri, g3e, ri, g3i, ri, g4e, ri, g3i, fs3i, ri, fs3i, ri, fs4e, ri, fs3e, ri, fs3i, ri, fs4e, ri, fs3i, f3i, ri, f3i, ri, f4e, ri, f3e, ri, f3i, ri, f4e, ri, f3i, e3i, ri, e3i, ri, e4e, ri, e3e, ri, e3i],
            &[volume(0), duty(34), hiti, duty(35), hite, duty(34), hiti, duty(35), hite, duty(35), hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti],
        ),
        section(
            &[g4q + g4i, g4h, a4ed, g4ed, a4e, b4qd, d4qd, b4q, a4qd, e4qd, g4q, g4e, fs4e, e4e, d4q, d5qd, b4h, c5ed],
            &[a4ed, g4q + g4i, g4h, a4ed, g4ed, a4e, b4qd, d4qd, b4q, a4qd, e4qd, g4q, g4e, fs4e, e4e, d4q, d5qd, b4h],
            &[ri, e4e, ri, e3i, ds3i, ri, ds3i, ri, ds4e, ri, ds3e, ri, ds3i, ri, ds4e, ri, ds3i, d3i, ri, d3i, ri, d4e, ri, d3e, ri, d3i, ri, d4e, ri, d3i, cs3i, ri, cs3i, ri, cs4e, ri, cs3e, ri, cs3i, ri, cs4e, ri, cs3i, d3i, ri, d3i, ri, d4e, ri, d3e, ri, d3i, d4i, e3i, e4i, fs3i, fs4i, g3i, ri, g3i, ri, g4e, ri, g3e, ri, g3i],
            &[hiti, duty(35), hiti, duty(34), hiti, duty(35), hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, duty(35), hiti, hiti, duty(34), hiti, duty(35), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti],
        ),
        section(
            &[b4ed, c5e, d5h, e5ed, fs5q + fs5i, g5h, a5ed, g5q + g5i, g5qd, e5h + e5e, g5e, fs5e, e5e, d5e, re],
            &[c5ed, b4ed, c5e, d5h, e5ed, fs5q + fs5i, g5h, a5ed, g5q + g5i, g5qd, e5qdd, pitch(0), volume(254), e5e, d5e, c5e, b4e, re],
            &[ri, g4e, ri, g3i, fs3i, ri, fs3i, ri, fs4e, ri, fs3e, ri, fs3i, ri, fs4e, ri, fs3i, f3i, ri, f3i, ri, f4e, ri, f3e, ri, f3i, ri, f4e, ri, f3i, e3i, ri, e3i, ri, e4e, ri, e3e, ri, e3i, ri, e4e, ri, e3i, c3i, ri, c3i, ri, c4e, ri, c3e, ri],
            &[hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti],
        ),
        section(
            &[e5e, re, d5q, re, d5e, re, c5e, b4e, c5e, b4hd, a4e, g4q, d5h, e5q, fs5q, g5w],
            &[c5e, re, b4q, re, b4e, re, a4e, g4e, a4e, g4w, re, fs4h, b4h, e5w],
            &[c3i, ri, c4e, ri, c3i, d3i, ri, d3i, ri, d4e, ri, d3e, ri, d3i, ri, d4e, ri, d3i, g3i, ri, g3i, ri, g4e, ri, g3e, ri, g3i, ri, g4e, ri, g3i, fs3i, ri, fs3i, ri, fs4e, ri, fs3e, ri, fs3i, ri, fs4e, ri, fs3i, e3i, ri, e3i, ri, e4e, ri, e3e, ri, e3i, ri, e4e, ri, e3i],
            &[hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, hiti, hiti, duty(34), hiti, duty(35), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti],
        ),
        section(
            &[re, g5e, re, g5e, fs5e, e5e, fs5e, g5h + g5e, b4e, a4e, g4e, a4h + a4e, d5h, g4w],
            &[re, pitch(8), g5e, re, g5e, fs5e, e5e, fs5e, g5w, pitch(0), fs4h + fs4e, b4h, pitch(8), g4w],
            &[c3i, ri, c3i, ri, c4e, ri, c3i, d3i, ri, d3i, ri, d4e, ri, d3i, g3i, ri, g3i, ri, g4e, ri, g3e, ri, g3i, ri, g4e, ri, g3i, fs3i, ri, fs3i, ri, fs4e, ri, fs3e, ri, fs3i, ri, fs4e, ri, fs3i, e3i, ri, e3i, ri, e4e, ri, e3e, ri, e3i, ri, e4e, ri, e3i],
            &[hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, hiti, duty(34), hiti, duty(35), hiti, hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti],
        ),
        section(
            &[re, g5e, re, g5e, fs5e, e5e, fs5e, g5w, duty(36), volume(254), raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3, raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3, volume(0), duty(40), re, c6i, ri, c6i, b5ed, c6i, ri, c6i, b5q + b5i, re, c5i, ri, c5i, b4ed, c5i, ri, c5i, b4q + b4i],
            &[re, g5e, re, g5e, fs5e, e5e, fs5e, g5q, pitch(0), c5i, ri, c5i, b4ed, c5i, ri, c5i, b4q + b4i, re, g5i, ri, g5i, g5ed, g5i, ri, g5i, g5ed, duty(36), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), rt3, raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), rt3, duty(40), re, g5i, ri, g5i, g5ed, g5i, ri, g5i, g5ed, duty(36), raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3, raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3],
            &[c3i, ri, c3i, ri, c4e, ri, c3i, d3i, ri, d3i, ri, d4e, ri, d3i, g3i, d4i, b4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i, g3i, d4i, g4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i, g3i, d4i, b4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i, c5i, g4i, d4i, b4i, g4i, d4i],
            &[hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hitq, duty(34), hite, re, hite, re, hite, hiti, hiti, duty(35), hitq, duty(34), hite, re, hite, re, hite, hiti, hiti, duty(35), hitq, duty(34), hite, re, hite, re, hite, hiti, hiti],
        ),
        section(
            &[re, c6i, ri, c6i, b5ed, volume(254), d5i, fs5i, a5i, fs5i, a5i, d6i, fs6i, a6i],
            &[duty(40), re, g5i, ri, g5i, g5ed, duty(36), raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3, raw(82, 1), raw(84, 1), raw(82, 1), raw(84, 1), rt3, ri, raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), raw(74, 1), raw(76, 1), rt3],
            &[g3i, d4i, g4i, d4i, c5i, g4i, d4i, b4i, d4i, a4i, d5i, d4i, a4i, d5i, d4i, a4i],
            &[duty(35), hitq, duty(34), hite, re, hiti, duty(35), hite, duty(34), hiti, duty(35), hite, hiti, hiti],
        ),
    ])
}

pub fn death_jingle() -> Song {
    song(32, &[
        section(
            &[duty(32), volume(0), ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, a4ed, d4ed, g4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, e5ed, d5ed, as4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, a4ed, d4ed, g4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, e5ed, d5ed, g5e],
            &[volume(252), pitch(5), duty(32), red, a3i, e4i, a4i, b4i, c5i, b4i, g4i, a4ed, d4ed, g4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, e5ed, d5ed, as4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, a4ed, d4ed, g4e, ri, a3i, e4i, a4i, b4i, c5i, b4i, g4i, e5ed, d5ed],
            &[a3i, e4i, a4qd, as3i, f4i, as4qd, a3i, e4i, a4qd, as3i, f4i, as4qd, a3i, e4i, a4qd, as3i, f4i, as4qd, a3i, e4i, a4qd, as3i, f4i, as4qd],
            &[volume(0), duty(34), hite, re, hite, re, hite, re, duty(35), hitq, duty(34), hite, re, hite, re, hite, re, duty(35), hitq, duty(34), hite, re, hite, re, hite, re, duty(35), hitq, duty(34), hite, re, hite, re, hite, re, duty(35), hitq],
        ),
        section(
            &[e5i, a4i, ri, volume(253), e5i, a4i, ri, volume(250), e5i, a4i, ri, volume(247), e5i, a4i, ri, volume(244), e5i, a4i, re],
            &[g5e, e5i, a4i, ri, volume(250), e5i, a4i, ri, volume(247), e5i, a4i, ri, volume(244), e5i, a4i, ri, re],
            &[a3i, ri, c4i, ri, d4i, e4i, g4i, a4i, ri, e4i, g4i, d4i, ri, c4i, b3i, g3i],
            &[duty(34), hite, re, hite, re, hite, re, duty(35), hitq],
        ),
    ])
}

pub fn title_theme() -> Song {
    song(24, &[
        section(
            &[duty(37), pitch(1), volume(254), a3h, e4hd, cs4q, d4q, e4q, d4h, d5hd, a4q, f4q3, c4q3, d4q3],
            &[duty(37), pitch(8), volume(254), a3h, e4hd, cs4q, d4q, e4q, d4h, d5hd, a4q, f4q3, c4q3, d4q3],
            &[sweep(255), a4w, g4w, fs4w, f4w],
            &[rw, rw, rw, rw],
        ),
        section(
            &[e4w, e5w, duty(32), volume(0), a4h, e4hd, cs4q, a3ed, cs4ed, e4e],
            &[e4w, volume(253), e4i, gs4i, b4i, e5i, gs4i, b4i, e5i, gs5i, b4i, e5i, gs5i, b5i, e5i, gs5i, b5i, e6i, duty(32), pitch(8), volume(254), a4h, e4q, a6q, e6q, a4e3, e4e3, cs4e3, e4ed, a4ed, cs5e],
            &[sweep(19), e4e, e4e, e4e, e4e, e4e, e4e, e4e, e4e, e3e, e3e, e3e, e3e, e3e, e3e, e3e, e3e, a3e, a3e, a4e, a3e, a3e, a3e, a4e, a3e, g3e, g3e, g4e, g3e, g3e, g3e, g4e, g3e],
            &[volume(254), duty(34), hite, re, hite, re, hite, re, hite, re, hite, hite, hiti, hiti, duty(35), hite, duty(34), hite, hite, hiti, hiti, hiti, hiti, volume(254), duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[fs4h, d4hd, a4q, d5ed, e5ed, d5e, duty(32), cs5h, a4h, b4q, e4q, b4q, cs5q],
            &[d5h, fs4q, a6q, d6q, a4e3, f4e3, d4e3, f4ed, g4ed, f4e, volume(253), cs5e3, a4e3, e4e3, a4e3, e4e3, cs4e3, e4e3, cs4e3, a3e3, cs4e3, e4e3, a4e3, e5e3, b4e3, gs4e3, b4e3, gs4e3, e4e3, gs4e3, e4e3, b3e3, e4e3, gs4e3, b4e3],
            &[fs3e, fs3e, fs4e, fs3e, fs3e, fs3e, fs4e, fs3e, f3e, f3e, f4e, f3e, f3e, f3e, f4e, f3e, e3e, e3e, e4e, e3e, e3e, e3e, e4e, e3e, e3e, e3e, e4e, e3e, e3e, e3e, e4e, e3e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[d5h, fs4h, gs4h, e4h],
            &[fs5e3, d5e3, a4e3, d5e3, a4e3, fs4e3, a4e3, fs4e3, d4e3, a3e3, d4e3, fs4e3, e4e3, gs4e3, b4e3, gs4e3, b4e3, e5e3, b4e3, e5e3, gs5e3, e5i, gs5i, b5i, e6i],
            &[d3e, d3e, d4e, d3e, d3e, d3e, d4e, d3e, e3e, e3e, e4e, e3e, e3e, e3e, e4e, e3e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
    ])
}

pub fn ending_theme() -> Song {
    song(24, &[
        section(
            &[duty(32), volume(0), as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5e, a5i, g5i, a5e, g5i, fs5i, a5ed, d5ed, c6e, as5i, g5i],
            &[pitch(5), duty(32), volume(252), re, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, as5e, a5i, g5i, a5e, g5i, fs5i, a5ed, d5ed, c6e, ri],
            &[g3e, ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, ds3e, ri, ds3e, ri, ds3ed, ri, as3i, ri, ds4i, ri, as3i, ri, d3e, ri, d3e, ri, d3ed, ri, a3i, ri, d4i, ri, a3i, ri, g3e],
            &[volume(0), duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, duty(35), hite, hiti, duty(34), hiti, duty(35), hite, hiti, duty(34), hiti, ri],
        ),
        section(
            &[ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5e, a5i],
            &[as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, as5e],
            &[ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, ds3e, ri, ds3e, ri, ds3ed, ri, as3i, ri, ds4i, ri, as3i, ri, d3e, ri],
            &[hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti],
        ),
        section(
            &[g5i, c6e, as5i, a5i, d6ed, f6ed, fs6e, duty(41), g6h, g5e, a5e, b5e, c6e, b5e, g5e, d5e, d6e, b5e, g5e, f6e, d6e, ds6qd, c6qd, g5h, g6q, f6q, ds6q, d6hdd, a5i, as5i, c6h, as5q, a5q, as5i, a5i, g5hdd, re],
            &[a5i, g5i, c6e, as5i, a5i, d6ed, f6ed, fs6e, duty(41), re, g6h, g5e, a5e, b5e, c6e, b5e, g5e, d5e, d6e, b5e, g5e, f6e, d6e, ds6qd, c6qd, g5h, g6q, f6q, ds6q, d6hdd, a5i, as5i, c6h, as5q, a5q, as5i, a5i, g5hd + g5i],
            &[d3e, ri, d3ed, ri, a3i, ri, d4i, ri, a3i, ri, g3e, ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri, g3e, ri, g3e, ri, g3ed, ri, b3i, ri, g4i, ri, b3i, ri, c4e, ri, c4e, ri, c4ed, ri, g4i, ri, c5i, ri, g4i, ri, c4e, ri, c4e, ri, c4ed, ri, g4i, ri, c5i, ri, g4i, ri, d4e, ri, d4e, ri, d4ed, ri, a4i, ri, d5i, ri, a4i, ri, d4e, ri, d4e, ri, d4ed, ri, a4i, ri, d5i, ri, a4i, ri, g3e, ri, g3e, ri, g3ed, ri, d3i, ri, f3i, ri, fs3i, ri, g3e],
            &[hiti, duty(35), hite, duty(34), hiti, hiti, hiti, duty(35), hite, hiti, duty(34), hiti, duty(35), hiti, hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(35), hite],
        ),
        section(
            &[duty(32), d4i, d4i, d4i, d4i, re, d4ed, f4ed, fs4e, duty(42), d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e],
            &[duty(32), a3i, a3i, a3i, a3i, re, a3ed, d4ed, d4e, duty(42), volume(253), re, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e],
            &[d4t, rt, d4t, rt, d4t, rt, d4t, rt, re, d4e, ri, f4e, ri, fs4e, volume(0), g3e, ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, e3e, ri, e3e, ri, e3ed, ri, c4i, ri, e4i, ri, c4i, ri],
            &[duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[as5qd, c6i, as5i, a5q3, g5q3, f5q3, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e],
            &[c6e, as5qd, c6i, as5i, a5q3, g5q3, f5q3, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e],
            &[ds3e, ri, ds4e, ri, ds3i, ri, f3e, ri, f4e, ri, f3i, ri, g3e, ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, e3e, ri, e3e, ri, e3ed, ri, c4i, ri, e4i, ri, c4i, ri],
            &[hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[as5qd, c6i, as5i, a5q3, as5q3, c6q3, duty(40), d6h + d6e, as5e, d6e, c6h + c6e, a5q, c6q, d6h + d6e, as5e, d6e],
            &[c6e, as5qd, c6i, as5i, a5q3, as5q3, c6q3, volume(0), pitch(0), duty(40), re, g5qd, g5e, g5e, a5h + a5e, f5q, a5q, rq, g5qd, g5e, g5e],
            &[ds3e, ri, ds4e, ri, ds3i, ri, f3e, ri, f4e, ri, f3i, ri, ds3e, ri, ds3e, ri, ds3ed, ri, as3i, ri, ds4i, ri, as3i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, ds3e, ri, ds3e, ri, ds3ed, ri, as3i, ri, ds4i, ri],
            &[hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite],
        ),
        section(
            &[c6h + c6e, a5q3, as5q3, c6q3, as5i, a5i, g5hdd, re, duty(41), volume(253), d4e, g4e, a4e, as4e, c5e, d5e, f5e, g5qd, g4h + g4e],
            &[a5h + a5e, f5q3, g5q3, a5q3, pitch(5), g5h, volume(252), as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i],
            &[as3i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, g3e, re, g3e, re, g3e, re, g3e, re, f3e, re, f3e, re, f3e, re, f3e, re, ds3e, re, ds3e, re, ds3e, re, ds3e, re],
            &[duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(34), hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, hiti, hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti],
        ),
        section(
            &[volume(0), duty(32), g5q, fs5q, e5q, fs5q, g5h, as4e, ds5e, f5e, g5e, a5h, f5q3, g5q3, a5q3, b5w],
            &[pitch(0), volume(0), d5q, d5q, d5q, d5q, ds5h, pitch(5), g4e, as4e, as4e, as4e, c5h, c5q3, c5q3, c5q3, d5w],
            &[d3i, ri, d3i, ri, d3i, ri, d3i, ri, d3i, ri, d3i, ri, d3i, ri, d3i, ri, ds3e, ri, ds3e, ri, ds3ed, ri, as3i, ri, ds4i, ri, as3i, ri, f3e, ri, f3e, ri, f3ed, ri, c4i, ri, f4i, ri, c4i, ri, g3e, ri, g3e, ri, g3ed, ri, d4i, ri, g4i, ri, d4i, ri],
            &[duty(35), hite, hite, hite, hite, hite, hite, hite, hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, duty(34), hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti, hiti, ri, hiti, hiti, duty(35), hite, duty(34), hiti, hiti],
        ),
        section(
            &[d3e, ri, e3e, ri, f3i, ri, g3e, ri, a3e, ri, c4i, ri],
            &[g3e, ri, a3e, ri, as3i, ri, c4e, ri, d4e, ri, f4i, ri],
            &[g3e, ri, a3e, ri, as3i, ri, c4e, ri, d4e, ri, f4i, ri],
            &[duty(35), hitq3, hitq3, hitq3, duty(34), hitt, hitt, hiti, hiti, hiti, duty(35), hiti, hiti, hiti, hiti],
        ),
    ])
}

pub fn song_11() -> Song {
    song(16, &[
        section(
            &[volume(255), duty(32), d4e, d4e, f4e, a4e, f4e, d4e, f4e, a4e, c5qd, b4qd, as4q, d4e, d4e, f4e, a4e, f4e, d4e, f4e, a4e, c5qd, b4qd, as4q],
            &[volume(254), duty(32), a3e, a3e, d4e, f4e, d4e, a3e, d4e, f4e, gs4qd, g4qd, fs4q, a3e, a3e, d4e, f4e, d4e, a3e, d4e, f4e, gs4qd, g4qd, fs4q],
            &[d3i, ri, d3i, ri, rhd, f3qd, e3qd, ds3q, d3i, ri, d3i, ri, rhd, f3qd, e3qd, ds3q],
            &[volume(254), duty(35), hite, hite, duty(34), hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, hite, duty(35), hite, hite, duty(34), hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hiti, hiti, hiti, hiti],
        ),
        section(
            &[a4h, d4e, e4e, f4e, g4e, a4q, g4e, f4e, re, c5qd, a4w, re, duty(0), as5e, a5e, gs5e, g5e, fs5e, f5e, e5e],
            &[pitch(8), a4h, d4e, e4e, f4e, g4e, a4q, g4e, f4e, re, c5qd, a4w, re, re, volume(252), pitch(0), duty(0), as5e, a5e, gs5e, g5e, fs5e, f5e],
            &[d3e, d4e, d3e, d4e, d3e, d4e, d3e, d4e, e3e, e4e, e3e, e4e, e3e, e4e, e3e, e4e, f3e, f4e, f3e, f4e, f3e, f4e, f3e, f4e, e3e, e4e, e3e, e4e, e3e, e4e, e3e, e4e],
            &[duty(34), hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite],
        ),
        section(
            &[duty(32), a4h, d4e, e4e, f4e, g4e, a4q, g4e, f4e, re, c5qd, a4w, re, duty(0), as5e, a5e, gs5e, g5e, fs5e, f5e, e5e],
            &[duty(32), pitch(8), volume(254), a4h, d4e, e4e, f4e, g4e, a4q, g4e, f4e, re, c5qd, a4w, re, re, duty(0), pitch(0), volume(252), as5e, a5e, gs5e, g5e, fs5e, f5e],
            &[d3e, d4e, d3e, d4e, d3e, d4e, d3e, d4e, e3e, e4e, e3e, e4e, e3e, e4e, e3e, e4e, f3e, f4e, f3e, f4e, f3e, f4e, f3e, f4e, e3e, e4e, e3e, e4e, e3e, e4e, e3e, e4e],
            &[duty(34), hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, hite],
        ),
        section(
            &[duty(32), as4e, f4e, d4e, f4e, re, d4qd, rq, duty(0), d5e, d5e, c5e, d5q, re, duty(32), d4h, e4h, f4h, g4h],
            &[duty(32), pitch(8), volume(254), as4e, f4e, d4e, f4e, re, d4qd, rq, pitch(0), duty(0), g4e, g4e, f4e, g4q, re, duty(32), a3h, b3h, c4h, d4h],
            &[ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, ds3e, ds4e, f3e, f4e, f3e, f4e, g3e, g4e, g3e, g4e, gs3e, gs4e, gs3e, gs4e, as3e, as4e, as3e, as4e],
            &[duty(35), hite, duty(34), hite, hite, hite, hite, hite, hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, duty(35), hite, hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[a4w, rw, a4w, rw],
            &[e4w, re, a4e, c5e, e5e, a5e, c5e, e5e, a5e, e4w, re, a4e, cs5e, e5e, a5e, cs5e, e5e, a5e],
            &[c4e, c5e, c4e, c5e, c4e, c5e, c4e, c5e, c4e, c5e, c4e, c5e, c4e, c5e, c4e, c5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e, cs4e, cs5e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, hiti, hiti, hite, hiti, hiti, hite, hiti, hiti, hite, hiti, hiti],
        ),
    ])
}

pub fn song_12() -> Song {
    song(16, &[
        section(
            &[duty(41), volume!(250 a5e fs5e, 251 d5e b5e, 252 g5e d5e, 253 a5e fs5e d5e b5e g5e d5e a5e fs5e, 254 d5e b5e g5e d5e a5e fs5e d5e b5e g5e d5e a5e fs5e d5e b5e g5e d5e a5e fs5e d5e b5e)],
            &[duty(41), pitch(8), re, a5e, fs5e, volume!(248 d5e b5e, 249 g5e d5e, 250 a5e fs5e d5e b5e g5e d5e, 251 a5e fs5e d5e b5e g5e, 252 d5e a5e fs5e d5e b5e g5e d5e a5e fs5e d5e b5e g5e d5e a5e fs5e d5e)],
            &[sweep(255), d4w + d4h, c4w + c4h, b3w + b3q],
            &[volume(0), duty(34), hite, rw, re, hite, hite, hite, rw, re, hite, hite, hite, rw, re],
        ),
        section(
            &[g5e, d5e, a5e, f5e, c5e, d5e, c5e, g4e],
            &[b5e, g5e, d5e, a5e, f5e, c5e, d5e, c5e],
            &[g3e, a3e, as3q, re, as3q, re],
            &[hiti, hiti, hiti, hiti, hite, re, hite, hite, re, hite],
        ),
        section(
            &[a4qd, g4qd, volume(0), fs4hd, g4qd, a4qd, e4hd, fs4hd, g4hd, d4qd, e4qd, f4h, e4e, f4e],
            &[g4e, a4qd, g4qd, re, fs4hd, g4qd, a4qd, e4hd, fs4hd, g4hd, d4qd, e4qd, f4h],
            &[sweep(26), c4e, c4e, c4e, c4e, c4e, c4e, d4e, rq, d4e, rq, d4e, d4e, d4e, d4e, rq, c4e, rq, c4e, rq, c4e, c4e, c4e, c4e, rq, b3e, rq, b3e, rq, b3e, b3e, b3e, b3e, rq, as3e, rq, as3e, rq],
            &[hiti, hiti, hite, hite, hiti, hiti, hiti, hiti, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq],
        ),
        section(
            &[c4e, g3e, c4e, d4e, e4e, g4e, fs4hd, g4qd, a4qd, e4hd, fs4hd, g4hd, d4qd, g4qd, d5qd, c5qd, g4qd, a4qd],
            &[e4e, f4e, c4e, g3e, c4e, d4e, e4e, g4e, fs4hd, g4qd, a4qd, e4hd, fs4hd, g4hd, d4qd, g4qd, d5qd, c5qd, g4qd, a4e],
            &[c4e, c4e, c4e, c4e, c4e, c4e, d4e, rq, d4e, rq, d4e, d4e, d4e, d4e, rq, c4e, rq, c4e, rq, c4e, c4e, c4e, c4e, rq, b3e, rq, b3e, rq, b3e, b3e, b3e, b3e, rq, as3e, rq, as3e, rq, c4e, c4e, c4e, c4e, c4e, c4e],
            &[hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, re, hite, re, hite, hite],
        ),
        section(
            &[as4h + as4e, ds5h, as5hd, g5qd, gs5qd, as5qd, ds5h + ds5e, ds5h],
            &[ds4h + ds4e, ds5h, as5hd, g5qd, gs5qd, as5qd, ds5h + ds5e, as4h],
            &[ds4e, rq, ds4e, rq, ds4e, ds4e, ds4e, ds4e, rq, cs4e, rq, cs4e, rq, cs4e, cs4e, cs4e, cs4e, rq, c4e, rq, c4e, rq, c4e, c4e, c4e],
            &[hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq],
        ),
        section(
            &[ds5q, ds5hd, ds5hdd, ds5h + ds5e, ds5hdd, c5h + c5e, c5q, c5e, a4e],
            &[ds5q, as5hd, gs5hd, re, gs5h + gs5e, g5hdd, g5h + g5e, f5qd, c5e],
            &[c4e, rq, b3e, rq, b3e, rq, b3e, b3e, b3e, b3e, rq, as3e, rq, as3e, rq, as3e, as3e, as3e, as3e, rq, gs3e, rq, gs3e, rq, gs3e, gs3e, gs3e],
            &[hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq],
        ),
        section(
            &[f4e, c4e, f4e, g4hd, d5hd, g5w + g5q],
            &[a4e, f4e, c4e, f4e, g4hd, g4hd, g5hdd, d4q],
            &[gs3e, rq, g3e, rq, g3e, rq, g3e, g3e, g3e, g3e, rq, g3e, rq, g3e, rq, g3e, g3e, g3e, g3e],
            &[hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite, re, hiti, hiti, hite, rq, hite, rq, hite, rq, hite],
        ),
        section(
            &[d4e, e4e],
            &[e4qd],
            &[g3e, g3e],
            &[hite, hite],
        ),
    ])
}

pub fn song_13() -> Song {
    song(64, &[
        section(
            &[volume(0), duty!(40 a4e e5i d5e g5ed f5ed c5e d5ed a4e e5i d5e g5ed f5ed as5e d6ed, 0 b7i g7i a7i e7i g7i d7i e7i b6i b7i g7i a7i e7i g7i d7i e7i b6i b7i g7i a7i e7i g7i d7i e7i b6i b7i g7i a7i e7i g7i d7i e7i b6i, 40 e5i d5i a4i e5i d5i a4ed e5i d5i g4i e5i d5i g4ed f4ed as4ed d5e f5i e5i c5i), volume!(253 f5i e5i c5i, 250 f5i e5i)],
            &[volume(251), pitch(5), duty(40), rid, a4e, e5i, d5e, g5ed, f5ed, c5e, d5ed, a4e, e5i, d5e, g5ed, f5ed, as5e, d6ed, duty(0), pitch(0), b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, b7i, g7i, a7i, e7i, g7i, d7i, e7i, b6i, duty(40), pitch(5), e5i, d5i, a4i, e5i, d5i, a4ed, e5i, d5i, g4i, e5i, d5i, g4ed, f4ed, as4ed, d5e, f5i, e5i, c5i, volume!(248 f5i e5i c5i, 245 f5t)],
            &[a3q, a3q, as3q, ri, as3t, rt, as3t, d4id, a3q, a3q, as3q, d4e, f4e, a4q, a4q, g4q, g4q, fs4q, fs4q, f4q, f4q, as3q, as3q, c4q, e4e, g4e, as3q, as3q, c4q, c4q],
            &[volume(254), duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, duty(34), hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, hiti, ri, duty(35), hite, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, duty(34), hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti, hiti, hiti, duty(35), hiti, duty(34), hiti],
        ),
    ])
}

pub fn door_unlock_jingle() -> Song {
    song(24, &[
        section(
            &[duty(41), volume!(0 a3i e4i a4i b4i cs5i e5i, 254 a4i e5i a5i b5i cs6i e6i, 252 a3i e4i a4i b4i cs5i e5i, 250 a4i e5i a5i b5i cs6i e6i)],
            &[duty(41), volume(254), ri, a3i, e4i, a4i, b4i, cs5i, e5i, volume!(252 a4i e5i a5i b5i cs6i e6i, 250 a3i e4i a4i b4i cs5i e5i, 249 a4i e5i a5i b5i cs6i)],
            &[a3hd, rhd],
            &[],
        ),
    ])
}

pub fn song_15() -> Song {
    song(12, &[
        section(
            &[duty(32), volume(0), a4qd, g4h + g4e, a4qd, c5qd, g4q, a4qd, g4h + g4e, a4qd, c5qd, d5q],
            &[duty(32), pitch(8), volume(255), e4qd, d4h + d4e, e4qd, g4qd, d4q, e4qd, d4h + d4e, e4qd, g4qd, a4q],
            &[a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e],
            &[volume(254), duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[d5qd, e5q, c5qd, d5qd, e5q, c5qd, d5qd, e5q, c5qd, d5qd, c5q, d5qd],
            &[a4qd, a4q, a4qd, a4qd, a4q, a4qd, a4qd, a4q, a4qd, a4qd, a4q, g4qd],
            &[fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, f3e, f4e, g3e, g4e],
            &[hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[a4qd, g4h + g4e, a4qd, c5qd, g4q, a4qd, g4h + g4e, a4qd, c5qd, d5q],
            &[e4qd, d4h + d4e, e4qd, g4qd, d4q, e4qd, d4h + d4e, e4qd, g4qd, a4q],
            &[a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e],
            &[hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[d5qd, e5q, c5qd, d5qd, e5q, c5qd, d5qd, e5q, c5qd, d5e, c5e, g4e, a4e, e4e, d4e, c4e, b3e],
            &[a4qd, a4q, a4qd, a4qd, a4q, a4qd, a4qd, a4q, a4qd, a4e, g4e, d4e, e4e, c4e, b3e, a3e, g3e],
            &[fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, f3e, f4e, g3e, g4e],
            &[hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, hite, hite, hite, hite, hite, hite, hite],
        ),
        section(
            &[a3qd, e3qd, a4qd, e4hdd, a3qd, e3qd, a4qd, e4hdd, a3qd, e3qd, a4qd, e4qd],
            &[volume(254), re, a3qd, e3qd, a4qd, e4hdd, a3qd, e3qd, a4qd, e4hdd, a3qd, e3qd, a4qd, e4q],
            &[a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e],
            &[duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[a4q, b4q, c5qd, b4qd, a4q, d5qd, c5qd],
            &[volume(255), e4q, fs4q, a4qd, g4qd, a4q, g4qd, d4qd],
            &[fs3e, re, fs4e, fs4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, g3e, re, g4e, g4e, g3e, re],
            &[hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite],
        ),
        section(
            &[b4q, a4qd, e4qd, a5qd, e5hdd, a4qd, e4qd, a5qd, e5hdd, a4qd, e4qd, a5qd, e5qd],
            &[g4qd, volume(254), a4qd, e4qd, a5qd, e5hdd, a4qd, e4qd, a5qd, e5hdd, a4qd, e4qd, a5qd, e5q],
            &[g4e, g4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, a3e, re, a4e, a4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e, fs3e, re, fs4e, fs4e],
            &[duty(35), hite, duty(34), hite, duty(34), hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite],
        ),
        section(
            &[a5q, g5q, f5qd, g5q, a5q, f5e, g5qd, d5qd, b4q],
            &[volume!(255 e5q d5q c5qd d5q e5qd, 254 f5e g5e d5e b4e d5e b4e g4e d4e)],
            &[fs3e, re, fs4e, fs4e, f3e, re, f4e, f4e, f3e, re, f4e, f4e, g3e, re, g4e, g4e, g3e, re, g4e, g4e],
            &[hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, hite, hite, duty(35), hite, duty(34), hite, duty(35), hite, hite, hite, hite, hite, hite, duty(34), hite, duty(35), hite],
        ),
    ])
}

pub fn song_16() -> Song {
    song(24, &[
        section(
            &[c10i3, rw + redd, red, rw + redd, red, rest(58)],
            &[duty(32), volume(0), as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5e, a5i, g5i, a5e, g5i, fs5i, a5ed, d5ed, c6e, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5e, a5i, g5i, c6e, as5i, a5i, d6ed, f6ed, fs6e, duty(41), g6h, g5e, a5e, b5e, c6e, b5e, g5e, d5e, d6e, b5e, g5e, f6e, d6e, ds6qd, c6qd, g5h, g6q, f6q, ds6q, d6hdd, a5i, as5i, c6h, as5q, a5q, as5i, a5i, g5hdd, re, duty(32), d4i, d4i, d4i, d4i, re, d4ed, f4ed, fs4e, duty(42), d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e, as5qd, c6i, as5i, a5q3, g5q3, f5q3, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e, as5qd, c6i, as5i, a5q3, as5q3, c6q3, duty(40), d6h + d6e, as5e, d6e, c6h + c6e, a5q, c6q, d6h + d6e, as5e, d6e, c6h + c6e, a5q3, as5q3, c6q3, as5i, a5i, g5hdd, re, duty(41), volume(253), d4e, g4e, a4e, as4e, c5e, d5e, f5e, g5qd, g4h + g4e, volume(0), duty(32), g5q, fs5q, e5q, fs5q, g5h, as4e, ds5e, f5e, g5e, a5h, f5q3, g5q3, a5q3, b5w, d3e, ri, e3e, ri, f3i, ri, g3e, ri, a3e, ri, c4i, ri],
            &[pitch(5), duty(32), volume(252), re, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, as5e, a5i, g5i, a5e, g5i, fs5i, a5ed, d5ed, c6e, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, c6i, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, as5i, g5i, ri, g5i, a5i, as5i, as5e, a5i, g5i, c6e, as5i, a5i, d6ed, f6ed, fs6e, duty(41), re, g6h, g5e, a5e, b5e, c6e, b5e, g5e, d5e, d6e, b5e, g5e, f6e, d6e, ds6qd, c6qd, g5h, g6q, f6q, ds6q, d6hdd, a5i, as5i, c6h, as5q, a5q, as5i, a5i, g5hd + g5i, duty(32), a3i, a3i, a3i, a3i, re, a3ed, d4ed, d4e, duty(42), volume(253), re, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e, as5qd, c6i, as5i, a5q3, g5q3, f5q3, d6h, g5e, a5e, as5e, c6e, d6hdd, c6i, as5i, c6h, g5e, a5e, as5e, c6e, as5qd, c6i, as5i, a5q3, as5q3, c6q3, volume(0), pitch(0), duty(40), re, g5qd, g5e, g5e, a5h + a5e, f5q, a5q, rq, g5qd, g5e, g5e, a5h + a5e, f5q3, g5q3, a5q3, pitch(5), g5h, volume(252), as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, as5i, a5i, g5i, d5i, pitch(0), volume(0), d5q, d5q, d5q, d5q, ds5h, pitch(5), g4e, as4e, as4e, as4e, c5h, c5q3, c5q3, c5q3, d5w, g3e, ri, a3e, ri, as3i, ri, c4e, ri, d4e, ri, f4i, ri],
            &[hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hit(19), ri, hite, hit(19), ri, hited, hit(19), ri, hiti, hitq + hitt, ri, hiti, hith3 + hitt, ri, hiti, hitq + hitt, ri, hite, hited, ri, hite, hited, ri, hited, hited, ri, hiti, hitq + hitt3, ri, hiti, hith3 + hitt3, ri, hiti, hitq + hitt3, ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hit(19), ri, hite, hit(19), ri, hited, hit(19), ri, hiti, hitq + hitt, ri, hiti, hith3 + hitt, ri, hiti, hitq + hitt, ri, hite, hited, ri, hite, hited, ri, hited, hited, ri, hiti, hitq + hitt3, ri, hiti, hith3 + hitt3, ri, hiti, hitq + hitt3, ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hitq + hiti3, ri, hiti, hitqd + hiti3, ri, hiti, hitq + hiti3, ri, hite, hith3, ri, hite, hith3, ri, hited, hith3, ri, hiti, hitqd + hiti3, ri, hiti, hith, ri, hiti, hitqd + hiti3, ri, hite, hith3, ri, hite, hith3, ri, hited, hith3, ri, hiti, hitqd + hiti3, ri, hiti, hith, ri, hiti, hitqd + hiti3, ri, hite, hith3 + hitt3, ri, hite, hith3 + hitt3, ri, hited, hith3 + hitt3, ri, hiti, hitqdd, ri, hiti, hith + hitt3, ri, hiti, hitqdd, ri, hite, hith3 + hitt3, ri, hite, hith3 + hitt3, ri, hited, hith3 + hitt3, ri, hiti, hitqdd, ri, hiti, hith + hitt3, ri, hiti, hitqdd, ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hited, ri, hiti, hit(22), ri, hiti, hitedd + hitt3, ri, hite, hitq, hitt, hith3 + hitt3, rt, hitt, hith3 + hitt3, rt, hitt, hith3 + hitt3, rt, hitt, hith3 + hitt3, rt, re, hite, hith3 + hitt3, ri, hite, hitqd + hitt3, ri, hite, hitqd + hitt, volume(0), hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hited + hitt3, ri, hite, hited + hitt3, ri, hited, hited + hitt3, ri, hiti, hith3, ri, hiti, hitqd, ri, hiti, hith3, ri, hite, hit(19), ri, hite, hith3 + hitt, ri, hiti, hit(19), ri, hite, hit(22), ri, hite, hitqd + hitt3, ri, hiti, hit(22), ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hited + hitt3, ri, hite, hited + hitt3, ri, hited, hited + hitt3, ri, hiti, hith3, ri, hiti, hitqd, ri, hiti, hith3, ri, hite, hit(19), ri, hite, hith3 + hitt, ri, hiti, hit(19), ri, hite, hit(22), ri, hite, hitqd + hitt3, ri, hiti, hit(22), ri, hite, hit(19), ri, hite, hit(19), ri, hited, hit(19), ri, hiti, hitq + hitt, ri, hiti, hith3 + hitt, ri, hiti, hitq + hitt, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hit(19), ri, hite, hit(19), ri, hited, hit(19), ri, hiti, hitq + hitt, ri, hiti, hith3 + hitt, ri, hiti, hitq + hitt, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hitq, re, hite, hitq, re, hite, hitq, re, hite, hitq, re, hite, hit(22), re, hite, hit(22), re, hite, hit(22), re, hite, hit(22), re, hite, hit(19), re, hite, hit(19), re, hite, hit(19), re, hite, hit(19), re, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hiti, hited, ri, hite, hit(19), ri, hite, hit(19), ri, hited, hit(19), ri, hiti, hitq + hitt, ri, hiti, hith3 + hitt, ri, hiti, hitq + hitt, ri, hite, hit(22), ri, hite, hit(22), ri, hited, hit(22), ri, hiti, hith3, ri, hiti, hitqd + hitt3, ri, hiti, hith3, ri, hite, hitq, ri, hite, hitq, ri, hited, hitq, ri, hiti, hith3 + hitt3, ri, hiti, hitqd + hiti3, ri, hiti, hith3 + hitt3, ri, hite, hitq, ri, hite, hitq + hitt3, ri, hiti, hitq + hitt, ri, hite, hith3, ri, hite, hith3 + hitt3, ri, hiti, hitqd + hitt3, ri],
        ),
    ])
}

pub fn song_17() -> Song {
    song(1, &[
        section(
            &[],
            &[],
            &[],
            &[],
        ),
    ])
}

pub fn song_18() -> Song {
    song(1, &[
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
        2 => song_02(),
        3 => area_east(),
        4 => area_central(),
        5 => song_05(),
        6 => home_theme(),
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
    line(4, &[c10q, raw(162, 124), raw(162, 125), rest(58)])
}

pub fn sfx_01() -> Vec<Tok> {
    line(2, &[duty(36), volume(0), f7e, b6e, rq, c8e, raw(101, 1)])
}

pub fn sfx_02() -> Vec<Tok> {
    line(1, &[duty(4), volume(0), c5q, b4q, as4q, gs4q, fs4q, ds4q, c4q, fs3q, e3q])
}

pub fn sfx_char_select_open() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), raw(69, 1), as5q, g6q, ds6q, c7q, as6q])
}

pub fn sfx_char_select_close() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), a6q, b6q, cs7q, fs7q, e7q, a7q, e7q, fs7q])
}

pub fn sfx_05() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), f3q, a7q, fs3q, gs7q, g3q, g7q, gs3q, fs7q])
}

pub fn sfx_blocked() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), g2q, d3q, as2q, g3q, d4q, a3q, as3q])
}

pub fn sfx_07() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), e8q, ds8q, b2q, a2q, as2q])
}

pub fn sfx_08() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), d7q, fs7q, as7q, d8q, fs8q])
}

pub fn sfx_09() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), c5q, ds5q, fs5q, a5q, c6q, ds6q])
}

pub fn sfx_damage_bounce() -> Vec<Tok> {
    line(2, &[duty(36), volume(0), c6q, re, f6e, fs6e, f6e])
}

pub fn sfx_11() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), d5q, e5q, f5q, g5q, a5q, b5q, b4q, a4q, g4q, f4q, e4q, d4q, c4q])
}

pub fn sfx_cursor_select() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), c7q])
}

pub fn sfx_13() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), as9q, as8q])
}

pub fn sfx_14() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), g6q, c6q, g5q, c5q, g4q, c4q, g3q, c3q, g2q, c2q])
}

pub fn sfx_15() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), b7q, g7q, e7q, c7q, b6q, g6q, e6q, c6q])
}

pub fn sfx_16() -> Vec<Tok> {
    line(4, &[duty(37), volume(0), g4i, e5i, c6i, c5i, c7hd, rw + rh])
}

pub fn sfx_magic_pickup() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 gs6q d6q cs6q b5q c6q d6q fs6q b6q d7q fs7q b7q, 252 gs6q d6q cs6q b5q c6q d6q fs6q b6q d7q fs7q b7q a7q b7q)])
}

pub fn sfx_18() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 d5q fs5q a5q b5q fs5q a5q b5q d6q, 255 a5q b5q d6q g6q, 254 g5q b5q d6q f6q, 253 fs5q a5q b5q d6q, 252 g5q b5q d6q f6q, 251 f5q a5q d6q g6q, 250 f5q a5q d6q g6q, 249 f5q a5q d6q g6q, 248 f5q a5q d6q g6q, 247 f5q a5q d6q g6q, 246 f5q a5q d6q g6q, 245 f5q a5q d6q g6q)])
}

pub fn sfx_got_item() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 d6q fs6q a6q b6q b4q, 255 d5q fs5q d5q fs5q a5q b5q d6q b4q, 254 d5q fs5q d6q fs6q a6q as6q b6q)])
}

pub fn sfx_20() -> Vec<Tok> {
    line(2, &[duty(36), volume!(0 b2e g2e e2e c2e, 254 fs3q, 253 d3q cs3q b2q as2q a2q gs2q g2q, 252 fs3q, 251 d3q cs3q b2q as2q a2q gs2q g2q)])
}

pub fn sfx_key_pickup() -> Vec<Tok> {
    line(8, &[duty(36), volume!(0 fs6i b5i d6i a6i a7q, 252 fs6i b5i d6i a6i e7i a7h)])
}

pub fn sfx_22() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), g6q, d6q])
}

pub fn sfx_23() -> Vec<Tok> {
    line(2, &[duty(36), volume(0), g6q, d6q, f6q, a6q, b6q, d7q, g7q])
}

pub fn sfx_24() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 b7q as7q g7q e7q c7q g5q c5q b5q b7q g7q c4q d4q g3q e3q d3q g2q, 254 g5q c5q b5q b7q g7q c4q d4q g3q e3q d3q g2q, 252 g5q c5q b5q b7q g7q c4q d4q g3q e3q d3q g2q, 250 g5q c5q b5q b7q g7q c4q d4q g3q e3q d3q g2q, 248 g5q c5q b5q b7q g7q c4q d4q g3q e3q d3q g2q)])
}

pub fn sfx_fire() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 b5q as5q b5q a5q, 255 as5q gs5q a5q g5q, 254 gs5q fs5q g5q f5q, 253 fs5q e5q f5q ds5q, 252 e5q d5q ds5q cs5q, 251 d5q c5q cs5q b4q, 250 c5q as4q b4q a4q, 249 as4q gs4q a4q g4q, 248 gs4q fs4q g4q f4q, 247 fs4q e4q f4q, 246 ds4q e4q c4q ds4q c4q)])
}

pub fn sfx_low_magic() -> Vec<Tok> {
    line(4, &[duty(36), volume(0), e4q, cs5q, e4q, cs5q, e4q, cs5q, e4q, cs5q])
}

pub fn sfx_jump() -> Vec<Tok> {
    line(2, &[duty(36), volume!(0 fs4q g4q gs4q a4q, 255 as4q b4q c5q cs5q)])
}

pub fn sfx_password_error() -> Vec<Tok> {
    line(4, &[duty(36), volume(0), c6q, d6q, cs6q, ds6q, d6q, e6q, ds6q, f6q, e6q, fs6q, ds6q, f6q, d6q, e6q, cs6q, ds6q, c6q, d6q])
}

pub fn sfx_inventory_full() -> Vec<Tok> {
    line(2, &[duty(36), volume(0), ds5q, f5q, d5q, e5q, cs5q, ds5q, c5q, d5q, b4q, cs5q, as4q, c5q, cs5q, d5q, ds5q, e5q])
}

pub fn sfx_health_pickup() -> Vec<Tok> {
    line(4, &[duty(36), volume(0), g6i, g5i, d5i, c5i, rh, b6i, c5i, e5i, g5i, b5i])
}

pub fn sfx_31() -> Vec<Tok> {
    line(4, &[duty(36), volume!(0 e7q b6q, 254 e7q b6q, 252 e7q b6q, 250 e7q b6q, 248 e7q b6q, 246 e7q b6q, 244 e7q b6q)])
}

pub fn sfx_32() -> Vec<Tok> {
    line(2, &[duty(52), volume!(0 a3e as3e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e, 255 c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e c4e b3e, 254 c4e as3q b3q, 253 a3q as3q, 252 g3qd gs3qd, 251 f3q fs3q e3e), pitch(8), volume(250), e3e, pitch(16), e3e, pitch(32), volume(249), e3e, pitch!(40 e3e, 50 e3e, 0 f3e e3e ds3e d3e cs3e)])
}

pub fn sfx_hurt() -> Vec<Tok> {
    line(1, &[volume!(0 a4q a3q a2q a3q f3q b4q b4q d4q f5q b5q d6q f6q b5q d6q a6q, 253 f7q b6q d7q a7q, 251 f6q b5q d6q a6q, 249 f7q b6q d7q a7q f6q b6q d7q a7q e8q)])
}

pub fn sfx_fire_char0() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 c3q ds3q f3q fs3q g3q g4q d3q c3q b2q, 252 c3q ds3q f3q fs3q g3q g4q d3q c3q b2q, 249 c3q ds3q f3q fs3q g3q g4q d3q c3q b2q)])
}

pub fn sfx_fire_char1() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 b4q fs5q a5q a5q e5q fs5q a5q b5q fs5q fs5q a5q a5q fs5q, 255 f5q e5q, 254 ds5q, 253 g5q, 252 gs5q d5q d5q fs5q b5q b5q b5q a5q b5q, 250 e5q fs5q a5q b5q b5q b5q)])
}

pub fn sfx_fire_char2() -> Vec<Tok> {
    line(1, &[duty(36), volume(0), d6q, f6q, b6q, d7q, a7q, e8q])
}

pub fn sfx_fire_char3() -> Vec<Tok> {
    line(2, &[duty(36), volume!(0 b7q g6q g5q g4q g3q, 252 b7q g6q g5q g4q g3q, 249 b7q g6q g5q g4q g3q)])
}

pub fn sfx_fire_char4() -> Vec<Tok> {
    line(1, &[duty(36), volume!(0 b5q as5q b5q a5q, 255 as5q gs5q a5q g5q, 254 gs5q fs5q g5q f5q, 253 fs5q e5q f5q ds5q, 252 e5q d5q ds5q cs5q, 251 d5q c5q cs5q b4q, 250 c5q as4q b4q a4q, 249 as4q gs4q a4q g4q, 248 gs4q fs4q g4q f4q, 247 fs4q e4q f4q, 246 ds4q e4q c4q ds4q c4q)])
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
