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

/// One decoded channel-stream token.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tok {
    Note { dur: u8, pitch: u8 }, // dur has bit7 clear
    Rest { dur: u8 },            // 0..=127 (the stored byte was dur | 0x80)
    Cmd { id: u8, arg: u8 },
    End,
}

/// Re-emit the exact channel bytes for a token list.
pub fn assemble(toks: &[Tok]) -> Vec<u8> {
    let mut out = Vec::new();
    for t in toks {
        match *t {
            Tok::Note { dur, pitch } => {
                out.push(dur & 0x7F);
                out.push(pitch);
            }
            Tok::Rest { dur } => out.push((dur & 0x7F) | 0x80),
            Tok::Cmd { id, arg } => out.extend_from_slice(&[0xFF, id, arg]),
            Tok::End => out.push(0x00),
        }
    }
    out
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

/// A single DSL element: a pitched note, a rest, a raw (off-grid) note/rest, a
/// channel command, or a `Seq` — a spliced sub-sequence (e.g. an `env!`
/// envelope), so a multi-note run is still one `Note` and channels stay flat
/// `&[Note]`. The end-of-stream marker is implicit — `song`/`line` terminate
/// each non-empty channel automatically.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Note {
    Pitched { pitch: u8, val: Val },
    Rest { val: Val },
    RawNote { pitch: u8, ticks: u8 },
    RawRest { ticks: u8 },
    Cmd { id: u8, arg: u8 },
    Seq(&'static [Note]),
}

impl Note {
    fn emit(self, tempo: u32, out: &mut Vec<Tok>) {
        match self {
            Note::Pitched { pitch, val } => out.push(Tok::Note { dur: val.ticks(tempo), pitch }),
            Note::Rest { val } => out.push(Tok::Rest { dur: val.ticks(tempo) }),
            Note::RawNote { pitch, ticks } => out.push(Tok::Note { dur: ticks, pitch }),
            Note::RawRest { ticks } => out.push(Tok::Rest { dur: ticks }),
            Note::Cmd { id, arg } => out.push(Tok::Cmd { id, arg }),
            Note::Seq(notes) => {
                for &n in notes {
                    n.emit(tempo, out);
                }
            }
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
