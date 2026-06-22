#![allow(dead_code)]

use std::{
    collections::BTreeSet,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use lotw::{Engine, PPU_H, PPU_W, RoutineContext, apu::wav_write, frame::FrameRunner, game, ppu};

pub const FPS: usize = 60;
pub const SPF: usize = lotw::engine::APU_SR / FPS;

pub fn ensure_parent(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn load_rom(path: impl AsRef<Path>, init_ram_pattern: bool) -> Result<Engine, Box<dyn Error>> {
    let rom = fs::read(path)?;
    let mut engine = Engine::new();
    engine.load_ines(&rom, init_ram_pattern)?;
    Ok(engine)
}

pub fn start_runner(engine: Engine) -> Result<FrameRunner, Box<dyn Error>> {
    let mut runner = FrameRunner::new(engine, game::reset);
    if !runner.start() {
        return Err("game loop returned during boot".into());
    }
    Ok(runner)
}

pub fn step_frame(runner: &mut FrameRunner) -> bool {
    runner.with_engine_regs(game::vblank_commit);
    runner.resume_until_wait()
}

pub fn render_frame(engine: &mut Engine) -> Vec<u8> {
    let mut fb = vec![0; PPU_W * PPU_H * 3];
    let memory = engine.state.ram;
    engine.ppu.render(&memory, &mut fb);
    fb
}

pub fn write_ppm(path: impl AsRef<Path>, rgb: &[u8]) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    ppu::ppm_write(path, rgb, PPU_W, PPU_H)?;
    Ok(())
}

pub fn write_wav(path: impl AsRef<Path>, samples: &[i16]) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    wav_write(path, samples, lotw::engine::APU_SR)?;
    Ok(())
}

pub fn write_ram(path: impl AsRef<Path>, engine: &Engine) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    fs::write(path, &engine.state.ram[..0x800])?;
    Ok(())
}

pub fn write_ppu_state(path: impl AsRef<Path>, engine: &Engine) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    let mut f = File::create(path)?;
    let mirror = engine.ppu.mirror;
    for nt in 0..4 {
        let phys = if mirror == 0 { nt >> 1 } else { nt & 1 };
        f.write_all(&engine.ppu.vram[phys * 0x400..phys * 0x400 + 0x400])?;
    }
    f.write_all(&engine.ppu.pal)?;
    f.write_all(&engine.ppu.oam)?;
    f.write_all(&[
        engine.ppu.ctrl,
        engine.ppu.mask,
        engine.ppu.scroll_x,
        engine.ppu.scroll_y,
        mirror as u8,
    ])?;
    Ok(())
}

pub fn parse_replay(path: impl AsRef<Path>) -> Result<Vec<u8>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut out = vec![0];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(kind) = parts.next() else {
            continue;
        };
        if kind != "frame" {
            return Err(format!("unknown replay directive: {kind}").into());
        }
        let count: usize = parts.next().ok_or("missing frame count")?.parse()?;
        let mut buttons = 0;
        for part in parts {
            buttons |= button_bit(part)?;
        }
        out.extend(std::iter::repeat(buttons).take(count));
    }
    Ok(out)
}

fn button_bit(name: &str) -> Result<u8, Box<dyn Error>> {
    Ok(match name {
        "A" => 0x01,
        "B" => 0x02,
        "select" => 0x04,
        "start" => 0x08,
        "up" => 0x10,
        "down" => 0x20,
        "left" => 0x40,
        "right" => 0x80,
        _ => return Err(format!("unknown replay button: {name}").into()),
    })
}

pub fn parse_capture_set(csv: &str) -> Result<(BTreeSet<usize>, usize), Box<dyn Error>> {
    let mut set = BTreeSet::new();
    let mut max = 0;
    for part in csv.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let frame: usize = part.parse()?;
        if frame == 0 {
            return Err("capture frames are 1-based".into());
        }
        max = max.max(frame);
        set.insert(frame);
    }
    Ok((set, max))
}

pub fn input_closure(input: Vec<u8>) -> (impl FnMut() -> i32 + Send + 'static, Arc<Mutex<usize>>) {
    let input = Arc::new(input);
    let pos = Arc::new(Mutex::new(0usize));
    let pos_for_closure = Arc::clone(&pos);
    let closure_input = Arc::clone(&input);
    let next = move || {
        let mut pos = pos_for_closure.lock().expect("input mutex poisoned");
        if *pos < closure_input.len() {
            let v = closure_input[*pos] as i32;
            *pos += 1;
            v
        } else {
            0
        }
    };
    (next, pos)
}

pub fn read_binary(path: impl AsRef<Path>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn build_path(parts: &[&str]) -> PathBuf {
    parts.iter().collect()
}

pub fn init_game_scene(engine: &mut Engine, r: &mut RoutineContext) {
    game::ram_state_init(engine, r);
    game::farcall_bank_0C0D_seed(engine, r);
    engine.state.set_song(0x09);
    engine.set_mem(0x41, 0xff);
    engine.set_mem(0x39, 0xc5);
    engine.set_mem(0x3a, 0x17);
    engine.set_mem(0x3b, 0x42);
    engine.set_mem(0x47, 0x01);
    engine.set_mem(0x48, 0x05);
    engine.set_mem(0x40, 0x00);
    for i in 0..4 {
        engine.set_mem(0x5c + i, engine.mem(0xffa7 + i));
    }
    engine.set_mem(0x51, engine.mem(0xb0ac));
    engine.set_mem(0x55, 0);
    engine.set_mem(0x2c, 0x38);
    engine.set_mem(0x2e, 0x3e);
    engine.set_mem(0x2f, 0x20);
    engine.set_mem(0x56, 0x0d);
    engine.set_mem(0x57, 0);
    engine.set_mem(0x42, 1);
    engine.state.set_player_health(0x64);
    engine.state.set_player_magic(0x64);
    engine.set_mem(0xeb, 0);
    engine.set_mem(0x44, 0x20);
    engine.set_mem(0x45, 0x80);
    engine.set_mem(0x43, 0);
    engine.set_mem(0x7c, 0x18);
    engine.set_mem(0x7b, 0);
    game::scene_assemble(engine, r);
}
