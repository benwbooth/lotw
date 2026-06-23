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

#[repr(C)]
pub struct GameState {
    pub audio_duty_work: u8,
    _pad0: [u8; 1],
    pub sound_channel_offset: u8,
    _pad1: [u8; 1],
    pub sound_command: u8,
    pub sound_length: u8,
    pub saved_audio_handler_lo: u8,
    pub saved_audio_handler_hi: u8,
    pub scratch0: u8,
    pub scratch1: u8,
    pub scratch2: u8,
    pub scratch3: u8,
    pub data_ptr_lo: u8,
    pub data_ptr_hi: u8,
    pub indirect_ptr_lo: u8,
    pub indirect_ptr_hi: u8,
    pub tile_fetch_counter: u8,
    pub aux_ptr_hi: u8,
    _pad2: [u8; 4],
    pub vram_addr_lo: u8,
    pub vram_addr_hi: u8,
    pub vram_addr2_lo: u8,
    pub vram_addr2_hi: u8,
    pub inventory_upload_col: u8,
    pub inventory_upload_row: u8,
    pub scroll_pixel_x: u8,
    pub nametable_select: u8,
    pub scroll_y: u8,
    _pad3: [u8; 1],
    pub buttons: u8,
    pub button_chord: u8,
    pub collision_flag: u8,
    pub ppu_ctrl_shadow: u8,
    pub ppu_mask_shadow: u8,
    pub mmc3_bank_select: u8,
    pub frame_status: u8,
    pub sound_status_flags: u8,
    pub nmi_vram_req: u8,
    pub statusbar_split_flag: u8,
    _pad4: [u8; 6],
    pub prg_bank_8000: u8,
    pub prg_bank_a000: u8,
    pub saved_prg_bank_8000: u8,
    pub saved_prg_bank_a000: u8,
    pub song_ptr_lo: u8,
    pub song_ptr_hi: u8,
    pub frame_counter: u8,
    pub continue_timer: u8,
    pub rng_limit: u8,
    pub rng_seed_scratch: u8,
    pub rng_low: u8,
    pub rng_high: u8,
    pub hud_refresh_flag: u8,
    pub main_loop_phase: u8,
    pub sprite_index: u8,
    pub oam_cursor: u8,
    pub character_index: u8,
    pub family_member_mask: u8,
    pub title_timer: u8,
    pub player_x_fine: u8,
    pub player_x_tile: u8,
    pub player_y: u8,
    pub landing_timer: u8,
    pub map_screen_x: u8,
    pub map_screen_y: u8,
    pub horizontal_subtile_delta: u8,
    pub player_x_velocity: u8,
    pub vertical_delta: u8,
    pub nudge_pending: u8,
    pub anim_step_counter: u8,
    pub fall_frames: u8,
    pub jump_timer: u8,
    pub pose_state: u8,
    _pad5: [u8; 4],
    pub selected_item_slot: u8,
    pub player_pose: u8,
    pub player_facing: u8,
    pub player_health: u8,
    pub player_magic: u8,
    pub coins: u8,
    pub keys: u8,
    pub jump_strength: u8,
    pub projectile_damage: u8,
    pub projectile_count: u8,
    pub projectile_lifetime: u8,
    _pad6: [u8; 1],
    pub shop_active: u8,
    _pad7: [u8; 12],
    pub fragment_count: u8,
    _pad8: [u8; 1],
    pub text_attr_ptr_lo: u8,
    pub text_attr_ptr_hi: u8,
    _pad9: [u8; 2],
    pub room_tile_action: u8,
    pub room_metadef_lo: u8,
    pub room_metadef_hi: u8,
    pub palette_src_ptr_lo: u8,
    pub palette_src_ptr_hi: u8,
    pub tile_table_ptr_lo: u8,
    pub tile_table_ptr_hi: u8,
    pub scroll_fine_x: u8,
    pub scroll_tile_x: u8,
    _pad10: [u8; 1],
    pub saved_scroll_tile: u8,
    pub camera_scroll_flag: u8,
    _pad11: [u8; 4],
    pub frame_prescaler: u8,
    pub sprite_blink_timer: u8,
    pub airborne_flag: u8,
    pub magic_contact_flag: u8,
    pub displaced_timer: u8,
    pub boost_timer: u8,
    pub short_boost_timer: u8,
    pub long_boost_timer: u8,
    pub countdown_timer: u8,
    pub sound_paused: u8,
    pub song: u8,
    pub prompt_state: u8,
    pub prompt_argument: u8,
    pub sfx_priority: u8,
    pub music_volume_override: u8,
    _pad12: [u8; 17],
    pub sound_channel_flags: u8,
    _pad13: [u8; 14],
    pub triangle_timer: u8,
    _pad14: [u8; 32],
    pub sfx_voice_active: u8,
    _pad15: [u8; 14],
    pub slot_index: u8,
    pub slot_index_limit: u8,
    pub obj_slot_ptr_lo: u8,
    pub obj_slot_ptr_hi: u8,
    pub actor_record_ptr_lo: u8,
    pub actor_record_ptr_hi: u8,
    pub scheduler_phase: u8,
    pub overlap_flag: u8,
    pub pending_special_exit: u8,
    pub final_exit_flag: u8,
    pub obj_tile: u8,
    pub obj_state: u8,
    pub obj_attr: u8,
    pub obj_move_scratch: u8,
    pub obj_cooldown: u8,
    pub obj_health: u8,
    pub obj_timer: u8,
    pub obj_move_state: u8,
    pub obj_x_vel_lo: u8,
    pub obj_x_vel_hi: u8,
    pub obj_y_vel: u8,
    pub obj_damage: u8,
    pub obj_x_sub: u8,
    pub obj_x_tile: u8,
    pub obj_y_pixel: u8,
    pub obj_y_extra: u8,
    pub direction_latch: u8,
    pub room_restore_scratch: u8,
    _pad16: [u8; 650],
    pub password_checksum_add: u8,
    pub password_checksum_xor: u8,
    _pad17: [u8; 64629],
}

const _: () = assert!(core::mem::size_of::<GameState>() == 0x10000);
const _: () = assert!(core::mem::align_of::<GameState>() == 1);
// Compile-time proof that every named field lands at its exact NES RAM address.
const _: () = assert!(core::mem::offset_of!(GameState, audio_duty_work) == 0x0);
const _: () = assert!(core::mem::offset_of!(GameState, sound_channel_offset) == 0x2);
const _: () = assert!(core::mem::offset_of!(GameState, sound_command) == 0x4);
const _: () = assert!(core::mem::offset_of!(GameState, sound_length) == 0x5);
const _: () = assert!(core::mem::offset_of!(GameState, saved_audio_handler_lo) == 0x6);
const _: () = assert!(core::mem::offset_of!(GameState, saved_audio_handler_hi) == 0x7);
const _: () = assert!(core::mem::offset_of!(GameState, scratch0) == 0x8);
const _: () = assert!(core::mem::offset_of!(GameState, scratch1) == 0x9);
const _: () = assert!(core::mem::offset_of!(GameState, scratch2) == 0xa);
const _: () = assert!(core::mem::offset_of!(GameState, scratch3) == 0xb);
const _: () = assert!(core::mem::offset_of!(GameState, data_ptr_lo) == 0xc);
const _: () = assert!(core::mem::offset_of!(GameState, data_ptr_hi) == 0xd);
const _: () = assert!(core::mem::offset_of!(GameState, indirect_ptr_lo) == 0xe);
const _: () = assert!(core::mem::offset_of!(GameState, indirect_ptr_hi) == 0xf);
const _: () = assert!(core::mem::offset_of!(GameState, tile_fetch_counter) == 0x10);
const _: () = assert!(core::mem::offset_of!(GameState, aux_ptr_hi) == 0x11);
const _: () = assert!(core::mem::offset_of!(GameState, vram_addr_lo) == 0x16);
const _: () = assert!(core::mem::offset_of!(GameState, vram_addr_hi) == 0x17);
const _: () = assert!(core::mem::offset_of!(GameState, vram_addr2_lo) == 0x18);
const _: () = assert!(core::mem::offset_of!(GameState, vram_addr2_hi) == 0x19);
const _: () = assert!(core::mem::offset_of!(GameState, inventory_upload_col) == 0x1a);
const _: () = assert!(core::mem::offset_of!(GameState, inventory_upload_row) == 0x1b);
const _: () = assert!(core::mem::offset_of!(GameState, scroll_pixel_x) == 0x1c);
const _: () = assert!(core::mem::offset_of!(GameState, nametable_select) == 0x1d);
const _: () = assert!(core::mem::offset_of!(GameState, scroll_y) == 0x1e);
const _: () = assert!(core::mem::offset_of!(GameState, buttons) == 0x20);
const _: () = assert!(core::mem::offset_of!(GameState, button_chord) == 0x21);
const _: () = assert!(core::mem::offset_of!(GameState, collision_flag) == 0x22);
const _: () = assert!(core::mem::offset_of!(GameState, ppu_ctrl_shadow) == 0x23);
const _: () = assert!(core::mem::offset_of!(GameState, ppu_mask_shadow) == 0x24);
const _: () = assert!(core::mem::offset_of!(GameState, mmc3_bank_select) == 0x25);
const _: () = assert!(core::mem::offset_of!(GameState, frame_status) == 0x26);
const _: () = assert!(core::mem::offset_of!(GameState, sound_status_flags) == 0x27);
const _: () = assert!(core::mem::offset_of!(GameState, nmi_vram_req) == 0x28);
const _: () = assert!(core::mem::offset_of!(GameState, statusbar_split_flag) == 0x29);
const _: () = assert!(core::mem::offset_of!(GameState, prg_bank_8000) == 0x30);
const _: () = assert!(core::mem::offset_of!(GameState, prg_bank_a000) == 0x31);
const _: () = assert!(core::mem::offset_of!(GameState, saved_prg_bank_8000) == 0x32);
const _: () = assert!(core::mem::offset_of!(GameState, saved_prg_bank_a000) == 0x33);
const _: () = assert!(core::mem::offset_of!(GameState, song_ptr_lo) == 0x34);
const _: () = assert!(core::mem::offset_of!(GameState, song_ptr_hi) == 0x35);
const _: () = assert!(core::mem::offset_of!(GameState, frame_counter) == 0x36);
const _: () = assert!(core::mem::offset_of!(GameState, continue_timer) == 0x37);
const _: () = assert!(core::mem::offset_of!(GameState, rng_limit) == 0x38);
const _: () = assert!(core::mem::offset_of!(GameState, rng_seed_scratch) == 0x39);
const _: () = assert!(core::mem::offset_of!(GameState, rng_low) == 0x3a);
const _: () = assert!(core::mem::offset_of!(GameState, rng_high) == 0x3b);
const _: () = assert!(core::mem::offset_of!(GameState, hud_refresh_flag) == 0x3c);
const _: () = assert!(core::mem::offset_of!(GameState, main_loop_phase) == 0x3d);
const _: () = assert!(core::mem::offset_of!(GameState, sprite_index) == 0x3e);
const _: () = assert!(core::mem::offset_of!(GameState, oam_cursor) == 0x3f);
const _: () = assert!(core::mem::offset_of!(GameState, character_index) == 0x40);
const _: () = assert!(core::mem::offset_of!(GameState, family_member_mask) == 0x41);
const _: () = assert!(core::mem::offset_of!(GameState, title_timer) == 0x42);
const _: () = assert!(core::mem::offset_of!(GameState, player_x_fine) == 0x43);
const _: () = assert!(core::mem::offset_of!(GameState, player_x_tile) == 0x44);
const _: () = assert!(core::mem::offset_of!(GameState, player_y) == 0x45);
const _: () = assert!(core::mem::offset_of!(GameState, landing_timer) == 0x46);
const _: () = assert!(core::mem::offset_of!(GameState, map_screen_x) == 0x47);
const _: () = assert!(core::mem::offset_of!(GameState, map_screen_y) == 0x48);
const _: () = assert!(core::mem::offset_of!(GameState, horizontal_subtile_delta) == 0x49);
const _: () = assert!(core::mem::offset_of!(GameState, player_x_velocity) == 0x4a);
const _: () = assert!(core::mem::offset_of!(GameState, vertical_delta) == 0x4b);
const _: () = assert!(core::mem::offset_of!(GameState, nudge_pending) == 0x4c);
const _: () = assert!(core::mem::offset_of!(GameState, anim_step_counter) == 0x4d);
const _: () = assert!(core::mem::offset_of!(GameState, fall_frames) == 0x4e);
const _: () = assert!(core::mem::offset_of!(GameState, jump_timer) == 0x4f);
const _: () = assert!(core::mem::offset_of!(GameState, pose_state) == 0x50);
const _: () = assert!(core::mem::offset_of!(GameState, selected_item_slot) == 0x55);
const _: () = assert!(core::mem::offset_of!(GameState, player_pose) == 0x56);
const _: () = assert!(core::mem::offset_of!(GameState, player_facing) == 0x57);
const _: () = assert!(core::mem::offset_of!(GameState, player_health) == 0x58);
const _: () = assert!(core::mem::offset_of!(GameState, player_magic) == 0x59);
const _: () = assert!(core::mem::offset_of!(GameState, coins) == 0x5a);
const _: () = assert!(core::mem::offset_of!(GameState, keys) == 0x5b);
const _: () = assert!(core::mem::offset_of!(GameState, jump_strength) == 0x5c);
const _: () = assert!(core::mem::offset_of!(GameState, projectile_damage) == 0x5d);
const _: () = assert!(core::mem::offset_of!(GameState, projectile_count) == 0x5e);
const _: () = assert!(core::mem::offset_of!(GameState, projectile_lifetime) == 0x5f);
const _: () = assert!(core::mem::offset_of!(GameState, shop_active) == 0x61);
const _: () = assert!(core::mem::offset_of!(GameState, fragment_count) == 0x6e);
const _: () = assert!(core::mem::offset_of!(GameState, text_attr_ptr_lo) == 0x70);
const _: () = assert!(core::mem::offset_of!(GameState, text_attr_ptr_hi) == 0x71);
const _: () = assert!(core::mem::offset_of!(GameState, room_tile_action) == 0x74);
const _: () = assert!(core::mem::offset_of!(GameState, room_metadef_lo) == 0x75);
const _: () = assert!(core::mem::offset_of!(GameState, room_metadef_hi) == 0x76);
const _: () = assert!(core::mem::offset_of!(GameState, palette_src_ptr_lo) == 0x77);
const _: () = assert!(core::mem::offset_of!(GameState, palette_src_ptr_hi) == 0x78);
const _: () = assert!(core::mem::offset_of!(GameState, tile_table_ptr_lo) == 0x79);
const _: () = assert!(core::mem::offset_of!(GameState, tile_table_ptr_hi) == 0x7a);
const _: () = assert!(core::mem::offset_of!(GameState, scroll_fine_x) == 0x7b);
const _: () = assert!(core::mem::offset_of!(GameState, scroll_tile_x) == 0x7c);
const _: () = assert!(core::mem::offset_of!(GameState, saved_scroll_tile) == 0x7e);
const _: () = assert!(core::mem::offset_of!(GameState, camera_scroll_flag) == 0x7f);
const _: () = assert!(core::mem::offset_of!(GameState, frame_prescaler) == 0x84);
const _: () = assert!(core::mem::offset_of!(GameState, sprite_blink_timer) == 0x85);
const _: () = assert!(core::mem::offset_of!(GameState, airborne_flag) == 0x86);
const _: () = assert!(core::mem::offset_of!(GameState, magic_contact_flag) == 0x87);
const _: () = assert!(core::mem::offset_of!(GameState, displaced_timer) == 0x88);
const _: () = assert!(core::mem::offset_of!(GameState, boost_timer) == 0x89);
const _: () = assert!(core::mem::offset_of!(GameState, short_boost_timer) == 0x8a);
const _: () = assert!(core::mem::offset_of!(GameState, long_boost_timer) == 0x8b);
const _: () = assert!(core::mem::offset_of!(GameState, countdown_timer) == 0x8c);
const _: () = assert!(core::mem::offset_of!(GameState, sound_paused) == 0x8d);
const _: () = assert!(core::mem::offset_of!(GameState, song) == 0x8e);
const _: () = assert!(core::mem::offset_of!(GameState, prompt_state) == 0x8f);
const _: () = assert!(core::mem::offset_of!(GameState, prompt_argument) == 0x90);
const _: () = assert!(core::mem::offset_of!(GameState, sfx_priority) == 0x91);
const _: () = assert!(core::mem::offset_of!(GameState, music_volume_override) == 0x92);
const _: () = assert!(core::mem::offset_of!(GameState, sound_channel_flags) == 0xa4);
const _: () = assert!(core::mem::offset_of!(GameState, triangle_timer) == 0xb3);
const _: () = assert!(core::mem::offset_of!(GameState, sfx_voice_active) == 0xd4);
const _: () = assert!(core::mem::offset_of!(GameState, slot_index) == 0xe3);
const _: () = assert!(core::mem::offset_of!(GameState, slot_index_limit) == 0xe4);
const _: () = assert!(core::mem::offset_of!(GameState, obj_slot_ptr_lo) == 0xe5);
const _: () = assert!(core::mem::offset_of!(GameState, obj_slot_ptr_hi) == 0xe6);
const _: () = assert!(core::mem::offset_of!(GameState, actor_record_ptr_lo) == 0xe7);
const _: () = assert!(core::mem::offset_of!(GameState, actor_record_ptr_hi) == 0xe8);
const _: () = assert!(core::mem::offset_of!(GameState, scheduler_phase) == 0xe9);
const _: () = assert!(core::mem::offset_of!(GameState, overlap_flag) == 0xea);
const _: () = assert!(core::mem::offset_of!(GameState, pending_special_exit) == 0xeb);
const _: () = assert!(core::mem::offset_of!(GameState, final_exit_flag) == 0xec);
const _: () = assert!(core::mem::offset_of!(GameState, obj_tile) == 0xed);
const _: () = assert!(core::mem::offset_of!(GameState, obj_state) == 0xee);
const _: () = assert!(core::mem::offset_of!(GameState, obj_attr) == 0xef);
const _: () = assert!(core::mem::offset_of!(GameState, obj_move_scratch) == 0xf0);
const _: () = assert!(core::mem::offset_of!(GameState, obj_cooldown) == 0xf1);
const _: () = assert!(core::mem::offset_of!(GameState, obj_health) == 0xf2);
const _: () = assert!(core::mem::offset_of!(GameState, obj_timer) == 0xf3);
const _: () = assert!(core::mem::offset_of!(GameState, obj_move_state) == 0xf4);
const _: () = assert!(core::mem::offset_of!(GameState, obj_x_vel_lo) == 0xf5);
const _: () = assert!(core::mem::offset_of!(GameState, obj_x_vel_hi) == 0xf6);
const _: () = assert!(core::mem::offset_of!(GameState, obj_y_vel) == 0xf7);
const _: () = assert!(core::mem::offset_of!(GameState, obj_damage) == 0xf8);
const _: () = assert!(core::mem::offset_of!(GameState, obj_x_sub) == 0xf9);
const _: () = assert!(core::mem::offset_of!(GameState, obj_x_tile) == 0xfa);
const _: () = assert!(core::mem::offset_of!(GameState, obj_y_pixel) == 0xfb);
const _: () = assert!(core::mem::offset_of!(GameState, obj_y_extra) == 0xfc);
const _: () = assert!(core::mem::offset_of!(GameState, direction_latch) == 0xfd);
const _: () = assert!(core::mem::offset_of!(GameState, room_restore_scratch) == 0xfe);
const _: () = assert!(core::mem::offset_of!(GameState, password_checksum_add) == 0x389);
const _: () = assert!(core::mem::offset_of!(GameState, password_checksum_xor) == 0x38a);

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        // All fields are u8 / byte arrays, so an all-zero image is valid.
        unsafe { std::mem::zeroed() }
    }

    /// Clear all RAM back to zero. Mapped PRG/CHR is re-established by the
    /// loader, so a full zero-fill here is correct for a fresh boot.
    pub fn reset(&mut self) {
        self.ram_bytes_mut().fill(0);
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
        unsafe { std::ptr::read_volatile((self as *const GameState as *const u8).add(idx)) as i32 }
    }

    /// Write the low byte of `value` at `addr` (volatile; see [`Self::byte`]).
    #[inline]
    pub fn set_byte(&mut self, addr: i32, value: i32) {
        let idx = (addr as usize) & 0xffff;
        unsafe {
            std::ptr::write_volatile((self as *mut GameState as *mut u8).add(idx), value as u8)
        };
    }

    /// The whole CPU address space as a byte slice (the struct *is* its bytes).
    /// Used for bulk operations: OAM DMA, PRG bank mapping, ROM load, reset.
    #[inline]
    pub fn ram_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const GameState as *const u8, 0x10000) }
    }
    #[inline]
    pub fn ram_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut GameState as *mut u8, 0x10000) }
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

    array_field!(
        chr_bank,
        set_chr_bank,
        0x2A,
        "MMC3 CHR bank shadow for register `reg` (R0-R5), shadowed at `$2A..$2F`. R0/R1 select 2 KiB CHR windows; R2-R5 select 1 KiB windows. A per-frame committer replays these to `$8001`."
    );

    // ---- Controller input -------------------------------------------------

    // ---- Frame sync / timers ----------------------------------------------

    /// True when the captured PPUSTATUS shadow reports a sprite-0 hit (`$26`
    /// bit6) — i.e. rendering reached the status-bar split this frame.
    #[inline]
    pub fn sprite0_hit(&self) -> bool {
        (self.frame_status & 0x40) != 0
    }

    #[inline]
    pub fn frame_counter_active(&self) -> bool {
        self.frame_counter != 0
    }

    array_field!(
        coarse_timer,
        set_coarse_timer,
        0x85,
        "Coarse timer slot `i` (0-7) in the `$85..$8C` array, each decremented once per 60 frames by `frame_counters`. Slot 0 is the sprite-blink timer ([`Self::sprite_blink_timer`]); slot 7 is the countdown timer ([`Self::countdown_timer`])."
    );

    #[inline]
    pub fn countdown_timer_active(&self) -> bool {
        self.countdown_timer != 0
    }

    // ---- Player vitals ----------------------------------------------------

    // ---- Audio ------------------------------------------------------------

    // ---- Text/prompt UI ---------------------------------------------------

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

    // ---- Player position / motion -----------------------------------------

    // ---- VRAM upload address ($16/$17) ------------------------------------
    //
    // Target address for the next PPU VRAM transfer. The high and low bytes
    // are written separately to PPUADDR ($2006), so the byte accessors are the
    // primary form; `vram_addr` folds the pair when a full address is handy.

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

    // ---- Audio / scheduler ------------------------------------------------

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

    // ---- General-purpose scratch bytes ($08..$0B) -------------------------
    //
    // Reusable zero-page scratch the 6502 code uses as per-routine temporaries
    // (mask/shift math, holding a value across a few instructions). They carry
    // no persistent meaning; a given write/read is local to its routine, so
    // the names are intentionally generic.

    // ---- Scroll / nametable / status-bar split ----------------------------

    /// Status-bar sprite-0 split enable flag (`$29`); nonzero makes the
    /// renderer split the screen for the HUD band.
    pub const STATUSBAR_SPLIT_FLAG: i32 = 0x29;

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

    array_field!(
        temp_save,
        set_temp_save,
        0x80,
        "Temporary save slot `i` (`$80 + i`, 4 bytes): a scratch group preserved across nested calls (e.g. the shop/menu state handlers)."
    );

    // ---- NMI VRAM request -------------------------------------------------

    // ---- More player / render / sound scalars -----------------------------

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

    // ---- RNG state ($38..$3B) ---------------------------------------------
    //
    // `rng_update` advances a 16-bit LFSR-style seed ($3A low, $3B high) mixed
    // with a saved previous low byte ($39), and rejection-samples below a
    // requested limit ($38). $3B is also the returned random byte.

    // ---- Misc player / UI / frame state -----------------------------------

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
}
