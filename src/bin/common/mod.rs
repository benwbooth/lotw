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
    let memory = engine.state.ram_bytes().to_vec();
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
    fs::write(path, engine.state.ram_bytes()[..2048].as_ref())?;
    Ok(())
}

pub fn write_ppu_state(path: impl AsRef<Path>, engine: &Engine) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    let mut f = File::create(path)?;
    let mirror = engine.ppu.mirror;
    for nt in 0..4 {
        let phys = if mirror == 0 { nt >> 1 } else { nt & 1 };
        f.write_all(&engine.ppu.vram[phys * 1024..phys * 1024 + 1024])?;
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
        "A" => 1,
        "B" => 2,
        "select" => 4,
        "start" => 8,
        "up" => 16,
        "down" => 32,
        "left" => 64,
        "right" => 128,
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
    engine.state.song = 9;
    engine.state.family_member_mask = 255;
    engine.state.rng_seed_scratch = 197;
    engine.state.rng_low = 23;
    engine.state.rng_high = 66;
    engine.state.map_screen_x = 1;
    engine.state.map_screen_y = 5;
    engine.state.character_index = 0;
    for i in 0..4 {
        engine
            .state
            .set_byte(92 + i, engine.state.byte(game::CHARACTER_STATS_TABLE + i));
    }
    engine
        .state
        .set_item_slot(0, engine.state.byte(game::START_ITEM_TABLE));
    engine.state.selected_item_slot = 0;
    engine.state.set_chr_bank(2, 56);
    engine.state.set_chr_bank(4, 62);
    engine.state.set_chr_bank(5, 32);
    engine.state.player_pose = 13;
    engine.state.player_facing = 0;
    engine.state.title_timer = 1;
    engine.state.player_health = 100;
    engine.state.player_magic = 100;
    engine.state.pending_special_exit = 0;
    engine.state.player_x_tile = 32;
    engine.state.player_y = 128;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 24;
    engine.state.scroll_fine_x = 0;
    game::scene_assemble(engine, r);
}
