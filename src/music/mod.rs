//! The Legacy of the Wizard music DSL.
//!
//! Music is written as composable [`Note`] values (imported from [`note`]) laid
//! out in plain nested arrays — no grouping macros:
//!
//! ```ignore
//! use crate::music::{song, section, note::*};
//! pub fn home_theme() -> Song {
//!     song(32, &[                                   // 32 ticks per quarter
//!         section(
//!             &[duty(0x20), volume(0xff), c5q, d5e, ds5e],  // pulse1
//!             &[duty(0x20), g4q, as4e, c5e],                // pulse2
//!             &[c4i, c4i, c5i, c4i, ri, c4i],               // triangle
//!             &[],                                          // noise
//!         ),
//!         section( /* next ~2 bars */ ),
//!     ])
//! }
//! ```
//!
//! A note's pitch is fixed but its duration is a *note value* (a fraction of a
//! quarter); the per-song `tempo` (ticks per quarter) turns it into the exact
//! frame count the ROM stores. Any `&[Note]` is a reusable phrase you can
//! `let`-bind and drop into several channels.

use crate::audio::{Song, Tok, CHANNEL_NAMES};

pub mod note;

#[cfg(has_music)]
mod songs;
#[cfg(has_music)]
pub use songs::*;

// Clean-checkout stub when `src/music/songs.rs` hasn't been generated.
#[cfg(not(has_music))]
pub fn get(_: usize) -> Option<Song> {
    None
}
#[cfg(not(has_music))]
pub fn sfx(_: usize) -> Option<Vec<Tok>> {
    None
}

/// A note value as a fraction of a quarter note (`num/den` quarters). The tick
/// count is `num * tempo / den`, which must come out a whole number ≤ 127.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Val {
    pub num: u16,
    pub den: u16,
}

impl Val {
    pub const fn ticks(self, tempo: u32) -> u8 {
        let t = self.num as u32 * tempo;
        debug_assert!(t % self.den as u32 == 0, "note value not a whole number of ticks at this tempo");
        (t / self.den as u32) as u8
    }
}

/// A single DSL element: a pitched note, a rest, a raw (off-grid) note/rest, a
/// channel command, or the end/loop marker.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Note {
    Pitched { pitch: u8, val: Val },
    Rest { val: Val },
    RawNote { pitch: u8, ticks: u8 },
    RawRest { ticks: u8 },
    Cmd { id: u8, arg: u8 },
    End,
}

impl Note {
    fn emit(self, tempo: u32, out: &mut Vec<Tok>) {
        out.push(match self {
            Note::Pitched { pitch, val } => Tok::Note { dur: val.ticks(tempo), pitch },
            Note::Rest { val } => Tok::Rest { dur: val.ticks(tempo) },
            Note::RawNote { pitch, ticks } => Tok::Note { dur: ticks, pitch },
            Note::RawRest { ticks } => Tok::Rest { dur: ticks },
            Note::Cmd { id, arg } => Tok::Cmd { id, arg },
            Note::End => Tok::End,
        });
    }
}

/// One score section: the four channels' fragments for a stretch of time.
pub struct Section<'a> {
    pub channels: [&'a [Note]; 4],
}

/// Build a section from its four channel fragments (`&[]` for a silent one).
pub fn section<'a>(pulse1: &'a [Note], pulse2: &'a [Note], triangle: &'a [Note], noise: &'a [Note]) -> Section<'a> {
    Section { channels: [pulse1, pulse2, triangle, noise] }
}

/// Assemble a song from its sections at `tempo` ticks per quarter, concatenating
/// each channel's fragments across sections.
pub fn song(tempo: u32, sections: &[Section]) -> Song {
    let mut chans: [Vec<Tok>; 4] = Default::default();
    for sec in sections {
        for (ci, frag) in sec.channels.iter().enumerate() {
            for n in *frag {
                n.emit(tempo, &mut chans[ci]);
            }
        }
    }
    let mut s = Song::default();
    for (i, c) in chans.into_iter().enumerate() {
        s.add(CHANNEL_NAMES[i], c);
    }
    s
}

/// Assemble a single-channel stream (used for SFX) at `tempo`.
pub fn line(tempo: u32, notes: &[Note]) -> Vec<Tok> {
    let mut out = Vec::new();
    for n in notes {
        n.emit(tempo, &mut out);
    }
    out
}
