//! Legacy of the Wizard music DSL + 2A03 channel-stream model.
//!
//! Factored into its own small crate so it compiles fast on its own (for
//! live-edit playback) while the game crate (`lotw`) depends on it. Songs are
//! written as composable [`Note`] values laid out in plain nested arrays:
//!
//! ```ignore
//! use lotw_music::{song, section, note::*};
//! pub fn home_theme() -> Song {
//!     song(32, &[                                   // 32 ticks per quarter
//!         section(
//!             &[duty(32), volume(255), c5q, d5e, ds5e],  // pulse1
//!             &[duty(32), g4q, as4e, c5e],               // pulse2
//!             &[c4i, c4i, c5i, c4i, ri, c4i],            // triangle
//!             &[],                                       // noise
//!         ),
//!     ])
//! }
//! ```

/// Channel names in header order (pulse1/pulse2/triangle/noise).
pub const CHANNEL_NAMES: [&str; 4] = ["pulse1", "pulse2", "triangle", "noise"];

pub mod note;

/// The `env!` parameter-envelope macro, plus `duty!`/`volume!`/`flags!`/
/// `pitch!`/`sweep!` (same thing with the parameter baked into the macro name;
/// these don't clash with the `duty()`/`volume()`/… note functions).
pub use lotw_music_macros::{duty, env, flags, pitch, sweep, volume};

/// Everything a song file needs in one glob: note symbols (`a3i`, `rq`), the
/// channel-command functions and envelope macros (`duty`/`duty!`/…), the
/// `section`/`song`/`line` builders and the core types. Song files just write
/// `use lotw_music::music::*;`.
pub mod music {
    pub use crate::note::*;
    pub use crate::{duty, env, flags, pitch, sweep, volume};
    pub use crate::{line, section, song, Note, Song, Tok, Val};
}

/// One decoded channel-stream token.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tok {
    Note { dur: u8, pitch: u8 }, // pitched note (pulse/triangle): 2 bytes (dur, pitch)
    Hit { dur: u8 },             // noise hit: 1 byte (dur only; the drum period is a command)
    Rest { dur: u8 },            // 0..=127 (the stored byte was dur | 0x80)
    Cmd { id: u8, arg: u8 },
    End,
    // Zero-byte markers (not emitted) recording the channel's loop behaviour for
    // the song header's per-channel loop pointer (bytes 4/5). See `loop_of`.
    LoopStart, // the byte the loop pointer should target (intros loop past their head)
    NoLoop,    // the channel doesn't loop — disable it on End (header loop-hi = 0)
}

/// Re-emit the exact channel bytes for a token list.
pub fn assemble(toks: &[Tok]) -> Vec<u8> {
    let mut out = Vec::new();
    for t in toks {
        match *t {
            Tok::Hit { dur } => out.push(dur & 0x7F),
            Tok::Note { dur, pitch } => {
                out.push(dur & 0x7F);
                out.push(pitch);
            }
            Tok::Rest { dur } => out.push((dur & 0x7F) | 0x80),
            Tok::Cmd { id, arg } => out.extend_from_slice(&[0xFF, id, arg]),
            Tok::End => out.push(0x00),
            Tok::LoopStart | Tok::NoLoop => {} // markers: no bytes
        }
    }
    out
}

/// Where a channel jumps when it hits End, recorded by the `LoopStart`/`NoLoop`
/// markers in its token stream (the song header's per-channel loop pointer).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Loop {
    To(usize), // loop back to this byte offset (0 = stream start, the default)
    None,      // don't loop — disable the channel on End
}

/// Resolve a token stream's loop target. A `NoLoop` marker → `None`; the first
/// `LoopStart` marker → `To(its byte offset)`; neither → `To(0)` (loop to start).
/// The byte offset matches `assemble`'s layout so it can be turned into the
/// header's absolute loop pointer.
pub fn loop_of(toks: &[Tok]) -> Loop {
    let mut off = 0usize;
    for t in toks {
        match t {
            Tok::NoLoop => return Loop::None,
            Tok::LoopStart => return Loop::To(off),
            Tok::Hit { .. } | Tok::Rest { .. } | Tok::End => off += 1,
            Tok::Note { .. } => off += 2,
            Tok::Cmd { .. } => off += 3,
        }
    }
    Loop::To(0)
}

/// A song: its four channel streams in header order (pulse1/pulse2/tri/noise),
/// plus, per channel, the token index where each score section begins (so a
/// player can map the playhead back to a source section for highlighting).
#[derive(Default, Clone)]
pub struct Song {
    pub channels: Vec<(String, Vec<Tok>)>,
    pub section_starts: [Vec<usize>; 4],
}

impl Song {
    pub fn add(&mut self, name: &str, stream: Vec<Tok>) {
        self.channels.push((name.to_string(), stream));
    }
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

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Sum two note values (`q + i` = a quarter plus a sixteenth = 5/4 of a quarter).
impl std::ops::Add for Val {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        let num = self.num as u32 * rhs.den as u32 + rhs.num as u32 * self.den as u32;
        let den = self.den as u32 * rhs.den as u32;
        let g = gcd(num, den).max(1);
        Val { num: (num / g) as u16, den: (den / g) as u16 }
    }
}

/// Tie notes of the same pitch: `a2q + a2i` holds `a2` for a quarter plus a
/// sixteenth (the left note's pitch is kept; the right's duration is added).
impl std::ops::Add for Note {
    type Output = Note;
    fn add(self, rhs: Note) -> Note {
        let rv = match rhs {
            Note::Pitched { val, .. } | Note::Rest { val } | Note::Hit { val } => val,
            _ => return self,
        };
        match self {
            Note::Pitched { pitch, val } => Note::Pitched { pitch, val: val + rv },
            Note::Rest { val } => Note::Rest { val: val + rv },
            Note::Hit { val } => Note::Hit { val: val + rv },
            other => other,
        }
    }
}

/// A single DSL element: a pitched note, a rest, a raw (off-grid) note/rest, a
/// channel command, or a `Seq` — a spliced sub-sequence (e.g. an `env!`
/// envelope), so a multi-note run is still one `Note` and channels stay flat
/// `&[Note]`. The end-of-stream marker is implicit — `song`/`line` terminate
/// each non-empty channel automatically.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Note {
    Pitched { pitch: u8, val: Val },
    Rest { val: Val },
    Hit { val: Val },     // noise drum hit (no pitch; the period is a command)
    RawNote { pitch: u8, ticks: u8 },
    RawRest { ticks: u8 },
    RawHit { ticks: u8 }, // off-grid noise hit
    Cmd { id: u8, arg: u8 },
    Seq(&'static [Note]),
    LoopStart, // marks the channel's loop target (see Tok::LoopStart)
    NoLoop,    // the channel doesn't loop (see Tok::NoLoop)
}

impl Note {
    fn emit(self, tempo: u32, out: &mut Vec<Tok>) {
        match self {
            Note::Pitched { pitch, val } => out.push(Tok::Note { dur: val.ticks(tempo), pitch }),
            Note::Rest { val } => out.push(Tok::Rest { dur: val.ticks(tempo) }),
            Note::Hit { val } => out.push(Tok::Hit { dur: val.ticks(tempo) }),
            Note::RawNote { pitch, ticks } => out.push(Tok::Note { dur: ticks, pitch }),
            Note::RawRest { ticks } => out.push(Tok::Rest { dur: ticks }),
            Note::RawHit { ticks } => out.push(Tok::Hit { dur: ticks }),
            Note::Cmd { id, arg } => out.push(Tok::Cmd { id, arg }),
            Note::Seq(notes) => {
                for &n in notes {
                    n.emit(tempo, out);
                }
            }
            Note::LoopStart => out.push(Tok::LoopStart),
            Note::NoLoop => out.push(Tok::NoLoop),
        }
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
/// each channel's fragments across sections. Each non-empty channel is
/// terminated with the implicit 0x00.
pub fn song(tempo: u32, sections: &[Section]) -> Song {
    let mut chans: [Vec<Tok>; 4] = Default::default();
    let mut starts: [Vec<usize>; 4] = Default::default();
    for sec in sections {
        for (ci, frag) in sec.channels.iter().enumerate() {
            starts[ci].push(chans[ci].len()); // this section begins at this token
            for n in *frag {
                n.emit(tempo, &mut chans[ci]);
            }
        }
    }
    let mut s = Song::default();
    s.section_starts = starts;
    for (i, mut c) in chans.into_iter().enumerate() {
        if !c.is_empty() {
            c.push(Tok::End);
        }
        s.add(CHANNEL_NAMES[i], c);
    }
    s
}

/// Assemble a single-channel stream (used for SFX) at `tempo`, terminated.
pub fn line(tempo: u32, notes: &[Note]) -> Vec<Tok> {
    let mut out = Vec::new();
    for n in notes {
        n.emit(tempo, &mut out);
    }
    if !out.is_empty() {
        out.push(Tok::End);
    }
    out
}
