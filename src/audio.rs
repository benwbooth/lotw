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
    u8::from_str_radix(s.trim_start_matches('$'), 16).map_err(|_| format!("bad hex {s:?}"))
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

fn parse_note(tok: &str) -> Result<Tok, String> {
    let b = tok.as_bytes();
    if b.is_empty() || !(b[0] as char).is_ascii_alphabetic() {
        return Err(format!("bad note {tok:?}"));
    }
    // Note name = a letter, plus an optional 's' if that forms a real sharp.
    let two = tok.get(..2).filter(|s| NOTE_NAMES.contains(s));
    let (name, rest) = match two {
        Some(n) => (n, &tok[2..]),
        None => (&tok[..1], &tok[1..]),
    };
    let idx = NOTE_NAMES.iter().position(|n| *n == name && !n.is_empty()).ok_or_else(|| format!("unknown note {name:?}"))? as u8;
    // Octave = leading digits of the remainder; the rest is the duration.
    let split = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let (oct_s, dur_s) = rest.split_at(split);
    let octave: i32 = oct_s.parse().map_err(|_| format!("bad octave in {tok:?}"))?;
    let nibble = octave - BASE_OCTAVE;
    if !(0..=15).contains(&nibble) {
        return Err(format!("octave {octave} out of range in {tok:?}"));
    }
    Ok(Tok::Note { dur: parse_dur(dur_s)?, pitch: ((nibble as u8) << 4) | idx })
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
