use crate::{apu::Apu, ppu::Ppu, state::GameState};

pub const PPU_W: usize = 256;
pub const PPU_H: usize = 240;
pub const APU_SR: usize = 44_100;

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

    // Memory access delegates to `self.state`. These keep existing call sites
    // compiling while the codebase migrates from raw `engine.mem(addr)` access
    // to the named state accessors on `GameState`; new code should reach for
    // `engine.state.<field>()` (or `engine.state.byte(addr)` for genuinely
    // dynamic addresses) instead.

    #[inline]
    pub fn mem(&self, addr: i32) -> i32 {
        self.state.byte(addr)
    }

    #[inline]
    pub fn set_mem(&mut self, addr: i32, value: i32) {
        self.state.set_byte(addr, value);
    }

    #[inline]
    pub fn add_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.add_byte(addr, value)
    }

    #[inline]
    pub fn sub_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.sub_byte(addr, value)
    }

    #[inline]
    pub fn and_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.and_byte(addr, value)
    }

    #[inline]
    pub fn or_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.or_byte(addr, value)
    }

    #[inline]
    pub fn xor_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.xor_byte(addr, value)
    }

    #[inline]
    pub fn shl_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.shl_byte(addr, value)
    }

    #[inline]
    pub fn shr_mem(&mut self, addr: i32, value: i32) -> i32 {
        self.state.shr_byte(addr, value)
    }

    #[inline]
    pub fn inc_mem(&mut self, addr: i32) -> i32 {
        self.add_mem(addr, 1)
    }

    #[inline]
    pub fn dec_mem(&mut self, addr: i32) -> i32 {
        self.sub_mem(addr, 1)
    }

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
        self.next_input.as_mut().map(|next| next() & 0xff)
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
            trace(addr & 0xffff, value & 0xff);
        }
        self.apu.write(addr, value);
    }

    pub fn device_write(&mut self, addr: i32, value: i32) {
        let addr = addr & 0xffff;
        let value = value & 0xff;
        if (0x2000..=0x2007).contains(&addr) {
            self.ppu.openbus = value as u8;
        }
        match addr {
            0x2000 => self.ppu.ctrl = value as u8,
            0x2001 => self.ppu.mask = value as u8,
            0x2003 => self.ppu.oamaddr = value as u8,
            0x2004 => {
                let idx = self.ppu.oamaddr as usize;
                self.ppu.oam[idx] = value as u8;
                self.ppu.oamaddr = self.ppu.oamaddr.wrapping_add(1);
            }
            0x2005 => {
                if self.ppu.wtoggle == 0 {
                    self.ppu.scroll_x = value as u8;
                    self.ppu.wtoggle = 1;
                } else {
                    self.ppu.scroll_y = value as u8;
                    self.ppu.wtoggle = 0;
                }
            }
            0x2006 => {
                if self.ppu.wtoggle == 0 {
                    self.ppu.vaddr = (self.ppu.vaddr & 0x00ff) | ((value as u16) << 8);
                    self.ppu.wtoggle = 1;
                } else {
                    self.ppu.vaddr = (self.ppu.vaddr & 0xff00) | value as u16;
                    self.ppu.wtoggle = 0;
                }
            }
            0x2007 => {
                let a = self.ppu.vaddr & 0x3fff;
                if a >= 0x3f00 {
                    let mut p = a & 0x1f;
                    if (p & 3) == 0 {
                        p &= 0x0f;
                        self.ppu.pal[p as usize] = value as u8;
                        self.ppu.pal[(p | 0x10) as usize] = value as u8;
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
                        .wrapping_add(if (self.ppu.ctrl & 0x04) != 0 { 32 } else { 1 });
            }
            0x4014 => {
                let base = (value as usize) << 8;
                for i in 0..256 {
                    let dst = self.ppu.oamaddr.wrapping_add(i as u8) as usize;
                    self.ppu.oam[dst] = self.state.ram[(base + i) & 0xffff];
                }
            }
            0x8000 => {
                self.ppu.mmc3_sel = value as u8;
                self.ppu.recompute_chr();
            }
            0x8001 => {
                let sel = self.ppu.mmc3_sel & 7;
                self.ppu.mmc3_bank[sel as usize] = value as u8;
                self.ppu.recompute_chr();
                if sel == 6 {
                    self.ppu_map_prg(0x8000, value);
                } else if sel == 7 {
                    self.ppu_map_prg(0xa000, value);
                }
            }
            0xa000 => self.ppu.mirror = if (value & 1) != 0 { 0 } else { 1 },
            0x4016 => {
                self.ppu.strobe = (value & 1) as u8;
                if self.ppu.strobe != 0 {
                    self.ppu.ctrl_latch = self.ppu.buttons;
                }
            }
            _ => {
                if (0x4000..=0x4017).contains(&addr) && addr != 0x4014 {
                    self.apu_write_with_trace(addr, value);
                }
            }
        }
    }

    pub fn device_read(&mut self, addr: i32) -> i32 {
        let addr = addr & 0xffff;
        match addr {
            0x2002 => {
                let s = (self.ppu.status & 0xe0) | (self.ppu.openbus & 0x1f);
                self.ppu.status &= !0x80;
                self.ppu.wtoggle = 0;
                s as i32
            }
            0x2004 => {
                let ret = self.ppu.oam[self.ppu.oamaddr as usize];
                self.ppu.openbus = ret;
                ret as i32
            }
            0x2007 => {
                let a = self.ppu.vaddr & 0x3fff;
                let ret = if a >= 0x3f00 {
                    let mut p = a & 0x1f;
                    if (p & 3) == 0 {
                        p &= 0x0f;
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
                        .wrapping_add(if (self.ppu.ctrl & 0x04) != 0 { 32 } else { 1 });
                self.ppu.openbus = ret;
                ret as i32
            }
            0x4016 => {
                if self.ppu.strobe != 0 {
                    (self.ppu.buttons & 1) as i32
                } else {
                    let bit = self.ppu.ctrl_latch & 1;
                    self.ppu.ctrl_latch >>= 1;
                    bit as i32
                }
            }
            0x4017 => 0,
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
        let nbanks = self.ppu.prg_len / 0x2000;
        let off = ((bank8k as usize) % nbanks) * 0x2000;
        let dst = (cpu_base as usize) & 0xffff;
        self.state.ram[dst..dst + 0x2000].copy_from_slice(&self.ppu.prg[off..off + 0x2000]);
    }

    pub fn prg_map_shadow(&mut self) {
        self.ppu_map_prg(0x8000, self.state.prg_bank_8000());
        self.ppu_map_prg(0xa000, self.state.prg_bank_a000());
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
            for a in 0..0x0800 {
                self.state.ram[a] = if (a & 4) != 0 { 0xff } else { 0x00 };
            }
        }
        self.ppu_load_prg(&rom[16..16 + prg]);
        self.ppu_load_chr(&rom[16 + prg..16 + prg + chr]);
        self.ppu.reset();
        self.apu.reset();
        self.state.ram[0xc000..0x10000].copy_from_slice(&rom[16 + prg - 0x4000..16 + prg]);
        self.ppu_map_prg(0x8000, 12);
        self.ppu_map_prg(0xa000, 13);
        self.ppu.set_vblank(true);
        Ok(())
    }
}
