//! Asset extraction / rebuild tool for the LotW ROM.
//!
//! The ROM is the single binary artifact; this tool extracts its assets into
//! editable, human-readable files under `assets/`, and rebuilds a byte-identical
//! ROM from them. The rebuild asserts the result matches the original ROM
//! exactly (sha + first-diff), so the extraction is provably lossless and the
//! existing FCEUX oracle / faithfulness checks keep applying unchanged.
//!
//! Usage:
//!   assettool extract [rom] [assets_dir]
//!   assettool build   [assets_dir] [out_rom] [reference_rom]
//!
//! Defaults: rom=rom/lotw.nes, assets_dir=assets, out_rom=build/rebuilt.nes.
//!
//! Built with `--features assets`.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

mod assets;

const HEADER_LEN: usize = 16;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    match cmd {
        "extract" => {
            let rom = args.get(2).map(String::as_str).unwrap_or("rom/lotw.nes");
            let dir = args.get(3).map(String::as_str).unwrap_or("assets");
            extract(rom, dir)
        }
        "build" => {
            let dir = args.get(2).map(String::as_str).unwrap_or("assets");
            let out = args.get(3).map(String::as_str).unwrap_or("build/rebuilt.nes");
            let reference = args.get(4).map(String::as_str).unwrap_or("rom/lotw.nes");
            build(dir, out, reference)
        }
        _ => {
            eprintln!(
                "usage:\n  assettool extract [rom] [assets_dir]\n  assettool build [assets_dir] [out_rom] [reference_rom]"
            );
            std::process::exit(2);
        }
    }
}

/// iNES layout of a ROM image: header + PRG + CHR byte ranges.
struct Ines {
    header: Vec<u8>,
    prg: Vec<u8>,
    chr: Vec<u8>,
}

fn parse_ines(rom: &[u8]) -> Result<Ines, Box<dyn Error>> {
    if rom.len() < HEADER_LEN || &rom[0..3] != b"NES" {
        return Err("not an iNES file".into());
    }
    let prg_len = rom[4] as usize * 16_384;
    let chr_len = rom[5] as usize * 8_192;
    if rom.len() < HEADER_LEN + prg_len + chr_len {
        return Err("short ROM".into());
    }
    Ok(Ines {
        header: rom[0..HEADER_LEN].to_vec(),
        prg: rom[HEADER_LEN..HEADER_LEN + prg_len].to_vec(),
        chr: rom[HEADER_LEN + prg_len..HEADER_LEN + prg_len + chr_len].to_vec(),
    })
}

fn extract(rom_path: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let rom = fs::read(rom_path)?;
    let ines = parse_ines(&rom)?;
    let dir = PathBuf::from(dir);
    fs::create_dir_all(&dir)?;

    // Header + raw PRG are stored verbatim. As data regions are extracted into
    // structured files (palettes, text, rooms, ...), the build overlays them
    // onto this PRG image; un-extracted bytes (code + not-yet-extracted data)
    // come straight from here, so the rebuild stays byte-identical throughout.
    fs::write(dir.join("header.bin"), &ines.header)?;
    fs::write(dir.join("prg.bin"), &ines.prg)?;

    // CHR graphics -> indexed PNG sheets (lossless: each pixel is its 2-bit
    // pattern value 0..3).
    assets::chr::extract(&ines.chr, &dir.join("chr"))?;
    // Palette tables -> palettes.json (indices + NES RGB).
    assets::palettes::extract(&ines.prg, &dir)?;
    // Text/nametable templates -> font.json + text.json.
    assets::text::extract(&ines.prg, &dir)?;

    println!(
        "extracted -> {} (prg {} B, chr {} tiles)",
        dir.display(),
        ines.prg.len(),
        ines.chr.len() / 16
    );
    Ok(())
}

fn build(dir: &str, out_path: &str, reference: &str) -> Result<(), Box<dyn Error>> {
    let dir = PathBuf::from(dir);
    let header = fs::read(dir.join("header.bin"))?;
    // Start from the raw PRG, then overlay each structured data region edited in
    // assets/ (palettes today; text/rooms/objects/audio as they land).
    let mut prg = fs::read(dir.join("prg.bin"))?;
    assets::palettes::apply(&mut prg, &dir)?;
    assets::text::apply(&mut prg, &dir)?;
    let chr = assets::chr::build(&dir.join("chr"))?;

    let mut rom = Vec::with_capacity(header.len() + prg.len() + chr.len());
    rom.extend_from_slice(&header);
    rom.extend_from_slice(&prg);
    rom.extend_from_slice(&chr);

    if let Some(parent) = Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_path, &rom)?;

    // Bit-for-bit gate: the rebuilt ROM must equal the reference exactly.
    let want = fs::read(reference)?;
    verify(&rom, &want, out_path)
}

fn verify(got: &[u8], want: &[u8], out_path: &str) -> Result<(), Box<dyn Error>> {
    if got == want {
        println!("BUILD OK: {} is byte-identical to the reference ROM", out_path);
        return Ok(());
    }
    if got.len() != want.len() {
        return Err(format!("length mismatch: rebuilt {} vs reference {}", got.len(), want.len()).into());
    }
    let mut diffs = 0usize;
    let mut first = None;
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        if g != w {
            if first.is_none() {
                first = Some(i);
            }
            diffs += 1;
        }
    }
    let i = first.unwrap();
    Err(format!(
        "BUILD MISMATCH: {diffs} bytes differ; first at offset {i:#x} (got {:#04x}, want {:#04x})",
        got[i], want[i]
    )
    .into())
}
