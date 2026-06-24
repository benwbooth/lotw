//! PPU smoke test that exercises the renderer with synthetic VRAM/OAM contents.
//!
//! This driver does not run the game. It loads CHR data from the ROM, dumps both
//! pattern-table tile sheets to PPM images, then hand-fills the nametable,
//! attribute table, palette, and sprite OAM with a known test pattern and
//! renders one frame. It is used to sanity-check the standalone PPU "shim"
//! renderer (tiles, attributes, palettes, and sprites) in isolation.
//!
//! Usage: `ppu_shim_test [rom]` (ROM defaults to `rom/lotw.nes`).

mod common;

use std::{env, error::Error};

use lotw::{PPU_H, PPU_W};

/// Width/height in pixels of a CHR pattern-table tile sheet (16x16 tiles, 8px each).
const TILESHEET_DIM: usize = 128;
/// Byte offset of the attribute table within a 1 KiB nametable page
/// (32x30 tile bytes = 960, then 64 attribute bytes).
const ATTR_TABLE_OFFSET: usize = 960;

/// Render a synthetic PPU test frame and tile sheets to `build/*.ppm`, reporting
/// the number of lit pixels. Returns an error on I/O failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Load the ROM (CHR data only is needed; no game state is run).
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let mut engine = common::load_rom(rom, false)?;

    // Dump both pattern tables (0 and 1) as 128x128 tile-sheet images.
    let mut sheet = vec![0; TILESHEET_DIM * TILESHEET_DIM * 3];
    engine.ppu.debug_tilesheet(0, &mut sheet);
    common::ensure_parent("build/ppu_tiles_0.ppm")?;
    lotw::ppu::ppm_write(
        "build/ppu_tiles_0.ppm",
        &sheet,
        TILESHEET_DIM,
        TILESHEET_DIM,
    )?;
    engine.ppu.debug_tilesheet(1, &mut sheet);
    lotw::ppu::ppm_write(
        "build/ppu_tiles_1.ppm",
        &sheet,
        TILESHEET_DIM,
        TILESHEET_DIM,
    )?;

    // Fill the visible nametable (32x30 tiles) with an incrementing tile index.
    for ty in 0..30 {
        for tx in 0..32 {
            engine.ppu.vram[ty * 32 + tx] = ((ty * 32 + tx) & 255) as u8;
        }
    }
    // Fill the 64-byte attribute table so the four palettes cycle across the
    // screen (85 == 0b01010101 packs palette index i&3 into all four 2-bit fields).
    for i in 0..64 {
        engine.ppu.vram[ATTR_TABLE_OFFSET + i] = (85 * (i & 3)) as u8;
    }
    // Install a fixed 8-palette (32-byte) palette set as the background/sprite colors.
    engine.ppu.pal.copy_from_slice(&[
        15, 0, 16, 48, 15, 6, 22, 38, 15, 9, 25, 41, 15, 1, 17, 33, 15, 18, 34, 50, 15, 20, 36, 52,
        15, 26, 42, 58, 15, 5, 21, 37,
    ]);
    // Clear OAM (255 = off-screen Y), then place 8 test sprites in a diagonal row.
    engine.ppu.oam.fill(255);
    for i in 0..8 {
        let o = i * 4; // each OAM entry is 4 bytes: Y, tile, attr, X
        engine.ppu.oam[o] = 112; // Y position (vertically centered)
        engine.ppu.oam[o + 1] = 16 + i as u8; // tile index, stepping per sprite
        engine.ppu.oam[o + 2] = (i & 3) as u8; // attribute byte (palette select)
        engine.ppu.oam[o + 3] = 40 + i as u8 * 24; // X position, 24px apart
    }
    // PPU registers: pattern table 0, show background+sprites (mask 24 = 0b11000), no scroll.
    engine.ppu.ctrl = 0;
    engine.ppu.mask = 24;
    engine.ppu.scroll_x = 0;
    engine.ppu.scroll_y = 0;

    // Render the synthetic frame and write it out.
    let mut frame = vec![0; PPU_W * PPU_H * 3];
    let memory = engine.state.ram_bytes().to_vec();
    engine.ppu.render(&memory, &mut frame);
    common::write_ppm("build/ppu_frame.ppm", &frame)?;
    // Count non-black pixels as a quick "did anything render" sanity check.
    let lit = frame
        .chunks_exact(3)
        .filter(|px| px.iter().any(|c| *c != 0))
        .count();
    println!("wrote build/ppu_frame.ppm ({PPU_W}x{PPU_H}), lit pixels={lit}");
    Ok(())
}
