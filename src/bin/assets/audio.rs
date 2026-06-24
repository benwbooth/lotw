//! 2A03 music streams <-> a note/command DSL (assets/audio.json).
//!
//! Per the project plan, DSL->binary is a deterministic, byte-exact compiler
//! (this module); binary->DSL musical transcription is an agent task. This
//! module ships the lossless low-level disassembler that bootstraps the DSL and
//! verifies the compiler round-trips, and the compiler that re-emits the exact
//! bytes. An agent later refines the token streams into musical notation that
//! still compiles to the same bytes.
//!
//! Stream grammar (per channel, decoded by tick_*_channel / dispatch_audio_
//! stream_command in src/game.rs):
//!   0x00            -> `end`            (end of stream: loop/stop)
//!   0xFF id arg     -> `cmd <name> arg` (id 0..4 = duty/volume/flags/pitch/sweep)
//!   d (bit7 set)    -> `rest <d&0x7f>`  (timed silence)
//!   d p (bit7 clr)  -> `note <d> <p>`   (duration d, pitch index p)
//!
//! Songs live in PRG bank pairs 10/11 (songs 0-9) and 12/13 (10-19), mapped at
//! $8000/$A000; each song's $8000-table pointer -> a 32-byte header (4 channels
//! x 8 bytes; bytes 2/3 = the channel's stream pointer). We disassemble every
//! reachable channel stream and, on build, re-emit each at its original offset
//! (unreached bytes -- headers, pointer tables, SFX, padding -- stay verbatim,
//! so the rebuild is byte-identical).

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;

const CMD_NAMES: [&str; 5] = ["duty", "volume", "flags", "pitch", "sweep"];
const SONGS_PER_TABLE: usize = 10;
/// (song-pointer table PRG offset, $8000 bank base, $A000 bank base).
const PAIRS: &[(usize, usize, usize)] = &[(0x14000, 0x14000, 0x16000), (0x18000, 0x18000, 0x1A000)];

#[derive(Serialize, Deserialize)]
struct AudioFile {
    note: String,
    streams: Vec<StreamRec>,
}

#[derive(Serialize, Deserialize)]
struct StreamRec {
    label: String,
    prg_offset: usize,
    tokens: Vec<String>,
}

fn addr_to_off(addr: usize, base_lo: usize, base_hi: usize) -> Option<usize> {
    if (0x8000..0xA000).contains(&addr) {
        Some(base_lo + addr - 0x8000)
    } else if (0xA000..0xC000).contains(&addr) {
        Some(base_hi + addr - 0xA000)
    } else {
        None
    }
}

pub fn extract(prg: &[u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut seen = BTreeSet::new();
    let mut streams = Vec::new();
    for (pi, &(table, base_lo, base_hi)) in PAIRS.iter().enumerate() {
        for song in 0..SONGS_PER_TABLE {
            let hdr_addr = prg[table + song * 2] as usize | (prg[table + song * 2 + 1] as usize) << 8;
            let Some(hdr) = addr_to_off(hdr_addr, base_lo, base_hi) else { continue };
            if hdr + 32 > prg.len() {
                continue;
            }
            for ch in 0..4 {
                let s = hdr + ch * 8;
                let sp = prg[s + 2] as usize | (prg[s + 3] as usize) << 8;
                let Some(off) = addr_to_off(sp, base_lo, base_hi) else { continue };
                if !seen.insert(off) {
                    continue;
                }
                if let Some(tokens) = disasm(prg, off) {
                    streams.push(StreamRec {
                        label: format!("song{:02}_ch{ch}", pi * SONGS_PER_TABLE + song),
                        prg_offset: off,
                        tokens,
                    });
                }
            }
        }
    }
    streams.sort_by_key(|s| s.prg_offset);
    let file = AudioFile {
        note: "2A03 channel streams. tokens: `note <dur> <pitch>`, `rest <dur>`, \
               `cmd <duty|volume|flags|pitch|sweep|N> <arg>`, `end`. Compiles back \
               byte-exact at prg_offset; refine into musical notation as desired."
            .into(),
        streams,
    };
    fs::write(dir.join("audio.json"), serde_json::to_string_pretty(&file)? + "\n")?;
    Ok(())
}

/// Disassemble one channel stream from `off` until the `end` (0x00) byte.
fn disasm(prg: &[u8], mut off: usize) -> Option<Vec<String>> {
    let mut tokens = Vec::new();
    for _ in 0..8192 {
        let b = *prg.get(off)?;
        if b == 0x00 {
            tokens.push("end".into());
            return Some(tokens);
        } else if b == 0xFF {
            let id = *prg.get(off + 1)?;
            let arg = *prg.get(off + 2)?;
            let name = CMD_NAMES.get(id as usize).map(|s| s.to_string()).unwrap_or_else(|| id.to_string());
            tokens.push(format!("cmd {name} {arg:#04x}"));
            off += 3;
        } else if b & 0x80 != 0 {
            tokens.push(format!("rest {}", b & 0x7F));
            off += 1;
        } else {
            let p = *prg.get(off + 1)?;
            tokens.push(format!("note {b} {p:#04x}"));
            off += 2;
        }
    }
    None
}

pub fn apply(prg: &mut [u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let path = dir.join("audio.json");
    if !path.exists() {
        return Ok(());
    }
    let file: AudioFile = serde_json::from_str(&fs::read_to_string(&path)?)?;
    for s in &file.streams {
        let bytes = assemble(&s.tokens, &s.label)?;
        prg[s.prg_offset..s.prg_offset + bytes.len()].copy_from_slice(&bytes);
    }
    Ok(())
}

fn assemble(tokens: &[String], label: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut out = Vec::new();
    for tok in tokens {
        let p: Vec<&str> = tok.split_whitespace().collect();
        match p.as_slice() {
            ["end"] => out.push(0x00),
            ["rest", d] => out.push(parse_u8(d)? & 0x7F | 0x80),
            ["note", d, pitch] => {
                let dur = parse_u8(d)?;
                if dur & 0x80 != 0 {
                    return Err(format!("{label}: note duration {dur} has bit7 set").into());
                }
                out.push(dur);
                out.push(parse_u8(pitch)?);
            }
            ["cmd", name, arg] => {
                let id = CMD_NAMES.iter().position(|n| n == name).map(|i| i as u8);
                let id = id.map(Ok).unwrap_or_else(|| parse_u8(name))?;
                out.push(0xFF);
                out.push(id);
                out.push(parse_u8(arg)?);
            }
            _ => return Err(format!("{label}: bad token {tok:?}").into()),
        }
    }
    Ok(out)
}

fn parse_u8(s: &str) -> Result<u8, Box<dyn Error>> {
    let v = if let Some(h) = s.strip_prefix("0x") {
        u16::from_str_radix(h, 16)?
    } else {
        s.parse::<u16>()?
    };
    if v > 0xFF {
        return Err(format!("value {s} > 0xFF").into());
    }
    Ok(v as u8)
}
