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

/// Generate a getter/setter pair for an array-like region of RAM.
///
/// Many NES variables are not single bytes but fixed-stride tables (OAM
/// entries, object records, password nibble cells, …). Rather than name every
/// element, this macro defines a `$get(i)` / `$set(i, value)` accessor pair that
/// indexes the region relative to a fixed `$base` address, so the translated
/// 6502 code can keep doing `base + index` reads/writes against named regions.
///
/// Parameters:
/// - `$get` / `$set`: identifiers for the generated read/write methods.
/// - `$base`: the region's first NES RAM address (the element-0 address).
/// - `$doc`: the rustdoc string attached to the getter (the `///` text).
///
/// Both methods route through the volatile [`GameState::byte`] /
/// [`GameState::set_byte`] primitives, so they inherit the same volatile-access
/// semantics described there. `i` is added to `$base` with plain `i32`
/// arithmetic, matching how the original code computes table offsets.
macro_rules! array_field {
    ($get:ident, $set:ident, $base:expr, $doc:expr) => {
        // Getter: read the byte `i` elements past the region base.
        #[doc = $doc]
        #[inline]
        pub fn $get(&self, i: i32) -> i32 {
            self.byte($base + i)
        }
        // Setter: write the low byte of `value` `i` elements past the base.
        #[inline]
        pub fn $set(&mut self, i: i32, value: i32) {
            self.set_byte($base + i, value);
        }
    };
}

#[repr(C)]
pub struct GameState {
    /// Audio duty/volume work byte for the duty/instrument command (`$00`).
    pub audio_duty_work: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad0: [u8; 1],
    /// Sound channel byte offset currently being processed (`$02`).
    pub sound_channel_offset: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad1: [u8; 1],
    /// Current sound command id (`$04`).
    pub sound_command: u8,
    /// Sound length/period parameter for the current note (`$05`).
    pub sound_length: u8,
    /// Saved audio stream command handler pointer, low/high (`$06`/`$07`).
    pub saved_audio_handler_lo: u8,
    /// High byte of the saved audio handler pointer (`$7`).
    pub saved_audio_handler_hi: u8,
    /// General-purpose scratch byte 0 (`$08`).
    pub scratch0: u8,
    /// General-purpose scratch byte 1 (`$09`).
    pub scratch1: u8,
    /// General-purpose scratch byte 2 (`$0A`).
    pub scratch2: u8,
    /// General-purpose scratch byte 3 (`$0B`).
    pub scratch3: u8,
    /// Low byte of the data ptr pointer (`$C`).
    pub data_ptr_lo: u8,
    /// High byte of the data ptr pointer (`$D`).
    pub data_ptr_hi: u8,
    /// Low byte of the indirect ptr pointer (`$E`).
    pub indirect_ptr_lo: u8,
    /// High byte of the indirect ptr pointer (`$F`).
    pub indirect_ptr_hi: u8,
    /// Countdown used while resolving the room tile pointer (`$10`).
    pub tile_fetch_counter: u8,
    /// Auxiliary stream pointer high byte (`$11`); secondary to `data_ptr`.
    pub aux_ptr_hi: u8,
    /// Unnamed RAM gap (4 byte(s)) between named fields.
    _pad2: [u8; 4],
    /// VRAM upload address, low byte (`$16`).
    pub vram_addr_lo: u8,
    /// VRAM upload address, high byte (`$17`).
    pub vram_addr_hi: u8,
    /// Secondary VRAM transfer address, low byte (`$18`).
    pub vram_addr2_lo: u8,
    /// Secondary VRAM transfer address, high byte (`$19`).
    pub vram_addr2_hi: u8,
    /// Column cursor while uploading the inventory item list (`$1A`).
    pub inventory_upload_col: u8,
    /// Row cursor while uploading the inventory item list (`$1B`).
    pub inventory_upload_row: u8,
    /// Horizontal scroll position in pixels (`$1C`); added to object positions to convert room coordinates to on-screen coordinates.
    pub scroll_pixel_x: u8,
    /// Active nametable selection bit (`$1D`), toggled as the camera crosses nametable boundaries.
    pub nametable_select: u8,
    /// Vertical scroll value written to PPUSCROLL `$2005` (`$1E`).
    pub scroll_y: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad3: [u8; 1],
    /// Buttons held this frame (`$20`). Bit layout, LSB first: `0`=Right `1`=Left `2`=Down `3`=Up `4`=Start `5`=Select `6`=B `7`=A.
    pub buttons: u8,
    /// Buttons newly pressed this frame (`$21`): the rising edge of [`Self::buttons`], used for one-shot actions like menu confirms.
    pub button_chord: u8,
    /// Collision/blocked flag set during movement resolution (`$22`).
    pub collision_flag: u8,
    /// PPUCTRL ($2000) shadow (`$23`): nametable/increment/sprite-size and pattern-table selection bits replayed to the PPU each frame.
    pub ppu_ctrl_shadow: u8,
    /// PPUMASK ($2001) shadow (`$24`): rendering-enable and emphasis bits.
    pub ppu_mask_shadow: u8,
    /// MMC3 bank-select shadow (`$25`): which register R0-R7 the next bank write targets, plus the PRG/CHR mode bits.
    pub mmc3_bank_select: u8,
    /// PPUSTATUS shadow captured by the NMI (`$26`): bit7 = vblank, bit6 = sprite-0 hit (the status-bar split marker).
    pub frame_status: u8,
    /// Sound engine status flag bits (`$27`).
    pub sound_status_flags: u8,
    /// Pending NMI VRAM upload request id (`$28`); foreground code sets it and spins until the NMI handler drains the queued transfer back to zero.
    pub nmi_vram_req: u8,
    /// Statusbar split flag (`$29`).
    pub statusbar_split_flag: u8,
    /// Unnamed RAM gap (6 byte(s)) between named fields.
    _pad4: [u8; 6],
    /// PRG bank mapped at `$8000` — MMC3 R6 shadow (`$30`).
    pub prg_bank_8000: u8,
    /// PRG bank mapped at `$A000` — MMC3 R7 shadow (`$31`).
    pub prg_bank_a000: u8,
    /// Saved R6/`$8000` PRG bank stashed across a far call (`$32`).
    pub saved_prg_bank_8000: u8,
    /// Saved R7/`$A000` PRG bank stashed across a far call (`$33`).
    pub saved_prg_bank_a000: u8,
    /// Current song pointer low/high used by `song_init` (`$34`/`$35`).
    pub song_ptr_lo: u8,
    /// High byte of the song ptr pointer (`$35`).
    pub song_ptr_hi: u8,
    /// Foreground frame-wait countdown (`$36`): foreground code sets it to N and spins on [`Self::frame_counter_active`]; the NMI tail decrements it once per frame, so the spin releases after N frames.
    pub frame_counter: u8,
    /// Continue/respawn countdown timer (`$37`).
    pub continue_timer: u8,
    /// Requested RNG range/limit for the current draw (`$38`).
    pub rng_limit: u8,
    /// Saved previous seed low byte mixed into the next draw (`$39`).
    pub rng_seed_scratch: u8,
    /// RNG seed, low byte (`$3A`).
    pub rng_low: u8,
    /// RNG seed, high byte; also the value returned by a draw (`$3B`).
    pub rng_high: u8,
    /// HUD refresh-needed flag set by `sync_health_hud` (`$3C`).
    pub hud_refresh_flag: u8,
    /// Main-loop dispatch phase counter (`$3D`).
    pub main_loop_phase: u8,
    /// Sprite slot index/counter while building the OAM buffer (`$3E`).
    pub sprite_index: u8,
    /// OAM buffer write cursor / current sprite byte offset (`$3F`).
    pub oam_cursor: u8,
    /// Current character index (which Drasle family member is active) (`$40`).
    pub character_index: u8,
    /// Bitmask of available/active Drasle family members (`$41`).
    pub family_member_mask: u8,
    /// Title-screen loop timer (`$42`).
    pub title_timer: u8,
    /// Player X fine (sub-tile) position (`$43`).
    pub player_x_fine: u8,
    /// Player X tile position (`$44`).
    pub player_x_tile: u8,
    /// Player Y position (`$45`).
    pub player_y: u8,
    /// Post-landing recovery/stun countdown (`$46`); seeded from the fall distance and decremented each frame while nonzero.
    pub landing_timer: u8,
    /// Map screen X (which room column the player occupies) (`$47`).
    pub map_screen_x: u8,
    /// Map screen Y (which room row the player occupies) (`$48`).
    pub map_screen_y: u8,
    /// Horizontal sub-tile movement delta for this frame (`$49`).
    pub horizontal_subtile_delta: u8,
    /// Player horizontal velocity, packed sub-tile delta + sign (`$4A`).
    pub player_x_velocity: u8,
    /// Vertical movement delta for this frame (`$4B`).
    pub vertical_delta: u8,
    /// \"Nudge to tile boundary\" pending flag (`$4C`).
    pub nudge_pending: u8,
    /// Player walk-animation step counter (`$4D`); low 3 bits set the frame cadence.
    pub anim_step_counter: u8,
    /// Frames the player has been falling (`$4E`).
    pub fall_frames: u8,
    /// Remaining jump/ascent timer (`$4F`).
    pub jump_timer: u8,
    /// Pose state flag from `update_player_pose_from_motion` (`$50`).
    pub pose_state: u8,
    /// Unnamed RAM gap (4 byte(s)) between named fields.
    _pad5: [u8; 4],
    /// Selected inventory/menu item slot (cursor index) (`$55`).
    pub selected_item_slot: u8,
    /// Player animation pose/frame selector (`$56`).
    pub player_pose: u8,
    /// Player facing/direction flag (`$57`); bit6 marks the horizontal flip.
    pub player_facing: u8,
    /// Player health/life points (`$58`).
    pub player_health: u8,
    /// Player magic points (`$59`).
    pub player_magic: u8,
    /// Gold/coin count (`$5A`).
    pub coins: u8,
    /// Key count (`$5B`).
    pub keys: u8,
    /// Current character's jump strength / fall-duration parameter (`$5C`): seeds the jump timer and caps accumulated fall frames.
    pub jump_strength: u8,
    /// Effective projectile damage / count / lifetime parameters (`$5D`/`$5E`/`$5F`).
    pub projectile_damage: u8,
    /// Projectile count (`$5E`).
    pub projectile_count: u8,
    /// Projectile lifetime (`$5F`).
    pub projectile_lifetime: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad6: [u8; 1],
    /// Shop room active flag (`$61`).
    pub shop_active: u8,
    /// Unnamed RAM gap (12 byte(s)) between named fields.
    _pad7: [u8; 12],
    /// Remaining fragment-pickup count (`$6E`).
    pub fragment_count: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad8: [u8; 1],
    /// Text attribute source pointer low/high (`$70`/`$71`).
    pub text_attr_ptr_lo: u8,
    /// High byte of the text attr ptr pointer (`$71`).
    pub text_attr_ptr_hi: u8,
    /// Unnamed RAM gap (2 byte(s)) between named fields.
    _pad9: [u8; 2],
    /// Decoded room tile action value (`$74`).
    pub room_tile_action: u8,
    /// Room metatile-definition pointer low/high (`$75`/`$76`).
    pub room_metadef_lo: u8,
    /// High byte of the room metadef pointer (`$76`).
    pub room_metadef_hi: u8,
    /// Low byte of the palette src ptr pointer (`$77`).
    pub palette_src_ptr_lo: u8,
    /// High byte of the palette src ptr pointer (`$78`).
    pub palette_src_ptr_hi: u8,
    /// Low byte of the tile table ptr pointer (`$79`).
    pub tile_table_ptr_lo: u8,
    /// High byte of the tile table ptr pointer (`$7A`).
    pub tile_table_ptr_hi: u8,
    /// Room horizontal scroll, fine (sub-tile) component (`$7B`).
    pub scroll_fine_x: u8,
    /// Room horizontal scroll, tile component (`$7C`).
    pub scroll_tile_x: u8,
    /// Unnamed RAM gap (1 byte(s)) between named fields.
    _pad10: [u8; 1],
    /// Saved horizontal scroll tile during the main loop (`$7E`).
    pub saved_scroll_tile: u8,
    /// Camera scroll-pending flag (`$7F`).
    pub camera_scroll_flag: u8,
    /// Unnamed RAM gap (4 byte(s)) between named fields.
    _pad11: [u8; 4],
    /// 60-frame prescaler (`$84`): reloads to 0x3C and counts down each frame; its low bits drive blink/animation cadence and the coarse timer ticks.
    pub frame_prescaler: u8,
    /// Sprite blink/invulnerability timer (`$85`), one of the coarse timer slots ticked once per 60 frames by the frame counters.
    pub sprite_blink_timer: u8,
    /// Airborne/falling flag for the top-boundary exit check (`$86`).
    pub airborne_flag: u8,
    /// Magic-contact-with-actor flag (`$87`).
    pub magic_contact_flag: u8,
    /// Displaced-block / temporary-tile restore timer (`$88`).
    pub displaced_timer: u8,
    /// Speed-boost / temporary-effect timer (`$89`).
    pub boost_timer: u8,
    /// Short / long speed-boost timers (`$8A`/`$8B`).
    pub short_boost_timer: u8,
    /// Long boost timer (`$8B`).
    pub long_boost_timer: u8,
    /// Coarse countdown timer (`$8C`), e.g. the title-screen attract timeout.
    pub countdown_timer: u8,
    /// Sound paused/disabled flag checked at the top of `sound_tick` (`$8D`).
    pub sound_paused: u8,
    /// Current/requested song id for the sound engine (`$8E`).
    pub song: u8,
    /// Prompt/message state machine selector (`$8F`).
    pub prompt_state: u8,
    /// Argument byte for the active prompt/message (`$90`).
    pub prompt_argument: u8,
    /// Active SFX priority threshold (`$91`).
    pub sfx_priority: u8,
    /// Music volume override flag (`$92`).
    pub music_volume_override: u8,
    /// Unnamed RAM gap (17 byte(s)) between named fields.
    _pad12: [u8; 17],
    /// Sound channel active/control flags (`$A4`).
    pub sound_channel_flags: u8,
    /// Unnamed RAM gap (14 byte(s)) between named fields.
    _pad13: [u8; 14],
    /// Triangle channel note-duration countdown (`$B3`).
    pub triangle_timer: u8,
    /// Unnamed RAM gap (32 byte(s)) between named fields.
    _pad14: [u8; 32],
    /// SFX overlay voice active flag, bit7 (`$D4`).
    pub sfx_voice_active: u8,
    /// Unnamed RAM gap (14 byte(s)) between named fields.
    _pad15: [u8; 14],
    /// Current object/actor slot index for iteration loops (`$E3`); shifted left 4 to form the slot's byte offset into the object table.
    pub slot_index: u8,
    /// Upper bound for the [`Self::slot_index`] iteration loop (`$E4`).
    pub slot_index_limit: u8,
    /// Low byte of the obj slot ptr pointer (`$E5`).
    pub obj_slot_ptr_lo: u8,
    /// High byte of the obj slot ptr pointer (`$E6`).
    pub obj_slot_ptr_hi: u8,
    /// Low byte of the actor record ptr pointer (`$E7`).
    pub actor_record_ptr_lo: u8,
    /// High byte of the actor record ptr pointer (`$E8`).
    pub actor_record_ptr_hi: u8,
    /// Actor scheduler phase counter (`$E9`).
    pub scheduler_phase: u8,
    /// Player/actor overlap-detected flag (`$EA`).
    pub overlap_flag: u8,
    /// Pending special-exit-room flag checked by `game_update` (`$EB`).
    pub pending_special_exit: u8,
    /// Final-exit trigger reached flag (`$EC`).
    pub final_exit_flag: u8,
    /// Sprite/tile id and animation bits — slot `+0x00` (`$ED`).
    pub obj_tile: u8,
    /// Active/state/lifetime byte — slot `+0x01` (`$EE`).
    pub obj_state: u8,
    /// Attribute/direction bits — slot `+0x02` (`$EF`).
    pub obj_attr: u8,
    /// Tile-replacement / movement scratch — slot `+0x03` (`$F0`).
    pub obj_move_scratch: u8,
    /// Cooldown / path scratch — slot `+0x04` (`$F1`).
    pub obj_cooldown: u8,
    /// Health / damage threshold — slot `+0x05` (`$F2`).
    pub obj_health: u8,
    /// Timer / animation phase — slot `+0x06` (`$F3`).
    pub obj_timer: u8,
    /// Movement/direction state bits — slot `+0x07` (`$F4`); high bit and the low two bits encode turn/animation direction state.
    pub obj_move_state: u8,
    /// X velocity, low nibble — slot `+0x08` (`$F5`).
    pub obj_x_vel_lo: u8,
    /// X velocity carry/sign — slot `+0x09` (`$F6`).
    pub obj_x_vel_hi: u8,
    /// Y velocity — slot `+0x0A` (`$F7`).
    pub obj_y_vel: u8,
    /// Damage / effect strength — slot `+0x0B` (`$F8`).
    pub obj_damage: u8,
    /// X sub-tile fraction — slot `+0x0C` (`$F9`).
    pub obj_x_sub: u8,
    /// X tile coordinate — slot `+0x0D` (`$FA`).
    pub obj_x_tile: u8,
    /// Y pixel coordinate — slot `+0x0E` (`$FB`).
    pub obj_y_pixel: u8,
    /// Extra y / sprite scratch — slot `+0x0F` (`$FC`).
    pub obj_y_extra: u8,
    /// Direction latch (`$FD`): low nibble holds the current movement direction, high nibble the previously latched one.
    pub direction_latch: u8,
    /// Scratch byte used by `restore_room_from_checkpoint` (`$FE`).
    pub room_restore_scratch: u8,
    /// Unnamed RAM gap (650 byte(s)) between named fields.
    _pad16: [u8; 650],
    /// Password additive checksum (`$0389`).
    pub password_checksum_add: u8,
    /// Password XOR checksum (`$038A`).
    pub password_checksum_xor: u8,
    /// Unnamed RAM gap (64629 byte(s)) between named fields.
    _pad17: [u8; 64629],
}

/// Size of the CPU-visible address space: a full 64 KiB (`$0000..=$FFFF`).
pub const ADDRESS_SPACE_SIZE: usize = 0x10000;
/// Mask that wraps any address into the 64 KiB space (`addr & 0xFFFF`),
/// reproducing 6502 16-bit address wraparound.
const ADDRESS_MASK: usize = 0xffff;
/// Mask that truncates a value to a single byte (`value & 0xFF`), reproducing
/// the 8-bit width of 6502 registers and memory cells.
const BYTE_MASK: i32 = 0xff;

// The struct must be exactly the 64 KiB address space, byte-packed (align 1),
// so that `offset_of!` field offsets equal real NES RAM addresses and the
// whole struct can be reinterpreted as a flat `[u8; 0x10000]` (see
// `ram_bytes`). These asserts fail the build if either invariant is violated.
const _: () = assert!(core::mem::size_of::<GameState>() == ADDRESS_SPACE_SIZE);
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
    /// The default image is the same all-zero memory as [`GameState::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Construct a fresh, all-zero memory image (equivalent to a powered-off
    /// machine before the loader maps any banks).
    pub fn new() -> Self {
        // All fields are u8 / byte arrays, so an all-zero image is valid:
        // `zeroed()` is sound because every byte 0 is a valid `u8` and the
        // struct has no padding bytes with niche requirements.
        unsafe { std::mem::zeroed() }
    }

    /// Clear all RAM back to zero. Mapped PRG/CHR is re-established by the
    /// loader, so a full zero-fill here is correct for a fresh boot.
    pub fn reset(&mut self) {
        // Zero the entire 64 KiB image; the bank loader repopulates the mapped
        // PRG/CHR windows afterward.
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
        // Wrap the address into the 64 KiB space (6502 addresses are 16-bit and
        // wrap), giving an in-bounds byte index into the struct.
        let idx = (addr as usize) & ADDRESS_MASK;
        // Volatile load through a raw `*const u8` reinterpretation of the
        // struct; the zero-extended byte is returned as `i32` so call sites can
        // do 6502-style 8-bit math without repeated casts.
        unsafe { std::ptr::read_volatile((self as *const GameState as *const u8).add(idx)) as i32 }
    }

    /// Write the low byte of `value` at `addr` (volatile; see [`Self::byte`]).
    #[inline]
    pub fn set_byte(&mut self, addr: i32, value: i32) {
        // Same 16-bit address wrap as `byte`.
        let idx = (addr as usize) & ADDRESS_MASK;
        // Volatile store of the truncated low byte (the `as u8` cast keeps only
        // bits 0-7, matching a 6502 store of an 8-bit register).
        unsafe {
            std::ptr::write_volatile((self as *mut GameState as *mut u8).add(idx), value as u8)
        };
    }

    /// The whole CPU address space as a byte slice (the struct *is* its bytes).
    /// Used for bulk operations: OAM DMA, PRG bank mapping, ROM load, reset.
    #[inline]
    pub fn ram_bytes(&self) -> &[u8] {
        // Safe: the struct is exactly `ADDRESS_SPACE_SIZE` bytes (asserted at
        // compile time) and byte-packed, so it aliases a `[u8; 0x10000]`.
        unsafe {
            std::slice::from_raw_parts(self as *const GameState as *const u8, ADDRESS_SPACE_SIZE)
        }
    }
    /// Mutable view of the whole CPU address space as bytes (mutable counterpart of [`Self::ram_bytes`]).
    #[inline]
    pub fn ram_bytes_mut(&mut self) -> &mut [u8] {
        // Mutable counterpart of `ram_bytes`; same size/packing guarantees.
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut GameState as *mut u8, ADDRESS_SPACE_SIZE)
        }
    }

    /// Add `value` to the byte at `addr` (wrapping, masked to 8 bits); returns
    /// the new value.
    ///
    /// Models a 6502 read-modify-write add: the result is truncated to 8 bits
    /// (`& BYTE_MASK`) and stored back, then returned for the caller's use.
    #[inline]
    pub fn add_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Read, wrapping-add, then keep only the low 8 bits (8-bit storage).
        let next = self.byte(addr).wrapping_add(value) & BYTE_MASK;
        self.set_byte(addr, next);
        next
    }

    /// Subtract `value` from the byte at `addr` (wrapping, masked); returns it.
    #[inline]
    pub fn sub_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Read, wrapping-subtract, then truncate to 8 bits.
        let next = self.byte(addr).wrapping_sub(value) & BYTE_MASK;
        self.set_byte(addr, next);
        next
    }

    /// `byte &= value`; returns the new value.
    #[inline]
    pub fn and_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Bitwise AND stays within 8 bits as long as the byte is, so no extra
        // mask is needed.
        let next = self.byte(addr) & value;
        self.set_byte(addr, next);
        next
    }

    /// `byte |= value`; returns the new value.
    #[inline]
    pub fn or_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Bitwise OR of the stored byte with `value`.
        let next = self.byte(addr) | value;
        self.set_byte(addr, next);
        next
    }

    /// `byte ^= value`; returns the new value.
    #[inline]
    pub fn xor_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Bitwise XOR of the stored byte with `value`.
        let next = self.byte(addr) ^ value;
        self.set_byte(addr, next);
        next
    }

    /// `byte <<= value` (masked to 8 bits); returns the new value.
    #[inline]
    pub fn shl_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Left-shift then drop any bits shifted above bit 7 (8-bit register).
        let next = (self.byte(addr) << value) & BYTE_MASK;
        self.set_byte(addr, next);
        next
    }

    /// `byte >>= value` (logical); returns the new value.
    #[inline]
    pub fn shr_byte(&mut self, addr: i32, value: i32) -> i32 {
        // Mask to 8 bits first so the logical right shift fills with zeros from
        // bit 7 (an unsigned 6502 LSR), never sign-extending the `i32`.
        let next = (self.byte(addr) & BYTE_MASK) >> value;
        self.set_byte(addr, next);
        next
    }

    /// Increment the byte at `addr` (wrapping); returns the new value.
    ///
    /// Equivalent to the 6502 `INC` instruction: `0xFF` wraps to `0x00`.
    #[inline]
    pub fn inc_byte(&mut self, addr: i32) -> i32 {
        self.add_byte(addr, 1)
    }

    /// Decrement the byte at `addr` (wrapping); returns the new value.
    ///
    /// Equivalent to the 6502 `DEC` instruction: `0x00` wraps to `0xFF`.
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
        // PPUSTATUS ($2002) bit 6 is the sprite-0 hit flag; the engine latches
        // PPUSTATUS into `frame_status` ($26) and tests that bit here.
        const PPUSTATUS_SPRITE0_HIT: u8 = 0x40; // bit 6
        (self.frame_status & PPUSTATUS_SPRITE0_HIT) != 0
    }

    /// True while the global frame counter ([`Self::frame_counter`], `$36`) is
    /// running (nonzero) — used to gate logic that should pause when the count
    /// has been zeroed.
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

    /// True while the countdown timer ([`Self::countdown_timer`], `$8C`, the
    /// last coarse-timer slot) has not yet expired (nonzero).
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
        // Fold the little-endian pair: low byte in $E5, high byte in $E6 << 8.
        self.byte(0xE5) | (self.byte(0xE6) << 8)
    }
    /// Store the 16-bit object-slot pointer back into `$E5`/`$E6` (little-endian).
    #[inline]
    pub fn set_obj_slot_ptr(&mut self, value: i32) {
        // Split a 16-bit value back into the low/high zero-page bytes.
        self.set_byte(0xE5, value & BYTE_MASK);
        self.set_byte(0xE6, (value >> 8) & BYTE_MASK);
    }

    /// Pointer to the room actor record feeding the current slot (`$E7`/`$E8`).
    #[inline]
    pub fn actor_record_ptr(&self) -> i32 {
        // Low byte $E7, high byte $E8.
        self.byte(0xE7) | (self.byte(0xE8) << 8)
    }
    /// Store the 16-bit actor-record pointer into `$E7`/`$E8` (little-endian).
    #[inline]
    pub fn set_actor_record_ptr(&mut self, value: i32) {
        self.set_byte(0xE7, value & BYTE_MASK);
        self.set_byte(0xE8, (value >> 8) & BYTE_MASK);
    }

    /// Pointer to the active palette source data (`$77`/`$78`).
    #[inline]
    pub fn palette_src_ptr(&self) -> i32 {
        // Low byte $77, high byte $78.
        self.byte(0x77) | (self.byte(0x78) << 8)
    }
    /// Store the 16-bit palette-source pointer into `$77`/`$78` (little-endian).
    #[inline]
    pub fn set_palette_src_ptr(&mut self, value: i32) {
        self.set_byte(0x77, value & BYTE_MASK);
        self.set_byte(0x78, (value >> 8) & BYTE_MASK);
    }

    /// Pointer to the current room's metatile table (`$79`/`$7A`).
    #[inline]
    pub fn tile_table_ptr(&self) -> i32 {
        // Low byte $79, high byte $7A.
        self.byte(0x79) | (self.byte(0x7A) << 8)
    }
    /// Store the 16-bit tile-table pointer into `$79`/`$7A` (little-endian).
    #[inline]
    pub fn set_tile_table_ptr(&mut self, value: i32) {
        self.set_byte(0x79, value & BYTE_MASK);
        self.set_byte(0x7A, (value >> 8) & BYTE_MASK);
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
        // PPUADDR is a 14-bit address written high-then-low; here it is kept as
        // a folded 16-bit value with low byte $16, high byte $17.
        self.byte(0x16) | (self.byte(0x17) << 8)
    }
    /// Store the 16-bit VRAM address shadow into `$16`/`$17` (little-endian).
    #[inline]
    pub fn set_vram_addr(&mut self, value: i32) {
        self.set_byte(0x16, value & BYTE_MASK);
        self.set_byte(0x17, (value >> 8) & BYTE_MASK);
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
        // Low byte $0C, high byte $0D.
        self.byte(0x0C) | (self.byte(0x0D) << 8)
    }
    /// Store the 16-bit data pointer into `$0C`/`$0D` (little-endian).
    #[inline]
    pub fn set_data_ptr(&mut self, value: i32) {
        self.set_byte(0x0C, value & BYTE_MASK);
        self.set_byte(0x0D, (value >> 8) & BYTE_MASK);
    }

    /// General indirect / far-call target pointer (`$0E`/`$0F`).
    #[inline]
    pub fn indirect_ptr(&self) -> i32 {
        // Low byte $0E, high byte $0F.
        self.byte(0x0E) | (self.byte(0x0F) << 8)
    }
    /// Store the 16-bit indirect pointer into `$0E`/`$0F` (little-endian).
    #[inline]
    pub fn set_indirect_ptr(&mut self, value: i32) {
        self.set_byte(0x0E, value & BYTE_MASK);
        self.set_byte(0x0F, (value >> 8) & BYTE_MASK);
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

    /// Base address of the per-channel sound state records (`$93`); each record
    /// is 16 bytes, indexed by `ch` (a multiple of 16) and `field` (0-15).
    pub const SOUND_CHANNEL_BASE: i32 = 0x93;

    /// Byte `field` (0-15) of the sound channel record at byte offset `ch`
    /// (`$93 + field + ch`).
    ///
    /// `field` selects the byte within a 16-byte channel record; `ch` is the
    /// record's byte offset from the channel-array base (a multiple of 16).
    #[inline]
    pub fn sound_channel_byte(&self, field: i32, ch: i32) -> i32 {
        // SOUND_CHANNEL_BASE ($93) + within-record field + per-channel offset.
        self.byte(Self::SOUND_CHANNEL_BASE + field + ch)
    }
    /// Write `value` to sound-channel field `field` of channel `ch` (setter counterpart of [`Self::sound_channel_byte`]).
    #[inline]
    pub fn set_sound_channel_byte(&mut self, field: i32, ch: i32, value: i32) {
        self.set_byte(Self::SOUND_CHANNEL_BASE + field + ch, value);
    }
    /// Decrement a sound channel field (wrapping) and return the new value.
    #[inline]
    pub fn dec_sound_channel_byte(&mut self, field: i32, ch: i32) -> i32 {
        // Subtract one and truncate to 8 bits so 0x00 wraps to 0xFF.
        let v = (self.sound_channel_byte(field, ch) - 1) & BYTE_MASK;
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
