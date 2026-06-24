//! Native Rust port of the NES game *Legacy of the Wizard*.
//!
//! This crate is a software re-implementation of the original 6502 game logic
//! together with software PPU/APU shims, so the game runs without an emulator.
//! Execution is driven by a stackful-coroutine frame runner, and playback is
//! presented through SDL3.
//!
//! # Modules
//!
//! - [`engine`] — the CPU memory image plus the MMC3 mapper and device
//!   (memory-mapped register) decode.
//! - [`state`] — [`GameState`], the named memory and field accessors over the
//!   game's RAM.
//! - [`game`] — the translated game routines and the runtime glue that ties
//!   them together.
//! - [`ppu`] / [`apu`] — software shims standing in for the NES picture and
//!   audio hardware.
//! - [`frame`] — the stackful-coroutine runner that advances the game one
//!   frame at a time.
//! - [`bits`] — named bit/mask constants used in place of raw hex literals.
//! - [`scripts`] — replay helpers for driving recorded inputs.
//!
//! The crate-wide `#![allow(...)]` below relaxes the usual style lints: the
//! mechanically translated 6502 code triggers many of them (non-snake-case
//! names, unused locals, redundant parentheses, and so on) that are expected
//! and not worth flagging.
#![allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    unreachable_code,
    unused_assignments,
    unused_imports,
    unused_mut,
    unused_parens,
    unused_variables
)]

pub mod apu;
pub mod bits;
pub mod engine;
pub mod frame;
pub mod game;
pub mod ppu;
pub mod scripts;
pub mod state;

pub use engine::{Engine, PPU_H, PPU_W, RoutineContext};
pub use state::GameState;
