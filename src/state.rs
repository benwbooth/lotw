//! Game state: the CPU-visible memory image plus named accessors for it.
//!
//! The port is a faithful translation of 6502 code that addresses memory
//! uniformly: zero-page game variables, the stack page, the OAM DMA source
//! page, object/room buffers, and the MMC3-mapped PRG windows all live in one
//! flat 64 KiB space, and the original code freely indexes across it and chases
//! pointers through it. So the backing store stays a single `[u8; 0x10000]`
//! array ([`GameState::ram`]).
//!
//! What this module adds on top is *meaning*: named, documented accessor
//! methods for the individual RAM locations the game uses as discrete state
//! variables. Call sites should prefer the named accessor
//! (`state.prg_bank_8000()`) over a raw magic-number access
//! (`state.byte(0x30)`); the raw [`GameState::byte`] / [`GameState::set_byte`]
//! pair remains the documented escape hatch for the genuinely dynamic accesses
//! (computed indices, pointer dereferences, table/buffer scans) that cannot be
//! expressed as a fixed field.

/// The CPU-visible memory image and the named state living inside it.
///
/// Indexing is uniform across the whole 64 KiB address space because the
/// translated 6502 code performs cross-field indexing, aliasing, and pointer
/// dereferences that a struct of typed fields could not reproduce byte-for-byte.
pub struct GameState {
    /// Flat backing store for the entire CPU address space:
    /// `$0000-$07FF` RAM (+ mirrors), `$0100` stack page, `$0200` OAM DMA
    /// source, object/room buffers, and the MMC3-mapped PRG at `$8000-$FFFF`.
    pub ram: [u8; 0x10000],
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self { ram: [0; 0x10000] }
    }

    /// Clear all RAM back to zero. Mapped PRG/CHR is re-established by the
    /// loader, so a full zero-fill here is correct for a fresh boot.
    pub fn reset(&mut self) {
        self.ram = [0; 0x10000];
    }

    // ---- Raw byte access (escape hatch for dynamic addresses) --------------
    //
    // Use these only where the address is computed at run time (indexed table
    // reads, pointer dereferences, buffer scans). For a fixed, known location,
    // prefer the named accessor further down so the call site reads as state.

    /// Read one byte at `addr`.
    ///
    /// The read is volatile: the frame runner parks the game thread while
    /// vblank mutates RAM from the control thread, and a volatile load keeps
    /// resumed game code from reusing a stale value across that wait boundary.
    #[inline]
    pub fn byte(&self, addr: i32) -> i32 {
        let idx = (addr as usize) & 0xffff;
        unsafe { std::ptr::read_volatile(self.ram.as_ptr().add(idx)) as i32 }
    }

    /// Write the low byte of `value` at `addr` (volatile; see [`Self::byte`]).
    #[inline]
    pub fn set_byte(&mut self, addr: i32, value: i32) {
        let idx = (addr as usize) & 0xffff;
        unsafe { std::ptr::write_volatile(self.ram.as_mut_ptr().add(idx), value as u8) };
    }

    /// Add `value` to the byte at `addr` (wrapping, masked to 8 bits); returns
    /// the new value.
    #[inline]
    pub fn add_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = self.byte(addr).wrapping_add(value) & 0xff;
        self.set_byte(addr, next);
        next
    }

    /// Subtract `value` from the byte at `addr` (wrapping, masked); returns it.
    #[inline]
    pub fn sub_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = self.byte(addr).wrapping_sub(value) & 0xff;
        self.set_byte(addr, next);
        next
    }

    /// `byte &= value`; returns the new value.
    #[inline]
    pub fn and_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = self.byte(addr) & value;
        self.set_byte(addr, next);
        next
    }

    /// `byte |= value`; returns the new value.
    #[inline]
    pub fn or_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = self.byte(addr) | value;
        self.set_byte(addr, next);
        next
    }

    /// `byte ^= value`; returns the new value.
    #[inline]
    pub fn xor_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = self.byte(addr) ^ value;
        self.set_byte(addr, next);
        next
    }

    /// `byte <<= value` (masked to 8 bits); returns the new value.
    #[inline]
    pub fn shl_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = (self.byte(addr) << value) & 0xff;
        self.set_byte(addr, next);
        next
    }

    /// `byte >>= value` (logical); returns the new value.
    #[inline]
    pub fn shr_byte(&mut self, addr: i32, value: i32) -> i32 {
        let next = (self.byte(addr) & 0xff) >> value;
        self.set_byte(addr, next);
        next
    }

    /// Increment the byte at `addr` (wrapping); returns the new value.
    #[inline]
    pub fn inc_byte(&mut self, addr: i32) -> i32 {
        self.add_byte(addr, 1)
    }

    /// Decrement the byte at `addr` (wrapping); returns the new value.
    #[inline]
    pub fn dec_byte(&mut self, addr: i32) -> i32 {
        self.sub_byte(addr, 1)
    }

    // ---- MMC3 bank shadows + far-call save slots --------------------------
    //
    // The MMC3 mapper is driven through zero-page shadows that mirror the eight
    // bank registers; a per-frame committer ($D41D) replays them to hardware.
    // `$25` holds the bank-select value last written to `$8000` (which of R0-R7
    // the next `$8001` write targets); `$2A-$31` shadow R0-R7. R6 (`$30`) and
    // R7 (`$31`) select the swappable 8 KiB PRG windows at `$8000`/`$A000`.
    // Far calls into a switchable bank stash the live PRG banks in `$32`/`$33`
    // and restore them on return.

    /// MMC3 bank-select shadow (`$25`): which register R0-R7 the next bank
    /// write targets, plus the PRG/CHR mode bits.
    #[inline]
    pub fn mmc3_bank_select(&self) -> i32 {
        self.byte(0x25)
    }
    #[inline]
    pub fn set_mmc3_bank_select(&mut self, value: i32) {
        self.set_byte(0x25, value);
    }

    /// PRG bank mapped at `$8000` — MMC3 R6 shadow (`$30`).
    #[inline]
    pub fn prg_bank_8000(&self) -> i32 {
        self.byte(0x30)
    }
    #[inline]
    pub fn set_prg_bank_8000(&mut self, value: i32) {
        self.set_byte(0x30, value);
    }

    /// PRG bank mapped at `$A000` — MMC3 R7 shadow (`$31`).
    #[inline]
    pub fn prg_bank_a000(&self) -> i32 {
        self.byte(0x31)
    }
    #[inline]
    pub fn set_prg_bank_a000(&mut self, value: i32) {
        self.set_byte(0x31, value);
    }

    /// Saved R6/`$8000` PRG bank stashed across a far call (`$32`).
    #[inline]
    pub fn saved_prg_bank_8000(&self) -> i32 {
        self.byte(0x32)
    }
    #[inline]
    pub fn set_saved_prg_bank_8000(&mut self, value: i32) {
        self.set_byte(0x32, value);
    }

    /// Saved R7/`$A000` PRG bank stashed across a far call (`$33`).
    #[inline]
    pub fn saved_prg_bank_a000(&self) -> i32 {
        self.byte(0x33)
    }
    #[inline]
    pub fn set_saved_prg_bank_a000(&mut self, value: i32) {
        self.set_byte(0x33, value);
    }

    /// MMC3 CHR bank shadow for register `reg` (R0-R5), shadowed at
    /// `$2A..$2F`. R0/R1 select 2 KiB CHR windows; R2-R5 select 1 KiB windows.
    /// A per-frame committer replays these to `$8001`.
    #[inline]
    pub fn chr_bank(&self, reg: i32) -> i32 {
        self.byte(0x2A + reg)
    }
    #[inline]
    pub fn set_chr_bank(&mut self, reg: i32, value: i32) {
        self.set_byte(0x2A + reg, value);
    }

    /// PPUCTRL ($2000) shadow (`$23`): nametable/increment/sprite-size and
    /// pattern-table selection bits replayed to the PPU each frame.
    #[inline]
    pub fn ppu_ctrl_shadow(&self) -> i32 {
        self.byte(0x23)
    }
    #[inline]
    pub fn set_ppu_ctrl_shadow(&mut self, value: i32) {
        self.set_byte(0x23, value);
    }

    /// PPUMASK ($2001) shadow (`$24`): rendering-enable and emphasis bits.
    #[inline]
    pub fn ppu_mask_shadow(&self) -> i32 {
        self.byte(0x24)
    }
    #[inline]
    pub fn set_ppu_mask_shadow(&mut self, value: i32) {
        self.set_byte(0x24, value);
    }

    // ---- Controller input -------------------------------------------------

    /// Buttons held this frame (`$20`). Bit layout, LSB first:
    /// `0`=Right `1`=Left `2`=Down `3`=Up `4`=Start `5`=Select `6`=B `7`=A.
    #[inline]
    pub fn buttons(&self) -> i32 {
        self.byte(0x20)
    }
    #[inline]
    pub fn set_buttons(&mut self, value: i32) {
        self.set_byte(0x20, value);
    }

    /// Buttons newly pressed this frame (`$21`): the rising edge of
    /// [`Self::buttons`], used for one-shot actions like menu confirms.
    #[inline]
    pub fn button_chord(&self) -> i32 {
        self.byte(0x21)
    }
    #[inline]
    pub fn set_button_chord(&mut self, value: i32) {
        self.set_byte(0x21, value);
    }

    // ---- Frame sync / timers ----------------------------------------------

    /// PPUSTATUS shadow captured by the NMI (`$26`): bit7 = vblank,
    /// bit6 = sprite-0 hit (the status-bar split marker).
    #[inline]
    pub fn frame_status(&self) -> i32 {
        self.byte(0x26)
    }
    #[inline]
    pub fn set_frame_status(&mut self, value: i32) {
        self.set_byte(0x26, value);
    }

    /// True when the captured PPUSTATUS shadow reports a sprite-0 hit (`$26`
    /// bit6) — i.e. rendering reached the status-bar split this frame.
    #[inline]
    pub fn sprite0_hit(&self) -> bool {
        (self.frame_status() & 0x40) != 0
    }

    /// Foreground frame-wait countdown (`$36`): foreground code sets it to N
    /// and spins on [`Self::frame_counter_active`]; the NMI tail decrements it
    /// once per frame, so the spin releases after N frames.
    #[inline]
    pub fn frame_counter(&self) -> i32 {
        self.byte(0x36)
    }
    #[inline]
    pub fn set_frame_counter(&mut self, value: i32) {
        self.set_byte(0x36, value);
    }
    #[inline]
    pub fn frame_counter_active(&self) -> bool {
        self.frame_counter() != 0
    }

    /// Coarse timer slot `i` (0-7) in the `$85..$8C` array, each decremented
    /// once per 60 frames by `frame_counters`. Slot 0 is the sprite-blink
    /// timer ([`Self::sprite_blink_timer`]); slot 7 is the countdown timer
    /// ([`Self::countdown_timer`]).
    #[inline]
    pub fn coarse_timer(&self, i: i32) -> i32 {
        self.byte(0x85 + i)
    }
    #[inline]
    pub fn set_coarse_timer(&mut self, i: i32, value: i32) {
        self.set_byte(0x85 + i, value);
    }

    /// Sprite blink/invulnerability timer (`$85`), one of the coarse timer
    /// slots ticked once per 60 frames by the frame counters.
    #[inline]
    pub fn sprite_blink_timer(&self) -> i32 {
        self.byte(0x85)
    }
    #[inline]
    pub fn set_sprite_blink_timer(&mut self, value: i32) {
        self.set_byte(0x85, value);
    }

    /// Coarse countdown timer (`$8C`), e.g. the title-screen attract timeout.
    #[inline]
    pub fn countdown_timer(&self) -> i32 {
        self.byte(0x8C)
    }
    #[inline]
    pub fn set_countdown_timer(&mut self, value: i32) {
        self.set_byte(0x8C, value);
    }
    #[inline]
    pub fn countdown_timer_active(&self) -> bool {
        self.countdown_timer() != 0
    }

    // ---- Player vitals ----------------------------------------------------

    /// Player health/life points (`$58`).
    #[inline]
    pub fn player_health(&self) -> i32 {
        self.byte(0x58)
    }
    #[inline]
    pub fn set_player_health(&mut self, value: i32) {
        self.set_byte(0x58, value);
    }

    /// Player magic points (`$59`).
    #[inline]
    pub fn player_magic(&self) -> i32 {
        self.byte(0x59)
    }
    #[inline]
    pub fn set_player_magic(&mut self, value: i32) {
        self.set_byte(0x59, value);
    }

    // ---- Audio ------------------------------------------------------------

    /// Current/requested song id for the sound engine (`$8E`).
    #[inline]
    pub fn song(&self) -> i32 {
        self.byte(0x8E)
    }
    #[inline]
    pub fn set_song(&mut self, value: i32) {
        self.set_byte(0x8E, value);
    }

    // ---- Text/prompt UI ---------------------------------------------------

    /// Prompt/message state machine selector (`$8F`).
    #[inline]
    pub fn prompt_state(&self) -> i32 {
        self.byte(0x8F)
    }
    #[inline]
    pub fn set_prompt_state(&mut self, value: i32) {
        self.set_byte(0x8F, value);
    }

    /// Argument byte for the active prompt/message (`$90`).
    #[inline]
    pub fn prompt_argument(&self) -> i32 {
        self.byte(0x90)
    }
    #[inline]
    pub fn set_prompt_argument(&mut self, value: i32) {
        self.set_byte(0x90, value);
    }

    // ---- Current object scratch slot (`$ED..$FC`) -------------------------
    //
    // Actors, items, doors, and projectiles live as 16-byte records under
    // `$0400`. Most actor code copies the slot it is working on into this
    // scratch window (`load_object_slot_scratch`), mutates the named fields
    // below, then writes it back (`store_object_slot_scratch`). The slot
    // offset for each field is noted alongside its scratch address.

    /// Base address of the 16-byte object scratch window.
    pub const OBJ_SCRATCH_BASE: i32 = 0x00ED;

    /// Read scratch byte at slot `offset` (`0x00..=0x0F`). For the whole-slot
    /// copy helpers; prefer the named field accessors for individual fields.
    #[inline]
    pub fn obj_scratch_byte(&self, offset: i32) -> i32 {
        self.byte(Self::OBJ_SCRATCH_BASE + offset)
    }
    #[inline]
    pub fn set_obj_scratch_byte(&mut self, offset: i32, value: i32) {
        self.set_byte(Self::OBJ_SCRATCH_BASE + offset, value);
    }

    /// Sprite/tile id and animation bits — slot `+0x00` (`$ED`).
    #[inline]
    pub fn obj_tile(&self) -> i32 {
        self.byte(0xED)
    }
    #[inline]
    pub fn set_obj_tile(&mut self, value: i32) {
        self.set_byte(0xED, value);
    }

    /// Active/state/lifetime byte — slot `+0x01` (`$EE`).
    #[inline]
    pub fn obj_state(&self) -> i32 {
        self.byte(0xEE)
    }
    #[inline]
    pub fn set_obj_state(&mut self, value: i32) {
        self.set_byte(0xEE, value);
    }

    /// Attribute/direction bits — slot `+0x02` (`$EF`).
    #[inline]
    pub fn obj_attr(&self) -> i32 {
        self.byte(0xEF)
    }
    #[inline]
    pub fn set_obj_attr(&mut self, value: i32) {
        self.set_byte(0xEF, value);
    }

    /// Tile-replacement / movement scratch — slot `+0x03` (`$F0`).
    #[inline]
    pub fn obj_move_scratch(&self) -> i32 {
        self.byte(0xF0)
    }
    #[inline]
    pub fn set_obj_move_scratch(&mut self, value: i32) {
        self.set_byte(0xF0, value);
    }

    /// Cooldown / path scratch — slot `+0x04` (`$F1`).
    #[inline]
    pub fn obj_cooldown(&self) -> i32 {
        self.byte(0xF1)
    }
    #[inline]
    pub fn set_obj_cooldown(&mut self, value: i32) {
        self.set_byte(0xF1, value);
    }

    /// Health / damage threshold — slot `+0x05` (`$F2`).
    #[inline]
    pub fn obj_health(&self) -> i32 {
        self.byte(0xF2)
    }
    #[inline]
    pub fn set_obj_health(&mut self, value: i32) {
        self.set_byte(0xF2, value);
    }

    /// Timer / animation phase — slot `+0x06` (`$F3`).
    #[inline]
    pub fn obj_timer(&self) -> i32 {
        self.byte(0xF3)
    }
    #[inline]
    pub fn set_obj_timer(&mut self, value: i32) {
        self.set_byte(0xF3, value);
    }

    /// Movement/direction state bits — slot `+0x07` (`$F4`); high bit and the
    /// low two bits encode turn/animation direction state.
    #[inline]
    pub fn obj_move_state(&self) -> i32 {
        self.byte(0xF4)
    }
    #[inline]
    pub fn set_obj_move_state(&mut self, value: i32) {
        self.set_byte(0xF4, value);
    }

    /// X velocity, low nibble — slot `+0x08` (`$F5`).
    #[inline]
    pub fn obj_x_vel_lo(&self) -> i32 {
        self.byte(0xF5)
    }
    #[inline]
    pub fn set_obj_x_vel_lo(&mut self, value: i32) {
        self.set_byte(0xF5, value);
    }

    /// X velocity carry/sign — slot `+0x09` (`$F6`).
    #[inline]
    pub fn obj_x_vel_hi(&self) -> i32 {
        self.byte(0xF6)
    }
    #[inline]
    pub fn set_obj_x_vel_hi(&mut self, value: i32) {
        self.set_byte(0xF6, value);
    }

    /// Y velocity — slot `+0x0A` (`$F7`).
    #[inline]
    pub fn obj_y_vel(&self) -> i32 {
        self.byte(0xF7)
    }
    #[inline]
    pub fn set_obj_y_vel(&mut self, value: i32) {
        self.set_byte(0xF7, value);
    }

    /// Damage / effect strength — slot `+0x0B` (`$F8`).
    #[inline]
    pub fn obj_damage(&self) -> i32 {
        self.byte(0xF8)
    }
    #[inline]
    pub fn set_obj_damage(&mut self, value: i32) {
        self.set_byte(0xF8, value);
    }

    /// X sub-tile fraction — slot `+0x0C` (`$F9`).
    #[inline]
    pub fn obj_x_sub(&self) -> i32 {
        self.byte(0xF9)
    }
    #[inline]
    pub fn set_obj_x_sub(&mut self, value: i32) {
        self.set_byte(0xF9, value);
    }

    /// X tile coordinate — slot `+0x0D` (`$FA`).
    #[inline]
    pub fn obj_x_tile(&self) -> i32 {
        self.byte(0xFA)
    }
    #[inline]
    pub fn set_obj_x_tile(&mut self, value: i32) {
        self.set_byte(0xFA, value);
    }

    /// Y pixel coordinate — slot `+0x0E` (`$FB`).
    #[inline]
    pub fn obj_y_pixel(&self) -> i32 {
        self.byte(0xFB)
    }
    #[inline]
    pub fn set_obj_y_pixel(&mut self, value: i32) {
        self.set_byte(0xFB, value);
    }

    /// Extra y / sprite scratch — slot `+0x0F` (`$FC`).
    #[inline]
    pub fn obj_y_extra(&self) -> i32 {
        self.byte(0xFC)
    }
    #[inline]
    pub fn set_obj_y_extra(&mut self, value: i32) {
        self.set_byte(0xFC, value);
    }

    // ---- Zero-page pointer pairs ------------------------------------------
    //
    // Each is a 16-bit little-endian pointer split across two zero-page bytes
    // (low byte first). The combined `*_ptr` accessors fold the pair into one
    // address; the `*_ptr_lo`/`*_ptr_hi` accessors expose the individual bytes
    // for code that loads or stores them separately. Reads through these
    // pointers use `byte()`/`set_byte()` with the (now-named) pointer value.

    /// Pointer to the object slot currently being processed (`$E5`/`$E6`).
    #[inline]
    pub fn obj_slot_ptr(&self) -> i32 {
        self.byte(0xE5) | (self.byte(0xE6) << 8)
    }
    #[inline]
    pub fn set_obj_slot_ptr(&mut self, value: i32) {
        self.set_byte(0xE5, value & 0xFF);
        self.set_byte(0xE6, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn obj_slot_ptr_lo(&self) -> i32 {
        self.byte(0xE5)
    }
    #[inline]
    pub fn set_obj_slot_ptr_lo(&mut self, value: i32) {
        self.set_byte(0xE5, value);
    }
    #[inline]
    pub fn obj_slot_ptr_hi(&self) -> i32 {
        self.byte(0xE6)
    }
    #[inline]
    pub fn set_obj_slot_ptr_hi(&mut self, value: i32) {
        self.set_byte(0xE6, value);
    }

    /// Pointer to the room actor record feeding the current slot (`$E7`/`$E8`).
    #[inline]
    pub fn actor_record_ptr(&self) -> i32 {
        self.byte(0xE7) | (self.byte(0xE8) << 8)
    }
    #[inline]
    pub fn set_actor_record_ptr(&mut self, value: i32) {
        self.set_byte(0xE7, value & 0xFF);
        self.set_byte(0xE8, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn actor_record_ptr_lo(&self) -> i32 {
        self.byte(0xE7)
    }
    #[inline]
    pub fn set_actor_record_ptr_lo(&mut self, value: i32) {
        self.set_byte(0xE7, value);
    }
    #[inline]
    pub fn actor_record_ptr_hi(&self) -> i32 {
        self.byte(0xE8)
    }
    #[inline]
    pub fn set_actor_record_ptr_hi(&mut self, value: i32) {
        self.set_byte(0xE8, value);
    }

    /// Pointer to the active palette source data (`$77`/`$78`).
    #[inline]
    pub fn palette_src_ptr(&self) -> i32 {
        self.byte(0x77) | (self.byte(0x78) << 8)
    }
    #[inline]
    pub fn set_palette_src_ptr(&mut self, value: i32) {
        self.set_byte(0x77, value & 0xFF);
        self.set_byte(0x78, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn palette_src_ptr_lo(&self) -> i32 {
        self.byte(0x77)
    }
    #[inline]
    pub fn set_palette_src_ptr_lo(&mut self, value: i32) {
        self.set_byte(0x77, value);
    }
    #[inline]
    pub fn palette_src_ptr_hi(&self) -> i32 {
        self.byte(0x78)
    }
    #[inline]
    pub fn set_palette_src_ptr_hi(&mut self, value: i32) {
        self.set_byte(0x78, value);
    }

    /// Pointer to the current room's metatile table (`$79`/`$7A`).
    #[inline]
    pub fn tile_table_ptr(&self) -> i32 {
        self.byte(0x79) | (self.byte(0x7A) << 8)
    }
    #[inline]
    pub fn set_tile_table_ptr(&mut self, value: i32) {
        self.set_byte(0x79, value & 0xFF);
        self.set_byte(0x7A, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn tile_table_ptr_lo(&self) -> i32 {
        self.byte(0x79)
    }
    #[inline]
    pub fn set_tile_table_ptr_lo(&mut self, value: i32) {
        self.set_byte(0x79, value);
    }
    #[inline]
    pub fn tile_table_ptr_hi(&self) -> i32 {
        self.byte(0x7A)
    }
    #[inline]
    pub fn set_tile_table_ptr_hi(&mut self, value: i32) {
        self.set_byte(0x7A, value);
    }

    // ---- Player position / motion -----------------------------------------

    /// Player X fine (sub-tile) position (`$43`).
    #[inline]
    pub fn player_x_fine(&self) -> i32 {
        self.byte(0x43)
    }
    #[inline]
    pub fn set_player_x_fine(&mut self, value: i32) {
        self.set_byte(0x43, value);
    }

    /// Player X tile position (`$44`).
    #[inline]
    pub fn player_x_tile(&self) -> i32 {
        self.byte(0x44)
    }
    #[inline]
    pub fn set_player_x_tile(&mut self, value: i32) {
        self.set_byte(0x44, value);
    }

    /// Player Y position (`$45`).
    #[inline]
    pub fn player_y(&self) -> i32 {
        self.byte(0x45)
    }
    #[inline]
    pub fn set_player_y(&mut self, value: i32) {
        self.set_byte(0x45, value);
    }

    /// Post-landing recovery/stun countdown (`$46`); seeded from the fall
    /// distance and decremented each frame while nonzero.
    #[inline]
    pub fn landing_timer(&self) -> i32 {
        self.byte(0x46)
    }
    #[inline]
    pub fn set_landing_timer(&mut self, value: i32) {
        self.set_byte(0x46, value);
    }

    /// Map screen X (which room column the player occupies) (`$47`).
    #[inline]
    pub fn map_screen_x(&self) -> i32 {
        self.byte(0x47)
    }
    #[inline]
    pub fn set_map_screen_x(&mut self, value: i32) {
        self.set_byte(0x47, value);
    }

    /// Map screen Y (which room row the player occupies) (`$48`).
    #[inline]
    pub fn map_screen_y(&self) -> i32 {
        self.byte(0x48)
    }
    #[inline]
    pub fn set_map_screen_y(&mut self, value: i32) {
        self.set_byte(0x48, value);
    }

    /// Horizontal sub-tile movement delta for this frame (`$49`).
    #[inline]
    pub fn horizontal_subtile_delta(&self) -> i32 {
        self.byte(0x49)
    }
    #[inline]
    pub fn set_horizontal_subtile_delta(&mut self, value: i32) {
        self.set_byte(0x49, value);
    }

    /// Vertical movement delta for this frame (`$4B`).
    #[inline]
    pub fn vertical_delta(&self) -> i32 {
        self.byte(0x4B)
    }
    #[inline]
    pub fn set_vertical_delta(&mut self, value: i32) {
        self.set_byte(0x4B, value);
    }

    /// Frames the player has been falling (`$4E`).
    #[inline]
    pub fn fall_frames(&self) -> i32 {
        self.byte(0x4E)
    }
    #[inline]
    pub fn set_fall_frames(&mut self, value: i32) {
        self.set_byte(0x4E, value);
    }

    /// Remaining jump/ascent timer (`$4F`).
    #[inline]
    pub fn jump_timer(&self) -> i32 {
        self.byte(0x4F)
    }
    #[inline]
    pub fn set_jump_timer(&mut self, value: i32) {
        self.set_byte(0x4F, value);
    }

    /// Room horizontal scroll, fine (sub-tile) component (`$7B`).
    #[inline]
    pub fn scroll_fine_x(&self) -> i32 {
        self.byte(0x7B)
    }
    #[inline]
    pub fn set_scroll_fine_x(&mut self, value: i32) {
        self.set_byte(0x7B, value);
    }

    /// Room horizontal scroll, tile component (`$7C`).
    #[inline]
    pub fn scroll_tile_x(&self) -> i32 {
        self.byte(0x7C)
    }
    #[inline]
    pub fn set_scroll_tile_x(&mut self, value: i32) {
        self.set_byte(0x7C, value);
    }

    // ---- VRAM upload address ($16/$17) ------------------------------------
    //
    // Target address for the next PPU VRAM transfer. The high and low bytes
    // are written separately to PPUADDR ($2006), so the byte accessors are the
    // primary form; `vram_addr` folds the pair when a full address is handy.

    /// VRAM upload address, low byte (`$16`).
    #[inline]
    pub fn vram_addr_lo(&self) -> i32 {
        self.byte(0x16)
    }
    #[inline]
    pub fn set_vram_addr_lo(&mut self, value: i32) {
        self.set_byte(0x16, value);
    }
    /// VRAM upload address, high byte (`$17`).
    #[inline]
    pub fn vram_addr_hi(&self) -> i32 {
        self.byte(0x17)
    }
    #[inline]
    pub fn set_vram_addr_hi(&mut self, value: i32) {
        self.set_byte(0x17, value);
    }
    /// VRAM upload address as a 16-bit value (`$16` low, `$17` high).
    #[inline]
    pub fn vram_addr(&self) -> i32 {
        self.byte(0x16) | (self.byte(0x17) << 8)
    }
    #[inline]
    pub fn set_vram_addr(&mut self, value: i32) {
        self.set_byte(0x16, value & 0xFF);
        self.set_byte(0x17, (value >> 8) & 0xFF);
    }

    // ---- Resource counters / character params -----------------------------

    /// Gold/coin count (`$5A`).
    #[inline]
    pub fn coins(&self) -> i32 {
        self.byte(0x5A)
    }
    #[inline]
    pub fn set_coins(&mut self, value: i32) {
        self.set_byte(0x5A, value);
    }

    /// Key count (`$5B`).
    #[inline]
    pub fn keys(&self) -> i32 {
        self.byte(0x5B)
    }
    #[inline]
    pub fn set_keys(&mut self, value: i32) {
        self.set_byte(0x5B, value);
    }

    /// Current character's jump strength / fall-duration parameter (`$5C`):
    /// seeds the jump timer and caps accumulated fall frames.
    #[inline]
    pub fn jump_strength(&self) -> i32 {
        self.byte(0x5C)
    }
    #[inline]
    pub fn set_jump_strength(&mut self, value: i32) {
        self.set_byte(0x5C, value);
    }

    // ---- Audio / scheduler ------------------------------------------------

    /// Music volume override flag (`$92`).
    #[inline]
    pub fn music_volume_override(&self) -> i32 {
        self.byte(0x92)
    }
    #[inline]
    pub fn set_music_volume_override(&mut self, value: i32) {
        self.set_byte(0x92, value);
    }

    /// Actor scheduler phase counter (`$E9`).
    #[inline]
    pub fn scheduler_phase(&self) -> i32 {
        self.byte(0xE9)
    }
    #[inline]
    pub fn set_scheduler_phase(&mut self, value: i32) {
        self.set_byte(0xE9, value);
    }

    // ---- General indirect pointers ----------------------------------------
    //
    // The two reusable 6502 indirect-addressing pointers. `data_ptr`
    // ($0C/$0D) is predominantly the room/tile data source pointer;
    // `indirect_ptr` ($0E/$0F) is the far-call target and a general scratch
    // pointer. Both are reused per routine, so the meaning of a deref is local
    // to its caller; the names capture the dominant role.

    /// Data/source indirect pointer (`$0C`/`$0D`).
    #[inline]
    pub fn data_ptr(&self) -> i32 {
        self.byte(0x0C) | (self.byte(0x0D) << 8)
    }
    #[inline]
    pub fn set_data_ptr(&mut self, value: i32) {
        self.set_byte(0x0C, value & 0xFF);
        self.set_byte(0x0D, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn data_ptr_lo(&self) -> i32 {
        self.byte(0x0C)
    }
    #[inline]
    pub fn set_data_ptr_lo(&mut self, value: i32) {
        self.set_byte(0x0C, value);
    }
    #[inline]
    pub fn data_ptr_hi(&self) -> i32 {
        self.byte(0x0D)
    }
    #[inline]
    pub fn set_data_ptr_hi(&mut self, value: i32) {
        self.set_byte(0x0D, value);
    }

    /// General indirect / far-call target pointer (`$0E`/`$0F`).
    #[inline]
    pub fn indirect_ptr(&self) -> i32 {
        self.byte(0x0E) | (self.byte(0x0F) << 8)
    }
    #[inline]
    pub fn set_indirect_ptr(&mut self, value: i32) {
        self.set_byte(0x0E, value & 0xFF);
        self.set_byte(0x0F, (value >> 8) & 0xFF);
    }
    #[inline]
    pub fn indirect_ptr_lo(&self) -> i32 {
        self.byte(0x0E)
    }
    #[inline]
    pub fn set_indirect_ptr_lo(&mut self, value: i32) {
        self.set_byte(0x0E, value);
    }
    #[inline]
    pub fn indirect_ptr_hi(&self) -> i32 {
        self.byte(0x0F)
    }
    #[inline]
    pub fn set_indirect_ptr_hi(&mut self, value: i32) {
        self.set_byte(0x0F, value);
    }

    // ---- General-purpose scratch bytes ($08..$0B) -------------------------
    //
    // Reusable zero-page scratch the 6502 code uses as per-routine temporaries
    // (mask/shift math, holding a value across a few instructions). They carry
    // no persistent meaning; a given write/read is local to its routine, so
    // the names are intentionally generic.

    /// General-purpose scratch byte 0 (`$08`).
    #[inline]
    pub fn scratch0(&self) -> i32 {
        self.byte(0x08)
    }
    #[inline]
    pub fn set_scratch0(&mut self, value: i32) {
        self.set_byte(0x08, value);
    }

    /// General-purpose scratch byte 1 (`$09`).
    #[inline]
    pub fn scratch1(&self) -> i32 {
        self.byte(0x09)
    }
    #[inline]
    pub fn set_scratch1(&mut self, value: i32) {
        self.set_byte(0x09, value);
    }

    /// General-purpose scratch byte 2 (`$0A`).
    #[inline]
    pub fn scratch2(&self) -> i32 {
        self.byte(0x0A)
    }
    #[inline]
    pub fn set_scratch2(&mut self, value: i32) {
        self.set_byte(0x0A, value);
    }

    /// General-purpose scratch byte 3 (`$0B`).
    #[inline]
    pub fn scratch3(&self) -> i32 {
        self.byte(0x0B)
    }
    #[inline]
    pub fn set_scratch3(&mut self, value: i32) {
        self.set_byte(0x0B, value);
    }

    // ---- Scroll / nametable / status-bar split ----------------------------

    /// Horizontal scroll position in pixels (`$1C`); added to object positions
    /// to convert room coordinates to on-screen coordinates.
    #[inline]
    pub fn scroll_pixel_x(&self) -> i32 {
        self.byte(0x1C)
    }
    #[inline]
    pub fn set_scroll_pixel_x(&mut self, value: i32) {
        self.set_byte(0x1C, value);
    }

    /// Active nametable selection bit (`$1D`), toggled as the camera crosses
    /// nametable boundaries.
    #[inline]
    pub fn nametable_select(&self) -> i32 {
        self.byte(0x1D)
    }
    #[inline]
    pub fn set_nametable_select(&mut self, value: i32) {
        self.set_byte(0x1D, value);
    }

    /// Status-bar sprite-0 split enable flag (`$29`); nonzero makes the
    /// renderer split the screen for the HUD band.
    pub const STATUSBAR_SPLIT_FLAG: i32 = 0x29;
    #[inline]
    pub fn statusbar_split_flag(&self) -> i32 {
        self.byte(0x29)
    }
    #[inline]
    pub fn set_statusbar_split_flag(&mut self, value: i32) {
        self.set_byte(0x29, value);
    }

    // ---- OAM sprite buffer ($0200-$02FF) ----------------------------------
    //
    // The 64-entry sprite shadow buffer DMA'd to the PPU each frame: 4 bytes
    // per sprite (Y, tile, attribute, X). Accessors take the entry's byte
    // offset (`sprite * 4`); the field name selects the byte within the entry.

    /// Sprite Y position, entry at byte offset `off` (`$0200 + off`).
    #[inline]
    pub fn oam_y(&self, off: i32) -> i32 {
        self.byte(0x0200 + off)
    }
    #[inline]
    pub fn set_oam_y(&mut self, off: i32, value: i32) {
        self.set_byte(0x0200 + off, value);
    }
    /// Sprite tile index (`$0201 + off`).
    #[inline]
    pub fn oam_tile(&self, off: i32) -> i32 {
        self.byte(0x0201 + off)
    }
    #[inline]
    pub fn set_oam_tile(&mut self, off: i32, value: i32) {
        self.set_byte(0x0201 + off, value);
    }
    /// Sprite attribute byte (`$0202 + off`).
    #[inline]
    pub fn oam_attr(&self, off: i32) -> i32 {
        self.byte(0x0202 + off)
    }
    #[inline]
    pub fn set_oam_attr(&mut self, off: i32, value: i32) {
        self.set_byte(0x0202 + off, value);
    }
    /// Sprite X position (`$0203 + off`).
    #[inline]
    pub fn oam_x(&self, off: i32) -> i32 {
        self.byte(0x0203 + off)
    }
    #[inline]
    pub fn set_oam_x(&mut self, off: i32, value: i32) {
        self.set_byte(0x0203 + off, value);
    }

    // ---- Object table ($0400-$04BF) ---------------------------------------
    //
    // Twelve 16-byte object records (actors/items/doors/projectiles) at
    // `$0400`, stride 16. Each field uses the same layout as the scratch slot
    // ([`Self::obj_tile`] et al). Accessors take the record's byte offset
    // (`slot * 16`); the field name selects the byte within the record.

    /// Object record tile/animation byte, slot at byte offset `slot` (`$0400`).
    #[inline]
    pub fn object_tile(&self, slot: i32) -> i32 {
        self.byte(0x0400 + slot)
    }
    #[inline]
    pub fn set_object_tile(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0400 + slot, value);
    }
    /// Object active/state/lifetime byte (`$0401 + slot`).
    #[inline]
    pub fn object_state(&self, slot: i32) -> i32 {
        self.byte(0x0401 + slot)
    }
    #[inline]
    pub fn set_object_state(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0401 + slot, value);
    }
    /// Object attribute/direction byte (`$0402 + slot`).
    #[inline]
    pub fn object_attr(&self, slot: i32) -> i32 {
        self.byte(0x0402 + slot)
    }
    #[inline]
    pub fn set_object_attr(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0402 + slot, value);
    }
    /// Object tile-replacement/movement scratch (`$0403 + slot`).
    #[inline]
    pub fn object_move_scratch(&self, slot: i32) -> i32 {
        self.byte(0x0403 + slot)
    }
    #[inline]
    pub fn set_object_move_scratch(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0403 + slot, value);
    }
    /// Object health/damage threshold (`$0405 + slot`).
    #[inline]
    pub fn object_health(&self, slot: i32) -> i32 {
        self.byte(0x0405 + slot)
    }
    #[inline]
    pub fn set_object_health(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0405 + slot, value);
    }
    /// Object timer/animation phase (`$0406 + slot`).
    #[inline]
    pub fn object_timer(&self, slot: i32) -> i32 {
        self.byte(0x0406 + slot)
    }
    #[inline]
    pub fn set_object_timer(&mut self, slot: i32, value: i32) {
        self.set_byte(0x0406 + slot, value);
    }
    /// Object X sub-tile fraction (`$040C + slot`).
    #[inline]
    pub fn object_x_sub(&self, slot: i32) -> i32 {
        self.byte(0x040C + slot)
    }
    #[inline]
    pub fn set_object_x_sub(&mut self, slot: i32, value: i32) {
        self.set_byte(0x040C + slot, value);
    }
    /// Object X tile coordinate (`$040D + slot`).
    #[inline]
    pub fn object_x_tile(&self, slot: i32) -> i32 {
        self.byte(0x040D + slot)
    }
    #[inline]
    pub fn set_object_x_tile(&mut self, slot: i32, value: i32) {
        self.set_byte(0x040D + slot, value);
    }
    /// Object Y pixel coordinate (`$040E + slot`).
    #[inline]
    pub fn object_y_pixel(&self, slot: i32) -> i32 {
        self.byte(0x040E + slot)
    }
    #[inline]
    pub fn set_object_y_pixel(&mut self, slot: i32, value: i32) {
        self.set_byte(0x040E + slot, value);
    }
    /// Object extra-Y/sprite scratch (`$040F + slot`).
    #[inline]
    pub fn object_y_extra(&self, slot: i32) -> i32 {
        self.byte(0x040F + slot)
    }
    #[inline]
    pub fn set_object_y_extra(&mut self, slot: i32, value: i32) {
        self.set_byte(0x040F + slot, value);
    }

    // ---- Save state / password codec --------------------------------------
    //
    // The password subsystem packs the save state ($0300 region) into two
    // banks of nibble cells ($0322 / $0332), scrambles them via the RNG into
    // copies ($0342 / $0352), and folds in checksums. These are indexed
    // working buffers, named by region.

    /// Save-state payload byte `i` (`$0300 + i`).
    #[inline]
    pub fn save_payload(&self, i: i32) -> i32 {
        self.byte(0x0300 + i)
    }
    #[inline]
    pub fn set_save_payload(&mut self, i: i32, value: i32) {
        self.set_byte(0x0300 + i, value);
    }

    /// Save-state progress byte `i` (`$0308 + i`).
    #[inline]
    pub fn save_progress(&self, i: i32) -> i32 {
        self.byte(0x0308 + i)
    }
    #[inline]
    pub fn set_save_progress(&mut self, i: i32, value: i32) {
        self.set_byte(0x0308 + i, value);
    }

    /// Save-state inventory snapshot byte `i` (`$0310 + i`).
    #[inline]
    pub fn save_inventory(&self, i: i32) -> i32 {
        self.byte(0x0310 + i)
    }
    #[inline]
    pub fn set_save_inventory(&mut self, i: i32, value: i32) {
        self.set_byte(0x0310 + i, value);
    }

    /// Password nibble cell, bank A, index `i` (`$0322 + i`).
    #[inline]
    pub fn password_nibbles_a(&self, i: i32) -> i32 {
        self.byte(0x0322 + i)
    }
    #[inline]
    pub fn set_password_nibbles_a(&mut self, i: i32, value: i32) {
        self.set_byte(0x0322 + i, value);
    }

    /// Password nibble cell, bank B, index `i` (`$0332 + i`).
    #[inline]
    pub fn password_nibbles_b(&self, i: i32) -> i32 {
        self.byte(0x0332 + i)
    }
    #[inline]
    pub fn set_password_nibbles_b(&mut self, i: i32, value: i32) {
        self.set_byte(0x0332 + i, value);
    }

    /// Scrambled password cell, bank A, index `i` (`$0342 + i`).
    #[inline]
    pub fn password_scramble_a(&self, i: i32) -> i32 {
        self.byte(0x0342 + i)
    }
    #[inline]
    pub fn set_password_scramble_a(&mut self, i: i32, value: i32) {
        self.set_byte(0x0342 + i, value);
    }

    /// Scrambled password cell, bank B, index `i` (`$0352 + i`).
    #[inline]
    pub fn password_scramble_b(&self, i: i32) -> i32 {
        self.byte(0x0352 + i)
    }
    #[inline]
    pub fn set_password_scramble_b(&mut self, i: i32, value: i32) {
        self.set_byte(0x0352 + i, value);
    }

    /// Password additive checksum (`$0389`).
    #[inline]
    pub fn password_checksum_add(&self) -> i32 {
        self.byte(0x0389)
    }
    #[inline]
    pub fn set_password_checksum_add(&mut self, value: i32) {
        self.set_byte(0x0389, value);
    }

    /// Password XOR checksum (`$038A`).
    #[inline]
    pub fn password_checksum_xor(&self) -> i32 {
        self.byte(0x038A)
    }
    #[inline]
    pub fn set_password_checksum_xor(&mut self, value: i32) {
        self.set_byte(0x038A, value);
    }

    /// VRAM staging buffer byte `i` (`$0140 + i`): tile + attribute bytes
    /// assembled here before being uploaded to the PPU.
    #[inline]
    pub fn vram_stage(&self, i: i32) -> i32 {
        self.byte(0x0140 + i)
    }
    #[inline]
    pub fn set_vram_stage(&mut self, i: i32, value: i32) {
        self.set_byte(0x0140 + i, value);
    }

    // ---- Palette staging buffer ($0180-$019F) -----------------------------

    /// Palette staging buffer byte `i` (`$0180 + i`), the 32-byte image copied
    /// to PPU palette RAM ($3F00) on the next upload.
    #[inline]
    pub fn palette_buffer(&self, i: i32) -> i32 {
        self.byte(0x0180 + i)
    }
    #[inline]
    pub fn set_palette_buffer(&mut self, i: i32, value: i32) {
        self.set_byte(0x0180 + i, value);
    }

    /// Current object/actor slot index for iteration loops (`$E3`); shifted
    /// left 4 to form the slot's byte offset into the object table.
    #[inline]
    pub fn slot_index(&self) -> i32 {
        self.byte(0xE3)
    }
    #[inline]
    pub fn set_slot_index(&mut self, value: i32) {
        self.set_byte(0xE3, value);
    }

    /// Upper bound for the [`Self::slot_index`] iteration loop (`$E4`).
    #[inline]
    pub fn slot_index_limit(&self) -> i32 {
        self.byte(0xE4)
    }
    #[inline]
    pub fn set_slot_index_limit(&mut self, value: i32) {
        self.set_byte(0xE4, value);
    }

    /// Bitmask of available/active Drasle family members (`$41`).
    #[inline]
    pub fn family_member_mask(&self) -> i32 {
        self.byte(0x41)
    }
    #[inline]
    pub fn set_family_member_mask(&mut self, value: i32) {
        self.set_byte(0x41, value);
    }

    /// OAM buffer write cursor / current sprite byte offset (`$3F`).
    #[inline]
    pub fn oam_cursor(&self) -> i32 {
        self.byte(0x3F)
    }
    #[inline]
    pub fn set_oam_cursor(&mut self, value: i32) {
        self.set_byte(0x3F, value);
    }

    /// Player facing/direction flag (`$57`); bit6 marks the horizontal flip.
    #[inline]
    pub fn player_facing(&self) -> i32 {
        self.byte(0x57)
    }
    #[inline]
    pub fn set_player_facing(&mut self, value: i32) {
        self.set_byte(0x57, value);
    }

    /// Auxiliary stream pointer high byte (`$11`); secondary to `data_ptr`.
    #[inline]
    pub fn aux_ptr_hi(&self) -> i32 {
        self.byte(0x11)
    }
    #[inline]
    pub fn set_aux_ptr_hi(&mut self, value: i32) {
        self.set_byte(0x11, value);
    }

    /// Speed-boost / temporary-effect timer (`$89`).
    #[inline]
    pub fn boost_timer(&self) -> i32 {
        self.byte(0x89)
    }
    #[inline]
    pub fn set_boost_timer(&mut self, value: i32) {
        self.set_byte(0x89, value);
    }

    /// Temporary save slot `i` (`$80 + i`, 4 bytes): a scratch group preserved
    /// across nested calls (e.g. the shop/menu state handlers).
    #[inline]
    pub fn temp_save(&self, i: i32) -> i32 {
        self.byte(0x80 + i)
    }
    #[inline]
    pub fn set_temp_save(&mut self, i: i32, value: i32) {
        self.set_byte(0x80 + i, value);
    }

    /// Sound length/period parameter for the current note (`$05`).
    #[inline]
    pub fn sound_length(&self) -> i32 {
        self.byte(0x05)
    }
    #[inline]
    pub fn set_sound_length(&mut self, value: i32) {
        self.set_byte(0x05, value);
    }

    /// Sound engine status flag bits (`$27`).
    #[inline]
    pub fn sound_status_flags(&self) -> i32 {
        self.byte(0x27)
    }
    #[inline]
    pub fn set_sound_status_flags(&mut self, value: i32) {
        self.set_byte(0x27, value);
    }

    /// Displaced-block / temporary-tile restore timer (`$88`).
    #[inline]
    pub fn displaced_timer(&self) -> i32 {
        self.byte(0x88)
    }
    #[inline]
    pub fn set_displaced_timer(&mut self, value: i32) {
        self.set_byte(0x88, value);
    }

    /// Direction latch (`$FD`): low nibble holds the current movement
    /// direction, high nibble the previously latched one.
    #[inline]
    pub fn direction_latch(&self) -> i32 {
        self.byte(0xFD)
    }
    #[inline]
    pub fn set_direction_latch(&mut self, value: i32) {
        self.set_byte(0xFD, value);
    }

    // ---- NMI VRAM request -------------------------------------------------

    /// Pending NMI VRAM upload request id (`$28`); foreground code sets it and
    /// spins until the NMI handler drains the queued transfer back to zero.
    #[inline]
    pub fn nmi_vram_req(&self) -> i32 {
        self.byte(0x28)
    }
    #[inline]
    pub fn set_nmi_vram_req(&mut self, value: i32) {
        self.set_byte(0x28, value);
    }

    // ---- More player / render / sound scalars -----------------------------

    /// Player horizontal velocity, packed sub-tile delta + sign (`$4A`).
    #[inline]
    pub fn player_x_velocity(&self) -> i32 {
        self.byte(0x4A)
    }
    #[inline]
    pub fn set_player_x_velocity(&mut self, value: i32) {
        self.set_byte(0x4A, value);
    }

    /// Player walk-animation step counter (`$4D`); low 3 bits set the frame
    /// cadence.
    #[inline]
    pub fn anim_step_counter(&self) -> i32 {
        self.byte(0x4D)
    }
    #[inline]
    pub fn set_anim_step_counter(&mut self, value: i32) {
        self.set_byte(0x4D, value);
    }

    /// Player animation pose/frame selector (`$56`).
    #[inline]
    pub fn player_pose(&self) -> i32 {
        self.byte(0x56)
    }
    #[inline]
    pub fn set_player_pose(&mut self, value: i32) {
        self.set_byte(0x56, value);
    }

    /// Secondary VRAM transfer address, low byte (`$18`).
    #[inline]
    pub fn vram_addr2_lo(&self) -> i32 {
        self.byte(0x18)
    }
    #[inline]
    pub fn set_vram_addr2_lo(&mut self, value: i32) {
        self.set_byte(0x18, value);
    }
    /// Secondary VRAM transfer address, high byte (`$19`).
    #[inline]
    pub fn vram_addr2_hi(&self) -> i32 {
        self.byte(0x19)
    }
    #[inline]
    pub fn set_vram_addr2_hi(&mut self, value: i32) {
        self.set_byte(0x19, value);
    }

    /// Vertical scroll value written to PPUSCROLL `$2005` (`$1E`).
    #[inline]
    pub fn scroll_y(&self) -> i32 {
        self.byte(0x1E)
    }
    #[inline]
    pub fn set_scroll_y(&mut self, value: i32) {
        self.set_byte(0x1E, value);
    }

    /// Sprite slot index/counter while building the OAM buffer (`$3E`).
    #[inline]
    pub fn sprite_index(&self) -> i32 {
        self.byte(0x3E)
    }
    #[inline]
    pub fn set_sprite_index(&mut self, value: i32) {
        self.set_byte(0x3E, value);
    }

    /// Sound channel active/control flags (`$A4`).
    #[inline]
    pub fn sound_channel_flags(&self) -> i32 {
        self.byte(0xA4)
    }
    #[inline]
    pub fn set_sound_channel_flags(&mut self, value: i32) {
        self.set_byte(0xA4, value);
    }

    // ---- Sound engine channel state ---------------------------------------
    //
    // Per-channel playback state lives in 16-byte records starting at `$93`
    // (one per APU channel), indexed by a channel byte offset. Known fields
    // within a record: +2/+3 current pattern pointer lo/hi, +4/+5 loop pointer
    // lo/hi, +6 duty/volume, +8 envelope offset.

    /// Byte `field` (0-15) of the sound channel record at byte offset `ch`
    /// (`$93 + field + ch`).
    #[inline]
    pub fn sound_channel_byte(&self, field: i32, ch: i32) -> i32 {
        self.byte(0x93 + field + ch)
    }
    #[inline]
    pub fn set_sound_channel_byte(&mut self, field: i32, ch: i32, value: i32) {
        self.set_byte(0x93 + field + ch, value);
    }

    /// Sound channel byte offset currently being processed (`$02`).
    #[inline]
    pub fn sound_channel_offset(&self) -> i32 {
        self.byte(0x02)
    }
    #[inline]
    pub fn set_sound_channel_offset(&mut self, value: i32) {
        self.set_byte(0x02, value);
    }

    /// Current sound command id (`$04`).
    #[inline]
    pub fn sound_command(&self) -> i32 {
        self.byte(0x04)
    }
    #[inline]
    pub fn set_sound_command(&mut self, value: i32) {
        self.set_byte(0x04, value);
    }

    // ---- RNG state ($38..$3B) ---------------------------------------------
    //
    // `rng_update` advances a 16-bit LFSR-style seed ($3A low, $3B high) mixed
    // with a saved previous low byte ($39), and rejection-samples below a
    // requested limit ($38). $3B is also the returned random byte.

    /// Requested RNG range/limit for the current draw (`$38`).
    #[inline]
    pub fn rng_limit(&self) -> i32 {
        self.byte(0x38)
    }
    #[inline]
    pub fn set_rng_limit(&mut self, value: i32) {
        self.set_byte(0x38, value);
    }

    /// Saved previous seed low byte mixed into the next draw (`$39`).
    #[inline]
    pub fn rng_seed_scratch(&self) -> i32 {
        self.byte(0x39)
    }
    #[inline]
    pub fn set_rng_seed_scratch(&mut self, value: i32) {
        self.set_byte(0x39, value);
    }

    /// RNG seed, low byte (`$3A`).
    #[inline]
    pub fn rng_low(&self) -> i32 {
        self.byte(0x3A)
    }
    #[inline]
    pub fn set_rng_low(&mut self, value: i32) {
        self.set_byte(0x3A, value);
    }

    /// RNG seed, high byte; also the value returned by a draw (`$3B`).
    #[inline]
    pub fn rng_high(&self) -> i32 {
        self.byte(0x3B)
    }
    #[inline]
    pub fn set_rng_high(&mut self, value: i32) {
        self.set_byte(0x3B, value);
    }

    // ---- Misc player / UI / frame state -----------------------------------

    /// Current character index (which Drasle family member is active) (`$40`).
    #[inline]
    pub fn character_index(&self) -> i32 {
        self.byte(0x40)
    }
    #[inline]
    pub fn set_character_index(&mut self, value: i32) {
        self.set_byte(0x40, value);
    }

    /// Equipped item slot `i` (`$51 + i`), selected by the inventory cursor
    /// ([`Self::selected_item_slot`]).
    #[inline]
    pub fn item_slot(&self, i: i32) -> i32 {
        self.byte(0x51 + i)
    }
    #[inline]
    pub fn set_item_slot(&mut self, i: i32, value: i32) {
        self.set_byte(0x51 + i, value);
    }

    /// Inventory item byte `i` in the 16-entry inventory table (`$0060 + i`).
    #[inline]
    pub fn inventory_item(&self, i: i32) -> i32 {
        self.byte(0x0060 + i)
    }
    #[inline]
    pub fn set_inventory_item(&mut self, i: i32, value: i32) {
        self.set_byte(0x0060 + i, value);
    }

    /// Selected inventory/menu item slot (cursor index) (`$55`).
    #[inline]
    pub fn selected_item_slot(&self) -> i32 {
        self.byte(0x55)
    }
    #[inline]
    pub fn set_selected_item_slot(&mut self, value: i32) {
        self.set_byte(0x55, value);
    }

    /// Continue/respawn countdown timer (`$37`).
    #[inline]
    pub fn continue_timer(&self) -> i32 {
        self.byte(0x37)
    }
    #[inline]
    pub fn set_continue_timer(&mut self, value: i32) {
        self.set_byte(0x37, value);
    }

    /// 60-frame prescaler (`$84`): reloads to 0x3C and counts down each frame;
    /// its low bits drive blink/animation cadence and the coarse timer ticks.
    #[inline]
    pub fn frame_prescaler(&self) -> i32 {
        self.byte(0x84)
    }
    #[inline]
    pub fn set_frame_prescaler(&mut self, value: i32) {
        self.set_byte(0x84, value);
    }
}
