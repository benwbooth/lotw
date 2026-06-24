use crate::{apu::Apu, ppu::Ppu, state::GameState};

/// Visible NES picture width in pixels (one PPU scanline is 256 dots).
pub const PPU_W: usize = 256;
/// Visible NES picture height in pixels (240 rendered scanlines).
pub const PPU_H: usize = 240;
/// Audio output sample rate in Hz used by the APU mixer.
pub const APU_SR: usize = 44_100;

/// Memory-mapped NES hardware register addresses, accessed via
/// [`Engine::device_read`] / [`Engine::device_write`]. Named so call sites read
/// as register operations rather than raw addresses.
pub mod reg {
    // PPU registers ($2000-$2007), mirrored every 8 bytes through $3FFF.
    /// PPUCTRL ($2000): VRAM-increment, nametable, sprite/bg pattern, NMI enable.
    pub const PPU_CTRL: i32 = 0x2000;
    /// PPUMASK ($2001): rendering enable, color emphasis, greyscale.
    pub const PPU_MASK: i32 = 0x2001;
    /// PPUSTATUS ($2002): vblank, sprite-0 hit, overflow; read clears wtoggle.
    pub const PPU_STATUS: i32 = 0x2002;
    /// OAMADDR ($2003): sets the OAM byte index used by OAM_DATA.
    pub const OAM_ADDR: i32 = 0x2003;
    /// OAMDATA ($2004): read/write the OAM byte at OAMADDR, auto-incrementing.
    pub const OAM_DATA: i32 = 0x2004;
    /// PPUSCROLL ($2005): two-write X then Y fine-scroll latch.
    pub const PPU_SCROLL: i32 = 0x2005;
    /// PPUADDR ($2006): two-write high then low VRAM address latch.
    pub const PPU_ADDR: i32 = 0x2006;
    /// PPUDATA ($2007): read/write VRAM at the latched address, auto-incrementing.
    pub const PPU_DATA: i32 = 0x2007;
    // APU pulse 1
    /// SQ1_VOL ($4000): pulse 1 duty / envelope / volume.
    pub const SQ1_VOL: i32 = 0x4000;
    /// SQ1_SWEEP ($4001): pulse 1 frequency sweep unit.
    pub const SQ1_SWEEP: i32 = 0x4001;
    /// SQ1_LO ($4002): pulse 1 timer low 8 bits.
    pub const SQ1_LO: i32 = 0x4002;
    /// SQ1_HI ($4003): pulse 1 timer high bits + length-counter load.
    pub const SQ1_HI: i32 = 0x4003;
    // APU pulse 2
    /// SQ2_VOL ($4004): pulse 2 duty / envelope / volume.
    pub const SQ2_VOL: i32 = 0x4004;
    /// SQ2_SWEEP ($4005): pulse 2 frequency sweep unit.
    pub const SQ2_SWEEP: i32 = 0x4005;
    /// SQ2_LO ($4006): pulse 2 timer low 8 bits.
    pub const SQ2_LO: i32 = 0x4006;
    /// SQ2_HI ($4007): pulse 2 timer high bits + length-counter load.
    pub const SQ2_HI: i32 = 0x4007;
    // APU triangle
    /// TRI_LINEAR ($4008): triangle linear-counter control.
    pub const TRI_LINEAR: i32 = 0x4008;
    /// TRI_LO ($400A): triangle timer low 8 bits.
    pub const TRI_LO: i32 = 0x400A;
    /// TRI_HI ($400B): triangle timer high bits + length-counter load.
    pub const TRI_HI: i32 = 0x400B;
    // APU noise
    /// NOISE_VOL ($400C): noise envelope / volume.
    pub const NOISE_VOL: i32 = 0x400C;
    /// NOISE_LO ($400E): noise period / mode.
    pub const NOISE_LO: i32 = 0x400E;
    /// NOISE_HI ($400F): noise length-counter load.
    pub const NOISE_HI: i32 = 0x400F;
    // APU DMC / status / frame counter, OAM DMA, controllers
    /// DMC_FREQ ($4010): delta-modulation channel rate / IRQ / loop.
    pub const DMC_FREQ: i32 = 0x4010;
    /// OAMDMA ($4014): writing a page number triggers a 256-byte OAM transfer.
    pub const OAM_DMA: i32 = 0x4014;
    /// APU_STATUS ($4015): channel enable (write) / status (read).
    pub const APU_STATUS: i32 = 0x4015;
    /// JOY1 ($4016): controller 1 strobe (write) / serial read (read).
    pub const JOY1: i32 = 0x4016;
    /// APU_FRAME ($4017): APU frame-counter mode / IRQ inhibit.
    pub const APU_FRAME: i32 = 0x4017;
    // MMC3 mapper registers (writes to ROM space are intercepted by the mapper)
    /// MMC3 bank-select ($8000, even): chooses which bank register $8001 updates.
    pub const MMC3_BANK_SELECT: i32 = 0x8000;
    /// MMC3 bank-data ($8001, odd): value for the bank selected by $8000.
    pub const MMC3_BANK_DATA: i32 = 0x8001;
    /// MMC3 mirroring ($A000, even): nametable mirroring select.
    pub const MMC3_MIRROR: i32 = 0xA000;
    /// MMC3 PRG-RAM protect ($A001, odd): PRG-RAM enable / write-protect.
    pub const MMC3_PRG_RAM: i32 = 0xA001;
    /// MMC3 IRQ disable ($E000, even): acknowledge and disable the scanline IRQ.
    pub const MMC3_IRQ_DISABLE: i32 = 0xE000;
    /// MMC3 IRQ enable ($E001, odd): enable the scanline IRQ.
    pub const MMC3_IRQ_ENABLE: i32 = 0xE001;
}

/// Software model of the 6502 register/flag state threaded through the ported
/// game routines. Each ported subroutine takes a `&mut RoutineContext` in place
/// of the real CPU's registers and processor-status flags, so control flow that
/// originally branched on the 6502 flags can be reproduced faithfully. Flag
/// fields hold 0 (clear) or 1 (set) rather than packing into a status byte.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoutineContext {
    /// Accumulator (6502 `A`): the primary arithmetic/data register.
    pub value: u8,
    /// Index register `X`: loop counter and indexed-addressing offset.
    pub index: u8,
    /// Index register `Y`: second index, used for indirect-indexed addressing.
    pub offset: u8,
    /// Carry flag (`C`): 1 after an unsigned overflow / set by `SEC`/shift-out.
    pub carry: u8,
    /// Zero flag (`Z`): 1 when the last result was zero.
    pub zero: u8,
    /// Negative flag (`N`): 1 when bit 7 of the last result was set.
    pub negative: u8,
    /// Overflow flag (`V`): 1 on signed overflow of an add/subtract.
    pub overflow: u8,
}

/// Signature of a ported game subroutine: it mutates the shared [`Engine`]
/// (memory/PPU/APU) and the per-call 6502 register/flag [`RoutineContext`].
pub type RoutineFn = fn(&mut Engine, &mut RoutineContext);

/// The whole emulated machine: the CPU memory image, the PPU and APU devices,
/// and the small amount of host-side glue (input injection, APU tracing) needed
/// to drive the ported game. Acts as the `&mut self` passed to every routine.
pub struct Engine {
    /// CPU-visible memory image plus named game-state accessors.
    pub state: GameState,
    /// Picture Processing Unit: registers, VRAM, OAM, palette, MMC3 CHR banking.
    pub ppu: Ppu,
    /// Audio Processing Unit: channel registers and the sample mixer.
    pub apu: Apu,
    /// Port-specific scratch used to carry a control-flow handoff across the
    /// non-local jumps in the original code that don't map to plain returns.
    pub lotw_nonlocal_handoff: i32,
    /// Stack of saved room checkpoints: 4 slots of 7 bytes each, used to
    /// push/pop room state around transitions.
    pub room_ckpt_stack: [[u8; 7]; 4],
    /// Stack pointer (next free slot index) into `room_ckpt_stack`.
    pub room_ckpt_sp: usize,
    /// Optional host hook supplying the next controller-input byte, letting a
    /// test/replay harness feed deterministic input instead of live polling.
    next_input: Option<Box<dyn FnMut() -> i32 + Send>>,
    /// Optional host hook invoked on every APU register write `(addr, value)`,
    /// used to record an audio-register trace for diffing/playback.
    apu_trace: Option<Box<dyn FnMut(i32, i32) + Send>>,
}

impl Default for Engine {
    /// Construct a fresh [`Engine`] via [`Engine::new`] (no ROM loaded yet).
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    /// Create a powered-on but unloaded machine: zeroed game state, reset PPU
    /// and APU, empty checkpoint stack, and no host input/trace hooks installed.
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

    /// Clear the CPU memory image back to its reset state and drop any pending
    /// non-local handoff. Does not touch loaded ROM/CHR or PPU/APU device state.
    pub fn reset_memory(&mut self) {
        self.state.reset();
        self.lotw_nonlocal_handoff = 0;
    }

    // Memory is accessed through `self.state` (a `GameState`): named accessors
    // for known locations, or `state.byte`/`set_byte` for genuinely dynamic
    // addresses. The old `engine.mem(addr)` delegates have been fully removed
    // now that every call site uses the labeled API.

    /// Install a host callback that supplies the next controller-input byte,
    /// overriding live controller polling (used for deterministic replay/tests).
    pub fn set_next_input<F>(&mut self, next: F)
    where
        F: FnMut() -> i32 + Send + 'static,
    {
        self.next_input = Some(Box::new(next));
    }

    /// Remove any installed input callback, restoring normal controller polling.
    pub fn clear_next_input(&mut self) {
        self.next_input = None;
    }

    /// Pull the next injected input byte, if a callback is installed. The result
    /// is masked to 8 bits (`& 255`) since controller state is one byte.
    pub fn next_input(&mut self) -> Option<i32> {
        self.next_input.as_mut().map(|next| next() & 255)
    }

    /// Install a host callback invoked on every APU register write as
    /// `(addr, value)`, used to capture an audio-register trace.
    pub fn set_apu_trace<F>(&mut self, trace: F)
    where
        F: FnMut(i32, i32) + Send + 'static,
    {
        self.apu_trace = Some(Box::new(trace));
    }

    /// Remove any installed APU trace callback.
    pub fn clear_apu_trace(&mut self) {
        self.apu_trace = None;
    }

    /// Forward an APU register write to the APU, first notifying the trace hook
    /// (if any) with the normalized 16-bit address and 8-bit value.
    pub fn apu_write_with_trace(&mut self, addr: i32, value: i32) {
        // Report the write to the trace hook with canonical widths: 16-bit
        // address (& 65535) and 8-bit value (& 255).
        if let Some(trace) = self.apu_trace.as_mut() {
            trace(addr & 65535, value & 255);
        }
        self.apu.write(addr, value);
    }

    /// Decode a CPU write to a memory-mapped device register and apply its
    /// side effects on the PPU, APU, OAM-DMA engine, or MMC3 mapper.
    ///
    /// `addr` is normalized to 16 bits and `value` to 8 bits, matching the
    /// NES bus widths. The function reproduces the hardware's write semantics,
    /// including the PPU's shared write toggle, VRAM auto-increment, palette
    /// mirroring, and the mapper's bank latching.
    pub fn device_write(&mut self, addr: i32, value: i32) {
        // Normalize to bus widths: 16-bit address, 8-bit data.
        let addr = addr & 65535;
        let value = value & 255;
        // Any write into the PPU register window ($2000-$2007) refreshes the
        // PPU open-bus latch with the value last driven on the bus.
        if (reg::PPU_CTRL..=reg::PPU_DATA).contains(&addr) {
            self.ppu.openbus = value as u8;
        }
        match addr {
            // PPUCTRL / PPUMASK / OAMADDR are simple latched registers.
            reg::PPU_CTRL => self.ppu.ctrl = value as u8,
            reg::PPU_MASK => self.ppu.mask = value as u8,
            reg::OAM_ADDR => self.ppu.oamaddr = value as u8,
            reg::OAM_DATA => {
                // Store at the current OAM index, then post-increment OAMADDR
                // (wrapping within the 256-byte OAM, hence the u8 wrap).
                let idx = self.ppu.oamaddr as usize;
                self.ppu.oam[idx] = value as u8;
                self.ppu.oamaddr = self.ppu.oamaddr.wrapping_add(1);
            }
            reg::PPU_SCROLL => {
                // Two-write register sharing `wtoggle`: first write = X scroll,
                // second write = Y scroll; the toggle flips after each write.
                if self.ppu.wtoggle == 0 {
                    self.ppu.scroll_x = value as u8;
                    self.ppu.wtoggle = 1;
                } else {
                    self.ppu.scroll_y = value as u8;
                    self.ppu.wtoggle = 0;
                }
            }
            reg::PPU_ADDR => {
                // Two-write VRAM address latch: first write sets the high byte,
                // second sets the low byte (sharing `wtoggle` with PPUSCROLL).
                if self.ppu.wtoggle == 0 {
                    // Keep low 8 bits (& 255), replace high byte (<< 8).
                    self.ppu.vaddr = (self.ppu.vaddr & 255) | ((value as u16) << 8);
                    self.ppu.wtoggle = 1;
                } else {
                    // Keep high byte (& 65280 = 0xFF00), replace low byte.
                    self.ppu.vaddr = (self.ppu.vaddr & 65280) | value as u16;
                    self.ppu.wtoggle = 0;
                }
            }
            reg::PPU_DATA => {
                // Address wraps within the 16 KiB PPU space (& 16383 = 0x3FFF).
                let a = self.ppu.vaddr & 16383;
                if a >= 16128 {
                    // $3F00-$3FFF: palette RAM. Index within 32-entry palette.
                    let mut p = a & 31;
                    if (p & 3) == 0 {
                        // Every 4th entry ($3F00/$04/$08/$0C) is a shared
                        // background/sprite mirror: fold to the 16-entry bank
                        // (& 15) and write both halves (| 16 = sprite mirror).
                        p &= 15;
                        self.ppu.pal[p as usize] = value as u8;
                        self.ppu.pal[(p | 16) as usize] = value as u8;
                    } else {
                        self.ppu.pal[p as usize] = value as u8;
                    }
                } else {
                    // $0000-$3EFF: nametable/CHR VRAM via mirroring resolver.
                    let off = self.ppu.nt_addr_offset(a);
                    self.ppu.vram[off] = value as u8;
                }
                // Auto-increment by 32 (one row) when PPUCTRL bit 2 is set,
                // otherwise by 1 (one column).
                self.ppu.vaddr =
                    self.ppu
                        .vaddr
                        .wrapping_add(if (self.ppu.ctrl & 4) != 0 { 32 } else { 1 });
            }
            reg::OAM_DMA => {
                // Writing page N copies CPU $N00-$NFF into OAM. base = N << 8.
                let base = (value as usize) << 8;
                for i in 0..256 {
                    // Destination wraps in OAM relative to current OAMADDR;
                    // source wraps in the 64 KiB CPU image (& 65535).
                    let dst = self.ppu.oamaddr.wrapping_add(i as u8) as usize;
                    self.ppu.oam[dst] = self.state.ram_bytes()[(base + i) & 65535];
                }
            }
            reg::MMC3_BANK_SELECT => {
                // Latch which bank register the next $8001 write targets, then
                // recompute the CHR window mapping.
                self.ppu.mmc3_sel = value as u8;
                self.ppu.recompute_chr();
            }
            reg::MMC3_BANK_DATA => {
                // Low 3 bits of the select register pick one of 8 bank slots.
                let sel = self.ppu.mmc3_sel & 7;
                self.ppu.mmc3_bank[sel as usize] = value as u8;
                self.ppu.recompute_chr();
                // Slots 6 and 7 are the two switchable 8 KiB PRG-ROM windows;
                // map slot 6 to CPU $8000 and slot 7 to CPU $A000.
                if sel == 6 {
                    self.ppu_map_prg(32768, value);
                } else if sel == 7 {
                    self.ppu_map_prg(40960, value);
                }
            }
            // Mirroring select: bit 0 chooses horizontal (0) vs vertical (1).
            reg::MMC3_MIRROR => self.ppu.mirror = if (value & 1) != 0 { 0 } else { 1 },
            reg::JOY1 => {
                // Controller strobe in bit 0; while high, reload the shift latch
                // from the current button state so reads start from button 0.
                self.ppu.strobe = (value & 1) as u8;
                if self.ppu.strobe != 0 {
                    self.ppu.ctrl_latch = self.ppu.buttons;
                }
            }
            _ => {
                // Remaining APU register range ($4000-$4017), excluding OAM_DMA
                // ($4014) which is handled above as a PPU transfer.
                if (reg::SQ1_VOL..=reg::APU_FRAME).contains(&addr) && addr != reg::OAM_DMA {
                    self.apu_write_with_trace(addr, value);
                }
            }
        }
    }

    /// Decode a CPU read from a memory-mapped device register and return the
    /// byte the hardware would drive on the bus, applying read side effects
    /// (PPUSTATUS clears vblank + write toggle, PPUDATA increments and buffers,
    /// JOY1 shifts the controller latch). Unmapped reads return 0.
    pub fn device_read(&mut self, addr: i32) -> i32 {
        // Normalize to a 16-bit bus address.
        let addr = addr & 65535;
        match addr {
            reg::PPU_STATUS => {
                // Top 3 flag bits (& 224 = 0xE0) come from the status register;
                // the low 5 bits (& 31) reflect the PPU open-bus.
                let s = (self.ppu.status & 224) | (self.ppu.openbus & 31);
                // Reading clears the vblank flag (bit 7, & !128) and resets the
                // shared PPUSCROLL/PPUADDR write toggle.
                self.ppu.status &= !128;
                self.ppu.wtoggle = 0;
                s as i32
            }
            reg::OAM_DATA => {
                // Return the OAM byte at the current OAMADDR (no auto-increment
                // on read); also refresh the open-bus latch.
                let ret = self.ppu.oam[self.ppu.oamaddr as usize];
                self.ppu.openbus = ret;
                ret as i32
            }
            reg::PPU_DATA => {
                // Address wraps within the 16 KiB PPU space (& 16383 = 0x3FFF).
                let a = self.ppu.vaddr & 16383;
                let ret = if a >= 16128 {
                    // $3F00-$3FFF palette reads are immediate (not buffered).
                    let mut p = a & 31;
                    if (p & 3) == 0 {
                        // Fold mirrored entries to the 16-entry bank (& 15).
                        p &= 15;
                    }
                    self.ppu.pal[p as usize]
                } else {
                    // Non-palette reads return the previously buffered byte and
                    // then refill the buffer from the resolved VRAM address.
                    let ret = self.ppu.readbuf;
                    self.ppu.readbuf = self.ppu.vram[self.ppu.nt_addr_offset(a)];
                    ret
                };
                // Auto-increment by 32 (row) if PPUCTRL bit 2 set, else 1 (col).
                self.ppu.vaddr =
                    self.ppu
                        .vaddr
                        .wrapping_add(if (self.ppu.ctrl & 4) != 0 { 32 } else { 1 });
                self.ppu.openbus = ret;
                ret as i32
            }
            reg::JOY1 => {
                // While strobing, always report button 0 (bit 0); otherwise
                // shift the serial controller latch out one bit per read.
                if self.ppu.strobe != 0 {
                    (self.ppu.buttons & 1) as i32
                } else {
                    let bit = self.ppu.ctrl_latch & 1;
                    self.ppu.ctrl_latch >>= 1;
                    bit as i32
                }
            }
            // Frame-counter read is not modeled here; report 0.
            reg::APU_FRAME => 0,
            // Any other address is unmapped from the device decoder's view.
            _ => 0,
        }
    }

    /// Hand the PRG-ROM image to the PPU/mapper subsystem (which owns the ROM
    /// banks used by [`Engine::ppu_map_prg`]).
    pub fn ppu_load_prg(&mut self, prg: &[u8]) {
        self.ppu.load_prg(prg);
    }

    /// Hand the CHR-ROM (character/pattern) image to the PPU subsystem.
    pub fn ppu_load_chr(&mut self, chr: &[u8]) {
        self.ppu.load_chr(chr);
    }

    /// Copy an 8 KiB PRG-ROM bank into the CPU memory image so that subsequent
    /// CPU reads see the banked code/data (this port shadows ROM into RAM rather
    /// than dispatching through a mapper on every fetch).
    ///
    /// `cpu_base` is the destination CPU address (e.g. 0x8000 or 0xA000) and
    /// `bank8k` is the 8 KiB bank number, taken modulo the number of banks.
    pub fn ppu_map_prg(&mut self, cpu_base: i32, bank8k: i32) {
        // Nothing to map if no PRG-ROM has been loaded yet.
        if self.ppu.prg_len == 0 {
            return;
        }
        // Number of 8 KiB banks, and the source offset of the requested bank
        // (wrapped via % so out-of-range bank numbers alias like the mapper).
        let nbanks = self.ppu.prg_len / 8192;
        let off = ((bank8k as usize) % nbanks) * 8192;
        // Destination offset in the 64 KiB CPU image (& 65535).
        let dst = (cpu_base as usize) & 65535;
        // Blit the 8 KiB (8192-byte) bank into place.
        self.state.ram_bytes_mut()[dst..dst + 8192].copy_from_slice(&self.ppu.prg[off..off + 8192]);
    }

    /// Re-apply the two switchable PRG windows from the game state's recorded
    /// bank numbers: slot at CPU $8000 (32768) and slot at CPU $A000 (40960).
    /// Used to restore the banking after loading/restoring state.
    pub fn prg_map_shadow(&mut self) {
        self.ppu_map_prg(32768, ((self.state.prg_bank_8000) as i32));
        self.ppu_map_prg(40960, ((self.state.prg_bank_a000) as i32));
    }

    /// Parse an iNES (`.nes`) ROM image, load its PRG/CHR banks, reset the
    /// devices, install the initial MMC3 bank layout, and bring the machine up
    /// to a post-power-on state ready to run.
    ///
    /// When `init_ram_pattern` is set, work RAM is filled with the typical
    /// power-on $00/$FF pattern instead of being left zeroed. Returns an error
    /// for non-iNES or truncated images.
    pub fn load_ines(&mut self, rom: &[u8], init_ram_pattern: bool) -> Result<(), String> {
        // Validate the 16-byte iNES header and its "NES" magic.
        if rom.len() < 16 || &rom[0..3] != b"NES" {
            return Err("not an iNES file".to_string());
        }
        // Header byte 4 = PRG size in 16 KiB units; byte 5 = CHR in 8 KiB units.
        let prg = rom[4] as usize * 16_384;
        let chr = rom[5] as usize * 8_192;
        // Ensure the file actually contains the header + both ROM regions.
        if rom.len() < 16 + prg + chr {
            return Err("short ROM".to_string());
        }
        if init_ram_pattern {
            // Fill the 2 KiB ($0800) work RAM with the power-on pattern: bytes
            // whose address bit 2 is set read as $FF, others as $00.
            for a in 0..2048 {
                self.state.ram_bytes_mut()[a] = if (a & 4) != 0 { 255 } else { 0 };
            }
        }
        // PRG follows the 16-byte header; CHR follows the PRG region.
        self.ppu_load_prg(&rom[16..16 + prg]);
        self.ppu_load_chr(&rom[16 + prg..16 + prg + chr]);
        self.ppu.reset();
        self.apu.reset();
        // MMC3 hard-wires the last 16 KiB of PRG to CPU $C000-$FFFF
        // (49152..65536). Source it from the final 16 KiB of the PRG image.
        self.state.ram_bytes_mut()[49152..65536].copy_from_slice(&rom[16 + prg - 16384..16 + prg]);
        // Initial switchable banks: 8 KiB bank 12 at $8000, bank 13 at $A000.
        self.ppu_map_prg(32768, 12);
        self.ppu_map_prg(40960, 13);
        // Start inside vblank so the reset code's vblank-wait completes.
        self.ppu.set_vblank(true);
        Ok(())
    }
}
