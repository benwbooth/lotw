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

// --- proc-macro DSL generation (the `ser!`/`song!`/`param!`/... forms) ---
//
// `render_macro` emits the item syntax the proc macros in `lotw-music-macros`
// parse: `c4q` clean notes, `c4 30` raw-tick notes, `rq`/`r 30` rests,
// `param!(duty=0x0b, volume=0xff)` commands, `raw!(0x9f, e)` un-nameable
// pitches, and `|` for end of stream.

/// A duration as a proc-macro token. Returns `(text, joined)` where `joined`
/// means the text is a single letter that can be appended directly to a note
/// name (`c4q`); otherwise it needs a separating space (`c4 q~i`, `c4 30`):
///   * a single grid letter            -> `q`
///   * a tie of letters summing to it  -> `q~i` (any multiple of 3)
///   * a raw tick count                -> `30`
fn dur_token(dur: u8) -> (String, bool) {
    if let Some((d, _)) = DURS.iter().find(|(_, t)| *t == dur) {
        return ((*d).to_string(), true);
    }
    // Greedy decomposition into tied note-values (3 ticks is the smallest, so
    // any multiple of 3 decomposes exactly; everything else falls back to ticks).
    let mut rem = dur;
    let mut parts = Vec::new();
    for (name, t) in DURS {
        while rem >= *t {
            parts.push(*name);
            rem -= *t;
        }
    }
    if rem == 0 && parts.len() > 1 {
        (parts.join("~"), false)
    } else {
        (dur.to_string(), false)
    }
}

/// Note values (name, num/den of a quarter). Mirrors build.rs. The tick count
/// at tempo `q` is `num*q/den` (when whole). Ordered longest-first so the
/// generator prefers fewer, larger note values.
const VALS: &[(&str, u32, u32)] = &[
    ("w", 4, 1), ("hdd", 7, 2), ("hd", 3, 1), ("h", 2, 1), ("qdd", 7, 4), ("qd", 3, 2),
    ("q", 1, 1), ("edd", 7, 8), ("ed", 3, 4), ("e", 1, 2), ("id", 3, 8), ("i", 1, 4),
    ("td", 3, 16), ("t", 1, 8), ("x", 1, 16),
];

/// The note-value name whose tick count at tempo `q` equals `dur`, if any.
fn val_name(dur: u8, q: u32) -> Option<&'static str> {
    VALS.iter().find(|(_, n, d)| n * q % d == 0 && n * q / d == dur as u32).map(|(name, _, _)| *name)
}

/// Pick the tempo (ticks per quarter) that lets the most notes/rests in `toks`
/// render as named note values rather than `raw`/`rest` tick counts.
pub fn detect_tempo(channels: &[Vec<Tok>]) -> u32 {
    let durs: Vec<u8> = channels
        .iter()
        .flat_map(|c| c.iter())
        .filter_map(|t| match t {
            Tok::Note { dur, .. } | Tok::Rest { dur } => Some(*dur),
            _ => None,
        })
        .collect();
    (1..=96)
        .max_by_key(|&q| durs.iter().filter(|&&d| val_name(d, q).is_some()).count())
        .unwrap_or(24)
}

/// Render one token as a `note`-module DSL element at tempo `q`.
fn note_item(t: &Tok, q: u32) -> String {
    match *t {
        // Name it only when the pitch is a real chromatic note in the generated
        // octave range (nibble 0..=9 = octaves 2..=11); else keep the raw byte.
        Tok::Note { dur, pitch } => match (pitch_str(pitch), val_name(dur, q)) {
            (name, Some(v)) if !name.starts_with('~') && pitch >> 4 <= 9 => format!("{name}{v}"),
            _ => format!("raw({pitch}, {dur})"),
        },
        Tok::Rest { dur } => match val_name(dur, q) {
            Some(v) => format!("r{v}"),
            None => format!("rest({dur})"),
        },
        Tok::Cmd { id, arg } => match CMD_NAMES.get(id as usize) {
            Some(name) => format!("{name}({arg})"),
            None => format!("cmd({id}, {arg})"),
        },
        // The terminator is implicit (the builder appends it); it never reaches here.
        Tok::End => String::new(),
    }
}

/// Render a channel fragment as a `&[ items ]` slice literal at tempo `q`. The
/// stream terminator is dropped — `song`/`line` re-append it.
fn render_channel(frag: &[Tok], q: u32) -> String {
    let items: Vec<String> = frag.iter().filter(|t| !matches!(t, Tok::End)).map(|t| note_item(t, q)).collect();
    format!("&[{}]", items.join(", "))
}

/// Split a channel stream into score sections aligned to a fixed tick grid:
/// each token goes in the section of its start tick (commands/end inherit the
/// current position), so notes are never split and concatenating the fragments
/// reproduces the stream exactly. A note longer than a section simply leaves the
/// spanned later sections empty for that channel (it's being held).
pub fn split_sections(toks: &[Tok], section_ticks: u32) -> Vec<Vec<Tok>> {
    let mut secs: Vec<Vec<Tok>> = vec![Vec::new()];
    let mut cum = 0u32;
    for &t in toks {
        let si = (cum / section_ticks) as usize;
        while secs.len() <= si {
            secs.push(Vec::new());
        }
        secs[si].push(t);
        if let Tok::Note { dur, .. } | Tok::Rest { dur } = t {
            cum += dur as u32;
        }
    }
    secs
}

/// A song: its four channel streams in header order (pulse1/pulse2/tri/noise).
#[derive(Default, Clone)]
pub struct Song {
    pub channels: Vec<(String, Vec<Tok>)>,
}

impl Song {
    pub fn add(&mut self, name: &str, stream: Vec<Tok>) {
        self.channels.push((name.to_string(), stream));
    }
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

/// SFX pointer table (PRG offset; CPU $8014 in bank 10) and entry count. Each
/// entry points at a single pulse2 channel stream (same grammar as music).
const SFX_POINTER_TABLE: usize = 0x14014;
const SFX_COUNT: usize = 39;

/// Each sound effect as `(index, stream PRG offset)`.
pub fn sfx_streams(prg: &[u8]) -> Vec<(usize, usize)> {
    (0..SFX_COUNT)
        .filter_map(|i| {
            let addr = prg[SFX_POINTER_TABLE + i * 2] as usize | (prg[SFX_POINTER_TABLE + i * 2 + 1] as usize) << 8;
            addr_to_off(addr, 0x14000, 0x16000).map(|off| (i, off))
        })
        .collect()
}

/// A function name for a song index — descriptive where the context is known in
/// game.rs, else `song_NN`.
pub fn song_name(i: usize) -> String {
    match i {
        // Area themes, named by the world-map region whose rooms select them
        // (room descriptor +11); see the song-per-room map.
        0 => "area_north".into(),  // top rows
        1 => "area_west".into(),   // left column
        2 => "home_theme".into(),  // the Drasle family home / hub
        3 => "area_east".into(),   // right columns
        4 => "area_central".into(),
        // Jingles / fixed-context cues identified in game.rs.
        8 => "death_jingle".into(),
        9 => "title_theme".into(),
        10 => "ending_theme".into(),
        14 => "door_unlock_jingle".into(),
        _ => format!("song_{i:02}"),
    }
}

/// A function name for an SFX index — descriptive where its callers in game.rs
/// make the purpose clear, else `sfx_NN`.
pub fn sfx_name(i: usize) -> String {
    match i {
        3 => "sfx_char_select_open".into(),
        4 => "sfx_char_select_close".into(),
        6 => "sfx_blocked".into(),
        10 => "sfx_damage_bounce".into(),
        12 => "sfx_cursor_select".into(),
        17 => "sfx_magic_pickup".into(),
        19 => "sfx_got_item".into(),
        21 => "sfx_key_pickup".into(),
        25 => "sfx_fire".into(),
        26 => "sfx_low_magic".into(),
        27 => "sfx_jump".into(),
        28 => "sfx_password_error".into(),
        29 => "sfx_inventory_full".into(),
        30 => "sfx_health_pickup".into(),
        33 => "sfx_hurt".into(),
        34..=38 => format!("sfx_fire_char{}", i - 34),
        _ => format!("sfx_{i:02}"),
    }
}

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

/// Emit `src/music/songs.rs`: each song as a `song(tempo, &[section(..), ..])`
/// using the `note` DSL, each SFX as a `line(tempo, &[..])`. Byte-exact.
pub fn emit_music_rs(prg: &[u8]) -> String {
    // ~2 bars per section (96 ticks ≈ a 4/4 bar at quarter = 24).
    const SECTION_TICKS: u32 = 192;
    let mut out = String::new();
    out.push_str("//! Legacy of the Wizard songs + SFX as the music DSL — generated from the ROM\n");
    out.push_str("//! by `gen_music` (deterministic, byte-exact). Refine the notation freely; it\n");
    out.push_str("//! must still assemble to the same bytes (see `tests/audio_dsl.rs`).\n\n");
    out.push_str("#![allow(clippy::all)]\n");
    out.push_str("use super::note::*;\nuse super::{line, section, song};\nuse crate::audio::{Song, Tok};\n\n");

    out.push_str("// ===== songs =====\n\n");
    let songs = song_channels(prg);
    for (idx, chans) in &songs {
        let streams: Vec<Vec<Tok>> = chans.iter().map(|off| off.and_then(|o| disasm(prg, o)).unwrap_or_default()).collect();
        let tempo = detect_tempo(&streams);
        // Split each channel into tick-aligned sections.
        let secs: Vec<Vec<Vec<Tok>>> = streams.iter().map(|s| split_sections(s, SECTION_TICKS)).collect();
        let n = secs.iter().map(Vec::len).max().unwrap_or(0);
        out.push_str(&format!("pub fn {}() -> Song {{\n    song({tempo}, &[\n", song_name(*idx)));
        for k in 0..n {
            out.push_str("        section(\n");
            for frags in &secs {
                let frag = frags.get(k).map(Vec::as_slice).unwrap_or(&[]);
                out.push_str(&format!("            {},\n", render_channel(frag, tempo)));
            }
            out.push_str("        ),\n");
        }
        out.push_str("    ])\n}\n\n");
    }
    out.push_str("/// All songs by ROM index.\npub fn get(i: usize) -> Option<Song> {\n    Some(match i {\n");
    for (idx, _) in &songs {
        out.push_str(&format!("        {idx} => {}(),\n", song_name(*idx)));
    }
    out.push_str("        _ => return None,\n    })\n}\n\n");

    out.push_str("// ===== sound effects (one pulse2 stream each) =====\n\n");
    let sfx = sfx_streams(prg);
    for (i, off) in &sfx {
        let stream = disasm(prg, *off).unwrap_or_default();
        let tempo = detect_tempo(std::slice::from_ref(&stream));
        out.push_str(&format!("pub fn {}() -> Vec<Tok> {{\n    line({tempo}, {})\n}}\n\n", sfx_name(*i), render_channel(&stream, tempo)));
    }
    out.push_str("/// All sound effects by ROM index.\npub fn sfx(i: usize) -> Option<Vec<Tok>> {\n    Some(match i {\n");
    for (i, _) in &sfx {
        out.push_str(&format!("        {i} => {}(),\n", sfx_name(*i)));
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
