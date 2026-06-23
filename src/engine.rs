use crate::{apu::Apu, ppu::Ppu, state::GameState};

pub const PPU_W: usize = 256;
pub const PPU_H: usize = 240;
pub const APU_SR: usize = 44_100;

/// Memory-mapped NES hardware register addresses, accessed via
/// [`Engine::device_read`] / [`Engine::device_write`]. Named so call sites read
/// as register operations rather than raw addresses.
pub mod reg {
    // PPU registers ($2000-$2007)
    pub const PPU_CTRL: i32 = 0x2000;
    pub const PPU_MASK: i32 = 0x2001;
    pub const PPU_STATUS: i32 = 0x2002;
    pub const OAM_ADDR: i32 = 0x2003;
    pub const OAM_DATA: i32 = 0x2004;
    pub const PPU_SCROLL: i32 = 0x2005;
    pub const PPU_ADDR: i32 = 0x2006;
    pub const PPU_DATA: i32 = 0x2007;
    // APU pulse 1
    pub const SQ1_VOL: i32 = 0x4000;
    pub const SQ1_SWEEP: i32 = 0x4001;
    pub const SQ1_LO: i32 = 0x4002;
    pub const SQ1_HI: i32 = 0x4003;
    // APU pulse 2
    pub const SQ2_VOL: i32 = 0x4004;
    pub const SQ2_SWEEP: i32 = 0x4005;
    pub const SQ2_LO: i32 = 0x4006;
    pub const SQ2_HI: i32 = 0x4007;
    // APU triangle
    pub const TRI_LINEAR: i32 = 0x4008;
    pub const TRI_LO: i32 = 0x400A;
    pub const TRI_HI: i32 = 0x400B;
    // APU noise
    pub const NOISE_VOL: i32 = 0x400C;
    pub const NOISE_LO: i32 = 0x400E;
    pub const NOISE_HI: i32 = 0x400F;
    // APU DMC / status / frame counter, OAM DMA, controllers
    pub const DMC_FREQ: i32 = 0x4010;
    pub const OAM_DMA: i32 = 0x4014;
    pub const APU_STATUS: i32 = 0x4015;
    pub const JOY1: i32 = 0x4016;
    pub const APU_FRAME: i32 = 0x4017;
    // MMC3 mapper registers
    pub const MMC3_BANK_SELECT: i32 = 0x8000;
    pub const MMC3_BANK_DATA: i32 = 0x8001;
    pub const MMC3_MIRROR: i32 = 0xA000;
    pub const MMC3_PRG_RAM: i32 = 0xA001;
    pub const MMC3_IRQ_DISABLE: i32 = 0xE000;
    pub const MMC3_IRQ_ENABLE: i32 = 0xE001;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoutineContext {
    pub value: i32,
    pub index: i32,
    pub offset: i32,
    pub carry: i32,
    pub zero: i32,
    pub negative: i32,
    pub overflow: i32,
}

pub type RoutineFn = fn(&mut Engine, &mut RoutineContext);

pub struct Engine {
    /// CPU-visible memory image plus named game-state accessors.
    pub state: GameState,
    pub ppu: Ppu,
    pub apu: Apu,
    pub lotw_nonlocal_handoff: i32,
    pub room_ckpt_stack: [[u8; 7]; 4],
    pub room_ckpt_sp: usize,
    next_input: Option<Box<dyn FnMut() -> i32 + Send>>,
    apu_trace: Option<Box<dyn FnMut(i32, i32) + Send>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            lotw_nonlocal_handoff: 0,
            room_ckpt_stack: [[0; 7]; 4],
            room_ckpt_sp: 0,
            next_input: None,
            apu_trace: None,
        }
    }

    pub fn reset_memory(&mut self) {
        self.state.reset();
        self.lotw_nonlocal_handoff = 0;
    }

    // Memory is accessed through `self.state` (a `GameState`): named accessors
    // for known locations, or `state.byte`/`set_byte` for genuinely dynamic
    // addresses. The old `engine.mem(addr)` delegates have been fully removed
    // now that every call site uses the labeled API.

    pub fn set_next_input<F>(&mut self, next: F)
    where
        F: FnMut() -> i32 + Send + 'static,
    {
        self.next_input = Some(Box::new(next));
    }

    pub fn clear_next_input(&mut self) {
        self.next_input = None;
    }

    pub fn next_input(&mut self) -> Option<i32> {
        self.next_input.as_mut().map(|next| next() & 255)
    }

    pub fn set_apu_trace<F>(&mut self, trace: F)
    where
        F: FnMut(i32, i32) + Send + 'static,
    {
        self.apu_trace = Some(Box::new(trace));
    }

    pub fn clear_apu_trace(&mut self) {
        self.apu_trace = None;
    }

    pub fn apu_write_with_trace(&mut self, addr: i32, value: i32) {
        if let Some(trace) = self.apu_trace.as_mut() {
            trace(addr & 65535, value & 255);
        }
        self.apu.write(addr, value);
    }

    pub fn device_write(&mut self, addr: i32, value: i32) {
        let addr = addr & 65535;
        let value = value & 255;
        if (reg::PPU_CTRL..=reg::PPU_DATA).contains(&addr) {
            self.ppu.openbus = value as u8;
        }
        match addr {
            reg::PPU_CTRL => self.ppu.ctrl = value as u8,
            reg::PPU_MASK => self.ppu.mask = value as u8,
            reg::OAM_ADDR => self.ppu.oamaddr = value as u8,
            reg::OAM_DATA => {
                let idx = self.ppu.oamaddr as usize;
                self.ppu.oam[idx] = value as u8;
                self.ppu.oamaddr = self.ppu.oamaddr.wrapping_add(1);
            }
            reg::PPU_SCROLL => {
                if self.ppu.wtoggle == 0 {
                    self.ppu.scroll_x = value as u8;
                    self.ppu.wtoggle = 1;
                } else {
                    self.ppu.scroll_y = value as u8;
                    self.ppu.wtoggle = 0;
                }
            }
            reg::PPU_ADDR => {
                if self.ppu.wtoggle == 0 {
                    self.ppu.vaddr = (self.ppu.vaddr & 255) | ((value as u16) << 8);
                    self.ppu.wtoggle = 1;
                } else {
                    self.ppu.vaddr = (self.ppu.vaddr & 65280) | value as u16;
                    self.ppu.wtoggle = 0;
                }
            }
            reg::PPU_DATA => {
                let a = self.ppu.vaddr & 16383;
                if a >= 16128 {
                    let mut p = a & 31;
                    if (p & 3) == 0 {
                        p &= 15;
                        self.ppu.pal[p as usize] = value as u8;
                        self.ppu.pal[(p | 16) as usize] = value as u8;
                    } else {
                        self.ppu.pal[p as usize] = value as u8;
                    }
                } else {
                    let off = self.ppu.nt_addr_offset(a);
                    self.ppu.vram[off] = value as u8;
                }
                self.ppu.vaddr =
                    self.ppu
                        .vaddr
                        .wrapping_add(if (self.ppu.ctrl & 4) != 0 { 32 } else { 1 });
            }
            reg::OAM_DMA => {
                let base = (value as usize) << 8;
                for i in 0..256 {
                    let dst = self.ppu.oamaddr.wrapping_add(i as u8) as usize;
                    self.ppu.oam[dst] = self.state.ram[(base + i) & 65535];
                }
            }
            reg::MMC3_BANK_SELECT => {
                self.ppu.mmc3_sel = value as u8;
                self.ppu.recompute_chr();
            }
            reg::MMC3_BANK_DATA => {
                let sel = self.ppu.mmc3_sel & 7;
                self.ppu.mmc3_bank[sel as usize] = value as u8;
                self.ppu.recompute_chr();
                if sel == 6 {
                    self.ppu_map_prg(32768, value);
                } else if sel == 7 {
                    self.ppu_map_prg(40960, value);
                }
            }
            reg::MMC3_MIRROR => self.ppu.mirror = if (value & 1) != 0 { 0 } else { 1 },
            reg::JOY1 => {
                self.ppu.strobe = (value & 1) as u8;
                if self.ppu.strobe != 0 {
                    self.ppu.ctrl_latch = self.ppu.buttons;
                }
            }
            _ => {
                if (reg::SQ1_VOL..=reg::APU_FRAME).contains(&addr) && addr != reg::OAM_DMA {
                    self.apu_write_with_trace(addr, value);
                }
            }
        }
    }

    pub fn device_read(&mut self, addr: i32) -> i32 {
        let addr = addr & 65535;
        match addr {
            reg::PPU_STATUS => {
                let s = (self.ppu.status & 224) | (self.ppu.openbus & 31);
                self.ppu.status &= !128;
                self.ppu.wtoggle = 0;
                s as i32
            }
            reg::OAM_DATA => {
                let ret = self.ppu.oam[self.ppu.oamaddr as usize];
                self.ppu.openbus = ret;
                ret as i32
            }
            reg::PPU_DATA => {
                let a = self.ppu.vaddr & 16383;
                let ret = if a >= 16128 {
                    let mut p = a & 31;
                    if (p & 3) == 0 {
                        p &= 15;
                    }
                    self.ppu.pal[p as usize]
                } else {
                    let ret = self.ppu.readbuf;
                    self.ppu.readbuf = self.ppu.vram[self.ppu.nt_addr_offset(a)];
                    ret
                };
                self.ppu.vaddr =
                    self.ppu
                        .vaddr
                        .wrapping_add(if (self.ppu.ctrl & 4) != 0 { 32 } else { 1 });
                self.ppu.openbus = ret;
                ret as i32
            }
            reg::JOY1 => {
                if self.ppu.strobe != 0 {
                    (self.ppu.buttons & 1) as i32
                } else {
                    let bit = self.ppu.ctrl_latch & 1;
                    self.ppu.ctrl_latch >>= 1;
                    bit as i32
                }
            }
            reg::APU_FRAME => 0,
            _ => 0,
        }
    }

    pub fn ppu_load_prg(&mut self, prg: &[u8]) {
        self.ppu.load_prg(prg);
    }

    pub fn ppu_load_chr(&mut self, chr: &[u8]) {
        self.ppu.load_chr(chr);
    }

    pub fn ppu_map_prg(&mut self, cpu_base: i32, bank8k: i32) {
        if self.ppu.prg_len == 0 {
            return;
        }
        let nbanks = self.ppu.prg_len / 8192;
        let off = ((bank8k as usize) % nbanks) * 8192;
        let dst = (cpu_base as usize) & 65535;
        self.state.ram[dst..dst + 8192].copy_from_slice(&self.ppu.prg[off..off + 8192]);
    }

    pub fn prg_map_shadow(&mut self) {
        self.ppu_map_prg(32768, self.state.prg_bank_8000());
        self.ppu_map_prg(40960, self.state.prg_bank_a000());
    }

    pub fn load_ines(&mut self, rom: &[u8], init_ram_pattern: bool) -> Result<(), String> {
        if rom.len() < 16 || &rom[0..3] != b"NES" {
            return Err("not an iNES file".to_string());
        }
        let prg = rom[4] as usize * 16_384;
        let chr = rom[5] as usize * 8_192;
        if rom.len() < 16 + prg + chr {
            return Err("short ROM".to_string());
        }
        if init_ram_pattern {
            for a in 0..2048 {
                self.state.ram[a] = if (a & 4) != 0 { 255 } else { 0 };
            }
        }
        self.ppu_load_prg(&rom[16..16 + prg]);
        self.ppu_load_chr(&rom[16 + prg..16 + prg + chr]);
        self.ppu.reset();
        self.apu.reset();
        self.state.ram[49152..65536].copy_from_slice(&rom[16 + prg - 16384..16 + prg]);
        self.ppu_map_prg(32768, 12);
        self.ppu_map_prg(40960, 13);
        self.ppu.set_vblank(true);
        Ok(())
    }
}
