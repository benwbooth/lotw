mod common;

use std::{env, error::Error};

use lotw::{PPU_H, PPU_W};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let mut engine = common::load_rom(rom, false)?;

    let mut sheet = vec![0; 128 * 128 * 3];
    engine.ppu.debug_tilesheet(0, &mut sheet);
    common::ensure_parent("build/ppu_tiles_0.ppm")?;
    lotw::ppu::ppm_write("build/ppu_tiles_0.ppm", &sheet, 128, 128)?;
    engine.ppu.debug_tilesheet(1, &mut sheet);
    lotw::ppu::ppm_write("build/ppu_tiles_1.ppm", &sheet, 128, 128)?;

    for ty in 0..30 {
        for tx in 0..32 {
            engine.ppu.vram[ty * 32 + tx] = ((ty * 32 + tx) & 255) as u8;
        }
    }
    for i in 0..64 {
        engine.ppu.vram[960 + i] = (85 * (i & 3)) as u8;
    }
    engine.ppu.pal.copy_from_slice(&[
        15, 0, 16, 48, 15, 6, 22, 38, 15, 9, 25, 41, 15, 1, 17, 33, 15, 18, 34, 50, 15, 20, 36, 52,
        15, 26, 42, 58, 15, 5, 21, 37,
    ]);
    engine.ppu.oam.fill(255);
    for i in 0..8 {
        let o = i * 4;
        engine.ppu.oam[o] = 112;
        engine.ppu.oam[o + 1] = 16 + i as u8;
        engine.ppu.oam[o + 2] = (i & 3) as u8;
        engine.ppu.oam[o + 3] = 40 + i as u8 * 24;
    }
    engine.ppu.ctrl = 0;
    engine.ppu.mask = 24;
    engine.ppu.scroll_x = 0;
    engine.ppu.scroll_y = 0;

    let mut frame = vec![0; PPU_W * PPU_H * 3];
    let memory = engine.state.ram;
    engine.ppu.render(&memory, &mut frame);
    common::write_ppm("build/ppu_frame.ppm", &frame)?;
    let lit = frame
        .chunks_exact(3)
        .filter(|px| px.iter().any(|c| *c != 0))
        .count();
    println!("wrote build/ppu_frame.ppm ({PPU_W}x{PPU_H}), lit pixels={lit}");
    Ok(())
}
