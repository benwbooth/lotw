//! LotW 2A03 music: byte-exact disassembly of channel streams to a CP437-style
//! note DSL, and back.
//!
//! The sound engine (see `dispatch_audio_stream_command` / `load_note_period`
//! in game.rs) decodes each channel stream byte-by-byte:
//!   * `0x00`            -> end of stream (loop/stop)
//!   * `0xFF id arg`     -> command (id 0..4 = duty/volume/flags/pitch/sweep)
//!   * `d` (bit7 set)    -> rest of `d & 0x7F` ticks
//!   * `d p` (bit7 clear)-> note: `d` ticks at pitch byte `p`
//!
//! A note's pitch byte `p` is `(octave_nibble << 4) | note_idx`. `note_idx`
//! indexes the 13-entry chromatic period table at `$FDB1` (idx 5 is an unused
//! gap, idx 0..=12 = C..B of a base octave starting at C2); each octave nibble
//! step halves the period (raises one octave). So pitch `0x00` = C2, `0x10` =
//! C3, `0x0C` = B2, `0x2C` = B4.
//!
//! The DSL is a whitespace-separated token stream, e.g.
//!   `duty=$0b volume=$ff c3hd c4hd g4e f4e g4e | ...`
//! Every token maps 1:1 to bytes, so `assemble(parse(render(disasm(b)))) == b`.

/// note_idx -> note name. Index 5 is the unused gap (E#/Fb position).
const NOTE_NAMES: [&str; 13] = ["c", "cs", "d", "ds", "e", "", "f", "fs", "g", "gs", "a", "as", "b"];
/// Command ids 0..4 (the `0xFF id arg` form).
pub const CMD_NAMES: [&str; 5] = ["duty", "volume", "flags", "pitch", "sweep"];
/// The base octave of `note_idx` 0 (period table starts at C2).
const BASE_OCTAVE: i32 = 2;
/// Duration letter <-> tick count (96 ticks per whole note; `d`/`dd` = dotted).
const DURS: &[(&str, u8)] = &[
    ("w", 96), ("hdd", 84), ("hd", 72), ("h", 48), ("qdd", 42), ("qd", 36),
    ("q", 24), ("edd", 21), ("ed", 18), ("e", 12), ("id", 9), ("i", 6), ("t", 3),
];

/// One decoded stream token.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tok {
    Note { dur: u8, pitch: u8 }, // dur has bit7 clear
    Rest { dur: u8 },            // 0..=127 (the stored byte was dur | 0x80)
    Cmd { id: u8, arg: u8 },
    End,
}

/// Disassemble one channel stream starting at PRG offset `off`, up to and
/// including the terminating `end`. Returns None on a truncated/runaway stream.
pub fn disasm(prg: &[u8], mut off: usize) -> Option<Vec<Tok>> {
    let mut out = Vec::new();
    for _ in 0..8192 {
        let b = *prg.get(off)?;
        if b == 0x00 {
            out.push(Tok::End);
            return Some(out);
        } else if b == 0xFF {
            out.push(Tok::Cmd { id: *prg.get(off + 1)?, arg: *prg.get(off + 2)? });
            off += 3;
        } else if b & 0x80 != 0 {
            out.push(Tok::Rest { dur: b & 0x7F });
            off += 1;
        } else {
            out.push(Tok::Note { dur: b, pitch: *prg.get(off + 1)? });
            off += 2;
        }
    }
    None
}

/// Re-emit the exact bytes for a token list (inverse of `disasm`).
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

fn dur_str(d: u8) -> String {
    match DURS.iter().find(|(_, t)| *t == d) {
        Some((s, _)) => (*s).to_string(),
        None => format!("={d}"),
    }
}

/// Render a pitch byte as a note name + octave, or `~XX` for the cases that
/// aren't a plain chromatic note (idx 5, idx 13..15, e.g. noise periods).
fn pitch_str(p: u8) -> String {
    let idx = (p & 0x0F) as usize;
    let name = NOTE_NAMES.get(idx).copied().unwrap_or("");
    if name.is_empty() {
        format!("~{p:02x}")
    } else {
        format!("{name}{}", BASE_OCTAVE + (p >> 4) as i32)
    }
}

/// Render a token stream to the DSL text form.
pub fn render(toks: &[Tok]) -> String {
    toks.iter()
        .map(|t| match *t {
            Tok::Note { dur, pitch } => format!("{}{}", pitch_str(pitch), dur_str(dur)),
            Tok::Rest { dur } => format!("r{}", dur_str(dur)),
            Tok::Cmd { id, arg } => {
                let name = CMD_NAMES.get(id as usize).map(|s| s.to_string()).unwrap_or_else(|| format!("cmd{id}"));
                format!("{name}=${arg:02x}")
            }
            Tok::End => "|".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Parse a duration suffix (`q`, `ed`, `=30`) into a tick count (0..=127).
fn parse_dur(s: &str) -> Result<u8, String> {
    if let Some(n) = s.strip_prefix('=') {
        let v: u16 = n.parse().map_err(|_| format!("bad duration {s:?}"))?;
        if v > 0x7F {
            return Err(format!("duration {v} > 127"));
        }
        Ok(v as u8)
    } else {
        DURS.iter().find(|(k, _)| *k == s).map(|(_, t)| *t).ok_or_else(|| format!("unknown duration {s:?}"))
    }
}

fn parse_u8_hex(s: &str) -> Result<u8, String> {
    let h = s.strip_prefix('$').or_else(|| s.strip_prefix("0x")).unwrap_or(s);
    u8::from_str_radix(h, 16).map_err(|_| format!("bad hex {s:?}"))
}

/// Parse the DSL text back into tokens.
pub fn parse(text: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    for tok in text.split_whitespace() {
        if tok == "|" {
            out.push(Tok::End);
            continue;
        }
        // Command: a known name (or cmdN) followed by `=`.
        if let Some((name, arg)) = tok.split_once('=') {
            let id = CMD_NAMES.iter().position(|n| *n == name).map(|i| i as u8).or_else(|| name.strip_prefix("cmd").and_then(|d| d.parse().ok()));
            if let Some(id) = id {
                out.push(Tok::Cmd { id, arg: parse_u8_hex(arg)? });
                continue;
            }
        }
        // Rest: `r<dur>`.
        if let Some(rest) = tok.strip_prefix('r') {
            // Guard against a note name 'r' (none exists) — any `r...` is a rest.
            out.push(Tok::Rest { dur: parse_dur(rest)? });
            continue;
        }
        // Raw note: `~XX<dur>`.
        if let Some(rest) = tok.strip_prefix('~') {
            if rest.len() < 2 {
                return Err(format!("bad raw note {tok:?}"));
            }
            let pitch = parse_u8_hex(&rest[..2])?;
            out.push(Tok::Note { dur: parse_dur(&rest[2..])?, pitch });
            continue;
        }
        // Melodic note: <name><octave><dur>.
        out.push(parse_note(tok)?);
    }
    Ok(out)
}

/// Split a melodic note name (`c`, `cs`, ... `b`) off the front, returning its
/// `note_idx` and the remainder (octave [+ duration]).
fn split_name(tok: &str) -> Result<(u8, &str), String> {
    let b = tok.as_bytes();
    if b.is_empty() || !(b[0] as char).is_ascii_alphabetic() {
        return Err(format!("bad note {tok:?}"));
    }
    // Note name = a letter, plus an optional 's' if that forms a real sharp.
    let (name, rest) = match tok.get(..2).filter(|s| NOTE_NAMES.contains(s)) {
        Some(n) => (n, &tok[2..]),
        None => (&tok[..1], &tok[1..]),
    };
    let idx = NOTE_NAMES.iter().position(|n| *n == name && !n.is_empty()).ok_or_else(|| format!("unknown note {name:?}"))?;
    Ok((idx as u8, rest))
}

fn octave_to_nibble(octave: i32, tok: &str) -> Result<u8, String> {
    let nibble = octave - BASE_OCTAVE;
    if (0..=15).contains(&nibble) {
        Ok(nibble as u8)
    } else {
        Err(format!("octave {octave} out of range in {tok:?}"))
    }
}

fn parse_note(tok: &str) -> Result<Tok, String> {
    let (idx, rest) = split_name(tok)?;
    // Octave = leading digits of the remainder; the rest is the duration.
    let split = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let (oct_s, dur_s) = rest.split_at(split);
    let octave: i32 = oct_s.parse().map_err(|_| format!("bad octave in {tok:?}"))?;
    Ok(Tok::Note { dur: parse_dur(dur_s)?, pitch: (octave_to_nibble(octave, tok)? << 4) | idx })
}

// --- `play!` macro support: one token (`tt`) per stream element ---
//
// The macro stringifies each token tree and hands it here. Forms:
//   `c4q`, `fs5e`, `rhd`   bare idents: a clean note/rest (letter duration)
//   `[c4:30]`, `[r:30]`    a note/rest whose duration isn't a letter (raw ticks)
//   `[duty:0x0b]`          a command (`duty/volume/flags/pitch/sweep/cmdN`)
//   `[~ff:e]`              a raw pitch byte
//   `|`                    end of stream
// Bracket contents are split on `:`/whitespace so both `[duty:0x0b]` and the
// macro's stringified `[duty : 0x0b]` parse the same.

/// Parse one `play!` token and push it onto `v`.
pub fn push_tok(v: &mut Vec<Tok>, tok: &str) -> Result<(), String> {
    let t = tok.trim();
    if t == "|" {
        v.push(Tok::End);
        return Ok(());
    }
    if let Some(inner) = t.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let parts: Vec<&str> = inner.split([':', ' ', '\t']).filter(|p| !p.is_empty()).collect();
        let [head, val] = parts.as_slice() else {
            return Err(format!("bad token {tok:?}"));
        };
        // Command?
        let cmd_id = CMD_NAMES.iter().position(|n| n == head).map(|i| i as u8).or_else(|| head.strip_prefix("cmd").and_then(|d| d.parse().ok()));
        if let Some(id) = cmd_id {
            v.push(Tok::Cmd { id, arg: parse_u8_hex(val)? });
        } else if *head == "r" {
            v.push(Tok::Rest { dur: parse_dur_value(val)? });
        } else if let Some(hex) = head.strip_prefix('~') {
            v.push(Tok::Note { dur: parse_dur_value(val)?, pitch: parse_u8_hex(hex)? });
        } else {
            // `[c4:30]` raw-duration melodic note.
            let p = note_pitch(head)?;
            v.push(Tok::Note { dur: parse_dur_value(val)?, pitch: p });
        }
        return Ok(());
    }
    // Bare ident: a clean note/rest (single text token).
    v.extend(parse(t)?);
    Ok(())
}

/// A duration value inside `[..]`: a tick count (decimal/hex) or a letter.
fn parse_dur_value(s: &str) -> Result<u8, String> {
    if let Some(h) = s.strip_prefix("0x") {
        let v = u16::from_str_radix(h, 16).map_err(|_| format!("bad dur {s:?}"))?;
        return if v <= 0x7F { Ok(v as u8) } else { Err(format!("dur {v} > 127")) };
    }
    if let Ok(v) = s.parse::<u16>() {
        return if v <= 0x7F { Ok(v as u8) } else { Err(format!("dur {v} > 127")) };
    }
    DURS.iter().find(|(k, _)| *k == s).map(|(_, t)| *t).ok_or_else(|| format!("bad dur {s:?}"))
}

/// Pitch byte for a bare melodic note name+octave like `c4`, `fs5`, `b2`.
fn note_pitch(s: &str) -> Result<u8, String> {
    let (idx, oct_s) = split_name(s)?;
    let octave: i32 = oct_s.parse().map_err(|_| format!("bad octave in {s:?}"))?;
    Ok((octave_to_nibble(octave, s)? << 4) | idx)
}

/// Render tokens as a `play!`-compatible token sequence (idents for clean
/// notes/rests, `[name:value]` groups for commands and raw durations).
pub fn render_play(toks: &[Tok]) -> String {
    toks.iter()
        .map(|t| match *t {
            Tok::Note { dur, pitch } => {
                let p = pitch_str(pitch); // `c4` or `~ff`
                match DURS.iter().find(|(_, tk)| *tk == dur) {
                    Some((d, _)) if !p.starts_with('~') => format!("{p}{d}"),
                    _ => format!("[{p}:{dur}]"),
                }
            }
            Tok::Rest { dur } => match DURS.iter().find(|(_, tk)| *tk == dur) {
                Some((d, _)) => format!("r{d}"),
                None => format!("[r:{dur}]"),
            },
            Tok::Cmd { id, arg } => {
                let name = CMD_NAMES.get(id as usize).map(|s| s.to_string()).unwrap_or_else(|| format!("cmd{id}"));
                format!("[{name}:{arg:#04x}]")
            }
            Tok::End => "|".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A song: its named channel streams, in header order (pulse1/pulse2/tri/noise).
#[derive(Default, Clone)]
pub struct Song {
    pub channels: Vec<(String, Vec<Tok>)>,
}

impl Song {
    pub fn add(&mut self, name: &str, stream: Vec<Tok>) {
        self.channels.push((name.to_string(), stream));
    }
}

/// `play! { pulse1 { c4q ... | } pulse2 { ... | } ... }` -> a [`Song`].
#[macro_export]
macro_rules! play {
    ( $( $ch:ident { $($t:tt)* } )* ) => {{
        let mut song = $crate::audio::Song::default();
        $(
            let mut stream = ::std::vec::Vec::new();
            $( $crate::audio::push_tok(&mut stream, stringify!($t)).expect("play! token"); )*
            song.add(stringify!($ch), stream);
        )*
        song
    }};
}

// --- ROM song layout (for enumerating streams) ---

const SONGS_PER_TABLE: usize = 10;
/// (song-pointer table PRG offset, $8000 bank base, $A000 bank base).
const PAIRS: &[(usize, usize, usize)] = &[(0x14000, 0x14000, 0x16000), (0x18000, 0x18000, 0x1A000)];

fn addr_to_off(addr: usize, base_lo: usize, base_hi: usize) -> Option<usize> {
    match addr {
        0x8000..0xA000 => Some(base_lo + addr - 0x8000),
        0xA000..0xC000 => Some(base_hi + addr - 0xA000),
        _ => None,
    }
}

/// Channel names in header order.
pub const CHANNEL_NAMES: [&str; 4] = ["pulse1", "pulse2", "triangle", "noise"];

/// Per song: its four channel stream offsets (None if the pointer is out of the
/// mapped range). Songs are not deduplicated (each lists its own channels).
pub fn song_channels(prg: &[u8]) -> Vec<(usize, [Option<usize>; 4])> {
    let mut out = Vec::new();
    for (pi, &(table, base_lo, base_hi)) in PAIRS.iter().enumerate() {
        for song in 0..SONGS_PER_TABLE {
            let hdr_addr = prg[table + song * 2] as usize | (prg[table + song * 2 + 1] as usize) << 8;
            let Some(hdr) = addr_to_off(hdr_addr, base_lo, base_hi) else { continue };
            if hdr + 32 > prg.len() {
                continue;
            }
            let mut chans = [None; 4];
            for (ch, slot) in chans.iter_mut().enumerate() {
                let sp = prg[hdr + ch * 8 + 2] as usize | (prg[hdr + ch * 8 + 3] as usize) << 8;
                *slot = addr_to_off(sp, base_lo, base_hi);
            }
            out.push((pi * SONGS_PER_TABLE + song, chans));
        }
    }
    out
}

/// Emit a `music.rs` source file: one `play!`-DSL function per song, byte-exact.
pub fn emit_music_rs(prg: &[u8]) -> String {
    let mut out = String::new();
    out.push_str("//! Legacy of the Wizard music as a `play!` DSL — generated from the ROM by\n");
    out.push_str("//! `gen_music` (deterministic, byte-exact). Each function round-trips to the\n");
    out.push_str("//! original channel-stream bytes. Refine the notation freely; it must still\n");
    out.push_str("//! compile to the same bytes (see `tests/audio_dsl.rs`).\n\n");
    out.push_str("use crate::audio::Song;\nuse crate::play;\n\n");
    let songs = song_channels(prg);
    for (song, chans) in &songs {
        out.push_str(&format!("pub fn song{song:02}() -> Song {{\n    play! {{\n"));
        for (ci, off) in chans.iter().enumerate() {
            let body = off
                .and_then(|o| disasm(prg, o))
                .map(|t| render_play(&t))
                .unwrap_or_default();
            out.push_str(&format!("        {} {{ {body} }}\n", CHANNEL_NAMES[ci]));
        }
        out.push_str("    }\n}\n\n");
    }
    // Index dispatch.
    out.push_str("/// All songs by index (matches the ROM's song order).\npub fn song(i: usize) -> Option<Song> {\n    Some(match i {\n");
    for (song, _) in &songs {
        out.push_str(&format!("        {song} => song{song:02}(),\n"));
    }
    out.push_str("        _ => return None,\n    })\n}\n");
    out
}

/// Every reachable channel stream: `(song_index, channel 0..3, PRG offset)`.
/// Channels: 0 = pulse1, 1 = pulse2, 2 = triangle, 3 = noise.
pub fn song_streams(prg: &[u8]) -> Vec<(usize, usize, usize)> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (pi, &(table, base_lo, base_hi)) in PAIRS.iter().enumerate() {
        for song in 0..SONGS_PER_TABLE {
            let hdr_addr = prg[table + song * 2] as usize | (prg[table + song * 2 + 1] as usize) << 8;
            let Some(hdr) = addr_to_off(hdr_addr, base_lo, base_hi) else { continue };
            if hdr + 32 > prg.len() {
                continue;
            }
            for ch in 0..4 {
                let sp = prg[hdr + ch * 8 + 2] as usize | (prg[hdr + ch * 8 + 3] as usize) << 8;
                let Some(off) = addr_to_off(sp, base_lo, base_hi) else { continue };
                if seen.insert(off) {
                    out.push((pi * SONGS_PER_TABLE + song, ch, off));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsl_round_trips() {
        // C2, B2, B4, a rest, a command, end.
        let toks = vec![
            Tok::Cmd { id: 0, arg: 0x0b },
            Tok::Note { dur: 72, pitch: 0x00 }, // c2 (hd)
            Tok::Note { dur: 24, pitch: 0x0c }, // b2 (q)
            Tok::Note { dur: 30, pitch: 0x2c }, // b4 (=30)
            Tok::Rest { dur: 48 },
            Tok::Note { dur: 12, pitch: 0xff }, // raw (~ff)
            Tok::End,
        ];
        let text = render(&toks);
        assert_eq!(text, "duty=$0b c2hd b2q b4=30 rh ~ffe |");
        assert_eq!(parse(&text).unwrap(), toks);
    }
}
