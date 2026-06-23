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
macro_rules! byte_field {
    ($get:ident, $set:ident, $addr:expr, $doc:expr) => {
        #[doc = $doc]
        #[inline]
        pub fn $get(&self) -> i32 {
            self.byte($addr)
        }
        #[inline]
        pub fn $set(&mut self, value: i32) {
            self.set_byte($addr, value);
        }
    };
}
macro_rules! array_field {
    ($get:ident, $set:ident, $base:expr, $doc:expr) => {
        #[doc = $doc]
        #[inline]
        pub fn $get(&self, i: i32) -> i32 {
            self.byte($base + i)
        }
        #[inline]
        pub fn $set(&mut self, i: i32, value: i32) {
            self.set_byte($base + i, value);
        }
    };
}

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

    byte_field!(
        mmc3_bank_select,
        set_mmc3_bank_select,
        0x25,
        "MMC3 bank-select shadow (`$25`): which register R0-R7 the next bank write targets, plus the PRG/CHR mode bits."
    );

    byte_field!(
        prg_bank_8000,
        set_prg_bank_8000,
        0x30,
        "PRG bank mapped at `$8000` — MMC3 R6 shadow (`$30`)."
    );

    byte_field!(
        prg_bank_a000,
        set_prg_bank_a000,
        0x31,
        "PRG bank mapped at `$A000` — MMC3 R7 shadow (`$31`)."
    );

    byte_field!(
        saved_prg_bank_8000,
        set_saved_prg_bank_8000,
        0x32,
        "Saved R6/`$8000` PRG bank stashed across a far call (`$32`)."
    );

    byte_field!(
        saved_prg_bank_a000,
        set_saved_prg_bank_a000,
        0x33,
        "Saved R7/`$A000` PRG bank stashed across a far call (`$33`)."
    );

    array_field!(
        chr_bank,
        set_chr_bank,
        0x2A,
        "MMC3 CHR bank shadow for register `reg` (R0-R5), shadowed at `$2A..$2F`. R0/R1 select 2 KiB CHR windows; R2-R5 select 1 KiB windows. A per-frame committer replays these to `$8001`."
    );

    byte_field!(
        ppu_ctrl_shadow,
        set_ppu_ctrl_shadow,
        0x23,
        "PPUCTRL ($2000) shadow (`$23`): nametable/increment/sprite-size and pattern-table selection bits replayed to the PPU each frame."
    );

    byte_field!(
        ppu_mask_shadow,
        set_ppu_mask_shadow,
        0x24,
        "PPUMASK ($2001) shadow (`$24`): rendering-enable and emphasis bits."
    );

    // ---- Controller input -------------------------------------------------

    byte_field!(
        buttons,
        set_buttons,
        0x20,
        "Buttons held this frame (`$20`). Bit layout, LSB first: `0`=Right `1`=Left `2`=Down `3`=Up `4`=Start `5`=Select `6`=B `7`=A."
    );

    byte_field!(
        button_chord,
        set_button_chord,
        0x21,
        "Buttons newly pressed this frame (`$21`): the rising edge of [`Self::buttons`], used for one-shot actions like menu confirms."
    );

    // ---- Frame sync / timers ----------------------------------------------

    byte_field!(
        frame_status,
        set_frame_status,
        0x26,
        "PPUSTATUS shadow captured by the NMI (`$26`): bit7 = vblank, bit6 = sprite-0 hit (the status-bar split marker)."
    );

    /// True when the captured PPUSTATUS shadow reports a sprite-0 hit (`$26`
    /// bit6) — i.e. rendering reached the status-bar split this frame.
    #[inline]
    pub fn sprite0_hit(&self) -> bool {
        (self.frame_status() & 0x40) != 0
    }

    byte_field!(
        frame_counter,
        set_frame_counter,
        0x36,
        "Foreground frame-wait countdown (`$36`): foreground code sets it to N and spins on [`Self::frame_counter_active`]; the NMI tail decrements it once per frame, so the spin releases after N frames."
    );
    #[inline]
    pub fn frame_counter_active(&self) -> bool {
        self.frame_counter() != 0
    }

    array_field!(
        coarse_timer,
        set_coarse_timer,
        0x85,
        "Coarse timer slot `i` (0-7) in the `$85..$8C` array, each decremented once per 60 frames by `frame_counters`. Slot 0 is the sprite-blink timer ([`Self::sprite_blink_timer`]); slot 7 is the countdown timer ([`Self::countdown_timer`])."
    );

    byte_field!(
        sprite_blink_timer,
        set_sprite_blink_timer,
        0x85,
        "Sprite blink/invulnerability timer (`$85`), one of the coarse timer slots ticked once per 60 frames by the frame counters."
    );

    byte_field!(
        countdown_timer,
        set_countdown_timer,
        0x8C,
        "Coarse countdown timer (`$8C`), e.g. the title-screen attract timeout."
    );
    #[inline]
    pub fn countdown_timer_active(&self) -> bool {
        self.countdown_timer() != 0
    }

    // ---- Player vitals ----------------------------------------------------

    byte_field!(
        player_health,
        set_player_health,
        0x58,
        "Player health/life points (`$58`)."
    );

    byte_field!(
        player_magic,
        set_player_magic,
        0x59,
        "Player magic points (`$59`)."
    );

    // ---- Audio ------------------------------------------------------------

    byte_field!(
        song,
        set_song,
        0x8E,
        "Current/requested song id for the sound engine (`$8E`)."
    );

    // ---- Text/prompt UI ---------------------------------------------------

    byte_field!(
        prompt_state,
        set_prompt_state,
        0x8F,
        "Prompt/message state machine selector (`$8F`)."
    );

    byte_field!(
        prompt_argument,
        set_prompt_argument,
        0x90,
        "Argument byte for the active prompt/message (`$90`)."
    );

    // ---- Current object scratch slot (`$ED..$FC`) -------------------------
    //
    // Actors, items, doors, and projectiles live as 16-byte records under
    // `$0400`. Most actor code copies the slot it is working on into this
    // scratch window (`load_object_slot_scratch`), mutates the named fields
    // below, then writes it back (`store_object_slot_scratch`). The slot
    // offset for each field is noted alongside its scratch address.

    /// Base address of the 16-byte object scratch window.
    pub const OBJ_SCRATCH_BASE: i32 = 0x00ED;

    array_field!(
        obj_scratch_byte,
        set_obj_scratch_byte,
        Self::OBJ_SCRATCH_BASE,
        "Read scratch byte at slot `offset` (`0x00..=0x0F`). For the whole-slot copy helpers; prefer the named field accessors for individual fields."
    );

    byte_field!(
        obj_tile,
        set_obj_tile,
        0xED,
        "Sprite/tile id and animation bits — slot `+0x00` (`$ED`)."
    );

    byte_field!(
        obj_state,
        set_obj_state,
        0xEE,
        "Active/state/lifetime byte — slot `+0x01` (`$EE`)."
    );

    byte_field!(
        obj_attr,
        set_obj_attr,
        0xEF,
        "Attribute/direction bits — slot `+0x02` (`$EF`)."
    );

    byte_field!(
        obj_move_scratch,
        set_obj_move_scratch,
        0xF0,
        "Tile-replacement / movement scratch — slot `+0x03` (`$F0`)."
    );

    byte_field!(
        obj_cooldown,
        set_obj_cooldown,
        0xF1,
        "Cooldown / path scratch — slot `+0x04` (`$F1`)."
    );

    byte_field!(
        obj_health,
        set_obj_health,
        0xF2,
        "Health / damage threshold — slot `+0x05` (`$F2`)."
    );

    byte_field!(
        obj_timer,
        set_obj_timer,
        0xF3,
        "Timer / animation phase — slot `+0x06` (`$F3`)."
    );

    byte_field!(
        obj_move_state,
        set_obj_move_state,
        0xF4,
        "Movement/direction state bits — slot `+0x07` (`$F4`); high bit and the low two bits encode turn/animation direction state."
    );

    byte_field!(
        obj_x_vel_lo,
        set_obj_x_vel_lo,
        0xF5,
        "X velocity, low nibble — slot `+0x08` (`$F5`)."
    );

    byte_field!(
        obj_x_vel_hi,
        set_obj_x_vel_hi,
        0xF6,
        "X velocity carry/sign — slot `+0x09` (`$F6`)."
    );

    byte_field!(
        obj_y_vel,
        set_obj_y_vel,
        0xF7,
        "Y velocity — slot `+0x0A` (`$F7`)."
    );

    byte_field!(
        obj_damage,
        set_obj_damage,
        0xF8,
        "Damage / effect strength — slot `+0x0B` (`$F8`)."
    );

    byte_field!(
        obj_x_sub,
        set_obj_x_sub,
        0xF9,
        "X sub-tile fraction — slot `+0x0C` (`$F9`)."
    );

    byte_field!(
        obj_x_tile,
        set_obj_x_tile,
        0xFA,
        "X tile coordinate — slot `+0x0D` (`$FA`)."
    );

    byte_field!(
        obj_y_pixel,
        set_obj_y_pixel,
        0xFB,
        "Y pixel coordinate — slot `+0x0E` (`$FB`)."
    );

    byte_field!(
        obj_y_extra,
        set_obj_y_extra,
        0xFC,
        "Extra y / sprite scratch — slot `+0x0F` (`$FC`)."
    );

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
    byte_field!(obj_slot_ptr_lo, set_obj_slot_ptr_lo, 0xE5, "");
    byte_field!(obj_slot_ptr_hi, set_obj_slot_ptr_hi, 0xE6, "");

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
    byte_field!(actor_record_ptr_lo, set_actor_record_ptr_lo, 0xE7, "");
    byte_field!(actor_record_ptr_hi, set_actor_record_ptr_hi, 0xE8, "");

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
    byte_field!(palette_src_ptr_lo, set_palette_src_ptr_lo, 0x77, "");
    byte_field!(palette_src_ptr_hi, set_palette_src_ptr_hi, 0x78, "");

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
    byte_field!(tile_table_ptr_lo, set_tile_table_ptr_lo, 0x79, "");
    byte_field!(tile_table_ptr_hi, set_tile_table_ptr_hi, 0x7A, "");

    // ---- Player position / motion -----------------------------------------

    byte_field!(
        player_x_fine,
        set_player_x_fine,
        0x43,
        "Player X fine (sub-tile) position (`$43`)."
    );

    byte_field!(
        player_x_tile,
        set_player_x_tile,
        0x44,
        "Player X tile position (`$44`)."
    );

    byte_field!(player_y, set_player_y, 0x45, "Player Y position (`$45`).");

    byte_field!(
        landing_timer,
        set_landing_timer,
        0x46,
        "Post-landing recovery/stun countdown (`$46`); seeded from the fall distance and decremented each frame while nonzero."
    );

    byte_field!(
        map_screen_x,
        set_map_screen_x,
        0x47,
        "Map screen X (which room column the player occupies) (`$47`)."
    );

    byte_field!(
        map_screen_y,
        set_map_screen_y,
        0x48,
        "Map screen Y (which room row the player occupies) (`$48`)."
    );

    byte_field!(
        horizontal_subtile_delta,
        set_horizontal_subtile_delta,
        0x49,
        "Horizontal sub-tile movement delta for this frame (`$49`)."
    );

    byte_field!(
        vertical_delta,
        set_vertical_delta,
        0x4B,
        "Vertical movement delta for this frame (`$4B`)."
    );

    byte_field!(
        fall_frames,
        set_fall_frames,
        0x4E,
        "Frames the player has been falling (`$4E`)."
    );

    byte_field!(
        jump_timer,
        set_jump_timer,
        0x4F,
        "Remaining jump/ascent timer (`$4F`)."
    );

    byte_field!(
        scroll_fine_x,
        set_scroll_fine_x,
        0x7B,
        "Room horizontal scroll, fine (sub-tile) component (`$7B`)."
    );

    byte_field!(
        scroll_tile_x,
        set_scroll_tile_x,
        0x7C,
        "Room horizontal scroll, tile component (`$7C`)."
    );

    // ---- VRAM upload address ($16/$17) ------------------------------------
    //
    // Target address for the next PPU VRAM transfer. The high and low bytes
    // are written separately to PPUADDR ($2006), so the byte accessors are the
    // primary form; `vram_addr` folds the pair when a full address is handy.

    byte_field!(
        vram_addr_lo,
        set_vram_addr_lo,
        0x16,
        "VRAM upload address, low byte (`$16`)."
    );
    byte_field!(
        vram_addr_hi,
        set_vram_addr_hi,
        0x17,
        "VRAM upload address, high byte (`$17`)."
    );
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

    byte_field!(coins, set_coins, 0x5A, "Gold/coin count (`$5A`).");

    byte_field!(keys, set_keys, 0x5B, "Key count (`$5B`).");

    byte_field!(
        jump_strength,
        set_jump_strength,
        0x5C,
        "Current character's jump strength / fall-duration parameter (`$5C`): seeds the jump timer and caps accumulated fall frames."
    );

    // ---- Audio / scheduler ------------------------------------------------

    byte_field!(
        music_volume_override,
        set_music_volume_override,
        0x92,
        "Music volume override flag (`$92`)."
    );

    byte_field!(
        scheduler_phase,
        set_scheduler_phase,
        0xE9,
        "Actor scheduler phase counter (`$E9`)."
    );

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
    byte_field!(data_ptr_lo, set_data_ptr_lo, 0x0C, "");
    byte_field!(data_ptr_hi, set_data_ptr_hi, 0x0D, "");

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
    byte_field!(indirect_ptr_lo, set_indirect_ptr_lo, 0x0E, "");
    byte_field!(indirect_ptr_hi, set_indirect_ptr_hi, 0x0F, "");

    // ---- General-purpose scratch bytes ($08..$0B) -------------------------
    //
    // Reusable zero-page scratch the 6502 code uses as per-routine temporaries
    // (mask/shift math, holding a value across a few instructions). They carry
    // no persistent meaning; a given write/read is local to its routine, so
    // the names are intentionally generic.

    byte_field!(
        scratch0,
        set_scratch0,
        0x08,
        "General-purpose scratch byte 0 (`$08`)."
    );

    byte_field!(
        scratch1,
        set_scratch1,
        0x09,
        "General-purpose scratch byte 1 (`$09`)."
    );

    byte_field!(
        scratch2,
        set_scratch2,
        0x0A,
        "General-purpose scratch byte 2 (`$0A`)."
    );

    byte_field!(
        scratch3,
        set_scratch3,
        0x0B,
        "General-purpose scratch byte 3 (`$0B`)."
    );

    // ---- Scroll / nametable / status-bar split ----------------------------

    byte_field!(
        scroll_pixel_x,
        set_scroll_pixel_x,
        0x1C,
        "Horizontal scroll position in pixels (`$1C`); added to object positions to convert room coordinates to on-screen coordinates."
    );

    byte_field!(
        nametable_select,
        set_nametable_select,
        0x1D,
        "Active nametable selection bit (`$1D`), toggled as the camera crosses nametable boundaries."
    );

    /// Status-bar sprite-0 split enable flag (`$29`); nonzero makes the
    /// renderer split the screen for the HUD band.
    pub const STATUSBAR_SPLIT_FLAG: i32 = 0x29;
    byte_field!(statusbar_split_flag, set_statusbar_split_flag, 0x29, "");

    // ---- OAM sprite buffer ($0200-$02FF) ----------------------------------
    //
    // The 64-entry sprite shadow buffer DMA'd to the PPU each frame: 4 bytes
    // per sprite (Y, tile, attribute, X). Accessors take the entry's byte
    // offset (`sprite * 4`); the field name selects the byte within the entry.

    array_field!(
        oam_y,
        set_oam_y,
        0x0200,
        "Sprite Y position, entry at byte offset `off` (`$0200 + off`)."
    );
    array_field!(
        oam_tile,
        set_oam_tile,
        0x0201,
        "Sprite tile index (`$0201 + off`)."
    );
    array_field!(
        oam_attr,
        set_oam_attr,
        0x0202,
        "Sprite attribute byte (`$0202 + off`)."
    );
    array_field!(
        oam_x,
        set_oam_x,
        0x0203,
        "Sprite X position (`$0203 + off`)."
    );

    // ---- Object table ($0400-$04BF) ---------------------------------------
    //
    // Twelve 16-byte object records (actors/items/doors/projectiles) at
    // `$0400`, stride 16. Each field uses the same layout as the scratch slot
    // ([`Self::obj_tile`] et al). Accessors take the record's byte offset
    // (`slot * 16`); the field name selects the byte within the record.

    array_field!(
        object_tile,
        set_object_tile,
        0x0400,
        "Object record tile/animation byte, slot at byte offset `slot` (`$0400`)."
    );
    array_field!(
        object_state,
        set_object_state,
        0x0401,
        "Object active/state/lifetime byte (`$0401 + slot`)."
    );
    array_field!(
        object_attr,
        set_object_attr,
        0x0402,
        "Object attribute/direction byte (`$0402 + slot`)."
    );
    array_field!(
        object_move_scratch,
        set_object_move_scratch,
        0x0403,
        "Object tile-replacement/movement scratch (`$0403 + slot`)."
    );
    array_field!(
        object_health,
        set_object_health,
        0x0405,
        "Object health/damage threshold (`$0405 + slot`)."
    );
    array_field!(
        object_timer,
        set_object_timer,
        0x0406,
        "Object timer/animation phase (`$0406 + slot`)."
    );
    array_field!(
        object_x_sub,
        set_object_x_sub,
        0x040C,
        "Object X sub-tile fraction (`$040C + slot`)."
    );
    array_field!(
        object_x_tile,
        set_object_x_tile,
        0x040D,
        "Object X tile coordinate (`$040D + slot`)."
    );
    array_field!(
        object_y_pixel,
        set_object_y_pixel,
        0x040E,
        "Object Y pixel coordinate (`$040E + slot`)."
    );
    array_field!(
        object_y_extra,
        set_object_y_extra,
        0x040F,
        "Object extra-Y/sprite scratch (`$040F + slot`)."
    );

    // ---- Save state / password codec --------------------------------------
    //
    // The password subsystem packs the save state ($0300 region) into two
    // banks of nibble cells ($0322 / $0332), scrambles them via the RNG into
    // copies ($0342 / $0352), and folds in checksums. These are indexed
    // working buffers, named by region.

    array_field!(
        save_payload,
        set_save_payload,
        0x0300,
        "Save-state payload byte `i` (`$0300 + i`)."
    );

    array_field!(
        save_progress,
        set_save_progress,
        0x0308,
        "Save-state progress byte `i` (`$0308 + i`)."
    );

    array_field!(
        save_inventory,
        set_save_inventory,
        0x0310,
        "Save-state inventory snapshot byte `i` (`$0310 + i`)."
    );

    array_field!(
        password_nibbles_a,
        set_password_nibbles_a,
        0x0322,
        "Password nibble cell, bank A, index `i` (`$0322 + i`)."
    );

    array_field!(
        password_nibbles_b,
        set_password_nibbles_b,
        0x0332,
        "Password nibble cell, bank B, index `i` (`$0332 + i`)."
    );

    array_field!(
        password_scramble_a,
        set_password_scramble_a,
        0x0342,
        "Scrambled password cell, bank A, index `i` (`$0342 + i`)."
    );

    array_field!(
        password_scramble_b,
        set_password_scramble_b,
        0x0352,
        "Scrambled password cell, bank B, index `i` (`$0352 + i`)."
    );

    byte_field!(
        password_checksum_add,
        set_password_checksum_add,
        0x0389,
        "Password additive checksum (`$0389`)."
    );

    byte_field!(
        password_checksum_xor,
        set_password_checksum_xor,
        0x038A,
        "Password XOR checksum (`$038A`)."
    );

    array_field!(
        vram_stage,
        set_vram_stage,
        0x0140,
        "VRAM staging buffer byte `i` (`$0140 + i`): tile + attribute bytes assembled here before being uploaded to the PPU."
    );

    // ---- Palette staging buffer ($0180-$019F) -----------------------------

    array_field!(
        room_buffer,
        set_room_buffer,
        0x0500,
        "Room layout buffer byte `i` (`$0500 + i`): the staged room tile/metadata working area ($0500-$07FF)."
    );

    array_field!(
        palette_buffer,
        set_palette_buffer,
        0x0180,
        "Palette staging buffer byte `i` (`$0180 + i`), the 32-byte image copied to PPU palette RAM ($3F00) on the next upload."
    );

    byte_field!(
        saved_audio_handler_lo,
        set_saved_audio_handler_lo,
        0x06,
        "Saved audio stream command handler pointer, low/high (`$06`/`$07`)."
    );
    byte_field!(saved_audio_handler_hi, set_saved_audio_handler_hi, 0x07, "");

    byte_field!(
        inventory_upload_row,
        set_inventory_upload_row,
        0x1B,
        "Row cursor while uploading the inventory item list (`$1B`)."
    );

    byte_field!(
        collision_flag,
        set_collision_flag,
        0x22,
        "Collision/blocked flag set during movement resolution (`$22`)."
    );

    byte_field!(
        song_ptr_lo,
        set_song_ptr_lo,
        0x34,
        "Current song pointer low/high used by `song_init` (`$34`/`$35`)."
    );
    byte_field!(song_ptr_hi, set_song_ptr_hi, 0x35, "");

    byte_field!(
        main_loop_phase,
        set_main_loop_phase,
        0x3D,
        "Main-loop dispatch phase counter (`$3D`)."
    );

    byte_field!(
        nudge_pending,
        set_nudge_pending,
        0x4C,
        "\"Nudge to tile boundary\" pending flag (`$4C`)."
    );

    byte_field!(
        projectile_damage,
        set_projectile_damage,
        0x5D,
        "Effective projectile damage / count / lifetime parameters (`$5D`/`$5E`/`$5F`)."
    );
    byte_field!(projectile_count, set_projectile_count, 0x5E, "");
    byte_field!(projectile_lifetime, set_projectile_lifetime, 0x5F, "");

    byte_field!(
        shop_active,
        set_shop_active,
        0x61,
        "Shop room active flag (`$61`)."
    );

    byte_field!(
        fragment_count,
        set_fragment_count,
        0x6E,
        "Remaining fragment-pickup count (`$6E`)."
    );

    byte_field!(
        text_attr_ptr_lo,
        set_text_attr_ptr_lo,
        0x70,
        "Text attribute source pointer low/high (`$70`/`$71`)."
    );
    byte_field!(text_attr_ptr_hi, set_text_attr_ptr_hi, 0x71, "");

    byte_field!(
        room_tile_action,
        set_room_tile_action,
        0x74,
        "Decoded room tile action value (`$74`)."
    );

    byte_field!(
        room_metadef_lo,
        set_room_metadef_lo,
        0x75,
        "Room metatile-definition pointer low/high (`$75`/`$76`)."
    );
    byte_field!(room_metadef_hi, set_room_metadef_hi, 0x76, "");

    byte_field!(
        saved_scroll_tile,
        set_saved_scroll_tile,
        0x7E,
        "Saved horizontal scroll tile during the main loop (`$7E`)."
    );

    byte_field!(
        camera_scroll_flag,
        set_camera_scroll_flag,
        0x7F,
        "Camera scroll-pending flag (`$7F`)."
    );

    byte_field!(
        airborne_flag,
        set_airborne_flag,
        0x86,
        "Airborne/falling flag for the top-boundary exit check (`$86`)."
    );

    byte_field!(
        magic_contact_flag,
        set_magic_contact_flag,
        0x87,
        "Magic-contact-with-actor flag (`$87`)."
    );

    byte_field!(
        short_boost_timer,
        set_short_boost_timer,
        0x8A,
        "Short / long speed-boost timers (`$8A`/`$8B`)."
    );
    byte_field!(long_boost_timer, set_long_boost_timer, 0x8B, "");

    byte_field!(
        sfx_priority,
        set_sfx_priority,
        0x91,
        "Active SFX priority threshold (`$91`)."
    );

    byte_field!(
        pending_special_exit,
        set_pending_special_exit,
        0xEB,
        "Pending special-exit-room flag checked by `game_update` (`$EB`)."
    );

    byte_field!(
        sound_paused,
        set_sound_paused,
        0x8D,
        "Sound paused/disabled flag checked at the top of `sound_tick` (`$8D`)."
    );

    byte_field!(
        sfx_voice_active,
        set_sfx_voice_active,
        0xD4,
        "SFX overlay voice active flag, bit7 (`$D4`)."
    );

    byte_field!(
        triangle_timer,
        set_triangle_timer,
        0xB3,
        "Triangle channel note-duration countdown (`$B3`)."
    );

    byte_field!(
        audio_duty_work,
        set_audio_duty_work,
        0x00,
        "Audio duty/volume work byte for the duty/instrument command (`$00`)."
    );

    byte_field!(
        hud_refresh_flag,
        set_hud_refresh_flag,
        0x3C,
        "HUD refresh-needed flag set by `sync_health_hud` (`$3C`)."
    );

    byte_field!(
        title_timer,
        set_title_timer,
        0x42,
        "Title-screen loop timer (`$42`)."
    );

    byte_field!(
        inventory_upload_col,
        set_inventory_upload_col,
        0x1A,
        "Column cursor while uploading the inventory item list (`$1A`)."
    );

    byte_field!(
        overlap_flag,
        set_overlap_flag,
        0xEA,
        "Player/actor overlap-detected flag (`$EA`)."
    );

    byte_field!(
        final_exit_flag,
        set_final_exit_flag,
        0xEC,
        "Final-exit trigger reached flag (`$EC`)."
    );

    byte_field!(
        pose_state,
        set_pose_state,
        0x50,
        "Pose state flag from `update_player_pose_from_motion` (`$50`)."
    );

    byte_field!(
        tile_fetch_counter,
        set_tile_fetch_counter,
        0x10,
        "Countdown used while resolving the room tile pointer (`$10`)."
    );

    byte_field!(
        room_restore_scratch,
        set_room_restore_scratch,
        0xFE,
        "Scratch byte used by `restore_room_from_checkpoint` (`$FE`)."
    );

    byte_field!(
        slot_index,
        set_slot_index,
        0xE3,
        "Current object/actor slot index for iteration loops (`$E3`); shifted left 4 to form the slot's byte offset into the object table."
    );

    byte_field!(
        slot_index_limit,
        set_slot_index_limit,
        0xE4,
        "Upper bound for the [`Self::slot_index`] iteration loop (`$E4`)."
    );

    byte_field!(
        family_member_mask,
        set_family_member_mask,
        0x41,
        "Bitmask of available/active Drasle family members (`$41`)."
    );

    byte_field!(
        oam_cursor,
        set_oam_cursor,
        0x3F,
        "OAM buffer write cursor / current sprite byte offset (`$3F`)."
    );

    byte_field!(
        player_facing,
        set_player_facing,
        0x57,
        "Player facing/direction flag (`$57`); bit6 marks the horizontal flip."
    );

    byte_field!(
        aux_ptr_hi,
        set_aux_ptr_hi,
        0x11,
        "Auxiliary stream pointer high byte (`$11`); secondary to `data_ptr`."
    );

    byte_field!(
        boost_timer,
        set_boost_timer,
        0x89,
        "Speed-boost / temporary-effect timer (`$89`)."
    );

    array_field!(
        temp_save,
        set_temp_save,
        0x80,
        "Temporary save slot `i` (`$80 + i`, 4 bytes): a scratch group preserved across nested calls (e.g. the shop/menu state handlers)."
    );

    byte_field!(
        sound_length,
        set_sound_length,
        0x05,
        "Sound length/period parameter for the current note (`$05`)."
    );

    byte_field!(
        sound_status_flags,
        set_sound_status_flags,
        0x27,
        "Sound engine status flag bits (`$27`)."
    );

    byte_field!(
        displaced_timer,
        set_displaced_timer,
        0x88,
        "Displaced-block / temporary-tile restore timer (`$88`)."
    );

    byte_field!(
        direction_latch,
        set_direction_latch,
        0xFD,
        "Direction latch (`$FD`): low nibble holds the current movement direction, high nibble the previously latched one."
    );

    // ---- NMI VRAM request -------------------------------------------------

    byte_field!(
        nmi_vram_req,
        set_nmi_vram_req,
        0x28,
        "Pending NMI VRAM upload request id (`$28`); foreground code sets it and spins until the NMI handler drains the queued transfer back to zero."
    );

    // ---- More player / render / sound scalars -----------------------------

    byte_field!(
        player_x_velocity,
        set_player_x_velocity,
        0x4A,
        "Player horizontal velocity, packed sub-tile delta + sign (`$4A`)."
    );

    byte_field!(
        anim_step_counter,
        set_anim_step_counter,
        0x4D,
        "Player walk-animation step counter (`$4D`); low 3 bits set the frame cadence."
    );

    byte_field!(
        player_pose,
        set_player_pose,
        0x56,
        "Player animation pose/frame selector (`$56`)."
    );

    byte_field!(
        vram_addr2_lo,
        set_vram_addr2_lo,
        0x18,
        "Secondary VRAM transfer address, low byte (`$18`)."
    );
    byte_field!(
        vram_addr2_hi,
        set_vram_addr2_hi,
        0x19,
        "Secondary VRAM transfer address, high byte (`$19`)."
    );

    byte_field!(
        scroll_y,
        set_scroll_y,
        0x1E,
        "Vertical scroll value written to PPUSCROLL `$2005` (`$1E`)."
    );

    byte_field!(
        sprite_index,
        set_sprite_index,
        0x3E,
        "Sprite slot index/counter while building the OAM buffer (`$3E`)."
    );

    byte_field!(
        sound_channel_flags,
        set_sound_channel_flags,
        0xA4,
        "Sound channel active/control flags (`$A4`)."
    );

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
    /// Decrement a sound channel field (wrapping) and return the new value.
    #[inline]
    pub fn dec_sound_channel_byte(&mut self, field: i32, ch: i32) -> i32 {
        let v = (self.sound_channel_byte(field, ch) - 1) & 0xFF;
        self.set_sound_channel_byte(field, ch, v);
        v
    }

    byte_field!(
        sound_channel_offset,
        set_sound_channel_offset,
        0x02,
        "Sound channel byte offset currently being processed (`$02`)."
    );

    byte_field!(
        sound_command,
        set_sound_command,
        0x04,
        "Current sound command id (`$04`)."
    );

    // ---- RNG state ($38..$3B) ---------------------------------------------
    //
    // `rng_update` advances a 16-bit LFSR-style seed ($3A low, $3B high) mixed
    // with a saved previous low byte ($39), and rejection-samples below a
    // requested limit ($38). $3B is also the returned random byte.

    byte_field!(
        rng_limit,
        set_rng_limit,
        0x38,
        "Requested RNG range/limit for the current draw (`$38`)."
    );

    byte_field!(
        rng_seed_scratch,
        set_rng_seed_scratch,
        0x39,
        "Saved previous seed low byte mixed into the next draw (`$39`)."
    );

    byte_field!(rng_low, set_rng_low, 0x3A, "RNG seed, low byte (`$3A`).");

    byte_field!(
        rng_high,
        set_rng_high,
        0x3B,
        "RNG seed, high byte; also the value returned by a draw (`$3B`)."
    );

    // ---- Misc player / UI / frame state -----------------------------------

    byte_field!(
        character_index,
        set_character_index,
        0x40,
        "Current character index (which Drasle family member is active) (`$40`)."
    );

    array_field!(
        item_slot,
        set_item_slot,
        0x51,
        "Equipped item slot `i` (`$51 + i`), selected by the inventory cursor ([`Self::selected_item_slot`])."
    );

    array_field!(
        inventory_item,
        set_inventory_item,
        0x0060,
        "Inventory item byte `i` in the 16-entry inventory table (`$0060 + i`)."
    );

    byte_field!(
        selected_item_slot,
        set_selected_item_slot,
        0x55,
        "Selected inventory/menu item slot (cursor index) (`$55`)."
    );

    byte_field!(
        continue_timer,
        set_continue_timer,
        0x37,
        "Continue/respawn countdown timer (`$37`)."
    );

    byte_field!(
        frame_prescaler,
        set_frame_prescaler,
        0x84,
        "60-frame prescaler (`$84`): reloads to 0x3C and counts down each frame; its low bits drive blink/animation cadence and the coarse timer ticks."
    );
}
