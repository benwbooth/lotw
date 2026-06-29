//! Regenerate `src/music.rs` from the ROM: each song as a byte-exact `play!`
//! DSL function. Run: `cargo run --bin gen_music -- [rom] [out]`.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let rom_path = args.next().unwrap_or_else(|| "rom/lotw.nes".into());
    let out_path = args.next().unwrap_or_else(|| "src/music/songs.rs".into());
    let rom = std::fs::read(&rom_path)?;
    let prg_len = rom[4] as usize * 16_384;
    let prg = &rom[16..16 + prg_len];
    let src = lotw::audio::emit_music_rs(prg);
    std::fs::write(&out_path, &src)?;
    eprintln!("wrote {out_path} ({} bytes)", src.len());
    Ok(())
}
