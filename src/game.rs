// Rust game routine module. The functions here are the checked-in native game logic.
//
// Numbered `routine_####` names are retained as stable port labels while the
// original game systems are being identified. Keep semantic discoveries in
// `docs/routine_catalog.md` first, then rename or alias routines only after the
// dataflow is understood well enough to make the name useful.
use crate::engine::RoutineFn;
use crate::frame;
use crate::native::*;
use crate::{Engine, RoutineContext};

// ROM data tables (mapped PRG). Each is the base address of a lookup table the
// game indexes; reads go through `engine.state.byte(TABLE + index)`.
/// Zero-page initialization image copied into `$00..` at boot (`$9B9F`).
pub const ZP_INIT_TABLE: i32 = 0x9B9F;
/// Stack-page initialization image copied into `$0100..` at boot (`$9C9E`).
pub const STACK_INIT_TABLE: i32 = 0x9C9E;
/// Save-state initialization template (`$9D3E`).
pub const SAVE_INIT_TABLE: i32 = 0x9D3E;
/// Object-table initialization template (`$9DC9`).
pub const OBJECT_INIT_TABLE: i32 = 0x9DC9;
/// Title-screen palette data (`$A2C9`).
pub const TITLE_PALETTE_TABLE: i32 = 0xA2C9;
/// Title-screen CHR bank list (`$A2E9`).
pub const TITLE_CHR_BANK_TABLE: i32 = 0xA2E9;
/// Sprite Y-position templates for menu/cursor sprite groups.
pub const SPRITE_Y_TABLE_A: i32 = 0xAAFC;
pub const SPRITE_Y_TABLE_B: i32 = 0xAB3C;
pub const SPRITE_Y_TABLE_C: i32 = 0xAB7C;
/// Attract-demo scripted controller input sequence (`$B0FE`).
pub const DEMO_INPUT_TABLE: i32 = 0xB0FE;
/// Sprite Y-position templates for the title/credits sprite groups.
pub const SPRITE_Y_TABLE_D: i32 = 0xB71C;
pub const SPRITE_Y_TABLE_E: i32 = 0xB6FC;
/// Per-direction horizontal / vertical movement delta tables (`$FE8B`/`$FE8C`).
pub const MOVE_DELTA_X_TABLE: i32 = 0xFE8B;
pub const MOVE_DELTA_Y_TABLE: i32 = 0xFE8C;
/// Drasle-family palette table (`$FFC5`).
pub const FAMILY_PALETTE_TABLE: i32 = 0xFFC5;
/// Note period table for the sound engine, 16-bit LE entries (`0xFDB1`).
pub const NOTE_PERIOD_TABLE: i32 = 0xFDB1;
/// Sound envelope table, 4 bytes per entry (`0xFDCB`).
pub const ENVELOPE_TABLE: i32 = 0xFDCB;
/// Per-character starting-item table used by the warp-in test scaffolding (`0xB0AC`).
pub const START_ITEM_TABLE: i32 = 0xB0AC;
/// Character palette source table used by palette helpers (`0xB000`).
pub const PALETTE_DATA_TABLE: i32 = 0xB000;
/// Base address of the 16-byte object record table ($0400-$04BF).
pub const OBJECT_TABLE_BASE: i32 = 0x0400;
/// Base of the staged room layout buffer ($0500-$07FF).
pub const ROOM_BUFFER_BASE: i32 = 0x0500;
/// PPU nametable 0 base address in VRAM ($2000).
pub const VRAM_NAMETABLE0: i32 = 0x2000;
/// Base ROM address of the four character-palette source pages (stride 256).
pub const PALETTE_SOURCE_BASE: i32 = 0x9EC9;
/// Per-character base-stats table (health/magic/jump etc.) at boot (`0xFFA7`).
pub const CHARACTER_STATS_TABLE: i32 = 0xFFA7;
/// Per-direction actor spawn X/Y offset tables (`0xFEAB`/`0xFEAC`).
pub const SPAWN_OFFSET_X_TABLE: i32 = 0xFEAB;
pub const SPAWN_OFFSET_Y_TABLE: i32 = 0xFEAC;
/// Additional sprite-Y position templates (`0xFF6B`/`0xFF6F`).
pub const SPRITE_Y_TABLE_F: i32 = 0xFF6B;
pub const SPRITE_Y_TABLE_G: i32 = 0xFF6F;
/// Status-bar/HUD nametable template (`0xFECB`).
pub const HUD_TEMPLATE_TABLE: i32 = 0xFECB;
/// Movement-pattern lookup table (`0xFFBB`).
pub const MOVEMENT_PATTERN_TABLE: i32 = 0xFFBB;
/// Object initial move-state table (`0xEEB3`).
pub const OBJ_MOVE_STATE_TABLE: i32 = 0xEEB3;
/// Sound sustain/envelope secondary table (`0xFDD2`).
pub const SUSTAIN_TABLE: i32 = 0xFDD2;
/// Song pointer table at the base of the mapped music bank (`0x8000`).
pub const SONG_POINTER_TABLE: i32 = 0x8000;
/// SFX pointer table (`0x8014`).
pub const SFX_POINTER_TABLE: i32 = 0x8014;
/// Stack-page scratch buffer base used for per-tile counters (`0x0100`).
pub const STACK_SCRATCH: i32 = 0x0100;

fn with_large_actor_asset_banks<F>(engine: &mut Engine, r: &mut RoutineContext, action: F)
where
    F: FnOnce(&mut Engine, &mut RoutineContext),
{
    let saved_bank6: i32 = (engine.state.prg_bank_8000 as i32);
    let saved_bank7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.saved_prg_bank_8000 = (saved_bank6 as u8);
    engine.state.saved_prg_bank_a000 = (saved_bank7 as u8);
    engine.state.prg_bank_8000 = 12;
    engine.state.prg_bank_a000 = 13;
    engine.state.mmc3_bank_select = 7;
    engine.prg_map_shadow();
    action(engine, r);
    engine.state.prg_bank_a000 = (saved_bank7 as u8);
    engine.state.prg_bank_8000 = (saved_bank6 as u8);
    engine.state.mmc3_bank_select = 6;
    engine.prg_map_shadow();
}

pub fn farcall_bank_09_r7(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_r7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.mmc3_bank_select = 7;
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = 9;
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 9);
    engine.state.data_ptr_hi = 0;
    r.value = 0;
    resolve_room_tile_pointer(engine, r);
    queue_room_column_vram_upload(engine, r);
    engine.state.mmc3_bank_select = 7;
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = (saved_r7 as u8);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, saved_r7);
    r.value = (saved_r7 as u8);
}

pub fn farcall_bank_0C0D_seed(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.saved_prg_bank_8000 = engine.state.prg_bank_8000;
    engine.state.saved_prg_bank_a000 = engine.state.prg_bank_a000;
    engine.state.mmc3_bank_select = 6;
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.state.prg_bank_8000 = 12;
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 12);
    engine.state.mmc3_bank_select = 7;
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = 13;
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 13);
    r.value = 13;
    r.offset = 7;
}

pub fn farcall_return_home(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prg_bank_a000 = engine.state.saved_prg_bank_a000;
    engine.state.prg_bank_8000 = engine.state.saved_prg_bank_8000;
}

/// Ticks the frame prescaler at `0x84` and decrements the eight coarse
/// timers at `0x85..0x8C` once per 60 frames.
pub fn frame_counters(engine: &mut Engine, r: &mut RoutineContext) {
    let prescaler_after = (engine.state.frame_prescaler - 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.frame_prescaler = prescaler_after;
    if (prescaler_after != 0) {
        return;
    }
    for timer_index in (0..=7).rev() {
        if (engine.state.coarse_timer(timer_index) != 0) {
            engine.state.set_coarse_timer(
                timer_index,
                (engine.state.coarse_timer(timer_index) - 1) & crate::bits::BYTE_MASK,
            );
        }
    }
    engine.state.frame_prescaler = 60;
    r.index = 255;
}

pub fn game_update(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = 0;
    let mut y: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.slot_index = 255;
                if (engine.state.pending_special_exit != 0) {
                    enter_pending_special_exit_room(engine, r);
                    return;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                check_final_exit_trigger(engine, r);
                if ((engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0) {
                    run_character_select_overlay(engine, r);
                    return;
                }
                tick_selected_item_effect(engine, r);
                if (engine.state.landing_timer != 0) {
                    engine.state.landing_timer =
                        (engine.state.landing_timer - 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.buttons = 0;
                }
                {
                    let mut clear_hi: i32 = 1;
                    if (engine.state.character_index == 4) {
                        if ((engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0)
                        {
                            clear_hi = 1;
                        } else {
                            clear_hi =
                                (if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                                    0
                                } else {
                                    1
                                });
                        }
                    } else {
                        clear_hi = (if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                            0
                        } else {
                            1
                        });
                    }
                    if ((clear_hi) != 0) {
                        engine.state.direction_latch =
                            engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8);
                    }
                }
                a = ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) as i32);
                if (a != 0) {
                    engine.state.scratch0 = (a as u8);
                    engine.state.direction_latch = (engine.state.direction_latch
                        & ((crate::bits::HIGH_NIBBLE) as u8))
                        | (a as u8);
                }
                if ((engine.state.buttons & ((crate::bits::BIT5) as u8)) != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.buttons & ((crate::bits::BIT3) as u8)) != 0) {
                    dispatch_overhead_tile_action(engine, r);
                    if ((engine.lotw_nonlocal_handoff) != 0) {
                        return;
                    }
                }
                y = 1;
                while (engine.state.inventory_item(39 + y) != 0) {
                    {
                        let __old = y;
                        y += 1;
                        __old
                    };
                    if (y >= 5) {
                        y = 6;
                        break;
                    }
                }
                r.offset = (y as u8);
                build_input_movement_delta(engine, r);
                if (engine.state.fall_frames != 0) {
                    engine.state.vertical_delta = (engine.state.fall_frames >> 2) + 1;
                    try_move_player_with_collision(engine, r);
                    if ((r.carry) == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    engine.state.horizontal_subtile_delta = 0;
                    engine.state.player_x_velocity = 0;
                    try_move_player_with_collision(engine, r);
                    if ((r.carry) == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                if (engine.state.jump_timer != 0) {
                    tick_player_jump_action(engine, r);
                    if ((engine.lotw_nonlocal_handoff) != 0) {
                        return;
                    }
                    engine.state.jump_timer = 0;
                } else if ((engine.state.buttons & ((crate::bits::BIT7) as u8)) != 0) {
                    tick_player_jump_action(engine, r);
                    if ((engine.lotw_nonlocal_handoff) != 0) {
                        return;
                    }
                    engine.state.jump_timer = 0;
                } else {
                    engine.state.collision_flag = 0;
                    engine.state.jump_timer = 0;
                }
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                try_nudge_player_to_tile_boundary(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                a = (engine.state.scratch2 as i32);
                if (a >= 239) {
                    a = 0;
                }
                engine.state.player_y = (a as u8);
                update_player_terrain_contact(engine, r);
                {
                    state = 6;
                    continue 'dispatch;
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                engine.state.jump_timer = 0;
                engine.state.fall_frames = 0;
                update_player_terrain_contact(engine, r);
                {
                    state = 6;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                engine.state.prompt_state = 16;
                loop {
                    read_debounced_buttons(engine, r);
                    if ((r.value & ((crate::bits::HIGH_NIBBLE) as u8)) != 0) {
                        break;
                    }
                    if ((engine.state.buttons & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        continue;
                    }
                    engine.state.buttons = ((((engine.state.buttons as i32) << 1)
                        & (((crate::bits::BYTE_MASK) as u8) as i32))
                        as u8);
                    engine.state.buttons = ((((engine.state.buttons as i32) << 1)
                        & (((crate::bits::BYTE_MASK) as u8) as i32))
                        as u8);
                    r.offset = 1;
                    build_input_movement_delta(engine, r);
                    {
                        let mut t: i32 = ((engine.state.vertical_delta
                            + engine.state.selected_item_slot)
                            as u8 as i32);
                        let mut ni: i32 = 0;
                        if ((t & crate::bits::BIT7) != 0) {
                            ni = 3;
                        } else if (t < 4) {
                            ni = t;
                        } else {
                            ni = 0;
                        }
                        engine.state.selected_item_slot = (ni as u8);
                    }
                    engine.state.prompt_state = 12;
                }
                engine.state.prompt_state = 16;
                state = 6;
                continue 'dispatch;
            }
            6 => {
                update_player_pose_from_motion(engine, r);
                tick_player_walk_animation(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Advances the 16-bit music stream pointer selected by the channel offset
/// in `0x02`.
pub fn increment_selected_music_stream_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    let channel_pointer_offset: i32 = (engine.state.sound_channel_offset as i32);
    let pattern_ptr_lo =
        (engine.state.sound_channel_byte(2, channel_pointer_offset) + 1) & crate::bits::BYTE_MASK;
    engine
        .state
        .set_sound_channel_byte(2, channel_pointer_offset, pattern_ptr_lo);
    if (pattern_ptr_lo == 0) {
        engine.state.set_sound_channel_byte(
            3,
            channel_pointer_offset,
            (engine.state.sound_channel_byte(3, channel_pointer_offset) + 1)
                & crate::bits::BYTE_MASK,
        );
    }
    r.index = (channel_pointer_offset as u8);
}

fn farcall_0C0D(
    engine: &mut Engine,
    r: &mut RoutineContext,
    mut lo: i32,
    mut hi: i32,
    target: RoutineFn,
) {
    let saved_bank_6: i32 = (engine.state.prg_bank_8000 as i32);
    let saved_bank_7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.saved_prg_bank_8000 = (saved_bank_6 as u8);
    engine.state.saved_prg_bank_a000 = (saved_bank_7 as u8);
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    engine.state.prg_bank_8000 = 12;
    engine.state.prg_bank_a000 = 13;
    engine.state.mmc3_bank_select = 7;
    engine.prg_map_shadow();
    target(engine, r);
    engine.state.prg_bank_a000 = (saved_bank_7 as u8);
    engine.state.prg_bank_8000 = (saved_bank_6 as u8);
    engine.state.mmc3_bank_select = 6;
    engine.prg_map_shadow();
}

/// Performs the cold-start initialization path and enters the main game
/// dispatcher after the title screen flow completes.
pub fn main_init(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(crate::engine::reg::PPU_CTRL, 0);
    engine.device_write(crate::engine::reg::PPU_MASK, 0);
    engine.device_write(crate::engine::reg::DMC_FREQ, 0);
    engine.state.sound_status_flags = 31;
    engine.device_write(crate::engine::reg::APU_STATUS, 31);
    engine.device_write(crate::engine::reg::APU_FRAME, 192);
    engine.device_write(crate::engine::reg::MMC3_MIRROR, 0);
    farcall_bank_0C0D_seed(engine, r);
    ram_state_init(engine, r);
    farcall_0C0D(engine, r, 100, 174, run_title_screen_loop);
    engine.state.landing_timer = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 48;
    engine.state.player_x_tile = 60;
    engine.state.player_y = 160;
    scene_assemble(engine, r);
    engine.state.buttons = 8;
    game_update(engine, r);
    main_loop_dispatch(engine, r);
}

/// Stages one room-column upload from the current room tile source pointer
/// and queues VRAM job `0x03`.
///
/// The nametable bytes are written to `0x0140` and `0x0158`; the matching
/// attribute byte addresses and masks are written to `0x0170..0x017B`.
pub fn queue_room_column_vram_upload(engine: &mut Engine, r: &mut RoutineContext) {
    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let tileset_quads_ptr: i32 = ((engine.state.tile_table_ptr()) as u16 as i32);

    engine.state.scratch3 = 0;
    for staging_offset in (0..=22).rev().step_by(2) {
        let metatile_id: i32 = engine
            .state
            .byte(((source_ptr + (engine.state.scratch3 as i32)) as u16 as i32));
        let tile_quad_offset: i32 = (((metatile_id << 2) as u8 as i32) as u16 as i32);
        engine.state.set_vram_stage(
            1 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 0) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        engine.state.set_vram_stage(
            staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 1) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        engine.state.set_vram_stage(
            25 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 2) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        engine.state.set_vram_stage(
            24 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 3) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        engine.state.scratch3 = (engine.state.scratch3 + 1) & ((crate::bits::BYTE_MASK) as u8);
    }

    engine.state.vram_addr2_hi = engine.state.vram_addr_hi + 3;
    let destination_low_byte: i32 = (engine.state.vram_addr_lo as i32);
    engine.state.scratch3 = (((destination_low_byte >> 2) + 192) as u8);

    let attribute_side_mask: i32 = ((destination_low_byte & crate::bits::BIT1) as u8 as i32);
    engine.state.vram_addr2_lo = if ((attribute_side_mask) != 0) {
        51
    } else {
        204
    };

    let mut source_attribute_offset: i32 = 0;
    for attribute_offset in (0..=10).rev().step_by(2) {
        engine
            .state
            .set_vram_stage(48 + attribute_offset, (engine.state.scratch3 as i32));
        engine.state.scratch3 = engine.state.scratch3 + 8;

        let top_metatile_id: i32 = engine
            .state
            .byte(((source_ptr + source_attribute_offset) as u16 as i32));
        source_attribute_offset += 1;
        let mut attribute_bits: i32 =
            (((top_metatile_id & crate::bits::HIGH_2_BITS) >> 4) as u8 as i32);

        let bottom_metatile_id: i32 = engine
            .state
            .byte(((source_ptr + source_attribute_offset) as u16 as i32));
        source_attribute_offset += 1;
        attribute_bits =
            (((bottom_metatile_id & crate::bits::HIGH_2_BITS) | attribute_bits) as u8 as i32);

        if (attribute_side_mask == 0) {
            attribute_bits = ((attribute_bits >> 2) as u8 as i32);
        }
        engine
            .state
            .set_vram_stage(49 + attribute_offset, attribute_bits);
    }

    r.value = 3;
    queue_ppu_job_and_wait(engine, r);
}

/// Writes the eight PPU bank shadows at `0x2A..0x31` to the mapper.
pub fn ppu_commit_banks(engine: &mut Engine, r: &mut RoutineContext) {
    for bank_register in (0..=7).rev() {
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, bank_register);
        engine.device_write(
            crate::engine::reg::MMC3_BANK_DATA,
            engine.state.chr_bank(bank_register),
        );
    }
    r.index = 255;
}

/// Initializes zero-page, stack work RAM, palette RAM, and persistent object
/// pages from the ROM default tables.
pub fn ram_state_init(engine: &mut Engine, r: &mut RoutineContext) {
    for zero_page_addr in 0..=255 {
        engine.state.set_byte(
            zero_page_addr,
            engine.state.byte(ZP_INIT_TABLE + zero_page_addr),
        );
    }

    for stack_offset in (0..=63).rev() {
        engine.state.set_inventory_item(
            160 + stack_offset,
            engine.state.byte(STACK_INIT_TABLE + stack_offset),
        );
    }

    for palette_offset in (0..=31).rev() {
        engine.state.set_palette_buffer(palette_offset, 15);
    }

    for save_ram_offset in 0..=255 {
        engine.state.set_save_payload(
            save_ram_offset,
            engine.state.byte(SAVE_INIT_TABLE + save_ram_offset),
        );
    }

    for object_ram_offset in 0..=255 {
        engine.state.set_password_nibbles_a(
            222 + object_ram_offset,
            engine.state.byte(OBJECT_INIT_TABLE + object_ram_offset),
        );
    }
}

/// Polls both controller ports and stores the merged button state in
/// `0x20`, using replay input when one is configured.
pub fn read_controllers(engine: &mut Engine, r: &mut RoutineContext) {
    if let Some(replay_buttons) = engine.next_input() {
        engine.ppu.buttons = (replay_buttons as u8);
    }
    engine.device_write(crate::engine::reg::JOY1, 1);
    engine.device_write(crate::engine::reg::JOY1, 0);

    for _ in 0..8 {
        let mut controller_sample: i32 = ((engine.device_read(crate::engine::reg::JOY1)
            | engine.device_read(crate::engine::reg::APU_FRAME))
            as u8 as i32);
        let player_one_bit: i32 = controller_sample & 1;
        controller_sample >>= 1;
        let player_two_bit: i32 = controller_sample & 1;
        engine.state.buttons =
            ((((engine.state.buttons as i32) << 1) | ((player_one_bit as u8) as i32)) as u8);
        engine.state.button_chord =
            ((((engine.state.button_chord as i32) << 1) | ((player_two_bit as u8) as i32)) as u8);
    }

    engine.state.buttons = engine.state.buttons | engine.state.button_chord;
}

pub fn reset(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 0);
    engine.device_write(crate::engine::reg::MMC3_PRG_RAM, 0);
    engine.device_write(crate::engine::reg::MMC3_IRQ_DISABLE, 0);
    main_init(engine, r);
}

/// Advances the two-byte RNG seed at `0x3A..0x3B` until the generated value
/// is below the limit supplied in `r.value`.
pub fn rng_update(engine: &mut Engine, r: &mut RoutineContext) {
    let limit: i32 = (r.value as u8 as i32);
    engine.state.rng_limit = (limit as u8);
    if (limit == 0) {
        r.value = (engine.state.rng_high as u8);
        return;
    }
    let mut rng_high: i32 = (engine.state.rng_high as i32);
    let mut rng_low: i32 = (engine.state.rng_low as i32);
    loop {
        engine.state.rng_seed_scratch = (rng_low as u8);

        let shifted_seed: i32 =
            ((((((rng_high << 8) | rng_low) as u16 as i32) << 1) + 1) as u16 as i32);
        rng_high = ((shifted_seed >> 8) as u8 as i32);
        rng_low = (shifted_seed as u8 as i32);

        let low_sum: i32 = ((rng_low + (engine.state.rng_low as i32)) as u16 as i32);
        rng_low = (low_sum as u8 as i32);
        let carry: i32 = ((low_sum >> 8) as u8 as i32);

        let mut candidate: i32 = ((rng_high + (engine.state.rng_high as i32) + carry) as u8 as i32);
        candidate = ((candidate + (engine.state.rng_seed_scratch as i32)) as u8 as i32);
        candidate &= 127;

        rng_high = candidate;
        engine.state.rng_high = (candidate as u8);
        engine.state.rng_low = (rng_low as u8);
        if !(candidate >= limit) {
            break;
        }
    }
    r.value = (rng_high as u8);
}

/// Builds one bank-9 metasprite/room-column slice for the scripted scrolling
/// sequence. `0xF9` is the source column, `0xFA` counts the remaining slices,
/// and `0x1D` flips between nametable halves after each 9-slice run.
pub fn advance_scripted_scroll_slice(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_x_tile == 0) {
        engine.state.vram_addr_lo = 14;
        engine.state.vram_addr_hi = 32;
        engine.state.vram_addr_hi =
            (((((engine.state.nametable_select ^ ((crate::bits::BIT0) as u8)) << 2) as u8 as i32)
                | (engine.state.vram_addr_hi as i32)) as u8);
        engine.state.obj_x_sub =
            ((((((engine.state.nametable_select ^ ((crate::bits::BIT0) as u8)) << 4) + 7) as u8
                as i32)
                | (engine.state.scroll_tile_x as i32)) as u8);
        engine.state.obj_x_tile = 9;
    }
    engine.state.data_ptr_lo = engine.state.obj_x_sub;
    farcall_bank_09_r7(engine, r);
    engine.state.vram_addr_lo = engine.state.vram_addr_lo + 1;
    engine.state.vram_addr_lo = engine.state.vram_addr_lo + 1;
    engine.state.obj_x_sub = engine.state.obj_x_sub + 1;
    engine.state.obj_x_tile = engine.state.obj_x_tile - 1;
    if (engine.state.obj_x_tile == 0) {
        engine.state.nametable_select = engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
    }
}

/// Updates the three final-exit projectile slots at `0x0410..0x043F`,
/// spawning a new shot on the action-button edge when a slot is empty.
pub fn update_final_exit_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.slot_index = 1;
    engine.state.obj_slot_ptr_lo = 16;
    engine.state.obj_slot_ptr_hi = 4;
    loop {
        let slot_ptr = ((engine.state.obj_slot_ptr()) as u16 as i32);
        if (engine.state.byte(((slot_ptr + 1) as u16 as i32)) != 0) {
            update_final_exit_projectile_slot(engine, r);
        } else if (((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0)
            && ((engine.state.direction_latch & ((crate::bits::BIT6) as u8)) == 0))
        {
            spawn_final_exit_projectile(engine, r);
        }
        engine.state.slot_index = engine.state.slot_index + 1;
        {
            let next_slot_ptr = ((engine.state.obj_slot_ptr_lo + 16) as u16 as i32);
            engine.state.obj_slot_ptr_lo = (next_slot_ptr as u8);
            engine.state.obj_slot_ptr_hi =
                engine.state.obj_slot_ptr_hi + ((next_slot_ptr >> 8) as u8);
        }
        if !(engine.state.slot_index < 4) {
            break;
        }
    }
    draw_final_exit_projectile_sprites(engine, r);
}

/// Initializes one final-exit projectile slot from the player's current
/// position and action direction.
pub fn spawn_final_exit_projectile(engine: &mut Engine, r: &mut RoutineContext) {
    load_object_slot_scratch(engine, r);
    engine.state.direction_latch =
        (engine.state.buttons & ((crate::bits::BIT6) as u8)) | engine.state.direction_latch;
    r.value = (engine.state.direction_latch as u8);
    r.offset = 2;
    build_final_exit_projectile_velocity(engine, r);
    project_final_exit_projectile_spawn(engine, r);
    check_final_exit_projectile_bounds(engine, r);
    if ((r.carry) == 0) {
        engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
        engine.state.obj_y_pixel = engine.state.scratch2;
        engine.state.obj_state = 24;
        engine.state.obj_attr = 0;
        engine.state.obj_tile = 33;
        engine.state.prompt_state = 25;
    }
    if (engine.state.obj_state != 0) {
        update_final_exit_projectile_animation_bits(engine, r);
    }
    store_object_slot_scratch(engine, r);
}

/// Ticks one active final-exit projectile slot, clearing it when its
/// lifetime expires or the projected position trips the bounds check.
pub fn update_final_exit_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
    load_object_slot_scratch(engine, r);
    engine.state.obj_state = engine.state.obj_state - 1;
    if (engine.state.obj_state != 0) {
        project_final_exit_projectile_motion(engine, r);
        check_final_exit_projectile_bounds(engine, r);
        if ((r.carry) != 0) {
            engine.state.obj_state = 0;
        } else {
            engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
            engine.state.obj_y_pixel = engine.state.scratch2;
        }
    }
    if (engine.state.obj_state != 0) {
        update_final_exit_projectile_animation_bits(engine, r);
    }
    store_object_slot_scratch(engine, r);
}

/// Projects the spawn point from the player position using velocity scaled
/// by four pixels so new shots start ahead of the player.
pub fn project_final_exit_projectile_spawn(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.scratch2 = engine.state.player_y;
    if (engine.state.obj_y_vel != 0) {
        let scaled_y_delta = (((engine.state.obj_y_vel as i32) << 2) as u8 as i32);
        engine.state.scratch2 = ((scaled_y_delta + (engine.state.scratch2 as i32)) as u8);
    }
    if (engine.state.obj_x_vel_lo != 0) {
        let scaled_x_delta = (((engine.state.obj_x_vel_lo as i32) << 2) as u8 as i32);
        engine.state.indirect_ptr_lo =
            ((scaled_x_delta + (engine.state.indirect_ptr_lo as i32)) as u8);
    }
}

/// Folds the projectile lifetime phase into the slot state bits used by the
/// final-exit projectile sprite animation.
pub fn update_final_exit_projectile_animation_bits(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch0 = engine.state.obj_state & ((crate::bits::BITS_2_3) as u8);
    engine.state.obj_tile =
        (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8)) | engine.state.scratch0;
}

/// Raises carry when the projected projectile has crossed the right edge
/// while still in the scripted vertical range. Other paths intentionally
/// leave carry untouched to preserve the original branch contract.
pub fn check_final_exit_projectile_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.scratch2 >= 161) {
        return;
    }
    if (engine.state.indirect_ptr_lo < 241) {
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    r.carry = 1;
}

/// Projects one active final-exit projectile from its saved slot position
/// and per-frame velocity.
pub fn project_final_exit_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    if (engine.state.obj_y_vel != 0) {
        engine.state.scratch2 = engine.state.obj_y_vel + engine.state.scratch2;
    }
    if (engine.state.obj_x_vel_lo != 0) {
        engine.state.indirect_ptr_lo = engine.state.obj_x_vel_lo + engine.state.indirect_ptr_lo;
    }
}

/// Draws all three final-exit projectile slots into their fixed OAM ranges.
pub fn draw_final_exit_projectile_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_hi = 136;
    engine.state.indirect_ptr_lo = 16;
    for _ in 0..3 {
        draw_final_exit_projectile_slot_sprites(engine, r);
        engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi + 8;
        engine.state.indirect_ptr_lo = engine.state.indirect_ptr_lo + 16;
    }
}

/// Draws one final-exit projectile as a two-sprite pair or hides it when the
/// slot is inactive/offscreen.
pub fn draw_final_exit_projectile_slot_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let oam_offset = engine.state.indirect_ptr_hi;
    let slot_offset = engine.state.indirect_ptr_lo;
    if (engine.state.object_state((slot_offset as i32)) == 0)
        || (engine.state.object_y_pixel((slot_offset as i32)) >= 191)
    {
        engine.state.set_oam_y((oam_offset as i32), 239);
        engine.state.set_oam_y(((4 + oam_offset) as i32), 239);
        return;
    }

    let attributes = engine.state.object_attr((slot_offset as i32));
    engine.state.set_oam_attr((oam_offset as i32), attributes);
    engine
        .state
        .set_oam_attr(((4 + oam_offset) as i32), attributes);

    let tile_id = engine.state.object_tile((slot_offset as i32));
    if ((attributes & crate::bits::BIT6) != 0) {
        engine
            .state
            .set_oam_tile(((4 + oam_offset) as i32), tile_id);
        engine.state.set_oam_tile((oam_offset as i32), tile_id + 2);
    } else {
        engine.state.set_oam_tile((oam_offset as i32), tile_id);
        engine
            .state
            .set_oam_tile(((4 + oam_offset) as i32), tile_id + 2);
    }

    let projectile_x = engine.state.object_x_sub((slot_offset as i32));
    engine.state.set_oam_x((oam_offset as i32), projectile_x);
    engine
        .state
        .set_oam_x(((4 + oam_offset) as i32), projectile_x + 8);

    let projectile_y = ((engine.state.object_y_pixel((slot_offset as i32)) + 43) as u8 as i32);
    engine.state.set_oam_y((oam_offset as i32), projectile_y);
    engine
        .state
        .set_oam_y(((4 + oam_offset) as i32), projectile_y);
}

/// Rotates one scripted OAM entry into sprite zero and hides the source
/// sprite. The sequence cycles through player/projectile sprites via `0x3E`.
pub fn rotate_sprite_zero_from_scripted_oam(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sprite_index = ((engine.state.sprite_index - 1) as u8 as i32);
    if ((sprite_index & crate::bits::BIT7) != 0) {
        sprite_index = 7;
    }
    engine.state.sprite_index = (sprite_index as u8);
    let oam_offset = ((sprite_index << 2) as u8 as i32);
    let source_base = if ((sprite_index & crate::bits::BITS_1_2) != 0) {
        640
    } else {
        528
    };
    engine.state.set_oam_y(
        0,
        engine
            .state
            .byte(((source_base + oam_offset) as u16 as i32)),
    );
    engine.state.set_oam_tile(
        0,
        engine
            .state
            .byte(((source_base + 1 + oam_offset) as u16 as i32)),
    );
    engine.state.set_oam_attr(
        0,
        engine
            .state
            .byte(((source_base + 2 + oam_offset) as u16 as i32)),
    );
    engine.state.set_oam_x(
        0,
        engine
            .state
            .byte(((source_base + 3 + oam_offset) as u16 as i32)),
    );
    engine
        .state
        .set_byte(((source_base + oam_offset) as u16 as i32), 239);
}

/// Converts the latched action direction into final-exit projectile velocity
/// by accumulating the movement table for `r.offset` steps.
pub fn build_final_exit_projectile_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let direction_table_offset =
        (((r.value & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    let step_count = r.offset;
    let mut x_velocity = 0;
    let mut remaining_steps = step_count;
    loop {
        x_velocity = ((x_velocity
            + engine
                .state
                .byte(((MOVE_DELTA_X_TABLE + direction_table_offset) as u16 as i32)))
            as u8 as i32);
        remaining_steps -= 1;
        if (remaining_steps == 0) {
            break;
        }
    }
    engine.state.obj_x_vel_lo = (x_velocity as u8);

    let mut y_velocity = 0;
    remaining_steps = step_count;
    loop {
        y_velocity = ((y_velocity
            + engine
                .state
                .byte(((MOVE_DELTA_Y_TABLE + direction_table_offset) as u16 as i32)))
            as u8 as i32);
        remaining_steps -= 1;
        if (remaining_steps == 0) {
            break;
        }
    }
    engine.state.obj_y_vel = (y_velocity as u8);
}

/// Loads the final-exit object OAM template and rebuilds the standard object
/// health meter.
pub fn load_final_exit_object_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    for oam_offset in (0..=63).rev() {
        engine.state.set_oam_y(
            64 + oam_offset,
            engine.state.byte(SPRITE_Y_TABLE_A + oam_offset),
        );
    }
    build_object_health_meter_standard_tiles(engine, r);
}

/// Loads the large-actor OAM template and rebuilds the alternate object
/// health meter.
pub fn load_large_actor_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    for oam_offset in (0..=63).rev() {
        engine.state.set_oam_y(
            64 + oam_offset,
            engine.state.byte(SPRITE_Y_TABLE_B + oam_offset),
        );
    }
    build_object_health_meter_alt_tiles(engine, r);
}

/// Loads the final-exit player-side OAM template and rebuilds the player
/// health meter.
pub fn load_final_exit_player_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    for oam_offset in (0..=63).rev() {
        engine.state.set_oam_y(
            192 + oam_offset,
            engine.state.byte(SPRITE_Y_TABLE_C + oam_offset),
        );
    }
    build_player_health_meter_sprites(engine, r);
}

/// Mirrors the player pose and position into the three linked final-exit
/// body slots used by the scripted cutscene.
pub fn sync_final_exit_body_slots_from_player(engine: &mut Engine, r: &mut RoutineContext) {
    let pose_tile_bits =
        ((engine.state.player_pose & ((crate::bits::LOW_5_BITS) as u8)) as u8 as i32);
    engine.state.scratch0 = (pose_tile_bits as u8);
    engine.state.set_object_tile(
        16,
        (((engine.state.object_tile(16) & crate::bits::HIGH_3_BITS) | pose_tile_bits) as u8 as i32),
    );
    engine.state.set_object_tile(
        32,
        (((engine.state.object_tile(32) & crate::bits::HIGH_3_BITS) | pose_tile_bits) as u8 as i32),
    );
    engine.state.set_object_tile(
        48,
        (((engine.state.object_tile(48) & crate::bits::HIGH_3_BITS) | pose_tile_bits) as u8 as i32),
    );

    let player_x = engine.state.player_x_fine;
    engine.state.set_object_x_sub(16, (player_x as i32));
    engine.state.set_object_x_sub(32, (player_x as i32));
    engine.state.set_object_x_sub(48, (player_x as i32));

    let player_tile_x = engine.state.player_x_tile;
    engine
        .state
        .set_object_x_tile(32, ((player_tile_x + 1) as i32));
    engine
        .state
        .set_object_x_tile(48, ((player_tile_x - 2) as i32));
    engine
        .state
        .set_object_x_tile(16, ((player_tile_x - 3) as i32));
}

fn finish_scripted_player_motion_frame(engine: &mut Engine, r: &mut RoutineContext) {
    update_scripted_player_pose_from_motion(engine, r);
    tick_scripted_player_walk_animation(engine, r);
    draw_scripted_player_sprites(engine, r);
}

fn commit_scripted_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.player_x_fine = engine.state.indirect_ptr_lo;
    engine.state.player_y = engine.state.scratch2;
    update_scripted_player_fall_state(engine, r);
    finish_scripted_player_motion_frame(engine, r);
}

fn cancel_scripted_player_motion(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.jump_timer = 0;
    engine.state.fall_frames = 0;
    update_scripted_player_fall_state(engine, r);
    finish_scripted_player_motion_frame(engine, r);
}

/// Ticks the reduced player controller used inside scripted/final-exit
/// scenes. It mirrors the normal gameplay movement path but only checks the
/// scripted screen bounds room tiles or object contacts.
pub fn tick_scripted_player_motion(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = (engine.state.buttons as u8);
    if ((r.value & ((crate::bits::BIT4) as u8)) != 0) {
        wait_for_start_button_prompt(engine, r);
        return;
    }

    if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) == 0) {
        engine.state.direction_latch =
            engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8);
    }
    let directional_buttons =
        ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) as u8 as i32);
    r.value = (directional_buttons as u8);
    if (directional_buttons != 0) {
        engine.state.scratch0 = (directional_buttons as u8);
        engine.state.direction_latch = (engine.state.direction_latch
            & ((crate::bits::HIGH_NIBBLE) as u8))
            | engine.state.scratch0;
    }

    if (engine.state.sprite_blink_timer == 0) {
        if engine.state.sprite0_hit() {
            r.index = ((engine.state.sprite_index + 1) as u8);
            if ((r.index & ((crate::bits::BITS_1_2) as u8)) == 0) {
                let collision_screen_x = (((engine.state.scroll_pixel_x
                    + ((engine.state.object_x_sub((r.index as i32))) as u8))
                    as u8) as i32);
                r.value = ((if (collision_screen_x < 176) { 10 } else { 5 }) as u8);
                subtract_scripted_player_health(engine, r);
                engine.state.jump_timer = 10;
                engine.state.prompt_state = 33;
                engine.state.prompt_argument = 2;
                engine.state.sprite_blink_timer = 1;
                build_player_health_meter_sprites(engine, r);
            }
        }
    }

    if (engine.state.jump_timer == 0) && (engine.state.fall_frames == 0) {
        engine.state.sprite_blink_timer = 0;
    } else {
        engine.state.buttons = (engine.state.buttons & ((crate::bits::HIGH_NIBBLE) as u8))
            | ((crate::bits::BIT1) as u8);
    }

    build_scripted_player_input_delta(engine, r);
    if (engine.state.fall_frames != 0) {
        r.value = (((engine.state.fall_frames >> 2) + 1) as u8);
        engine.state.vertical_delta = (r.value as u8);
        try_move_scripted_player_in_bounds(engine, r);
        if ((r.carry) == 0) {
            commit_scripted_player_position(engine, r);
            return;
        }

        engine.state.horizontal_subtile_delta = 0;
        try_move_scripted_player_in_bounds(engine, r);
        if ((r.carry) == 0) {
            return;
        }

        cancel_scripted_player_motion(engine, r);
        return;
    }

    if (engine.state.jump_timer != 0) || ((engine.state.buttons & ((crate::bits::BIT7) as u8)) != 0)
    {
        tick_scripted_player_jump_action(engine, r);
        r.value = 0;
    } else {
        engine.state.collision_flag = 0;
        r.value = 0;
    }

    engine.state.jump_timer = (r.value as u8);
    try_move_scripted_player_in_bounds(engine, r);
    if ((r.carry) != 0) {
        cancel_scripted_player_motion(engine, r);
        return;
    }
    commit_scripted_player_position(engine, r);
}

/// Starts or advances the scripted jump arc. `0x4F` is the jump timer and
/// `0x22` blocks held-button retriggers, matching the normal player jump
/// helper without item/magic extensions.
pub fn tick_scripted_player_jump_action(engine: &mut Engine, r: &mut RoutineContext) {
    let jump_timer = engine.state.jump_timer;
    if (jump_timer == 0) {
        if (engine.state.collision_flag != 0) {
            return;
        }
        engine.state.prompt_state = 27;
        engine.state.jump_timer = engine.state.jump_strength;
    }
    engine.state.collision_flag = 1;
    engine.state.jump_timer = engine.state.jump_timer - 1;
    engine.state.vertical_delta =
        (((((jump_timer >> 2) as u8 as i32) ^ crate::bits::BYTE_MASK) + 1) as u8);
    try_move_scripted_player_in_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.horizontal_subtile_delta = 0;
        try_move_scripted_player_in_bounds(engine, r);
    }
    if ((r.carry) == 0) {
        engine.state.player_x_fine = engine.state.indirect_ptr_lo;
        engine.state.player_y = engine.state.scratch2;
        update_scripted_player_fall_state(engine, r);
    } else {
        engine.state.jump_timer = 0;
        engine.state.fall_frames = 0;
        update_scripted_player_fall_state(engine, r);
    }
    update_scripted_player_pose_from_motion(engine, r);
    tick_scripted_player_walk_animation(engine, r);
    draw_scripted_player_sprites(engine, r);
}

/// Projects scripted player X/Y into `0x0E/0x0A` from the current position
/// and movement deltas `0x49/0x4B`.
pub fn project_scripted_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.scratch2 = engine.state.player_y;
    if (engine.state.vertical_delta != 0) {
        engine.state.scratch2 = engine.state.vertical_delta + engine.state.scratch2;
    }
    if (engine.state.horizontal_subtile_delta != 0) {
        engine.state.indirect_ptr_lo =
            engine.state.horizontal_subtile_delta + engine.state.indirect_ptr_lo;
    }
}

fn apply_scripted_horizontal_pose(
    engine: &mut Engine,
    r: &mut RoutineContext,
    pose_bits: i32,
    preserve_mask: i32,
) -> bool {
    r.index = (pose_bits as u8);
    r.offset = 0;
    if ((engine.state.horizontal_subtile_delta & ((crate::bits::BIT7) as u8)) != 0) {
        // Negative horizontal deltas face left with no sprite flip.
    } else if (engine.state.horizontal_subtile_delta == 0) {
        return false;
    } else {
        r.offset = 64;
    }

    engine.state.scratch0 = (r.index as u8);
    engine.state.player_pose =
        (engine.state.player_pose & (preserve_mask as u8)) | engine.state.scratch0;
    engine.state.player_facing = (r.offset as u8);
    true
}

/// Chooses the scripted player pose and horizontal flip from movement,
/// jump/fall state, and the action button.
pub fn update_scripted_player_pose_from_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let jump_pose = 9;
    if (((engine.state.buttons & ((crate::bits::CLEAR_BIT6) as u8)) as u8 as i32) == 128) {
        r.index = jump_pose;
        engine.state.player_pose = (r.index as u8);
        return;
    }

    if (engine.state.vertical_delta != 0) {
        if ((engine.state.vertical_delta & ((crate::bits::BIT7) as u8)) != 0) {
            if (engine.state.jump_timer == 0) {
                r.index = jump_pose;
                engine.state.player_pose = (r.index as u8);
                return;
            }
        } else if (engine.state.fall_frames == 0) {
            if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) != 0) {
                r.index = 13;
                engine.state.player_pose = (r.index as u8);
                return;
            }
            apply_scripted_horizontal_pose(engine, r, 1, 7);
            return;
        }

        apply_scripted_horizontal_pose(engine, r, 57, 3);
        return;
    }

    apply_scripted_horizontal_pose(engine, r, 1, 7);
}

/// Applies action-button pose bits and toggles the scripted walk frame every
/// eight moving frames when not jumping or falling.
pub fn tick_scripted_player_walk_animation(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.player_pose < 32) {
        let mut pose = engine.state.player_pose;
        if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
            pose = ((pose | ((crate::bits::BIT4) as u8)) as u8);
        } else {
            pose = ((pose & ((crate::bits::CLEAR_BIT4) as u8)) as u8);
        }
        engine.state.player_pose = pose;
    }
    if ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    if ((engine.state.jump_timer | engine.state.fall_frames) != 0) {
        return;
    }
    engine.state.anim_step_counter = engine.state.anim_step_counter + 1;
    if ((engine.state.anim_step_counter & ((crate::bits::LOW_3_BITS) as u8)) != 0) {
        return;
    }
    if ((engine.state.player_pose & ((crate::bits::BIT3) as u8)) != 0) {
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8);
    } else {
        engine.state.player_pose = engine.state.player_pose ^ ((crate::bits::BIT2) as u8);
    }
}

/// Draws the two scripted-player sprites into fixed OAM entries
/// `0x0210/0x0214`, including blink hiding and horizontal tile order.
pub fn draw_scripted_player_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.sprite_blink_timer != 0) {
        if ((engine.state.frame_prescaler & ((crate::bits::BIT0) as u8)) == 0) {
            engine.state.set_oam_y(16, 239);
            engine.state.set_oam_y(20, 239);
            return;
        }
    }
    engine
        .state
        .set_oam_y(16, ((engine.state.player_y + 43) as i32));
    engine
        .state
        .set_oam_y(20, ((engine.state.player_y + 43) as i32));
    engine
        .state
        .set_oam_x(16, (engine.state.player_x_fine as i32));
    engine
        .state
        .set_oam_x(20, ((engine.state.player_x_fine + 8) as i32));
    engine.state.set_oam_attr(
        16,
        ((engine.state.player_facing | ((crate::bits::BIT5) as u8)) as i32),
    );
    engine.state.set_oam_attr(
        20,
        ((engine.state.player_facing | ((crate::bits::BIT5) as u8)) as i32),
    );
    if ((engine.state.player_facing & ((crate::bits::BIT6) as u8)) != 0) {
        r.index = (engine.state.player_pose as u8);
        engine.state.set_oam_tile(20, (r.index as i32));
        r.index = ((r.index + 2) as u8);
        engine.state.set_oam_tile(16, (r.index as i32));
    } else {
        r.index = (engine.state.player_pose as u8);
        engine.state.set_oam_tile(16, (r.index as i32));
        r.index = ((r.index + 2) as u8);
        engine.state.set_oam_tile(20, (r.index as i32));
    }
}

/// Projects scripted player motion and retries vertical movement toward zero
/// until the screen-bounds check succeeds or the delta is exhausted.
pub fn try_move_scripted_player_in_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_y_delta = engine.state.vertical_delta;
    loop {
        project_scripted_player_position(engine, r);
        check_scripted_player_bounds(engine, r);
        if ((r.carry) == 0) {
            break;
        }
        {
            let mut adjusted_y_delta = engine.state.vertical_delta;
            if (adjusted_y_delta == 0) {
                r.carry = 1;
                break;
            }
            if ((adjusted_y_delta & ((crate::bits::BIT7) as u8)) == 0) {
                adjusted_y_delta = ((adjusted_y_delta - 1) as u8);
                adjusted_y_delta = ((adjusted_y_delta - 1) as u8);
            }
            adjusted_y_delta = ((adjusted_y_delta + 1) as u8);
            engine.state.vertical_delta = adjusted_y_delta;
            if (adjusted_y_delta != 0) {
                continue;
            }
            r.carry = 1;
            break;
        }
    }
    engine.state.vertical_delta = saved_y_delta;
}

/// Updates scripted falling/contact timers. `0x4E` counts fall frames while
/// the player is above the landing Y, and a long fall seeds jump timer
/// `0x4F` for a bounce prompt.
pub fn update_scripted_player_fall_state(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.jump_timer != 0) {
        r.carry = 0;
        return;
    }
    if (engine.state.player_y < 160) {
        engine.state.fall_frames = engine.state.fall_frames + 1;
        return;
    }
    {
        let mut fall_frames = engine.state.fall_frames;
        if (fall_frames >= engine.state.jump_strength) {
            fall_frames = ((fall_frames - 7) as u8);
            if (fall_frames >= engine.state.jump_strength) {
                fall_frames = engine.state.jump_strength;
            }
            fall_frames = ((fall_frames - 1) as u8);
            engine.state.jump_timer = fall_frames;
            engine.state.prompt_state = 10;
        }
    }
    engine.state.fall_frames = 0;
}

/// Subtracts scripted contact damage from health and saturates underflow at
/// zero while preserving the 6502-style flags in `RoutineContext`.
pub fn subtract_scripted_player_health(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch0 = (r.value as u8);
    let current_health = engine.state.player_health;
    {
        let difference = (current_health as u16 as i32) - (engine.state.scratch0 as u16 as i32);
        let result = (difference as u8 as i32);
        r.carry = if ((difference & crate::bits::BIT8) != 0) {
            0
        } else {
            1
        };
        r.zero = if (result == 0) { 1 } else { 0 };
        r.negative = (((result >> 7) & 1) as u8);
        engine.state.player_health = result as u8;
    }
    if ((r.carry) == 0) {
        engine.state.player_health = 0;
    }
}

/// Rejects projected scripted-player positions outside the final-exit screen
/// bounds.
pub fn check_scripted_player_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.scratch2 >= 161) {
        r.carry = 1;
        return;
    }
    if (engine.state.indirect_ptr_lo >= 241) {
        r.carry = 1;
        return;
    }
    r.carry = 0;
}

/// Converts the lower controller nibble into scripted-player X/Y velocity
/// scratch using the same ROM movement table as the original routine.
pub fn build_scripted_player_input_delta(engine: &mut Engine, r: &mut RoutineContext) {
    r.index = (((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8);
    engine.state.horizontal_subtile_delta = ((engine
        .state
        .byte(((MOVE_DELTA_X_TABLE + (r.index as i32)) as u16 as i32)))
        as u8);
    engine.state.vertical_delta = ((engine
        .state
        .byte(((MOVE_DELTA_Y_TABLE + (r.index as i32)) as u16 as i32)))
        as u8);
}

/// Chooses a pseudo-random controller byte for the title-screen demo loop.
pub fn choose_random_demo_input(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 4;
    rng_update(engine, r);
    r.index = r.value;
    engine.state.buttons = ((engine
        .state
        .byte(((DEMO_INPUT_TABLE + (r.index as i32)) as u16 as i32)))
        as u8);
    r.value = 10;
    rng_update(engine, r);
    r.index = r.value;
    if (r.index == 0) {
        engine.state.buttons = engine.state.buttons | ((crate::bits::BIT6) as u8);
    }
}

/// Loads the full title-screen OAM template into the sprite staging area.
pub fn load_title_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    for offset in (0..=127).rev() {
        engine
            .state
            .set_oam_y(64 + offset, engine.state.byte(SPRITE_Y_TABLE_D + offset));
    }
    r.index = 255;
}

/// Loads the smaller demo-mode OAM template used after the title timeout.
pub fn load_demo_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    for offset in (0..=31).rev() {
        engine
            .state
            .set_oam_y(64 + offset, engine.state.byte(SPRITE_Y_TABLE_E + offset));
    }
    r.index = 255;
}

/// Toggles the first eight demo sprites on and off from the frame timer.
pub fn blink_demo_oam_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sprite_y: i32 = 239;
    if ((engine.state.frame_prescaler & ((crate::bits::BITS_4_5) as u8)) != 0) {
        sprite_y = 128;
    }
    for oam_offset in (0..=28).step_by(4) {
        engine.state.set_oam_y(64 + oam_offset, sprite_y);
    }
    r.index = (sprite_y as u8);
}

/// Stages one intro text line into `0x0140` until CR or terminator.
pub fn stage_intro_text_line(engine: &mut Engine, r: &mut RoutineContext) {
    clear_text_staging_buffer(engine, r);

    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let mut text_offset: i32 = 0;
    let mut guard: i32 = 0;
    while (guard < 256) {
        let source_byte: i32 = engine
            .state
            .byte(((source_ptr + text_offset) as u16 as i32));
        if (source_byte == 0) {
            r.carry = 1;
            return;
        }
        if (source_byte == 13) {
            set_intro_text_vram_address(engine, r);
            r.value = 5;
            upload_intro_text_scroll_slice(engine, r);
            r.carry = 0;
            return;
        }

        engine.state.scratch0 = ((source_byte & crate::bits::LOW_NIBBLE) as u8);
        engine.state.set_vram_stage(
            text_offset,
            ((((source_byte & crate::bits::HIGH_NIBBLE) << 1) | (engine.state.scratch0 as i32))
                as u8 as i32),
        );
        guard += 1;
        text_offset += 1;
    }
}

/// Stages the next intro text line, advances the source pointer past CR,
/// and offsets the tile ids for the scrolling text row.
pub fn stage_scrolling_intro_text_line(engine: &mut Engine, r: &mut RoutineContext) {
    clear_text_staging_buffer(engine, r);

    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let mut text_offset: i32 = 0;
    let mut scan_guard: i32 = 0;
    while (scan_guard < 256) {
        let source_byte: i32 = engine
            .state
            .byte(((source_ptr + text_offset) as u16 as i32));
        if (source_byte == 0) {
            r.carry = 1;
            return;
        }
        if (source_byte == 13) {
            text_offset += 1;

            let advanced_source: i32 =
                ((text_offset + (engine.state.data_ptr_lo as i32)) as u16 as i32);
            engine.state.data_ptr_lo = (advanced_source as u8);
            if (advanced_source > 255) {
                engine.state.data_ptr_hi = engine.state.data_ptr_hi + 1;
            }

            r.value = 8;
            set_intro_text_vram_address(engine, r);
            r.value = 5;
            upload_intro_text_scroll_slice(engine, r);
            r.carry = 0;
            return;
        }

        let low_nibble: i32 = source_byte & crate::bits::LOW_NIBBLE;
        engine.state.scratch0 = (low_nibble as u8);

        let high_bits: i32 = (((source_byte & crate::bits::HIGH_NIBBLE) << 1) as u8 as i32);
        let tile_id: i32 = (((high_bits | (engine.state.scratch0 as i32)) + 16) as u8 as i32);
        engine.state.set_vram_stage(text_offset, tile_id);

        text_offset += 1;
        scan_guard += 1;
    }
}

/// Converts intro text scroll offset `0x0A` into a nametable address.
pub fn set_intro_text_vram_address(engine: &mut Engine, r: &mut RoutineContext) {
    let address: i32 = VRAM_NAMETABLE0 + (((engine.state.scratch2 as i32) << 2) as i32);
    engine.state.vram_addr_hi = ((address >> 8) as u8);
    engine.state.vram_addr_lo = (address as u8);
    r.value = (address as u8);
}

/// Advances intro text scroll one pixel at a time, flushing partial slices
/// until the offset reaches the next 8-pixel row boundary.
pub fn advance_intro_text_scroll(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.scratch2 = (engine.state.scratch2 + 1) & ((crate::bits::BYTE_MASK) as u8);
        if ((engine.state.scratch2 & ((crate::bits::LOW_3_BITS) as u8)) == 0) {
            break;
        }
        r.value = 255;
        upload_intro_text_scroll_slice(engine, r);
    }
    if (engine.state.scratch2 == 240) {
        engine.state.scratch2 = 0;
    }
}

/// Uploads the staged intro text row plus three spacer rows for the current
/// scroll offset.
pub fn upload_intro_text_scroll_slice(engine: &mut Engine, r: &mut RoutineContext) {
    let first_job_id: i32 = (r.value as u8 as i32);
    let mut scroll_upload_row: i32 = ((engine.state.scratch2 + 6) as u8 as i32);
    if (scroll_upload_row >= 240) {
        scroll_upload_row = ((scroll_upload_row + 16) as u8 as i32);
    }
    engine.state.scroll_y = (scroll_upload_row as u8);
    r.value = (first_job_id as u8);
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
}

/// Loads the intro/text palette and queues it for upload.
pub fn load_intro_text_palette(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_palette_buffer(0, 15);
    engine.state.set_palette_buffer(1, 12);
    engine.state.set_palette_buffer(2, 16);
    engine.state.set_palette_buffer(3, 48);
    for palette_offset in (0..=27).rev() {
        engine.state.set_palette_buffer(4 + palette_offset, 15);
    }
    r.value = 15;
    upload_palette_buffer(engine, r);
}

/// Hides every staged sprite by writing the offscreen Y value to each OAM
/// entry while leaving tile/attribute/X bytes untouched.
pub fn hide_all_sprite_y_positions(engine: &mut Engine, r: &mut RoutineContext) {
    for oam_offset in (0..=252).step_by(4) {
        engine.state.set_oam_y(oam_offset, 239);
    }
    r.index = 0;
    r.value = 239;
}

/// Clears the 32-byte text staging buffer to blank tile `0xC0`.
pub fn clear_text_staging_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    for offset in (0..=31).rev() {
        engine.state.set_vram_stage(offset, 192);
    }
    r.value = 192;
    r.offset = 255;
}

/// Encodes the saved progress/inventory snapshot into the 32-byte item-list
/// buffer used by the status/password page.
///
/// The first half of `0x0322..0x0341` stores progress nibbles plus key bits;
/// the second half stores inventory nibbles plus coin bits. Sum/xor
/// checksums are folded into spare bits before all non-seed entries are
/// scrambled with `rng_update`.
pub fn encode_inventory_snapshot_item_list(engine: &mut Engine, r: &mut RoutineContext) {
    // Split the saved progress bytes into low-nibble item-list entries.
    let mut progress_entry_offset: i32 = 15;
    for progress_byte_offset in (0..=7).rev() {
        let progress_byte: i32 = engine.state.save_progress(progress_byte_offset);
        engine
            .state
            .set_password_nibbles_a(progress_entry_offset, progress_byte >> 4);
        progress_entry_offset -= 1;
        engine.state.set_password_nibbles_a(
            progress_entry_offset,
            progress_byte & crate::bits::LOW_NIBBLE,
        );
        progress_entry_offset -= 1;
    }
    // Copy saved inventory counts into the second half of the item list.
    for inventory_offset in (0..=15).rev() {
        engine.state.set_password_nibbles_b(
            inventory_offset,
            ((engine.state.save_inventory(inventory_offset) & crate::bits::LOW_NIBBLE) as u8
                as i32),
        );
    }
    // Fold the saved key and coin counters into every other high-bit slot.
    {
        let mut key_bits: i32 = engine.state.save_inventory(16);
        for entry_offset in (0..=15).rev().step_by(2) {
            let carry_bit: i32 = ((key_bits & 1) as u8 as i32);
            key_bits >>= 1;
            let entry: i32 = engine.state.password_nibbles_a(entry_offset);
            engine
                .state
                .set_password_nibbles_a(entry_offset, (entry << 1) | carry_bit);
        }
    }
    {
        let mut coin_bits: i32 = engine.state.save_inventory(17);
        for entry_offset in (0..=15).rev().step_by(2) {
            let carry_bit: i32 = ((coin_bits & 1) as u8 as i32);
            coin_bits >>= 1;
            let entry: i32 = engine.state.password_nibbles_b(entry_offset);
            engine
                .state
                .set_password_nibbles_b(entry_offset, (entry << 1) | carry_bit);
        }
    }
    // Compute the checksum bytes over the packed-but-unscrambled entries.
    {
        let mut additive_checksum: i32 = 0;
        for entry_offset in (0..=31).rev() {
            additive_checksum =
                ((additive_checksum + engine.state.password_nibbles_a(entry_offset)) as u8 as i32);
        }
        engine.state.password_checksum_add = (additive_checksum as u8);
    }
    {
        let mut xor_checksum: i32 = 10;
        for entry_offset in (0..=31).rev() {
            xor_checksum =
                ((xor_checksum ^ engine.state.password_nibbles_a(entry_offset)) as u8 as i32);
        }
        engine.state.password_checksum_xor = (xor_checksum as u8);
    }
    // Store the checksum bits in the remaining high-bit slots.
    {
        let mut additive_checksum_bits: i32 = (engine.state.password_checksum_add as i32);
        for entry_offset in (0..=14).rev().step_by(2) {
            let carry_bit: i32 = ((additive_checksum_bits & 1) as u8 as i32);
            additive_checksum_bits >>= 1;
            let entry: i32 = engine.state.password_nibbles_a(entry_offset);
            engine
                .state
                .set_password_nibbles_a(entry_offset, (entry << 1) | carry_bit);
        }
    }
    {
        let mut xor_checksum_bits: i32 = (engine.state.password_checksum_xor as i32);
        for entry_offset in (0..=14).rev().step_by(2) {
            let carry_bit: i32 = ((xor_checksum_bits & 1) as u8 as i32);
            xor_checksum_bits >>= 1;
            let entry: i32 = engine.state.password_nibbles_b(entry_offset);
            engine
                .state
                .set_password_nibbles_b(entry_offset, (entry << 1) | carry_bit);
        }
    }
    // Entries at offsets 0x0F and 0x1F seed the RNG and are intentionally
    // not scrambled.
    engine.state.rng_low = ((engine.state.password_nibbles_a(15)) as u8);
    engine.state.rng_high = ((engine.state.password_nibbles_b(15)) as u8);
    let mut scramble_offset: i32 = 14;
    while (scramble_offset >= 0) {
        engine.state.scratch0 = (scramble_offset as u8);
        r.value = 32;
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_nibbles_a(
            scramble_offset,
            ((r.value ^ ((engine.state.password_nibbles_a(scramble_offset)) as u8)) as u8 as i32),
        );

        r.value = 32;
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_nibbles_b(
            scramble_offset,
            ((r.value ^ ((engine.state.password_nibbles_b(scramble_offset)) as u8)) as u8 as i32),
        );

        scramble_offset -= 1;
    }
}

/// Validates and decodes the status/password item-list buffer back into the
/// saved progress/inventory snapshot. Carry is set and the error sound is
/// queued when either checksum fails.
pub fn decode_inventory_item_list_snapshot(engine: &mut Engine, r: &mut RoutineContext) {
    // Work in a copy so a bad checksum leaves the visible list untouched.
    for entry_offset in (0..=31).rev() {
        engine
            .state
            .set_password_scramble_a(entry_offset, engine.state.password_nibbles_a(entry_offset));
    }

    // Unscramble every non-seed entry with the same RNG stream used by the
    // encoder.
    engine.state.rng_low = ((engine.state.password_scramble_a(15)) as u8);
    engine.state.rng_high = ((engine.state.password_scramble_b(15)) as u8);
    let mut scramble_offset: i32 = 14;
    while (scramble_offset >= 0) {
        engine.state.scratch0 = (scramble_offset as u8);
        r.value = 32;
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_scramble_a(
            scramble_offset,
            engine.state.password_scramble_a(scramble_offset) ^ (r.value as i32),
        );

        r.value = 32;
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_scramble_b(
            scramble_offset,
            engine.state.password_scramble_b(scramble_offset) ^ (r.value as i32),
        );

        scramble_offset -= 1;
    }

    // Pull the stored checksum bits back out of the high-bit slots before
    // verifying the decoded entries.
    {
        let mut stored_xor_checksum: i32 = 0;
        for entry_offset in (0..=14).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_b(entry_offset);
            stored_xor_checksum = (((stored_xor_checksum >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_b(entry_offset, entry >> 1);
        }
        engine.state.password_checksum_xor = (stored_xor_checksum as u8);
    }
    {
        let mut stored_additive_checksum: i32 = 0;
        for entry_offset in (0..=14).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_a(entry_offset);
            stored_additive_checksum =
                (((stored_additive_checksum >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_a(entry_offset, entry >> 1);
        }
        engine.state.password_checksum_add = (stored_additive_checksum as u8);
    }

    // Verify additive and xor checksums before updating the snapshot
    // buffers.
    let mut additive_checksum: i32 = 0;
    for entry_offset in (0..=31).rev() {
        additive_checksum =
            ((additive_checksum + engine.state.password_scramble_a(entry_offset)) as u8 as i32);
    }
    if (additive_checksum != (engine.state.password_checksum_add as i32)) {
        engine.state.prompt_state = 28;
        engine.state.prompt_argument = 28;
        r.carry = 1;
        return;
    }

    let mut xor_checksum: i32 = 10;
    for entry_offset in (0..=31).rev() {
        xor_checksum =
            ((xor_checksum ^ engine.state.password_scramble_a(entry_offset)) as u8 as i32);
    }
    if (xor_checksum != (engine.state.password_checksum_xor as i32)) {
        engine.state.prompt_state = 28;
        engine.state.prompt_argument = 28;
        r.carry = 1;
        return;
    }

    // Decode key and coin counters from every other high-bit slot.
    {
        let mut key_bits: i32 = 0;
        for entry_offset in (0..=15).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_a(entry_offset);
            key_bits = (((key_bits >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_a(entry_offset, entry >> 1);
        }
        engine.state.set_save_inventory(16, key_bits);
    }
    {
        let mut coin_bits: i32 = 0;
        for entry_offset in (0..=15).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_b(entry_offset);
            coin_bits = (((coin_bits >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_b(entry_offset, entry >> 1);
        }
        engine.state.set_save_inventory(17, coin_bits);
    }

    // Recombine progress nibbles and copy inventory counts back to the
    // snapshot area.
    let mut progress_entry_offset: i32 = 15;
    for progress_byte_offset in (0..=7).rev() {
        let high_nibble: i32 = engine.state.password_scramble_a(progress_entry_offset);
        progress_entry_offset -= 1;
        let low_nibble: i32 = engine.state.password_scramble_a(progress_entry_offset);
        progress_entry_offset -= 1;
        engine
            .state
            .set_save_progress(progress_byte_offset, (high_nibble << 4) | low_nibble);
    }
    for inventory_offset in (0..=15).rev() {
        engine.state.set_save_inventory(
            inventory_offset,
            engine.state.password_scramble_b(inventory_offset),
        );
    }
    r.carry = 0;
}

/// Restores the title/menu working state from ROM defaults and blacks out
/// the palette buffer. Unlike the full boot RAM initializer, this only
/// rewrites `0x40..0x8B`, leaving broader runtime buffers intact.
pub fn reset_menu_state_and_palette(engine: &mut Engine, r: &mut RoutineContext) {
    for addr in 64..140 {
        engine
            .state
            .set_byte(addr, engine.state.byte(ZP_INIT_TABLE + addr));
    }
    for palette_offset in (0..=31).rev() {
        engine.state.set_palette_buffer(palette_offset, 15);
    }
    r.value = 15;
    r.index = 255;
}

/// Uploads the title-screen nametable image and title CHR bank shadows.
///
/// The source image occupies four consecutive 256-byte pages at
/// `0x9EC9..0xA1C8`; `0xA2E9/0xA2EA` provide the title CHR banks.
pub fn upload_title_screen_nametables(engine: &mut Engine, r: &mut RoutineContext) {
    let ctrl: i32 = (engine.state.ppu_ctrl_shadow as i32);
    let mask: i32 = (engine.state.ppu_mask_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        ctrl & crate::bits::CLEAR_BITS_2_7,
    );
    engine.state.statusbar_split_flag = 0;
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        mask & crate::bits::CLEAR_BITS_3_4,
    );
    engine.device_write(crate::engine::reg::PPU_ADDR, 32);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);

    for page in 0..4 {
        let source_page = PALETTE_SOURCE_BASE + page * 256;
        for offset in 0..256 {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.byte(((source_page + offset) as u16 as i32)),
            );
        }
    }

    engine
        .state
        .set_chr_bank(0, engine.state.byte(TITLE_CHR_BANK_TABLE));
    engine
        .state
        .set_chr_bank(1, engine.state.byte(TITLE_CHR_BANK_TABLE + 1));
    engine.state.ppu_mask_shadow = (mask as u8);
    engine.state.ppu_ctrl_shadow = (ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl);
    r.value = (ctrl as u8);
    r.index = 0;
}

/// Copies the title-screen palette from ROM into the palette upload buffer.
pub fn load_title_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    for palette_offset in (0..=31).rev() {
        engine.state.set_palette_buffer(
            palette_offset,
            engine.state.byte(TITLE_PALETTE_TABLE + palette_offset),
        );
    }
    r.index = 255;
}

/// Keeps the horizontal camera inside the playfield while the player moves.
///
/// `0x7B/0x7C` store the scroll position as sub-tile low bits plus tile X.
/// `0x7F` records which edge column must be uploaded when scrolling exposes
/// a new strip. Carry is set when no new scroll strip is needed.
pub fn update_camera_scroll_from_player(engine: &mut Engine, r: &mut RoutineContext) {
    let scroll_world_x: i32 = ((((engine.state.scroll_tile_x as i32) << 4)
        | (engine.state.scroll_fine_x as i32)) as u8 as i32);
    let player_world_x: i32 = ((((engine.state.player_x_tile as i32) << 4)
        | (engine.state.player_x_fine as i32)) as u8 as i32);
    let camera_delta: i32 = ((player_world_x - scroll_world_x) as u8 as i32);
    let mut no_scroll_column_needed: i32 = 0;
    engine.state.scratch0 = (scroll_world_x as u8);
    if (camera_delta < 96) {
        if ((engine.state.scroll_tile_x | engine.state.scroll_fine_x) == 0) {
            no_scroll_column_needed = 1;
        } else {
            let left_scroll_tile: i32 = ((engine.state.player_x_tile - 6) as u8 as i32);
            if (engine.state.player_x_tile < 6) {
                engine.state.scroll_fine_x = 0;
                engine.state.scroll_tile_x = 0;
                no_scroll_column_needed = 0;
            } else {
                engine.state.scroll_tile_x = (left_scroll_tile as u8);
                engine.state.scroll_fine_x = engine.state.player_x_fine;
                engine.state.camera_scroll_flag = 255;
                no_scroll_column_needed = 0;
            }
        }
    } else if (camera_delta < 145) {
        no_scroll_column_needed = 1;
    } else if (engine.state.scroll_tile_x >= 48) {
        engine.state.scroll_tile_x = 48;
        engine.state.scroll_fine_x = 0;
        no_scroll_column_needed = 1;
    } else {
        engine.state.scroll_tile_x = engine.state.player_x_tile - 9;
        engine.state.scroll_fine_x = engine.state.player_x_fine;
        engine.state.camera_scroll_flag = 1;
        no_scroll_column_needed = 0;
    }
    refresh_scroll_register_shadows(engine, r);
    r.carry = (no_scroll_column_needed as u8);
}

/// Converts the tile/sub-tile camera position into PPU scroll shadows.
///
/// `0x1C` is the fine X scroll byte used by the status split. `0x1D` is the
/// horizontal nametable bit that is folded into the PPUCTRL shadow at vblank.
pub fn refresh_scroll_register_shadows(engine: &mut Engine, r: &mut RoutineContext) {
    let scroll_tile_x: i32 = (engine.state.scroll_tile_x as i32);
    let scroll_fine_x: i32 = (engine.state.scroll_fine_x as i32);
    let scroll_pixel_x: i32 = (((scroll_tile_x << 4) | scroll_fine_x) as u8 as i32);
    let nametable_x_bit: i32 = (scroll_tile_x >> 4) & crate::bits::BIT0;

    engine.state.scroll_pixel_x = (scroll_pixel_x as u8);
    engine.state.nametable_select = (nametable_x_bit as u8);
    r.index = (scroll_pixel_x as u8);
    r.value = (nametable_x_bit as u8);
}

/// Projects the two 8x16 player sprites into OAM staging.
///
/// Player world X is stored in `0x43/0x44`; screen X subtracts the camera at
/// `0x7B/0x7C`. `0x85`/`0x84` drive the invulnerability blink hide phase.
pub fn draw_player_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.sprite_blink_timer != 0)
        && ((engine.state.frame_prescaler & ((crate::bits::BIT0) as u8)) == 0))
    {
        engine.state.set_oam_y(16, 239);
        engine.state.set_oam_y(20, 239);
        return;
    }

    let sprite_y: i32 = ((engine.state.player_y + 43) as u8 as i32);
    engine.state.set_oam_y(16, sprite_y);
    engine.state.set_oam_y(20, sprite_y);

    let camera_world_x: i32 = ((((engine.state.scroll_tile_x as i32) << 4)
        | (engine.state.scroll_fine_x as i32)) as u8 as i32);
    let player_world_x: i32 = ((((engine.state.player_x_tile as i32) << 4)
        | (engine.state.player_x_fine as i32)) as u8 as i32);
    let screen_x: i32 = ((player_world_x - camera_world_x) as u8 as i32);
    engine.state.scratch0 = (camera_world_x as u8);
    engine.state.set_oam_x(16, screen_x);
    engine.state.set_oam_x(20, screen_x + 8);
    engine
        .state
        .set_oam_attr(16, (engine.state.player_facing as i32));
    engine
        .state
        .set_oam_attr(20, (engine.state.player_facing as i32));

    let left_tile: i32 = (engine.state.player_pose as i32);
    if ((engine.state.player_facing & ((crate::bits::BIT6) as u8)) != 0) {
        engine.state.set_oam_tile(20, left_tile);
        engine.state.set_oam_tile(16, left_tile + 2);
    } else {
        engine.state.set_oam_tile(16, left_tile);
        engine.state.set_oam_tile(20, left_tile + 2);
    }
}

/// Draws the selected item cursor and the three equipped item icons in the
/// status area. High-bit item ids hide a slot.
pub fn draw_status_item_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    if (selected_slot >= 3) {
        engine.state.set_oam_y(56, 239);
        engine.state.set_oam_y(60, 239);
    } else {
        let cursor_x: i32 = (((selected_slot << 4) + 200) as u8 as i32);
        engine.state.set_oam_y(56, 19);
        engine.state.set_oam_y(60, 19);
        engine.state.set_oam_x(56, cursor_x);
        engine.state.set_oam_x(60, cursor_x + 8);
        engine.state.set_oam_tile(56, 255);
        engine.state.set_oam_tile(60, 255);
        engine.state.set_oam_attr(56, 1);
        engine.state.set_oam_attr(60, 65);
    }

    for item_slot in (0..=2).rev() {
        let oam_offset: i32 = item_slot << 3;
        let item_id: i32 = engine.state.item_slot(item_slot);
        let sprite_y: i32 = if ((item_id & crate::bits::BIT7) != 0) {
            239
        } else {
            let left_tile: i32 = (((item_id << 2) + 161) as u8 as i32);
            let left_x: i32 = (((oam_offset << 1) + 200) as u8 as i32);
            engine.state.set_oam_tile(32 + oam_offset, left_tile);
            engine.state.set_oam_tile(36 + oam_offset, left_tile + 2);
            engine.state.set_oam_x(32 + oam_offset, left_x);
            engine.state.set_oam_x(36 + oam_offset, left_x + 8);
            engine.state.set_oam_attr(32 + oam_offset, 1);
            engine.state.set_oam_attr(36 + oam_offset, 1);
            19
        };
        engine.state.set_oam_y(32 + oam_offset, sprite_y);
        engine.state.set_oam_y(36 + oam_offset, sprite_y);
    }
}

/// Projects up to 16 room object slots into two-sprite OAM entries.
///
/// `0x3F/0x3E` carry the OAM/object cursors between calls, matching the
/// original scheduler's rolling object sprite pass.
pub fn draw_room_object_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch2 = 16;
    let mut oam_offset: i32 = (engine.state.oam_cursor as i32);
    let mut object_offset: i32 = (engine.state.sprite_index as i32);
    loop {
        r.index = (oam_offset as u8);
        r.offset = (object_offset as u8);
        draw_object_slot_sprites(engine, r);
        oam_offset = ((((oam_offset + 8) as u8 as i32) | crate::bits::BIT7) as u8 as i32);
        object_offset = ((object_offset + 48) as u8 as i32);
        engine.state.scratch2 = engine.state.scratch2 - 1;
        if (engine.state.scratch2 == 0) {
            break;
        }
    }
    engine.state.oam_cursor = ((((oam_offset + 56) as u8 as i32) | crate::bits::BIT7) as u8);
    engine.state.sprite_index = ((object_offset + 16) as u8);
}

/// Draws one 16-byte room object slot into a two-sprite OAM entry.
///
/// Inactive slots, vertically out-of-range objects, and objects scrolled out
/// of the visible horizontal window hide both sprites. When the left sprite
/// is visible but the right sprite would wrap beyond `0xEF`, only the right
/// half is hidden.
pub fn draw_object_slot_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let oam_offset: i32 = (r.index as u8 as i32);
    let object_offset: i32 = (r.offset as u8 as i32);
    let object_base: i32 = OBJECT_TABLE_BASE + object_offset;

    if (engine.state.byte(object_base + 1) == 0) || (engine.state.byte(object_base + 14) >= 191) {
        engine.state.set_oam_y(oam_offset, 239);
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    let attributes: i32 = engine.state.byte(object_base + 2);
    engine.state.set_oam_attr(oam_offset, attributes);
    engine.state.set_oam_attr(4 + oam_offset, attributes);

    let left_tile: i32 = engine.state.byte(object_base);
    if ((attributes & crate::bits::BIT6) != 0) {
        engine.state.set_oam_tile(4 + oam_offset, left_tile);
        engine.state.set_oam_tile(oam_offset, left_tile + 2);
    } else {
        engine.state.set_oam_tile(oam_offset, left_tile);
        engine.state.set_oam_tile(4 + oam_offset, left_tile + 2);
    }

    let subtile_delta: i32 = ((engine.state.byte(object_base + 12)) as u16 as i32) + 256
        - (engine.state.scroll_fine_x as i32);
    let fine_x: i32 = (subtile_delta as u8 as i32) & crate::bits::LOW_NIBBLE;
    let tile_borrow: i32 = ((subtile_delta >> 8) as u8 as i32);
    let tile_delta: i32 = ((((engine.state.byte(object_base + 13)) as u16 as i32) + tile_borrow
        - (engine.state.scroll_tile_x as i32)
        - 1) as u8 as i32);
    if (tile_delta >= 16) {
        engine.state.set_oam_y(oam_offset, 239);
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    let mut screen_x: i32 = (((tile_delta << 4) | fine_x) as u8 as i32);
    engine.state.scratch0 = (screen_x as u8);

    if (engine.state.byte(object_base + 1) == 1) && (engine.state.byte(object_base + 15) != 0) {
        screen_x = ((screen_x + engine.state.byte(object_base + 15)) as u8 as i32);
        engine.state.scratch0 = (screen_x as u8);
        engine.state.set_byte(object_base + 15, 0);
    }

    let sprite_y: i32 = ((engine.state.byte(object_base + 14) + 43) as u8 as i32);
    engine.state.set_oam_x(oam_offset, screen_x);
    engine.state.set_oam_y(oam_offset, sprite_y);
    if (screen_x >= 239) {
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    engine.state.set_oam_x(4 + oam_offset, screen_x + 8);
    engine.state.set_oam_y(4 + oam_offset, sprite_y);
}

/// Clears staged OAM while preserving the sprite-zero template.
///
/// The first sprite is copied from `0xFF6B..0xFF6E`; every remaining OAM
/// byte is set to `0xF8`, the offscreen clear value used by the startup and
/// title flows.
pub fn clear_oam_with_sprite_zero_template(engine: &mut Engine, r: &mut RoutineContext) {
    for template_offset in 0..=3 {
        engine.state.set_oam_y(
            template_offset,
            engine
                .state
                .byte(((SPRITE_Y_TABLE_F + template_offset) as u16 as i32)),
        );
    }
    for oam_offset in 4..=255 {
        engine.state.set_oam_y(oam_offset, 248);
    }
    r.index = 0;
}

/// Clears the visible nametables to blank tile `0xC0` with zero attributes.
///
/// Rendering is disabled around the direct PPU writes and the PPUCTRL/PPUMASK
/// shadows are restored before returning.
pub fn clear_name_tables_to_blank_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let ctrl: i32 = (engine.state.ppu_ctrl_shadow as i32);
    let mask: i32 = (engine.state.ppu_mask_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        ctrl & crate::bits::CLEAR_BITS_2_7,
    );
    engine.state.statusbar_split_flag = 0;
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        mask & crate::bits::CLEAR_BITS_3_4,
    );
    engine.device_write(crate::engine::reg::PPU_ADDR, 32);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);

    for _ in 0..2 {
        for _ in 0..(5 * 192) {
            engine.device_write(crate::engine::reg::PPU_DATA, 192);
        }
        for _ in 0..64 {
            engine.device_write(crate::engine::reg::PPU_DATA, 0);
        }
    }
    engine.state.ppu_mask_shadow = (mask as u8);
    engine.state.ppu_ctrl_shadow = (ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl);
    r.value = (ctrl as u8);
    r.index = 0;
    r.offset = 0;
}

/// Dims `r.offset` bytes in the palette buffer starting at `0x0180 +
/// r.index` by subtracting the high-nibble step in `0x09`.
pub fn dim_palette_range_by_step(engine: &mut Engine, r: &mut RoutineContext) {
    let mut palette_offset: i32 = (r.index as u8 as i32);
    let mut remaining: i32 = (r.offset as u8 as i32);
    loop {
        let color = engine.state.palette_buffer(palette_offset);
        let low_nibble: i32 = color & crate::bits::LOW_NIBBLE;
        engine.state.scratch0 = (low_nibble as u8);
        let high_nibble: i32 = color & crate::bits::HIGH_NIBBLE;
        let fade_step: i32 = (engine.state.scratch1 as i32);
        let dimmed_color: i32 = if (high_nibble >= fade_step) {
            ((((high_nibble - fade_step) as u8 as i32) | low_nibble) as u8 as i32)
        } else {
            15
        };
        engine
            .state
            .set_palette_buffer(palette_offset, dimmed_color);
        palette_offset += 1;
        remaining -= 1;
        if (remaining == 0) {
            break;
        }
    }
    r.index = (palette_offset as u8);
    r.offset = (remaining as u8);
}

/// Queues a PPU upload of the palette buffer to `$3F00`.
pub fn upload_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    clear_pending_vram_job(engine, r);
    engine.state.vram_addr_lo = 0;
    engine.state.vram_addr_hi = 63;
    r.value = 2;
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the fixed status-panel nametable template and clears its
/// attribute bytes.
pub fn upload_status_panel_template(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_ctrl: i32 = 0;
    let mut saved_mask: i32 = 0;
    let mut i: i32 = 0;
    clear_pending_vram_job(engine, r);
    saved_ctrl = (engine.state.ppu_ctrl_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        saved_ctrl & crate::bits::CLEAR_BITS_2_7,
    );
    engine.state.statusbar_split_flag = 0;
    saved_mask = (engine.state.ppu_mask_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        saved_mask & crate::bits::CLEAR_BITS_3_4,
    );
    engine.device_write(crate::engine::reg::PPU_ADDR, 35);
    engine.device_write(crate::engine::reg::PPU_ADDR, 32);
    {
        i = 0;
        while (i < 160) {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.byte(((HUD_TEMPLATE_TABLE + i) as u16 as i32)),
            );
            {
                i += 1;
                i
            };
        }
    }
    engine.device_write(crate::engine::reg::PPU_ADDR, 35);
    engine.device_write(crate::engine::reg::PPU_ADDR, 240);
    {
        i = 0;
        while (i < 16) {
            engine.device_write(crate::engine::reg::PPU_DATA, 0);
            {
                i += 1;
                i
            };
        }
    }
    engine.state.statusbar_split_flag =
        (engine.state.statusbar_split_flag + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.ppu_mask_shadow = (saved_mask as u8);
    engine.state.ppu_ctrl_shadow = (saved_ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, saved_ctrl);
    r.value = (saved_ctrl as u8);
    r.offset = 0;
}

/// Resolves the current scroll column and uploads the full room view.
pub fn upload_current_room_view(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_lo = engine.state.scroll_tile_x & ((crate::bits::CLEAR_BIT0) as u8);
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    upload_room_view_from_tile_pointer(engine, r);
}

/// Uploads the full room view from the staged room tile pages.
pub fn upload_staged_room_view(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_lo = engine.state.scroll_tile_x & ((crate::bits::CLEAR_BIT0) as u8);
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    engine.state.data_ptr_hi = (engine.state.data_ptr_hi - 5) + engine.state.room_metadef_hi;
    upload_room_view_from_tile_pointer(engine, r);
}

/// Uploads room tiles and attributes from the tile pointer in `0x0C/0x0D`.
pub fn upload_room_view_from_tile_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    let mut ctrl_save: i32 = (engine.state.ppu_ctrl_shadow as i32);
    let mut v29_save: i32 = (engine.state.statusbar_split_flag as i32);
    let mut v24_save: i32 = (engine.state.ppu_mask_shadow as i32);
    let mut c0c_save: i32 = (engine.state.data_ptr_lo as i32);
    let mut c0d_save: i32 = (engine.state.data_ptr_hi as i32);
    let mut p0C: i32 = 0;
    let mut p79: i32 = 0;
    let mut outer: i32 = 0;
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        (ctrl_save & crate::bits::LOW_7_BITS) | crate::bits::BIT2,
    );
    engine.state.statusbar_split_flag = 0;
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        v24_save & crate::bits::CLEAR_BITS_3_4,
    );
    p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
    {
        let mut sx: i32 = (engine.state.scroll_tile_x as i32);
        let mut lo: i32 = (((sx << 1) & crate::bits::BITS_2_3_4) as u8 as i32);
        let mut hi: i32 = (((sx & crate::bits::BIT4) >> 2) as u8 as i32);
        let mut t: i32 = ((0 + lo) as u16 as i32);
        engine.state.vram_addr_lo = (t as u8);
        engine.state.vram_addr_hi = ((32 + hi + (t >> 8)) as u8);
    }
    engine.state.scratch2 = 18;
    p0C = ((c0c_save | (c0d_save << 8)) as u16 as i32);
    {
        outer = 0;
        while (outer < 18) {
            let mut inner: i32 = 0;
            engine.state.scratch3 = 12;
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_hi as i32),
            );
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_lo as i32),
            );
            engine.state.scratch0 = 0;
            loop {
                let mut idx: i32 = engine
                    .state
                    .byte(((p0C + (engine.state.scratch0 as i32)) as u16 as i32));
                let mut y: i32 = ((idx << 2) as u8 as i32);
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine.state.byte(((p79 + y) as u16 as i32)),
                );
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine
                        .state
                        .byte(((p79 + ((y + 1) as u8 as i32)) as u16 as i32)),
                );
                engine.state.scratch0 =
                    (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
                engine.state.scratch3 =
                    (engine.state.scratch3 - 1) & ((crate::bits::BYTE_MASK) as u8);
                if (engine.state.scratch3 == 0) {
                    break;
                }
            }
            engine.state.scratch3 = 12;
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_hi as i32),
            );
            inner = ((engine.state.vram_addr_lo + 1) as u8 as i32);
            engine.device_write(crate::engine::reg::PPU_ADDR, inner);
            engine.state.scratch0 = 0;
            loop {
                let mut idx: i32 = engine
                    .state
                    .byte(((p0C + (engine.state.scratch0 as i32)) as u16 as i32));
                let mut y: i32 = (((idx << 2) + 2) as u8 as i32);
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine.state.byte(((p79 + y) as u16 as i32)),
                );
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine
                        .state
                        .byte(((p79 + ((y + 1) as u8 as i32)) as u16 as i32)),
                );
                engine.state.scratch0 =
                    (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
                engine.state.scratch3 =
                    (engine.state.scratch3 - 1) & ((crate::bits::BYTE_MASK) as u8);
                if (engine.state.scratch3 == 0) {
                    break;
                }
            }
            engine.state.vram_addr_lo =
                (engine.state.vram_addr_lo + 2) & ((crate::bits::BYTE_MASK) as u8);
            if ((engine.state.vram_addr_lo & ((crate::bits::BIT5) as u8)) != 0) {
                engine.state.vram_addr_lo = 0;
                engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
            }
            {
                let mut t: i32 = ((12 + engine.state.data_ptr_lo) as u16 as i32);
                engine.state.data_ptr_lo = (t as u8);
                engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((t >> 8) as u8);
                p0C = ((engine.state.data_ptr()) as u16 as i32);
            }
            engine.state.scratch2 = (engine.state.scratch2 - 1) & ((crate::bits::BYTE_MASK) as u8);
            {
                let __old = outer;
                outer += 1;
                __old
            };
        }
    }
    engine.state.data_ptr_hi = (c0d_save as u8);
    engine.state.data_ptr_lo = (c0c_save as u8);
    p0C = ((c0c_save | (c0d_save << 8)) as u16 as i32);
    {
        let mut sx: i32 = (engine.state.scroll_tile_x as i32);
        let mut lo: i32 = (((sx >> 1) & crate::bits::LOW_3_BITS) as u8 as i32);
        let mut hi: i32 = (((sx & crate::bits::BIT4) >> 2) as u8 as i32);
        let mut t: i32 = ((192 + lo) as u16 as i32);
        engine.state.vram_addr_lo = (t as u8);
        engine.state.vram_addr_hi = ((35 + hi + (t >> 8)) as u8);
    }
    engine.state.scratch2 = 9;
    loop {
        let mut x: i32 = 0;
        {
            x = 6;
            while (x > 0) {
                let mut a: i32 = 0;
                a = engine.state.byte(((p0C + 13) as u16 as i32));
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                a = engine.state.byte(((p0C + 1) as u16 as i32));
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                a = engine.state.byte(((p0C + 12) as u16 as i32));
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                a = engine.state.byte(((p0C + 0) as u16 as i32));
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                {
                    let mut c1: i32 = (a >> 7) & 1;
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                engine.device_write(
                    crate::engine::reg::PPU_ADDR,
                    (engine.state.vram_addr_hi as i32),
                );
                engine.device_write(
                    crate::engine::reg::PPU_ADDR,
                    (engine.state.vram_addr_lo as i32),
                );
                engine.device_write(crate::engine::reg::PPU_DATA, (engine.state.scratch0 as i32));
                {
                    let mut t: i32 = ((2 + engine.state.data_ptr_lo) as u16 as i32);
                    engine.state.data_ptr_lo = (t as u8);
                    engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((t >> 8) as u8);
                }
                {
                    let mut t: i32 = ((8 + engine.state.vram_addr_lo) as u16 as i32);
                    engine.state.vram_addr_lo = (t as u8);
                    engine.state.vram_addr_hi = engine.state.vram_addr_hi + ((t >> 8) as u8);
                }
                p0C = ((engine.state.data_ptr()) as u16 as i32);
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            let mut t: i32 = ((12 + engine.state.data_ptr_lo) as u16 as i32);
            engine.state.data_ptr_lo = (t as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((t >> 8) as u8);
        }
        {
            let mut t: i32 = ((209 + engine.state.vram_addr_lo) as u16 as i32);
            engine.state.vram_addr_lo = (t as u8);
            engine.state.vram_addr_hi = engine.state.vram_addr_hi + 255 + ((t >> 8) as u8);
        }
        p0C = ((engine.state.data_ptr()) as u16 as i32);
        if ((engine.state.vram_addr_lo & ((crate::bits::BIT3) as u8)) != 0) {
            engine.state.vram_addr_lo = 192;
            engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
        }
        engine.state.scratch2 = (engine.state.scratch2 - 1) & ((crate::bits::BYTE_MASK) as u8);
        if (engine.state.scratch2 == 0) {
            break;
        }
    }
    engine.state.ppu_mask_shadow = (v24_save as u8);
    engine.state.statusbar_split_flag = (v29_save as u8);
    engine.state.ppu_ctrl_shadow = (ctrl_save as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl_save);
    r.value = (ctrl_save as u8);
    r.index = 0;
}

/// Uploads the 16 visible room columns using the bank-9 room-column builder.
pub fn upload_room_columns_from_bank9(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sx: i32 = 0;
    clear_pending_vram_job(engine, r);
    sx = (engine.state.scroll_tile_x as i32);
    engine.state.vram_addr_lo = (((sx << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((sx & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
    engine.state.scratch0 = (sx as u8);
    engine.state.scratch1 = 16;
    loop {
        engine.state.data_ptr_lo = engine.state.scratch0;
        farcall_bank_09_r7(engine, r);
        engine.state.vram_addr_lo = engine.state.vram_addr_lo + 2;
        if ((engine.state.vram_addr_lo & ((crate::bits::BIT5) as u8)) != 0) {
            engine.state.vram_addr_lo = 0;
            engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
        }
        engine.state.scratch0 = engine.state.scratch0 + 1;
        engine.state.scratch1 = engine.state.scratch1 - 1;
        if (engine.state.scratch1 == 0) {
            break;
        }
    }
}

/// Uploads the 16 visible room columns using the current staged room data.
pub fn upload_staged_room_columns(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sx: i32 = 0;
    clear_pending_vram_job(engine, r);
    sx = (engine.state.scroll_tile_x as i32);
    engine.state.vram_addr_lo = (((sx << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((sx & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
    engine.state.scratch0 = (sx as u8);
    engine.state.scratch1 = 16;
    loop {
        engine.state.data_ptr_lo = engine.state.scratch0;
        build_staged_room_column(engine, r);
        engine.state.vram_addr_lo = engine.state.vram_addr_lo + 2;
        if ((engine.state.vram_addr_lo & ((crate::bits::BIT5) as u8)) != 0) {
            engine.state.vram_addr_lo = 0;
            engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
        }
        engine.state.scratch0 = engine.state.scratch0 + 1;
        engine.state.scratch1 = engine.state.scratch1 - 1;
        if (engine.state.scratch1 == 0) {
            break;
        }
    }
}

/// Uploads the room column that is about to scroll into view.
pub fn upload_scroll_edge_room_column(engine: &mut Engine, r: &mut RoutineContext) {
    let mut col: i32 = 0;
    clear_pending_vram_job(engine, r);
    if ((engine.state.camera_scroll_flag & ((crate::bits::BIT7) as u8)) != 0) {
        col = (engine.state.scroll_tile_x as i32);
    } else {
        col = ((engine.state.scroll_tile_x + 16) as u8 as i32);
    }
    engine.state.data_ptr_lo = (col as u8);
    engine.state.vram_addr_lo = (((col << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((col & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
    farcall_bank_09_r7(engine, r);
}

/// Builds one staged room column from the current room tile pointer and
/// tileset metadata.
pub fn build_staged_room_column(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    engine.state.data_ptr_hi = ((((engine.state.data_ptr_hi - 5) as u8 as i32)
        + (engine.state.room_metadef_hi as i32)) as u8);
    queue_room_column_vram_upload(engine, r);
}

/// Selects the room data bank/pointers, derives room metadata, and builds
/// the palette buffer for the active room.
pub fn prepare_room_metadata_and_palette(engine: &mut Engine, r: &mut RoutineContext) {
    select_room_data_bank_and_pointers(engine, r);
    text_attr_build(engine, r);
    build_room_palette_buffer(engine, r);
}

/// Copies three room tile pages from the active room data pointer into
/// `0x0500..0x07FF`.
pub fn copy_room_tile_pages(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.palette_src_ptr_lo = engine.state.room_metadef_lo;
    engine.state.palette_src_ptr_hi = engine.state.room_metadef_hi;

    let source_lo: i32 = (engine.state.palette_src_ptr_lo as i32);
    let mut source_hi: i32 = (engine.state.palette_src_ptr_hi as i32);
    for page_index in 0..=2 {
        let source_ptr: i32 = ((source_lo | (source_hi << 8)) as u16 as i32);
        let dest_base: i32 = ROOM_BUFFER_BASE + (page_index << 8);
        for page_offset in 0..256 {
            engine.state.set_byte(
                dest_base + page_offset,
                engine
                    .state
                    .byte(((source_ptr + page_offset) as u16 as i32)),
            );
        }
        source_hi += 1;
        engine.state.palette_src_ptr_hi = (source_hi as u8);
    }
    r.offset = 0;
}

/// Selects the PRG bank and base room data pointers for `0x47/0x48`.
pub fn select_room_data_bank_and_pointers(engine: &mut Engine, r: &mut RoutineContext) {
    let room_bank: i32 = ((engine.state.map_screen_y >> 1) as u8 as i32);
    if (room_bank != (engine.state.prg_bank_8000 as i32)) {
        engine.state.prg_bank_8000 = (room_bank as u8);
        r.value = 255;
        queue_ppu_job_and_wait(engine, r);
    }

    let room_table_offset: i32 = ((((((engine.state.map_screen_y & ((crate::bits::BIT0) as u8))
        << 2) as u8 as i32)
        | (engine.state.map_screen_x as i32))
        << 2) as u8 as i32);
    let room_ptr_lo: i32 = ((room_table_offset + 128) as u8 as i32);
    engine.state.room_metadef_hi = (room_ptr_lo as u8);
    engine.state.palette_src_ptr_hi = ((room_ptr_lo + 3) as u8);
    engine.state.palette_src_ptr_lo = 0;
    engine.state.room_metadef_lo = 0;
    r.carry = ((if ((room_ptr_lo + 3) > 255) { 1 } else { 0 }) as u8);
}

/// Copies room palette/attribute bytes into the palette buffer and applies
/// the active family-member palette when applicable.
pub fn build_room_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    let room_palette_ptr: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    for room_palette_offset in 224..=255 {
        engine.state.set_inventory_item(
            64 + room_palette_offset,
            engine
                .state
                .byte(((room_palette_ptr + room_palette_offset) as u16 as i32)),
        );
    }

    let family_member: i32 = (engine.state.character_index as i32);
    if (family_member >= 6) {
        r.value = (family_member as u8);
        r.carry = 1;
        return;
    }

    let family_palette_end_offset: i32 = (((family_member << 2) + 3) as u8 as i32);
    let mut family_palette_offset: i32 = family_palette_end_offset;
    for dest_offset in (0..=3).rev() {
        engine.state.set_vram_stage(
            80 + dest_offset,
            engine
                .state
                .byte(FAMILY_PALETTE_TABLE + family_palette_offset),
        );
        family_palette_offset -= 1;
    }

    r.value = (family_palette_end_offset as u8);
    r.index = (family_palette_offset as u8);
    r.offset = ((255) as u8);
    r.carry = 0;
}

/// Reads the persistent room-progress bit for the current map coordinates.
pub fn read_room_persistent_flag(engine: &mut Engine, r: &mut RoutineContext) {
    let map_y: i32 = (engine.state.map_screen_y as i32);
    let map_x: i32 = (engine.state.map_screen_x as i32);
    let flag_byte_index: i32 = ((((map_y << 2) & crate::bits::BIT2) | map_x) as u8 as i32);
    let mut shifted_flags: i32 = engine.state.save_payload(flag_byte_index);
    let shift_count: i32 = (((map_y >> 1) + 1) as u8 as i32);
    for _ in 0..shift_count {
        shifted_flags = ((shifted_flags << 1) as u8 as i32);
    }
    r.value = (shifted_flags as u8);
}

/// Clears the persistent room-progress bit for the current map coordinates.
pub fn clear_room_persistent_flag(engine: &mut Engine, r: &mut RoutineContext) {
    let map_y: i32 = (engine.state.map_screen_y as i32);
    let shift_count: i32 = (((map_y >> 1) + 1) as u8 as i32);
    let clear_mask: i32 = ((255 ^ (128 >> (shift_count - 1))) as u8 as i32);
    let flag_byte_index: i32 = (((((map_y << 2) as u8 as i32) & crate::bits::BIT2)
        | (engine.state.map_screen_x as i32)) as u8 as i32);
    engine.state.set_save_payload(
        flag_byte_index,
        engine.state.save_payload(flag_byte_index) & clear_mask,
    );
    r.value = ((engine.state.save_payload(flag_byte_index)) as u8);
    r.index = (flag_byte_index as u8);
}

/// Converts tile coordinates in `0x0C/0x0D` into the current room tile
/// pointer. `0x10/0x11` receives the same offset plus the room base pointer.
pub fn resolve_room_tile_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_y: i32 = (engine.state.data_ptr_hi as i32);
    scale_room_tile_column(engine, r);
    engine.state.aux_ptr_hi = engine.state.data_ptr_hi;
    {
        let tile_row: i32 = ((tile_y >> 4) as u8 as i32);
        let room_offset: i32 = ((tile_row + (engine.state.data_ptr_lo as i32)) as u16 as i32);
        engine.state.data_ptr_lo = (room_offset as u8);
        engine.state.tile_fetch_counter = (room_offset as u8);
        if ((room_offset & crate::bits::BIT8) != 0) {
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + 1;
            engine.state.aux_ptr_hi = engine.state.aux_ptr_hi + 1;
        }
    }
    engine.state.data_ptr_hi = engine.state.data_ptr_hi + 5;
    {
        let room_ptr_lo: i32 =
            ((engine.state.tile_fetch_counter + engine.state.room_metadef_lo) as u16 as i32);
        let carry: i32 = ((room_ptr_lo >> 8) as u8 as i32);
        engine.state.tile_fetch_counter = (room_ptr_lo as u8);
        engine.state.aux_ptr_hi =
            engine.state.aux_ptr_hi + engine.state.room_metadef_hi + (carry as u8);
    }
}

/// Multiplies the tile column in `0x0C` by the room-data stride of 12.
pub fn scale_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
    let column_times_four: i32 = (((engine.state.data_ptr_lo as i32) << 2) as u16 as i32);
    let column_times_eight: i32 = (((engine.state.data_ptr_lo as i32) << 3) as u16 as i32);
    let column_offset: i32 = ((column_times_four + column_times_eight) as u16 as i32);
    engine.state.data_ptr_lo = (column_offset as u8);
    engine.state.data_ptr_hi = ((column_offset >> 8) as u8);
    r.index = ((column_times_four >> 8) as u8);
    r.offset = (column_times_four as u8);
    r.value = ((column_offset >> 8) as u8);
}

/// Queues the resource HUD VRAM upload after resource counters changed.
pub fn upload_resource_hud(engine: &mut Engine, r: &mut RoutineContext) {
    clear_pending_vram_job(engine, r);
    engine.state.vram_addr_lo = 96;
    engine.state.vram_addr_hi = 35;
    r.value = 4;
    queue_ppu_job_and_wait(engine, r);
}

/// Clamps the health counter and queues the health HUD digits for redraw.
pub fn sync_health_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.player_health as i32;
    if (health >= 109) {
        health = 109;
    }
    engine.state.player_health = health as u8;
    engine.state.scratch0 = (health as u8);
    r.value = (health as u8);
    r.index = 0;
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the magic counter and queues the magic HUD digits for redraw.
pub fn sync_magic_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut magic: i32 = engine.state.player_magic as i32;
    if (magic >= 109) {
        magic = 109;
    }
    engine.state.player_magic = magic as u8;
    engine.state.scratch0 = (magic as u8);
    r.value = (magic as u8);
    r.index = 6;
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the key counter and queues the key HUD digits for redraw.
pub fn sync_key_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut keys: i32 = (engine.state.keys as i32);
    if (keys >= 109) {
        keys = 109;
    }
    engine.state.keys = (keys as u8);
    engine.state.scratch0 = (keys as u8);
    r.value = (keys as u8);
    r.index = 12;
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the coin counter and queues the coin HUD digits for redraw.
pub fn sync_coin_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut coins: i32 = (engine.state.coins as i32);
    if (coins >= 109) {
        coins = 109;
    }
    engine.state.coins = (coins as u8);
    engine.state.scratch0 = (coins as u8);
    r.value = (coins as u8);
    r.index = 18;
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Builds the two-row status resource meter in the VRAM staging buffers.
/// `r.index` selects the meter column and `0x08` contains the resource value.
pub fn build_status_resource_meter_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let base_slot: i32 = (r.index as i32);
    engine.state.set_palette_buffer(123, base_slot);
    for tile_offset in 0..5 {
        engine
            .state
            .set_inventory_item(161 + base_slot + tile_offset, 220);
    }

    let base_slot: i32 = engine.state.palette_buffer(123);
    engine.state.set_palette_buffer(123, base_slot);
    for tile_offset in 0..5 {
        engine
            .state
            .set_inventory_item(193 + base_slot + tile_offset, 223);
    }

    let base_slot: i32 = engine.state.palette_buffer(123);
    r.index = (base_slot as u8);
    split_meter_value(engine, r);

    let mut filled_blocks: i32 = (r.offset as i32);
    let mut tile_slot: i32 = base_slot;
    loop {
        filled_blocks = ((filled_blocks - 1) as u8 as i32);
        if (filled_blocks == 0) {
            break;
        }
        engine.state.set_byte(
            ((STACK_SCRATCH + 1 + tile_slot) as u16 as i32),
            (engine
                .state
                .byte(((STACK_SCRATCH + 1 + tile_slot) as u16 as i32))
                - 1)
                & crate::bits::BYTE_MASK,
        );
        filled_blocks = ((filled_blocks - 1) as u8 as i32);
        if (filled_blocks == 0) {
            break;
        }
        engine.state.set_byte(
            ((STACK_SCRATCH + 1 + tile_slot) as u16 as i32),
            (engine
                .state
                .byte(((STACK_SCRATCH + 1 + tile_slot) as u16 as i32))
                - 1)
                & crate::bits::BYTE_MASK,
        );
        tile_slot = ((tile_slot + 1) as u8 as i32);
    }

    tile_slot = base_slot;
    let mut partial_blocks: i32 = (engine.state.scratch0 as i32);
    loop {
        partial_blocks = ((partial_blocks - 1) as u8 as i32);
        if (partial_blocks == 0) {
            break;
        }
        engine.state.set_byte(
            ((STACK_SCRATCH + 33 + tile_slot) as u16 as i32),
            (engine
                .state
                .byte(((STACK_SCRATCH + 33 + tile_slot) as u16 as i32))
                - 1)
                & crate::bits::BYTE_MASK,
        );
        partial_blocks = ((partial_blocks - 1) as u8 as i32);
        if (partial_blocks == 0) {
            break;
        }
        engine.state.set_byte(
            ((STACK_SCRATCH + 33 + tile_slot) as u16 as i32),
            (engine
                .state
                .byte(((STACK_SCRATCH + 33 + tile_slot) as u16 as i32))
                - 1)
                & crate::bits::BYTE_MASK,
        );
        tile_slot = ((tile_slot + 1) as u8 as i32);
    }
    r.offset = (partial_blocks as u8);
    r.index = (tile_slot as u8);
    r.value = (base_slot as u8);
}

/// Builds an object health meter using the alternate `0xA5/0xAB` sprite
/// tile pair.
pub fn build_object_health_meter_alt_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.object_health(0);
    if (health >= 109) {
        health = 109;
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 0;
    r.index = 165;
    r.offset = 171;
    build_health_meter_sprites(engine, r);
}

/// Builds an object health meter using the standard `0x65/0x6B` sprite
/// tile pair.
pub fn build_object_health_meter_standard_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.object_health(0);
    if (health >= 109) {
        health = 109;
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 0;
    let full_tile: i32 = 101;
    let empty_tile: i32 = 107;
    let mut sprite_slot: i32 = (engine.state.scratch1 as i32);
    engine.state.set_oam_tile(88 + sprite_slot, full_tile);
    engine.state.set_oam_tile(92 + sprite_slot, full_tile);
    engine.state.set_oam_tile(96 + sprite_slot, full_tile);
    engine.state.set_oam_tile(100 + sprite_slot, full_tile);
    engine.state.set_oam_tile(104 + sprite_slot, full_tile);
    engine.state.set_oam_tile(108 + sprite_slot, empty_tile);
    engine.state.set_oam_tile(112 + sprite_slot, empty_tile);
    engine.state.set_oam_tile(116 + sprite_slot, empty_tile);
    engine.state.set_oam_tile(120 + sprite_slot, empty_tile);
    engine.state.set_oam_tile(124 + sprite_slot, empty_tile);
    split_meter_value(engine, r);
    let mut filled_blocks: i32 = (r.offset as i32);
    sprite_slot = ((engine.state.scratch1 + 24) as u8 as i32);
    loop {
        filled_blocks = ((filled_blocks - 1) as u8 as i32);
        if (filled_blocks == 0) {
            break;
        }
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        filled_blocks = ((filled_blocks - 1) as u8 as i32);
        if (filled_blocks == 0) {
            break;
        }
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        sprite_slot = ((sprite_slot + 4) as u8 as i32);
    }

    sprite_slot = ((engine.state.scratch1 + 44) as u8 as i32);
    let mut partial_blocks: i32 = (engine.state.scratch0 as i32);
    loop {
        partial_blocks = ((partial_blocks - 1) as u8 as i32);
        if (partial_blocks == 0) {
            break;
        }
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        partial_blocks = ((partial_blocks - 1) as u8 as i32);
        if (partial_blocks == 0) {
            break;
        }
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        engine.state.set_oam_tile(
            64 + sprite_slot,
            (engine.state.oam_tile(64 + sprite_slot) - 1) & crate::bits::BYTE_MASK,
        );
        sprite_slot = ((sprite_slot + 4) as u8 as i32);
    }
    r.value = (full_tile as u8);
    r.index = (sprite_slot as u8);
    r.offset = (partial_blocks as u8);
}

/// Builds the player health meter sprite strip at the second OAM meter
/// slot.
pub fn build_player_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.player_health as i32;
    if (health >= 109) {
        health = 109;
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 128;
    r.index = 101;
    r.offset = 107;
    build_health_meter_sprites(engine, r);
}

/// Builds a ten-sprite two-row health meter. `0x09` selects the OAM slot,
/// `r.index` is the full tile, `r.offset` is the empty tile, and `0x08`
/// contains the value.
pub fn build_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let sprite_slot: i32 = (engine.state.scratch1 as i32);
    let full_tile: i32 = (r.index as u8 as i32);
    engine.state.set_oam_tile(88 + sprite_slot, full_tile);
    engine.state.set_oam_tile(92 + sprite_slot, full_tile);
    engine.state.set_oam_tile(96 + sprite_slot, full_tile);
    engine.state.set_oam_tile(100 + sprite_slot, full_tile);
    engine.state.set_oam_tile(104 + sprite_slot, full_tile);
    {
        let empty_tile: i32 = (r.offset as u8 as i32);
        engine.state.set_oam_tile(108 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(112 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(116 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(120 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(124 + sprite_slot, empty_tile);
    }
    split_meter_value(engine, r);
    {
        let mut filled_blocks: i32 = (r.offset as u8 as i32);
        let mut sprite_slot: i32 = ((engine.state.scratch1 + 24) as u8 as i32);
        loop {
            filled_blocks = ((filled_blocks - 1) as u8 as i32);
            if (filled_blocks == 0) {
                break;
            }
            engine.state.set_oam_tile(
                64 + sprite_slot,
                (engine.state.oam_tile(64 + sprite_slot) - 2) & crate::bits::BYTE_MASK,
            );
            filled_blocks = ((filled_blocks - 1) as u8 as i32);
            if (filled_blocks == 0) {
                break;
            }
            engine.state.set_oam_tile(
                64 + sprite_slot,
                (engine.state.oam_tile(64 + sprite_slot) - 2) & crate::bits::BYTE_MASK,
            );
            sprite_slot = ((sprite_slot + 4) as u8 as i32);
        }
    }
    {
        let mut partial_blocks: i32 = (engine.state.scratch0 as i32);
        let mut sprite_slot: i32 = ((engine.state.scratch1 + 44) as u8 as i32);
        loop {
            partial_blocks = ((partial_blocks - 1) as u8 as i32);
            if (partial_blocks == 0) {
                break;
            }
            engine.state.set_oam_tile(
                64 + sprite_slot,
                (engine.state.oam_tile(64 + sprite_slot) - 2) & crate::bits::BYTE_MASK,
            );
            partial_blocks = ((partial_blocks - 1) as u8 as i32);
            if (partial_blocks == 0) {
                break;
            }
            engine.state.set_oam_tile(
                64 + sprite_slot,
                (engine.state.oam_tile(64 + sprite_slot) - 2) & crate::bits::BYTE_MASK,
            );
            sprite_slot = ((sprite_slot + 4) as u8 as i32);
        }
    }
}

/// Splits the value in `0x08` into full 10-point meter blocks (`r.offset`)
/// and a one-based partial block (`0x08`/`r.value`).
pub fn split_meter_value(engine: &mut Engine, r: &mut RoutineContext) {
    let mut remainder: i32 = (engine.state.scratch0 as i32);
    let mut full_blocks: i32 = 0;
    let mut carry: i32 = 1;
    loop {
        full_blocks = ((full_blocks + 1) as u8 as i32);
        let trial: i32 = (remainder) - 10 - (1 - carry);
        remainder = (trial as u8 as i32);
        carry = ((if (trial >= 0) { 1 } else { 0 }) as u8 as i32);
        if ((carry) == 0) {
            break;
        }
    }
    remainder = ((remainder + 11 + carry) as u8 as i32);
    engine.state.scratch0 = (remainder as u8);
    r.value = (remainder as u8);
    r.offset = (full_blocks as u8);
}

/// Waits for release, then press, then release again, returning the pressed
/// button byte in `r.value` and `0x20`.
pub fn read_debounced_buttons(engine: &mut Engine, r: &mut RoutineContext) {
    wait_for_buttons_released(engine, r);
    wait_for_button_press(engine, r);
    {
        let pressed_buttons: i32 = (r.value as u8 as i32);
        wait_for_buttons_released(engine, r);
        r.value = (pressed_buttons as u8);
        engine.state.buttons = (pressed_buttons as u8);
    }
}

/// Clears the deferred VRAM job selector at `0x28`.
pub fn clear_pending_vram_job(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.nmi_vram_req = 0;
}

/// Builds player movement deltas from current directional input and speed
/// `r.offset`, storing them in `0x49..0x4B`.
pub fn build_input_movement_delta(engine: &mut Engine, r: &mut RoutineContext) {
    let speed: i32 = (r.offset as u8 as i32);
    engine.state.scratch1 = (speed as u8);
    if (speed == 0) {
        engine.state.horizontal_subtile_delta = 0;
        engine.state.player_x_velocity = 0;
        engine.state.vertical_delta = 0;
        return;
    }
    let direction_index: i32 =
        (((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    let mut horizontal_delta: i32 = 0;
    {
        let mut steps = speed;
        while (steps != 0) {
            horizontal_delta = ((horizontal_delta
                + engine.state.byte(MOVE_DELTA_X_TABLE + direction_index))
                as u8 as i32);
            {
                let __old = steps;
                steps -= 1;
                __old
            };
        }
    }
    engine.state.horizontal_subtile_delta = ((horizontal_delta & crate::bits::LOW_NIBBLE) as u8);
    let sign_fill: i32 = (if ((horizontal_delta & crate::bits::BIT7) != 0) {
        240
    } else {
        0
    });
    engine.state.scratch0 = (sign_fill as u8);
    engine.state.player_x_velocity =
        ((((horizontal_delta & crate::bits::HIGH_NIBBLE) >> 4) | sign_fill) as u8);
    let mut vertical_delta: i32 = 0;
    {
        let mut steps = speed;
        while (steps != 0) {
            vertical_delta = ((vertical_delta
                + engine.state.byte(MOVE_DELTA_Y_TABLE + direction_index))
                as u8 as i32);
            {
                let __old = steps;
                steps -= 1;
                __old
            };
        }
    }
    engine.state.vertical_delta = (vertical_delta as u8);
}

/// Builds object/projectile velocity from direction bits in `r.value` and
/// speed `r.offset`, storing it in `0xF5..0xF7`.
pub fn build_direction_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let speed: i32 = (r.offset as u8 as i32);
    engine.state.scratch1 = (speed as u8);
    if (speed == 0) {
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_y_vel = 0;
        return;
    }
    let direction_index: i32 = (((r.value & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    let mut horizontal_delta: i32 = 0;
    {
        let mut steps = speed;
        while (steps != 0) {
            horizontal_delta = ((horizontal_delta
                + engine.state.byte(MOVE_DELTA_X_TABLE + direction_index))
                as u8 as i32);
            {
                let __old = steps;
                steps -= 1;
                __old
            };
        }
    }
    engine.state.obj_x_vel_lo = ((horizontal_delta & crate::bits::LOW_NIBBLE) as u8);
    let sign_fill: i32 = (if ((horizontal_delta & crate::bits::BIT7) != 0) {
        240
    } else {
        0
    });
    engine.state.scratch0 = (sign_fill as u8);
    engine.state.obj_x_vel_hi =
        ((((horizontal_delta & crate::bits::HIGH_NIBBLE) >> 4) | sign_fill) as u8);
    let mut vertical_delta: i32 = 0;
    {
        let mut steps = speed;
        while (steps != 0) {
            vertical_delta = ((vertical_delta
                + engine.state.byte(MOVE_DELTA_Y_TABLE + direction_index))
                as u8 as i32);
            {
                let __old = steps;
                steps -= 1;
                __old
            };
        }
    }
    engine.state.obj_y_vel = (vertical_delta as u8);
}

/// Checks the projected object position against the player hitbox. Carry
/// and `0xEA` are set on overlap.
pub fn check_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.overlap_flag = 0;
    check_player_y_overlap(engine, r);
    if (r.carry == 0) {
        return;
    }
    check_player_x_overlap(engine, r);
    if (r.carry == 0) {
        return;
    }
    engine.state.overlap_flag = 1;
    r.carry = 1;
}

/// Checks horizontal player overlap using projected tile/subtile position
/// in `0x0E/0x0F`.
pub fn check_player_x_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_delta: i32 =
        ((engine.state.indirect_ptr_hi - engine.state.player_x_tile) as u8 as i32);
    if (tile_delta == 0) {
        return;
    }
    if (tile_delta < 2) {
        let subtile_delta: i32 =
            ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
        r.carry = (if ((subtile_delta & crate::bits::BIT7) != 0) {
            1
        } else {
            0
        });
        return;
    }
    if (tile_delta < 255) {
        return;
    }
    {
        let subtile_delta: i32 =
            ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
        if (subtile_delta == 0) {
            return;
        }
        if ((subtile_delta & crate::bits::BIT7) != 0) {
            return;
        }
        r.carry = 1;
    }
}

/// Checks vertical player overlap using projected y position in `0x0A`.
pub fn check_player_y_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let y_delta: i32 = ((engine.state.scratch2 - engine.state.player_y) as u8 as i32);
    if (y_delta < 16) {
        r.carry = 1;
    } else if (y_delta < 241) {
        r.carry = 0;
    } else {
        r.carry = 1;
    }
}

/// Wider player-overlap test used by falling/large movement probes. Carry
/// and `0xEA` are set on overlap.
pub fn check_player_overlap_wide(engine: &mut Engine, r: &mut RoutineContext) {
    let mut dy: i32 = 0;
    let mut dx: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.overlap_flag = 0;
                dy = ((engine.state.scratch2 - engine.state.player_y) as u8 as i32);
                if ((dy >= 16) && (dy < 225)) {
                    r.carry = 0;
                    return;
                }
                dx = ((engine.state.indirect_ptr_hi - engine.state.player_x_tile) as u8 as i32);
                if (dx == 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (dx == 255) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (dx < 2) {
                    let subtile_delta: i32 =
                        ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
                    if ((subtile_delta & crate::bits::BIT7) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.carry = 0;
                    return;
                }
                if (dx < 254) {
                    return;
                }
                {
                    let subtile_delta: i32 =
                        ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
                    if (subtile_delta == 0) {
                        return;
                    }
                    if ((subtile_delta & crate::bits::BIT7) != 0) {
                        return;
                    }
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.state.overlap_flag = 1;
                r.carry = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Checks projected position against the general playfield bounds. Carry is
/// set when the position is outside the allowed area.
pub fn check_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.scratch2 >= 192) {
        r.carry = 1;
    } else if (engine.state.indirect_ptr_hi < 63) {
        r.carry = 0;
    } else if (engine.state.indirect_ptr_lo == 0) {
        r.carry = 0;
    } else {
        r.carry = 1;
    }
}

/// Checks projected actor position against the tighter actor playfield
/// bounds. Carry is set when the position is outside the allowed area.
pub fn check_actor_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.scratch2 >= 176) {
        r.carry = 1;
        return;
    }
    if (engine.state.indirect_ptr_hi < 63) {
        r.carry = 0;
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        r.carry = 0;
        return;
    }
    r.carry = 1;
}

/// Uploads every inventory item count to the item/status screen.
pub fn upload_inventory_count_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    {
        x = 15;
        while (x >= 0) {
            r.index = (x as u8);
            r.offset = ((engine.state.inventory_item(x)) as u8);
            upload_inventory_item_count_tiles(engine, r);
            r.index = (x as u8);
            {
                x -= 1;
                x
            };
        }
    }
    r.index = 255;
}

/// Uploads one inventory item count and applies the active family-member
/// availability palette adjustment for that item.
pub fn upload_inventory_item_count_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (r.index as u8 as i32);
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut s: i32 = 0;
    lo = (((x & crate::bits::LOW_3_BITS) << 2) as u8 as i32);
    lo = ((((x & crate::bits::BIT3) << 4) | lo) as u8 as i32);
    hi = 0;
    s = ((194 + lo) as u16 as i32);
    engine.state.vram_addr_lo = (s as u8);
    engine.state.vram_addr_hi = ((32 + hi + (s >> 8)) as u8);
    r.value = r.offset;
    build_decimal_digit_tiles(engine, r);
    {
        let mut in_: i32 = x;
        let mut dx: i32 = (((engine.state.character_index as i32) << 1) as u8 as i32);
        let mut yy: i32 = 0;
        let mut carry: i32 = 0;
        let mut v: i32 = 0;
        if (in_ >= 8) {
            {
                let __old = dx;
                dx += 1;
                __old
            };
        }
        yy = (((in_ & crate::bits::LOW_3_BITS) + 1) as u8 as i32);
        v = engine
            .state
            .byte(((MOVEMENT_PATTERN_TABLE + dx) as u16 as i32));
        carry = 0;
        loop {
            carry = ((v >> 7) as u8 as i32);
            v = ((v << 1) as u8 as i32);
            if ({
                yy -= 1;
                yy
            } == 0)
            {
                break;
            }
        }
        r.carry = (carry as u8);
    }
    r.value = (x as u8);
    load_family_item_permission_bits(engine, r);
    if ((r.carry) == 0) {
        engine.state.vram_addr2_lo = engine.state.vram_addr2_lo - 64;
        engine.state.vram_addr2_hi = engine.state.vram_addr2_hi - 64;
    }
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the effective projectile damage, jump duration, and projectile
/// lifetime values for the selected loadout.
pub fn upload_equipped_item_stat_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.vram_addr_lo = 222;
    engine.state.vram_addr_hi = 33;
    load_effective_projectile_damage(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
    engine.state.vram_addr_lo = 30;
    engine.state.vram_addr_hi = 34;
    load_effective_jump_duration(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
    engine.state.vram_addr_lo = 94;
    engine.state.vram_addr_hi = 34;
    load_effective_projectile_lifetime(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the two visible shop item prices.
pub fn upload_shop_price_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut c: i32 = 0;
    engine.state.vram_addr_lo = 71;
    engine.state.vram_addr_hi = 34;
    if ((engine.state.scroll_tile_x & ((crate::bits::BIT4) as u8)) != 0) {
        let mut s: i32 = ((0 + engine.state.vram_addr_lo) as u16 as i32);
        engine.state.vram_addr_lo = (s as u8);
        engine.state.vram_addr_hi = 4 + engine.state.vram_addr_hi + ((s >> 8) as u8);
    }
    r.value = ((engine.state.temp_save(1)) as u8);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
    lo = (engine.state.vram_addr_lo as i32);
    c = (((14 + lo) >> 8) as u8 as i32);
    engine.state.vram_addr_lo = ((14 + lo) as u8);
    hi = (engine.state.vram_addr_hi as i32);
    engine.state.vram_addr_hi = ((0 + hi + c) as u8);
    r.value = ((engine.state.temp_save(3)) as u8);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Converts `r.value` into two decimal digit tile ids in `0x18/0x19`.
pub fn build_decimal_digit_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32);
    let mut hi: i32 = 208;
    while (a >= 10) {
        a = ((a - 10) as u8 as i32);
        {
            hi += 1;
            hi
        };
    }
    a = ((a + 208) as u8 as i32);
    engine.state.vram_addr2_lo = (a as u8);
    if (hi == 208) {
        hi = 192;
    }
    engine.state.vram_addr2_hi = (hi as u8);
}

/// Loads the shifted family/item permission bits for `r.value`. Carry is
/// the bit shifted out by the final shift.
pub fn load_family_item_permission_bits(engine: &mut Engine, r: &mut RoutineContext) {
    let mut in_: i32 = (r.value as u8 as i32);
    let mut x: i32 = (((engine.state.character_index as i32) << 1) as u8 as i32);
    if (in_ >= 8) {
        {
            let __old = x;
            x += 1;
            __old
        };
    }
    let mut y: i32 = (((in_ & crate::bits::LOW_3_BITS) + 1) as u8 as i32);
    let mut a: i32 = engine
        .state
        .byte(((MOVEMENT_PATTERN_TABLE + x) as u16 as i32));
    let mut carry: i32 = 0;
    loop {
        carry = ((a >> 7) as u8 as i32);
        a = ((a << 1) as u8 as i32);
        if ({
            y -= 1;
            y
        } == 0)
        {
            break;
        }
    }
    r.carry = (carry as u8);
    r.value = (a as u8);
}

/// Starts `r.value` as the current song only when it differs from `0x8E`.
pub fn switch_song_if_needed(engine: &mut Engine, r: &mut RoutineContext) {
    if (r.value == (engine.state.song as u8)) {
        return;
    }
    engine.state.song = (r.value as u8);
    song_init(engine, r);
}

/// Loads the active character's jump duration. Carry is clear when the
/// selected jump item is present and magic can pay for the boosted value.
pub fn load_effective_jump_duration(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_item_slot);
    r.index = (selected_item_slot as u8);
    if (selected_item == 6) && (engine.state.player_magic != 0) {
        let base_jump_duration: i32 = (engine.state.jump_strength as i32);
        r.value = (((base_jump_duration >> 2) + base_jump_duration) as u8);
        r.carry = 0;
    } else {
        r.value = (engine.state.jump_strength as u8);
        r.carry = 1;
    }
}

/// Loads the projectile damage stat. Carry is clear when the selected
/// projectile-power item is active and magic can pay for the boosted shot.
pub fn load_effective_projectile_damage(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_item_slot);
    if (selected_item == 8) && (engine.state.player_magic != 0) {
        r.value = (((engine.state.projectile_damage as i32) << 2) as u8);
        r.carry = 0;
    } else {
        r.value = (engine.state.projectile_damage as u8);
        r.carry = 1;
    }
}

/// Loads the projectile lifetime/state byte. Carry is clear when the
/// selected projectile-range item is active and magic can pay for it.
pub fn load_effective_projectile_lifetime(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    r.index = (selected_item_slot as u8);
    if (engine.state.item_slot(selected_item_slot) == 9) && (engine.state.player_magic != 0) {
        r.value = (((engine.state.projectile_lifetime as i32) << 1) as u8);
        r.carry = 0;
        return;
    }
    r.value = (engine.state.projectile_lifetime as u8);
    r.carry = 1;
}

/// Hides the gameplay-object half of OAM, leaving HUD sprites untouched.
pub fn clear_gameplay_object_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut oam_offset: i32 = 128;
    loop {
        engine.state.set_oam_y(oam_offset, 239);
        oam_offset = ((oam_offset + 4) as u8 as i32);
        if (oam_offset == 0) {
            break;
        }
    }
    r.index = (oam_offset as u8);
    r.value = 239;
}

/// Clears all 16 object slots to inactive and resets the actor scheduler.
pub fn reset_room_object_slots(engine: &mut Engine, r: &mut RoutineContext) {
    let mut slot_offset: i32 = 0;
    let mut slots_remaining: i32 = 16;
    loop {
        engine.state.set_object_state(slot_offset, 0);
        engine.state.set_object_timer(slot_offset, 2);
        slot_offset = ((slot_offset + 16) as u8 as i32);
        if ({
            slots_remaining -= 1;
            slots_remaining
        } == 0)
        {
            break;
        }
    }
    engine.state.scheduler_phase = 0;
    r.value = 0;
    r.index = (slot_offset as u8);
    r.offset = 0;
}

/// Saves mutable inventory/progress state before the status or inventory
/// flows temporarily repurpose the same RAM range.
pub fn snapshot_inventory_state(engine: &mut Engine, r: &mut RoutineContext) {
    for progress_offset in (0..8).rev() {
        engine
            .state
            .set_save_progress(progress_offset, engine.state.save_payload(progress_offset));
    }
    for inventory_offset in (0..16).rev() {
        engine.state.set_save_inventory(
            inventory_offset,
            engine.state.inventory_item(inventory_offset),
        );
    }
    engine
        .state
        .set_save_inventory(17, (engine.state.coins as i32));
    engine
        .state
        .set_save_inventory(16, (engine.state.keys as i32));
    r.index = 255;
}

/// Restores the progress, inventory counts, coins, and keys saved by
/// `snapshot_inventory_state`.
pub fn restore_inventory_state_snapshot(engine: &mut Engine, r: &mut RoutineContext) {
    for progress_offset in (0..8).rev() {
        engine
            .state
            .set_save_payload(progress_offset, engine.state.save_progress(progress_offset));
    }
    for inventory_offset in (0..16).rev() {
        engine.state.set_inventory_item(
            inventory_offset,
            engine.state.save_inventory(inventory_offset),
        );
    }
    engine.state.coins = ((engine.state.save_inventory(17)) as u8);
    engine.state.keys = ((engine.state.save_inventory(16)) as u8);
    r.index = 255;
}

/// Converts the 32-byte item-list buffer at `0x0322` into the VRAM staging
/// buffer at `0x0362`, then uploads the two visible nametable rows.
pub fn upload_inventory_item_list(engine: &mut Engine, r: &mut RoutineContext) {
    let mut source_offset: i32 = 31;
    let mut staging_offset: i32 = 38;
    loop {
        {
            let mut chars_in_column: i32 = 0;
            while (chars_in_column < 4) {
                let mut tile: i32 = ((engine.state.password_nibbles_a(source_offset)
                    | crate::bits::BIT7) as u8 as i32);
                if (tile >= 160) {
                    tile = 127;
                }
                engine
                    .state
                    .set_password_nibbles_a(64 + (staging_offset & crate::bits::BYTE_MASK), tile);
                staging_offset = (staging_offset - 1) & crate::bits::BYTE_MASK;
                source_offset = (source_offset - 1) & crate::bits::BYTE_MASK;
                {
                    chars_in_column += 1;
                    chars_in_column
                };
            }
        }
        staging_offset = (staging_offset - 1) & crate::bits::BYTE_MASK;
        if !((staging_offset & crate::bits::BIT7) == 0) {
            break;
        }
    }
    engine.state.inventory_upload_col = 19;
    engine.state.inventory_upload_row = 0;
    engine.state.vram_addr_lo = 230;
    engine.state.vram_addr_hi = 36;
    engine.state.vram_addr2_lo = 98;
    engine.state.vram_addr2_hi = 3;
    r.value = 5;
    queue_ppu_job_and_wait(engine, r);
    engine.state.vram_addr_lo = 6;
    engine.state.vram_addr_hi = 37;
    engine.state.vram_addr2_lo = 118;
    engine.state.vram_addr2_hi = 3;
    r.value = 5;
    queue_ppu_job_and_wait(engine, r);
}

/// Fills the item-list source buffer with blank tile ids.
pub fn clear_inventory_item_list_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    for item_list_offset in (0..32).rev() {
        engine.state.set_password_nibbles_a(item_list_offset, 127);
    }
    r.value = 127;
    r.index = 255;
}

/// Starts or continues the player jump/action arc. `0x4F` is the active
/// jump timer, `0x22` prevents a held button from restarting the jump, and
/// selected item `0x06` extends the timer by spending magic.
pub fn tick_player_jump_action(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.jump_timer != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.collision_flag != 0) {
                    return;
                }
                engine.state.prompt_state = 27;
                engine.state.jump_timer = engine.state.jump_strength;
                {
                    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
                    if (engine.state.item_slot(selected_slot) == 6) {
                        consume_magic_point(engine, r);
                        if ((r.carry) == 0) {
                            let jump_timer: i32 = (engine.state.jump_timer as i32);
                            engine.state.jump_timer = (((jump_timer >> 2) + jump_timer) as u8);
                        }
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.lotw_nonlocal_handoff = 1;
                engine.state.collision_flag = 1;
                {
                    let jump_timer: i32 = (engine.state.jump_timer as i32);
                    engine.state.jump_timer = ((jump_timer - 1) as u8);
                    let upward_speed: i32 = ((jump_timer >> 2) as u8 as i32);
                    engine.state.vertical_delta =
                        (((upward_speed ^ crate::bits::BYTE_MASK) + 1) as u8);
                }
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                engine.state.horizontal_subtile_delta = 0;
                engine.state.player_x_velocity = 0;
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                engine.state.jump_timer =
                    (engine.state.jump_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
                try_nudge_player_to_tile_boundary(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                {
                    let mut y: i32 = (engine.state.scratch2 as i32);
                    if (y >= 239) {
                        y = 0;
                    }
                    engine.state.player_y = (y as u8);
                }
                update_player_terrain_contact(engine, r);
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                engine.state.jump_timer = 0;
                engine.state.fall_frames = 0;
                update_player_terrain_contact(engine, r);
                state = 4;
                continue 'dispatch;
            }
            4 => {
                update_player_pose_from_motion(engine, r);
                tick_player_walk_animation(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Applies the currently selected passive/consumable item effect. Item ids
/// below `0x02` are magic-draining effect timers, `0x0B` refills magic when
/// empty, and `0x0D` returns the player to the fixed safe room.
pub fn tick_selected_item_effect(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_slot);
    if (selected_item >= 2) {
        if (selected_item == 11) {
            if (engine.state.player_magic != 0) {
                return;
            }
            engine.state.set_item_slot(selected_slot, 255);
            draw_status_item_sprites(engine, r);
            animate_magic_refill_to_cap(engine, r);
            return;
        }
        if (selected_item != 13) {
            return;
        }
        if (engine.state.map_screen_y >= 17) {
            engine.state.selected_item_slot = 3;
            return;
        }
        engine.state.set_item_slot(selected_slot, 255);
        draw_status_item_sprites(engine, r);
        engine.state.prompt_state = 18;
        engine.state.map_screen_y = 16;
        engine.state.map_screen_x = 3;
        engine.state.scroll_tile_x = 18;
        engine.state.player_y = 176;
        engine.state.player_x_tile = 26;
        engine.state.player_x_fine = 0;
        engine.state.scroll_fine_x = 0;
        fade_room_palette_out_reset_audio(engine, r);
        reset_room_object_slots(engine, r);
        scene_assemble(engine, r);
        upload_current_room_view(engine, r);
        clear_gameplay_object_sprites(engine, r);
        refresh_scroll_register_shadows(engine, r);
        draw_player_sprites(engine, r);
        fade_room_palette_in(engine, r);
        r.carry = 1;
        return;
    }
    if (engine.state.inventory_item(38 + selected_item) != 0) {
        return;
    }
    r.index = (selected_item as u8);
    consume_magic_point(engine, r);
    if (r.carry == 0) {
        engine.state.set_inventory_item(38 + selected_item, 2);
        return;
    }
    {
        let continue_timer: i32 = (engine.state.continue_timer as i32);
        if ((continue_timer == 0) || ((continue_timer & crate::bits::BIT7) != 0)) {
            return;
        }
        engine.state.continue_timer = 253;
        engine.state.prompt_state = 26;
    }
}

/// Enters the destination encoded in the active room link record at
/// `0x77/0x78 + 0x0C..0x0F`.
pub fn enter_room_link_destination(engine: &mut Engine, r: &mut RoutineContext) {
    let link_ptr: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    engine.state.map_screen_x = ((engine.state.byte(((link_ptr + 12) as u16 as i32))) as u8);
    engine.state.map_screen_y = ((engine.state.byte(((link_ptr + 13) as u16 as i32))) as u8);

    let player_tile_x: i32 = engine.state.byte(((link_ptr + 14) as u16 as i32));
    engine.state.player_x_tile = (player_tile_x as u8);
    let scroll_x: i32 = if (player_tile_x >= 8) {
        ((player_tile_x - 8) as u8 as i32)
    } else {
        0
    };
    engine.state.scroll_tile_x = if (scroll_x >= 49) {
        48
    } else {
        (scroll_x as u8)
    };
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;

    r.value = ((engine.state.byte(((link_ptr + 15) as u16 as i32))) as u8);
    engine.state.player_y = (r.value as u8);
    fade_room_palette_out_reset_audio(engine, r);
    reset_room_object_slots(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    clear_gameplay_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    draw_player_sprites(engine, r);
    fade_room_palette_in(engine, r);
    r.carry = 1;
}

/// After collecting a `0x0E` fragment item, runs the warp transition and
/// moves to the fragment-specific room selected by `0x6E`.
pub fn enter_fragment_pickup_room(engine: &mut Engine, r: &mut RoutineContext) {
    run_warp_transition_effect(engine, r);
    engine.state.map_screen_y = 17;
    r.index = ((engine.state.fragment_count - 1) as u8);
    engine.state.map_screen_x = (r.index as u8);
    engine.state.scroll_tile_x = 18;
    engine.state.player_y = 16;
    engine.state.player_x_tile = 26;
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;
    r.value = 0;
    fade_room_palette_out_reset_audio(engine, r);
    reset_room_object_slots(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    clear_gameplay_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    draw_player_sprites(engine, r);
    fade_room_palette_in(engine, r);
    r.carry = 1;
}

/// Consumes the pending special-exit flag set by the high-bit actor path
/// and moves to its fixed destination room.
pub fn enter_pending_special_exit_room(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.pending_special_exit = 0;
    run_warp_transition_effect(engine, r);
    engine.state.set_chr_bank(4, 62);
    engine.state.map_screen_y = 16;
    engine.state.map_screen_x = 3;
    engine.state.scroll_tile_x = 18;
    engine.state.player_y = 176;
    engine.state.player_x_tile = 26;
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;
    r.value = 0;
    fade_room_palette_out_reset_audio(engine, r);
    reset_room_object_slots(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    clear_gameplay_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    draw_player_sprites(engine, r);
    fade_room_palette_in(engine, r);
    r.carry = 1;
}

/// Raises the final-exit flag when item `0x0F` is selected at the exact
/// room/scroll/player position expected by the original game.
pub fn check_final_exit_trigger(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    if ((engine.state.item_slot(selected_slot) == 15)
        && (engine.state.map_screen_x == 1)
        && (engine.state.map_screen_y == 5)
        && (engine.state.scroll_tile_x == 16)
        && (engine.state.scroll_fine_x == 0)
        && (engine.state.player_y == 160))
    {
        engine.state.final_exit_flag = 1;
    }
}

/// Shared scroll/audio transition used before scripted room warps.
pub fn run_warp_transition_effect(engine: &mut Engine, r: &mut RoutineContext) {
    let mut outer: i32 = 0;
    clear_oam_with_sprite_zero_template(engine, r);
    engine.state.sprite_blink_timer = 0;
    draw_player_sprites(engine, r);
    draw_status_item_sprites(engine, r);
    if (engine.state.scroll_tile_x >= 33) {
        engine.state.scroll_tile_x = 32;
    }
    upload_room_columns_from_bank9(engine, r);
    engine.state.scroll_tile_x = engine.state.scroll_tile_x + 16;
    upload_room_columns_from_bank9(engine, r);
    engine.state.scratch0 = 1;
    loop {
        let mut x: i32 = 12;
        loop {
            let mut sum: i32 =
                ((engine.state.scroll_pixel_x + engine.state.scratch0) as u16 as i32);
            engine.state.scroll_pixel_x = (sum as u8);
            if ((sum & crate::bits::BIT8) != 0) {
                engine.state.nametable_select =
                    engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
            }
            r.value = 255;
            queue_ppu_job_and_wait(engine, r);
            if ({
                x -= 1;
                x
            } == 0)
            {
                break;
            }
        }
        engine.state.scratch0 = engine.state.scratch0 + 1;
        outer = (engine.state.scratch0 as i32);
        if !(outer < 32) {
            break;
        }
    }
    engine.state.prompt_state = 24;
    engine.state.prompt_argument = 255;
    r.index = 8;
    flash_palette_buffer(engine, r);
}

fn scene_rebuild_full(engine: &mut Engine, r: &mut RoutineContext) {
    fade_room_palette_out_reset_audio(engine, r);
    reset_room_object_slots(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    clear_gameplay_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    draw_player_sprites(engine, r);
    fade_room_palette_in(engine, r);
    engine.state.frame_counter = 0;
    r.carry = 1;
}

fn scene_rebuild_vert(engine: &mut Engine, r: &mut RoutineContext) {
    reset_room_object_slots(engine, r);
    clear_gameplay_object_sprites(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    upload_palette_buffer(engine, r);
    engine.state.frame_counter = 0;
    r.carry = 1;
}

/// Handles player transitions across room edges. Vertical exits can rebuild
/// a whole room or a vertical strip; horizontal exits play the side-scroll
/// transition while moving the map-space room coordinate.
pub fn handle_player_room_transition(engine: &mut Engine, r: &mut RoutineContext) {
    let player_y: i32 = (engine.state.player_y as i32);
    if (player_y < 16) {
        check_top_boundary_exit_clear(engine, r);
        if (r.carry == 0) {
            return;
        }
        if (engine.state.map_screen_y == 0) {
            engine.state.map_screen_y = 16;
            engine.state.map_screen_x = 3;
            engine.state.scroll_tile_x = 18;
            engine.state.player_y = 176;
            engine.state.player_x_tile = 26;
            engine.state.player_x_fine = 0;
            engine.state.scroll_fine_x = 0;
            scene_rebuild_full(engine, r);
            return;
        }
        if (engine.state.map_screen_y == 16) {
            return;
        }
        engine.state.map_screen_y = engine.state.map_screen_y - 1;
        engine.state.player_y = 176;
        scene_rebuild_vert(engine, r);
        return;
    }
    if (player_y >= 161) {
        if (engine.state.map_screen_y == 16) {
            engine.state.map_screen_y = 0;
            engine.state.map_screen_x = 0;
            engine.state.scroll_tile_x = 0;
            engine.state.player_y = 0;
            engine.state.player_x_fine = 0;
            engine.state.scroll_fine_x = 0;
            engine.state.player_x_tile = 1;
            scene_rebuild_full(engine, r);
            return;
        }
        if (((engine.state.map_screen_y + 1) as u8 as i32) >= 16) {
            return;
        }
        engine.state.map_screen_y = engine.state.map_screen_y + 1;
        engine.state.player_y = 0;
        scene_rebuild_vert(engine, r);
        return;
    }
    if (engine.state.map_screen_y == 16) {
        return;
    }
    update_player_terrain_contact(engine, r);
    engine.state.sprite_blink_timer = 0;
    engine.state.player_pose = engine.state.player_pose & ((crate::bits::LOW_3_BITS) as u8);
    if (engine.state.player_x_tile == 0) {
        if ((((engine.state.map_screen_x - 1) as u8 as i32) & crate::bits::BIT7) != 0) {
            return;
        }
        engine.state.map_screen_x = engine.state.map_screen_x - 1;
        engine.state.player_facing = 0;
        draw_player_sprites(engine, r);
        engine.state.scroll_tile_x = 48;
        engine.state.player_x_tile = 63;
        engine.state.player_x_fine = 0;
    } else {
        if (engine.state.player_x_tile < 62) {
            return;
        }
        if (((engine.state.map_screen_x + 1) as u8 as i32) >= 4) {
            return;
        }
        engine.state.map_screen_x = engine.state.map_screen_x + 1;
        engine.state.player_facing = 64;
        draw_player_sprites(engine, r);
        engine.state.scroll_tile_x = 0;
        engine.state.player_x_fine = 0;
        engine.state.player_x_tile = 0;
    }
    reset_room_object_slots(engine, r);
    clear_gameplay_object_sprites(engine, r);
    engine.state.scroll_fine_x = 0;
    scene_assemble(engine, r);
    upload_room_columns_from_bank9(engine, r);
    upload_palette_buffer(engine, r);
    if (engine.state.player_x_tile != 0) {
        engine.state.nametable_select = 1;
        engine.state.scroll_pixel_x = 0;
        engine.state.set_oam_x(16, 0);
        engine.state.set_oam_x(20, 8);
        engine.state.scratch2 = 15;
        loop {
            engine.state.scratch3 = 3;
            loop {
                if (engine.state.scratch3 == 0) {
                    engine.state.set_oam_x(16, engine.state.oam_x(16) - 1);
                    engine.state.set_oam_x(20, engine.state.oam_x(20) - 1);
                    if ((engine.state.fall_frames | engine.state.jump_timer) == 0) {
                        engine
                            .state
                            .set_oam_tile(16, engine.state.oam_tile(16) ^ crate::bits::BIT2);
                        engine
                            .state
                            .set_oam_tile(20, engine.state.oam_tile(20) ^ crate::bits::BIT2);
                    }
                }
                engine.state.set_oam_x(16, engine.state.oam_x(16) + 4);
                engine.state.set_oam_x(20, engine.state.oam_x(16) + 8);
                engine.state.scroll_pixel_x = engine.state.scroll_pixel_x - 4;
                r.value = 255;
                queue_ppu_job_and_wait(engine, r);
                engine.state.scratch3 = engine.state.scratch3 - 1;
                if !((engine.state.scratch3 & ((crate::bits::BIT7) as u8)) == 0) {
                    break;
                }
            }
            engine.state.scratch2 = engine.state.scratch2 - 1;
            if !((engine.state.scratch2 & ((crate::bits::BIT7) as u8)) == 0) {
                break;
            }
        }
        engine.state.vram_addr_lo = 30;
        engine.state.vram_addr_hi = 32;
        engine.state.data_ptr_lo = 47;
        farcall_bank_09_r7(engine, r);
        engine.state.frame_counter = 0;
        r.carry = 1;
        return;
    }
    engine.state.scroll_pixel_x = 252;
    engine.state.nametable_select = 1;
    engine.state.set_oam_x(16, 240);
    engine.state.set_oam_x(20, 248);
    engine.state.scratch2 = 15;
    loop {
        engine.state.scratch3 = 3;
        loop {
            if (engine.state.scratch3 == 0) {
                engine.state.set_oam_x(16, engine.state.oam_x(16) + 1);
                engine.state.set_oam_x(20, engine.state.oam_x(20) + 1);
                if ((engine.state.fall_frames | engine.state.jump_timer) == 0) {
                    engine
                        .state
                        .set_oam_tile(16, engine.state.oam_tile(16) ^ crate::bits::BIT2);
                    engine
                        .state
                        .set_oam_tile(20, engine.state.oam_tile(20) ^ crate::bits::BIT2);
                }
            }
            engine.state.set_oam_x(16, engine.state.oam_x(16) - 4);
            engine.state.set_oam_x(20, engine.state.oam_x(16) + 8);
            engine.state.scroll_pixel_x = engine.state.scroll_pixel_x + 4;
            r.value = 255;
            queue_ppu_job_and_wait(engine, r);
            engine.state.scratch3 = engine.state.scratch3 - 1;
            if !((engine.state.scratch3 & ((crate::bits::BIT7) as u8)) == 0) {
                break;
            }
        }
        engine.state.scratch2 = engine.state.scratch2 - 1;
        if !((engine.state.scratch2 & ((crate::bits::BIT7) as u8)) == 0) {
            break;
        }
    }
    engine.state.vram_addr_lo = 0;
    engine.state.vram_addr_hi = 36;
    engine.state.data_ptr_lo = 16;
    farcall_bank_09_r7(engine, r);
    engine.state.frame_counter = 0;
    r.carry = 1;
}

/// Copies player position `0x43..0x45` into projection scratch
/// `0x0E/0x0F/0x0A`, then applies horizontal delta `0x49/0x4A` and vertical
/// delta `0x4B`.
pub fn project_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.scratch2 = engine.state.player_y;
    if (engine.state.vertical_delta != 0) {
        engine.state.scratch2 = engine.state.vertical_delta + engine.state.scratch2;
    }
    let horizontal_subtile_delta: i32 = (engine.state.horizontal_subtile_delta as i32);
    if (horizontal_subtile_delta != 0) {
        let sum: i32 =
            ((horizontal_subtile_delta + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((sum & crate::bits::LOW_NIBBLE) as u8);
        let carry: i32 = (((sum >> 4) & 1) as u8 as i32);
        engine.state.indirect_ptr_hi =
            engine.state.indirect_ptr_hi + engine.state.player_x_velocity + (carry as u8);
    }
}

/// Updates the player pose byte `0x56` and horizontal flip `0x57` from the
/// current movement deltas, jump/fall counters, and action lockout.
pub fn update_player_pose_from_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut a: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                x = 61;
                if (engine.state.landing_timer != 0) {
                    return;
                }
                x = 9;
                if (engine.state.pose_state != 0) {
                    return;
                }
                if ((engine.state.buttons & ((crate::bits::CLEAR_BIT6) as u8)) == 128) {
                    return;
                }
                a = (engine.state.vertical_delta as i32);
                if (a == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if ((a & crate::bits::BIT7) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.fall_frames != 0) {
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                x = 13;
                engine.state.player_pose = (x as u8);
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if (engine.state.jump_timer == 0) {
                    return;
                }
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                x = 1;
                y = 0;
                if ((engine.state.player_x_velocity & ((crate::bits::BIT7) as u8)) != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.horizontal_subtile_delta == 0) {
                    return;
                }
                y = 64;
                state = 3;
                continue 'dispatch;
            }
            3 => {
                engine.state.scratch0 = (x as u8);
                engine.state.player_pose = (engine.state.player_pose
                    & ((crate::bits::LOW_3_BITS) as u8))
                    | engine.state.scratch0;
                engine.state.player_facing = (y as u8);
                return;
                state = 4;
                continue 'dispatch;
            }
            4 => {
                x = 57;
                y = 0;
                a = ((engine.state.player_x_velocity | engine.state.horizontal_subtile_delta)
                    as i32);
                if ((a & crate::bits::BIT7) != 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                if (a != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                x = 9;
                state = 5;
                continue 'dispatch;
            }
            5 => {
                y = 64;
                state = 6;
                continue 'dispatch;
            }
            6 => {
                engine.state.scratch0 = (x as u8);
                engine.state.player_pose = (engine.state.player_pose
                    & ((crate::bits::LOW_2_BITS) as u8))
                    | engine.state.scratch0;
                engine.state.player_facing = (y as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Advances the walking animation every eight movement ticks and folds the
/// current action/facing button into the pose byte.
pub fn tick_player_walk_animation(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.landing_timer == 0) {
        if (engine.state.player_pose < 32) {
            if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                engine.state.player_pose = engine.state.player_pose | ((crate::bits::BIT4) as u8);
            } else {
                engine.state.player_pose =
                    engine.state.player_pose & ((crate::bits::CLEAR_BIT4) as u8);
            }
        }
    }
    if ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    if ((engine.state.jump_timer | engine.state.fall_frames) != 0) {
        return;
    }
    engine.state.anim_step_counter = engine.state.anim_step_counter + 1;
    if ((engine.state.anim_step_counter & ((crate::bits::LOW_3_BITS) as u8)) != 0) {
        return;
    }
    if ((engine.state.player_pose & ((crate::bits::BIT3) as u8)) != 0) {
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8);
    } else {
        engine.state.player_pose = engine.state.player_pose ^ ((crate::bits::BIT2) as u8);
    }
}

/// Projects a player move, handles room exits/tile actions/object contact,
/// retries speed-boost nudges, and restores movement deltas before return.
pub fn try_move_player_with_collision(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_vertical_delta: i32 = (engine.state.vertical_delta as i32);
    let saved_horizontal_subtile_delta: i32 = (engine.state.horizontal_subtile_delta as i32);
    let mut a: i32 = 0;
    let mut x: i32 = 0;
    let mut v: i32 = 0;
    let mut state: i32 = 1;
    'dispatch: loop {
        match state {
            1 => {
                project_player_position(engine, r);
                check_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    handle_player_room_transition(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 7;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                dispatch_projected_tile_actions(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                find_player_object_overlap(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 8;
                        continue 'dispatch;
                    }
                }
                a = (engine.state.scratch0 as i32);
                if (a == 9) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                if (a < 9) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                x = (engine.state.scratch1 as i32);
                r.index = (x as u8);
                v = engine.state.object_state(x);
                r.value = (v as u8);
                if (v == 1) {
                    unlock_door_with_key(engine, r);
                    {
                        state = 8;
                        continue 'dispatch;
                    }
                }
                apply_event_collectible_reward(engine, r);
                clear_room_persistent_flag(engine, r);
                {
                    state = 7;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                x = (engine.state.scratch1 as i32);
                r.index = (x as u8);
                v = engine.state.object_state(x);
                r.value = (v as u8);
                if (v == 1) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (v >= 26) {
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                collect_room_pickup_object(engine, r);
                {
                    state = 7;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                try_trigger_magic_contact_actor(engine, r);
                state = 4;
                continue 'dispatch;
            }
            4 => {
                r.carry = 0;
                {
                    state = 8;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                if (engine.state.displaced_timer == 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                a = (engine.state.horizontal_subtile_delta as i32);
                if (a == 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                x = a;
                if ((a & crate::bits::BIT3) == 0) {
                    x = ((x - 2) as u8 as i32);
                }
                x = ((x + 1) as u8 as i32);
                a = ((x & crate::bits::LOW_NIBBLE) as u8 as i32);
                engine.state.horizontal_subtile_delta = (a as u8);
                if (a != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                state = 6;
                continue 'dispatch;
            }
            6 => {
                engine.state.horizontal_subtile_delta = (saved_horizontal_subtile_delta as u8);
                x = (engine.state.vertical_delta as i32);
                if (x == 0) {
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                }
                if ((x & crate::bits::BIT7) == 0) {
                    x = ((x - 2) as u8 as i32);
                }
                x = ((x + 1) as u8 as i32);
                engine.state.vertical_delta = (x as u8);
                if (x != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                state = 7;
                continue 'dispatch;
            }
            7 => {
                r.carry = 1;
                state = 8;
                continue 'dispatch;
            }
            8 => {
                engine.state.horizontal_subtile_delta = (saved_horizontal_subtile_delta as u8);
                engine.state.vertical_delta = (saved_vertical_delta as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Marks the contacted actor for its high-bit behavior when the selected
/// magic-contact timer is active and magic remains.
pub fn try_trigger_magic_contact_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.chr_bank(3) < 48)
        && (engine.state.magic_contact_flag != 0)
        && (engine.state.player_magic != 0))
    {
        let hit_slot: i32 = (engine.state.scratch1 as i32);
        engine.state.set_object_state(hit_slot, 128);
    }
}

/// Applies a collectible reward that came from an event/shop path where no
/// room object slot needs to be cleared.
pub fn apply_event_collectible_reward(engine: &mut Engine, r: &mut RoutineContext) {
    let reward_id: i32 = (((r.value - 2) as u8) as i32);
    engine.state.set_object_state(160, 0);
    if (reward_id >= 24) {
        engine.state.prompt_state = 6;
        return;
    }
    if (reward_id < 8) {
        const EVENT_REWARD_TEXT: [i32; 8] = [
            0xD16A, 0xD199, 0xDB47, 0xDB52, 0xDB66, 0xDB7B, 0xDBB7, 0xDB9B,
        ];
        engine.state.data_ptr_lo =
            ((EVENT_REWARD_TEXT[reward_id as usize] & crate::bits::BYTE_MASK) as u8);
        engine.state.data_ptr_hi = ((EVENT_REWARD_TEXT[reward_id as usize] >> 8) as u8);
        r.value = ((reward_id << 1) as u8);
        r.index = r.value;
        match reward_id {
            0 => {
                animate_health_refill_to_cap(engine, r);
            }
            1 => {
                animate_magic_refill_to_cap(engine, r);
            }
            2 => {
                collect_large_coin_reward(engine, r);
            }
            3 => {
                trigger_damage_pickup(engine, r);
            }
            4 => {
                collect_key_bundle_reward(engine, r);
            }
            5 => {
                grant_long_invulnerability(engine, r);
            }
            6 => {
                defeat_active_room_actors(engine, r);
            }
            7 => {
                grant_long_speed_boost(engine, r);
            }
            _ => {}
        }
        return;
    }
    {
        let inventory_item_id: i32 = ((reward_id - 8) as u8 as i32);
        if (engine.state.inventory_item(inventory_item_id) >= 11) {
            engine.state.prompt_state = 29;
            return;
        }
        engine.state.set_inventory_item(
            inventory_item_id,
            (engine.state.inventory_item(inventory_item_id) + 1) & crate::bits::BYTE_MASK,
        );
        engine.state.prompt_state = 19;
        if (inventory_item_id == 14) {
            clear_room_persistent_flag(engine, r);
            enter_fragment_pickup_room(engine, r);
        }
    }
}

/// Clears the touched room object slot/OAM entry and applies its reward.
pub fn collect_room_pickup_object(engine: &mut Engine, r: &mut RoutineContext) {
    let reward_id: i32 = (((r.value - 2) as u8) as i32);
    if (reward_id >= 24) {
        return;
    }
    {
        let object_slot_offset: i32 = (r.index as u8 as i32);
        engine.state.set_object_state(object_slot_offset, 0);
        engine.state.set_object_timer(object_slot_offset, 240);
    }
    {
        let oam_offset: i32 = ((((engine.state.scratch0 as i32) << 3)
            | (((crate::bits::BIT7) as u8) as i32)) as u8 as i32);
        engine.state.set_oam_y(oam_offset, 239);
        engine.state.set_oam_y(4 + oam_offset, 239);
        r.index = (oam_offset as u8);
    }
    if (reward_id < 8) {
        const PICKUP_REWARD_TEXT: [i32; 8] = [
            0xDB26, 0xDB31, 0xDB3C, 0xDB52, 0xDB5D, 0xDB71, 0xDBB7, 0xDB85,
        ];
        engine.state.data_ptr_lo =
            ((PICKUP_REWARD_TEXT[reward_id as usize] & crate::bits::BYTE_MASK) as u8);
        engine.state.data_ptr_hi = ((PICKUP_REWARD_TEXT[reward_id as usize] >> 8) as u8);
        r.value = ((reward_id << 1) as u8);
        r.index = r.value;
        match reward_id {
            0 => {
                collect_small_health_reward(engine, r);
            }
            1 => {
                collect_small_magic_reward(engine, r);
            }
            2 => {
                collect_small_coin_reward(engine, r);
            }
            3 => {
                trigger_damage_pickup(engine, r);
            }
            4 => {
                collect_single_key_reward(engine, r);
            }
            5 => {
                grant_short_invulnerability(engine, r);
            }
            6 => {
                defeat_active_room_actors(engine, r);
            }
            7 => {
                grant_short_speed_boost(engine, r);
            }
            _ => {}
        }
        return;
    }
    {
        let inventory_item_id: i32 = ((reward_id - 8) as u8 as i32);
        if (engine.state.inventory_item(inventory_item_id) >= 11) {
            engine.state.prompt_state = 29;
            return;
        }
        engine.state.set_inventory_item(
            inventory_item_id,
            (engine.state.inventory_item(inventory_item_id) + 1) & crate::bits::BYTE_MASK,
        );
        engine.state.prompt_state = 19;
        if (inventory_item_id == 14) {
            clear_room_persistent_flag(engine, r);
            enter_fragment_pickup_room(engine, r);
        }
    }
}

/// Adds a small health reward and plays the health pickup sound.
pub fn collect_small_health_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 30;
    r.value = 5;
    add_health_points(engine, r);
}

/// Adds a small magic reward.
pub fn collect_small_magic_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17;
    r.value = 5;
    add_magic_points(engine, r);
}

/// Adds the small coin reward.
pub fn collect_small_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17;
    r.value = 2;
    add_coins(engine, r);
}

/// Adds the large coin reward.
pub fn collect_large_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17;
    r.value = 50;
    add_coins(engine, r);
}

/// Applies the harmful pickup/trap effect.
pub fn trigger_damage_pickup(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 29;
    r.value = 5;
    subtract_health_points(engine, r);
}

/// Adds one key.
pub fn collect_single_key_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 21;
    add_key(engine, r);
}

/// Adds the large key bundle reward.
pub fn collect_key_bundle_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 21;
    r.value = 20;
    add_keys(engine, r);
}

/// Grants the short invulnerability timer.
pub fn grant_short_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 19;
    engine.state.sprite_blink_timer = 10;
    r.value = 10;
}

/// Grants the long invulnerability timer.
pub fn grant_long_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 19;
    engine.state.sprite_blink_timer = 30;
    r.value = 30;
}

/// Starts or queues a short speed/action boost timer in `0x88..0x8A`.
pub fn grant_short_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
    let boost_duration: i32 = 30;
    let mut displaced_timer: i32 = 0;
    engine.state.prompt_state = 19;
    displaced_timer = (engine.state.displaced_timer as i32);
    if (displaced_timer != 0) {
        displaced_timer = (engine.state.boost_timer as i32);
        if (displaced_timer != 0) {
            engine.state.short_boost_timer = (boost_duration as u8);
        }
        engine.state.boost_timer = (boost_duration as u8);
    }
    engine.state.displaced_timer = (boost_duration as u8);
    r.value = (displaced_timer as u8);
    r.index = (boost_duration as u8);
}

/// Starts or queues a long speed/action boost timer in `0x88..0x8B`.
pub fn grant_long_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
    let boost_duration: i32 = 60;
    let mut displaced_timer: i32 = 0;
    engine.state.prompt_state = 19;
    displaced_timer = (engine.state.displaced_timer as i32);
    if (displaced_timer != 0) {
        displaced_timer = (engine.state.boost_timer as i32);
        if (displaced_timer != 0) {
            displaced_timer = (engine.state.short_boost_timer as i32);
            if (displaced_timer != 0) {
                engine.state.long_boost_timer = (boost_duration as u8);
            }
            engine.state.short_boost_timer = (boost_duration as u8);
        }
        engine.state.boost_timer = (boost_duration as u8);
    }
    engine.state.displaced_timer = (boost_duration as u8);
    r.value = (displaced_timer as u8);
    r.index = (boost_duration as u8);
}

/// Marks active room actors as defeated, then runs the palette flash effect.
pub fn defeat_active_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
    let mut slot_offset: i32 = 0;
    for _ in 0..9 {
        if (engine.state.object_state(slot_offset) == 1) {
            engine.state.set_object_state(slot_offset, 128);
        }
        slot_offset = ((slot_offset + 16) as u8 as i32);
    }
    engine.state.prompt_state = 24;
    engine.state.prompt_argument = 255;
    r.index = 2;
    flash_palette_buffer(engine, r);
}

/// Returns carry set when the tile above the top screen edge is empty and
/// the player can wrap to the room above.
pub fn check_top_boundary_exit_clear(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.airborne_flag != 0 || engine.state.jump_timer != 0 {
        return;
    }
    if engine.state.indirect_ptr_lo != 0 {
        return;
    }
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    let tile = engine.state.byte(tile_ptr) & crate::bits::LOW_6_BITS;
    r.carry = ((tile == 0) as u8);
}

/// Applies tile `0x30` hazard contact at `tile_ptr + r.offset`, including
/// the short recoil timer and one-hit invulnerability latch.
pub fn apply_hazard_tile_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    let tile = engine
        .state
        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS;
    if tile != 48 {
        r.carry = 0;
        return;
    }
    if engine.state.jump_timer == 0 {
        engine.state.jump_timer = 10;
    }
    if engine.state.sprite_blink_timer == 0 {
        consume_health_point(engine, r);
        engine.state.prompt_state = 10;
        engine.state.sprite_blink_timer = 1;
    }
    r.carry = 1;
}

/// Reports whether a player footprint sample collides with terrain.
/// Empty tiles only count as contact when the player is tile-aligned.
pub fn probe_player_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    let tile = engine
        .state
        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS;
    if tile == 0 {
        if engine.state.player_x_fine == 0 {
            r.carry = 1;
        } else {
            r.carry = 0;
        }
    } else if tile == 2 {
        r.carry = 1;
    } else {
        r.carry = ((tile >= 48) as u8);
    }
}

/// Handles Up-button interactions with the tile directly above the player.
/// Tile `0x05` and `0x04` jump to their dedicated scripts; tile `0x03`
/// requires the selected `0x0E` item and all four matching fragments.
pub fn dispatch_overhead_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    let player_y = engine.state.player_y;
    if player_y == 0 {
        return;
    }

    engine.state.data_ptr_hi = player_y - 1;
    engine.state.data_ptr_lo = engine.state.player_x_tile;
    resolve_room_tile_pointer(engine, r);

    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    if dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 0) {
        return;
    }
    if engine.state.player_x_fine != 0 {
        dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 12);
    }
}

fn dispatch_overhead_tile_at_offset(
    engine: &mut Engine,
    r: &mut RoutineContext,
    tile_ptr: i32,
    offset: i32,
) -> bool {
    r.offset = (offset as u8);
    match engine
        .state
        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS
    {
        5 => {
            run_character_select_room_flow(engine, r);
            engine.lotw_nonlocal_handoff = 1;
            true
        }
        4 => {
            run_shop_room_flow(engine, r);
            engine.lotw_nonlocal_handoff = 1;
            true
        }
        3 => {
            dispatch_four_fragment_overhead_tile(engine, r);
            true
        }
        _ => false,
    }
}

fn dispatch_four_fragment_overhead_tile(engine: &mut Engine, r: &mut RoutineContext) -> bool {
    let selected_slot = engine.state.selected_item_slot;
    if engine.state.item_slot((selected_slot as i32)) != 14 {
        return false;
    }

    let mut fragment_count = engine.state.fragment_count;
    for slot in 0..=2 {
        if engine.state.item_slot(slot) == 14 {
            fragment_count = ((fragment_count + 1) as u8);
        }
    }
    if fragment_count != 4 {
        return false;
    }

    enter_room_link_destination(engine, r);
    engine.lotw_nonlocal_handoff = 1;
    true
}

/// Checks the projected player footprint for room tile actions. The
/// original projection scratch is restored before returning so callers can
/// continue collision resolution with the same candidate position.
pub fn dispatch_projected_tile_actions(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_slot_ptr_lo = 144;
    engine.state.obj_slot_ptr_hi = 4;

    let saved_subtile_x = engine.state.indirect_ptr_lo;
    let saved_tile_x = engine.state.indirect_ptr_hi;
    let saved_pixel_y = engine.state.scratch2;

    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);

    let mut handled = dispatch_projected_tile_action_at_offset(engine, r, 0);
    if !handled && engine.state.indirect_ptr_lo != 0 {
        handled = dispatch_projected_tile_action_at_offset(engine, r, 12);
    }

    let projected_y = engine.state.scratch2;
    if !handled && projected_y < 176 && (projected_y & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
        handled = dispatch_projected_tile_action_at_offset(engine, r, 1);
        if !handled && engine.state.indirect_ptr_lo != 0 {
            handled = dispatch_projected_tile_action_at_offset(engine, r, 13);
        }
    }

    r.carry = (handled as u8);
    engine.state.scratch2 = saved_pixel_y;
    engine.state.indirect_ptr_hi = saved_tile_x;
    engine.state.indirect_ptr_lo = saved_subtile_x;
}

fn dispatch_projected_tile_action_at_offset(
    engine: &mut Engine,
    r: &mut RoutineContext,
    offset: i32,
) -> bool {
    r.offset = (offset as u8);
    dispatch_room_tile_action(engine, r);
    ((r.carry) != 0)
}

/// Converts tile-sample offset `0x0B` plus projected tile coordinates into
/// object scratch position `0xF9..0xFC`.
pub fn seed_object_position_from_tile_offset(engine: &mut Engine, r: &mut RoutineContext) {
    let mut tile_offset: i32 = (engine.state.scratch3 as i32);
    if (tile_offset >= 12) {
        tile_offset = ((tile_offset - 12) as u8 as i32);
        engine.state.indirect_ptr_hi =
            (engine.state.indirect_ptr_hi + 1) & ((crate::bits::BYTE_MASK) as u8);
    }
    if (tile_offset != 0) {
        engine.state.scratch2 = engine.state.scratch2 + 16;
    }
    engine.state.obj_y_pixel = engine.state.scratch2 & ((crate::bits::HIGH_NIBBLE) as u8);
    engine.state.obj_y_extra = 0;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_x_sub = 0;
    r.value = 0;
    r.offset = (tile_offset as u8);
}

/// Rebuilds the background column containing object scratch tile-x `0xFA`.
pub fn redraw_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_x: i32 = (engine.state.obj_x_tile as i32);
    engine.state.data_ptr_lo = (tile_x as u8);
    engine.state.vram_addr_lo = (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
    farcall_bank_09_r7(engine, r);
}

/// Reads the current room-map tile at `0x10/0x11 + 0x0B`. Tile `0x3E`
/// resolves to the current room replacement value in `0x74`.
pub fn read_room_tile_action_value(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_offset: i32 = (engine.state.scratch3 as i32);
    let room_ptr: i32 = (((engine.state.tile_fetch_counter as i32)
        | ((engine.state.aux_ptr_hi as i32) << 8)) as u16 as i32);
    let room_tile: i32 = engine.state.byte(((room_ptr + tile_offset) as u16 as i32));
    let tile_id: i32 = room_tile & crate::bits::LOW_6_BITS;
    r.index = (tile_id as u8);
    r.offset = (tile_offset as u8);
    if (tile_id == 62) {
        r.value = (engine.state.room_tile_action as u8);
    } else {
        r.value = (room_tile as u8);
    }
}

/// After a blocked move, attempts a one-pixel/subtile nudge toward the
/// nearest tile boundary unless the player is pressing away from it.
pub fn try_nudge_player_to_tile_boundary(engine: &mut Engine, r: &mut RoutineContext) {
    let horizontal_delta: i32 = (engine.state.horizontal_subtile_delta as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.horizontal_subtile_delta = 0;
                engine.state.player_x_velocity = 0;
                if (horizontal_delta == 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    let mut a: i32 =
                        ((engine.state.player_y & ((crate::bits::LOW_NIBBLE) as u8)) as u8 as i32);
                    if (a == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if (a < 6) {
                        if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) != 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.vertical_delta = 255;
                        engine.state.nudge_pending = 255;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (a >= 11) {
                        if ((engine.state.buttons & ((crate::bits::BIT3) as u8)) != 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.vertical_delta = 1;
                        engine.state.nudge_pending = 0;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                {
                    let mut v4B: i32 = (engine.state.vertical_delta as i32);
                    engine.state.vertical_delta = 0;
                    engine.state.nudge_pending = 0;
                    if (v4B == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    let mut a: i32 = (engine.state.player_x_fine as i32);
                    if (a == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if (a < 6) {
                        if ((engine.state.buttons & ((crate::bits::BIT0) as u8)) != 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.horizontal_subtile_delta = 15;
                        engine.state.player_x_velocity = 255;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (a >= 11) {
                        if ((engine.state.buttons & ((crate::bits::BIT1) as u8)) != 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.horizontal_subtile_delta = 1;
                        engine.state.player_x_velocity = 0;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                try_move_player_with_collision(engine, r);
                return;
                state = 3;
                continue 'dispatch;
            }
            3 => {
                r.carry = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Attempts to close the item menu, restore the pre-menu gameplay snapshot,
/// and redraw the HUD. Carry from the text/prompt helper aborts the close.
pub fn close_inventory_item_menu(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = 119;
    engine.state.indirect_ptr_hi = 181;
    decode_inventory_item_list_snapshot(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    engine.state.prompt_state = 16;
    restore_inventory_state_snapshot(engine, r);
    sync_key_hud(engine, r);
    sync_coin_hud(engine, r);
    engine.state.scroll_tile_x = 32;
    upload_staged_room_columns(engine, r);
    refresh_scroll_register_shadows(engine, r);
    restore_status_sprite_template(engine, r);
}

/// Selects the current 7x5 item-grid entry. Values `0x20..0x22` are menu
/// controls; normal values are copied into the scrolling item-list buffer.
pub fn select_inventory_grid_entry(engine: &mut Engine, r: &mut RoutineContext) {
    let grid_column: i32 = (engine.state.obj_x_vel_lo as i32);
    let mut grid_value: i32 = ((((grid_column << 2) as u8 as i32) + grid_column) as u8 as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                grid_value = ((grid_value + (engine.state.obj_y_vel as i32)) as u8 as i32);
                if (grid_value == 32) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (grid_value == 33) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (grid_value == 34) {
                    close_inventory_item_menu(engine, r);
                    return;
                }
                r.value = (grid_value as u8);
                set_inventory_list_buffer_index(engine, r);
                engine
                    .state
                    .set_password_nibbles_a((r.index as i32), grid_value);
                if (r.index == 31) {
                    close_inventory_item_menu(engine, r);
                    return;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.state.obj_x_sub =
                    (engine.state.obj_x_sub + 1) & ((crate::bits::BYTE_MASK) as u8);
                update_inventory_list_cursor_sprites(engine, r);
                return;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.obj_x_sub =
                    (engine.state.obj_x_sub - 1) & ((crate::bits::BYTE_MASK) as u8);
                update_inventory_list_cursor_sprites(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Moves the inventory grid cursor right across seven columns, wrapping to
/// column zero.
pub fn move_inventory_cursor_right(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_x_vel_lo + 1) as u8 as i32);
    if (x >= 7) {
        x = 0;
    }
    engine.state.obj_x_vel_lo = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor left across seven columns, wrapping to
/// column six.
pub fn move_inventory_cursor_left(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_x_vel_lo - 1) as u8 as i32);
    if ((x & crate::bits::BIT7) != 0) {
        x = 6;
    }
    engine.state.obj_x_vel_lo = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor up across five rows, wrapping to row
/// four.
pub fn move_inventory_cursor_up(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_y_vel - 1) as u8 as i32);
    if ((x & crate::bits::BIT7) != 0) {
        x = 4;
    }
    engine.state.obj_y_vel = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor down across five rows, wrapping to row
/// zero.
pub fn move_inventory_cursor_down(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_y_vel + 1) as u8 as i32);
    if (x >= 5) {
        x = 0;
    }
    engine.state.obj_y_vel = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Positions the two arrow sprites that point at the scrolling selected
/// item-list slot `0xF9 & crate::bits::LOW_5_BITS`.
pub fn update_inventory_list_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut list_slot: i32 = ((engine.state.obj_x_sub & ((crate::bits::LOW_5_BITS) as u8)) as i32);
    let mut cursor_tile: i32 = 97;
    if (list_slot >= 16) {
        list_slot = ((list_slot - 16) as u8 as i32);
        cursor_tile = 105;
    }
    engine.state.set_oam_y(128, cursor_tile);
    engine.state.set_oam_y(132, cursor_tile);
    engine.state.scratch0 = (list_slot as u8);

    let scaled_slot: i32 = (((list_slot >> 2) + list_slot) as u8 as i32);
    let carry: i32 = (((scaled_slot >> 5) & 1) as u8 as i32);
    let right_x: i32 = ((((scaled_slot << 3) as u8 as i32) + 54 + carry) as u8 as i32);
    engine.state.set_oam_x(132, right_x);
    let left_x: i32 = ((right_x - 8) as u8 as i32);
    engine.state.set_oam_x(128, left_x);
    r.index = (cursor_tile as u8);
    r.value = (left_x as u8);
}

fn scale_grid_coordinate(value: i32) -> (i32, i32) {
    (
        ((value << 3) as u8 as i32),
        (((value >> 5) & 1) as u8 as i32),
    )
}

/// Positions the 2x2 cursor around the active inventory grid cell.
pub fn update_inventory_grid_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let (column_pixels, column_carry) = scale_grid_coordinate((engine.state.obj_x_vel_lo as i32));
    let right_x: i32 = ((column_pixels + 54 + column_carry) as u8 as i32);
    engine.state.set_oam_x(148, right_x);
    let left_x: i32 = ((right_x - 8) as u8 as i32);
    engine.state.set_oam_x(144, left_x);

    let (row_pixels, row_carry) = scale_grid_coordinate((engine.state.obj_y_vel as i32));
    let y: i32 = ((row_pixels + 129 + row_carry) as u8 as i32);
    engine.state.set_oam_y(144, y);
    engine.state.set_oam_y(148, y);
    r.value = (y as u8);
}

/// Converts the scrolling item-list cursor into a 32-byte buffer index.
pub fn set_inventory_list_buffer_index(engine: &mut Engine, r: &mut RoutineContext) {
    r.index = ((engine.state.obj_x_sub & ((crate::bits::LOW_5_BITS) as u8)) as u8);
}

/// Pops a temporary-room checkpoint and rebuilds the gameplay room,
/// including the saved song, room graphics, sprites, and player pose.
pub fn restore_room_from_checkpoint(engine: &mut Engine, r: &mut RoutineContext) {
    pop_room_checkpoint(engine, r);
    fade_room_palette_out_reset_audio(engine, r);
    clear_temporary_room_sprites(engine, r);
    r.value = (engine.state.room_restore_scratch as u8);
    switch_song_if_needed(engine, r);
    prepare_room_metadata_and_palette(engine, r);
    upload_current_room_view(engine, r);
    draw_player_sprites(engine, r);
    draw_room_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    fade_room_palette_in(engine, r);
    update_player_pose_from_motion(engine, r);
    tick_player_walk_animation(engine, r);
}

/// Enters a temporary room page selected by `r.value`, using the full
/// transition fade that also resets active music channel state.
pub fn enter_temporary_room_page(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32);
    fade_room_palette_out_reset_audio(engine, r);
    engine.state.scratch0 = (a as u8);
    engine.state.map_screen_x = (((a & crate::bits::BITS_2_3) >> 2) as u8);
    engine.state.scroll_tile_x = (((a & crate::bits::LOW_2_BITS) << 4) as u8);
    engine.state.player_x_tile = engine.state.scroll_tile_x + 7;
    engine.state.map_screen_y = 16;
    engine.state.player_x_fine = 8;
    engine.state.player_y = 160;
    engine.state.jump_timer = 0;
    engine.state.fall_frames = 0;
    engine.state.scroll_fine_x = 0;
    clear_gameplay_object_sprites(engine, r);
    prepare_room_metadata_and_palette(engine, r);
    if (a == 4) {
        engine.state.tile_table_ptr_hi = 31 + 160;
    }
    upload_staged_room_view(engine, r);
    update_player_pose_from_motion(engine, r);
    draw_player_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
}

/// Rebuilds a temporary room page selected by `r.value` while preserving the
/// currently playing audio state.
pub fn refresh_temporary_room_page(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32);
    fade_room_palette_out_keep_audio(engine, r);
    engine.state.scratch0 = (a as u8);
    engine.state.map_screen_x = (((a & crate::bits::BITS_2_3) >> 2) as u8);
    engine.state.scroll_tile_x = (((a & crate::bits::LOW_2_BITS) << 4) as u8);
    engine.state.player_x_tile = engine.state.scroll_tile_x + 7;
    engine.state.map_screen_y = 16;
    engine.state.player_x_fine = 8;
    engine.state.player_y = 160;
    engine.state.jump_timer = 0;
    engine.state.fall_frames = 0;
    engine.state.scroll_fine_x = 0;
    clear_gameplay_object_sprites(engine, r);
    prepare_room_metadata_and_palette(engine, r);
    if (a == 4) {
        engine.state.tile_table_ptr_hi = 31 + 160;
    }
    upload_staged_room_view(engine, r);
    update_player_pose_from_motion(engine, r);
    draw_player_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
}

/// Draws the three carried-item slots from `0x51..0x53` into the temporary
/// room OAM area, hiding slots whose item id has the high bit set.
pub fn draw_carried_item_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    let mut y: i32 = 16;
    let mut a: i32 = 0;
    engine.state.scratch0 = 88;
    {
        x = 2;
        while (x >= 0) {
            let mut item: i32 = engine.state.item_slot(x);
            if ((item & crate::bits::BIT7) != 0) {
                a = 239;
            } else {
                let mut t: i32 = ((((item << 2) as u8 as i32) + 161) as u8 as i32);
                engine.state.set_oam_tile(64 + y, t);
                engine.state.set_oam_tile(68 + y, t + 2);
                a = 187;
            }
            engine.state.set_oam_y(64 + y, a);
            engine.state.set_oam_y(68 + y, a);
            engine
                .state
                .set_oam_x(64 + y, (engine.state.scratch0 as i32));
            engine
                .state
                .set_oam_x(68 + y, ((engine.state.scratch0 + 8) as i32));
            engine.state.scratch0 = ((((engine.state.scratch0 + 8) as u8 as i32) - 40) as u8);
            engine.state.set_oam_attr(64 + y, 1);
            engine.state.set_oam_attr(68 + y, 1);
            y = ((y - 8) as u8 as i32);
            {
                x -= 1;
                x
            };
        }
    }
    r.index = 255;
    r.offset = (y as u8);
}

/// Draws the two shop item slots from `0x80/0x82`; sold-out, unavailable,
/// or overstocked items are hidden and marked unavailable.
pub fn draw_shop_item_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    let mut a: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                a = 239;
                x = engine.state.temp_save(0);
                if ((x & crate::bits::BIT7) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.inventory_item(x) >= 11) {
                    engine.state.set_temp_save(0, 239);
                    a = 239;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                a = ((x << 2) as u8 as i32);
                a = ((a + 161) as u8 as i32);
                engine.state.set_oam_tile(64, a);
                a = ((a + 2) as u8 as i32);
                engine.state.set_oam_tile(68, a);
                engine.state.set_oam_x(64, 64);
                engine.state.set_oam_x(68, 72);
                a = 164;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.state.set_oam_y(64, a);
                engine.state.set_oam_y(68, a);
                engine.state.set_oam_attr(64, 1);
                engine.state.set_oam_attr(68, 1);
                a = 239;
                x = engine.state.temp_save(2);
                if ((x & crate::bits::BIT7) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (engine.state.inventory_item(x) >= 11) {
                    engine.state.set_temp_save(2, 239);
                    a = 239;
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                a = ((x << 2) as u8 as i32);
                a = ((a + 161) as u8 as i32);
                engine.state.set_oam_tile(72, a);
                a = ((a + 2) as u8 as i32);
                engine.state.set_oam_tile(76, a);
                engine.state.set_oam_x(72, 176);
                engine.state.set_oam_x(76, 184);
                a = 160;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.set_oam_y(72, a);
                engine.state.set_oam_y(76, a);
                engine.state.set_oam_attr(72, 1);
                engine.state.set_oam_attr(76, 1);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Draws the two-sprite coin/cost marker shared by shop and refill rooms.
pub fn draw_coin_cost_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_oam_y(80, 152);
    engine.state.set_oam_y(84, 152);
    engine.state.set_oam_tile(80, 241);
    engine.state.set_oam_tile(84, 243);
    engine.state.set_oam_attr(80, 2);
    engine.state.set_oam_attr(84, 2);
    engine.state.set_oam_x(80, 120);
    engine.state.set_oam_x(84, 128);
    r.value = 128;
}

/// Hides the temporary room item and coin/cost sprites in OAM.
pub fn clear_temporary_room_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_oam_y(64, 239);
    engine.state.set_oam_y(68, 239);
    engine.state.set_oam_y(72, 239);
    engine.state.set_oam_y(76, 239);
    engine.state.set_oam_y(80, 239);
    engine.state.set_oam_y(84, 239);
    r.value = 239;
}

/// Restores the fixed status/menu sprite template and its four PPU bank
/// shadow bytes after temporary inventory/status pages.
pub fn restore_status_sprite_template(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    {
        x = 55;
        while (x >= 0) {
            engine.state.set_oam_y(
                128 + x,
                engine.state.byte(((SPRITE_Y_TABLE_G + x) as u16 as i32)),
            );
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    engine.state.set_chr_bank(2, 52);
    engine.state.set_chr_bank(3, 53);
    engine.state.set_chr_bank(4, 54);
    engine.state.set_chr_bank(5, 55);
    r.index = 255;
    r.value = 55;
}

/// Spends one health point, returning carry set when health was already
/// empty.
pub fn consume_health_point(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = engine.state.player_health;
    if (r.value == 0) {
        r.carry = 1;
        return;
    }
    engine.state.player_health = engine.state.player_health - 1;
    sync_health_hud(engine, r);
    r.carry = 0;
}

/// Subtracts `r.value` health, saturating at zero. Carry is set when the
/// subtraction did not underflow.
pub fn subtract_health_points(engine: &mut Engine, r: &mut RoutineContext) {
    let damage: i32 = (r.value as u8 as i32);
    engine.state.scratch0 = (damage as u8);
    let health: i32 = engine.state.player_health as i32;
    let enough_health: i32 = ((health >= damage) as u8 as i32);
    if ((enough_health) != 0) {
        engine.state.player_health = (health - damage) as u8;
    } else {
        engine.state.player_health = 0;
    }
    sync_health_hud(engine, r);
    r.carry = (enough_health as u8);
}

/// Spends one magic point and preserves the caller's `r.index`. Carry is
/// set when no magic was available.
pub fn consume_magic_point(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_index: i32 = (r.index as u8 as i32);
    r.value = engine.state.player_magic;
    r.carry = 1;
    if (engine.state.player_magic != 0) {
        engine.state.player_magic = engine.state.player_magic - 1;
        sync_magic_hud(engine, r);
        r.carry = 0;
    }
    r.index = (saved_index as u8);
}

/// Adds `r.value` health and clamps it to the HUD/resource maximum.
pub fn add_health_points(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + engine.state.player_health as i32) as i32);
    let capped_total: i32 = if (total > 255) {
        109
    } else if ((total as u8 as i32) >= 110) {
        109
    } else {
        (total as u8 as i32)
    };
    engine.state.player_health = capped_total as u8;
    sync_health_hud(engine, r);
}

/// Adds `r.value` magic and clamps it to the HUD/resource maximum.
pub fn add_magic_points(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + engine.state.player_magic as i32) as i32);
    let capped_total: i32 = if (total > 255) {
        109
    } else if ((total as u8 as i32) >= 110) {
        109
    } else {
        (total as u8 as i32)
    };
    engine.state.player_magic = capped_total as u8;
    sync_magic_hud(engine, r);
}

/// Adds `r.value` coins and clamps them to the HUD/resource maximum.
pub fn add_coins(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + (engine.state.coins as i32)) as u8 as i32);
    let capped_total: i32 = if (total > 255) {
        109
    } else if ((total as u8 as i32) >= 110) {
        109
    } else {
        (total as u8 as i32)
    };
    engine.state.coins = (capped_total as u8);
    sync_coin_hud(engine, r);
}

/// Spends `r.value` coins. Carry is set on success and clear when the
/// player cannot afford the cost.
pub fn spend_coins(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch0 = (r.value as u8);
    let remaining_coins: i32 =
        (engine.state.coins as u16 as i32) - (engine.state.scratch0 as u16 as i32);
    r.value = (remaining_coins as u8);
    if ((remaining_coins & crate::bits::BIT8) != 0) {
        r.carry = 0;
        return;
    }
    engine.state.coins = (r.value as u8);
    sync_coin_hud(engine, r);
    r.carry = 1;
}

/// Adds one key and refreshes the key HUD digits.
pub fn add_key(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.keys = engine.state.keys + 1;
    sync_key_hud(engine, r);
    r.carry = 0;
}

/// Adds `r.value` keys and clamps them to the HUD/resource maximum.
pub fn add_keys(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + (engine.state.keys as i32)) as u8 as i32);
    let capped_total: i32 = if (total > 255) {
        109
    } else if ((total as u8 as i32) >= 110) {
        109
    } else {
        (total as u8 as i32)
    };
    engine.state.keys = (capped_total as u8);
    sync_key_hud(engine, r);
}

/// Spends one key, returning carry set when no key was available.
pub fn consume_key(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = (engine.state.keys as u8);
    if (r.value == 0) {
        r.carry = 1;
        return;
    }
    engine.state.keys = engine.state.keys - 1;
    sync_key_hud(engine, r);
    r.carry = 0;
}

// Updates live room objects by copying each 16-byte object slot into
// scratch RAM `0xED..0xFC`, running the correct actor state path, then
// copying the scratch state back to the slot.
pub fn update_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.map_screen_y == 16) {
                    return;
                }
                if (engine.state.chr_bank(3) >= 48) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    let mut scheduler_phase: i32 = (engine.state.scheduler_phase as i32);
                    let mut first_actor_slot: i32 =
                        (((scheduler_phase << 1) + scheduler_phase) as u8 as i32);
                    engine.state.slot_index = (first_actor_slot as u8);
                    engine.state.slot_index_limit = ((first_actor_slot + 3) as u8);
                    let mut object_slot_lo: i32 =
                        (((engine.state.slot_index as i32) << 4) as u8 as i32);
                    engine.state.obj_slot_ptr_lo = (object_slot_lo as u8);
                    engine.state.actor_record_ptr_lo = ((object_slot_lo + 32) as u8);
                    engine.state.obj_slot_ptr_hi = 4;
                    engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                }
                loop {
                    let mut actor_state: i32 = 0;
                    load_object_slot_scratch(engine, r);
                    actor_state = (engine.state.obj_state as i32);
                    if (actor_state == 0) {
                        tick_inactive_actor_slot(engine, r);
                    } else if ((actor_state & crate::bits::BIT7) != 0) {
                        tick_defeated_actor_reward_drop(engine, r);
                    } else if (actor_state == 1) {
                        dispatch_actor_behavior(engine, r);
                    } else if (actor_state >= 24) {
                        tick_actor_materialize_delay(engine, r);
                    } else {
                        tick_standard_actor(engine, r);
                    }
                    store_object_slot_scratch(engine, r);
                    engine.state.slot_index =
                        (engine.state.slot_index + 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.obj_slot_ptr_lo = engine.state.obj_slot_ptr_lo + 16;
                    engine.state.actor_record_ptr_lo =
                        ((engine.state.actor_record_ptr_lo + 16) as u8);
                    if !(engine.state.slot_index < engine.state.slot_index_limit) {
                        break;
                    }
                }
                {
                    let mut next_scheduler_phase: i32 =
                        ((engine.state.scheduler_phase + 1) as u8 as i32);
                    engine.state.scheduler_phase = (if (next_scheduler_phase >= 3) {
                        0
                    } else {
                        (next_scheduler_phase as u8)
                    });
                }
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.scheduler_phase & ((crate::bits::BIT0) as u8)) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_slot_ptr_lo = 0;
                engine.state.obj_slot_ptr_hi = 4;
                engine.state.slot_index = 0;
                engine.state.actor_record_ptr_lo = 32;
                engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                load_object_slot_scratch(engine, r);
                {
                    let mut actor_state: i32 = (engine.state.obj_state as i32);
                    if (actor_state == 0) {
                        initialize_large_actor_slot(engine, r);
                    } else if ((actor_state & crate::bits::BIT7) != 0) {
                        update_large_actor_facing_from_velocity(engine, r);
                        animate_large_actor_body_tiles(engine, r);
                    } else {
                        tick_large_chasing_actor(engine, r);
                    }
                }
                store_object_slot_scratch(engine, r);
                compose_large_actor_body_slots(engine, r);
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.slot_index = 4;
                engine.state.obj_slot_ptr_lo = 64;
                engine.state.obj_slot_ptr_hi = 4;
                engine.state.actor_record_ptr_lo = 96;
                engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                loop {
                    let mut actor_state: i32 = 0;
                    load_object_slot_scratch(engine, r);
                    actor_state = (engine.state.obj_state as i32);
                    if ((actor_state == 0) || ((actor_state & crate::bits::BIT7) != 0)) {
                        engine.state.obj_state = 0;
                        maybe_spawn_pursuer_actor(engine, r);
                    } else {
                        dispatch_actor_behavior(engine, r);
                    }
                    store_object_slot_scratch(engine, r);
                    engine.state.slot_index =
                        (engine.state.slot_index + 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.obj_slot_ptr_lo = engine.state.obj_slot_ptr_lo + 16;
                    engine.state.actor_record_ptr_lo =
                        ((engine.state.actor_record_ptr_lo + 16) as u8);
                    if !(engine.state.slot_index < 9) {
                        break;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                engine.state.scheduler_phase =
                    engine.state.scheduler_phase ^ ((crate::bits::BIT0) as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Copies the object slot addressed by `0xE5..0xE6` into scratch RAM
/// `0xED..0xFC`.
pub fn load_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
    let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
    for slot_offset in (0..=15).rev() {
        engine.state.set_obj_scratch_byte(
            slot_offset,
            engine.state.byte(((slot_ptr + slot_offset) as u16 as i32)),
        );
    }
    r.offset = 255;
}

/// Writes scratch RAM `0xED..0xFC` back to the object slot addressed by
/// `0xE5..0xE6`.
pub fn store_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
    let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
    for slot_offset in (0..=15).rev() {
        engine.state.set_byte(
            ((slot_ptr + slot_offset) as u16 as i32),
            engine.state.obj_scratch_byte(slot_offset),
        );
    }
    r.offset = 255;
}

// Initializes an inactive scratch slot from the room actor record at
// `0xE7..0xE8`. A nonzero timer leaves the actor materializing; a zero
// timer promotes it to the normal active state.
pub fn tick_inactive_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_timer = engine.state.obj_timer - 1;
    let actor_timer: i32 = (engine.state.obj_timer as i32);
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    if ((engine.state.byte(((actor_data_ptr + 2) as u16 as i32))
        | engine.state.byte(((actor_data_ptr + 3) as u16 as i32)))
        == 0)
    {
        r.value = 12;
        rng_update(engine, r);
        engine.state.scratch2 = (((r.value as i32) << 4) as u8);
        r.value = 64;
        rng_update(engine, r);
        engine.state.indirect_ptr_hi = (r.value as u8);
    } else {
        engine.state.scratch2 = ((engine.state.byte(((actor_data_ptr + 3) as u16 as i32))) as u8);
        engine.state.indirect_ptr_hi =
            ((engine.state.byte(((actor_data_ptr + 2) as u16 as i32))) as u8);
    }
    engine.state.indirect_ptr_lo = 0;
    engine.state.scratch3 = 0;
    check_player_overlap(engine, r);
    if ((r.carry) != 0) {}
    check_projected_terrain_collision(engine, r);
    if ((r.carry) != 0) {}
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    engine.state.obj_y_extra = 0;
    engine.state.obj_health = ((engine.state.byte(((actor_data_ptr + 4) as u16 as i32))) as u8);
    engine.state.obj_damage = ((engine.state.byte(((actor_data_ptr + 5) as u16 as i32))) as u8);
    {
        let mut current_member_bit: i32 = 0;
        let mut carry_bit: i32 = 1;
        let mut member_index: i32 = (engine.state.character_index as i32);
        loop {
            let mut next_carry_bit: i32 = (((current_member_bit >> 7) & 1) as u8 as i32);
            current_member_bit = (((current_member_bit << 1) | carry_bit) as u8 as i32);
            carry_bit = next_carry_bit;
            member_index = ((member_index - 1) as u8 as i32);
            if !((member_index & crate::bits::BIT7) == 0) {
                break;
            }
        }
        current_member_bit =
            ((current_member_bit & (engine.state.family_member_mask as i32)) as u8 as i32);
        if (current_member_bit == 0) {
            let mut contact_damage: i32 = (engine.state.obj_damage as i32);
            let mut damage_overflow: i32 = (((contact_damage >> 7) & 1) as u8 as i32);
            engine.state.obj_damage = ((contact_damage << 1) as u8);
            if ((damage_overflow) != 0) {
                engine.state.obj_damage = 255;
            }
        }
    }
    engine.state.obj_state = 127;
    engine.state.obj_tile = 249;
    engine.state.obj_attr = 1;
    if (actor_timer == 0) {
        engine.state.obj_state = 1;
        engine.state.obj_tile = ((engine.state.byte(((actor_data_ptr + 0) as u16 as i32))) as u8);
        engine.state.obj_attr = ((engine.state.byte(((actor_data_ptr + 1) as u16 as i32))) as u8);
    } else {
        if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
            engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
        }
    }
}

// Counts down a materializing actor. When the timer reaches zero, the slot
// becomes behavior-dispatched state `0x01` with sprite bytes from room data.
pub fn tick_actor_materialize_delay(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_timer: i32 = ((engine.state.obj_timer - 1) as u8 as i32);
    engine.state.obj_timer = (actor_timer as u8);
    if (actor_timer == 0) {
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        engine.state.obj_state = 1;
        engine.state.obj_tile = ((engine.state.byte(actor_data_ptr)) as u8);
        engine.state.obj_attr = ((engine.state.byte(((actor_data_ptr + 1) as u16 as i32))) as u8);
    } else if ((actor_timer & crate::bits::LOW_2_BITS) == 0) {
        engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
    }
}

// Some late-game rooms periodically seed extra actors from the player slot.
// The 1-in-30 roll keeps empty secondary slots from respawning every frame.
pub fn maybe_spawn_pursuer_actor(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 30;
    rng_update(engine, r);
    if (r.value != 0) {
        r.index = r.value;
        return;
    }
    r.index = 0;
    let mut scratch_offset: i32 = 3;
    let mut source_slot_offset: i32 = 3;
    if ((engine.state.object_attr(0) & crate::bits::BIT6) != 0) {
        source_slot_offset = 19;
    }
    loop {
        engine.state.set_inventory_item(
            153 + scratch_offset,
            engine.state.object_x_sub(source_slot_offset),
        );
        source_slot_offset = ((source_slot_offset - 1) as u8 as i32);
        if (({
            let __old = scratch_offset;
            scratch_offset -= 1;
            __old
        }) == 0)
        {
            break;
        }
    }
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    engine.state.obj_health = ((engine.state.byte(((actor_data_ptr + 4) as u16 as i32))) as u8);
    engine.state.obj_damage = ((engine.state.byte(((actor_data_ptr + 5) as u16 as i32))) as u8);
    engine.state.obj_state = 1;
    engine.state.obj_tile = 129;
    r.value = 4;
    rng_update(engine, r);
    engine.state.obj_attr = (r.value as u8);
    engine.state.obj_cooldown = 128;
    r.offset = (source_slot_offset as u8);
    r.index = (scratch_offset as u8);
}

const ACTOR_BEHAVIOR_HANDLERS: [i32; 9] = [
    0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
];

// Dispatches the behavior id stored at room actor data byte 8. The original
// handler address is mirrored into 0x0E/0x0F for trace-compatible scratch.
pub fn dispatch_actor_behavior(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    let mut behavior_id: i32 = engine.state.byte(((actor_data_ptr + 8) as u16 as i32));
    if (behavior_id >= 9) {
        behavior_id = 0;
    }
    engine.state.indirect_ptr_lo =
        ((ACTOR_BEHAVIOR_HANDLERS[behavior_id as usize] & crate::bits::BYTE_MASK) as u8);
    engine.state.indirect_ptr_hi = ((ACTOR_BEHAVIOR_HANDLERS[behavior_id as usize] >> 8) as u8);
    match behavior_id {
        0 => {
            tick_wandering_jump_actor(engine, r);
        }
        1 => {
            tick_random_floating_actor(engine, r);
        }
        2 => {
            tick_ledge_walking_actor(engine, r);
        }
        3 => {
            tick_chasing_jump_actor(engine, r);
        }
        4 => {
            tick_reflecting_chase_actor(engine, r);
        }
        5 => {
            tick_overhead_probe_actor(engine, r);
        }
        6 => {
            tick_contact_trigger_actor(engine, r);
        }
        7 => {
            tick_contact_recoil_actor(engine, r);
        }
        8 => {
            tick_timed_chase_actor(engine, r);
        }
        _ => {}
    }
}

// Generic non-boss actor tick: keep existing movement going, try terrain
// response, expire the actor when its timer reaches zero, then update the
// terrain probe for the next frame.
pub fn tick_standard_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_timer: i32 = 0;
    let mut saved_tile_y: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_scratch == 0) {
                    if (engine.state.obj_cooldown == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    try_actor_jump_arc_motion(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    commit_actor_projected_position(engine, r);
                }
                try_actor_gravity_motion(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                commit_actor_projected_position(engine, r);
                state = 1;
                continue 'dispatch;
            }
            1 => {
                actor_timer = ((engine.state.obj_timer - 1) as u8 as i32);
                if (actor_timer == 0) {
                    engine.state.obj_state = 0;
                    engine.state.obj_timer = 240;
                    r.index = (actor_timer as u8);
                    return;
                }
                engine.state.obj_timer = (actor_timer as u8);
                if (actor_timer < 60) {
                    actor_timer = 239;
                    saved_tile_y = (engine.state.obj_y_pixel as i32);
                    if (saved_tile_y == 239) {
                        actor_timer = (engine.state.obj_y_extra as i32);
                    }
                    engine.state.obj_y_pixel = (actor_timer as u8);
                    engine.state.obj_y_extra = (saved_tile_y as u8);
                }
                update_object_terrain_probe(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Wanders horizontally, occasionally starts a jump arc, then falls under
// the shared gravity helper until terrain accepts the projected position.
pub fn tick_wandering_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_tile_dx: i32 = 0;
    let mut keep_existing_motion: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_timer >= 32) {
                } else if (engine.state.obj_cooldown != 0) {
                    keep_existing_motion = 1;
                } else if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) != 0) {
                    keep_existing_motion = 1;
                }
                if ((keep_existing_motion) == 0) {
                    engine.state.obj_timer = 0;
                    choose_random_cardinal_actor_direction(engine, r);
                    r.value = 6;
                    rng_update(engine, r);
                    engine.state.obj_x_vel_hi = ((r.value + 1) as u8);
                    r.value = 4;
                    rng_update(engine, r);
                    r.index = r.value;
                    if (r.value == 0) {
                        engine.state.obj_move_state = 128 | engine.state.obj_move_state;
                    }
                }
                saved_tile_dx = (engine.state.obj_x_vel_hi as i32);
                r.offset = (engine.state.obj_x_vel_hi as u8);
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    try_actor_gravity_motion(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_cooldown != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::BIT7) as u8)) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                try_actor_jump_arc_motion(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.obj_cooldown = 0;
                try_move_actor_with_terrain(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                stop_actor_motion(engine, r);
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                commit_actor_projected_position(engine, r);
                state = 4;
                continue 'dispatch;
            }
            4 => {
                update_object_terrain_probe(engine, r);
                update_actor_animation(engine, r);
                engine.state.obj_x_vel_hi = (saved_tile_dx as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Chooses a random direction when stationary, then moves without terrain
// collision. Bounds/player contact can stop the motion.
pub fn tick_random_floating_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        choose_random_actor_direction(engine, r);
    }
    {
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        let mut speed: i32 = engine.state.byte(((actor_data_ptr + 9) as u16 as i32));
        r.offset = (speed as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        stop_actor_motion(engine, r);
    } else {
        commit_actor_projected_position(engine, r);
    }
    update_actor_animation(engine, r);
}

// Walks along terrain ledges: blocked movement stops motion, supported
// projections commit, and unsupported projections fall through gravity.
pub fn tick_ledge_walking_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut should_commit_position: i32 = 0;
    let mut should_stop_motion: i32 = 0;
    let mut skip_resolution: i32 = 0;
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        reverse_actor_horizontal_direction(engine, r);
    }
    if (engine.state.obj_move_scratch != 0) {
        try_actor_gravity_motion(engine, r);
        if (r.carry == 0) {
            should_commit_position = 1;
        } else {
            skip_resolution = 1;
        }
    } else {
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
        try_move_actor_with_terrain(engine, r);
        if ((r.carry) != 0) {
            should_stop_motion = 1;
        } else {
            r.offset = 1;
            probe_object_solid_tile(engine, r);
            if (r.carry == 0) {
                should_stop_motion = 1;
            } else if (engine.state.indirect_ptr_lo == 0) {
                should_commit_position = 1;
            } else {
                r.offset = 13;
                probe_object_solid_tile(engine, r);
                if (r.carry == 0) {
                    should_stop_motion = 1;
                } else {
                    should_commit_position = 1;
                }
            }
        }
    }
    if ((skip_resolution) == 0) {
        if ((should_stop_motion) != 0) {
            stop_actor_motion(engine, r);
        } else if ((should_commit_position) != 0) {
            commit_actor_projected_position(engine, r);
        }
    }
    update_object_terrain_probe(engine, r);
    update_actor_animation(engine, r);
}

// Re-aims toward the player, marks the direction as jump-capable with
// `0x80`, then uses the same jump/gravity movement path as wanderers.
pub fn tick_chasing_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.obj_move_state =
                    engine.state.obj_move_state & ((crate::bits::LOW_NIBBLE) as u8);
                if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) != 0) {
                    if (engine.state.obj_timer < 16) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_x_sub == 0) {
                    let mut room_tile_ptr: i32 = 0;
                    engine.state.data_ptr_lo = engine.state.obj_x_tile;
                    engine.state.data_ptr_hi = engine.state.obj_y_pixel;
                    resolve_room_tile_pointer(engine, r);
                    room_tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
                    if ((engine.state.byte(room_tile_ptr) & crate::bits::LOW_6_BITS) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if ((engine.state.byte(((room_tile_ptr + 1) as u16 as i32))
                        & crate::bits::LOW_6_BITS)
                        == 0)
                    {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                    engine.state.obj_move_state = 1;
                }
                {
                    let mut turn_timer: i32 = ((engine.state.obj_timer - 1) as u8 as i32);
                    engine.state.obj_timer = 0;
                    if (turn_timer == 0) {
                        if ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) == 0)
                        {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        engine.state.obj_move_state =
                            engine.state.obj_move_state ^ ((crate::bits::LOW_2_BITS) as u8);
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                }
                aim_actor_toward_player(engine, r);
                engine.state.obj_move_state = 128 | engine.state.obj_move_state;
                {
                    state = 2;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.state.obj_timer = 0;
                aim_actor_toward_player(engine, r);
                state = 2;
                continue 'dispatch;
            }
            2 => {
                {
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
                }
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    try_actor_gravity_motion(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_cooldown != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::BIT7) as u8)) == 0) {
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                try_actor_jump_arc_motion(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                engine.state.obj_cooldown = 0;
                try_move_actor_with_terrain(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                stop_actor_motion(engine, r);
                {
                    state = 6;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                commit_actor_projected_position(engine, r);
                state = 6;
                continue 'dispatch;
            }
            6 => {
                update_object_terrain_probe(engine, r);
                update_actor_animation(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Aims from player overlap, moves without terrain, and asks the velocity
// reflection helper to bounce away when blocked.
pub fn tick_reflecting_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut keep_current_direction: i32 = ((((engine.state.obj_x_vel_lo | engine.state.obj_y_vel)
        != 0)
        && (engine.state.obj_timer < 32)) as u8 as i32);
    if ((keep_current_direction) == 0) {
        aim_actor_from_player_overlap(engine, r);
    }
    {
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        try_reflect_object_velocity(engine, r);
        if ((r.carry) != 0) {
            stop_actor_motion(engine, r);
            update_actor_animation(engine, r);
            return;
        }
    }
    commit_actor_projected_position(engine, r);
    update_actor_animation(engine, r);
}

// Alternates between overhead probing, falling, and a jump arc. This is the
// only normal behavior that asks `probe_actor_overhead_step` before moving.
pub fn tick_overhead_probe_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_scratch != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_cooldown != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
                engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
                engine.state.scratch2 = engine.state.obj_y_pixel;
                probe_actor_overhead_step(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
                engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
                    reverse_actor_horizontal_direction(engine, r);
                }
                check_player_x_overlap(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                {
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
                }
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                try_move_actor_with_terrain(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                probe_actor_overhead_step(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                engine.state.obj_x_vel_lo = 0;
                engine.state.obj_x_vel_hi = 0;
                update_object_terrain_probe(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                try_actor_gravity_motion(engine, r);
                commit_actor_projected_position(engine, r);
                {
                    let mut saved_fall_counter: i32 = (engine.state.obj_move_scratch as i32);
                    update_object_terrain_probe(engine, r);
                    if ((r.carry) == 0) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    engine.state.obj_cooldown = ((saved_fall_counter + 5 + 1) as u8);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                commit_actor_projected_position(engine, r);
                {
                    state = 7;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                try_actor_jump_arc_motion(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                commit_actor_projected_position(engine, r);
                {
                    state = 7;
                    continue 'dispatch;
                }
                state = 6;
                continue 'dispatch;
            }
            6 => {
                stop_actor_motion(engine, r);
                state = 7;
                continue 'dispatch;
            }
            7 => {
                update_actor_animation(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Sits inert until the player overlaps a one-step projection in any
// cardinal direction, then switches into the chasing jump behavior.
pub fn tick_contact_trigger_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_state != 0) {
                    tick_chasing_jump_actor(engine, r);
                    return;
                }
                r.value = 1;
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 2;
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 4;
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 8;
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    let mut actor_type: i32 =
                        engine.state.byte(((actor_data_ptr + 4) as u16 as i32));
                    engine.state.obj_health = (actor_type as u8);
                    r.value = 0;
                    engine.state.obj_y_extra = 0;
                }
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                r.value = 1;
                engine.state.obj_move_state = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Projects the one-step direction in `r.value` and reports player contact.
pub fn check_actor_direction_contact(engine: &mut Engine, r: &mut RoutineContext) {
    r.offset = 1;
    build_direction_velocity(engine, r);
    project_actor_position(engine, r);
    check_player_overlap(engine, r);
    if (r.carry == 0) {
        return;
    }
    apply_actor_player_contact_damage(engine, r);
    r.carry = 1;
}

// Random floating behavior that turns into a high-bit/contact recoil state
// when movement was blocked specifically by player overlap.
pub fn tick_contact_recoil_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        choose_random_actor_direction(engine, r);
    }
    {
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        if (engine.state.overlap_flag != 0) {
            r.value = 128;
            engine.state.obj_state = 128;
            return;
        }
        stop_actor_motion(engine, r);
    } else {
        commit_actor_projected_position(engine, r);
    }
    update_actor_animation(engine, r);
}

// Chases for `0xF1` ticks. Once it has a direction, abrupt multi-axis
// changes are rejected unless the timer has settled for several frames.
pub fn tick_timed_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut chase_timer: i32 = ((engine.state.obj_cooldown - 1) as u8 as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.obj_cooldown = (chase_timer as u8);
                if (chase_timer == 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_move_state == 0) {
                    aim_actor_from_player_overlap(engine, r);
                } else {
                    if (engine.state.obj_timer >= 8) {
                        let mut direction_diff: i32 = 0;
                        let mut bit_count: i32 = 0;
                        let mut changed_bits: i32 = 0;
                        engine.state.scratch0 = engine.state.obj_move_state;
                        aim_actor_from_player_overlap(engine, r);
                        direction_diff =
                            ((engine.state.obj_move_state ^ engine.state.scratch0) as u8 as i32);
                        changed_bits = 0;
                        bit_count = 4;
                        loop {
                            let mut bit: i32 = direction_diff & 1;
                            direction_diff >>= 1;
                            if ((bit) != 0) {
                                {
                                    let __old = changed_bits;
                                    changed_bits += 1;
                                    __old
                                };
                            }
                            if ({
                                bit_count -= 1;
                                bit_count
                            } == 0)
                            {
                                break;
                            }
                        }
                        {
                            let __old = changed_bits;
                            changed_bits -= 1;
                            __old
                        };
                        if (changed_bits != 0) {
                            engine.state.obj_move_state = engine.state.scratch0;
                        }
                    }
                }
                {
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte(((actor_data_ptr + 9) as u16 as i32))) as u8);
                    r.value = (engine.state.obj_move_state as u8);
                    build_direction_velocity(engine, r);
                }
                try_move_actor_without_terrain(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                commit_actor_projected_position(engine, r);
                update_actor_animation(engine, r);
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                r.value = 0;
                engine.state.obj_state = 0;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Probes the projected tile one row above the actor when the projected Y
// position is tile-aligned. Carry is left from the solid-tile probe.
pub fn probe_actor_overhead_step(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) != 0) {
        return;
    }
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2 - 16;
    resolve_room_tile_pointer(engine, r);
    r.offset = 0;
    probe_projected_solid_tile(engine, r);
    if (r.carry == 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    r.offset = 12;
    probe_projected_solid_tile(engine, r);
    if (r.carry == 0) {
        return;
    }
}

// Sets direction bits in `0xF4` so an actor tends toward the player. Room
// actor data byte 9 allows occasional upward bias when the actor is below.
pub fn aim_actor_toward_player(engine: &mut Engine, r: &mut RoutineContext) {
    let mut direction_bits: i32 = 0;
    let mut dx: i32 = (((engine.state.obj_x_tile as u16 as i32)
        - (engine.state.player_x_tile as i32)) as u16 as i32);
    if ((dx as u8 as i32) != 0) {
        {
            direction_bits += 1;
            direction_bits
        };
        if ((dx & crate::bits::BIT8) == 0) {
            {
                direction_bits += 1;
                direction_bits
            };
        }
    }
    engine.state.obj_move_state = (direction_bits as u8);
    {
        let mut dy: i32 = (((engine.state.obj_y_pixel as u16 as i32)
            - (engine.state.player_y as i32)) as u16 as i32);
        if ((dy & crate::bits::BIT8) == 0) {
            let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
            let mut vertical_bias_enabled: i32 =
                engine.state.byte(((actor_data_ptr + 9) as u16 as i32));
            if (vertical_bias_enabled != 0) {
                r.value = 3;
                rng_update(engine, r);
                r.index = r.value;
                if (r.index == 0) {
                    engine.state.obj_move_state =
                        engine.state.obj_move_state | ((crate::bits::BIT7) as u8);
                }
            }
        } else {
            r.value = 3;
            rng_update(engine, r);
            r.index = r.value;
            if (r.index == 0) {
                engine.state.obj_move_state = 4;
            }
        }
    }
}

// Builds direction bits by checking whether the actor already overlaps the
// player on each axis.
pub fn aim_actor_from_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut direction_bits: i32 = 0;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    check_player_x_overlap(engine, r);
    direction_bits = 0;
    if (r.carry == 0) {
        let mut actor_is_right_of_player: i32 =
            ((if (engine.state.obj_x_tile >= engine.state.player_x_tile) {
                1
            } else {
                0
            }) as u8 as i32);
        direction_bits = 1;
        if ((actor_is_right_of_player) != 0) {
            direction_bits = 2;
        }
    }
    engine.state.obj_move_state = (direction_bits as u8);
    check_player_y_overlap(engine, r);
    direction_bits = 0;
    if (r.carry == 0) {
        let mut actor_is_below_player: i32 =
            ((if (engine.state.obj_y_pixel >= engine.state.player_y) {
                1
            } else {
                0
            }) as u8 as i32);
        direction_bits = 4;
        if ((actor_is_below_player) != 0) {
            direction_bits = 8;
        }
    }
    engine.state.obj_move_state = ((direction_bits | (engine.state.obj_move_state as i32)) as u8);
    engine.state.obj_timer = 0;
}

pub fn reverse_actor_horizontal_direction(engine: &mut Engine, r: &mut RoutineContext) {
    let mut direction_bits: i32 =
        ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) as i32);
    if (direction_bits == 0) {
        direction_bits = 1;
    }
    direction_bits ^= 3;
    engine.state.obj_move_state = (direction_bits as u8);
    r.value = (direction_bits as u8);
}

// Chooses one of the eight direction-bit patterns in the original table.
pub fn choose_random_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 8;
    rng_update(engine, r);
    r.index = r.value;
    engine.state.obj_move_state = ((engine
        .state
        .byte(((OBJ_MOVE_STATE_TABLE + (r.index as i32)) as u16 as i32)))
        as u8);
}

const DIRECTION_LOOKUP: [i32; 8] = [1, 5, 4, 6, 2, 10, 8, 9];

// Chooses from every other entry in the direction table, giving a smaller
// cardinal-ish set used by wandering actors.
pub fn choose_random_cardinal_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 3;
    rng_update(engine, r);
    let direction_index: i32 = (((r.value as i32) << 1) as u8 as i32);
    engine.state.obj_move_state = ((DIRECTION_LOOKUP[direction_index as usize]) as u8);
}

// Advances a falling actor. If the projected move is blocked, horizontal
// velocity is dropped and the move is retried before vertical motion stops.
pub fn try_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 1) + 2;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_y_vel = 0;
}

// Uses `0xF1` as a jump-arc countdown and converts it into upward velocity.
pub fn try_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut jump_counter: i32 = (engine.state.obj_cooldown as i32);
    if (jump_counter == 0) {
        jump_counter = 15;
    }
    jump_counter = ((jump_counter - 1) as u8 as i32);
    engine.state.obj_cooldown = (jump_counter as u8);
    r.index = (jump_counter as u8);
    engine.state.obj_y_vel = ((((jump_counter >> 1) ^ crate::bits::BYTE_MASK) + 1) as u8);
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_cooldown = engine.state.obj_cooldown + 1;
    try_reflect_object_velocity(engine, r);
}

// Commits projected actor position `0x0E/0x0F/0x0A` back to actor scratch.
pub fn commit_actor_projected_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    r.value = (engine.state.scratch2 as u8);
}

// Clears actor velocity and arc/probe counters.
pub fn stop_actor_motion(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_y_vel = 0;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
}

// Projects current actor scratch position `0xF9..0xFB` through velocity
// `0xF5..0xF7`, leaving the projected position in `0x0E/0x0F/0x0A`.
pub fn project_actor_position(engine: &mut Engine, r: &mut RoutineContext) {
    let mut subtile_dx: i32 = 0;
    let mut subtile_sum: i32 = 0;
    let mut tile_carry: i32 = 0;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    if (engine.state.obj_y_vel != 0) {
        engine.state.scratch2 = engine.state.obj_y_vel + engine.state.scratch2;
    }
    subtile_dx = (engine.state.obj_x_vel_lo as i32);
    if (subtile_dx != 0) {
        subtile_sum = ((subtile_dx + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((subtile_sum & crate::bits::LOW_NIBBLE) as u8);
        tile_carry = (((subtile_sum >> 4) & 1) as u8 as i32);
        engine.state.indirect_ptr_hi =
            engine.state.indirect_ptr_hi + engine.state.obj_x_vel_hi + (tile_carry as u8);
    }
}

const ANIMATION_HANDLERS: [i32; 4] = [0xF03B, 0xF04B, 0xF071, 0xF0B9];

// Dispatches the animation mode stored in room actor data byte 7.
pub fn update_actor_animation(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    let mut animation_id: i32 = ((engine.state.byte(((actor_data_ptr + 7) as u16 as i32))
        & crate::bits::LOW_2_BITS) as u8 as i32);
    let mut original_handler: i32 = ANIMATION_HANDLERS[animation_id as usize];
    engine.state.indirect_ptr_lo = ((original_handler & crate::bits::BYTE_MASK) as u8);
    engine.state.indirect_ptr_hi = ((original_handler >> 8) as u8);
    r.offset = 7;
    r.index = ((animation_id << 1) as u8);
    r.value = ((animation_id << 1) as u8);
    match animation_id {
        0 => {
            animate_actor_flip_toggle(engine, r);
        }
        1 => {
            animate_actor_walk_toggle(engine, r);
        }
        2 => {
            animate_actor_directional_walk(engine, r);
        }
        3 => {
            animate_actor_cycle_tiles(engine, r);
        }
        _ => {}
    }
}

pub fn animate_actor_flip_toggle(engine: &mut Engine, r: &mut RoutineContext) {
    let mut animation_phase: i32 = 0;
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    animation_phase = ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) as i32);
    if (animation_phase == 0) {
        animation_phase = ((engine.state.obj_attr ^ ((crate::bits::BIT6) as u8)) as i32);
        engine.state.obj_attr = (animation_phase as u8);
    }
    r.value = (animation_phase as u8);
}

// Faces the actor from horizontal velocity and toggles the sprite tile bit
// every four animation ticks.
pub fn animate_actor_walk_toggle(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_x_vel_lo != 0) {
        let mut facing_bit: i32 =
            (if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
                0
            } else {
                64
            });
        engine.state.scratch0 = (facing_bit as u8);
        engine.state.obj_attr =
            (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
    }
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
        engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
    }
}

// Similar to `animate_actor_walk_toggle`, with a separate vertical-motion
// tile bit so climbing/falling frames differ from horizontal frames.
pub fn animate_actor_directional_walk(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_x_vel_lo != 0) {
        let mut facing_bit: i32 =
            (if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
                0
            } else {
                64
            });
        engine.state.scratch0 = (facing_bit as u8);
        engine.state.obj_attr =
            (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
        engine.state.obj_tile = engine.state.obj_tile & ((crate::bits::CLEAR_BIT3) as u8);
    } else {
        if (engine.state.obj_y_vel != 0) {
            engine.state.obj_tile = (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8))
                | ((crate::bits::BIT3) as u8);
        }
    }
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
        if ((engine.state.obj_tile & ((crate::bits::BIT3) as u8)) != 0) {
            engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
        } else {
            engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
        }
    }
}

// Cycles the two sprite-tile animation bits from the frame timer.
pub fn animate_actor_cycle_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_x_vel_lo != 0) {
        let mut facing_bit: i32 =
            (if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
                0
            } else {
                64
            });
        engine.state.scratch0 = (facing_bit as u8);
        engine.state.obj_attr =
            (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
    }
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    let animation_tile_bits: i32 =
        (((engine.state.obj_timer & ((crate::bits::BITS_1_2) as u8)) << 1) as u8 as i32);
    engine.state.scratch0 = (animation_tile_bits as u8);
    engine.state.obj_tile = (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8))
        | (animation_tile_bits as u8);
}

// Projects motion, rejects out-of-bounds and solid terrain, applies player
// contact damage, and restores the original vertical velocity before
// returning carry set when movement was blocked.
pub fn try_move_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
    let mut blocked: i32 = 0;
    loop {
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            engine.state.obj_state = 0;
            engine.state.obj_timer = 240;
            blocked = 1;
            break;
        }
        if (((engine.state.obj_state - 1) as u8 as i32) == 0) {
            check_player_overlap(engine, r);
            if ((r.carry) != 0) {
                apply_actor_player_contact_damage(engine, r);
            }
        }
        check_projected_terrain_collision(engine, r);
        if (r.carry == 0) {
            blocked = 0;
            break;
        }
        {
            let mut adjusted_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
            if ((adjusted_vertical_velocity & crate::bits::BIT7) == 0) {
                adjusted_vertical_velocity = ((adjusted_vertical_velocity - 2) as u8 as i32);
            }
            adjusted_vertical_velocity = ((adjusted_vertical_velocity + 1) as u8 as i32);
            engine.state.obj_y_vel = (adjusted_vertical_velocity as u8);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
        }
    }
    engine.state.obj_y_vel = (saved_vertical_velocity as u8);
    r.carry = (blocked as u8);
}

// Projects motion for actors that ignore terrain, but still applies player
// contact and clears the actor if it leaves bounds.
pub fn try_move_actor_without_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    project_actor_position(engine, r);
    check_player_overlap(engine, r);
    if ((r.carry) != 0) {
        apply_actor_player_contact_damage(engine, r);
        r.carry = 1;
        return;
    }
    check_position_out_of_bounds(engine, r);
    if (r.carry == 0) {
        return;
    }
    engine.state.obj_state = 0;
    engine.state.obj_timer = 240;
}

// Applies contact damage unless the player is already invulnerable or a
// special character/item state suppresses the hit.
pub fn apply_actor_player_contact_damage(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.sprite_blink_timer != 0) {
        return;
    }
    if (((engine.state.obj_state - 1) as u8 as i32) != 0) {
        return;
    }
    if (engine.state.chr_bank(3) >= 48) {
        if (engine.state.slot_index != 0) {
            let mut selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
            if (engine.state.item_slot(selected_item_slot) == 10) {
                engine.state.prompt_state = 1;
                return;
            }
        }
    } else {
        if (engine.state.character_index == 4) {
            return;
        }
    }
    r.value = (engine.state.obj_damage as u8);
    subtract_health_points(engine, r);
    engine.state.prompt_state = 33;
    engine.state.prompt_argument = 1;
    engine.state.sprite_blink_timer = 1;
    engine.state.obj_attr = engine.state.obj_attr & ((crate::bits::CLEAR_BIT5) as u8);
}

fn mark_probe_clear(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
    r.carry = 0;
}

/// Updates the normal one-tile-wide terrain probe for the current object.
/// When the checked footprint stays clear, the object terrain counter
/// `0xF0` advances and carry is clear.
pub fn update_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_cooldown != 0) {
        return;
    }
    engine.state.data_ptr_lo = engine.state.obj_x_tile;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    let mut tile_y: i32 = (engine.state.obj_y_pixel as i32);
    let active_state: i32 = ((engine.state.obj_state - 1) as u8 as i32);
    if (active_state == 0) {
        if (tile_y >= 176) {
            return;
        }
        engine.state.data_ptr_hi = (tile_y as u8);
        tile_y = ((tile_y + 1) as u8 as i32);
        engine.state.scratch2 = (tile_y as u8);
        check_player_overlap(engine, r);
        if ((r.carry) != 0) {
            return;
        }
    } else {
        if (tile_y == 239) {
            tile_y = (engine.state.obj_y_extra as i32);
        }
        engine.state.data_ptr_hi = (tile_y as u8);
    }
    resolve_room_tile_pointer(engine, r);
    if (engine.state.obj_x_sub == 0) {
        let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
        if ((engine.state.byte(tile_ptr) & crate::bits::LOW_6_BITS) == 0) {
            return;
        }
        if ((engine.state.byte(((tile_ptr + 1) as u16 as i32)) & crate::bits::LOW_6_BITS) == 0) {
            return;
        }
    }
    r.offset = 1;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    if (engine.state.obj_x_sub == 0) {
        return;
    }
    r.offset = 13;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    mark_probe_clear(engine, r);
}

/// Updates the wider terrain probe used by large objects. It samples the
/// lower footprint and advances `0xF0` when no solid tile or player overlap
/// blocks movement.
pub fn update_wide_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_cooldown != 0) {
        return;
    }
    engine.state.data_ptr_lo = engine.state.obj_x_tile;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.data_ptr_hi = engine.state.obj_y_pixel;
    engine.state.scratch2 = engine.state.obj_y_pixel + 1;
    resolve_room_tile_pointer(engine, r);
    if (engine.state.obj_y_pixel >= 160) {
        engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
        return;
    }
    check_player_overlap_wide(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    r.offset = 2;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    r.offset = 14;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    if (engine.state.obj_x_sub != 0) {
        r.offset = 26;
        probe_object_solid_tile(engine, r);
        if ((r.carry) != 0) {
            return;
        }
    }
    engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
}

/// Probes the room tile at `current_tile_pointer + r.offset`. Carry is set
/// when the low six tile bits are in the solid range `>= 0x30`.
pub fn probe_object_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let tile_id: i32 = ((engine
        .state
        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS) as u8 as i32);
    r.carry = (((tile_id >= 48) as u8) as u8);
}

/// Checks the projected one-tile-wide object footprint in `0x0E..0x0F/0x0A`
/// against terrain. Carry is clear only when all sampled tiles are clear.
pub fn check_projected_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);
    r.offset = 0;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo != 0) {
        r.offset = 12;
        probe_projected_solid_tile(engine, r);
        if ((r.carry) != 0) {
            return;
        }
    }
    if (engine.state.scratch2 >= 176) {
        return;
    }
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    r.offset = 1;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    r.offset = 13;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    r.carry = 0;
}

fn probe(engine: &mut Engine, r: &mut RoutineContext, tile_offset: i32) -> i32 {
    r.offset = (tile_offset as u8);
    probe_projected_solid_tile(engine, r);
    return (r.carry as i32);
}

/// Checks the projected wide object footprint in `0x0E..0x0F/0x0A` against
/// terrain. Carry is clear only when every sampled tile is clear.
pub fn check_projected_wide_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);
    if ((probe(engine, r, 0)) != 0) {
        return;
    }
    if ((probe(engine, r, 1)) != 0) {
        return;
    }
    if ((probe(engine, r, 12)) != 0) {
        return;
    }
    if ((probe(engine, r, 13)) != 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo != 0) {
        if ((probe(engine, r, 24)) != 0) {
            return;
        }
        if ((probe(engine, r, 25)) != 0) {
            return;
        }
    }
    if (engine.state.scratch2 >= 176) {
        return;
    }
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    if ((probe(engine, r, 2)) != 0) {
        return;
    }
    if ((probe(engine, r, 14)) != 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    if ((probe(engine, r, 26)) != 0) {
        return;
    }
    r.carry = 0;
}

/// Probes a projected footprint tile at `current_tile_pointer + r.offset`.
/// Carry is set when the low six tile bits are in the solid range.
pub fn probe_projected_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let tile_id: i32 = ((engine
        .state
        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS) as u8 as i32);
    r.carry = (((tile_id >= 48) as u8) as u8);
}

/// Attempts to reflect object velocity away from the nearest subtile edge
/// and re-run movement validation. Carry remains set if no reflection was
/// possible.
pub fn try_reflect_object_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let mut edge_nibble: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.obj_x_vel_hi = 0;
                if (engine.state.obj_x_vel_lo != 0) {
                    engine.state.obj_x_vel_lo = 0;
                    edge_nibble =
                        ((engine.state.obj_y_pixel & ((crate::bits::LOW_NIBBLE) as u8)) as i32);
                    if (edge_nibble == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (edge_nibble < 6) {
                        if ((engine.state.obj_move_state & ((crate::bits::BIT2) as u8)) != 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.state.obj_y_vel = 255;
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if (edge_nibble >= 11) {
                        if ((engine.state.obj_move_state & ((crate::bits::BIT3) as u8)) != 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.state.obj_y_vel = 1;
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_y_vel == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_y_vel = 0;
                edge_nibble = (engine.state.obj_x_sub as i32);
                if (edge_nibble == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (edge_nibble < 6) {
                    if ((engine.state.obj_move_state & ((crate::bits::BIT0) as u8)) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.state.obj_x_vel_lo = 15;
                    engine.state.obj_x_vel_hi = 255;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (edge_nibble >= 11) {
                    if ((engine.state.obj_move_state & ((crate::bits::BIT1) as u8)) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.state.obj_x_vel_lo = 1;
                    engine.state.obj_x_vel_hi = 0;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    state = 2;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                try_move_actor_with_terrain(engine, r);
                return;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                r.carry = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Initializes the special large actor slot from room actor data.
///
/// Large actors use slot `0x0400` as their logical state and slots
/// `0x0410..0x043F` as linked body pieces. This routine rejects blocked
/// spawn positions before seeding the logical slot and initial health
/// state for the body pieces.
pub fn initialize_large_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    engine.state.set_chr_bank(4, 61);
    engine.state.scratch2 = ((engine.state.byte(((actor_data_ptr + 3) as u16 as i32))) as u8);
    engine.state.indirect_ptr_hi =
        ((engine.state.byte(((actor_data_ptr + 2) as u16 as i32))) as u8);
    engine.state.indirect_ptr_lo = 0;
    engine.state.scratch3 = 0;
    check_projected_wide_terrain_collision(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    engine.state.obj_state = 1;
    engine.state.obj_tile = 129;
    engine.state.obj_attr = 2;
    engine.state.obj_damage = ((engine.state.byte(((actor_data_ptr + 5) as u16 as i32))) as u8);
    {
        let actor_health: i32 = engine.state.byte(((actor_data_ptr + 4) as u16 as i32));
        engine.state.obj_health = (actor_health as u8);
        engine.state.set_object_health(16, actor_health);
        engine.state.set_object_health(32, actor_health);
        engine.state.set_object_health(48, actor_health);
    }
    engine.state.indirect_ptr_lo = 225;
    engine.state.indirect_ptr_hi = 167;
    with_large_actor_asset_banks(engine, r, load_large_actor_oam_template);
    engine.state.indirect_ptr_lo = 83;
    engine.state.indirect_ptr_hi = 203;
    with_large_actor_asset_banks(engine, r, build_object_health_meter_alt_tiles);
}

/// Updates the active large actor: aim toward the player, run the wide
/// jump/gravity movement path, then advance facing and animation state.
pub fn tick_large_chasing_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut horizontal_direction: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.obj_move_state =
                    engine.state.obj_move_state & ((crate::bits::LOW_NIBBLE) as u8);
                if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
                    if ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        engine.state.obj_move_state = 1;
                    }
                    {
                        let mut turn_timer: i32 = (engine.state.obj_timer as i32);
                        engine.state.obj_timer = 0;
                        turn_timer = ((turn_timer - 1) as u8 as i32);
                        if (turn_timer == 0) {
                            horizontal_direction = ((engine.state.obj_move_state
                                & ((crate::bits::LOW_2_BITS) as u8))
                                as i32);
                            if (horizontal_direction != 0) {
                                engine.state.obj_move_state =
                                    ((horizontal_direction ^ crate::bits::LOW_2_BITS) as u8);
                                {
                                    state = 2;
                                    continue 'dispatch;
                                }
                            }
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        aim_actor_toward_player(engine, r);
                        engine.state.obj_move_state = 128 | engine.state.obj_move_state;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                } else {
                    if (engine.state.obj_timer < 50) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                engine.state.obj_timer = 0;
                aim_actor_toward_player(engine, r);
                state = 2;
                continue 'dispatch;
            }
            2 => {
                r.value = (engine.state.obj_move_state as u8);
                r.offset = 2;
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    try_large_actor_gravity_motion(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_cooldown != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::BIT7) as u8)) == 0) {
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                try_large_actor_jump_arc_motion(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                engine.state.obj_cooldown = 0;
                try_move_large_actor_with_terrain(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                stop_actor_motion(engine, r);
                {
                    state = 6;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                commit_actor_projected_position(engine, r);
                state = 6;
                continue 'dispatch;
            }
            6 => {
                update_wide_object_terrain_probe(engine, r);
                update_large_actor_facing_from_velocity(engine, r);
                animate_large_actor_body_tiles(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Applies the large actor's falling motion. If wide movement is blocked,
/// it retries without horizontal velocity before cancelling vertical speed.
pub fn try_large_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let fall_velocity: i32 = (((engine.state.obj_move_scratch >> 2) + 1) as u8 as i32);
    engine.state.obj_y_vel = (fall_velocity as u8);
    r.value = (fall_velocity as u8);
    try_move_large_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    r.value = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_y_vel = 0;
    r.value = 0;
}

/// Advances the large actor's jump arc and retries straight-up movement
/// when horizontal motion collides with terrain.
pub fn try_large_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut jump_counter: i32 = (engine.state.obj_cooldown as i32);
    if (jump_counter == 0) {
        jump_counter = 25;
    }
    jump_counter = ((jump_counter - 1) as u8 as i32);
    engine.state.obj_cooldown = (jump_counter as u8);
    r.index = (jump_counter as u8);
    engine.state.obj_y_vel = ((((jump_counter >> 2) ^ crate::bits::BYTE_MASK) + 1) as u8);
    try_move_large_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_large_actor_with_terrain(engine, r);
}

/// Projects wide actor motion, applies player contact damage, and rejects
/// terrain using the three-tile-wide footprint. Carry is set when blocked.
pub fn try_move_large_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
    let mut blocked: i32 = 0;
    loop {
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            engine.state.obj_state = 0;
            engine.state.obj_timer = 240;
            blocked = 1;
            break;
        }
        check_player_overlap_wide(engine, r);
        if ((r.carry) != 0) {
            apply_actor_player_contact_damage(engine, r);
        }
        check_projected_wide_terrain_collision(engine, r);
        if (r.carry == 0) {
            blocked = 0;
            break;
        }
        {
            let mut adjusted_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
            if ((adjusted_vertical_velocity & crate::bits::BIT7) == 0) {
                adjusted_vertical_velocity = ((adjusted_vertical_velocity - 2) as u8 as i32);
            }
            adjusted_vertical_velocity = ((adjusted_vertical_velocity + 1) as u8 as i32);
            engine.state.obj_y_vel = (adjusted_vertical_velocity as u8);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
        }
    }
    engine.state.obj_y_vel = (saved_vertical_velocity as u8);
    r.carry = (blocked as u8);
}

/// Updates the large actor's facing bit from horizontal velocity.
pub fn update_large_actor_facing_from_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let mut facing_bit: i32 = 0;
    if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
    } else if (engine.state.obj_x_vel_lo == 0) {
        return;
    } else {
        facing_bit = 64;
    }
    engine.state.scratch0 = (facing_bit as u8);
    engine.state.obj_attr =
        (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
}

/// Advances the large actor's animation timer and stores the base body
/// tile id for the linked sprite slots.
pub fn animate_large_actor_body_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let animation_timer: i32 =
        (((engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8)) as i32);
    engine.state.obj_timer = (animation_timer as u8);
    let body_tile_id: i32 =
        ((((animation_timer & crate::bits::BITS_2_3) << 1) | crate::bits::BITS_0_6) as u8 as i32);
    engine.state.obj_tile = (body_tile_id as u8);
    r.value = (body_tile_id as u8);
}

fn swap_slot_sprite_id(engine: &mut Engine, a: i32, b: i32) {
    let slot_sprite_id: i32 = engine.state.byte(a);
    engine.state.set_byte(a, engine.state.byte(b));
    engine.state.set_byte(b, slot_sprite_id);
}

/// Mirrors the large actor's logical slot into the three linked body slots.
///
/// Slot `0x0400` remains the damage/state anchor. Slots `0x0410`,
/// `0x0420`, and `0x0430` are arranged as the visible 2x2 body, then their
/// sprite ids are swapped by facing/flip bits so draw order stays correct.
pub fn compose_large_actor_body_slots(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_object_y_extra(16, (engine.state.obj_y_extra as i32));
    engine
        .state
        .set_object_y_extra(32, (engine.state.obj_y_extra as i32));
    engine
        .state
        .set_object_y_extra(48, (engine.state.obj_y_extra as i32));
    {
        let tile_y: i32 = (engine.state.obj_y_pixel as i32);
        engine.state.set_object_y_pixel(16, tile_y);
        engine.state.set_object_y_pixel(32, tile_y + 16);
        engine.state.set_object_y_pixel(48, tile_y + 16);
    }
    engine
        .state
        .set_object_x_sub(16, (engine.state.obj_x_sub as i32));
    engine
        .state
        .set_object_x_sub(32, (engine.state.obj_x_sub as i32));
    engine
        .state
        .set_object_x_sub(48, (engine.state.obj_x_sub as i32));
    {
        let tile_x: i32 = (engine.state.obj_x_tile as i32);
        engine.state.set_object_x_tile(32, tile_x);
        engine.state.set_object_x_tile(16, tile_x + 1);
        engine.state.set_object_x_tile(48, tile_x + 1);
    }
    {
        let mut actor_state: i32 = (engine.state.obj_state as i32);
        if ((actor_state & crate::bits::BIT7) == 0) {
            if (((engine.state.object_state(16)
                | engine.state.object_state(32)
                | engine.state.object_state(48))
                & crate::bits::BIT7)
                != 0)
            {
                actor_state = 128;
            }
        }
        engine.state.set_object_state(0, actor_state);
        engine.state.set_object_state(16, actor_state);
        engine.state.set_object_state(32, actor_state);
        engine.state.set_object_state(48, actor_state);
    }
    {
        let mut minimum_health: i32 = (engine.state.obj_health as i32);
        if (minimum_health >= engine.state.object_health(16)) {
            minimum_health = engine.state.object_health(16);
        }
        if (minimum_health >= engine.state.object_health(32)) {
            minimum_health = engine.state.object_health(32);
        }
        if (minimum_health >= engine.state.object_health(48)) {
            minimum_health = engine.state.object_health(48);
        }
        engine.state.set_object_health(0, minimum_health);
    }
    {
        let body_tile_id: i32 = (engine.state.obj_tile as i32);
        let upper_right_tile: i32 = ((body_tile_id | crate::bits::BIT2) as u8 as i32);
        engine.state.set_object_tile(16, upper_right_tile);
        let lower_right_tile: i32 = ((upper_right_tile | crate::bits::BIT5) as u8 as i32);
        engine.state.set_object_tile(48, lower_right_tile);
        let lower_left_tile: i32 = ((lower_right_tile & crate::bits::CLEAR_BIT2) as u8 as i32);
        engine.state.set_object_tile(32, lower_left_tile);
    }
    {
        let sprite_attrs: i32 = (engine.state.obj_attr as i32);
        engine.state.set_object_attr(16, sprite_attrs);
        engine.state.set_object_attr(32, sprite_attrs);
        engine.state.set_object_attr(48, sprite_attrs);
        if ((sprite_attrs & crate::bits::BIT6) != 0) {
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 0, OBJECT_TABLE_BASE + 16);
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 32, OBJECT_TABLE_BASE + 48);
        }
        if ((sprite_attrs & crate::bits::BIT7) != 0) {
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 0, OBJECT_TABLE_BASE + 32);
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 16, OBJECT_TABLE_BASE + 48);
        }
    }
    with_large_actor_asset_banks(engine, r, |engine, r| {
        engine.state.indirect_ptr_lo = 83;
        engine.state.indirect_ptr_hi = 203;
        build_object_health_meter_alt_tiles(engine, r);
    });
}

/// Walks the pooled player projectile slots at `0x04B0` and either updates
/// an active slot or spawns a new shot on a fire-button edge.
pub fn update_player_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.slot_index = 11;
    engine.state.obj_slot_ptr_lo = 176;
    engine.state.obj_slot_ptr_hi = 4;
    loop {
        let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
        let active_lifetime: i32 = engine.state.byte(((slot_ptr + 1) as u16 as i32));
        if (active_lifetime != 0) {
            r.value = (active_lifetime as u8);
            r.offset = 1;
            update_player_projectile_slot(engine, r);
        } else {
            if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                if ((engine.state.direction_latch & ((crate::bits::BIT6) as u8)) == 0) {
                    r.value = 0;
                    r.offset = 1;
                    spawn_player_projectile(engine, r);
                }
            }
        }
        engine.state.slot_index = (engine.state.slot_index + 1) & ((crate::bits::BYTE_MASK) as u8);
        {
            let next_slot_lo: i32 = ((16 + engine.state.obj_slot_ptr_lo) as u16 as i32);
            engine.state.obj_slot_ptr_lo = (next_slot_lo as u8);
            engine.state.obj_slot_ptr_hi =
                engine.state.obj_slot_ptr_hi + ((next_slot_lo >> 8) as u8);
        }
        if !(((engine.state.slot_index - 11) as u8 as i32) < (engine.state.projectile_count as i32))
        {
            break;
        }
    }
}

/// Initializes the current empty projectile slot from the player's facing,
/// current pose, and resource constraints.
pub fn spawn_player_projectile(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                load_object_slot_scratch(engine, r);
                engine.state.direction_latch = (engine.state.buttons & ((crate::bits::BIT6) as u8))
                    | engine.state.direction_latch;
                r.offset = ((if (engine.state.displaced_timer != 0) {
                    4
                } else {
                    2
                }) as u8);
                r.value = (engine.state.direction_latch as u8);
                build_direction_velocity(engine, r);
                project_player_projectile_position(engine, r);
                check_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                consume_magic_point(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
                engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
                engine.state.obj_y_pixel = engine.state.scratch2;
                load_effective_projectile_lifetime(engine, r);
                engine.state.obj_state = (r.value as u8);
                if (r.carry == 0) {
                    consume_magic_point(engine, r);
                }
                load_effective_projectile_damage(engine, r);
                engine.state.obj_damage = (r.value as u8);
                if (r.carry == 0) {
                    consume_magic_point(engine, r);
                }
                engine.state.obj_attr = 0;
                engine.state.obj_tile = 33;
                engine.state.prompt_state = 34 + engine.state.character_index;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if (engine.state.obj_state != 0) {
                    apply_projectile_direction_bits(engine, r);
                }
                store_object_slot_scratch(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

fn store_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
}

fn finish_projectile_slot_update(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_state != 0) {
        apply_projectile_direction_bits(engine, r);
    }
    store_object_slot_scratch(engine, r);
}

/// Advances one active player projectile, applying terrain collision,
/// actor hits, damage, and expiry back into the object slot.
pub fn update_player_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
    load_object_slot_scratch(engine, r);
    engine.state.obj_state = engine.state.obj_state - 1;
    if (engine.state.obj_state == 0) {
        finish_projectile_slot_update(engine, r);
        return;
    }
    project_actor_position(engine, r);
    check_position_out_of_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.obj_state = 0;
        finish_projectile_slot_update(engine, r);
        return;
    }
    find_damageable_actor_overlap(engine, r);
    if ((r.carry) == 0) {
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
        return;
    }
    if ((engine.state.chr_bank(3) >= 48) && (engine.state.scratch0 >= 4)) {
        let hit_slot: i32 = (engine.state.scratch1 as i32);
        engine.state.set_object_state(hit_slot, 128);
        engine.state.obj_state = 1;
        engine.state.prompt_state = 12;
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
        return;
    }
    {
        let mut hit_slot: i32 = (engine.state.scratch1 as i32);
        if (((engine.state.object_state(hit_slot) - 1) as u8 as i32) != 0) {
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        hit_slot = (engine.state.scratch1 as i32);
        {
            let knockback: i32 = (if ((engine.state.obj_state & ((crate::bits::BIT0) as u8)) != 0) {
                2
            } else {
                254
            });
            engine.state.set_object_y_extra(hit_slot, knockback);
        }
        {
            let target_health: i32 = engine.state.object_health(hit_slot);
            let projectile_damage: i32 = (engine.state.obj_damage as i32);
            engine.state.set_password_nibbles_a(
                227 + hit_slot,
                ((target_health - projectile_damage) as u8 as i32),
            );
            if (target_health >= projectile_damage) {
                engine.state.prompt_state = 6;
            } else {
                engine.state.set_object_state(hit_slot, 128);
                engine.state.set_object_health(hit_slot, 0);
            }
        }
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
    }
}

/// Projects player position plus projectile velocity into the shared
/// collision-coordinate scratch registers.
pub fn project_player_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.scratch2 = engine.state.player_y;
    if (engine.state.obj_y_vel != 0) {
        let mut a: i32 = (((engine.state.obj_y_vel as i32) << 2) as u8 as i32);
        a = ((a + (engine.state.scratch2 as i32)) as u8 as i32);
        engine.state.scratch2 = (a as u8);
    }
    if (engine.state.obj_x_vel_lo != 0) {
        let projected_subtile: i32 =
            ((((((engine.state.obj_x_vel_lo as i32) << 2)
                & (((crate::bits::LOW_NIBBLE) as u8) as i32)) as u8 as i32)
                + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((projected_subtile & crate::bits::LOW_NIBBLE) as u8);
        engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi
            + engine.state.obj_x_vel_hi
            + (((projected_subtile >> 4) & 1) as u8);
    }
}

/// Copies the projectile direction bits from its lifetime/state byte into
/// the sprite/object descriptor used by later render and collision code.
pub fn apply_projectile_direction_bits(engine: &mut Engine, r: &mut RoutineContext) {
    let direction_bits: i32 = ((engine.state.obj_state & ((crate::bits::BITS_2_3) as u8)) as i32);
    engine.state.scratch0 = (direction_bits as u8);
    engine.state.obj_tile =
        (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8)) | (direction_bits as u8);
    r.value = (engine.state.obj_tile as u8);
}

/// Updates the singleton tile-removal projectile stored at `0x0490` and
/// restores its saved background tile when it expires.
pub fn update_tile_projectile(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.object_state(144) == 0) {
        return;
    }
    engine.state.obj_slot_ptr_lo = 144;
    engine.state.obj_slot_ptr_hi = 4;
    load_object_slot_scratch(engine, r);
    engine.state.obj_timer = engine.state.obj_timer - 1;
    if (engine.state.obj_timer != 0) {
        update_tile_projectile_motion(engine, r);
        return;
    }
    if ((engine.state.obj_tile & ((crate::bits::BIT0) as u8)) == 0) {
        if ((((engine.state.obj_y_pixel & ((crate::bits::LOW_NIBBLE) as u8))
            | engine.state.obj_x_sub) as u8 as i32)
            != 0)
        {
            engine.state.obj_timer = engine.state.obj_timer + 1;
            update_tile_projectile_motion(engine, r);
            return;
        }
    }
    engine.state.obj_state = 0;
    if (engine.state.obj_move_scratch != 0) {
        engine.state.data_ptr_lo = engine.state.obj_x_tile;
        engine.state.data_ptr_hi = engine.state.obj_y_pixel;
        resolve_room_tile_pointer(engine, r);
        let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
        engine
            .state
            .set_byte(tile_ptr, (engine.state.obj_move_scratch as i32));
        let screen_diff: i32 =
            ((engine.state.obj_x_tile - engine.state.scroll_tile_x) as u8 as i32);
        if ((screen_diff < 17) || (screen_diff >= 254)) {
            let tile_x: i32 = (engine.state.obj_x_tile as i32);
            engine.state.data_ptr_lo = (tile_x as u8);
            engine.state.vram_addr_lo = (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
            engine.state.vram_addr_hi =
                (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
            engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
            engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
            farcall_bank_09_r7(engine, r);
        }
    }
    store_object_slot_scratch(engine, r);
}

/// Advances the tile-removal projectile, including collision checks,
/// bouncing, contact damage, and final tile replacement.
pub fn update_tile_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                {
                    let mut i: i32 = 0;
                    {
                        i = 2048;
                        while (i < 40960) {
                            engine.state.set_byte(i, 0);
                            {
                                i += 1;
                                i
                            };
                        }
                    }
                }
                if ((engine.state.obj_tile & ((crate::bits::BIT0) as u8)) != 0) {
                    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                engine.state.slot_index = 9;
                project_actor_position(engine, r);
                check_actor_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                check_projected_terrain_collision(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                check_player_overlap(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                find_damageable_actor_overlap(engine, r);
                if ((r.carry) != 0) {
                    let hit_slot: i32 = (engine.state.scratch1 as i32);
                    engine.state.set_object_state(hit_slot, 128);
                }
                engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
                engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
                engine.state.obj_y_pixel = engine.state.scratch2;
                engine.state.obj_move_state = 0;
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if (engine.state.obj_move_state != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.sprite_blink_timer != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                consume_health_point(engine, r);
                engine.state.prompt_state = 10;
                engine.state.sprite_blink_timer = 2;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                if (engine.state.obj_move_state != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_move_state = engine.state.obj_move_state + 1;
                if (engine.state.obj_x_vel_lo != 0) {
                    engine.state.obj_x_vel_lo =
                        (0 - engine.state.obj_x_vel_lo) & ((crate::bits::LOW_NIBBLE) as u8);
                    engine.state.obj_x_vel_hi =
                        engine.state.obj_x_vel_hi ^ ((crate::bits::BYTE_MASK) as u8);
                }
                engine.state.obj_y_vel = ((((!engine.state.obj_y_vel) as u8 as i32) + 1) as u8);
                if (engine.state.prompt_state == 0) {
                    engine.state.prompt_state = 6;
                }
                {
                    state = 4;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                if ((((engine.state.obj_y_pixel & ((crate::bits::LOW_NIBBLE) as u8))
                    | engine.state.obj_x_sub) as u8 as i32)
                    != 0)
                {
                    engine.state.obj_timer = engine.state.obj_timer + 1;
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                {
                    engine.state.obj_state = 0;
                    if (engine.state.obj_move_scratch != 0) {
                        engine.state.data_ptr_lo = engine.state.obj_x_tile;
                        engine.state.data_ptr_hi = engine.state.obj_y_pixel;
                        resolve_room_tile_pointer(engine, r);
                        let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
                        engine
                            .state
                            .set_byte(tile_ptr, (engine.state.obj_move_scratch as i32));
                        let screen_diff: i32 =
                            ((engine.state.obj_x_tile - engine.state.scroll_tile_x) as u8 as i32);
                        if ((screen_diff < 17) || (screen_diff >= 254)) {
                            let tile_x: i32 = (engine.state.obj_x_tile as i32);
                            engine.state.data_ptr_lo = (tile_x as u8);
                            engine.state.vram_addr_lo =
                                (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
                            engine.state.vram_addr_hi =
                                (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
                            engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
                            engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi;
                            farcall_bank_09_r7(engine, r);
                        }
                    }
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                store_object_slot_scratch(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

// Music channel state is stored in 0x10-byte lanes. The current lane offset
// lives in 0x02: 0x00 pulse 1, 0x10 pulse 2, 0x20 triangle, 0x30 noise, and
// 0x40 for the pulse-2 sound-effect overlay.
fn silence_pulse1(engine: &mut Engine, _r: &mut RoutineContext) {
    engine.device_write(
        crate::engine::reg::SQ1_VOL,
        (engine.state.sound_channel_byte(6, 0) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT0) as u8);
}

pub fn tick_pulse1_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if ((engine.state.sound_channel_byte(1, 0) & crate::bits::BIT7) == 0) {
                    silence_pulse1(engine, r);
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(0, 0)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                loop {
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 0)
                        | (engine.state.sound_channel_byte(3, 0) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_pulse1(engine, r);
                        return;
                    }
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 0, note_byte & crate::bits::LOW_7_BITS);
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        load_note_period(engine, r);
                        engine.state.sound_status_flags =
                            engine.state.sound_status_flags | ((crate::bits::BIT0) as u8);
                        engine.device_write(
                            crate::engine::reg::SQ1_SWEEP,
                            engine.state.sound_channel_byte(7, 0),
                        );
                        engine.device_write(
                            crate::engine::reg::SQ1_LO,
                            (engine.state.sound_command as i32),
                        );
                        engine.device_write(
                            crate::engine::reg::SQ1_HI,
                            (((engine.state.sound_length & ((crate::bits::LOW_3_BITS) as u8))
                                | ((crate::bits::BITS_3_4) as u8))
                                as i32),
                        );
                        start_note_envelope(engine, r);
                    }
                    break;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.sound_status_flags & ((crate::bits::BIT0) as u8)) == 0) {
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(10, 0)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ1_VOL, (r.value as i32));
                }
                advance_envelope_phase(engine, r);
                if ((r.carry) != 0) {
                    silence_pulse1(engine, r);
                }
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

fn silence_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
    engine.device_write(
        crate::engine::reg::SQ2_VOL,
        (engine.state.sound_channel_byte(6, 16) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT1) as u8);
}

pub fn tick_pulse2_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_flags: i32 = (engine.state.sound_channel_flags as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if ((channel_flags & crate::bits::BIT7) == 0) {
                    if ((channel_flags & crate::bits::BIT6) != 0) {
                        return;
                    }
                    silence_pulse2(engine, r);
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(0, 16)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                loop {
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 16)
                        | (engine.state.sound_channel_byte(3, 16) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_pulse2(engine, r);
                        return;
                    }
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 16, note_byte & crate::bits::LOW_7_BITS);
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        if ((engine.state.sound_channel_flags & ((crate::bits::BIT6) as u8)) != 0) {
                            return;
                        }
                        start_rest_envelope(engine, r);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if ((engine.state.sound_channel_flags & ((crate::bits::BIT6) as u8)) != 0) {
                        increment_selected_music_stream_pointer(engine, r);
                        return;
                    }
                    load_note_period(engine, r);
                    engine.state.sound_status_flags =
                        engine.state.sound_status_flags | ((crate::bits::BIT1) as u8);
                    engine.device_write(
                        crate::engine::reg::SQ2_VOL,
                        engine.state.sound_channel_byte(6, 16),
                    );
                    engine.device_write(
                        crate::engine::reg::SQ2_SWEEP,
                        engine.state.sound_channel_byte(7, 16),
                    );
                    engine.device_write(
                        crate::engine::reg::SQ2_LO,
                        (engine.state.sound_command as i32),
                    );
                    engine.device_write(
                        crate::engine::reg::SQ2_HI,
                        (((engine.state.sound_length & ((crate::bits::LOW_3_BITS) as u8))
                            | ((crate::bits::BITS_3_4) as u8)) as i32),
                    );
                    start_note_envelope(engine, r);
                    break;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.sound_channel_flags & ((crate::bits::BIT6) as u8)) != 0) {
                    return;
                }
                if ((engine.state.sound_status_flags & ((crate::bits::BIT1) as u8)) == 0) {
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(10, 16)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ2_VOL, (r.value as i32));
                }
                advance_envelope_phase(engine, r);
                if ((r.carry) != 0) {
                    silence_pulse2(engine, r);
                }
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

fn silence_triangle(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 0;
    engine.device_write(crate::engine::reg::TRI_LINEAR, 0);
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT2) as u8);
    r.value = (engine.state.sound_status_flags as u8);
}

pub fn tick_triangle_channel(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.sound_channel_byte(1, 32) & crate::bits::BIT7) == 0) {
        silence_triangle(engine, r);
        return;
    }
    if (((engine.state.triangle_timer - 1) as u8 as i32) != 0) {
        engine.state.triangle_timer = engine.state.triangle_timer - 1;
        return;
    }
    engine.state.triangle_timer = engine.state.triangle_timer - 1;
    loop {
        let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 32)
            | (engine.state.sound_channel_byte(3, 32) << 8))
            as u16 as i32);
        let mut note_byte: i32 = engine.state.byte(stream_ptr);
        if (note_byte == 0) {
            rewind_or_stop_audio_stream(engine, r);
            silence_triangle(engine, r);
            return;
        }
        if (note_byte != 255) {
            let mut is_rest: i32 = ((note_byte & crate::bits::BIT7) as u8 as i32);
            r.value = (note_byte as u8);
            increment_selected_music_stream_pointer(engine, r);
            r.value = ((note_byte & crate::bits::LOW_7_BITS) as u8);
            engine.state.triangle_timer = (r.value as u8);
            if ((is_rest) != 0) {
                silence_triangle(engine, r);
                return;
            }
            load_note_period(engine, r);
            engine.state.sound_status_flags =
                engine.state.sound_status_flags | ((crate::bits::BIT2) as u8);
            engine.device_write(
                crate::engine::reg::TRI_LINEAR,
                engine.state.sound_channel_byte(7, 32),
            );
            engine.device_write(
                crate::engine::reg::TRI_LO,
                (engine.state.sound_command as i32),
            );
            r.value = (((engine.state.sound_length & ((crate::bits::LOW_3_BITS) as u8))
                | ((crate::bits::HIGH_5_BITS) as u8)) as u8);
            engine.device_write(crate::engine::reg::TRI_HI, (r.value as i32));
            return;
        }
        dispatch_audio_stream_command(engine, r);
    }
}

fn silence_noise(engine: &mut Engine, _r: &mut RoutineContext) {
    engine.device_write(crate::engine::reg::NOISE_VOL, 48);
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT3) as u8);
}

pub fn tick_noise_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if ((engine.state.sound_channel_byte(1, 48) & crate::bits::BIT7) == 0) {
                    silence_noise(engine, r);
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(0, 48)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                loop {
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 48)
                        | (engine.state.sound_channel_byte(3, 48) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_noise(engine, r);
                        return;
                    }
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 48, note_byte & crate::bits::LOW_7_BITS);
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        engine.state.sound_status_flags =
                            engine.state.sound_status_flags | ((crate::bits::BIT3) as u8);
                        engine.device_write(
                            crate::engine::reg::NOISE_LO,
                            engine.state.sound_channel_byte(7, 48),
                        );
                        engine.device_write(crate::engine::reg::NOISE_HI, 128);
                        start_note_envelope(engine, r);
                    }
                    break;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.sound_status_flags & ((crate::bits::BIT3) as u8)) == 0) {
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(10, 48)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::NOISE_VOL, (r.value as i32));
                }
                advance_envelope_phase(engine, r);
                if ((r.carry) != 0) {
                    silence_noise(engine, r);
                }
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

fn deref_stream(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    let mut channel_offset: i32 = (r.index as u8 as i32);
    let mut lo: i32 = engine.state.sound_channel_byte(2, channel_offset);
    let mut hi: i32 = engine.state.sound_channel_byte(3, channel_offset);
    return engine.state.byte(((lo | (hi << 8)) as u16 as i32));
}

// A 0xFF stream byte is followed by command id and value bytes. The command
// updates per-channel playback state, then leaves the stream pointer at the
// next note/rest/control byte.
pub fn dispatch_audio_stream_command(engine: &mut Engine, r: &mut RoutineContext) {
    r.index = (engine.state.sound_channel_offset as u8);
    increment_selected_music_stream_pointer(engine, r);
    {
        let __v = deref_stream(engine, r);
        engine.state.sound_command = (__v as u8);
    }
    increment_selected_music_stream_pointer(engine, r);
    {
        let __v = deref_stream(engine, r);
        engine.state.sound_length = (__v as u8);
    }
    increment_selected_music_stream_pointer(engine, r);
    let mut command_id: i32 = (engine.state.sound_command as i32);
    if (command_id >= 5) {
        return;
    }
    const ORIGINAL_COMMAND_HANDLERS: [i32; 5] = [0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05];
    let mut original_handler: i32 = ORIGINAL_COMMAND_HANDLERS[command_id as usize];
    engine.state.saved_audio_handler_lo = ((original_handler & crate::bits::BYTE_MASK) as u8);
    engine.state.saved_audio_handler_hi = ((original_handler >> 8) as u8);
    r.value = (engine.state.sound_length as u8);
    r.index = (engine.state.sound_channel_offset as u8);
    match command_id {
        0 => {
            audio_cmd_set_duty_instrument(engine, r);
        }
        1 => {
            audio_cmd_set_volume_scale(engine, r);
        }
        2 => {
            audio_cmd_set_channel_flags(engine, r);
        }
        3 => {
            audio_cmd_set_pitch_offset(engine, r);
        }
        4 => {
            audio_cmd_set_sweep_value(engine, r);
        }
        _ => {}
    }
}

// Command 0 packs pulse duty in the high nibble and envelope table choice
// in the low nibble. The low nibble is expanded to a 16-byte table offset.
pub fn audio_cmd_set_duty_instrument(engine: &mut Engine, r: &mut RoutineContext) {
    let mut command_value: i32 = (r.value as u8 as i32);
    let mut channel_offset: i32 = (r.index as u8 as i32);
    let mut duty_bits: i32 =
        ((((command_value & crate::bits::HIGH_NIBBLE) as u8 as i32) << 2) as u8 as i32);
    engine.state.audio_duty_work = (duty_bits as u8);
    engine.state.set_sound_channel_byte(
        6,
        0,
        (((engine.state.sound_channel_byte(6, channel_offset) & crate::bits::LOW_6_BITS)
            | duty_bits) as u8 as i32),
    );
    let mut envelope_offset: i32 = ((command_value << 4) as u8 as i32);
    engine
        .state
        .set_sound_channel_byte(15, channel_offset, envelope_offset);
    engine.state.set_sound_channel_byte(
        7,
        0,
        engine
            .state
            .byte(((SUSTAIN_TABLE + envelope_offset) as u16 as i32)),
    );
    r.value = ((engine
        .state
        .byte(((SUSTAIN_TABLE + envelope_offset) as u16 as i32))) as u8);
    r.offset = (envelope_offset as u8);
    r.index = (channel_offset as u8);
}

// Command 1 stores the per-channel multiplier used after the envelope's raw
// 0..15 volume accumulator is updated.
pub fn audio_cmd_set_volume_scale(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (r.index as u8 as i32);
    if !(engine.state.sound_channel_offset == 64) {
        let mut music_volume_override: i32 = (engine.state.music_volume_override as i32);
        if (music_volume_override != 0) {
            r.value = (music_volume_override as u8);
            r.index = (channel_offset as u8);
            return;
        }
    }
    {
        let mut adjusted_command: i32 = ((15 + engine.state.sound_length) as u8 as i32);
        let mut scale: i32 = if (adjusted_command >= 8) {
            ((adjusted_command - 8) as u8 as i32)
        } else {
            0
        };
        scale = ((scale << 1) as u8 as i32);
        scale = ((scale + 1) as u8 as i32);
        engine
            .state
            .set_sound_channel_byte(13, channel_offset, scale);
        r.value = (scale as u8);
    }
    r.index = (channel_offset as u8);
}

// Command 2 replaces the channel flag/register shadow byte at 0x99+x.
pub fn audio_cmd_set_channel_flags(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(6, r.index as i32, r.value as i32);
}

// Command 3 stores a fine pitch offset subtracted from the period table.
pub fn audio_cmd_set_pitch_offset(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(14, r.index as i32, r.value as i32);
}

// Command 4 replaces the square-channel sweep/noise-period shadow byte.
pub fn audio_cmd_set_sweep_value(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(7, r.index as i32, r.value as i32);
}

// Note bytes use the low nibble as the pitch-table index and the high
// nibble as the octave shift. The resulting period lands in 0x04/0x05.
pub fn load_note_period(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, channel_offset)
        | (engine.state.sound_channel_byte(3, channel_offset) << 8))
        as u16 as i32);
    let mut note_byte: i32 = engine.state.byte(stream_ptr);
    increment_selected_music_stream_pointer(engine, r);
    {
        let mut pitch_index: i32 = (((note_byte & crate::bits::LOW_NIBBLE) << 1) as u8 as i32);
        let mut lo: i32 = engine
            .state
            .byte(((NOTE_PERIOD_TABLE + pitch_index) as u16 as i32));
        let mut hi: i32 = engine
            .state
            .byte((((NOTE_PERIOD_TABLE + 1) + pitch_index) as u16 as i32));
        channel_offset = (engine.state.sound_channel_offset as i32);
        {
            let mut sub: i32 = (((lo as u16 as i32)
                - engine.state.sound_channel_byte(14, channel_offset))
                as u16 as i32);
            lo = (sub as u8 as i32);
            if ((sub & crate::bits::BIT8) != 0) {
                hi = ((hi - 1) as u8 as i32);
            }
        }
        {
            let mut octave_shift_count: i32 = ((note_byte >> 4) as u8 as i32);
            while (octave_shift_count != 0) {
                let mut carry_from_hi: i32 = ((hi & 1) as u8 as i32);
                hi = ((hi >> 1) as u8 as i32);
                lo = (((lo >> 1) | (carry_from_hi << 7)) as u8 as i32);
                {
                    octave_shift_count -= 1;
                    octave_shift_count
                };
            }
        }
        engine.state.sound_command = (lo as u8);
        engine.state.sound_length = (hi as u8);
    }
}

// Multiply the raw envelope accumulator in 0x00 by r.offset+1, then divide
// by 16 to return the APU's 4-bit volume value.
pub fn scale_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
    let mut scaled_volume: i32 = 0;
    let mut multiplier: i32 = (((r.offset + 1) as u8) as i32);
    loop {
        scaled_volume = ((scaled_volume + (engine.state.audio_duty_work as i32)) as u8 as i32);
        multiplier = ((multiplier - 1) as u8 as i32);
        if (multiplier == 0) {
            break;
        }
    }
    scaled_volume >>= 4;
    engine.state.audio_duty_work = (scaled_volume as u8);
    r.value = (scaled_volume as u8);
    r.offset = 0;
}

// Load the first active-note envelope phase into the selected channel lane.
pub fn start_note_envelope(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut envelope_offset: i32 = engine.state.sound_channel_byte(15, channel_offset);
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, envelope_offset);
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + envelope_offset) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 1) + envelope_offset) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 2) + envelope_offset) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        12,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 3) + envelope_offset) as u16 as i32)),
    );
    r.index = (channel_offset as u8);
    r.offset = (envelope_offset as u8);
}

// Rest bytes reuse the same envelope table with a +0x0C offset, which
// gives the ticker a timed silent phase instead of an audible period.
pub fn start_rest_envelope(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut rest_envelope_offset: i32 =
        ((engine.state.sound_channel_byte(15, channel_offset) + 12) as u8 as i32);
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, rest_envelope_offset);
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + rest_envelope_offset) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 1) + rest_envelope_offset) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 2) + rest_envelope_offset) as u16 as i32)),
    );
    r.index = (channel_offset as u8);
    r.offset = (rest_envelope_offset as u8);
    r.value = ((engine
        .state
        .byte((((ENVELOPE_TABLE + 2) + rest_envelope_offset) as u16 as i32))) as u8);
}

// A zero stream byte jumps to the saved loop pointer when one exists; a
// missing loop pointer clears the active bit while preserving sfx overlay.
pub fn rewind_or_stop_audio_stream(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut loop_pointer_hi: i32 = 0;
    engine.state.set_sound_channel_byte(
        2,
        channel_offset,
        engine.state.sound_channel_byte(4, channel_offset),
    );
    loop_pointer_hi = engine.state.sound_channel_byte(5, channel_offset);
    engine
        .state
        .set_sound_channel_byte(3, channel_offset, loop_pointer_hi);
    if (loop_pointer_hi != 0) {
        engine.state.set_sound_channel_byte(0, channel_offset, 1);
    } else {
        engine.state.set_sound_channel_byte(
            1,
            channel_offset,
            engine.state.sound_channel_byte(1, channel_offset) & crate::bits::BIT6,
        );
    }
    r.index = (channel_offset as u8);
}

// Update the current envelope accumulator and compose the APU volume
// register value from channel flags, constant-volume bit, and scaled volume.
pub fn next_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut envelope_phase: i32 = engine.state.sound_channel_byte(8, channel_offset);
    engine.state.set_byte(
        ((157 + channel_offset) as u8 as i32),
        engine
            .state
            .byte((((ENVELOPE_TABLE + 1) + envelope_phase) as u16 as i32)),
    );
    {
        let mut envelope_delta: i32 = engine.state.sound_channel_byte(9, channel_offset);
        let mut accumulator: i32 =
            ((envelope_delta + engine.state.sound_channel_byte(12, channel_offset)) as u8 as i32);
        if ((envelope_delta & crate::bits::BIT7) != 0) {
            if (accumulator >= 16) {
                accumulator = 0;
            }
        } else {
            if (accumulator >= 16) {
                accumulator = 15;
            }
        }
        engine
            .state
            .set_sound_channel_byte(12, channel_offset, accumulator);
        engine.state.audio_duty_work = (accumulator as u8);
    }
    r.offset = ((engine.state.sound_channel_byte(13, channel_offset)) as u8);
    scale_envelope_volume(engine, r);
    {
        let mut volume_register: i32 = (((engine.state.sound_channel_byte(6, channel_offset)
            & crate::bits::HIGH_2_BITS)
            | (engine.state.audio_duty_work as i32)
            | crate::bits::BITS_4_5) as u8 as i32);
        r.value = (volume_register as u8);
    }
}

// Tick the phase duration. When it expires, advance four bytes in the
// envelope table; low nibbles >= 0x0C mark the terminal silent phase.
pub fn advance_envelope_phase(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut phase_low_nibble: i32 = 0;
    let mut next_phase: i32 = 0;
    let phase_timer =
        (engine.state.sound_channel_byte(11, channel_offset) - 1) & crate::bits::BYTE_MASK;
    engine
        .state
        .set_sound_channel_byte(11, channel_offset, phase_timer);
    if (phase_timer != 0) {
        r.index = (channel_offset as u8);
        r.carry = 0;
        return;
    }
    phase_low_nibble = engine.state.sound_channel_byte(8, channel_offset) & crate::bits::LOW_NIBBLE;
    if (phase_low_nibble >= 12) {
        r.index = (channel_offset as u8);
        r.value = (phase_low_nibble as u8);
        r.carry = 1;
        return;
    }
    next_phase = ((engine.state.sound_channel_byte(8, channel_offset) + 4) as u8 as i32);
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, next_phase);
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + next_phase) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 1) + next_phase) as u16 as i32)),
    );
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte((((ENVELOPE_TABLE + 2) + next_phase) as u16 as i32)),
    );
    r.index = (channel_offset as u8);
    r.offset = (next_phase as u8);
    r.carry = 0;
}

pub fn scene_assemble(engine: &mut Engine, r: &mut RoutineContext) {
    select_room_data_bank_and_pointers(engine, r);
    copy_room_tile_pages(engine, r);
    r.carry = (((if ((engine.state.room_metadef_hi as i32 + 3) > 255) {
        1
    } else {
        0
    }) as u8) as u8);
    text_attr_build(engine, r);
    build_room_palette_buffer(engine, r);
}

fn silence_sfx_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
    engine.device_write(
        crate::engine::reg::SQ2_VOL,
        (engine.state.sound_channel_byte(6, 64) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT1) as u8);
}

pub fn sfx_overlay_voice(engine: &mut Engine, r: &mut RoutineContext) {
    let mut start: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.prompt_state != 0) {
                    if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
                        start = 1;
                    } else if (engine.state.prompt_argument >= engine.state.sfx_priority) {
                        start = 1;
                    } else {
                        engine.state.prompt_argument = 0;
                        engine.state.prompt_state = 0;
                    }
                }
                if ((start) == 0) {
                    if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
                        return;
                    }
                    if (((engine.state.dec_sound_channel_byte(0, 64)) as u8 as i32) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                } else {
                    let mut sfx_table_index: i32 = 0;
                    engine.state.sfx_priority = engine.state.prompt_argument;
                    sfx_table_index = (((engine.state.prompt_state as i32) << 1) as u8 as i32);
                    engine.state.set_sound_channel_byte(
                        2,
                        64,
                        engine
                            .state
                            .byte(((SFX_POINTER_TABLE + sfx_table_index) as u16 as i32)),
                    );
                    engine.state.set_sound_channel_byte(
                        3,
                        64,
                        engine
                            .state
                            .byte((((SFX_POINTER_TABLE + 1) + sfx_table_index) as u16 as i32)),
                    );
                    engine.state.sfx_voice_active = 128;
                    engine.state.sound_channel_flags = ((engine.state.sound_channel_flags
                        | ((crate::bits::BIT6) as u8))
                        as u8 as u8);
                    engine.state.prompt_state = 0;
                    engine.state.prompt_argument = 0;
                }
                loop {
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 64)
                        | (engine.state.sound_channel_byte(3, 64) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    if (note_byte == 0) {
                        engine.state.sfx_voice_active = 0;
                        engine.state.sfx_priority = 0;
                        engine.state.sound_channel_flags =
                            engine.state.sound_channel_flags & ((crate::bits::CLEAR_BIT6) as u8);
                        silence_sfx_pulse2(engine, r);
                        return;
                    }
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 64, note_byte & crate::bits::LOW_7_BITS);
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        load_note_period(engine, r);
                        engine.state.sound_status_flags = 2 | engine.state.sound_status_flags;
                        engine.device_write(
                            crate::engine::reg::SQ2_SWEEP,
                            engine.state.sound_channel_byte(7, 64),
                        );
                        engine.device_write(
                            crate::engine::reg::SQ2_LO,
                            (engine.state.sound_command as i32),
                        );
                        engine.device_write(
                            crate::engine::reg::SQ2_HI,
                            (((engine.state.sound_length & ((crate::bits::LOW_3_BITS) as u8))
                                | ((crate::bits::HIGH_2_BITS) as u8))
                                as i32),
                        );
                        start_note_envelope(engine, r);
                    }
                    break;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                if ((engine.state.sound_status_flags & ((crate::bits::BIT1) as u8)) == 0) {
                    return;
                }
                if (((engine.state.dec_sound_channel_byte(10, 64)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ2_VOL, (r.value as i32));
                }
                advance_envelope_phase(engine, r);
                if ((r.carry) != 0) {
                    silence_sfx_pulse2(engine, r);
                }
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

pub fn song_init(engine: &mut Engine, r: &mut RoutineContext) {
    let mut song: i32 = (engine.state.song as i32);
    let mut idx: i32 = 0;
    let mut x: i32 = 0;
    let mut blk: i32 = 0;
    x = ((if (song < 10) { 10 } else { 12 }) as u8 as i32);
    engine.state.song_ptr_lo = (x as u8);
    engine.state.song_ptr_hi = ((x + 1) as u8);
    sound_set_song_banks(engine, r);
    engine.state.music_volume_override = 0;
    engine.state.prompt_state = 0;
    idx = ((if (song < 10) {
        song
    } else {
        ((song - 10) as u8 as i32)
    }) as u8 as i32);
    idx = ((idx << 1) as u8 as i32);
    {
        engine.state.indirect_ptr_lo = ((engine
            .state
            .byte(((SONG_POINTER_TABLE + idx) as u16 as i32)))
            as u8);
        engine.state.indirect_ptr_hi = ((engine
            .state
            .byte((((SONG_POINTER_TABLE + 1) + idx) as u16 as i32)))
            as u8);
    }
    engine.state.data_ptr_lo = 147;
    engine.state.data_ptr_hi = 0;
    {
        blk = 0;
        while (blk < 4) {
            let mut y: i32 = 0;
            let mut s: i32 = ((engine.state.indirect_ptr()) as u16 as i32);
            let mut d: i32 = ((engine.state.data_ptr()) as u16 as i32);
            {
                y = 7;
                while (y >= 0) {
                    engine.state.set_byte(
                        ((d + y) as u16 as i32),
                        engine.state.byte(((s + y) as u16 as i32)),
                    );
                    {
                        let __old = y;
                        y -= 1;
                        __old
                    };
                }
            }
            d = ((engine.state.data_ptr_lo + 8) as u16 as i32);
            engine.state.data_ptr_lo = (d as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((d >> 8) as u8);
            d = ((engine.state.data_ptr()) as u16 as i32);
            {
                y = 7;
                while (y >= 0) {
                    engine.state.set_byte(((d + y) as u16 as i32), 0);
                    {
                        let __old = y;
                        y -= 1;
                        __old
                    };
                }
            }
            d = ((engine.state.data_ptr_lo + 8) as u16 as i32);
            engine.state.data_ptr_lo = (d as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((d >> 8) as u8);
            s = ((engine.state.indirect_ptr_lo + 8) as u16 as i32);
            engine.state.indirect_ptr_lo = (s as u8);
            engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi + ((s >> 8) as u8);
            {
                let __old = blk;
                blk += 1;
                __old
            };
        }
    }
    ppu_commit_banks(engine, r);
}

pub fn sound_restore_game_banks(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.prg_bank_8000 as i32),
    );
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.prg_bank_a000 as i32),
    );
}

pub fn sound_set_default_banks(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 6;
    let mut y: i32 = 10;
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, x);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, y);
    x = ((x + 1) as u8 as i32);
    y = ((y + 1) as u8 as i32);
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, x);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, y);
    r.index = (x as u8);
    r.offset = (y as u8);
}

pub fn sound_set_song_banks(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.song_ptr_lo as i32),
    );
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.song_ptr_hi as i32),
    );
}

pub fn sound_tick(engine: &mut Engine, r: &mut RoutineContext) {
    sound_set_default_banks(engine, r);
    engine.state.sound_channel_offset = 64;
    r.value = 64;
    sfx_overlay_voice(engine, r);
    if (engine.state.sound_paused != 0) {
        if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
            engine.device_write(
                crate::engine::reg::SQ2_VOL,
                (engine.state.sound_channel_byte(6, 16) & crate::bits::HIGH_2_BITS)
                    | crate::bits::BITS_4_5,
            );
        }
        engine.device_write(
            crate::engine::reg::SQ1_VOL,
            (engine.state.sound_channel_byte(6, 0) & crate::bits::HIGH_2_BITS)
                | crate::bits::BITS_4_5,
        );
        engine.device_write(crate::engine::reg::TRI_LINEAR, 0);
        engine.device_write(crate::engine::reg::NOISE_VOL, 48);
        r.value = 48;
    } else {
        sound_set_song_banks(engine, r);
        engine.state.sound_channel_offset = 0;
        r.value = 0;
        tick_pulse1_channel(engine, r);
        engine.state.sound_channel_offset = 16;
        r.value = 16;
        tick_pulse2_channel(engine, r);
        engine.state.sound_channel_offset = 32;
        r.value = 32;
        tick_triangle_channel(engine, r);
        engine.state.sound_channel_offset = 48;
        r.value = 48;
        tick_noise_channel(engine, r);
    }
    sound_restore_game_banks(engine, r);
}

pub fn statusbar_split(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        (engine.state.ppu_mask_shadow as i32),
    );
    engine.state.ppu_ctrl_shadow = (engine.state.ppu_ctrl_shadow
        & ((crate::bits::CLEAR_BIT0) as u8))
        | engine.state.nametable_select;
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        (engine.state.ppu_ctrl_shadow as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_SCROLL,
        (engine.state.scroll_pixel_x as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_SCROLL,
        (engine.state.scroll_y as i32),
    );
    if (engine.state.statusbar_split_flag != 0) {
        let _ = engine.device_read(crate::engine::reg::PPU_STATUS);
        engine.device_write(
            crate::engine::reg::PPU_CTRL,
            ((engine.state.ppu_ctrl_shadow & ((crate::bits::CLEAR_BIT0) as u8)) as i32),
        );
        engine.device_write(crate::engine::reg::PPU_SCROLL, 0);
        engine.device_write(crate::engine::reg::PPU_SCROLL, 196);
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 1);
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 22);
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 4);
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 62);
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 5);
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 63);
    }
    sound_tick(engine, r);
    if (engine.state.statusbar_split_flag == 0) {
        return;
    }
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 1);
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        (engine.state.ppu_ctrl_shadow as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_SCROLL,
        (engine.state.scroll_pixel_x as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_SCROLL,
        (engine.state.scroll_y as i32),
    );
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, engine.state.chr_bank(1));
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 4);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, engine.state.chr_bank(4));
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 5);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, engine.state.chr_bank(5));
}

pub fn text_attr_build(engine: &mut Engine, r: &mut RoutineContext) {
    let mut p: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    let mut carry_in: i32 = (r.carry as u8 as i32);
    let mut b: i32 = 0;
    b = engine.state.byte(p);
    engine.state.tile_table_ptr_hi = ((b + 160 + carry_in) as u8);
    engine.state.tile_table_ptr_lo = 0;
    engine
        .state
        .set_chr_bank(3, engine.state.byte(((p + 1) as u16 as i32)));
    engine.state.text_attr_ptr_lo = ((engine.state.byte(((p + 2) as u16 as i32))) as u8);
    engine.state.text_attr_ptr_hi = ((engine.state.byte(((p + 3) as u16 as i32))) as u8);
    engine.state.room_tile_action = ((engine.state.byte(((p + 4) as u16 as i32))) as u8);
    engine
        .state
        .set_chr_bank(0, engine.state.byte(((p + 5) as u16 as i32)));
    engine
        .state
        .set_chr_bank(1, engine.state.byte(((p + 6) as u16 as i32)));
    {
        let mut ms_y: i32 = (engine.state.map_screen_y as i32);
        let mut ms_x: i32 = (engine.state.map_screen_x as i32);
        let mut idx: i32 = ((((ms_y << 2) & crate::bits::BIT2) | ms_x) as u8 as i32);
        let mut a: i32 = engine.state.save_payload(idx);
        let mut cnt: i32 = (((ms_y >> 1) + 1) as u8 as i32);
        let mut c: i32 = 0;
        loop {
            c = (((a >> 7) & 1) as u8 as i32);
            a = ((a << 1) as u8 as i32);
            if ({
                cnt -= 1;
                cnt
            } == 0)
            {
                break;
            }
        }
        r.value = (a as u8);
        r.carry = (c as u8);
    }
    {
        let mut y: i32 = 7;
        let mut a: i32 = 0;
        if ((r.carry) != 0) {
            a = engine.state.byte(((p + y) as u16 as i32));
        } else {
            a = 0;
        }
        engine.state.set_object_state(160, a);
        if (a != 0) {
            engine.state.set_object_attr(160, 1);
            {
                let __old = y;
                y += 1;
                __old
            };
            engine
                .state
                .set_object_x_tile(160, engine.state.byte(((p + y) as u16 as i32)));
            engine.state.set_object_x_sub(160, 0);
            {
                let __old = y;
                y += 1;
                __old
            };
            engine
                .state
                .set_object_y_pixel(160, engine.state.byte(((p + y) as u16 as i32)));
            {
                let __old = y;
                y += 1;
                __old
            };
            b = engine.state.byte(((p + y) as u16 as i32));
            if (b == 23) {
                engine.state.set_object_state(160, 25);
                engine.state.set_object_tile(160, 221);
            } else {
                engine.state.set_object_tile(160, 233);
            }
        }
    }
    {
        let mut x: i32 = (engine.state.song as i32);
        let mut do_d02e: i32 = 1;
        if (x < 5) {
            let mut a: i32 = 0;
            let mut c: i32 = 1;
            let mut i: i32 = (x);
            loop {
                let mut nc: i32 = (((a >> 7) & 1) as u8 as i32);
                a = (((a << 1) | c) as u8 as i32);
                c = nc;
                {
                    i -= 1;
                    i
                };
                if !(i >= 0) {
                    break;
                }
            }
            a = ((a & engine.state.byte(((p + 21) as u16 as i32))) as u8 as i32);
            if (a != 0) {
                do_d02e = 0;
            }
        }
        if ((do_d02e) != 0) {
            r.value = ((engine.state.byte(((p + 11) as u16 as i32))) as u8);
            switch_song_if_needed(engine, r);
        }
    }
    engine
        .state
        .set_temp_save(0, engine.state.byte(((p + 16) as u16 as i32)));
    engine
        .state
        .set_temp_save(1, engine.state.byte(((p + 17) as u16 as i32)));
    engine
        .state
        .set_temp_save(2, engine.state.byte(((p + 18) as u16 as i32)));
    engine
        .state
        .set_temp_save(3, engine.state.byte(((p + 19) as u16 as i32)));
    engine.state.family_member_mask = ((engine.state.byte(((p + 20) as u16 as i32))) as u8);
}

pub fn vblank_commit(engine: &mut Engine, r: &mut RoutineContext) {
    let save = *r;
    {
        engine.ppu.set_vblank(((1) != 0));
        engine.ppu.set_sprite0(
            ((if ((engine.state.ppu_mask_shadow & ((crate::bits::BITS_3_4) as u8)) != 0) {
                1
            } else {
                0
            }) != 0),
        );
        engine.ppu.eval_sprite_overflow();
    }
    {
        let __v = engine.device_read(crate::engine::reg::PPU_STATUS);
        engine.state.frame_status = (__v as u8);
    }
    engine.device_write(crate::engine::reg::OAM_ADDR, 0);
    engine.device_write(crate::engine::reg::OAM_DMA, 2);
    let mut req: i32 = (engine.state.nmi_vram_req as i32);
    if (req == 0) {
        vblank_commit_tail(engine, r);
        {
            *r = save;
            return;
        }
    }
    engine.state.nmi_vram_req = 0;
    if (req >= 7) {
        vblank_commit_tail(engine, r);
        {
            *r = save;
            return;
        }
    }
    {
        const jt_lo: [i32; 7] = [81, 82, 95, 144, 229, 52, 68];
        const jt_hi: [i32; 7] = [211, 210, 210, 210, 210, 211, 211];
        engine.state.saved_audio_handler_lo = ((jt_lo[req as usize]) as u8);
        engine.state.saved_audio_handler_hi = ((jt_hi[req as usize]) as u8);
    }
    let _ = engine.device_read(crate::engine::reg::PPU_STATUS);
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        (engine.state.vram_addr_hi as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        (engine.state.vram_addr_lo as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        ((engine.state.ppu_ctrl_shadow & ((crate::bits::BIT2) as u8)) as u8 as i32),
    );
    match req {
        1 => {
            vram_fill_run(engine, r);
        }
        2 => {
            vram_upload_palette(engine, r);
        }
        3 => {
            vram_upload_hud(engine, r);
        }
        4 => {
            vram_blit_stack(engine, r);
        }
        5 => {
            vram_copy_indirect(engine, r);
        }
        6 => {
            vram_poke2(engine, r);
        }
        _ => {}
    }
    *r = save;
}

pub fn vblank_commit_tail(engine: &mut Engine, r: &mut RoutineContext) {
    ppu_commit_banks(engine, r);
    statusbar_split(engine, r);
    if (engine.state.frame_counter != 0) {
        engine.state.frame_counter =
            (engine.state.frame_counter - 1) & ((crate::bits::BYTE_MASK) as u8);
    }
    frame_counters(engine, r);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_SELECT,
        (engine.state.mmc3_bank_select as i32),
    );
}

pub fn vram_blit_stack(engine: &mut Engine, r: &mut RoutineContext) {
    {
        let mut i: i32 = 0;
        while (i < 64) {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.inventory_item(160 + i),
            );
            {
                let __old = i;
                i += 1;
                __old
            };
        }
    }
    vblank_commit_tail(engine, r);
}

pub fn vram_copy_indirect(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (engine.state.inventory_upload_col as i32);
    let mut src: i32 = (((engine.state.vram_addr2_lo as i32)
        | ((engine.state.vram_addr2_hi as i32) << 8)) as u16 as i32);
    let mut y: i32 = 0;
    loop {
        engine.device_write(
            crate::engine::reg::PPU_DATA,
            engine.state.byte(((src + y) as u16 as i32)),
        );
        {
            let __old = y;
            y += 1;
            __old
        };
        if ({
            x -= 1;
            x
        } == 0)
        {
            break;
        }
    }
    vblank_commit_tail(engine, r);
}

pub fn vram_fill_run(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (engine.state.inventory_upload_col as i32);
    let mut a: i32 = (engine.state.vram_addr2_lo as i32);
    loop {
        engine.device_write(crate::engine::reg::PPU_DATA, a);
        if ({
            x -= 1;
            x
        } == 0)
        {
            break;
        }
    }
    vblank_commit_tail(engine, r);
}

pub fn vram_poke2(engine: &mut Engine, r: &mut RoutineContext) {
    engine.device_write(
        crate::engine::reg::PPU_DATA,
        (engine.state.vram_addr2_hi as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_DATA,
        (engine.state.vram_addr2_lo as i32),
    );
    vblank_commit_tail(engine, r);
}

pub fn vram_upload_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        ((engine.state.ppu_ctrl_shadow | ((crate::bits::BIT2) as u8)) as u8 as i32),
    );
    {
        x = 23;
        while (x >= 0) {
            engine.device_write(crate::engine::reg::PPU_DATA, engine.state.vram_stage(x));
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        (engine.state.vram_addr_hi as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        ((engine.state.vram_addr_lo + 1) as u8 as i32),
    );
    {
        x = 23;
        while (x >= 0) {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.vram_stage(24 + x),
            );
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    {
        x = 10;
        while (x >= 0) {
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr2_hi as i32),
            );
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                engine.state.vram_stage(48 + x),
            );
            let _ = engine.device_read(crate::engine::reg::PPU_DATA);
            {
                let mut v: i32 = (((engine.device_read(crate::engine::reg::PPU_DATA)
                    & (engine.state.vram_addr2_lo as i32))
                    | engine.state.vram_stage(49 + x)) as u8
                    as i32);
                engine.device_write(
                    crate::engine::reg::PPU_ADDR,
                    (engine.state.vram_addr2_hi as i32),
                );
                engine.device_write(
                    crate::engine::reg::PPU_ADDR,
                    engine.state.vram_stage(48 + x),
                );
                engine.device_write(crate::engine::reg::PPU_DATA, v);
            }
            x -= 2;
        }
    }
    vblank_commit_tail(engine, r);
}

pub fn vram_upload_palette(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 0;
    engine.device_write(crate::engine::reg::PPU_ADDR, 63);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);
    {
        y = 0;
        while (y < 32) {
            engine.device_write(crate::engine::reg::PPU_DATA, engine.state.palette_buffer(y));
            {
                let __old = y;
                y += 1;
                __old
            };
        }
    }
    engine.device_write(crate::engine::reg::PPU_ADDR, 63);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);
    vblank_commit_tail(engine, r);
}
