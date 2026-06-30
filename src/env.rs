//! A headless, deterministic **agent environment** for RL / search over the game.
//!
//! It boots the real game, steps one emulated frame at a time given a controller
//! byte, renders RGB frames (the agent's only observation), and exposes the work
//! RAM + named [`GameState`] as a *privileged* signal for training rewards (the
//! policy itself should observe only [`Env::render`], never the RAM).
//!
//! **Save-states are replay-based.** The game body runs as one long-lived
//! `corosensei` coroutine ([`FrameRunner`]), which can't be cloned, so a
//! "checkpoint" is simply an input prefix: [`Env::reset_replay`] reboots and
//! fast-forwards (without rendering). That's coroutine-safe and is exactly what
//! single-demo reverse-curriculum / Go-Explore need — the demo *is* an input
//! sequence, so checkpoint *k* is `demo[..k]`.
//!
//! Controller byte = the hardware controller register (what `ppu.buttons` takes),
//! LSB first: `0`=A `1`=B `2`=Select `3`=Start `4`=Up `5`=Down `6`=Left `7`=Right.
//! (This is the raw button order; the game reverses it internally into
//! [`GameState::buttons`] — don't confuse the two.)

use crate::{Engine, GameState, PPU_H, PPU_W, frame::FrameRunner, game};

/// Rendered frame dimensions (the NES screen) and RGB24 byte length.
pub const FRAME_W: usize = PPU_W;
pub const FRAME_H: usize = PPU_H;
pub const FRAME_BYTES: usize = PPU_W * PPU_H * 3;

/// One agent environment instance: a booted game you drive one frame at a time.
pub struct Env {
    rom: Vec<u8>, // kept so we can reboot without re-reading the ROM file
    init_ram: bool,
    runner: FrameRunner,
    frame: usize,
    done: bool,
}

impl Env {
    /// Boot a fresh game from an in-memory iNES image. `init_ram` seeds work RAM
    /// with the hardware power-on pattern (match a real NES / recorded replays).
    pub fn from_rom_bytes(rom: Vec<u8>, init_ram: bool) -> Result<Self, String> {
        let runner = boot(&rom, init_ram)?;
        Ok(Self { rom, init_ram, runner, frame: 0, done: false })
    }

    /// Boot from a ROM file on disk.
    pub fn from_path(path: &str, init_ram: bool) -> Result<Self, String> {
        let rom = std::fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
        Self::from_rom_bytes(rom, init_ram)
    }

    /// Reboot to a fresh game (power-on).
    pub fn reset(&mut self) -> Result<(), String> {
        self.runner = boot(&self.rom, self.init_ram)?;
        self.frame = 0;
        self.done = false;
        Ok(())
    }

    /// Reboot then fast-forward by replaying `inputs` (one controller byte per
    /// frame) **without rendering** — the cheap "load checkpoint *k*". Stops early
    /// if the game ends mid-prefix.
    pub fn reset_replay(&mut self, inputs: &[u8]) -> Result<(), String> {
        self.reset()?;
        for &b in inputs {
            if self.done {
                break;
            }
            self.advance(b);
        }
        Ok(())
    }

    /// Advance exactly one emulated frame with controller byte `action`, **without
    /// rendering**. Returns `done` (the game loop exited). Use for fast-forward /
    /// search where you don't need every frame's pixels.
    pub fn advance(&mut self, action: u8) -> bool {
        if self.done {
            return true;
        }
        self.runner.engine_mut().ppu.buttons = action;
        self.runner.with_engine_regs(game::vblank_commit);
        let alive = self.runner.resume_until_wait();
        self.frame += 1;
        self.done = !alive;
        self.done
    }

    /// Step one frame and return `(rgb_frame, done)` — the agent-facing API.
    pub fn step(&mut self, action: u8) -> (Vec<u8>, bool) {
        let done = self.advance(action);
        (self.render(), done)
    }

    /// Render the current PPU state into a fresh `FRAME_BYTES` RGB24 buffer.
    pub fn render(&mut self) -> Vec<u8> {
        let e = self.runner.engine_mut();
        let mut fb = vec![0u8; FRAME_BYTES];
        let mem = e.state.ram_bytes().to_vec(); // the PPU renderer reads CPU RAM
        e.ppu.render(&mem, &mut fb);
        fb
    }

    /// The 2 KiB NES work RAM — **privileged** training signal (reward shaping,
    /// success checks). Do not feed this to the policy.
    pub fn ram(&self) -> &[u8] {
        &self.runner.engine().state.ram_bytes()[..2048]
    }

    /// Named game-state accessors (player position, character, map cell, …) for
    /// writing reward / success predicates. Also privileged.
    pub fn state(&self) -> &GameState {
        &self.runner.engine().state
    }

    /// Frames advanced since the last (re)boot.
    pub fn frame_count(&self) -> usize {
        self.frame
    }

    /// Whether the game loop has exited.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

/// Boot a fresh engine from a ROM image and run it to the first frame wait.
fn boot(rom: &[u8], init_ram: bool) -> Result<FrameRunner, String> {
    let mut engine = Engine::new();
    engine.load_ines(rom, init_ram)?;
    let mut runner = FrameRunner::new(engine, game::reset);
    if !runner.start() {
        return Err("game loop returned during boot".into());
    }
    Ok(runner)
}
