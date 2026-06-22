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
}
