//! Shared test/driver harness for the `lotw` binary crates.
//!
//! This module is included (via `mod common;`) by every binary under
//! `src/bin/`. It centralizes the boilerplate those drivers need: loading an
//! iNES ROM into an [`Engine`], booting the game coroutine through a
//! [`FrameRunner`], advancing one emulated frame at a time, rendering the PPU
//! framebuffer, and serializing artifacts (PPM images, WAV audio, raw RAM /
//! PPU-state dumps) used by the byte-exact verification harness. It also holds
//! the replay-file parser and the recorded-input closure used to feed scripted
//! button sequences into the engine.

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

/// Emulated NES frame rate (frames per second).
pub const FPS: usize = 60;
/// Audio samples generated per video frame: APU sample rate divided by [`FPS`].
pub const SPF: usize = lotw::engine::APU_SR / FPS;

/// Create all parent directories of `path` so a subsequent file write succeeds.
///
/// Returns `Ok(())` (including when `path` has no parent component).
pub fn ensure_parent(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    // Only create directories if the path actually has a parent component.
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Read an iNES ROM from `path` and load it into a fresh [`Engine`].
///
/// `init_ram_pattern` selects whether work RAM is seeded with the hardware
/// power-on pattern (matching the real NES) or left zeroed. Returns the loaded
/// engine ready to be booted via [`start_runner`].
pub fn load_rom(path: impl AsRef<Path>, init_ram_pattern: bool) -> Result<Engine, Box<dyn Error>> {
    // Read the ROM file and parse its iNES header / banks into the engine.
    let rom = fs::read(path)?;
    let mut engine = Engine::new();
    engine.load_ines(&rom, init_ram_pattern)?;
    Ok(engine)
}

/// Wrap `engine` in a [`FrameRunner`] and run the game's reset/boot sequence.
///
/// Starts the stackful game coroutine at [`game::reset`] and drives it up to the
/// first vblank wait. Returns the runner, or an error if the game loop returned
/// (exited) during boot rather than parking at a frame boundary.
pub fn start_runner(engine: Engine) -> Result<FrameRunner, Box<dyn Error>> {
    let mut runner = FrameRunner::new(engine, game::reset);
    if !runner.start() {
        return Err("game loop returned during boot".into());
    }
    Ok(runner)
}

/// Advance the game by exactly one emulated frame.
///
/// Performs the per-frame vblank commit (NMI handler work) and then resumes the
/// game coroutine until it parks at its next vblank wait. Returns `false` if the
/// game loop returned instead of parking (i.e. the game exited).
pub fn step_frame(runner: &mut FrameRunner) -> bool {
    // Run the vblank/NMI commit work, then resume the coroutine to the next wait.
    runner.with_engine_regs(game::vblank_commit);
    runner.resume_until_wait()
}

/// Render the current PPU state into a freshly allocated RGB framebuffer.
///
/// Returns a `PPU_W * PPU_H * 3`-byte RGB24 buffer (one frame). The CPU RAM is
/// snapshotted into `memory` first because the PPU renderer reads from it.
pub fn render_frame(engine: &mut Engine) -> Vec<u8> {
    let mut fb = vec![0; PPU_W * PPU_H * 3];
    // Snapshot RAM for the renderer (it reads game state out of CPU memory).
    let memory = engine.state.ram_bytes().to_vec();
    engine.ppu.render(&memory, &mut fb);
    fb
}

/// Write an RGB24 framebuffer `rgb` to `path` as a binary PPM (P6) image.
///
/// The image is fixed at the NES screen size (`PPU_W` x `PPU_H`); parent
/// directories are created as needed.
pub fn write_ppm(path: impl AsRef<Path>, rgb: &[u8]) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    ppu::ppm_write(path, rgb, PPU_W, PPU_H)?;
    Ok(())
}

/// Write 16-bit PCM `samples` to `path` as a WAV file at the APU sample rate.
///
/// Parent directories are created as needed.
pub fn write_wav(path: impl AsRef<Path>, samples: &[i16]) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    wav_write(path, samples, lotw::engine::APU_SR)?;
    Ok(())
}

/// Dump the engine's 2 KiB of work RAM to `path` as a raw binary blob.
///
/// Only the first 2048 bytes (the NES internal RAM) are written; parent
/// directories are created as needed.
pub fn write_ram(path: impl AsRef<Path>, engine: &Engine) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    // The NES has 2 KiB of internal work RAM; dump exactly that range.
    fs::write(path, engine.state.ram_bytes()[..2048].as_ref())?;
    Ok(())
}

/// Serialize the full PPU state to `path` for byte-exact comparison.
///
/// The layout is: the four logical nametables (each 1 KiB, resolved through the
/// current mirroring), then the palette, then OAM, then a 5-byte trailer of
/// `ctrl`, `mask`, `scroll_x`, `scroll_y`, and `mirror`.
pub fn write_ppu_state(path: impl AsRef<Path>, engine: &Engine) -> Result<(), Box<dyn Error>> {
    ensure_parent(&path)?;
    let mut f = File::create(path)?;
    let mirror = engine.ppu.mirror;
    // Emit all four logical nametables. With only 2 KiB of VRAM, each logical
    // nametable maps to one of two physical 1 KiB pages: mirror==0 is horizontal
    // mirroring (pages selected by nt>>1), otherwise vertical (pages by nt&1).
    for nt in 0..4 {
        let phys = if mirror == 0 { nt >> 1 } else { nt & 1 };
        f.write_all(&engine.ppu.vram[phys * 1024..phys * 1024 + 1024])?;
    }
    // Palette RAM and sprite OAM follow the nametables.
    f.write_all(&engine.ppu.pal)?;
    f.write_all(&engine.ppu.oam)?;
    // 5-byte register/mirroring trailer.
    f.write_all(&[
        engine.ppu.ctrl,
        engine.ppu.mask,
        engine.ppu.scroll_x,
        engine.ppu.scroll_y,
        mirror as u8,
    ])?;
    Ok(())
}

/// Parse a human-readable replay file into a per-frame button bitmask vector.
///
/// Each non-empty, non-comment line has the form `frame <count> <button>...`,
/// expanding to `count` frames each holding the OR of the named buttons. The
/// returned vector is 1-based: index 0 is a synthetic leading no-input frame so
/// `out[frame]` matches the engine's 1-based frame numbering. Returns an error
/// on unknown directives or button names.
pub fn parse_replay(path: impl AsRef<Path>) -> Result<Vec<u8>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    // Index 0 is a synthetic leading frame so the vector is 1-based.
    let mut out = vec![0];
    for line in text.lines() {
        // Strip trailing '#' comments and surrounding whitespace.
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        // First token is the directive; only "frame" is supported.
        let mut parts = line.split_whitespace();
        let Some(kind) = parts.next() else {
            continue;
        };
        if kind != "frame" {
            return Err(format!("unknown replay directive: {kind}").into());
        }
        // Second token is the repeat count; remaining tokens are button names.
        let count: usize = parts.next().ok_or("missing frame count")?.parse()?;
        let mut buttons = 0;
        for part in parts {
            buttons |= button_bit(part)?;
        }
        // Hold this button combination for `count` consecutive frames.
        out.extend(std::iter::repeat(buttons).take(count));
    }
    Ok(out)
}

/// Map a replay button `name` to its NES controller bitmask.
///
/// The bit order matches the NES standard controller latch order (A in the low
/// bit through Right in the high bit). Returns an error for unknown names.
fn button_bit(name: &str) -> Result<u8, Box<dyn Error>> {
    Ok(match name {
        "A" => 1,       // bit 0
        "B" => 2,       // bit 1
        "select" => 4,  // bit 2
        "start" => 8,   // bit 3
        "up" => 16,     // bit 4
        "down" => 32,   // bit 5
        "left" => 64,   // bit 6
        "right" => 128, // bit 7
        _ => return Err(format!("unknown replay button: {name}").into()),
    })
}

/// Parse a comma-separated list of 1-based frame numbers into a capture set.
///
/// Returns the set of frames to capture together with the largest frame number
/// seen (so callers know how far they must run). Blank entries are ignored;
/// frame 0 is rejected because frame numbering is 1-based.
pub fn parse_capture_set(csv: &str) -> Result<(BTreeSet<usize>, usize), Box<dyn Error>> {
    let mut set = BTreeSet::new();
    let mut max = 0;
    for part in csv.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let frame: usize = part.parse()?;
        // Frame numbering is 1-based to match the replay vector indexing.
        if frame == 0 {
            return Err("capture frames are 1-based".into());
        }
        max = max.max(frame);
        set.insert(frame);
    }
    Ok((set, max))
}

/// Build a controller-input source closure from a recorded button stream.
///
/// Returns a `(next, pos)` pair: `next` is a `Send`/`'static` closure that
/// yields the next button byte (as `i32`) on each call and 0 once the stream is
/// exhausted, suitable for [`Engine::set_next_input`]. `pos` is a shared counter
/// of how many inputs have been consumed, readable by the caller after the run.
pub fn input_closure(input: Vec<u8>) -> (impl FnMut() -> i32 + Send + 'static, Arc<Mutex<usize>>) {
    // Share the input buffer and read cursor between the closure and the caller.
    let input = Arc::new(input);
    let pos = Arc::new(Mutex::new(0usize));
    let pos_for_closure = Arc::clone(&pos);
    let closure_input = Arc::clone(&input);
    let next = move || {
        // Advance through the recorded stream; return 0 once it runs out.
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

/// Read the entire contents of the file at `path` into a byte vector.
pub fn read_binary(path: impl AsRef<Path>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Join path `parts` into a single [`PathBuf`].
pub fn build_path(parts: &[&str]) -> PathBuf {
    parts.iter().collect()
}

/// Set up a deterministic in-game scene for the headless PPU drivers.
///
/// Runs the game's RAM-init and bank-seed routines, then forces a fixed set of
/// game-state values (character, map position, RNG seeds, player pose/health,
/// scroll position, etc.) and assembles the scene. This bypasses the title /
/// character-select flow so the `ppu_*` drivers land directly in a playable room
/// with reproducible contents.
pub fn init_game_scene(engine: &mut Engine, r: &mut RoutineContext) {
    // Standard engine init: zero/seed RAM and run the bank-0C/0D seed farcall.
    game::ram_state_init(engine, r);
    game::farcall_bank_0C0D_seed(engine, r);
    // Force fixed game state so the rendered scene is reproducible.
    engine.state.song = 9;
    engine.state.family_member_mask = 255; // all four family members available
    // Fixed RNG seed bytes for deterministic actor/enemy behaviour.
    engine.state.rng_seed_scratch = 197;
    engine.state.rng_low = 23;
    engine.state.rng_high = 66;
    // Start map screen position (x=1, y=5) and the first selectable character.
    engine.state.map_screen_x = 1;
    engine.state.map_screen_y = 5;
    engine.state.character_index = 0;
    // Copy this character's four stat bytes into the live stats area at $5C-$5F.
    for i in 0..4 {
        engine
            .state
            .set_byte(92 + i, engine.state.byte(game::CHARACTER_STATS_TABLE + i));
    }
    // Give the player their starting item in slot 0 and select it.
    engine
        .state
        .set_item_slot(0, engine.state.byte(game::START_ITEM_TABLE));
    engine.state.selected_item_slot = 0;
    // Select the CHR banks needed to render this scene's tiles/sprites.
    engine.state.set_chr_bank(2, 56);
    engine.state.set_chr_bank(4, 62);
    engine.state.set_chr_bank(5, 32);
    // Player appearance and starting vitals (full health/magic).
    engine.state.player_pose = 13;
    engine.state.player_facing = 0;
    engine.state.title_timer = 1;
    engine.state.player_health = 100;
    engine.state.player_magic = 100;
    engine.state.pending_special_exit = 0;
    // Player and camera placement within the room (tile + fine coordinates).
    engine.state.player_x_tile = 32;
    engine.state.player_y = 128;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 24;
    engine.state.scroll_fine_x = 0;
    // Build the room from the configured map position and state.
    game::scene_assemble(engine, r);
}
