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
pub mod engine;
pub mod frame;
pub mod game;
pub mod native;
pub mod ppu;
pub mod scripts;
pub mod state;

pub use engine::{Engine, PPU_H, PPU_W, RoutineContext};
pub use state::GameState;

pub trait CByte {
    fn c_byte(self) -> i32;
}

impl CByte for bool {
    fn c_byte(self) -> i32 {
        if self { 1 } else { 0 }
    }
}

impl CByte for i32 {
    fn c_byte(self) -> i32 {
        self
    }
}

impl CByte for u8 {
    fn c_byte(self) -> i32 {
        self as i32
    }
}

impl CByte for u16 {
    fn c_byte(self) -> i32 {
        self as i32
    }
}

impl CByte for usize {
    fn c_byte(self) -> i32 {
        self as i32
    }
}

pub fn u8v<T: CByte>(value: T) -> i32 {
    value.c_byte() & 0xff
}

pub fn u16v<T: CByte>(value: T) -> i32 {
    value.c_byte() & 0xffff
}
