// Rust game routine module. The functions here are the checked-in native game logic.
//
// Numbered `routine_####` names are retained as stable port labels while the
// original game systems are being identified. Keep semantic discoveries in
// `docs/routine_catalog.md` first, then rename or alias routines only after the
// dataflow is understood well enough to make the name useful.
use crate::engine::RoutineFn;
use crate::frame;
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

/// Runs `action` with the large-actor asset banks (PRG banks 12/13) swapped
/// into the `$8000`/`$A000` windows, then restores the previous banks.
///
/// This is the bank-trampoline used by routines that need the bank-0C/0D data
/// (large-actor/final-exit assets): the prior bank shadows are saved off,
/// banks 12 and 13 are mapped, the closure runs against that data, and the
/// original mapping is put back so the caller is unaffected.
fn with_large_actor_asset_banks<F>(engine: &mut Engine, r: &mut RoutineContext, action: F)
where
    F: FnOnce(&mut Engine, &mut RoutineContext),
{
    // Save the current $8000/$A000 PRG bank shadows so they can be restored.
    let saved_bank6: i32 = (engine.state.prg_bank_8000 as i32);
    let saved_bank7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.saved_prg_bank_8000 = (saved_bank6 as u8);
    engine.state.saved_prg_bank_a000 = (saved_bank7 as u8);
    // Map PRG banks 12/13 (the large-actor/final-exit data banks) into
    // $8000/$A000 via MMC3 register 7, then apply the shadow to the mapper.
    engine.state.prg_bank_8000 = 12; // PRG bank 12 -> $8000 window
    engine.state.prg_bank_a000 = 13; // PRG bank 13 -> $A000 window
    engine.state.mmc3_bank_select = 7; // MMC3 bank-select latch = R7 ($A000)
    engine.prg_map_shadow();
    action(engine, r);
    // Restore the caller's original banks and leave the latch at R6 ($8000).
    engine.state.prg_bank_a000 = (saved_bank7 as u8);
    engine.state.prg_bank_8000 = (saved_bank6 as u8);
    engine.state.mmc3_bank_select = 6; // MMC3 bank-select latch = R6 ($8000)
    engine.prg_map_shadow();
}

/// Bank trampoline that maps PRG bank 9 into the `$A000` window, resolves the
/// current room tile source pointer, and stages a room-column VRAM upload.
///
/// Used by the scripted-scroll path: the room layout data lives in bank 9, so
/// it is swapped in over the `$A000` (R7) window, the column is built, and the
/// caller's previous R7 bank is restored. On exit `r.value` (A) holds the
/// restored bank number.
pub fn farcall_bank_09_r7(engine: &mut Engine, r: &mut RoutineContext) {
    // Save the current $A000 (R7) bank, then map bank 9 over that window.
    let mut saved_r7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.mmc3_bank_select = 7; // select MMC3 R7 ($A000 window)
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = 9; // PRG bank 9 (room layout data)
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 9);
    // Resolve the room tile source pointer (data_ptr hi = 0) and stage the
    // column upload from it.
    engine.state.data_ptr_hi = 0;
    r.value = 0;
    resolve_room_tile_pointer(engine, r);
    queue_room_column_vram_upload(engine, r);
    // Restore the caller's original $A000 bank.
    engine.state.mmc3_bank_select = 7; // select MMC3 R7 again to write it back
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = (saved_r7 as u8);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, saved_r7);
    r.value = (saved_r7 as u8);
}

/// Saves the current PRG banks and maps banks 12/13 into `$8000`/`$A000` so a
/// subsequent bank-0C/0D far call can run; pairs with [`farcall_return_home`].
///
/// Unlike [`with_large_actor_asset_banks`], this leaves the new banks mapped on
/// return (the caller does its work then calls `farcall_return_home`). On exit
/// `r.value` (A) = 13 and `r.offset` (Y) = 7 from the final register writes.
pub fn farcall_bank_0C0D_seed(engine: &mut Engine, r: &mut RoutineContext) {
    // Remember the caller's $8000/$A000 banks for farcall_return_home.
    engine.state.saved_prg_bank_8000 = engine.state.prg_bank_8000;
    engine.state.saved_prg_bank_a000 = engine.state.prg_bank_a000;
    // Map PRG bank 12 into $8000 via MMC3 R6.
    engine.state.mmc3_bank_select = 6; // select MMC3 R6 ($8000 window)
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.state.prg_bank_8000 = 12; // PRG bank 12 -> $8000
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 12);
    // Map PRG bank 13 into $A000 via MMC3 R7.
    engine.state.mmc3_bank_select = 7; // select MMC3 R7 ($A000 window)
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.state.prg_bank_a000 = 13; // PRG bank 13 -> $A000
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 13);
    r.value = 13;
    r.offset = 7;
}

/// Restores the `$8000`/`$A000` PRG banks saved by [`farcall_bank_0C0D_seed`],
/// returning the mapping to the caller's original banks.
///
/// Note: this only rewrites the bank shadows; it does not re-emit the MMC3
/// register writes, matching the original routine's behavior.
pub fn farcall_return_home(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prg_bank_a000 = engine.state.saved_prg_bank_a000;
    engine.state.prg_bank_8000 = engine.state.saved_prg_bank_8000;
}

/// Ticks the frame prescaler at `0x84` and decrements the eight coarse
/// timers at `0x85..0x8C` once per 60 frames.
///
/// Decrements the prescaler every call; only when it underflows to zero are
/// the coarse timers serviced (each nonzero one counts down by one) and the
/// prescaler reloaded to 60. On the reload tick `r.index` (X) returns 255.
pub fn frame_counters(engine: &mut Engine, r: &mut RoutineContext) {
    // Decrement the per-frame prescaler with 8-bit wrap; bail until it hits 0.
    let prescaler_after = (engine.state.frame_prescaler - 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.frame_prescaler = prescaler_after;
    if (prescaler_after != 0) {
        return;
    }
    // Once per second (60 frames): tick down each of the 8 coarse timers
    // (indices 7..0) that is still nonzero.
    for timer_index in (0..=7).rev() {
        if (engine.state.coarse_timer(timer_index) != 0) {
            engine.state.set_coarse_timer(
                timer_index,
                (engine.state.coarse_timer(timer_index) - 1) & crate::bits::BYTE_MASK,
            );
        }
    }
    // Reload the prescaler for the next second; X=255 signals the tick.
    engine.state.frame_prescaler = 60; // 60 frames = ~1 second (NTSC)
    r.index = 255;
}

/// Per-frame player update: handles special-exit transitions, the character
/// select / start menu, item use, fall/jump/walk movement with collision, the
/// item-select sub-loop, and finally the player pose/animation.
///
/// This is the original routine's branchy player dispatcher modeled as a state
/// machine over the `state` cursor:
///  - 0: special-exit handoff check
///  - 1: input read, direction latch, fall/jump/walk movement
///  - 2/3: commit the projected player position after a successful move
///  - 4: blocked move -> clear jump/fall, settle terrain contact
///  - 5: Select-button item-selection sub-loop
///  - 6: update pose and walk animation, then finish the frame
/// `engine.lotw_nonlocal_handoff` short-circuits the frame when a sub-routine
/// transfers control elsewhere (e.g. room transition).
pub fn game_update(engine: &mut Engine, r: &mut RoutineContext) {
    // Start each frame with a clean non-local handoff. On the 6502 the "abort
    // the rest of the handler" effect is a per-call stack manipulation (PLA;PLA),
    // so it never persists across frames. The port emulates it with a flag that
    // is only reset on hard reset; without clearing it here, a handoff set during
    // one frame (e.g. tick_player_jump_action mid-jump) lingered and made the next
    // frame's Up/overhead-tile check ($8BBC dispatch) early-return, freezing the
    // player in mid-jump while a direction was held.
    engine.lotw_nonlocal_handoff = 0;
    let mut a: i32 = 0;
    let mut y: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // No active object slot selected yet.
                engine.state.slot_index = 255;
                // A pending special exit takes priority over normal play.
                if (engine.state.pending_special_exit != 0) {
                    enter_pending_special_exit_room(engine, r);
                    return;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                check_final_exit_trigger(engine, r);
                if engine.state.final_exit_flag != 0 {
                    // $D67A PLA; PLA: when the ending trigger fires it does a
                    // non-local return two levels up, aborting the rest of this
                    // per-frame input/movement handler for the frame.
                    return;
                }
                // Start button (bit4) opens the character-select overlay.
                if ((engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0) {
                    run_character_select_overlay(engine, r);
                    return;
                }
                tick_selected_item_effect(engine, r);
                // While the post-landing lock timer runs, suppress all input.
                if (engine.state.landing_timer != 0) {
                    engine.state.landing_timer =
                        (engine.state.landing_timer - 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.buttons = 0;
                }
                {
                    // Decide whether to clear the high nibble (held-direction
                    // bits) of the direction latch. Character 4 (the special
                    // flying character) keeps the latch every 8th frame while
                    // the action button is held; otherwise the latch high
                    // nibble is cleared unless the action button (bit6) is held.
                    let mut clear_hi: i32 = 1;
                    if (engine.state.character_index == 4) {
                        // Character index 4 = the levitating family member.
                        if ((engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0)
                        {
                            // Every 8th frame (low 3 prescaler bits == 0).
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
                // Latch the freshly pressed D-pad direction (low nibble) into
                // the low nibble of the direction latch.
                a = ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) as i32);
                if (a != 0) {
                    engine.state.scratch0 = (a as u8);
                    engine.state.direction_latch = (engine.state.direction_latch
                        & ((crate::bits::HIGH_NIBBLE) as u8))
                        | (a as u8);
                }
                // Select button (bit5) -> item-selection sub-loop (state 5).
                if ((engine.state.buttons & ((crate::bits::BIT5) as u8)) != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                // B button (bit3) -> use/interact with the tile ahead.
                if ((engine.state.buttons & ((crate::bits::BIT3) as u8)) != 0) {
                    dispatch_overhead_tile_action(engine, r);
                    if ((engine.lotw_nonlocal_handoff) != 0) {
                        return;
                    }
                }
                // Count owned consumable items in slots $28.. (inventory_item
                // 40..44); Y becomes the movement-delta step count, capped at 6.
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
                // Falling: vertical delta = (fall_frames / 4) + 1, accelerating
                // the fall. Try the full move, then a horizontal-only move.
                if (engine.state.fall_frames != 0) {
                    engine.state.vertical_delta = (engine.state.fall_frames >> 2) + 1;
                    try_move_player_with_collision(engine, r);
                    if ((r.carry) == 0) {
                        {
                            // Clear carry = move succeeded -> commit position.
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    // Full move blocked: retry with horizontal motion zeroed so
                    // the player lands straight down.
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
                        // Still blocked: land (state 4 clears jump/fall).
                        state = 4;
                        continue 'dispatch;
                    }
                }
                // Not falling: advance an in-progress jump, start one on the
                // jump button (bit7), or clear jump state when idle.
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
                // Apply the horizontal/walk movement with collision.
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        // Move succeeded -> commit (state 2 -> 3).
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Blocked: try to align the player to the tile boundary.
                try_nudge_player_to_tile_boundary(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                {
                    // Fully blocked -> settle (state 4).
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
                // Commit the projected position (indirect_ptr = X fine/tile,
                // scratch2 = Y). Wrap Y back to 0 if it ran past the bottom row.
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                a = (engine.state.scratch2 as i32);
                if (a >= 239) {
                    // 239 = past the bottom visible scanline; wrap to top.
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
                // Landed / blocked: stop any jump or fall and re-evaluate the
                // terrain contact at the current position.
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
                // Item-selection sub-loop (Select held). prompt_state 16 marks
                // the cursor display mode.
                engine.state.prompt_state = 16;
                loop {
                    read_debounced_buttons(engine, r);
                    // Exit when any face/start/select button (high nibble) is
                    // pressed.
                    if ((r.value & ((crate::bits::HIGH_NIBBLE) as u8)) != 0) {
                        break;
                    }
                    // Ignore frames with no left/right (low 2 bits) input.
                    if ((engine.state.buttons & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        continue;
                    }
                    // Shift buttons left twice to remap D-pad bits into the
                    // up/down delta lanes for build_input_movement_delta.
                    engine.state.buttons = ((((engine.state.buttons as i32) << 1)
                        & (((crate::bits::BYTE_MASK) as u8) as i32))
                        as u8);
                    engine.state.buttons = ((((engine.state.buttons as i32) << 1)
                        & (((crate::bits::BYTE_MASK) as u8) as i32))
                        as u8);
                    r.offset = 1; // single step
                    build_input_movement_delta(engine, r);
                    {
                        // Move the selection cursor by the vertical delta,
                        // clamping into the 0..3 item-slot range (a negative
                        // result snaps to the last slot, 3).
                        let mut t: i32 = ((engine.state.vertical_delta
                            + engine.state.selected_item_slot)
                            as u8 as i32);
                        let mut ni: i32 = 0;
                        if ((t & crate::bits::BIT7) != 0) {
                            ni = 3; // wrapped below 0 -> last slot
                        } else if (t < 4) {
                            ni = t; // 0..3 in range
                        } else {
                            ni = 0; // wrapped above 3 -> first slot
                        }
                        engine.state.selected_item_slot = (ni as u8);
                    }
                    engine.state.prompt_state = 12; // redraw cursor selection
                }
                engine.state.prompt_state = 16;
                state = 6;
                continue 'dispatch;
            }
            6 => {
                // Finish the frame: pick the pose from motion and advance the
                // walk animation cycle.
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
///
/// Increments the low byte (sound-channel field 2) of the selected channel's
/// stream pointer; on an 8-bit carry (low byte wrapped to 0) it also bumps the
/// high byte (field 3). `r.index` (X) returns the channel offset.
pub fn increment_selected_music_stream_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    let channel_pointer_offset: i32 = (engine.state.sound_channel_offset as i32);
    // Increment the pointer low byte (field 2) with 8-bit wrap.
    let pattern_ptr_lo =
        (engine.state.sound_channel_byte(2, channel_pointer_offset) + 1) & crate::bits::BYTE_MASK;
    engine
        .state
        .set_sound_channel_byte(2, channel_pointer_offset, pattern_ptr_lo);
    // Carry into the pointer high byte (field 3) when the low byte wrapped.
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

/// Bank-0C/0D far-call trampoline: maps PRG banks 12/13 over `$8000`/`$A000`,
/// seeds the indirect pointer (`lo`/`hi` into `0x0E`/`0x0F`), invokes `target`,
/// then restores the caller's banks.
///
/// `lo`/`hi` give the routine its working pointer/argument; `target` is the
/// routine that runs against the bank-0C/0D data (e.g. the title-screen loop).
fn farcall_0C0D(
    engine: &mut Engine,
    r: &mut RoutineContext,
    mut lo: i32,
    mut hi: i32,
    target: RoutineFn,
) {
    // Save the caller's $8000/$A000 banks.
    let saved_bank_6: i32 = (engine.state.prg_bank_8000 as i32);
    let saved_bank_7: i32 = (engine.state.prg_bank_a000 as i32);
    engine.state.saved_prg_bank_8000 = (saved_bank_6 as u8);
    engine.state.saved_prg_bank_a000 = (saved_bank_7 as u8);
    // Seed the indirect pointer ($0E/$0F) the target routine reads.
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    // Map banks 12/13 (bank-0C/0D data) and run the target.
    engine.state.prg_bank_8000 = 12; // PRG bank 12 -> $8000
    engine.state.prg_bank_a000 = 13; // PRG bank 13 -> $A000
    engine.state.mmc3_bank_select = 7; // latch = R7
    engine.prg_map_shadow();
    target(engine, r);
    // Restore the caller's banks, latch back to R6.
    engine.state.prg_bank_a000 = (saved_bank_7 as u8);
    engine.state.prg_bank_8000 = (saved_bank_6 as u8);
    engine.state.mmc3_bank_select = 6; // latch = R6
    engine.prg_map_shadow();
}

/// Performs the cold-start initialization path and enters the main game
/// dispatcher after the title screen flow completes.
///
/// Disables rendering/audio, configures the APU and MMC3 mirroring, seeds the
/// bank-0C/0D mapping, clears RAM from the ROM default tables, runs the title
/// screen, then places the player at the starting position and hands off to the
/// main loop. Does not return during normal play.
pub fn main_init(engine: &mut Engine, r: &mut RoutineContext) {
    // Turn off PPU rendering and NMI; silence the DMC.
    engine.device_write(crate::engine::reg::PPU_CTRL, 0);
    engine.device_write(crate::engine::reg::PPU_MASK, 0);
    engine.device_write(crate::engine::reg::DMC_FREQ, 0);
    // Enable all 5 APU channels (bits 0-4 = 0x1F) and set the frame counter
    // to 4-step + IRQ inhibit (0xC0).
    engine.state.sound_status_flags = 31; // 0x1F: enable pulse1/2, tri, noise, DMC
    engine.device_write(crate::engine::reg::APU_STATUS, 31);
    engine.device_write(crate::engine::reg::APU_FRAME, 192); // 0xC0
    engine.device_write(crate::engine::reg::MMC3_MIRROR, 0);
    // Map the bank-0C/0D data, clear RAM from defaults, run the title screen.
    farcall_bank_0C0D_seed(engine, r);
    ram_state_init(engine, r);
    farcall_0C0D(engine, r, 100, 174, run_title_screen_loop); // ptr = $AE64
    // Place the player at the game's starting screen position.
    engine.state.landing_timer = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 48; // starting scroll column
    engine.state.player_x_tile = 60; // starting player tile X
    engine.state.player_y = 160; // starting player Y (pixels)
    scene_assemble(engine, r);
    // Prime input with bit3 (B) set and run one update, then enter the loop.
    engine.state.buttons = 8; // 0x08 = bit3 (B button)
    game_update(engine, r);
    main_loop_dispatch(engine, r);
}

/// Stages one room-column upload from the current room tile source pointer
/// and queues VRAM job `0x03`.
///
/// The nametable bytes are written to `0x0140` and `0x0158`; the matching
/// attribute byte addresses and masks are written to `0x0170..0x017B`.
pub fn queue_room_column_vram_upload(engine: &mut Engine, r: &mut RoutineContext) {
    // Source = the room metatile column; tileset_quads = the 4-byte-per-metatile
    // CHR-tile table the metatile ids index into.
    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let tileset_quads_ptr: i32 = ((engine.state.tile_table_ptr()) as u16 as i32);

    // Expand 12 metatiles into 24 nametable bytes (two 12-tile screen columns).
    // Each metatile id selects a 4-tile quad (top-left, top-right, bottom-left,
    // bottom-right); the four CHR tiles are staged into two adjacent columns.
    engine.state.scratch3 = 0;
    for staging_offset in (0..=22).rev().step_by(2) {
        let metatile_id: i32 = engine
            .state
            .byte((source_ptr + (engine.state.scratch3 as i32)) as u16 as i32);
        // metatile_id * 4 = byte offset of this metatile's quad in the table.
        let tile_quad_offset: i32 = (((metatile_id << 2) as u8 as i32) as u16 as i32);
        // Quad byte 0 -> first column, top tile.
        engine.state.set_vram_stage(
            1 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 0) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        // Quad byte 1 -> first column, bottom tile.
        engine.state.set_vram_stage(
            staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 1) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        // Quad byte 2 -> second column (+24 stage offset), top tile.
        engine.state.set_vram_stage(
            25 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 2) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        // Quad byte 3 -> second column, bottom tile.
        engine.state.set_vram_stage(
            24 + staging_offset,
            engine.state.byte(
                ((tileset_quads_ptr + ((tile_quad_offset + 3) & crate::bits::BYTE_MASK)) as u16
                    as i32),
            ),
        );
        engine.state.scratch3 = (engine.state.scratch3 + 1) & ((crate::bits::BYTE_MASK) as u8);
    }

    // Second VRAM target = attribute table, 3 pages above the nametable base.
    engine.state.vram_addr2_hi = engine.state.vram_addr_hi + 3;
    let destination_low_byte: i32 = (engine.state.vram_addr_lo as i32);
    // Attribute address low byte: (col/4) + 0xC0 (attribute area offset).
    engine.state.scratch3 = (((destination_low_byte >> 2) + 192) as u8);

    // bit1 of the column selects which half of each attribute byte to update;
    // the low-byte step (0x33 vs 0xCC) advances by the corresponding columns.
    let attribute_side_mask: i32 = ((destination_low_byte & crate::bits::BIT1) as u8 as i32);
    engine.state.vram_addr2_lo = if ((attribute_side_mask) != 0) {
        51 // 0x33: right-side attribute step
    } else {
        204 // 0xCC: left-side attribute step
    };

    // Build the 6 attribute bytes (one per 2 metatile rows). Each combines the
    // top metatile's palette bits (bits 6-7 -> shifted to bits 2-3) with the
    // bottom metatile's palette bits (bits 6-7), then shifts into the correct
    // 2x2 quadrant for the left half of the attribute byte.
    let mut source_attribute_offset: i32 = 0;
    for attribute_offset in (0..=10).rev().step_by(2) {
        engine
            .state
            .set_vram_stage(48 + attribute_offset, (engine.state.scratch3 as i32));
        engine.state.scratch3 = engine.state.scratch3 + 8; // next attribute row (8 cols down)

        let top_metatile_id: i32 = engine
            .state
            .byte((source_ptr + source_attribute_offset) as u16 as i32);
        source_attribute_offset += 1;
        // Top metatile palette bits (0xC0) shifted down into bits 2-3.
        let mut attribute_bits: i32 =
            (((top_metatile_id & crate::bits::HIGH_2_BITS) >> 4) as u8 as i32);

        let bottom_metatile_id: i32 = engine
            .state
            .byte((source_ptr + source_attribute_offset) as u16 as i32);
        source_attribute_offset += 1;
        // OR in bottom metatile palette bits (0xC0) at bits 6-7.
        attribute_bits =
            (((bottom_metatile_id & crate::bits::HIGH_2_BITS) | attribute_bits) as u8 as i32);

        // Left half of the attribute byte uses the lower two quadrants:
        // shift the assembled bits down by 2.
        if (attribute_side_mask == 0) {
            attribute_bits = ((attribute_bits >> 2) as u8 as i32);
        }
        engine
            .state
            .set_vram_stage(49 + attribute_offset, attribute_bits);
    }

    // Queue VRAM job 0x03 (room column upload) and wait for it to drain.
    r.value = 3;
    queue_ppu_job_and_wait(engine, r);
}

/// Writes the eight PPU bank shadows at `0x2A..0x31` to the mapper.
///
/// Flushes the cached CHR bank values (registers R0..R7) to the MMC3 by
/// selecting each register and writing its data. `r.index` (X) returns 255.
pub fn ppu_commit_banks(engine: &mut Engine, r: &mut RoutineContext) {
    // For each MMC3 bank register 7..0, select it then write its CHR bank.
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
    // Copy the 256-byte zero-page default image ($00..$FF) from ROM.
    for zero_page_addr in 0..=255 {
        engine.state.set_byte(
            zero_page_addr,
            engine.state.byte(ZP_INIT_TABLE + zero_page_addr),
        );
    }

    // Copy the 64-byte stack-page default image into the work area at $01A0..
    // (inventory_item indices 160..223).
    for stack_offset in (0..=63).rev() {
        engine.state.set_inventory_item(
            160 + stack_offset,
            engine.state.byte(STACK_INIT_TABLE + stack_offset),
        );
    }

    // Clear the 32-byte palette buffer to color 0x0F (black).
    for palette_offset in (0..=31).rev() {
        engine.state.set_palette_buffer(palette_offset, 15); // 0x0F = black
    }

    // Copy the 256-byte save-state template into the save payload.
    for save_ram_offset in 0..=255 {
        engine.state.set_save_payload(
            save_ram_offset,
            engine.state.byte(SAVE_INIT_TABLE + save_ram_offset),
        );
    }

    // Copy the 256-byte object-table template into the persistent object page
    // (password-nibble buffer A at offset 222..).
    for object_ram_offset in 0..=255 {
        engine.state.set_password_nibbles_a(
            222 + object_ram_offset,
            engine.state.byte(OBJECT_INIT_TABLE + object_ram_offset),
        );
    }
}

/// Polls both controller ports and stores the merged button state in
/// `0x20`, using replay input when one is configured.
///
/// Strobes the controllers, then shifts in 8 bits from each of the two ports
/// ($4016 = player 1, $4017/APU_FRAME = player 2). The two ports are OR-ed
/// together so either controller can drive the game. When a replay/demo input
/// source is active it overrides the live port read.
pub fn read_controllers(engine: &mut Engine, r: &mut RoutineContext) {
    // Safety valve: a game loop that polls input without ever advancing a frame
    // (e.g. character-select "walk until A" on a non-selectable tile with A held)
    // would spin the coroutine forever with no vblank. This forces a frame yield
    // if input is polled far more than any real frame would; inert otherwise.
    frame::input_poll_watchdog(engine, r);
    // Replay/demo override: feed scripted buttons into the port read.
    if let Some(replay_buttons) = engine.next_input() {
        engine.ppu.buttons = (replay_buttons as u8);
    }
    // Strobe the controller shift registers (write 1 then 0 to $4016).
    engine.device_write(crate::engine::reg::JOY1, 1);
    engine.device_write(crate::engine::reg::JOY1, 0);

    // Shift in 8 button bits (A,B,Select,Start,Up,Down,Left,Right) MSB-first.
    for _ in 0..8 {
        // bit0 of $4016 = player 1, bit0 of $4017 = player 2; OR captures both.
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

    // Merge both controllers so either pad's buttons count.
    engine.state.buttons = engine.state.buttons | engine.state.button_chord;
}

/// Hardware reset entry point: resets the MMC3 (bank latch, PRG-RAM protect,
/// IRQ) and falls into the cold-start initialization in [`main_init`].
pub fn reset(engine: &mut Engine, r: &mut RoutineContext) {
    // Clear the MMC3 bank-select latch, PRG-RAM protect, and disable IRQs.
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 0);
    engine.device_write(crate::engine::reg::MMC3_PRG_RAM, 0);
    engine.device_write(crate::engine::reg::MMC3_IRQ_DISABLE, 0);
    main_init(engine, r);
}

/// Advances the two-byte RNG seed at `0x3A..0x3B` until the generated value
/// is below the limit supplied in `r.value`.
///
/// `r.value` (A) on entry is the exclusive upper bound; on exit it holds a
/// pseudo-random value in `0..limit`. A limit of 0 is treated as "no rolling":
/// the current high byte is returned unchanged. Otherwise the 16-bit seed is
/// run through a shift+add LCG-style step (masking the result to 7 bits, so the
/// usable range is 0..127) and re-rolled until it falls below the limit.
pub fn rng_update(engine: &mut Engine, r: &mut RoutineContext) {
    let limit: i32 = (r.value as u8 as i32);
    engine.state.rng_limit = (limit as u8);
    // Limit 0: return 0 without advancing ($CC66 BEQ $CC8E returns A = the
    // just-stored zero limit, not the seed).
    if (limit == 0) {
        r.value = 0;
        return;
    }
    let mut rng_high: i32 = (engine.state.rng_high as i32);
    let mut rng_low: i32 = (engine.state.rng_low as i32);
    loop {
        engine.state.rng_seed_scratch = (rng_low as u8);

        // seed = (seed << 1) + 1 (16-bit), splitting hi/lo bytes.
        let shifted_seed: i32 =
            ((((((rng_high << 8) | rng_low) as u16 as i32) << 1) + 1) as u16 as i32);
        rng_high = ((shifted_seed >> 8) as u8 as i32);
        rng_low = (shifted_seed as u8 as i32);

        // Add the original low byte back in, tracking the carry out.
        let low_sum: i32 = ((rng_low + (engine.state.rng_low as i32)) as u16 as i32);
        rng_low = (low_sum as u8 as i32);
        let carry: i32 = ((low_sum >> 8) as u8 as i32);

        // candidate = high + original high + carry + saved low, masked to 7
        // bits (0..127).
        let mut candidate: i32 = ((rng_high + (engine.state.rng_high as i32) + carry) as u8 as i32);
        candidate = ((candidate + (engine.state.rng_seed_scratch as i32)) as u8 as i32);
        candidate &= 127; // 0x7F: keep result in 0..127

        rng_high = candidate;
        engine.state.rng_high = (candidate as u8);
        engine.state.rng_low = (rng_low as u8);
        // Re-roll until the value is within the requested range.
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
    // At the start of a 9-slice run, set up the VRAM destination and source
    // column for the half-screen indicated by the (flipped) nametable select.
    if (engine.state.obj_x_tile == 0) {
        engine.state.vram_addr_lo = 14; // column start within nametable row
        engine.state.vram_addr_hi = 32; // 0x20 = nametable base high byte
        // OR the chosen nametable's high bits (select^1, shifted into bit2-3)
        // into the VRAM destination high byte.
        engine.state.vram_addr_hi =
            (((((engine.state.nametable_select ^ ((crate::bits::BIT0) as u8)) << 2) as u8 as i32)
                | (engine.state.vram_addr_hi as i32)) as u8);
        // Source column = scroll_tile_x within the opposite half: (select^1)<<4
        // selects the 16-column half, +7 offsets to the leading edge.
        engine.state.obj_x_sub =
            ((((((engine.state.nametable_select ^ ((crate::bits::BIT0) as u8)) << 4) + 7) as u8
                as i32)
                | (engine.state.scroll_tile_x as i32)) as u8);
        engine.state.obj_x_tile = 9; // 9 slices per half-screen run
    }
    // Build and upload one column from the current source position.
    engine.state.data_ptr_lo = engine.state.obj_x_sub;
    farcall_bank_09_r7(engine, r);
    // Advance the VRAM destination by 2 (two-tile-wide metatile column) and the
    // source by one; count down the remaining slices.
    engine.state.vram_addr_lo = engine.state.vram_addr_lo + 1;
    engine.state.vram_addr_lo = engine.state.vram_addr_lo + 1;
    engine.state.obj_x_sub = engine.state.obj_x_sub + 1;
    engine.state.obj_x_tile = engine.state.obj_x_tile - 1;
    // Run complete: flip to the other nametable half for the next run.
    if (engine.state.obj_x_tile == 0) {
        engine.state.nametable_select = engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
    }
}

/// Updates the three final-exit projectile slots at `0x0410..0x043F`,
/// spawning a new shot on the action-button edge when a slot is empty.
pub fn update_final_exit_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Iterate object slots 1..3 (records at $0410, $0420, $0430). The slot
    // pointer starts at $0410 (lo=0x10, hi=0x04).
    engine.state.slot_index = 1;
    engine.state.obj_slot_ptr_lo = 16; // 0x10 = slot 1 record low byte
    engine.state.obj_slot_ptr_hi = 4; // 0x04 = object page ($04xx)
    loop {
        let slot_ptr = ((engine.state.obj_slot_ptr()) as u16 as i32);
        // Record byte +1 (state) nonzero -> the slot is active; tick it.
        if (engine.state.byte((slot_ptr + 1) as u16 as i32) != 0) {
            update_final_exit_projectile_slot(engine, r);
        // Empty slot + fresh action-button press (bit6 held now but not in
        // the latch) -> fire a new projectile.
        } else if (((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0)
            && ((engine.state.direction_latch & ((crate::bits::BIT6) as u8)) == 0))
        {
            spawn_final_exit_projectile(engine, r);
        }
        engine.state.slot_index = engine.state.slot_index + 1;
        {
            // Advance the slot pointer by 16 bytes (one object record),
            // carrying into the high byte.
            let next_slot_ptr = ((engine.state.obj_slot_ptr_lo as i32 + 16) as u16 as i32);
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
    // Load the slot's record into the obj_* scratch fields.
    load_object_slot_scratch(engine, r);
    // Record the action button (bit6) into the direction latch so the press is
    // consumed (prevents auto-fire while held).
    engine.state.direction_latch =
        (engine.state.buttons & ((crate::bits::BIT6) as u8)) | engine.state.direction_latch;
    // Build velocity from the latched direction (2 movement-table steps).
    r.value = (engine.state.direction_latch as u8);
    r.offset = 2; // 2 delta-table accumulation steps -> projectile speed
    build_final_exit_projectile_velocity(engine, r);
    // Project the spawn point ahead of the player and bounds-check it.
    project_final_exit_projectile_spawn(engine, r);
    check_final_exit_projectile_bounds(engine, r);
    // In bounds (carry clear): initialize the new projectile record.
    if ((r.carry) == 0) {
        engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
        engine.state.obj_y_pixel = engine.state.scratch2;
        engine.state.obj_state = 24; // 0x18 = projectile lifetime (frames)
        engine.state.obj_attr = 0;
        engine.state.obj_tile = 33; // 0x21 = projectile base tile id
        engine.state.prompt_state = 25; // fire sound/effect id
    }
    // If active, fold the lifetime phase into the animation tile bits.
    if (engine.state.obj_state != 0) {
        update_final_exit_projectile_animation_bits(engine, r);
    }
    // Write the scratch record back to the slot.
    store_object_slot_scratch(engine, r);
}

/// Ticks one active final-exit projectile slot, clearing it when its
/// lifetime expires or the projected position trips the bounds check.
pub fn update_final_exit_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
    // Load the record and decrement its remaining-lifetime counter.
    load_object_slot_scratch(engine, r);
    engine.state.obj_state = engine.state.obj_state - 1;
    // Still alive: step its motion and check the new position.
    if (engine.state.obj_state != 0) {
        project_final_exit_projectile_motion(engine, r);
        check_final_exit_projectile_bounds(engine, r);
        if ((r.carry) != 0) {
            // Out of bounds -> kill the projectile this frame.
            engine.state.obj_state = 0;
        } else {
            // In bounds -> commit the projected position.
            engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
            engine.state.obj_y_pixel = engine.state.scratch2;
        }
    }
    // If still active, refresh the animation tile bits.
    if (engine.state.obj_state != 0) {
        update_final_exit_projectile_animation_bits(engine, r);
    }
    store_object_slot_scratch(engine, r);
}

/// Projects the spawn point from the player position using velocity scaled
/// by four pixels so new shots start ahead of the player.
pub fn project_final_exit_projectile_spawn(engine: &mut Engine, r: &mut RoutineContext) {
    // Start from the player's position (X fine, Y pixel).
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.scratch2 = engine.state.player_y;
    // Offset Y by 4x the vertical velocity so the shot starts ahead vertically.
    if (engine.state.obj_y_vel != 0) {
        let scaled_y_delta = (((engine.state.obj_y_vel as i32) << 2) as u8 as i32); // vel * 4
        engine.state.scratch2 = ((scaled_y_delta + (engine.state.scratch2 as i32)) as u8);
    }
    // Offset X by 4x the horizontal velocity (shot starts ahead horizontally).
    if (engine.state.obj_x_vel_lo != 0) {
        let scaled_x_delta = (((engine.state.obj_x_vel_lo as i32) << 2) as u8 as i32); // vel * 4
        engine.state.indirect_ptr_lo =
            ((scaled_x_delta + (engine.state.indirect_ptr_lo as i32)) as u8);
    }
}

/// Folds the projectile lifetime phase into the slot state bits used by the
/// final-exit projectile sprite animation.
pub fn update_final_exit_projectile_animation_bits(engine: &mut Engine, r: &mut RoutineContext) {
    // Take lifetime bits 2-3 (0x0C) as the animation frame selector...
    engine.state.scratch0 = engine.state.obj_state & ((crate::bits::BITS_2_3) as u8);
    // ...and splice them into the tile id (clearing its bits 2-3 first).
    engine.state.obj_tile =
        (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8)) | engine.state.scratch0;
}

/// Raises carry when the projected projectile has crossed the right edge
/// while still in the scripted vertical range. Other paths intentionally
/// leave carry untouched to preserve the original branch contract.
pub fn check_final_exit_projectile_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    // Y at/below the scripted floor (>= 0xA1): out of bounds ($86B5 BCS $86C1 SEC).
    if (engine.state.scratch2 >= 161) {
        r.carry = 1;
        return;
    }
    // X still left of the right edge (< 0xF1): in range ($86BB BCC $86C3 CLC).
    if (engine.state.indirect_ptr_lo < 241) {
        r.carry = 0;
        return;
    }
    // X exactly 0 (wrapped): in range ($86C3 CLC).
    if (engine.state.indirect_ptr_lo == 0) {
        r.carry = 0;
        return;
    }
    // Past the right edge while in the vertical band -> out of bounds ($86C1 SEC).
    r.carry = 1;
}

/// Projects one active final-exit projectile from its saved slot position
/// and per-frame velocity.
pub fn project_final_exit_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Start from the projectile's saved position.
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    // Add one frame of vertical, then horizontal, velocity.
    if (engine.state.obj_y_vel != 0) {
        engine.state.scratch2 = engine.state.obj_y_vel + engine.state.scratch2;
    }
    if (engine.state.obj_x_vel_lo != 0) {
        engine.state.indirect_ptr_lo = engine.state.obj_x_vel_lo + engine.state.indirect_ptr_lo;
    }
}

/// Draws all three final-exit projectile slots into their fixed OAM ranges.
pub fn draw_final_exit_projectile_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // ptr_hi = OAM byte offset (0x88 = sprite 34), ptr_lo = object record
    // offset (0x10 = slot 1). Each slot uses 8 OAM bytes (two sprites).
    engine.state.indirect_ptr_hi = 136; // 0x88: OAM offset of first projectile sprite
    engine.state.indirect_ptr_lo = 16; // 0x10: first projectile object record
    for _ in 0..3 {
        draw_final_exit_projectile_slot_sprites(engine, r);
        engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi + 8; // next 2-sprite OAM pair
        engine.state.indirect_ptr_lo = engine.state.indirect_ptr_lo + 16; // next object record
    }
}

/// Draws one final-exit projectile as a two-sprite pair or hides it when the
/// slot is inactive/offscreen.
pub fn draw_final_exit_projectile_slot_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let oam_offset = engine.state.indirect_ptr_hi;
    let slot_offset = engine.state.indirect_ptr_lo;
    // Inactive slot or below the visible area (>= 0xBF) -> hide both sprites
    // by parking their Y off-screen (0xEF = 239).
    if (engine.state.object_state(slot_offset as i32) == 0)
        || (engine.state.object_y_pixel(slot_offset as i32) >= 191)
    {
        engine.state.set_oam_y((oam_offset as i32), 239); // 0xEF: off-screen Y
        engine.state.set_oam_y(((4 + oam_offset) as i32), 239);
        return;
    }

    // Copy the object's attribute byte to both halves of the 2-sprite pair.
    let attributes = engine.state.object_attr(slot_offset as i32);
    engine.state.set_oam_attr((oam_offset as i32), attributes);
    engine
        .state
        .set_oam_attr(((4 + oam_offset) as i32), attributes);

    // Tiles: base and base+2. When horizontally flipped (attr bit6) the two
    // sprites swap so the pair reads correctly mirrored.
    let tile_id = engine.state.object_tile(slot_offset as i32);
    if ((attributes & crate::bits::BIT6) != 0) {
        // Flipped: right sprite gets the base tile, left gets base+2.
        engine
            .state
            .set_oam_tile(((4 + oam_offset) as i32), tile_id);
        engine.state.set_oam_tile((oam_offset as i32), tile_id + 2);
    } else {
        // Normal: left = base, right = base+2.
        engine.state.set_oam_tile((oam_offset as i32), tile_id);
        engine
            .state
            .set_oam_tile(((4 + oam_offset) as i32), tile_id + 2);
    }

    // X: left sprite at the projectile X, right sprite 8 pixels over.
    let projectile_x = engine.state.object_x_sub(slot_offset as i32);
    engine.state.set_oam_x((oam_offset as i32), projectile_x);
    engine
        .state
        .set_oam_x(((4 + oam_offset) as i32), projectile_x + 8); // +8 px = next tile

    // Y: object Y biased by +43 (0x2B) to convert playfield Y to screen Y,
    // applied to both sprites of the pair.
    let projectile_y = ((engine.state.object_y_pixel(slot_offset as i32) + 43) as u8 as i32);
    engine.state.set_oam_y((oam_offset as i32), projectile_y);
    engine
        .state
        .set_oam_y(((4 + oam_offset) as i32), projectile_y);
}

/// Rotates one scripted OAM entry into sprite zero and hides the source
/// sprite. The sequence cycles through player/projectile sprites via `0x3E`.
pub fn rotate_sprite_zero_from_scripted_oam(engine: &mut Engine, r: &mut RoutineContext) {
    // Step the cursor back through 8 source sprites, wrapping 0 -> 7
    // (bit7 set means it underflowed below 0).
    let mut sprite_index = ((engine.state.sprite_index - 1) as u8 as i32);
    if ((sprite_index & crate::bits::BIT7) != 0) {
        sprite_index = 7; // wrap to last of 8 entries
    }
    engine.state.sprite_index = (sprite_index as u8);
    let oam_offset = ((sprite_index << 2) as u8 as i32); // index * 4 bytes per sprite
    // Pick the source OAM page: indices with bits 1-2 set (>= 2) come from the
    // projectile page ($0280), others from the player page ($0210).
    let source_base = if ((sprite_index & crate::bits::BITS_1_2) != 0) {
        640 // 0x0280: projectile OAM mirror
    } else {
        528 // 0x0210: player OAM mirror
    };
    // Copy the 4-byte sprite (Y, tile, attr, X) into OAM sprite 0.
    engine.state.set_oam_y(
        0,
        engine
            .state
            .byte((source_base + oam_offset) as u16 as i32),
    );
    engine.state.set_oam_tile(
        0,
        engine
            .state
            .byte((source_base + 1 + oam_offset) as u16 as i32),
    );
    engine.state.set_oam_attr(
        0,
        engine
            .state
            .byte((source_base + 2 + oam_offset) as u16 as i32),
    );
    engine.state.set_oam_x(
        0,
        engine
            .state
            .byte((source_base + 3 + oam_offset) as u16 as i32),
    );
    // Hide the source sprite (park its Y off-screen at 0xEF = 239) so it is not
    // drawn twice now that it lives in sprite 0.
    engine
        .state
        .set_byte(((source_base + oam_offset) as u16 as i32), 239);
}

/// Converts the latched action direction into final-exit projectile velocity
/// by accumulating the movement table for `r.offset` steps.
pub fn build_final_exit_projectile_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    // Direction nibble (r.value low 4 bits) * 2 indexes the per-direction
    // signed delta tables (the tables are stride-2 per direction).
    let direction_table_offset =
        (((r.value & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    let step_count = r.offset;
    // Accumulate the X delta step_count times to scale the velocity.
    let mut x_velocity = 0;
    let mut remaining_steps = step_count;
    loop {
        x_velocity = ((x_velocity
            + engine
                .state
                .byte((MOVE_DELTA_X_TABLE + direction_table_offset) as u16 as i32))
            as u8 as i32);
        remaining_steps -= 1;
        if (remaining_steps == 0) {
            break;
        }
    }
    engine.state.obj_x_vel_lo = (x_velocity as u8);

    // Same accumulation for the Y delta.
    let mut y_velocity = 0;
    remaining_steps = step_count;
    loop {
        y_velocity = ((y_velocity
            + engine
                .state
                .byte((MOVE_DELTA_Y_TABLE + direction_table_offset) as u16 as i32))
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
    // Copy 64 bytes of the table-A sprite template into OAM starting at byte 64
    // (the object sprite region).
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
    // Copy 64 bytes of the table-B sprite template into OAM starting at byte 64.
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
    // Copy 64 bytes of the table-C sprite template into OAM starting at byte 192
    // (the player sprite region).
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
    // Take the pose/animation bits (low 5 = 0x1F) from the player's pose.
    let pose_tile_bits =
        ((engine.state.player_pose & ((crate::bits::LOW_5_BITS) as u8)) as u8 as i32);
    engine.state.scratch0 = (pose_tile_bits as u8);
    // Splice the pose bits into each body slot's tile id (slots at object
    // offsets 16/32/48 = records $0410/$0420/$0430), preserving the slot's high
    // 3 bits (0xE0).
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

    // All three body slots share the player's fine X.
    let player_x = engine.state.player_x_fine;
    engine.state.set_object_x_sub(16, (player_x as i32));
    engine.state.set_object_x_sub(32, (player_x as i32));
    engine.state.set_object_x_sub(48, (player_x as i32));

    // Spread the body slots horizontally around the player's tile column:
    // slot 32 at +1, slot 48 at -2, slot 16 at -3.
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

/// Finishes one scripted-player frame: choose the pose, advance the walk
/// animation, and emit the player sprites.
fn finish_scripted_player_motion_frame(engine: &mut Engine, r: &mut RoutineContext) {
    update_scripted_player_pose_from_motion(engine, r);
    tick_scripted_player_walk_animation(engine, r);
    draw_scripted_player_sprites(engine, r);
}

/// Commits a successful scripted move: store the projected position
/// (`0x0E`/`0x0A`), re-evaluate fall state, and finish the frame.
fn commit_scripted_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.player_x_fine = engine.state.indirect_ptr_lo;
    engine.state.player_y = engine.state.scratch2;
    update_scripted_player_fall_state(engine, r);
    finish_scripted_player_motion_frame(engine, r);
}

/// Cancels scripted motion when a move is blocked: clear the jump/fall timers,
/// re-evaluate fall state, and finish the frame at the unchanged position.
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
    // Start button (bit4) -> pause/prompt and return.
    r.value = (engine.state.buttons as u8);
    if ((r.value & ((crate::bits::BIT4) as u8)) != 0) {
        wait_for_start_button_prompt(engine, r);
        return;
    }

    // Without the action button (bit6) held, clear the held-direction (high)
    // nibble of the direction latch.
    if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) == 0) {
        engine.state.direction_latch =
            engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8);
    }
    // Latch the freshly pressed D-pad direction (low nibble) into the latch.
    let directional_buttons =
        ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) as u8 as i32);
    r.value = (directional_buttons as u8);
    if (directional_buttons != 0) {
        engine.state.scratch0 = (directional_buttons as u8);
        engine.state.direction_latch = (engine.state.direction_latch
            & ((crate::bits::HIGH_NIBBLE) as u8))
            | engine.state.scratch0;
    }

    // Collision via sprite-0 hit (player vs. scripted hazard sprite), only when
    // not already in the post-hit blink window.
    if (engine.state.sprite_blink_timer == 0) {
        if engine.state.sprite0_hit() {
            r.index = ((engine.state.sprite_index + 1) as u8);
            // Only the player-body sprite indices (bits 1-2 == 0) take damage.
            if ((r.index & ((crate::bits::BITS_1_2) as u8)) == 0) {
                // Damage depends on which screen half the hit occurred in:
                // left of x=176 costs 10, right side costs 5.
                let collision_screen_x = (((engine.state.scroll_pixel_x
                    + ((engine.state.object_x_sub(r.index as i32)) as u8))
                    as u8) as i32);
                r.value = ((if (collision_screen_x < 176) { 10 } else { 5 }) as u8); // 176 = screen midpoint
                subtract_scripted_player_health(engine, r);
                engine.state.jump_timer = 10; // 0x0A: knockback jump frames
                engine.state.prompt_state = 33; // hurt sound/effect id
                engine.state.prompt_argument = 2;
                engine.state.sprite_blink_timer = 1; // start invulnerability blink
                build_player_health_meter_sprites(engine, r);
            }
        }
    }

    // While airborne (jump or fall active) force a downward-bias input so the
    // arc continues; otherwise end the blink window.
    if (engine.state.jump_timer == 0) && (engine.state.fall_frames == 0) {
        engine.state.sprite_blink_timer = 0;
    } else {
        engine.state.buttons = (engine.state.buttons & ((crate::bits::HIGH_NIBBLE) as u8))
            | ((crate::bits::BIT1) as u8); // bit1 = down
    }

    build_scripted_player_input_delta(engine, r);
    // Falling path: vertical delta = (fall_frames / 4) + 1.
    if (engine.state.fall_frames != 0) {
        r.value = (((engine.state.fall_frames >> 2) + 1) as u8);
        engine.state.vertical_delta = (r.value as u8);
        try_move_scripted_player_in_bounds(engine, r);
        if ((r.carry) == 0) {
            // Full move ok -> commit.
            commit_scripted_player_position(engine, r);
            return;
        }

        // Full move blocked: retry vertical-only (drop straight).
        engine.state.horizontal_subtile_delta = 0;
        try_move_scripted_player_in_bounds(engine, r);
        if ((r.carry) == 0) {
            // Vertical-only drop succeeded -> commit ($8C68 BCC $8CA1).
            commit_scripted_player_position(engine, r);
            return;
        }

        // Still blocked -> land/cancel.
        cancel_scripted_player_motion(engine, r);
        return;
    }

    // Not falling: advance/start a jump on jump_timer or jump button (bit7),
    // else idle (clear collision flag). r.value carries the new jump_timer (0).
    if (engine.state.jump_timer != 0) || ((engine.state.buttons & ((crate::bits::BIT7) as u8)) != 0)
    {
        tick_scripted_player_jump_action(engine, r);
        r.value = 0;
    } else {
        engine.state.collision_flag = 0;
        r.value = 0;
    }

    engine.state.jump_timer = (r.value as u8);
    // Apply the resulting horizontal move; blocked -> cancel, else commit.
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
    // Jump start: only if not already jumping this press (collision_flag guards
    // held-button retrigger). Play the jump sound and load jump strength.
    if (jump_timer == 0) {
        if (engine.state.collision_flag != 0) {
            return;
        }
        engine.state.prompt_state = 27; // jump sound/effect id
        engine.state.jump_timer = engine.state.jump_strength;
    }
    engine.state.collision_flag = 1; // mark jump in progress (block retrigger)
    engine.state.jump_timer = engine.state.jump_timer - 1;
    // Upward vertical delta = -(jump_timer / 4): negate (timer>>2) via XOR 0xFF
    // + 1 (two's complement), so the rise decelerates as the timer counts down.
    engine.state.vertical_delta =
        (((((jump_timer >> 2) as u8 as i32) ^ crate::bits::BYTE_MASK) + 1) as u8);
    // Try the full move; if blocked, retry vertical-only.
    try_move_scripted_player_in_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.horizontal_subtile_delta = 0;
        try_move_scripted_player_in_bounds(engine, r);
    }
    if ((r.carry) == 0) {
        // Move ok -> commit projected position.
        engine.state.player_x_fine = engine.state.indirect_ptr_lo;
        engine.state.player_y = engine.state.scratch2;
        update_scripted_player_fall_state(engine, r);
    } else {
        // Hit ceiling/wall -> end the jump and stop falling.
        engine.state.jump_timer = 0;
        engine.state.fall_frames = 0;
        update_scripted_player_fall_state(engine, r);
    }
    // Finish the frame: pose, walk animation, sprite emit.
    update_scripted_player_pose_from_motion(engine, r);
    tick_scripted_player_walk_animation(engine, r);
    draw_scripted_player_sprites(engine, r);
}

/// Projects the scripted player's candidate position one frame ahead.
///
/// Seeds the scratch X/Y at `0x0E`/`0x0A` (`indirect_ptr_lo`/`scratch2`) with
/// the current fine X (`0x47`) and Y (`0x4A`), then adds the pending movement
/// deltas: vertical delta `0x4B` and horizontal sub-tile delta `0x49`. Deltas
/// are signed 6502 bytes, so the additions wrap as `u8`. The result is what
/// `check_scripted_player_bounds` and the move routines test before commit.
pub fn project_scripted_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    // Start from the player's current fine X / Y position.
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.scratch2 = engine.state.player_y;
    // Apply the pending vertical movement delta (signed; 0 means no motion).
    if (engine.state.vertical_delta != 0) {
        engine.state.scratch2 = engine.state.vertical_delta + engine.state.scratch2;
    }
    // Apply the pending horizontal sub-tile movement delta.
    if (engine.state.horizontal_subtile_delta != 0) {
        engine.state.indirect_ptr_lo =
            engine.state.horizontal_subtile_delta + engine.state.indirect_ptr_lo;
    }
}

/// Sets the scripted player pose and horizontal facing from the sign of the
/// horizontal movement delta `0x49`.
///
/// `pose_bits` are OR-ed in after masking the current pose with `preserve_mask`
/// (so callers can keep selected pose bits while replacing others). The new
/// facing byte goes to `player_facing` (`0x46`): 0 for left-facing / right-
/// moving (negative delta, bit7 set), 0x40 (BIT6, the OAM horizontal-flip bit)
/// for right-facing / left-moving (positive delta). Returns `false` without
/// touching state when the player is not moving horizontally (delta == 0), so
/// the caller can fall through to other pose logic.
fn apply_scripted_horizontal_pose(
    engine: &mut Engine,
    r: &mut RoutineContext,
    pose_bits: i32,
    preserve_mask: i32,
) -> bool {
    r.index = (pose_bits as u8);
    r.offset = 0;
    // Decide facing from the sign of the horizontal delta.
    if ((engine.state.horizontal_subtile_delta & ((crate::bits::BIT7) as u8)) != 0) {
        // Negative horizontal deltas face left with no sprite flip.
    } else if (engine.state.horizontal_subtile_delta == 0) {
        // No horizontal motion: leave pose/facing for the caller to handle.
        return false;
    } else {
        r.offset = 64; // BIT6 = OAM horizontal-flip bit for the mirrored pose.
    }

    // Merge the new pose bits into the preserved pose bits and store facing.
    engine.state.scratch0 = (r.index as u8);
    engine.state.player_pose =
        (engine.state.player_pose & (preserve_mask as u8)) | engine.state.scratch0;
    engine.state.player_facing = (r.offset as u8);
    true
}

/// Chooses the scripted player pose tile (`0x45`) and horizontal flip from
/// movement, jump/fall state, and the controller.
///
/// Reads buttons `0x16`, vertical delta `0x4B`, jump timer `0x4F`, and fall
/// frame count `0x4E`. The pose selection priority is: action-only input ->
/// jump pose; rising motion with the jump expired -> jump pose; falling with
/// the up/attack bit -> a dedicated fall pose 13; otherwise a horizontal
/// walk/airborne pose chosen by `apply_scripted_horizontal_pose`.
pub fn update_scripted_player_pose_from_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let jump_pose = 9; // pose tile index used while airborne/jumping
    // If only the action button is held (button byte == 0x80 after clearing
    // BIT6), force the jump pose immediately.
    if (((engine.state.buttons & ((crate::bits::CLEAR_BIT6) as u8)) as u8 as i32) == 128) {
        r.index = jump_pose;
        engine.state.player_pose = (r.index as u8);
        return;
    }

    // Branch on whether there is any vertical motion this frame.
    if (engine.state.vertical_delta != 0) {
        if ((engine.state.vertical_delta & ((crate::bits::BIT7) as u8)) != 0) {
            // Negative delta = moving up; show the jump pose once the jump
            // timer has expired.
            if (engine.state.jump_timer == 0) {
                r.index = jump_pose;
                engine.state.player_pose = (r.index as u8);
                return;
            }
        } else if (engine.state.fall_frames == 0) {
            // Positive delta with no accumulated fall frames = first descent.
            if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) != 0) {
                r.index = 13; // special falling pose when the up/jump bit is held
                engine.state.player_pose = (r.index as u8);
                return;
            }
            // Otherwise pick a walk pose, preserving the low 3 pose bits.
            apply_scripted_horizontal_pose(engine, r, 1, 7);
            return;
        }

        // Sustained jump or fall: airborne pose 57, keep low 2 pose bits.
        apply_scripted_horizontal_pose(engine, r, 57, 3);
        return;
    }

    // No vertical motion: pick a ground walk pose, preserving low 3 pose bits.
    apply_scripted_horizontal_pose(engine, r, 1, 7);
}

/// Applies the action-button pose bit and advances the scripted walk cycle.
///
/// Reads pose `0x45`, buttons `0x16`, jump timer `0x4F`, fall frames `0x4E`,
/// and the animation step counter `0x48`. For ground poses (pose < 32) the
/// attack/sword bit (BIT4) tracks button BIT6. While moving on the ground (any
/// of the four direction bits held, not jumping/falling) the step counter ticks
/// and every 8th frame the walk frame toggles: poses with BIT3 set flip facing
/// (BIT6 of `player_facing`), otherwise the alternate-step bit (BIT2) of the
/// pose is toggled.
pub fn tick_scripted_player_walk_animation(engine: &mut Engine, r: &mut RoutineContext) {
    // For ground poses, mirror the action button into the pose's attack bit.
    if (engine.state.player_pose < 32) {
        let mut pose = engine.state.player_pose;
        if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
            pose = ((pose | ((crate::bits::BIT4) as u8)) as u8);
        } else {
            pose = ((pose & ((crate::bits::CLEAR_BIT4) as u8)) as u8);
        }
        engine.state.player_pose = pose;
    }
    // Only animate while a direction button (low nibble of input) is held.
    if ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    // Never animate the walk cycle while jumping or falling.
    if ((engine.state.jump_timer | engine.state.fall_frames) != 0) {
        return;
    }
    // Tick the step counter; only advance the frame once every 8 frames.
    engine.state.anim_step_counter = engine.state.anim_step_counter + 1;
    if ((engine.state.anim_step_counter & ((crate::bits::LOW_3_BITS) as u8)) != 0) {
        return;
    }
    // Toggle the walk frame: BIT3 poses flip facing; others toggle BIT2.
    if ((engine.state.player_pose & ((crate::bits::BIT3) as u8)) != 0) {
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8);
    } else {
        engine.state.player_pose = engine.state.player_pose ^ ((crate::bits::BIT2) as u8);
    }
}

/// Draws the two 8x16 scripted-player sprites into the fixed OAM entries at
/// byte offsets 16 (`0x0210`) and 20 (`0x0214`).
///
/// During the invulnerability blink (`sprite_blink_timer` `0x85` non-zero) the
/// sprites are hidden on even frames (`frame_prescaler` BIT0 clear) by parking
/// their Y at 239 (offscreen). Otherwise Y is `player_y + 43` (the 43-pixel
/// status-bar offset at the top of the screen), X comes from `player_x_fine`
/// for the left half and +8 for the right half. Attributes OR in BIT5 (the
/// behind-background priority bit) plus the facing flip in `player_facing`; the
/// left/right tile order swaps when BIT6 (horizontal flip) is set.
pub fn draw_scripted_player_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // While blinking, hide both halves on every even frame and bail.
    if (engine.state.sprite_blink_timer != 0) {
        if ((engine.state.frame_prescaler & ((crate::bits::BIT0) as u8)) == 0) {
            engine.state.set_oam_y(16, 239); // 239 = offscreen Y
            engine.state.set_oam_y(20, 239);
            return;
        }
    }
    // Position both 8x16 halves at the player's screen Y (+43 status offset).
    engine
        .state
        .set_oam_y(16, ((engine.state.player_y + 43) as i32));
    engine
        .state
        .set_oam_y(20, ((engine.state.player_y + 43) as i32));
    // Left half at fine X, right half 8 pixels to the right.
    engine
        .state
        .set_oam_x(16, (engine.state.player_x_fine as i32));
    engine
        .state
        .set_oam_x(20, ((engine.state.player_x_fine + 8) as i32));
    // Attributes carry the facing flip plus BIT5 (background priority).
    engine.state.set_oam_attr(
        16,
        ((engine.state.player_facing | ((crate::bits::BIT5) as u8)) as i32),
    );
    engine.state.set_oam_attr(
        20,
        ((engine.state.player_facing | ((crate::bits::BIT5) as u8)) as i32),
    );
    // Assign tiles; swap left/right order when horizontally flipped (BIT6).
    if ((engine.state.player_facing & ((crate::bits::BIT6) as u8)) != 0) {
        r.index = (engine.state.player_pose as u8);
        engine.state.set_oam_tile(20, (r.index as i32));
        r.index = ((r.index + 2) as u8); // adjacent tile pair (+2 for 8x16)
        engine.state.set_oam_tile(16, (r.index as i32));
    } else {
        r.index = (engine.state.player_pose as u8);
        engine.state.set_oam_tile(16, (r.index as i32));
        r.index = ((r.index + 2) as u8);
        engine.state.set_oam_tile(20, (r.index as i32));
    }
}

/// Tries to move the scripted player, shrinking the vertical delta toward zero
/// until the projected position lands in bounds.
///
/// Repeatedly projects the candidate position and runs
/// `check_scripted_player_bounds`. On success (carry clear) the loop exits with
/// the working `vertical_delta` (`0x4B`) reflecting how far the move succeeded.
/// On a bounds rejection the magnitude of the vertical delta is reduced by one
/// (positive deltas step down by 2 then +1, negative deltas step toward 0 by
/// +1) and retried; if the delta reaches zero the move fails with carry set.
/// The original vertical delta is restored before returning so the caller's
/// nominal velocity is preserved.
pub fn try_move_scripted_player_in_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_y_delta = engine.state.vertical_delta;
    loop {
        // Project the candidate position and test it against the screen bounds.
        project_scripted_player_position(engine, r);
        check_scripted_player_bounds(engine, r);
        if ((r.carry) == 0) {
            break; // in bounds: accept this (possibly reduced) delta
        }
        {
            // Out of bounds: shrink the vertical delta magnitude by one.
            let mut adjusted_y_delta = engine.state.vertical_delta;
            if (adjusted_y_delta == 0) {
                r.carry = 1; // already exhausted: move fails
                break;
            }
            // For positive (downward) deltas, subtract 2 before the +1 below so
            // the net change is -1 toward zero; negative deltas just get +1.
            if ((adjusted_y_delta & ((crate::bits::BIT7) as u8)) == 0) {
                adjusted_y_delta = ((adjusted_y_delta - 1) as u8);
                adjusted_y_delta = ((adjusted_y_delta - 1) as u8);
            }
            adjusted_y_delta = ((adjusted_y_delta + 1) as u8);
            engine.state.vertical_delta = adjusted_y_delta;
            if (adjusted_y_delta != 0) {
                continue; // retry with the reduced delta
            }
            r.carry = 1; // reduced to zero without fitting: fail
            break;
        }
    }
    // Restore the caller's original vertical velocity.
    engine.state.vertical_delta = saved_y_delta;
}

/// Updates the scripted player's falling/landing timers.
///
/// Reads jump timer `0x4F`, player Y `0x4A`, fall frame count `0x4E`, and the
/// jump strength `0x50`. While airborne (jump active) it does nothing. Above the
/// landing line (Y < 160) it accumulates `fall_frames`. On landing (Y >= 160) a
/// fall longer than `jump_strength` produces a bounce: the excess fall distance
/// (clamped to `jump_strength`, minus a constant) seeds a new `jump_timer` and
/// triggers prompt `0x0A` (the bounce prompt). `fall_frames` is then reset.
pub fn update_scripted_player_fall_state(engine: &mut Engine, r: &mut RoutineContext) {
    // Still in the jump arc: nothing to do, report no landing (carry clear).
    if (engine.state.jump_timer != 0) {
        r.carry = 0;
        return;
    }
    // Above the landing line (Y < 160): keep counting fall frames.
    if (engine.state.player_y < 160) {
        engine.state.fall_frames = engine.state.fall_frames + 1;
        return;
    }
    {
        // Landed: a fall longer than jump strength triggers a bounce.
        let mut fall_frames = engine.state.fall_frames;
        if (fall_frames >= engine.state.jump_strength) {
            fall_frames = ((fall_frames - 7) as u8); // damp the bounce by 7 frames
            if (fall_frames >= engine.state.jump_strength) {
                fall_frames = engine.state.jump_strength; // clamp to max strength
            }
            fall_frames = ((fall_frames - 1) as u8);
            engine.state.jump_timer = fall_frames; // seed the bounce jump
            engine.state.prompt_state = 10; // 0x0A = bounce prompt
        }
    }
    // Reset the fall accumulator now that we are grounded.
    engine.state.fall_frames = 0;
}

/// Subtracts contact damage `A` (`r.value`) from scripted player health `0x53`,
/// saturating at zero on underflow.
///
/// Models the 6502 `SBC` behavior: carry is the borrow-out (clear means the
/// subtraction underflowed, i.e. damage exceeded health), and zero/negative
/// reflect the wrapped 8-bit result. When a borrow occurs the health is clamped
/// to 0 rather than left wrapped.
pub fn subtract_scripted_player_health(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch0 = (r.value as u8); // damage amount to apply
    let current_health = engine.state.player_health;
    {
        // Perform the subtraction in wider precision to detect borrow.
        let difference = (current_health as u16 as i32) - (engine.state.scratch0 as u16 as i32);
        let result = (difference as u8 as i32);
        // Carry = no borrow: clear when bit8 (the borrow flag) is set.
        r.carry = if ((difference & crate::bits::BIT8) != 0) {
            0
        } else {
            1
        };
        r.zero = if (result == 0) { 1 } else { 0 };
        r.negative = (((result >> 7) & 1) as u8); // bit7 of the wrapped result
        engine.state.player_health = result as u8;
    }
    // On underflow (borrow), clamp health to zero instead of wrapping.
    if ((r.carry) == 0) {
        engine.state.player_health = 0;
    }
}

/// Tests the projected scripted-player position against the screen bounds.
///
/// Reads the candidate Y in `scratch2` (`0x0A`) and candidate X in
/// `indirect_ptr_lo` (`0x0E`). Sets carry (rejected) when Y >= 161 (below the
/// playable area) or X >= 241 (past the right edge); clears carry when the
/// position is acceptable.
pub fn check_scripted_player_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    // Reject if the candidate Y is at/below the bottom playfield limit.
    if (engine.state.scratch2 >= 161) {
        r.carry = 1;
        return;
    }
    // Reject if the candidate X is at/past the right screen edge.
    if (engine.state.indirect_ptr_lo >= 241) {
        r.carry = 1;
        return;
    }
    r.carry = 0; // in bounds
}

/// Converts the controller direction nibble into the scripted player's X/Y
/// movement deltas via the ROM movement tables.
///
/// The low nibble of `buttons` (`0x16`, the four direction bits) is doubled to
/// form a table index (each direction combo has a 2-byte X/Y entry; the X and Y
/// tables are interleaved one byte apart). The looked-up bytes are stored as the
/// horizontal sub-tile delta `0x49` and vertical delta `0x4B`.
pub fn build_scripted_player_input_delta(engine: &mut Engine, r: &mut RoutineContext) {
    // Direction bits -> table index (x2 because entries are paired/interleaved).
    r.index = (((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8);
    engine.state.horizontal_subtile_delta = ((engine
        .state
        .byte((MOVE_DELTA_X_TABLE + (r.index as i32)) as u16 as i32))
        as u8);
    engine.state.vertical_delta = ((engine
        .state
        .byte((MOVE_DELTA_Y_TABLE + (r.index as i32)) as u16 as i32))
        as u8);
}

/// Synthesizes a pseudo-random controller byte for the title-screen demo loop.
///
/// Draws a value in 0..4 from `rng_update` to index `DEMO_INPUT_TABLE`, picking
/// one of four canned direction inputs into `buttons` (`0x16`). A second draw in
/// 0..10 occasionally (value == 0, ~1 in 10) sets the action button bit BIT6.
pub fn choose_random_demo_input(engine: &mut Engine, r: &mut RoutineContext) {
    // First RNG draw in [0,4): pick a canned direction from the demo table.
    r.value = 4;
    rng_update(engine, r);
    r.index = r.value;
    engine.state.buttons = ((engine
        .state
        .byte((DEMO_INPUT_TABLE + (r.index as i32)) as u16 as i32))
        as u8);
    // Second draw in [0,10): occasionally also press the action button (BIT6).
    r.value = 10;
    rng_update(engine, r);
    r.index = r.value;
    if (r.index == 0) {
        engine.state.buttons = engine.state.buttons | ((crate::bits::BIT6) as u8);
    }
}

/// Loads the full 128-byte title-screen OAM template from ROM
/// (`SPRITE_Y_TABLE_D`) into the sprite staging area starting at OAM byte 64.
///
/// Copies in reverse (127..=0) like the original loop, leaving `r.index` = 255.
pub fn load_title_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy 128 template bytes into OAM starting at byte 64 (sprite #16).
    for offset in (0..=127).rev() {
        engine
            .state
            .set_oam_y(64 + offset, engine.state.byte(SPRITE_Y_TABLE_D + offset));
    }
    r.index = 255;
}

/// Loads the smaller 32-byte demo-mode OAM template from ROM
/// (`SPRITE_Y_TABLE_E`) into staging at OAM byte 64, used after the title
/// timeout drops into the attract/demo sequence.
pub fn load_demo_oam_template(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy 32 template bytes into OAM starting at byte 64 (sprite #16).
    for offset in (0..=31).rev() {
        engine
            .state
            .set_oam_y(64 + offset, engine.state.byte(SPRITE_Y_TABLE_E + offset));
    }
    r.index = 255;
}

/// Blinks the first eight demo sprites (OAM bytes 64..92) on and off in step
/// with the frame timer.
///
/// When either of `frame_prescaler` bits 4-5 is set the sprites are shown at Y
/// 128; otherwise they are parked offscreen at Y 239, producing a slow blink.
pub fn blink_demo_oam_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sprite_y: i32 = 239; // default: offscreen (hidden)
    // Visible phase: bits 4-5 of the frame counter select the "shown" window.
    if ((engine.state.frame_prescaler & ((crate::bits::BITS_4_5) as u8)) != 0) {
        sprite_y = 128; // on-screen Y while blinking visible
    }
    // Apply Y to the 8 sprites' Y bytes (every 4th byte, offsets 0..=28).
    for oam_offset in (0..=28).step_by(4) {
        engine.state.set_oam_y(64 + oam_offset, sprite_y);
    }
    r.index = (sprite_y as u8);
}

/// Stages one intro text line into the VRAM staging buffer (`0x0140`).
///
/// Reads characters from the data pointer (`0x08/0x09`) until a NUL terminator
/// (returns carry set = end of text) or a CR (0x0D) line break. Each printable
/// char is converted to a tile id by spreading the high nibble up by one bit and
/// re-OR-ing the low nibble (the ROM's packed-nibble -> tile mapping), then
/// stored to the staging buffer. On CR the staged row is given a VRAM address
/// and uploaded (PPU job 5), returning carry clear (more text follows).
pub fn stage_intro_text_line(engine: &mut Engine, r: &mut RoutineContext) {
    clear_text_staging_buffer(engine, r);

    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let mut text_offset: i32 = 0;
    let mut guard: i32 = 0;
    // Scan at most one nametable row (256 chars) of source text.
    while (guard < 256) {
        let source_byte: i32 = engine
            .state
            .byte((source_ptr + text_offset) as u16 as i32);
        // NUL = end of all intro text.
        if (source_byte == 0) {
            r.carry = 1;
            return;
        }
        // CR = end of this line: address and upload the staged row.
        if (source_byte == 13) {
            set_intro_text_vram_address(engine, r);
            r.value = 5; // PPU job id 5 = text-row upload
            upload_intro_text_scroll_slice(engine, r);
            r.carry = 0;
            return;
        }

        // Map packed char -> tile id: high nibble shifted up 1 bit, low nibble
        // re-merged in.
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

/// Stages the next scrolling intro text line and advances the source pointer
/// past the line's CR.
///
/// Like `stage_intro_text_line`, but on CR it also advances the data pointer
/// (`0x08/0x09`) past the consumed characters (incrementing the high byte on
/// page wrap) so subsequent calls continue with the next line, and it adds 16 to
/// each tile id to select the scrolling text tile set. NUL returns carry set
/// (end of text); a completed line returns carry clear.
pub fn stage_scrolling_intro_text_line(engine: &mut Engine, r: &mut RoutineContext) {
    clear_text_staging_buffer(engine, r);

    let source_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    let mut text_offset: i32 = 0;
    let mut scan_guard: i32 = 0;
    // Scan at most one nametable row (256 chars) of source text.
    while (scan_guard < 256) {
        let source_byte: i32 = engine
            .state
            .byte((source_ptr + text_offset) as u16 as i32);
        // NUL = end of all intro text.
        if (source_byte == 0) {
            r.carry = 1;
            return;
        }
        // CR = end of this line.
        if (source_byte == 13) {
            text_offset += 1; // step past the CR itself

            // Advance the source data pointer past the consumed bytes, carrying
            // into the high byte on an 8-bit wrap.
            let advanced_source: i32 =
                ((text_offset + (engine.state.data_ptr_lo as i32)) as u16 as i32);
            engine.state.data_ptr_lo = (advanced_source as u8);
            if (advanced_source > 255) {
                engine.state.data_ptr_hi = engine.state.data_ptr_hi + 1;
            }

            r.value = 8; // scroll variant marker passed to the address setup
            set_intro_text_vram_address(engine, r);
            r.value = 5; // PPU job id 5 = text-row upload
            upload_intro_text_scroll_slice(engine, r);
            r.carry = 0;
            return;
        }

        // Map packed char -> tile id (high nibble << 1 | low nibble) and add 16
        // to select the scrolling text tile range.
        let low_nibble: i32 = source_byte & crate::bits::LOW_NIBBLE;
        engine.state.scratch0 = (low_nibble as u8);

        let high_bits: i32 = (((source_byte & crate::bits::HIGH_NIBBLE) << 1) as u8 as i32);
        let tile_id: i32 = (((high_bits | (engine.state.scratch0 as i32)) + 16) as u8 as i32);
        engine.state.set_vram_stage(text_offset, tile_id);

        text_offset += 1;
        scan_guard += 1;
    }
}

/// Converts the intro text scroll row in `scratch2` (`0x0A`) into a nametable
/// VRAM address and stores it in the upload address shadows `0x06/0x07`.
///
/// The row index is multiplied by 4 (each text row advances 4 in the address,
/// matching the 32-tile rows used here) and added to nametable 0 base `0x2000`.
pub fn set_intro_text_vram_address(engine: &mut Engine, r: &mut RoutineContext) {
    // row * 4 + $2000 nametable base.
    let address: i32 = VRAM_NAMETABLE0 + (((engine.state.scratch2 as i32) << 2) as i32);
    engine.state.vram_addr_hi = ((address >> 8) as u8);
    engine.state.vram_addr_lo = (address as u8);
    r.value = (address as u8);
}

/// Advances the intro text vertical scroll offset `0x0A` by one pixel,
/// uploading a slice for each sub-row step.
///
/// Increments the scroll offset (`scratch2`) until it lands on an 8-pixel row
/// boundary (low 3 bits clear); for each intermediate pixel it flushes a partial
/// scroll slice. Wraps the offset back to 0 when it reaches 240 (the bottom of
/// the visible nametable region).
pub fn advance_intro_text_scroll(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        // Advance one pixel (wrapping as a byte).
        engine.state.scratch2 = (engine.state.scratch2 + 1) & ((crate::bits::BYTE_MASK) as u8);
        // Stop once aligned to an 8-pixel row boundary.
        if ((engine.state.scratch2 & ((crate::bits::LOW_3_BITS) as u8)) == 0) {
            break;
        }
        // Mid-row pixel: flush a partial slice (job id 255 = continuation).
        r.value = 255;
        upload_intro_text_scroll_slice(engine, r);
    }
    // Wrap at the bottom of the visible region (240 px).
    if (engine.state.scratch2 == 240) {
        engine.state.scratch2 = 0;
    }
}

/// Uploads the staged intro text row plus three continuation slices for the
/// current scroll offset.
///
/// `r.value` on entry is the first PPU job id (5 = full text row, 255 =
/// continuation). The Y scroll register shadow `0x10` is set to `scratch2 + 6`
/// (the 6-row status/header offset), skipping the 16-line gap between visible
/// nametable rows when it crosses 240. Four PPU jobs are then queued and waited
/// on: the first with the supplied id, three more as continuations (255).
pub fn upload_intro_text_scroll_slice(engine: &mut Engine, r: &mut RoutineContext) {
    let first_job_id: i32 = (r.value as u8 as i32);
    // Scroll target = offset + 6 header rows.
    let mut scroll_upload_row: i32 = ((engine.state.scratch2 + 6) as u8 as i32);
    // Past the visible region, skip the 16-line inter-nametable gap.
    if (scroll_upload_row >= 240) {
        scroll_upload_row = ((scroll_upload_row + 16) as u8 as i32);
    }
    engine.state.scroll_y = (scroll_upload_row as u8);
    // Queue the lead job, then three continuation jobs (id 255).
    r.value = (first_job_id as u8);
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
}

/// Loads the intro/text palette into the upload buffer and queues it.
///
/// Sets the first 4-color subpalette to NES colors 0x0F/0x0C/0x10/0x30 (black,
/// blue, gray, white) for legible text, then blanks the remaining 28 entries to
/// black (0x0F) before uploading the whole palette buffer.
pub fn load_intro_text_palette(engine: &mut Engine, r: &mut RoutineContext) {
    // Background subpalette 0: black, blue, light gray, white.
    engine.state.set_palette_buffer(0, 15); // $0F black
    engine.state.set_palette_buffer(1, 12); // $0C blue
    engine.state.set_palette_buffer(2, 16); // $10 gray
    engine.state.set_palette_buffer(3, 48); // $30 white
    // Blank the remaining 28 palette entries to black.
    for palette_offset in (0..=27).rev() {
        engine.state.set_palette_buffer(4 + palette_offset, 15); // $0F black
    }
    r.value = 15;
    upload_palette_buffer(engine, r);
}

/// Hides every staged sprite by writing the offscreen Y (239) to each of the 64
/// OAM entries' Y bytes, leaving tile/attribute/X bytes untouched.
pub fn hide_all_sprite_y_positions(engine: &mut Engine, r: &mut RoutineContext) {
    // Step over each OAM entry's Y byte (offsets 0,4,...,252 = 64 sprites).
    for oam_offset in (0..=252).step_by(4) {
        engine.state.set_oam_y(oam_offset, 239); // 239 = offscreen Y
    }
    r.index = 0;
    r.value = 239;
}

/// Clears the 32-byte text staging buffer to blank tile `0xC0` (192).
pub fn clear_text_staging_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    // Fill all 32 staging slots with the blank tile.
    for offset in (0..=31).rev() {
        engine.state.set_vram_stage(offset, 192); // $C0 blank tile
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
    // Split the 8 saved progress bytes into 16 low-nibble item-list entries
    // (high nibble then low nibble, filling entries 15 down to 0).
    let mut progress_entry_offset: i32 = 15; // top entry of the 16-entry half
    for progress_byte_offset in (0..=7).rev() {
        let progress_byte: i32 = engine.state.save_progress(progress_byte_offset);
        engine
            .state
            .set_password_nibbles_a(progress_entry_offset, progress_byte >> 4); // high nibble
        progress_entry_offset -= 1;
        engine.state.set_password_nibbles_a(
            progress_entry_offset,
            progress_byte & crate::bits::LOW_NIBBLE, // low nibble
        );
        progress_entry_offset -= 1;
    }
    // Copy the 16 saved inventory counts into the second half of the item list.
    for inventory_offset in (0..=15).rev() {
        engine.state.set_password_nibbles_b(
            inventory_offset,
            ((engine.state.save_inventory(inventory_offset) & crate::bits::LOW_NIBBLE) as u8
                as i32),
        );
    }
    // Fold the saved key counter (inventory[16]) one bit at a time into the
    // spare high bit of every other half-A entry (offsets 15,13,...,1).
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
    // Likewise fold the saved coin counter (inventory[17]) into half-B entries.
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
    // Additive checksum: sum all 32 half-A entries (mod 256).
    {
        let mut additive_checksum: i32 = 0;
        for entry_offset in (0..=31).rev() {
            additive_checksum =
                ((additive_checksum + engine.state.password_nibbles_a(entry_offset)) as u8 as i32);
        }
        engine.state.password_checksum_add = (additive_checksum as u8);
    }
    // XOR checksum: seeded with 10 then XOR-folded over all 32 half-A entries.
    {
        let mut xor_checksum: i32 = 10; // non-zero seed so all-zero data still checks
        for entry_offset in (0..=31).rev() {
            xor_checksum =
                ((xor_checksum ^ engine.state.password_nibbles_a(entry_offset)) as u8 as i32);
        }
        engine.state.password_checksum_xor = (xor_checksum as u8);
    }
    // Fold the additive checksum bits into the remaining spare high bits
    // (even-numbered entries 14,12,...,0).
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
    // Fold the XOR checksum bits into the even half-B entries (14,12,...,0).
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
    engine.state.rng_low = ((engine.state.password_nibbles_a(15)) as u8); // entry 0x0F
    engine.state.rng_high = ((engine.state.password_nibbles_b(15)) as u8); // entry 0x1F
    // Scramble entries 14..0 of both halves by XOR-ing each with an RNG byte
    // drawn modulo 32; the RNG also permutes which entry is touched (the new
    // offset comes back in scratch0).
    let mut scramble_offset: i32 = 14;
    while (scramble_offset >= 0) {
        engine.state.scratch0 = (scramble_offset as u8);
        r.value = 32; // RNG range / entry count (0x20)
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_nibbles_a(
            scramble_offset,
            ((r.value ^ ((engine.state.password_nibbles_a(scramble_offset)) as u8)) as u8 as i32),
        );

        r.value = 32; // RNG range / entry count (0x20)
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
    // Work in a copy so a bad checksum leaves the visible list untouched
    // (copy all 32 half-A entries into the scratch buffer).
    for entry_offset in (0..=31).rev() {
        engine
            .state
            .set_password_scramble_a(entry_offset, engine.state.password_nibbles_a(entry_offset));
    }

    // Unscramble every non-seed entry with the same RNG stream used by the
    // encoder. Entries 0x0F/0x1F seed the RNG, as in the encoder.
    engine.state.rng_low = ((engine.state.password_scramble_a(15)) as u8); // entry 0x0F
    engine.state.rng_high = ((engine.state.password_scramble_b(15)) as u8); // entry 0x1F
    let mut scramble_offset: i32 = 14;
    while (scramble_offset >= 0) {
        engine.state.scratch0 = (scramble_offset as u8);
        r.value = 32; // RNG range / entry count (0x20)
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_scramble_a(
            scramble_offset,
            engine.state.password_scramble_a(scramble_offset) ^ (r.value as i32),
        );

        r.value = 32; // RNG range / entry count (0x20)
        rng_update(engine, r);
        scramble_offset = (engine.state.scratch0 as i32);
        engine.state.set_password_scramble_b(
            scramble_offset,
            engine.state.password_scramble_b(scramble_offset) ^ (r.value as i32),
        );

        scramble_offset -= 1;
    }

    // Pull the stored checksum bits back out of the spare high bits (reverse of
    // the encoder's fold), shifting each entry back down by one.
    {
        let mut stored_xor_checksum: i32 = 0;
        for entry_offset in (0..=14).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_b(entry_offset);
            // Shift the recovered bit into bit7 and right-rotate the accumulator.
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

    // Verify the additive checksum (sum of 32 half-A entries) before applying.
    let mut additive_checksum: i32 = 0;
    for entry_offset in (0..=31).rev() {
        additive_checksum =
            ((additive_checksum + engine.state.password_scramble_a(entry_offset)) as u8 as i32);
    }
    if (additive_checksum != (engine.state.password_checksum_add as i32)) {
        // Bad password: queue the error prompt (0x1C) and fail with carry set.
        engine.state.prompt_state = 28; // 0x1C error prompt
        engine.state.prompt_argument = 28;
        r.carry = 1;
        return;
    }

    // Verify the XOR checksum (same seed 10 as the encoder).
    let mut xor_checksum: i32 = 10; // matching non-zero seed
    for entry_offset in (0..=31).rev() {
        xor_checksum =
            ((xor_checksum ^ engine.state.password_scramble_a(entry_offset)) as u8 as i32);
    }
    if (xor_checksum != (engine.state.password_checksum_xor as i32)) {
        engine.state.prompt_state = 28; // 0x1C error prompt
        engine.state.prompt_argument = 28;
        r.carry = 1;
        return;
    }

    // Decode the key counter from the spare high bit of the odd half-A entries.
    {
        let mut key_bits: i32 = 0;
        for entry_offset in (0..=15).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_a(entry_offset);
            key_bits = (((key_bits >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_a(entry_offset, entry >> 1);
        }
        engine.state.set_save_inventory(16, key_bits); // inventory[16] = keys
    }
    // Decode the coin counter from the spare high bit of the odd half-B entries.
    {
        let mut coin_bits: i32 = 0;
        for entry_offset in (0..=15).rev().step_by(2) {
            let entry: i32 = engine.state.password_scramble_b(entry_offset);
            coin_bits = (((coin_bits >> 1) | ((entry & 1) << 7)) as u8 as i32);
            engine
                .state
                .set_password_scramble_b(entry_offset, entry >> 1);
        }
        engine.state.set_save_inventory(17, coin_bits); // inventory[17] = coins
    }

    // Recombine the 16 progress nibbles back into 8 progress bytes (high then
    // low nibble) and copy the 16 inventory counts to the save snapshot.
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
    r.carry = 0; // success
}

/// Restores the title/menu working state from ROM defaults and blacks out
/// the palette buffer. Unlike the full boot RAM initializer, this only
/// rewrites `0x40..0x8B`, leaving broader runtime buffers intact.
pub fn reset_menu_state_and_palette(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy ROM zero-page defaults into RAM 0x40..0x8B (the menu working set).
    for addr in 64..140 {
        engine
            .state
            .set_byte(addr, engine.state.byte(ZP_INIT_TABLE + addr));
    }
    // Black out all 32 palette buffer entries.
    for palette_offset in (0..=31).rev() {
        engine.state.set_palette_buffer(palette_offset, 15); // $0F black
    }
    r.value = 15;
    r.index = 255;
}

/// Uploads the title-screen nametable image and title CHR bank shadows.
///
/// The source image occupies four consecutive 256-byte pages at
/// `0x9EC9..0xA1C8`; `0xA2E9/0xA2EA` provide the title CHR banks.
pub fn upload_title_screen_nametables(engine: &mut Engine, r: &mut RoutineContext) {
    // Snapshot and disable rendering: clear NMI/sprite-pattern bits in PPUCTRL
    // and the bg/sprite show bits in PPUMASK before touching VRAM.
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
    // Point PPUADDR at $2000 (nametable 0): high byte 0x20 then low byte 0x00.
    engine.device_write(crate::engine::reg::PPU_ADDR, 32);
    engine.device_write(crate::engine::reg::PPU_ADDR, 0);

    // Stream the 4 x 256-byte source pages ($9EC9..) straight into PPUDATA.
    for page in 0..4 {
        let source_page = PALETTE_SOURCE_BASE + page * 256; // 256-byte page stride
        for offset in 0..256 {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.byte((source_page + offset) as u16 as i32),
            );
        }
    }

    // Select the title CHR banks for pattern tables 0 and 1.
    engine
        .state
        .set_chr_bank(0, engine.state.byte(TITLE_CHR_BANK_TABLE));
    engine
        .state
        .set_chr_bank(1, engine.state.byte(TITLE_CHR_BANK_TABLE + 1));
    // Restore the original PPUCTRL/PPUMASK shadows and re-enable via PPUCTRL.
    engine.state.ppu_mask_shadow = (mask as u8);
    engine.state.ppu_ctrl_shadow = (ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl);
    r.value = (ctrl as u8);
    r.index = 0;
}

/// Copies the title-screen palette from ROM into the palette upload buffer.
pub fn load_title_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy all 32 title palette bytes from ROM into the upload buffer.
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
    // Pack tile X (high nibble) and fine X (low nibble) into 8-bit world X
    // positions for both the camera (scroll) and the player.
    let scroll_world_x: i32 = ((((engine.state.scroll_tile_x as i32) << 4)
        | (engine.state.scroll_fine_x as i32)) as u8 as i32);
    let player_world_x: i32 = ((((engine.state.player_x_tile as i32) << 4)
        | (engine.state.player_x_fine as i32)) as u8 as i32);
    // Player position relative to the camera origin (wraps as a byte).
    let camera_delta: i32 = ((player_world_x - scroll_world_x) as u8 as i32);
    let mut no_scroll_column_needed: i32 = 0;
    engine.state.scratch0 = (scroll_world_x as u8);
    if (camera_delta < 96) {
        // Player is in the left scroll margin (< 96 px from origin).
        if ((engine.state.scroll_tile_x | engine.state.scroll_fine_x) == 0) {
            no_scroll_column_needed = 1; // already at the left edge: nothing to do
        } else {
            // Scroll left so the player sits 6 tiles in from the left.
            let left_scroll_tile: i32 = ((engine.state.player_x_tile - 6) as u8 as i32);
            if (engine.state.player_x_tile < 6) {
                // Can't keep 6 tiles of margin: clamp the camera to the start.
                engine.state.scroll_fine_x = 0;
                engine.state.scroll_tile_x = 0;
                no_scroll_column_needed = 0;
            } else {
                engine.state.scroll_tile_x = (left_scroll_tile as u8);
                engine.state.scroll_fine_x = engine.state.player_x_fine;
                engine.state.camera_scroll_flag = 255; // 0xFF = scrolling left
                no_scroll_column_needed = 0;
            }
        }
    } else if (camera_delta < 145) {
        // Player in the central dead zone (96..145 px): camera holds still.
        no_scroll_column_needed = 1;
    } else if (engine.state.scroll_tile_x >= 48) {
        // Past the right margin but the camera is at the rightmost tile (48):
        // clamp it there.
        engine.state.scroll_tile_x = 48;
        engine.state.scroll_fine_x = 0;
        no_scroll_column_needed = 1;
    } else {
        // Scroll right so the player sits 9 tiles in from the left.
        engine.state.scroll_tile_x = engine.state.player_x_tile - 9;
        engine.state.scroll_fine_x = engine.state.player_x_fine;
        engine.state.camera_scroll_flag = 1; // scrolling right
        no_scroll_column_needed = 0;
    }
    // Recompute the PPU scroll shadows from the (possibly updated) camera.
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
    // Combine tile X (<<4) and fine X into the 8-bit fine-scroll pixel value.
    let scroll_pixel_x: i32 = (((scroll_tile_x << 4) | scroll_fine_x) as u8 as i32);
    // Bit 4 of tile X (i.e. crossing 16 tiles = 256 px) selects the nametable.
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
    // During the invulnerability blink, hide both halves on even frames.
    if ((engine.state.sprite_blink_timer != 0)
        && ((engine.state.frame_prescaler & ((crate::bits::BIT0) as u8)) == 0))
    {
        engine.state.set_oam_y(16, 239); // 239 = offscreen Y
        engine.state.set_oam_y(20, 239);
        return;
    }

    // Screen Y = player Y plus the 43-pixel status-bar offset.
    let sprite_y: i32 = ((engine.state.player_y + 43) as u8 as i32);
    engine.state.set_oam_y(16, sprite_y);
    engine.state.set_oam_y(20, sprite_y);

    // Convert player world X to screen X by subtracting the camera world X
    // (both packed as tile<<4 | fine, wrapping as a byte).
    let camera_world_x: i32 = ((((engine.state.scroll_tile_x as i32) << 4)
        | (engine.state.scroll_fine_x as i32)) as u8 as i32);
    let player_world_x: i32 = ((((engine.state.player_x_tile as i32) << 4)
        | (engine.state.player_x_fine as i32)) as u8 as i32);
    let screen_x: i32 = ((player_world_x - camera_world_x) as u8 as i32);
    engine.state.scratch0 = (camera_world_x as u8);
    engine.state.set_oam_x(16, screen_x);
    engine.state.set_oam_x(20, screen_x + 8); // right half 8 px over
    // Both halves share the facing attribute (BIT6 = horizontal flip).
    engine
        .state
        .set_oam_attr(16, (engine.state.player_facing as i32));
    engine
        .state
        .set_oam_attr(20, (engine.state.player_facing as i32));

    // Assign tiles; swap left/right when horizontally flipped (BIT6 set).
    let left_tile: i32 = (engine.state.player_pose as i32);
    if ((engine.state.player_facing & ((crate::bits::BIT6) as u8)) != 0) {
        engine.state.set_oam_tile(20, left_tile);
        engine.state.set_oam_tile(16, left_tile + 2); // +2 = adjacent 8x16 tile
    } else {
        engine.state.set_oam_tile(16, left_tile);
        engine.state.set_oam_tile(20, left_tile + 2);
    }
}

/// Draws the selected item cursor and the three equipped item icons in the
/// status area. High-bit item ids hide a slot.
pub fn draw_status_item_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // Selection cursor (OAM bytes 56/60). Slot >= 3 means "no selection".
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    if (selected_slot >= 3) {
        engine.state.set_oam_y(56, 239); // hidden offscreen
        engine.state.set_oam_y(60, 239);
    } else {
        // Cursor X = slot * 16 + 200 (slots are spaced 16 px in the status bar).
        let cursor_x: i32 = (((selected_slot << 4) + 200) as u8 as i32);
        engine.state.set_oam_y(56, 19); // status-bar row Y
        engine.state.set_oam_y(60, 19);
        engine.state.set_oam_x(56, cursor_x);
        engine.state.set_oam_x(60, cursor_x + 8); // right half
        engine.state.set_oam_tile(56, 255); // cursor tile $FF
        engine.state.set_oam_tile(60, 255);
        engine.state.set_oam_attr(56, 1); // palette 1
        engine.state.set_oam_attr(60, 65); // palette 1 + BIT6 (flip) = 0x41
    }

    // Draw the three equipped item icons (slots 2..0).
    for item_slot in (0..=2).rev() {
        let oam_offset: i32 = item_slot << 3; // 8 OAM bytes (2 sprites) per slot
        let item_id: i32 = engine.state.item_slot(item_slot);
        let sprite_y: i32 = if ((item_id & crate::bits::BIT7) != 0) {
            239 // high-bit item id = empty slot, hide it
        } else {
            // Each item uses a 4-tile block: left tile = id*4 + 161.
            let left_tile: i32 = (((item_id << 2) + 161) as u8 as i32);
            // Icon X = slot*16 + 200 (oam_offset<<1 == item_slot*16).
            let left_x: i32 = (((oam_offset << 1) + 200) as u8 as i32);
            engine.state.set_oam_tile(32 + oam_offset, left_tile);
            engine.state.set_oam_tile(36 + oam_offset, left_tile + 2); // right tile
            engine.state.set_oam_x(32 + oam_offset, left_x);
            engine.state.set_oam_x(36 + oam_offset, left_x + 8);
            engine.state.set_oam_attr(32 + oam_offset, 1); // palette 1
            engine.state.set_oam_attr(36 + oam_offset, 1);
            19 // visible status-bar row Y
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
    engine.state.scratch2 = 16; // process 16 object slots this pass
    let mut oam_offset: i32 = (engine.state.oam_cursor as i32);
    let mut object_offset: i32 = (engine.state.sprite_index as i32);
    loop {
        // Draw one object slot into a two-sprite OAM entry.
        r.index = (oam_offset as u8);
        r.offset = (object_offset as u8);
        draw_object_slot_sprites(engine, r);
        // Advance the OAM cursor by 8 bytes (2 sprites) and re-set BIT7, which
        // marks the cursor as a rolling object-sprite slot.
        oam_offset = ((((oam_offset + 8) as u8 as i32) | crate::bits::BIT7) as u8 as i32);
        object_offset = ((object_offset + 48) as u8 as i32); // 48-byte object record stride
        engine.state.scratch2 = engine.state.scratch2 - 1;
        if (engine.state.scratch2 == 0) {
            break;
        }
    }
    // Persist the rolling cursors for the next scheduler pass.
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
    // r.index = OAM byte offset; r.offset = object record offset within the
    // object table at $0400. The object record fields used below:
    //   +0  left tile id          +1  active/type flag
    //   +2  OAM attributes        +12 fine X (sub-tile)
    //   +13 tile X                +14 world Y
    //   +15 one-shot X nudge
    let oam_offset: i32 = (r.index as u8 as i32);
    let object_offset: i32 = (r.offset as u8 as i32);
    let object_base: i32 = OBJECT_TABLE_BASE + object_offset;

    // Inactive (flag 0) or below the bottom of the playfield (Y >= 191): hide.
    if (engine.state.byte(object_base + 1) == 0) || (engine.state.byte(object_base + 14) >= 191) {
        engine.state.set_oam_y(oam_offset, 239); // 239 = offscreen Y
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    // Both halves share the object's OAM attribute byte.
    let attributes: i32 = engine.state.byte(object_base + 2);
    engine.state.set_oam_attr(oam_offset, attributes);
    engine.state.set_oam_attr(4 + oam_offset, attributes);

    // Tile assignment; swap halves when horizontally flipped (BIT6).
    let left_tile: i32 = engine.state.byte(object_base);
    if ((attributes & crate::bits::BIT6) != 0) {
        engine.state.set_oam_tile(4 + oam_offset, left_tile);
        engine.state.set_oam_tile(oam_offset, left_tile + 2); // +2 adjacent 8x16 tile
    } else {
        engine.state.set_oam_tile(oam_offset, left_tile);
        engine.state.set_oam_tile(4 + oam_offset, left_tile + 2);
    }

    // Horizontal scroll math: subtract camera fine X from object fine X. The
    // +256 keeps the subtraction non-negative so bit8 becomes the tile borrow.
    let subtile_delta: i32 = ((engine.state.byte(object_base + 12)) as u16 as i32) + 256
        - (engine.state.scroll_fine_x as i32);
    let fine_x: i32 = (subtile_delta as u8 as i32) & crate::bits::LOW_NIBBLE; // sub-tile pixel
    let tile_borrow: i32 = ((subtile_delta >> 8) as u8 as i32); // 1 if fine X borrowed
    // Tile X relative to the camera (minus 1 for the off-screen left column).
    let tile_delta: i32 = ((((engine.state.byte(object_base + 13)) as u16 as i32) + tile_borrow
        - (engine.state.scroll_tile_x as i32)
        - 1) as u8 as i32);
    // More than 16 tiles off the left/right edge: not visible, hide.
    if (tile_delta >= 16) {
        engine.state.set_oam_y(oam_offset, 239);
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    // Screen X = tile_delta * 16 + fine_x.
    let mut screen_x: i32 = (((tile_delta << 4) | fine_x) as u8 as i32);
    engine.state.scratch0 = (screen_x as u8);

    // Type-1 objects with a pending one-shot nudge (+15) shift once, then clear.
    if (engine.state.byte(object_base + 1) == 1) && (engine.state.byte(object_base + 15) != 0) {
        screen_x = ((screen_x + engine.state.byte(object_base + 15)) as u8 as i32);
        engine.state.scratch0 = (screen_x as u8);
        engine.state.set_byte(object_base + 15, 0);
    }

    // Screen Y = world Y + 43 status-bar offset; place the left sprite.
    let sprite_y: i32 = ((engine.state.byte(object_base + 14) + 43) as u8 as i32);
    engine.state.set_oam_x(oam_offset, screen_x);
    engine.state.set_oam_y(oam_offset, sprite_y);
    // If the right half would wrap past the screen edge (>= 239), hide only it.
    if (screen_x >= 239) {
        engine.state.set_oam_y(4 + oam_offset, 239);
        return;
    }

    // Place the right half 8 pixels to the right.
    engine.state.set_oam_x(4 + oam_offset, screen_x + 8);
    engine.state.set_oam_y(4 + oam_offset, sprite_y);
}

/// Clears staged OAM while preserving the sprite-zero template.
///
/// The first sprite is copied from `0xFF6B..0xFF6E`; every remaining OAM
/// byte is set to `0xF8`, the offscreen clear value used by the startup and
/// title flows.
pub fn clear_oam_with_sprite_zero_template(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy the 4-byte sprite-zero template from ROM ($FF6B) into OAM bytes 0..3.
    for template_offset in 0..=3 {
        engine.state.set_oam_y(
            template_offset,
            engine
                .state
                .byte((SPRITE_Y_TABLE_F + template_offset) as u16 as i32),
        );
    }
    // Blank every remaining OAM byte (4..255) to the offscreen clear value.
    for oam_offset in 4..=255 {
        engine.state.set_oam_y(oam_offset, 248); // $F8 offscreen clear value
    }
    r.index = 0;
}

/// Clears the visible nametables to blank tile `0xC0` with zero attributes.
///
/// Rendering is disabled around the direct PPU writes and the PPUCTRL/PPUMASK
/// shadows are restored before returning.
pub fn clear_name_tables_to_blank_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Snapshot and disable rendering before writing VRAM (see
    // upload_title_screen_nametables for the same bit masking).
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
    // Point PPUADDR at $2000 (nametable 0).
    engine.device_write(crate::engine::reg::PPU_ADDR, 32); // high byte $20
    engine.device_write(crate::engine::reg::PPU_ADDR, 0); // low byte $00

    // Two nametables: each gets 960 tile bytes (5*192) of blank tile $C0
    // followed by 64 attribute bytes of 0.
    for _ in 0..2 {
        for _ in 0..(5 * 192) {
            engine.device_write(crate::engine::reg::PPU_DATA, 192); // $C0 blank tile
        }
        for _ in 0..64 {
            engine.device_write(crate::engine::reg::PPU_DATA, 0); // attribute table
        }
    }
    // Restore the PPUCTRL/PPUMASK shadows and re-enable rendering.
    engine.state.ppu_mask_shadow = (mask as u8);
    engine.state.ppu_ctrl_shadow = (ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl);
    r.value = (ctrl as u8);
    r.index = 0;
    r.offset = 0;
}

/// Dims `r.offset` palette buffer entries starting at index `r.index` by
/// subtracting the brightness step in `0x09` (`scratch1`) from each color.
///
/// NES palette bytes encode brightness in the high nibble (the value byte) and
/// hue in the low nibble. Each entry's high nibble is reduced by `fade_step`
/// (the step is itself a high-nibble value); if it would go below the step the
/// color is forced to black (0x0F). On return `r.index`/`r.offset` reflect the
/// advanced cursor and the exhausted count, so callers can chain ranges.
pub fn dim_palette_range_by_step(engine: &mut Engine, r: &mut RoutineContext) {
    let mut palette_offset: i32 = (r.index as u8 as i32);
    let mut remaining: i32 = (r.offset as u8 as i32);
    loop {
        let color = engine.state.palette_buffer(palette_offset);
        let low_nibble: i32 = color & crate::bits::LOW_NIBBLE; // hue (preserved)
        engine.state.scratch0 = (low_nibble as u8);
        let high_nibble: i32 = color & crate::bits::HIGH_NIBBLE; // brightness
        let fade_step: i32 = (engine.state.scratch1 as i32);
        // Subtract the step from brightness, or black out if it would underflow.
        let dimmed_color: i32 = if (high_nibble >= fade_step) {
            ((((high_nibble - fade_step) as u8 as i32) | low_nibble) as u8 as i32)
        } else {
            15 // $0F black
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
    // Report the advanced cursor / exhausted count back to the caller.
    r.index = (palette_offset as u8);
    r.offset = (remaining as u8);
}

/// Queues a PPU upload of the palette buffer to `$3F00`.
/// Queues the staged palette buffer for upload to PPU palette RAM.
///
/// Drops any pending deferred VRAM job, points the upload destination at the
/// PPU palette region ($3F00, encoded here as hi=63/lo=0), selects palette job
/// type 2 in `r.value`, and dispatches the job, blocking until the NMI handler
/// has serviced it.
pub fn upload_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    clear_pending_vram_job(engine, r);
    // Target the PPU palette area at $3F00 (hi=$3F=63, lo=$00).
    engine.state.vram_addr_lo = 0;
    engine.state.vram_addr_hi = 63;
    r.value = 2; // VRAM job selector: palette buffer upload
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the fixed status-panel nametable template and clears its
/// attribute bytes.
///
/// Runs with rendering temporarily reconfigured: PPU_CTRL has its NMI/sprite
/// bits cleared and PPU_MASK has its background/sprite show bits cleared so the
/// CPU can write nametable RAM directly. The 160-byte HUD template at
/// `HUD_TEMPLATE_TABLE` is copied to nametable $2320, then the 16 attribute
/// bytes at $23F0 are zeroed. The saved PPU_CTRL/PPU_MASK shadows are restored
/// on exit. `r.value` returns the restored PPU_CTRL value; `r.offset`=0.
pub fn upload_status_panel_template(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_ctrl: i32 = 0;
    let mut saved_mask: i32 = 0;
    let mut i: i32 = 0;
    clear_pending_vram_job(engine, r);
    // Save PPU_CTRL and disable NMI + high bits so direct VRAM writes are safe.
    saved_ctrl = (engine.state.ppu_ctrl_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        saved_ctrl & crate::bits::CLEAR_BITS_2_7,
    );
    engine.state.statusbar_split_flag = 0;
    // Save PPU_MASK and clear the show-background/show-sprites bits (3,4).
    saved_mask = (engine.state.ppu_mask_shadow as i32);
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        saved_mask & crate::bits::CLEAR_BITS_3_4,
    );
    // Set PPU address to nametable $2320 (hi=$23=35, lo=$20=32).
    engine.device_write(crate::engine::reg::PPU_ADDR, 35);
    engine.device_write(crate::engine::reg::PPU_ADDR, 32);
    {
        // Copy 160 bytes of HUD nametable template into VRAM (5 rows x 32).
        i = 0;
        while (i < 160) {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.byte((HUD_TEMPLATE_TABLE + i) as u16 as i32),
            );
            {
                i += 1;
                i
            };
        }
    }
    // Set PPU address to the HUD attribute table at $23F0 (hi=35, lo=$F0=240).
    engine.device_write(crate::engine::reg::PPU_ADDR, 35);
    engine.device_write(crate::engine::reg::PPU_ADDR, 240);
    {
        // Zero the 16 attribute bytes covering the status panel rows.
        i = 0;
        while (i < 16) {
            engine.device_write(crate::engine::reg::PPU_DATA, 0);
            {
                i += 1;
                i
            };
        }
    }
    // Mark the status-bar split as armed again and restore PPU register shadows.
    engine.state.statusbar_split_flag =
        (engine.state.statusbar_split_flag + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.ppu_mask_shadow = (saved_mask as u8);
    engine.state.ppu_ctrl_shadow = (saved_ctrl as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, saved_ctrl);
    r.value = (saved_ctrl as u8);
    r.offset = 0;
}

/// Resolves the current scroll column and uploads the full room view.
///
/// Seeds the room tile pointer at `0x0C/0x0D` with the current scroll column
/// (rounded down to an even column via clearing bit 0) and row 0, resolves it
/// into the room tile pointer, then uploads the whole 18x12-tile room view.
pub fn upload_current_room_view(engine: &mut Engine, r: &mut RoutineContext) {
    // Even tile column = scroll_tile_x with bit 0 cleared; row index 0.
    engine.state.data_ptr_lo = engine.state.scroll_tile_x & ((crate::bits::CLEAR_BIT0) as u8);
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    upload_room_view_from_tile_pointer(engine, r);
}

/// Uploads the full room view from the staged room tile pages.
///
/// Like [`upload_current_room_view`], but after resolving the tile pointer the
/// high byte is rebased onto the staged room metadata page: undoing the +5
/// page offset added by [`resolve_room_tile_pointer`] and adding the staged
/// room data high byte so reads come from the room staging buffer instead.
pub fn upload_staged_room_view(engine: &mut Engine, r: &mut RoutineContext) {
    // Even tile column (clear bit 0), row 0.
    engine.state.data_ptr_lo = engine.state.scroll_tile_x & ((crate::bits::CLEAR_BIT0) as u8);
    engine.state.data_ptr_hi = 0;
    resolve_room_tile_pointer(engine, r);
    // Rebase pointer onto staged room page: -5 (undo resolve's page bias) + staged hi.
    engine.state.data_ptr_hi = (engine.state.data_ptr_hi - 5) + engine.state.room_metadef_hi;
    upload_room_view_from_tile_pointer(engine, r);
}

/// Uploads room tiles and attributes from the tile pointer in `0x0C/0x0D`.
///
/// This is the full-room blit. It writes both the 18-column x 12-row tile grid
/// (each room "tile" expands to a 2x2 group of PPU tiles via the tile-table at
/// `tile_table_ptr`) and the corresponding attribute table for the room area.
///
/// PPU_CTRL is set to a known state (low 7 bits preserved, bit 2 = VRAM +1
/// increment set), and rendering is disabled in PPU_MASK while writing. The
/// saved PPU register shadows and the `0x0C/0x0D` pointer are restored on exit.
/// `r.value` returns the restored PPU_CTRL value; `r.index`=0.
///
/// Two pointers drive the inner loops: `p0C` is the room tile-index pointer
/// (`0x0C/0x0D`), `p79` is the room tile-table base. The first phase walks 18
/// columns, each writing two interleaved 12-byte rows of expanded tiles. The
/// second phase rebuilds the attribute bytes by packing 2-bit palette indices
/// from four neighbouring room tiles.
pub fn upload_room_view_from_tile_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    // Save PPU register shadows and the room tile pointer to restore on exit.
    let mut ctrl_save: i32 = (engine.state.ppu_ctrl_shadow as i32);
    let mut v29_save: i32 = (engine.state.statusbar_split_flag as i32);
    let mut v24_save: i32 = (engine.state.ppu_mask_shadow as i32);
    let mut c0c_save: i32 = (engine.state.data_ptr_lo as i32);
    let mut c0d_save: i32 = (engine.state.data_ptr_hi as i32);
    let mut p0C: i32 = 0;
    let mut p79: i32 = 0;
    let mut outer: i32 = 0;
    // PPU_CTRL: keep low 7 bits, force bit 2 (VRAM address += 1 per write).
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        (ctrl_save & crate::bits::LOW_7_BITS) | crate::bits::BIT2,
    );
    engine.state.statusbar_split_flag = 0;
    // Disable show-background/show-sprites (bits 3,4) during VRAM writes.
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        v24_save & crate::bits::CLEAR_BITS_3_4,
    );
    p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
    {
        // Compute initial nametable VRAM address from the scroll column.
        // lo = (sx*2) within bits 2..4; hi selects the nametable via bit 4.
        let mut sx: i32 = (engine.state.scroll_tile_x as i32);
        let mut lo: i32 = (((sx << 1) & crate::bits::BITS_2_3_4) as u8 as i32);
        let mut hi: i32 = (((sx & crate::bits::BIT4) >> 2) as u8 as i32);
        let mut t: i32 = ((0 + lo) as u16 as i32);
        engine.state.vram_addr_lo = (t as u8);
        // Base nametable hi byte $20=32 plus nametable-select offset.
        engine.state.vram_addr_hi = ((32 + hi + (t >> 8)) as u8);
    }
    engine.state.scratch2 = 18; // outer column counter: 18 room columns
    p0C = ((c0c_save | (c0d_save << 8)) as u16 as i32);
    {
        outer = 0;
        while (outer < 18) {
            let mut inner: i32 = 0;
            engine.state.scratch3 = 12; // 12 rows per column (top half pass)
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_hi as i32),
            );
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_lo as i32),
            );
            // Top half of column: for each of 12 room tiles, look up its 4-byte
            // tile-table entry and write the two top sub-tiles (entry[0], [1]).
            engine.state.scratch0 = 0;
            loop {
                let mut idx: i32 = engine
                    .state
                    .byte((p0C + (engine.state.scratch0 as i32)) as u16 as i32);
                let mut y: i32 = ((idx << 2) as u8 as i32); // tile index * 4 (4 bytes/entry)
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine.state.byte((p79 + y) as u16 as i32),
                );
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine
                        .state
                        .byte((p79 + ((y + 1) as u8 as i32)) as u16 as i32),
                );
                engine.state.scratch0 =
                    (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
                engine.state.scratch3 =
                    (engine.state.scratch3 - 1) & ((crate::bits::BYTE_MASK) as u8);
                if (engine.state.scratch3 == 0) {
                    break;
                }
            }
            // Bottom half of the same column on the next nametable row.
            engine.state.scratch3 = 12; // 12 rows again (bottom half pass)
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr_hi as i32),
            );
            inner = ((engine.state.vram_addr_lo + 1) as u8 as i32); // second sub-tile column
            engine.device_write(crate::engine::reg::PPU_ADDR, inner);
            // Write the two bottom sub-tiles (entry[2], [3]) for each room tile.
            engine.state.scratch0 = 0;
            loop {
                let mut idx: i32 = engine
                    .state
                    .byte((p0C + (engine.state.scratch0 as i32)) as u16 as i32);
                let mut y: i32 = (((idx << 2) + 2) as u8 as i32); // tile*4 + 2 = bottom pair
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine.state.byte((p79 + y) as u16 as i32),
                );
                engine.device_write(
                    crate::engine::reg::PPU_DATA,
                    engine
                        .state
                        .byte((p79 + ((y + 1) as u8 as i32)) as u16 as i32),
                );
                engine.state.scratch0 =
                    (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
                engine.state.scratch3 =
                    (engine.state.scratch3 - 1) & ((crate::bits::BYTE_MASK) as u8);
                if (engine.state.scratch3 == 0) {
                    break;
                }
            }
            // Advance the nametable column by 2 sub-tiles; wrap to the adjacent
            // nametable (toggle bit 2 of hi) when crossing 32 columns (bit 5).
            engine.state.vram_addr_lo =
                (engine.state.vram_addr_lo + 2) & ((crate::bits::BYTE_MASK) as u8);
            if ((engine.state.vram_addr_lo & ((crate::bits::BIT5) as u8)) != 0) {
                engine.state.vram_addr_lo = 0;
                engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
            }
            {
                // Advance room tile pointer by 12 (one column = 12 room tiles).
                let mut t: i32 = ((12 + engine.state.data_ptr_lo as i32) as u16 as i32);
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
    // Restore the room tile pointer for the attribute-table pass.
    engine.state.data_ptr_hi = (c0d_save as u8);
    engine.state.data_ptr_lo = (c0c_save as u8);
    p0C = ((c0c_save | (c0d_save << 8)) as u16 as i32);
    {
        // Compute the starting attribute-table VRAM address from scroll column.
        let mut sx: i32 = (engine.state.scroll_tile_x as i32);
        let mut lo: i32 = (((sx >> 1) & crate::bits::LOW_3_BITS) as u8 as i32);
        let mut hi: i32 = (((sx & crate::bits::BIT4) >> 2) as u8 as i32);
        let mut t: i32 = ((192 + lo) as u16 as i32); // attribute area offset $C0=192
        engine.state.vram_addr_lo = (t as u8);
        // Attribute table base hi byte $23=35 plus nametable select.
        engine.state.vram_addr_hi = ((35 + hi + (t >> 8)) as u8);
    }
    engine.state.scratch2 = 9; // 9 attribute rows
    loop {
        let mut x: i32 = 0;
        {
            // Each attribute byte packs the top 2 bits of four room tiles
            // (a 2x2 quadrant), accumulated into scratch0 by shifting in pairs
            // of high bits. 6 attribute bytes are emitted per row.
            x = 6;
            while (x > 0) {
                let mut a: i32 = 0;
                // Tile at room offset +13 (lower-right of quadrant).
                a = engine.state.byte((p0C + 13) as u16 as i32);
                {
                    let mut c1: i32 = (a >> 7) & 1; // shift bit 7 into scratch0
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                {
                    let mut c1: i32 = (a >> 7) & 1; // shift next (now-bit7) bit in
                    a = ((a << 1) as u8 as i32);
                    engine.state.scratch0 =
                        ((((engine.state.scratch0 as i32) << 1) | ((c1 as u8) as i32)) as u8);
                }
                // Tile at room offset +1 (upper-right of quadrant).
                a = engine.state.byte((p0C + 1) as u16 as i32);
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
                // Tile at room offset +12 (lower-left of quadrant).
                a = engine.state.byte((p0C + 12) as u16 as i32);
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
                // Tile at room offset +0 (upper-left of quadrant).
                a = engine.state.byte((p0C + 0) as u16 as i32);
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
                // Set the VRAM address for this attribute byte and emit it.
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
                    // Advance room pointer by 2 tiles (one attribute quadrant wide).
                    let mut t: i32 = ((2 + engine.state.data_ptr_lo as i32) as u16 as i32);
                    engine.state.data_ptr_lo = (t as u8);
                    engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((t >> 8) as u8);
                }
                {
                    // Advance attribute VRAM address by 8 bytes (next quadrant).
                    let mut t: i32 = ((8 + engine.state.vram_addr_lo as i32) as u16 as i32);
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
            // Move room pointer to next attribute row: +12 (one room column).
            let mut t: i32 = ((12 + engine.state.data_ptr_lo as i32) as u16 as i32);
            engine.state.data_ptr_lo = (t as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((t >> 8) as u8);
        }
        {
            // Move attribute VRAM address back/down: +209 lo, -1 (+255) hi page wrap.
            let mut t: i32 = ((209 + engine.state.vram_addr_lo as i32) as u16 as i32);
            engine.state.vram_addr_lo = (t as u8);
            engine.state.vram_addr_hi = engine.state.vram_addr_hi + 255 + ((t >> 8) as u8);
        }
        p0C = ((engine.state.data_ptr()) as u16 as i32);
        // Wrap to the adjacent nametable's attribute table when crossing bit 3.
        if ((engine.state.vram_addr_lo & ((crate::bits::BIT3) as u8)) != 0) {
            engine.state.vram_addr_lo = 192; // back to attribute area base $C0
            engine.state.vram_addr_hi = engine.state.vram_addr_hi ^ ((crate::bits::BIT2) as u8);
        }
        engine.state.scratch2 = (engine.state.scratch2 - 1) & ((crate::bits::BYTE_MASK) as u8);
        if (engine.state.scratch2 == 0) {
            break;
        }
    }
    // Restore saved PPU register shadows and re-enable the previous PPU_CTRL.
    engine.state.ppu_mask_shadow = (v24_save as u8);
    engine.state.statusbar_split_flag = (v29_save as u8);
    engine.state.ppu_ctrl_shadow = (ctrl_save as u8);
    engine.device_write(crate::engine::reg::PPU_CTRL, ctrl_save);
    r.value = (ctrl_save as u8);
    r.index = 0;
}

/// Uploads the 16 visible room columns using the bank-9 room-column builder.
///
/// Computes the starting nametable VRAM address from the scroll column, then
/// loops 16 times — once per visible column — invoking the bank-9 column
/// builder ([`farcall_bank_09_r7`]) to write each 2-tile-wide column and
/// advancing the destination address, wrapping across nametables on overflow.
pub fn upload_room_columns_from_bank9(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sx: i32 = 0;
    clear_pending_vram_job(engine, r);
    sx = (engine.state.scroll_tile_x as i32);
    // Initial nametable address: lo = (sx*2) within 5 bits, hi from bit 4.
    engine.state.vram_addr_lo = (((sx << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((sx & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // nametable base hi $20
    engine.state.scratch0 = (sx as u8); // current column index
    engine.state.scratch1 = 16; // 16 columns to upload
    loop {
        engine.state.data_ptr_lo = engine.state.scratch0;
        farcall_bank_09_r7(engine, r);
        // Advance by 2 sub-tile columns; wrap nametables at column 32 (bit 5).
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
///
/// Identical structure to [`upload_room_columns_from_bank9`], but each column
/// is built locally from the staged room buffer via
/// [`build_staged_room_column`] rather than the bank-9 far call.
pub fn upload_staged_room_columns(engine: &mut Engine, r: &mut RoutineContext) {
    let mut sx: i32 = 0;
    clear_pending_vram_job(engine, r);
    sx = (engine.state.scroll_tile_x as i32);
    // Initial nametable address from the scroll column (see bank-9 variant).
    engine.state.vram_addr_lo = (((sx << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((sx & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // nametable base hi $20
    engine.state.scratch0 = (sx as u8); // current column index
    engine.state.scratch1 = 16; // 16 columns to upload
    loop {
        engine.state.data_ptr_lo = engine.state.scratch0;
        build_staged_room_column(engine, r);
        // Advance 2 sub-tile columns; wrap nametables at column 32 (bit 5).
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
///
/// Chooses which single column to redraw based on the scroll direction flag:
/// when scrolling left (camera flag bit 7 set) the new column is the current
/// `scroll_tile_x`; when scrolling right it is 16 columns ahead (just past the
/// visible window). The destination nametable address is derived from that
/// column and the bank-9 builder writes the column.
pub fn upload_scroll_edge_room_column(engine: &mut Engine, r: &mut RoutineContext) {
    let mut col: i32 = 0;
    clear_pending_vram_job(engine, r);
    if ((engine.state.camera_scroll_flag & ((crate::bits::BIT7) as u8)) != 0) {
        // Scrolling left: redraw the leftmost (current) column.
        col = (engine.state.scroll_tile_x as i32);
    } else {
        // Scrolling right: redraw the column 16 ahead (off-screen right edge).
        col = ((engine.state.scroll_tile_x + 16) as u8 as i32);
    }
    engine.state.data_ptr_lo = (col as u8);
    // Nametable address from the chosen column (lo within 5 bits, hi from bit 4).
    engine.state.vram_addr_lo = (((col << 1) & crate::bits::LOW_5_BITS) as u8);
    engine.state.vram_addr_hi = (((col & crate::bits::BIT4) >> 2) as u8);
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // nametable base hi $20
    farcall_bank_09_r7(engine, r);
}

/// Builds one staged room column from the current room tile pointer and
/// tileset metadata.
///
/// The column index is already in `0x0C` (`data_ptr_lo`); row 0 is forced,
/// the tile pointer is resolved, then its high byte is rebased onto the staged
/// room page (undo resolve's +5 page bias, add staged room hi) and the column
/// is queued for VRAM upload.
pub fn build_staged_room_column(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.data_ptr_hi = 0; // row 0
    resolve_room_tile_pointer(engine, r);
    // Rebase high byte: -5 (undo resolve page bias) + staged room data hi byte.
    engine.state.data_ptr_hi = ((((engine.state.data_ptr_hi - 5) as u8 as i32)
        + (engine.state.room_metadef_hi as i32)) as u8);
    queue_room_column_vram_upload(engine, r);
}

/// Selects the room data bank/pointers, derives room metadata, and builds
/// the palette buffer for the active room.
///
/// High-level room-load helper: pick the PRG bank and base pointers for the
/// current map screen, build the room's text/attribute metadata, then build
/// the room palette buffer (including the active family member's palette).
pub fn prepare_room_metadata_and_palette(engine: &mut Engine, r: &mut RoutineContext) {
    select_room_data_bank_and_pointers(engine, r);
    text_attr_build(engine, r);
    build_room_palette_buffer(engine, r);
}

/// Copies three room tile pages from the active room data pointer into
/// `0x0500..0x07FF`.
///
/// Seeds the source pointer `0x49/0x4A` from the room metadata pointer, then
/// copies 3 consecutive 256-byte pages into the room staging buffer starting
/// at `ROOM_BUFFER_BASE` ($0500). The source high byte is incremented per page
/// and written back to `palette_src_ptr_hi`. `r.offset`=0 on exit.
pub fn copy_room_tile_pages(engine: &mut Engine, r: &mut RoutineContext) {
    // Source pointer = room metadata pointer (low/high).
    engine.state.palette_src_ptr_lo = engine.state.room_metadef_lo;
    engine.state.palette_src_ptr_hi = engine.state.room_metadef_hi;

    let source_lo: i32 = (engine.state.palette_src_ptr_lo as i32);
    let mut source_hi: i32 = (engine.state.palette_src_ptr_hi as i32);
    for page_index in 0..=2 {
        // page_index 0..2 → three 256-byte pages of room tile data.
        let source_ptr: i32 = ((source_lo | (source_hi << 8)) as u16 as i32);
        let dest_base: i32 = ROOM_BUFFER_BASE + (page_index << 8); // $0500 + page*256
        for page_offset in 0..256 {
            engine.state.set_byte(
                dest_base + page_offset,
                engine
                    .state
                    .byte((source_ptr + page_offset) as u16 as i32),
            );
        }
        // Advance source by one page and persist the high byte.
        source_hi += 1;
        engine.state.palette_src_ptr_hi = (source_hi as u8);
    }
    r.offset = 0;
}

/// Selects the PRG bank and base room data pointers for `0x47/0x48`.
///
/// The map screen Y >> 1 selects the PRG bank mapped at $8000; if it differs
/// from the current bank the new bank is swapped in (job 255 = bank switch,
/// waited on). The room-table offset is `((mapY&1)<<2 | mapX) << 2` (4 rooms
/// per bank row, 4 bytes per room pointer entry). Two base pointers are set:
/// `room_metadef` (offset+$80) and `palette_src` (offset+$80+3, the +3 page
/// being the palette/attribute sub-table). `r.carry` reflects the +3 carry.
pub fn select_room_data_bank_and_pointers(engine: &mut Engine, r: &mut RoutineContext) {
    // Each PRG bank holds two map rows, so bank = mapY / 2.
    let room_bank: i32 = ((engine.state.map_screen_y >> 1) as u8 as i32);
    if (room_bank != (engine.state.prg_bank_8000 as i32)) {
        engine.state.prg_bank_8000 = (room_bank as u8);
        r.value = 255; // VRAM job selector: PRG bank switch
        queue_ppu_job_and_wait(engine, r);
    }

    // offset = ((mapY&1)<<2 | mapX) << 2 : 4 rooms/row, 4-byte pointer stride.
    let room_table_offset: i32 = ((((((engine.state.map_screen_y & ((crate::bits::BIT0) as u8))
        << 2) as u8 as i32)
        | (engine.state.map_screen_x as i32))
        << 2) as u8 as i32);
    let room_ptr_lo: i32 = ((room_table_offset + 128) as u8 as i32); // +$80 page base
    engine.state.room_metadef_hi = (room_ptr_lo as u8);
    engine.state.palette_src_ptr_hi = ((room_ptr_lo + 3) as u8); // +3 = palette sub-page
    engine.state.palette_src_ptr_lo = 0;
    engine.state.room_metadef_lo = 0;
    r.carry = ((if ((room_ptr_lo + 3) > 255) { 1 } else { 0 }) as u8);
}

/// Copies room palette/attribute bytes into the palette buffer and applies
/// the active family-member palette when applicable.
///
/// Copies the 32 room palette bytes (source offsets $E0..$FF) into the
/// inventory/palette staging area. Then, if a valid family member is active
/// (`character_index` < 6), overwrites one 4-entry palette slot (staged at
/// offset 80) with that member's palette from `FAMILY_PALETTE_TABLE`, copied
/// in reverse. When `character_index` >= 6 (no family member) the family
/// palette step is skipped and carry is set.
pub fn build_room_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy the room's 32 palette bytes (source $E0..$FF = 224..255).
    let room_palette_ptr: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    for room_palette_offset in 224..=255 {
        engine.state.set_inventory_item(
            64 + room_palette_offset,
            engine
                .state
                .byte((room_palette_ptr + room_palette_offset) as u16 as i32),
        );
    }

    // 6 = number of playable family members; >=6 means none → skip overlay.
    let family_member: i32 = (engine.state.character_index as i32);
    if (family_member >= 6) {
        r.value = (family_member as u8);
        r.carry = 1;
        return;
    }

    // Each member has a 4-byte palette: end index = member*4 + 3, copied down.
    let family_palette_end_offset: i32 = (((family_member << 2) + 3) as u8 as i32);
    let mut family_palette_offset: i32 = family_palette_end_offset;
    for dest_offset in (0..=3).rev() {
        engine.state.set_vram_stage(
            80 + dest_offset, // staged palette slot at offset 80
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
///
/// Each map screen owns one bit in the save payload (used to record one-shot
/// room events). The byte index is `((mapY<<2)&BIT2) | mapX` and the bit is
/// selected by shifting that byte left by `(mapY>>1)+1` positions so the target
/// bit lands in bit 7 (carry on the original 6502). The shifted byte is
/// returned in `r.value`; the caller inspects its top bit.
pub fn read_room_persistent_flag(engine: &mut Engine, r: &mut RoutineContext) {
    let map_y: i32 = (engine.state.map_screen_y as i32);
    let map_x: i32 = (engine.state.map_screen_x as i32);
    // Flag byte index: high map row contributes bit 2, map X the low bits.
    let flag_byte_index: i32 = ((((map_y << 2) & crate::bits::BIT2) | map_x) as u8 as i32);
    let mut shifted_flags: i32 = engine.state.save_payload(flag_byte_index);
    // Shift the target bit up to bit 7: shift count = (mapY/2)+1.
    let shift_count: i32 = (((map_y >> 1) + 1) as u8 as i32);
    for _ in 0..shift_count {
        shifted_flags = ((shifted_flags << 1) as u8 as i32);
    }
    r.value = (shifted_flags as u8);
}

/// Clears the persistent room-progress bit for the current map coordinates.
///
/// Inverse of [`read_room_persistent_flag`]: builds an AND mask that clears the
/// single bit `128 >> (shift_count-1)` (mirroring the read's left shift) and
/// applies it to the flag byte. `r.value` returns the updated byte and `r.index`
/// the byte index.
pub fn clear_room_persistent_flag(engine: &mut Engine, r: &mut RoutineContext) {
    let map_y: i32 = (engine.state.map_screen_y as i32);
    // Same bit position as the reader: shift count = (mapY/2)+1.
    let shift_count: i32 = (((map_y >> 1) + 1) as u8 as i32);
    // Mask with the target bit cleared: $FF XOR (0x80 >> (shift_count-1)).
    let clear_mask: i32 = ((255 ^ (128 >> (shift_count - 1))) as u8 as i32);
    // Flag byte index: high map row → bit 2, map X → low bits.
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
///
/// On entry `data_ptr_lo` is the tile column and `data_ptr_hi` is the tile row
/// (pixel-ish; the row contribution is `row >> 4`). The column is scaled by the
/// room stride (12) into a byte offset, the row offset is added, and the +5
/// page bias is applied to `data_ptr_hi`. In parallel `aux_ptr` (`0x10/0x11`)
/// is built as the same offset plus the room base pointer
/// (`room_metadef_lo/hi`), giving a directly dereferenceable room data pointer.
pub fn resolve_room_tile_pointer(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_y: i32 = (engine.state.data_ptr_hi as i32);
    // Scale the column by the room stride (sets data_ptr to column*12).
    scale_room_tile_column(engine, r);
    engine.state.aux_ptr_hi = engine.state.data_ptr_hi;
    {
        // Add the row contribution (tile_y >> 4) to the column offset.
        let tile_row: i32 = ((tile_y >> 4) as u8 as i32);
        let room_offset: i32 = ((tile_row + (engine.state.data_ptr_lo as i32)) as u16 as i32);
        engine.state.data_ptr_lo = (room_offset as u8);
        engine.state.tile_fetch_counter = (room_offset as u8);
        // Propagate carry out of the low byte (bit 8) into both high bytes.
        if ((room_offset & crate::bits::BIT8) != 0) {
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + 1;
            engine.state.aux_ptr_hi = engine.state.aux_ptr_hi + 1;
        }
    }
    engine.state.data_ptr_hi = engine.state.data_ptr_hi + 5; // +5 page bias for room buffer
    {
        // Build aux_ptr = offset + room base pointer (room_metadef_lo/hi).
        let room_ptr_lo: i32 = ((engine.state.tile_fetch_counter as i32
            + engine.state.room_metadef_lo as i32) as u16 as i32);
        let carry: i32 = ((room_ptr_lo >> 8) as u8 as i32);
        engine.state.tile_fetch_counter = (room_ptr_lo as u8);
        engine.state.aux_ptr_hi =
            engine.state.aux_ptr_hi + engine.state.room_metadef_hi + (carry as u8);
    }
}

/// Multiplies the tile column in `0x0C` by the room-data stride of 12.
///
/// 12 = 4 + 8, so the result is `(col<<2) + (col<<3)`, computed as a 16-bit
/// value and stored back into `data_ptr_lo/hi`. The intermediate col*4 high and
/// low bytes are returned in `r.index`/`r.offset`, and the product's high byte
/// in `r.value`.
pub fn scale_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
    let column_times_four: i32 = (((engine.state.data_ptr_lo as i32) << 2) as u16 as i32); // col*4
    let column_times_eight: i32 = (((engine.state.data_ptr_lo as i32) << 3) as u16 as i32); // col*8
    let column_offset: i32 = ((column_times_four + column_times_eight) as u16 as i32); // col*12
    engine.state.data_ptr_lo = (column_offset as u8);
    engine.state.data_ptr_hi = ((column_offset >> 8) as u8);
    r.index = ((column_times_four >> 8) as u8);
    r.offset = (column_times_four as u8);
    r.value = ((column_offset >> 8) as u8);
}

/// Queues the resource HUD VRAM upload after resource counters changed.
///
/// Targets the HUD counters area in the status nametable at $2360
/// (hi=$23=35, lo=$60=96), selects VRAM job type 4 (resource HUD), and waits
/// for the NMI handler to service it.
pub fn upload_resource_hud(engine: &mut Engine, r: &mut RoutineContext) {
    clear_pending_vram_job(engine, r);
    // HUD digit area at nametable $2360 (lo=$60=96, hi=$23=35).
    engine.state.vram_addr_lo = 96;
    engine.state.vram_addr_hi = 35;
    r.value = 4; // VRAM job selector: resource HUD upload
    queue_ppu_job_and_wait(engine, r);
}

/// Clamps the health counter and queues the health HUD digits for redraw.
///
/// Health is capped at 109 (the maximum displayable meter value), mirrored to
/// `scratch0` for the meter builder, and the health meter (column 0) is rebuilt
/// in the HUD staging buffers. Sets `hud_refresh_flag` so the new tiles are
/// uploaded on the next frame.
pub fn sync_health_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.player_health as i32;
    if (health >= 109) {
        // 109 = max meter value (clamp to full meter).
        health = 109;
    }
    engine.state.player_health = health as u8;
    engine.state.scratch0 = (health as u8);
    r.value = (health as u8);
    r.index = 0; // meter column 0 = health
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the magic counter and queues the magic HUD digits for redraw.
///
/// Same as [`sync_health_hud`] but for the magic meter (column 6).
pub fn sync_magic_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut magic: i32 = engine.state.player_magic as i32;
    if (magic >= 109) {
        // 109 = max meter value.
        magic = 109;
    }
    engine.state.player_magic = magic as u8;
    engine.state.scratch0 = (magic as u8);
    r.value = (magic as u8);
    r.index = 6; // meter column 6 = magic
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the key counter and queues the key HUD digits for redraw.
///
/// Same as [`sync_health_hud`] but for the key meter (column 12).
pub fn sync_key_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut keys: i32 = (engine.state.keys as i32);
    if (keys >= 109) {
        // 109 = max meter value.
        keys = 109;
    }
    engine.state.keys = (keys as u8);
    engine.state.scratch0 = (keys as u8);
    r.value = (keys as u8);
    r.index = 12; // meter column 12 = keys
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Clamps the coin counter and queues the coin HUD digits for redraw.
///
/// Same as [`sync_health_hud`] but for the coin meter (column 18).
pub fn sync_coin_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut coins: i32 = (engine.state.coins as i32);
    if (coins >= 109) {
        // 109 = max meter value.
        coins = 109;
    }
    engine.state.coins = (coins as u8);
    engine.state.scratch0 = (coins as u8);
    r.value = (coins as u8);
    r.index = 18; // meter column 18 = coins
    build_status_resource_meter_tiles(engine, r);
    r.value = 1;
    engine.state.hud_refresh_flag = 1;
}

/// Builds the two-row status resource meter in the VRAM staging buffers.
/// `r.index` selects the meter column and `0x08` contains the resource value.
///
/// Each meter is 5 tiles wide x 2 rows. Both rows are first filled with the
/// "empty" base tile (top row tile 220 at staging offset 161, bottom row tile
/// 223 at 193), giving a fully-empty meter. The value in `0x08` is then split
/// into full 10-point blocks and a partial block ([`split_meter_value`]); the
/// fill loops decrement the relevant tiles (top row stored on the stack at
/// `STACK_SCRATCH+1`, bottom at `STACK_SCRATCH+33`) to draw filled/partial
/// blocks. The "+1 then -1 if zero" pattern produces the two-pixel-per-step
/// fill granularity of the original 6502 routine.
pub fn build_status_resource_meter_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Top row: 5 empty tiles (tile 220) at staging offset 161+column.
    let base_slot: i32 = (r.index as i32);
    engine.state.set_palette_buffer(123, base_slot); // stash column base in $7B (slot 123)
    for tile_offset in 0..5 {
        engine
            .state
            .set_inventory_item(161 + base_slot + tile_offset, 220);
    }

    // Bottom row: 5 empty tiles (tile 223) at staging offset 193+column.
    let base_slot: i32 = engine.state.palette_buffer(123);
    engine.state.set_palette_buffer(123, base_slot);
    for tile_offset in 0..5 {
        engine
            .state
            .set_inventory_item(193 + base_slot + tile_offset, 223);
    }

    // Split the value into full blocks (r.offset) and partial (scratch0/r.value).
    let base_slot: i32 = engine.state.palette_buffer(123);
    r.index = (base_slot as u8);
    split_meter_value(engine, r);

    // Top-row fill: decrement tiles on the stack buffer for each filled step.
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
                .byte((STACK_SCRATCH + 1 + tile_slot) as u16 as i32)
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
                .byte((STACK_SCRATCH + 1 + tile_slot) as u16 as i32)
                - 1)
                & crate::bits::BYTE_MASK,
        );
        tile_slot = ((tile_slot + 1) as u8 as i32);
    }

    // Bottom-row (partial block) fill: same scheme at stack offset +33.
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
                .byte((STACK_SCRATCH + 33 + tile_slot) as u16 as i32)
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
                .byte((STACK_SCRATCH + 33 + tile_slot) as u16 as i32)
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
///
/// Reads object 0's health, clamps it to 109 (max meter), selects OAM meter
/// slot 0 (`scratch1`=0), and delegates to [`build_health_meter_sprites`] with
/// full tile $A5=165 and empty tile $AB=171.
pub fn build_object_health_meter_alt_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.object_health(0);
    if (health >= 109) {
        health = 109; // 109 = max meter value
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 0; // OAM meter slot 0
    r.index = 165; // full meter tile $A5
    r.offset = 171; // empty meter tile $AB
    build_health_meter_sprites(engine, r);
}

/// Builds an object health meter using the standard `0x65/0x6B` sprite
/// tile pair.
///
/// Like [`build_object_health_meter_alt_tiles`] but the sprite-build logic is
/// inlined here with full tile $65=101 and empty tile $6B=107. The meter is 10
/// sprites (5 full + 5 empty) laid out at OAM tile bytes 88..124 (stride 4).
/// After splitting the value, the filled and partial loops decrement OAM tiles
/// (two -1 steps per block) to draw the gauge. Unlike the shared helper this
/// variant decrements by 1 twice per step rather than by 2.
pub fn build_object_health_meter_standard_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.object_health(0);
    if (health >= 109) {
        health = 109; // 109 = max meter value
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 0; // OAM meter slot 0
    let full_tile: i32 = 101; // tile $65 = full block
    let empty_tile: i32 = 107; // tile $6B = empty block
    // Lay out 5 full + 5 empty meter sprites (OAM tile bytes 88..124, stride 4).
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
    // Filled-block loop: start 24 bytes past the slot base (6 sprites in).
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

    // Partial-block loop: start 44 bytes past slot base (the partial column).
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
        sprite_slot = ((sprite_slot + 4) as u8 as i32); // advance one sprite (4 OAM bytes)
    }
    r.value = (full_tile as u8);
    r.index = (sprite_slot as u8);
    r.offset = (partial_blocks as u8);
}

/// Builds the player health meter sprite strip at the second OAM meter
/// slot.
///
/// Clamps player health to 109 (max meter), selects OAM meter slot $80=128
/// (the second meter region), and builds the strip via
/// [`build_health_meter_sprites`] with full tile $65=101 and empty tile
/// $6B=107.
pub fn build_player_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut health: i32 = engine.state.player_health as i32;
    if (health >= 109) {
        health = 109; // 109 = max meter value
    }
    engine.state.scratch0 = (health as u8);
    engine.state.scratch1 = 128; // OAM meter slot $80 (second meter region)
    r.index = 101; // full meter tile $65
    r.offset = 107; // empty meter tile $6B
    build_health_meter_sprites(engine, r);
}

/// Builds a ten-sprite two-row health meter. `0x09` selects the OAM slot,
/// `r.index` is the full tile, `r.offset` is the empty tile, and `0x08`
/// contains the value.
///
/// Shared meter builder for sprite-based health gauges. Lays out 5 full + 5
/// empty meter sprites (OAM tile bytes 88..124, stride 4, biased by the slot in
/// `scratch1`), splits the value into full/partial blocks, then decrements OAM
/// tiles by 2 per step to draw filled blocks (starting +24 into the slot) and
/// the partial block (starting +44). Decrementing by 2 advances two tiles in
/// the gauge artwork per step.
pub fn build_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // Fill the 5 full-block sprites with the full tile.
    let sprite_slot: i32 = (engine.state.scratch1 as i32);
    let full_tile: i32 = (r.index as u8 as i32);
    engine.state.set_oam_tile(88 + sprite_slot, full_tile);
    engine.state.set_oam_tile(92 + sprite_slot, full_tile);
    engine.state.set_oam_tile(96 + sprite_slot, full_tile);
    engine.state.set_oam_tile(100 + sprite_slot, full_tile);
    engine.state.set_oam_tile(104 + sprite_slot, full_tile);
    {
        // Fill the 5 empty-block sprites with the empty tile.
        let empty_tile: i32 = (r.offset as u8 as i32);
        engine.state.set_oam_tile(108 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(112 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(116 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(120 + sprite_slot, empty_tile);
        engine.state.set_oam_tile(124 + sprite_slot, empty_tile);
    }
    split_meter_value(engine, r);
    {
        // Filled blocks: decrement gauge tiles by 2 per step, starting +24.
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
            sprite_slot = ((sprite_slot + 4) as u8 as i32); // next sprite (4 OAM bytes)
        }
    }
    {
        // Partial block: same decrement-by-2 scheme, starting +44.
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
            sprite_slot = ((sprite_slot + 4) as u8 as i32); // next sprite (4 OAM bytes)
        }
    }
}

/// Splits the value in `0x08` into full 10-point meter blocks (`r.offset`)
/// and a one-based partial block (`0x08`/`r.value`).
///
/// Implements the 6502 repeated-subtract divide-by-10: subtracts 10 (with
/// borrow modelled via `carry`) until the value goes negative, counting full
/// blocks. The leftover is corrected back to a positive one-based partial
/// block by adding 11+carry. Returns full-block count in `r.offset` and the
/// partial remainder in `scratch0`/`r.value`.
pub fn split_meter_value(engine: &mut Engine, r: &mut RoutineContext) {
    let mut remainder: i32 = (engine.state.scratch0 as i32);
    let mut full_blocks: i32 = 0;
    let mut carry: i32 = 1; // start with borrow clear (6502 SEC before SBC)
    loop {
        full_blocks = ((full_blocks + 1) as u8 as i32);
        // SBC #10: subtract 10 and the inverted borrow (1-carry).
        let trial: i32 = (remainder) - 10 - (1 - carry);
        remainder = (trial as u8 as i32);
        carry = ((if (trial >= 0) { 1 } else { 0 }) as u8 as i32);
        if ((carry) == 0) {
            break; // borrow out → last subtraction underflowed
        }
    }
    // Restore the last over-subtracted block: +10 +1 (one-based) + carry.
    remainder = ((remainder + 11 + carry) as u8 as i32);
    engine.state.scratch0 = (remainder as u8);
    r.value = (remainder as u8);
    r.offset = (full_blocks as u8);
}

/// Waits for release, then press, then release again, returning the pressed
/// button byte in `r.value` and `0x20`.
///
/// Full debounce cycle used by menus/prompts: blocks until all buttons are
/// released, then until a press occurs (capturing the pressed bits), then until
/// release again so a single press registers exactly once. The captured byte is
/// returned in `r.value` and stored in `buttons` (`0x20`).
pub fn read_debounced_buttons(engine: &mut Engine, r: &mut RoutineContext) {
    wait_for_buttons_released(engine, r);
    wait_for_button_press(engine, r);
    {
        // Preserve the pressed bits across the trailing wait-for-release.
        let pressed_buttons: i32 = (r.value as u8 as i32);
        wait_for_buttons_released(engine, r);
        r.value = (pressed_buttons as u8);
        engine.state.buttons = (pressed_buttons as u8);
    }
}

/// Clears the deferred VRAM job selector at `0x28`.
///
/// Resets `nmi_vram_req` so no stale deferred VRAM job is picked up by the NMI
/// handler before a fresh job is queued. `r` is unused.
pub fn clear_pending_vram_job(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.nmi_vram_req = 0;
}

/// Builds player movement deltas from current directional input and speed
/// `r.offset`, storing them in `0x49..0x4B`.
///
/// The low nibble of `buttons` (the direction bits) indexes per-direction delta
/// tables (X at `MOVE_DELTA_X_TABLE`, Y at `MOVE_DELTA_Y_TABLE`); the `<<1`
/// reflects a 2-byte stride per direction entry. The signed per-step delta is
/// accumulated `speed` times. The horizontal result is split into a sub-tile
/// fraction (low nibble) and a whole-tile velocity (high nibble) with sign
/// extension ($F0 fill for negatives). A speed of 0 zeroes all three outputs.
pub fn build_input_movement_delta(engine: &mut Engine, r: &mut RoutineContext) {
    let speed: i32 = (r.offset as u8 as i32);
    engine.state.scratch1 = (speed as u8);
    if (speed == 0) {
        // No movement this frame: clear all delta outputs.
        engine.state.horizontal_subtile_delta = 0;
        engine.state.player_x_velocity = 0;
        engine.state.vertical_delta = 0;
        return;
    }
    // Direction bits (low nibble) * 2 = delta-table index (2 bytes per dir).
    let direction_index: i32 =
        (((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    // Accumulate the X delta once per speed step.
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
    // Low nibble = sub-tile fraction.
    engine.state.horizontal_subtile_delta = ((horizontal_delta & crate::bits::LOW_NIBBLE) as u8);
    // Sign-extend: negative (bit 7) → high nibble fill $F0=240, else 0.
    let sign_fill: i32 = (if ((horizontal_delta & crate::bits::BIT7) != 0) {
        240
    } else {
        0
    });
    engine.state.scratch0 = (sign_fill as u8);
    // Whole-tile X velocity = (high nibble) with sign fill.
    engine.state.player_x_velocity =
        ((((horizontal_delta & crate::bits::HIGH_NIBBLE) >> 4) | sign_fill) as u8);
    // Accumulate the Y delta once per speed step.
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
///
/// Object-velocity twin of [`build_input_movement_delta`]: the direction comes
/// from `r.value`'s low nibble instead of the controller, and the results go to
/// the object velocity fields (`obj_x_vel_lo/hi`, `obj_y_vel`). Same delta
/// tables, accumulation, nibble split, and sign extension. Speed 0 zeroes all.
pub fn build_direction_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let speed: i32 = (r.offset as u8 as i32);
    engine.state.scratch1 = (speed as u8);
    if (speed == 0) {
        // No motion: clear object velocity outputs.
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_y_vel = 0;
        return;
    }
    // Direction bits (low nibble of r.value) * 2 = delta-table index.
    let direction_index: i32 = (((r.value & ((crate::bits::LOW_NIBBLE) as u8)) << 1) as u8 as i32);
    // Accumulate the X delta once per speed step.
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
    // Low nibble = sub-tile fraction of horizontal velocity.
    engine.state.obj_x_vel_lo = ((horizontal_delta & crate::bits::LOW_NIBBLE) as u8);
    // Sign-extend: negative (bit 7) → high nibble fill $F0=240, else 0.
    let sign_fill: i32 = (if ((horizontal_delta & crate::bits::BIT7) != 0) {
        240
    } else {
        0
    });
    engine.state.scratch0 = (sign_fill as u8);
    // Whole-tile X velocity = (high nibble) with sign fill.
    engine.state.obj_x_vel_hi =
        ((((horizontal_delta & crate::bits::HIGH_NIBBLE) >> 4) | sign_fill) as u8);
    // Accumulate the Y delta once per speed step.
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
///
/// AABB hit test: requires both vertical and horizontal overlap. Clears
/// `overlap_flag` first; tests Y, then X (short-circuiting if either misses,
/// leaving carry=0). On full overlap sets `overlap_flag`=1 and carry=1.
pub fn check_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.overlap_flag = 0;
    // Vertical test first; bail (no overlap) if it misses.
    check_player_y_overlap(engine, r);
    if (r.carry == 0) {
        return;
    }
    // Then horizontal test; bail if it misses.
    check_player_x_overlap(engine, r);
    if (r.carry == 0) {
        return;
    }
    // Both axes overlap → collision.
    engine.state.overlap_flag = 1;
    r.carry = 1;
}

/// Checks horizontal player overlap using projected tile/subtile position
/// in `0x0E/0x0F`.
///
/// Compares the object's projected X (tile `indirect_ptr_hi`, fine
/// `indirect_ptr_lo`) against the player's X. The tile delta is an unsigned
/// (wrapping) difference, so 1 means one tile right and 255 (-1) one tile left.
/// Within +-1 tile, the fine-position sign decides overlap; the entry leaves
/// carry=0 (no overlap) unless a branch sets carry=1.
pub fn check_player_x_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    // Wrapping tile-column difference (object - player).
    let tile_delta: i32 =
        ((engine.state.indirect_ptr_hi - engine.state.player_x_tile) as u8 as i32);
    if (tile_delta == 0) {
        r.carry = 1; // same tile column: overlap ($CE95 BEQ $CEB4 SEC)
        return;
    }
    if (tile_delta < 2) {
        // One tile to the right: overlap iff object's fine X is left of player.
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
        r.carry = 0; // more than one tile away: no overlap ($CE9D BCC $CEB2 CLC)
        return;
    }
    {
        // tile_delta == 255 (-1): one tile to the left.
        let subtile_delta: i32 =
            ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
        if (subtile_delta == 0) {
            r.carry = 0; // aligned: no overlap ($CEA4 BEQ $CEB2 CLC)
            return;
        }
        if ((subtile_delta & crate::bits::BIT7) != 0) {
            r.carry = 0; // object's fine X still left of player: no overlap ($CEA6 BMI $CEB2 CLC)
            return;
        }
        r.carry = 1; // overlapping from the left
    }
}

/// Checks vertical player overlap using projected y position in `0x0A`.
///
/// Wrapping vertical difference (`scratch2` - player Y): overlap when within
/// 16 pixels below (`< 16`) or within 16 above (`>= 241`, i.e. -15..-1); the
/// mid range `16..240` is a miss. Carry set on overlap.
pub fn check_player_y_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let y_delta: i32 = ((engine.state.scratch2 - engine.state.player_y) as u8 as i32);
    if (y_delta < 16) {
        r.carry = 1; // 0..15 below player: overlap
    } else if (y_delta < 241) {
        r.carry = 0; // 16..240: outside hitbox
    } else {
        r.carry = 1; // 241..255 (= -15..-1): overlap above player
    }
}

/// Wider player-overlap test used by falling/large movement probes. Carry
/// and `0xEA` are set on overlap.
///
/// Same shape as [`check_player_overlap`] but with a taller vertical hitbox
/// (the miss band is `16..224` instead of `16..240`, so it reaches ~16px
/// further down) and a +-2-tile horizontal window. Implemented as a small
/// state machine: state 0 runs the tests, falling through to state 1 (set
/// overlap_flag + carry) when an overlap is found, and returning early with
/// carry=0 otherwise. Each `0x0E/0x0F` access is the object's projected X.
pub fn check_player_overlap_wide(engine: &mut Engine, r: &mut RoutineContext) {
    let mut dy: i32 = 0;
    let mut dx: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.overlap_flag = 0;
                // Vertical: miss if in the wide mid band 16..224.
                dy = ((engine.state.scratch2 - engine.state.player_y) as u8 as i32);
                if ((dy >= 16) && (dy < 225)) {
                    r.carry = 0;
                    return;
                }
                // Horizontal tile difference (wrapping; 0 / 255 = same / -1 tile).
                dx = ((engine.state.indirect_ptr_hi - engine.state.player_x_tile) as u8 as i32);
                if (dx == 0) {
                    // Same tile column → overlap.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (dx == 255) {
                    // One tile left (-1) → overlap.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (dx < 2) {
                    // One tile right: overlap only if object's fine X is left of player.
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
                    r.carry = 0; // 2..253 tiles away: no overlap ($CEE9 BCC $CF00 CLC)
                    return;
                }
                {
                    // dx == 254 (-2): two tiles left; fine X decides.
                    let subtile_delta: i32 =
                        ((engine.state.indirect_ptr_lo - engine.state.player_x_fine) as u8 as i32);
                    if (subtile_delta == 0) {
                        r.carry = 0; // aligned: no overlap ($CEF1 BEQ $CF00 CLC)
                        return;
                    }
                    if ((subtile_delta & crate::bits::BIT7) != 0) {
                        r.carry = 0; // fine X left of player: no overlap ($CEF3 BMI $CF00 CLC)
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
                // Overlap found: set flag and carry.
                engine.state.overlap_flag = 1;
                r.carry = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Checks a projected position against the general playfield bounds, given a
/// projected Y in `scratch2` (0x0A) and a projected X tile/fine in
/// `indirect_ptr_hi`/`indirect_ptr_lo` (0x0F/0x0E). Sets carry (out of bounds)
/// when Y is below the floor or X has crossed the right room edge.
pub fn check_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    // Y at or past the bottom of the playfield (192 px) is out of bounds.
    if (engine.state.scratch2 >= 192) {
        r.carry = 1;
    // X tile < 63 is comfortably inside the 64-tile-wide room: in bounds.
    } else if (engine.state.indirect_ptr_hi < 63) {
        r.carry = 0;
    // At tile 63 with no fine offset we are still on the last column: in bounds.
    } else if (engine.state.indirect_ptr_lo == 0) {
        r.carry = 0;
    // Tile 63 with a nonzero fine offset has crossed the right edge.
    } else {
        r.carry = 1;
    }
}

/// Checks a projected actor position against the tighter actor playfield
/// bounds (same scratch inputs as `check_position_out_of_bounds`). Actors use a
/// lower floor (176 px instead of 192) so they stop short of the player's
/// reachable bottom edge. Sets carry when out of bounds.
pub fn check_actor_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
    // Actors are out of bounds 16 px higher than the player (176 px floor).
    if (engine.state.scratch2 >= 176) {
        r.carry = 1;
        return;
    }
    // X tile < 63: safely inside the room.
    if (engine.state.indirect_ptr_hi < 63) {
        r.carry = 0;
        return;
    }
    // Tile 63 with no fine offset: still on the last column, in bounds.
    if (engine.state.indirect_ptr_lo == 0) {
        r.carry = 0;
        return;
    }
    // Tile 63 with a fine offset: crossed the right edge.
    r.carry = 1;
}

/// Uploads every inventory item count to the item/status screen, iterating the
/// 16 inventory slots from last to first.
pub fn upload_inventory_count_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    {
        // Walk the 16 inventory slots (15..0) in descending order.
        x = 15;
        while (x >= 0) {
            r.index = (x as u8);
            // Pass the slot's stored count in Y (r.offset).
            r.offset = ((engine.state.inventory_item(x)) as u8);
            upload_inventory_item_count_tiles(engine, r);
            // Subroutine may clobber X; restore the loop index.
            r.index = (x as u8);
            {
                x -= 1;
                x
            };
        }
    }
    r.index = 255; // X = 0xFF on exit (loop ran past 0).
}

/// Uploads one inventory item's two-digit count to the status screen and
/// applies the active family-member availability palette adjustment: items the
/// current character cannot use are drawn with a dimmed/alternate tile set.
/// X (r.index) is the item slot 0..15, Y (r.offset) is the count to draw.
pub fn upload_inventory_item_count_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (r.index as u8 as i32);
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut s: i32 = 0;
    // Compute the destination nametable address from the slot index: the low 3
    // bits choose the column (4 tiles apart, << 2), and bit 3 chooses the row
    // half (<< 4 adds 0x80 = next row pair).
    lo = (((x & crate::bits::LOW_3_BITS) << 2) as u8 as i32);
    lo = ((((x & crate::bits::BIT3) << 4) | lo) as u8 as i32);
    hi = 0;
    s = ((194 + lo) as u16 as i32); // 0xC2 base offset within the nametable page.
    engine.state.vram_addr_lo = (s as u8);
    engine.state.vram_addr_hi = ((32 + hi + (s >> 8)) as u8); // 0x20xx nametable.
    // Render the count into the two digit-tile scratch bytes.
    r.value = r.offset;
    build_decimal_digit_tiles(engine, r);
    {
        // Look up this item's usability bit for the active character, identical
        // to load_family_item_permission_bits; the carry it leaves is unused
        // here (the real permission read happens below).
        let mut in_: i32 = x;
        let mut dx: i32 = (((engine.state.character_index as i32) << 1) as u8 as i32);
        let mut yy: i32 = 0;
        let mut carry: i32 = 0;
        let mut v: i32 = 0;
        // Items 8..15 live in the second permission byte of the character pair.
        if (in_ >= 8) {
            {
                let __old = dx;
                dx += 1;
                __old
            };
        }
        // Shift count = (slot & 7) + 1 so the wanted bit ends up in carry.
        yy = (((in_ & crate::bits::LOW_3_BITS) + 1) as u8 as i32);
        v = engine
            .state
            .byte((MOVEMENT_PATTERN_TABLE + dx) as u16 as i32);
        carry = 0;
        loop {
            carry = ((v >> 7) as u8 as i32); // capture top bit before shifting.
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
    // Authoritative permission read; carry clear means this character cannot use
    // the item, so dim it by moving the digit tiles back 64 (one CHR sub-bank).
    r.value = (x as u8);
    load_family_item_permission_bits(engine, r);
    if ((r.carry) == 0) {
        engine.state.vram_addr2_lo = engine.state.vram_addr2_lo - 64;
        engine.state.vram_addr2_hi = engine.state.vram_addr2_hi - 64;
    }
    // Queue PPU job type 6 (two-tile horizontal write) and wait for it.
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the three equipped-item stat numbers on the status screen: the
/// effective projectile damage, jump duration, and projectile lifetime for the
/// currently selected loadout. Each is drawn as two decimal digit tiles.
pub fn upload_equipped_item_stat_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Projectile damage at nametable 0x21DE.
    engine.state.vram_addr_lo = 222;
    engine.state.vram_addr_hi = 33;
    load_effective_projectile_damage(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6; // PPU job type 6: two-tile write.
    queue_ppu_job_and_wait(engine, r);
    // Jump duration at nametable 0x221E.
    engine.state.vram_addr_lo = 30;
    engine.state.vram_addr_hi = 34;
    load_effective_jump_duration(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
    // Projectile lifetime at nametable 0x225E.
    engine.state.vram_addr_lo = 94;
    engine.state.vram_addr_hi = 34;
    load_effective_projectile_lifetime(engine, r);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Uploads the two visible shop item prices (stored in temp_save slots 1 and 3)
/// as decimal digit tiles. When the shop is scrolled into the right nametable
/// page, the destination address is bumped into that page first.
pub fn upload_shop_price_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut c: i32 = 0;
    // First price destination: nametable 0x2247.
    engine.state.vram_addr_lo = 71;
    engine.state.vram_addr_hi = 34;
    // If scrolled past the page boundary (bit 4 of scroll tile X), move the
    // write into the second nametable page (+0x400).
    if ((engine.state.scroll_tile_x & ((crate::bits::BIT4) as u8)) != 0) {
        let mut s: i32 = ((0 + engine.state.vram_addr_lo as i32) as u16 as i32);
        engine.state.vram_addr_lo = (s as u8);
        engine.state.vram_addr_hi = 4 + engine.state.vram_addr_hi + ((s >> 8) as u8);
    }
    // First price digits.
    r.value = ((engine.state.temp_save(1)) as u8);
    build_decimal_digit_tiles(engine, r);
    r.value = 6; // two-tile PPU write.
    queue_ppu_job_and_wait(engine, r);
    // Second price destination is +14 tiles to the right, carrying into hi.
    lo = (engine.state.vram_addr_lo as i32);
    c = (((14 + lo) >> 8) as u8 as i32); // carry out of the low byte.
    engine.state.vram_addr_lo = ((14 + lo) as u8);
    hi = (engine.state.vram_addr_hi as i32);
    engine.state.vram_addr_hi = ((0 + hi + c) as u8);
    // Second price digits.
    r.value = ((engine.state.temp_save(3)) as u8);
    build_decimal_digit_tiles(engine, r);
    r.value = 6;
    queue_ppu_job_and_wait(engine, r);
}

/// Converts the byte in A (r.value, 0..99) into two decimal digit tile ids
/// stored in `vram_addr2_lo`/`vram_addr2_hi` (0x18/0x19), low digit then high
/// digit. Digit tiles start at 0xD0 ('0'); a leading zero in the tens place is
/// drawn as a blank tile 0xC0 instead.
pub fn build_decimal_digit_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32);
    let mut hi: i32 = 208; // tens-digit tile, starts at '0' = 0xD0.
    // Repeated subtraction divides by 10: hi counts tens, a keeps the remainder.
    while (a >= 10) {
        a = ((a - 10) as u8 as i32);
        {
            hi += 1;
            hi
        };
    }
    a = ((a + 208) as u8 as i32); // ones digit tile = 0xD0 + remainder.
    engine.state.vram_addr2_lo = (a as u8);
    // Suppress a leading zero: tens still 0xD0 -> use blank tile 0xC0.
    if (hi == 208) {
        hi = 192;
    }
    engine.state.vram_addr2_hi = (hi as u8);
}

/// Looks up whether the active character may use item id `r.value`. Each
/// character has a two-byte permission bitmap (one byte for items 0..7, one for
/// 8..15) in the ROM table at MOVEMENT_PATTERN_TABLE, indexed by
/// `character_index * 2`. The wanted bit is shifted left into carry; carry set
/// means the item is usable. The residual shifted byte is returned in A.
pub fn load_family_item_permission_bits(engine: &mut Engine, r: &mut RoutineContext) {
    let mut in_: i32 = (r.value as u8 as i32);
    // Base index = character * 2 (two permission bytes per character).
    let mut x: i32 = (((engine.state.character_index as i32) << 1) as u8 as i32);
    // Items 8..15 are described by the second byte of the pair.
    if (in_ >= 8) {
        {
            let __old = x;
            x += 1;
            __old
        };
    }
    // Shift count = (item & 7) + 1 lands the target bit in carry.
    let mut y: i32 = (((in_ & crate::bits::LOW_3_BITS) + 1) as u8 as i32);
    let mut a: i32 = engine
        .state
        .byte((MOVEMENT_PATTERN_TABLE + x) as u16 as i32);
    let mut carry: i32 = 0;
    loop {
        carry = ((a >> 7) as u8 as i32); // top bit becomes the shift-out carry.
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

/// Starts the song id in A (r.value) as the current track, but only if it is
/// not already playing (compared against `song` at 0x8E). Avoids restarting a
/// song that is already active.
pub fn switch_song_if_needed(engine: &mut Engine, r: &mut RoutineContext) {
    // Already the active song: do nothing.
    if (r.value == (engine.state.song as u8)) {
        return;
    }
    // Record the new song and (re)initialize the audio engine for it.
    engine.state.song = (r.value as u8);
    song_init(engine, r);
}

/// Computes the effective jump duration. When the selected item is the
/// jump-boost item (id 6) and the player has magic, the base jump strength is
/// multiplied by 1.25 (base + base/4); otherwise the base value is returned.
/// A (r.value) gets the result; carry is clear when the boost applies.
pub fn load_effective_jump_duration(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_item_slot);
    r.index = (selected_item_slot as u8);
    // Item 6 (jump boost) active and magic available: +25% jump strength.
    if (selected_item == 6) && (engine.state.player_magic != 0) {
        let base_jump_duration: i32 = (engine.state.jump_strength as i32);
        r.value = (((base_jump_duration >> 2) + base_jump_duration) as u8); // base * 1.25
        r.carry = 0;
    } else {
        r.value = (engine.state.jump_strength as u8);
        r.carry = 1;
    }
}

/// Computes the effective projectile damage. When the selected item is the
/// power item (id 8) and the player has magic, base damage is multiplied by 4
/// (<< 2); otherwise the base value is returned. A (r.value) gets the result;
/// carry is clear when the boost applies.
pub fn load_effective_projectile_damage(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_item_slot);
    // Item 8 (power) active and magic available: 4x damage.
    if (selected_item == 8) && (engine.state.player_magic != 0) {
        r.value = (((engine.state.projectile_damage as i32) << 2) as u8); // base * 4
        r.carry = 0;
    } else {
        r.value = (engine.state.projectile_damage as u8);
        r.carry = 1;
    }
}

/// Computes the effective projectile lifetime (how long/far a shot travels).
/// When the selected item is the range item (id 9) and the player has magic,
/// base lifetime is doubled (<< 1); otherwise the base value is returned. A
/// (r.value) gets the result; carry is clear when the boost applies.
pub fn load_effective_projectile_lifetime(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
    r.index = (selected_item_slot as u8);
    // Item 9 (range) active and magic available: 2x lifetime.
    if (engine.state.item_slot(selected_item_slot) == 9) && (engine.state.player_magic != 0) {
        r.value = (((engine.state.projectile_lifetime as i32) << 1) as u8); // base * 2
        r.carry = 0;
        return;
    }
    r.value = (engine.state.projectile_lifetime as u8);
    r.carry = 1;
}

/// Hides the gameplay-object half of OAM by moving those sprites off-screen,
/// leaving the HUD/player sprites in the lower half (0x00..0x7F) untouched. The
/// shadow OAM is 256 bytes / 64 sprites; this writes Y=239 to sprites 32..63.
pub fn clear_gameplay_object_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut oam_offset: i32 = 128; // byte 0x80 = start of the object sprite half.
    loop {
        engine.state.set_oam_y(oam_offset, 239); // Y=239 places sprite below screen.
        oam_offset = ((oam_offset + 4) as u8 as i32); // 4 bytes per OAM entry.
        // Wraps to 0 after byte 252, ending the pass over the upper half.
        if (oam_offset == 0) {
            break;
        }
    }
    r.index = (oam_offset as u8);
    r.value = 239;
}

/// Clears all 16 object slots to inactive (state 0) and primes each slot's
/// spawn timer, then resets the actor scheduler phase. Called when loading a new
/// room so no stale actors persist. Object records are 16 bytes apart.
pub fn reset_room_object_slots(engine: &mut Engine, r: &mut RoutineContext) {
    let mut slot_offset: i32 = 0;
    let mut slots_remaining: i32 = 16; // 16 object slots.
    loop {
        engine.state.set_object_state(slot_offset, 0); // inactive.
        engine.state.set_object_timer(slot_offset, 2); // initial spawn delay.
        slot_offset = ((slot_offset + 16) as u8 as i32); // 16-byte record stride.
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
    r.index = (slot_offset as u8); // 0 after 16 * 16 wraps a byte.
    r.offset = 0;
}

/// Saves mutable progress and inventory state into a backup buffer before the
/// status/inventory screens temporarily repurpose that RAM range, so it can be
/// restored on exit by `restore_inventory_state_snapshot`.
pub fn snapshot_inventory_state(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy the 8 progress/save-payload bytes into the progress backup.
    for progress_offset in (0..8).rev() {
        engine
            .state
            .set_save_progress(progress_offset, engine.state.save_payload(progress_offset));
    }
    // Copy the 16 inventory item counts into the inventory backup.
    for inventory_offset in (0..16).rev() {
        engine.state.set_save_inventory(
            inventory_offset,
            engine.state.inventory_item(inventory_offset),
        );
    }
    // Backup slots 16/17 hold keys and coins respectively.
    engine
        .state
        .set_save_inventory(17, (engine.state.coins as i32));
    engine
        .state
        .set_save_inventory(16, (engine.state.keys as i32));
    r.index = 255;
}

/// Restores the progress, inventory counts, coins, and keys that
/// `snapshot_inventory_state` backed up, reversing that copy exactly.
pub fn restore_inventory_state_snapshot(engine: &mut Engine, r: &mut RoutineContext) {
    // Restore the 8 progress/save-payload bytes.
    for progress_offset in (0..8).rev() {
        engine
            .state
            .set_save_payload(progress_offset, engine.state.save_progress(progress_offset));
    }
    // Restore the 16 inventory item counts.
    for inventory_offset in (0..16).rev() {
        engine.state.set_inventory_item(
            inventory_offset,
            engine.state.save_inventory(inventory_offset),
        );
    }
    // Backup slots 16/17 -> keys and coins.
    engine.state.coins = ((engine.state.save_inventory(17)) as u8);
    engine.state.keys = ((engine.state.save_inventory(16)) as u8);
    r.index = 255;
}

/// Converts the 32-byte item-name source buffer into a VRAM staging buffer
/// (offset +0x40 / 64) and uploads it as two nametable rows. The source is read
/// backwards in groups of 4 characters (one item name per column), each
/// character forced to its highlighted tile (bit 7 set), with one extra spacing
/// byte skipped between groups; characters past the printable range become the
/// blank tile 0x7F.
pub fn upload_inventory_item_list(engine: &mut Engine, r: &mut RoutineContext) {
    let mut source_offset: i32 = 31; // last byte of the 32-byte source buffer.
    let mut staging_offset: i32 = 38; // last slot of the staging window.
    loop {
        {
            // Copy 4 characters of one item name into the staging buffer.
            let mut chars_in_column: i32 = 0;
            while (chars_in_column < 4) {
                let mut tile: i32 = ((engine.state.password_nibbles_a(source_offset)
                    | crate::bits::BIT7) as u8 as i32); // set bit 7 = highlighted glyph.
                if (tile >= 160) {
                    tile = 127; // out-of-range glyph -> blank tile 0x7F.
                }
                engine
                    .state
                    .set_password_nibbles_a(64 + (staging_offset & crate::bits::BYTE_MASK), tile); // +0x40 staging window.
                staging_offset = (staging_offset - 1) & crate::bits::BYTE_MASK;
                source_offset = (source_offset - 1) & crate::bits::BYTE_MASK;
                {
                    chars_in_column += 1;
                    chars_in_column
                };
            }
        }
        // Skip one staging slot between columns; stop when it underflows past 0
        // (bit 7 set), i.e. the staging buffer is filled.
        staging_offset = (staging_offset - 1) & crate::bits::BYTE_MASK;
        if !((staging_offset & crate::bits::BIT7) == 0) {
            break;
        }
    }
    // Upload the first row: nametable 0x24E6 from staging source 0x0362.
    engine.state.inventory_upload_col = 19;
    engine.state.inventory_upload_row = 0;
    engine.state.vram_addr_lo = 230;
    engine.state.vram_addr_hi = 36;
    engine.state.vram_addr2_lo = 98;
    engine.state.vram_addr2_hi = 3;
    r.value = 5; // PPU job type 5: row upload.
    queue_ppu_job_and_wait(engine, r);
    // Upload the second row: nametable 0x2506 from staging source 0x0376.
    engine.state.vram_addr_lo = 6;
    engine.state.vram_addr_hi = 37;
    engine.state.vram_addr2_lo = 118;
    engine.state.vram_addr2_hi = 3;
    r.value = 5;
    queue_ppu_job_and_wait(engine, r);
}

/// Fills the 32-byte item-name source buffer with the blank tile id (0x7F),
/// clearing any previous item name before a new one is written.
pub fn clear_inventory_item_list_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    // Blank all 32 source bytes with tile 0x7F.
    for item_list_offset in (0..32).rev() {
        engine.state.set_password_nibbles_a(item_list_offset, 127);
    }
    r.value = 127;
    r.index = 255;
}

/// Starts or continues the player's jump arc. State 0 begins a jump (only when
/// not already mid-action via `collision_flag` at 0x22); the jump-boost item
/// (id 6) spends a magic point to extend the timer by 25%. State 1 advances one
/// frame of the arc, deriving upward speed from the timer and trying to move,
/// falling back to vertical-only and tile-boundary nudges on collision. States
/// 2/3 commit the new position (state 3 also ends the jump on a hard stop), and
/// state 4 refreshes the pose and walk animation.
/// `jump_timer` (0x4F) is the active arc countdown; `collision_flag` (0x22)
/// guards against a held button restarting the jump.
pub fn tick_player_jump_action(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Already mid-jump: skip straight to advancing the arc.
                if (engine.state.jump_timer != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Action already in progress this contact: do nothing.
                if (engine.state.collision_flag != 0) {
                    return;
                }
                // Begin a fresh jump: set the "jumping" prompt and seed the timer.
                engine.state.prompt_state = 27;
                engine.state.jump_timer = engine.state.jump_strength;
                {
                    // Jump-boost item (id 6) extends the arc by 25% for 1 magic.
                    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
                    if (engine.state.item_slot(selected_slot) == 6) {
                        consume_magic_point(engine, r);
                        if ((r.carry) == 0) {
                            let jump_timer: i32 = (engine.state.jump_timer as i32);
                            engine.state.jump_timer = (((jump_timer >> 2) + jump_timer) as u8); // *1.25
                        }
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Mark a non-local handoff and lock the action for this contact.
                engine.lotw_nonlocal_handoff = 1;
                engine.state.collision_flag = 1;
                {
                    // Decrement the timer; upward speed = timer/4, negated into a
                    // signed delta (two's complement: ^0xFF + 1).
                    let jump_timer: i32 = (engine.state.jump_timer as i32);
                    engine.state.jump_timer = ((jump_timer - 1) as u8);
                    let upward_speed: i32 = ((jump_timer >> 2) as u8 as i32);
                    engine.state.vertical_delta =
                        (((upward_speed ^ crate::bits::BYTE_MASK) + 1) as u8);
                }
                // Try the full diagonal move; carry set means it succeeded.
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Horizontal blocked: retry vertical-only by zeroing the X
                // sub-tile delta ($8C95 LDA #$00; STA $49). The 6502 leaves the
                // X velocity ($4A) untouched here.
                engine.state.horizontal_subtile_delta = 0;
                try_move_player_with_collision(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Both moves blocked: end the jump ($8C9E JMP $ACAF -> stop). The
                // 6502 does NOT restore the timer or nudge to the tile grid here
                // (that is the grounded-walk path); restoring jump_timer kept the
                // jump from ever ending, hanging the player against a wall.
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Commit the projected position back into the player coords.
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                {
                    // Projected Y; treat the 239..255 underflow band as 0 (top).
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
                // Hard stop: cancel the jump and any in-progress fall.
                engine.state.jump_timer = 0;
                engine.state.fall_frames = 0;
                update_player_terrain_contact(engine, r);
                state = 4;
                continue 'dispatch;
            }
            4 => {
                // Refresh the visible pose and walk-cycle animation.
                update_player_pose_from_motion(engine, r);
                tick_player_walk_animation(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Applies the currently selected passive/consumable item's effect. Items 0/1
/// are timed magic-draining effects (light/spell-like): they start a 2-tick
/// effect timer in the inventory cooldown slots (38+id) when a magic point can
/// be paid, otherwise warn via the continue timer. Item 11 is a magic potion
/// that refills magic to cap and is consumed. Item 13 is the recall/warp scroll
/// that returns the player to the fixed safe room and is consumed.
pub fn tick_selected_item_effect(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    let selected_item: i32 = engine.state.item_slot(selected_slot);
    if (selected_item >= 2) {
        // Item 11: magic-refill potion. Only usable when magic is empty.
        if (selected_item == 11) {
            if (engine.state.player_magic != 0) {
                return;
            }
            engine.state.set_item_slot(selected_slot, 255); // consume the item.
            draw_status_item_sprites(engine, r);
            animate_magic_refill_to_cap(engine, r);
            return;
        }
        // Anything other than item 13 below has no effect here.
        if (selected_item != 13) {
            return;
        }
        // Item 13: recall scroll. Disallowed in the special area (map row >= 17);
        // there it just reselects the default item slot 3.
        if (engine.state.map_screen_y >= 17) {
            engine.state.selected_item_slot = 3;
            return;
        }
        // Consume the scroll and teleport to the fixed safe room coordinates.
        engine.state.set_item_slot(selected_slot, 255);
        draw_status_item_sprites(engine, r);
        engine.state.prompt_state = 18;
        engine.state.map_screen_y = 16; // safe-room map cell.
        engine.state.map_screen_x = 3;
        engine.state.scroll_tile_x = 18;
        engine.state.player_y = 176;
        engine.state.player_x_tile = 26;
        engine.state.player_x_fine = 0;
        engine.state.scroll_fine_x = 0;
        // Rebuild the room: fade out, reset actors, assemble/upload, redraw, fade in.
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
    // Items 0/1: timed effects. Skip if the effect cooldown slot is still set.
    if (engine.state.inventory_item(38 + selected_item) != 0) {
        return;
    }
    r.index = (selected_item as u8);
    consume_magic_point(engine, r);
    // Paid for: arm the cooldown slot (38+id) with 2 ticks.
    if (r.carry == 0) {
        engine.state.set_inventory_item(38 + selected_item, 2);
        return;
    }
    {
        // Could not pay (out of magic): warn unless the continue timer is
        // already inactive (0) or counting (high bit set).
        let continue_timer: i32 = (engine.state.continue_timer as i32);
        if ((continue_timer == 0) || ((continue_timer & crate::bits::BIT7) != 0)) {
            return;
        }
        engine.state.continue_timer = 253; // 0xFD: start the low-magic warning.
        engine.state.prompt_state = 26;
    }
}

/// Enters the destination encoded in the active door/link record. The record is
/// pointed to by `palette_src_ptr` (0x77/0x78); bytes +12..+15 hold the
/// destination map cell X/Y, the player's landing tile X, and landing Y. The
/// scroll is positioned so the player tile sits ~8 columns from the left edge,
/// clamped to the room width, then the room is fully rebuilt.
pub fn enter_room_link_destination(engine: &mut Engine, r: &mut RoutineContext) {
    let link_ptr: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    // +12/+13: destination map-cell coordinates.
    engine.state.map_screen_x = ((engine.state.byte((link_ptr + 12) as u16 as i32)) as u8);
    engine.state.map_screen_y = ((engine.state.byte((link_ptr + 13) as u16 as i32)) as u8);

    // +14: landing tile X; center the view ~8 tiles behind the player.
    let player_tile_x: i32 = engine.state.byte((link_ptr + 14) as u16 as i32);
    engine.state.player_x_tile = (player_tile_x as u8);
    let scroll_x: i32 = if (player_tile_x >= 8) {
        ((player_tile_x - 8) as u8 as i32)
    } else {
        0
    };
    // Clamp scroll to the room's max left-edge tile (48; 64-wide room, 16 visible).
    engine.state.scroll_tile_x = if (scroll_x >= 49) {
        48
    } else {
        (scroll_x as u8)
    };
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;

    // +15: landing Y.
    r.value = ((engine.state.byte((link_ptr + 15) as u16 as i32)) as u8);
    engine.state.player_y = (r.value as u8);
    // Full room rebuild: fade out, reset actors, assemble/upload, redraw, fade in.
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

/// After collecting a fragment item (id 14), warps to the fragment shrine area
/// (map row 17), choosing the column from how many fragments are now held
/// (`fragment_count - 1`), and fully rebuilds the room.
pub fn enter_fragment_pickup_room(engine: &mut Engine, r: &mut RoutineContext) {
    run_warp_transition_effect(engine, r);
    engine.state.map_screen_y = 17; // fragment shrine map row.
    // Column = fragment_count - 1, so each fragment lands in its own cell.
    r.index = ((engine.state.fragment_count - 1) as u8);
    engine.state.map_screen_x = (r.index as u8);
    engine.state.scroll_tile_x = 18;
    engine.state.player_y = 16;
    engine.state.player_x_tile = 26;
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;
    r.value = 0;
    // Full room rebuild.
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

/// Consumes the pending special-exit flag (set by the high-bit actor contact
/// path) and warps to the fixed safe room, switching CHR bank slot 4 to the
/// safe-room tileset first.
pub fn enter_pending_special_exit_room(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.pending_special_exit = 0; // clear the request.
    run_warp_transition_effect(engine, r);
    engine.state.set_chr_bank(4, 62); // safe-room CHR bank in slot 4.
    engine.state.map_screen_y = 16; // safe-room map cell.
    engine.state.map_screen_x = 3;
    engine.state.scroll_tile_x = 18;
    engine.state.player_y = 176;
    engine.state.player_x_tile = 26;
    engine.state.player_x_fine = 0;
    engine.state.scroll_fine_x = 0;
    r.value = 0;
    // Full room rebuild.
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

/// Raises the final-exit flag (the win/ending trigger) only when the selected
/// item is the final item (id 15) and the player stands at the exact room cell,
/// scroll position, and Y the original game requires for the ending sequence.
pub fn check_final_exit_trigger(engine: &mut Engine, r: &mut RoutineContext) {
    let selected_slot: i32 = (engine.state.selected_item_slot as i32);
    // All six conditions must hold: item 15 at map (1,5), scroll tile 16, no
    // fine scroll, player Y exactly 160.
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

/// Shared accelerating side-scroll + audio transition used before scripted room
/// warps. Clears sprites, redraws player/HUD, stages the current and next-screen
/// columns, then scrolls horizontally with an increasing per-frame speed
/// (1..31), toggling the nametable on each 256-pixel wrap. Ends by arming the
/// fade prompt and flashing the palette buffer.
pub fn run_warp_transition_effect(engine: &mut Engine, r: &mut RoutineContext) {
    let mut outer: i32 = 0;
    // Reset OAM (with the sprite-0 hit template) and redraw the player + HUD.
    clear_oam_with_sprite_zero_template(engine, r);
    engine.state.sprite_blink_timer = 0;
    draw_player_sprites(engine, r);
    draw_status_item_sprites(engine, r);
    // Clamp scroll so the +16 staging below stays within the room (max 32).
    if (engine.state.scroll_tile_x >= 33) {
        engine.state.scroll_tile_x = 32;
    }
    // Stage the current screen, then the screen 16 tiles to the right.
    upload_room_columns_from_bank9(engine, r);
    engine.state.scroll_tile_x = engine.state.scroll_tile_x + 16;
    upload_room_columns_from_bank9(engine, r);
    engine.state.scratch0 = 1; // current scroll speed (px/frame).
    loop {
        let mut x: i32 = 12; // frames spent at the current speed.
        loop {
            // Advance the pixel scroll by the current speed; wrap past 256
            // (bit 8) flips which nametable is shown.
            let mut sum: i32 =
                ((engine.state.scroll_pixel_x as i32 + engine.state.scratch0 as i32) as u16 as i32);
            engine.state.scroll_pixel_x = (sum as u8);
            if ((sum & crate::bits::BIT8) != 0) {
                engine.state.nametable_select =
                    engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
            }
            r.value = 255; // PPU job 0xFF: present one frame.
            queue_ppu_job_and_wait(engine, r);
            if ({
                x -= 1;
                x
            } == 0)
            {
                break;
            }
        }
        // Ramp up the scroll speed each block until it reaches 31.
        engine.state.scratch0 = engine.state.scratch0 + 1;
        outer = (engine.state.scratch0 as i32);
        if !(outer < 32) {
            break;
        }
    }
    // Arm the fade prompt and flash the palette over 8 steps.
    engine.state.prompt_state = 24;
    engine.state.prompt_argument = 255;
    r.index = 8;
    flash_palette_buffer(engine, r);
}

/// Full room rebuild with palette fade (used for hard transitions, e.g.
/// wrapping off the top/bottom of the map): fade out + reset audio, reset
/// actors, assemble and upload the room, redraw sprites, fade in, reset the
/// frame counter, and return success (carry set).
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

/// Lightweight vertical room transition (no fade): reset actors, assemble and
/// upload the new room, and refresh the palette buffer directly. Used when the
/// player simply steps to the room above/below.
fn scene_rebuild_vert(engine: &mut Engine, r: &mut RoutineContext) {
    reset_room_object_slots(engine, r);
    clear_gameplay_object_sprites(engine, r);
    scene_assemble(engine, r);
    upload_current_room_view(engine, r);
    upload_palette_buffer(engine, r);
    engine.state.frame_counter = 0;
    r.carry = 1;
}

/// Handles the player crossing a room edge once a move has been projected out
/// of the current room. Vertical exits (player Y above 16 or at/below 161) step
/// the map row and rebuild the room (full rebuild when wrapping the whole map at
/// row 0/16, lightweight vertical strip otherwise). Horizontal exits (player at
/// tile 0 or >= 62) step the map column, then play an animated side-scroll into
/// the adjacent screen. Returns carry set on a successful transition.
pub fn handle_player_room_transition(engine: &mut Engine, r: &mut RoutineContext) {
    let player_y: i32 = (engine.state.player_y as i32);
    // --- Exit through the top edge (Y wrapped below 16) ---
    if (player_y < 16) {
        // Only proceed if the top edge is an open exit.
        check_top_boundary_exit_clear(engine, r);
        if (r.carry == 0) {
            return;
        }
        // From the overworld top row (0): warp into the safe room (cell 3,16).
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
        // Row 16 (safe room) has no room above it.
        if (engine.state.map_screen_y == 16) {
            return;
        }
        // Normal step up one row; re-enter at the bottom (Y=176).
        engine.state.map_screen_y = engine.state.map_screen_y - 1;
        engine.state.player_y = 176;
        scene_rebuild_vert(engine, r);
        return;
    }
    // --- Exit through the bottom edge (Y at/past 161) ---
    if (player_y >= 161) {
        // From the safe room (row 16): warp back to the overworld top-left.
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
        // No room below the last overworld row (15).
        if (((engine.state.map_screen_y + 1) as u8 as i32) >= 16) {
            return;
        }
        // Normal step down one row; re-enter at the top (Y=0).
        engine.state.map_screen_y = engine.state.map_screen_y + 1;
        engine.state.player_y = 0;
        scene_rebuild_vert(engine, r);
        return;
    }
    // --- Horizontal exit (player at a left/right room edge) ---
    // The safe room (row 16) does not scroll horizontally.
    if (engine.state.map_screen_y == 16) {
        return;
    }
    update_player_terrain_contact(engine, r);
    engine.state.sprite_blink_timer = 0;
    // Keep only the pose's low 3 bits (clear flip/animation state) across the wipe.
    engine.state.player_pose = engine.state.player_pose & ((crate::bits::LOW_3_BITS) as u8);
    if (engine.state.player_x_tile == 0) {
        // Exiting left: refuse if there is no room to the left (column underflow
        // sets bit 7).
        if ((((engine.state.map_screen_x - 1) as u8 as i32) & crate::bits::BIT7) != 0) {
            return;
        }
        engine.state.map_screen_x = engine.state.map_screen_x - 1;
        engine.state.player_facing = 0; // face right (entering from the left edge).
        draw_player_sprites(engine, r);
        engine.state.scroll_tile_x = 48; // show the right end of the new room.
        engine.state.player_x_tile = 63; // place player at the far-right column.
        engine.state.player_x_fine = 0;
    } else {
        // Exiting right: only at tiles >= 62, and only if a room exists to the
        // right (columns are 0..3).
        if (engine.state.player_x_tile < 62) {
            return;
        }
        if (((engine.state.map_screen_x + 1) as u8 as i32) >= 4) {
            return;
        }
        engine.state.map_screen_x = engine.state.map_screen_x + 1;
        engine.state.player_facing = 64; // face left (entering from the right edge).
        draw_player_sprites(engine, r);
        engine.state.scroll_tile_x = 0; // show the left end of the new room.
        engine.state.player_x_fine = 0;
        engine.state.player_x_tile = 0; // place player at the far-left column.
    }
    // Build the adjacent room into the off-screen nametable.
    reset_room_object_slots(engine, r);
    clear_gameplay_object_sprites(engine, r);
    engine.state.scroll_fine_x = 0;
    scene_assemble(engine, r);
    upload_room_columns_from_bank9(engine, r);
    upload_palette_buffer(engine, r);
    // player_x_tile != 0 here means we exited LEFT (player was placed at tile 63);
    // animate the scroll rightward. (== 0 means we exited right; handled below.)
    if (engine.state.player_x_tile != 0) {
        // Scroll-in from the left edge: start at the new screen and walk the
        // player sprite (OAM entries 16/20) inward as the view scrolls.
        engine.state.nametable_select = 1;
        engine.state.scroll_pixel_x = 0;
        engine.state.set_oam_x(16, 0);
        engine.state.set_oam_x(20, 8); // second sprite half is 8 px right.
        engine.state.scratch2 = 15; // 16 outer steps (15..0).
        loop {
            engine.state.scratch3 = 3; // 4 inner frames per step (3..0).
            loop {
                // On the last inner frame, nudge the player sprite and toggle the
                // walk frame (tile bit 2) unless airborne (falling/jumping).
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
                // Move sprite right 4 px and scroll the view left 4 px each frame.
                engine.state.set_oam_x(16, engine.state.oam_x(16) + 4);
                engine.state.set_oam_x(20, engine.state.oam_x(16) + 8);
                engine.state.scroll_pixel_x = engine.state.scroll_pixel_x - 4;
                r.value = 255; // present one frame.
                queue_ppu_job_and_wait(engine, r);
                engine.state.scratch3 = engine.state.scratch3 - 1;
                if !((engine.state.scratch3 & ((crate::bits::BIT7) as u8)) == 0) {
                    break; // inner loop ends when counter underflows past 0.
                }
            }
            engine.state.scratch2 = engine.state.scratch2 - 1;
            if !((engine.state.scratch2 & ((crate::bits::BIT7) as u8)) == 0) {
                break; // outer loop ends when counter underflows past 0.
            }
        }
        // Patch the right-edge attribute/name column from bank 9 and finish.
        engine.state.vram_addr_lo = 30;
        engine.state.vram_addr_hi = 32; // nametable 0x201E.
        engine.state.data_ptr_lo = 47;
        farcall_bank_09_r7(engine, r);
        // ($D7E6 tail: JSR $C833 -> SEC; RTS, leaving frame_counter unchanged.)
        r.carry = 1;
        return;
    }
    // Scroll-in from the right edge (exited right): mirror image of the above,
    // starting the player sprite off the right side and scrolling rightward.
    engine.state.scroll_pixel_x = 252;
    engine.state.nametable_select = 1;
    engine.state.set_oam_x(16, 240);
    engine.state.set_oam_x(20, 248);
    engine.state.scratch2 = 15; // 16 outer steps.
    loop {
        engine.state.scratch3 = 3; // 4 inner frames per step.
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
            // Move sprite left 4 px and scroll the view right 4 px each frame.
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
    // Patch the left-edge column from bank 9 and finish.
    engine.state.vram_addr_lo = 0;
    engine.state.vram_addr_hi = 36; // nametable 0x2400 (second page).
    engine.state.data_ptr_lo = 16;
    farcall_bank_09_r7(engine, r);
    // ($D855 tail: JSR $C833 -> SEC; RTS, leaving frame_counter unchanged.)
    r.carry = 1;
}

/// Projects the player's next position into the projection scratch without
/// committing it. Copies player X fine/tile (0x43/0x44) into
/// `indirect_ptr_lo`/`indirect_ptr_hi` (0x0E/0x0F) and player Y (0x45) into
/// `scratch2` (0x0A), then applies the vertical delta (0x4B) and the horizontal
/// sub-tile delta (0x49) plus the X tile velocity (0x4A). The fine X is a 4-bit
/// sub-tile position; its overflow carries into the tile.
pub fn project_player_position(engine: &mut Engine, r: &mut RoutineContext) {
    // Seed the projection from the current player position.
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.scratch2 = engine.state.player_y;
    // Apply vertical movement.
    if (engine.state.vertical_delta != 0) {
        engine.state.scratch2 = engine.state.vertical_delta + engine.state.scratch2;
    }
    // Apply horizontal movement: add the sub-tile delta to fine X, keep the low
    // nibble, and carry the bit-4 overflow plus tile velocity into the tile X.
    let horizontal_subtile_delta: i32 = (engine.state.horizontal_subtile_delta as i32);
    if (horizontal_subtile_delta != 0) {
        let sum: i32 =
            ((horizontal_subtile_delta + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((sum & crate::bits::LOW_NIBBLE) as u8); // fine X is 4 bits.
        let carry: i32 = (((sum >> 4) & 1) as u8 as i32); // overflow out of the nibble.
        engine.state.indirect_ptr_hi =
            engine.state.indirect_ptr_hi + engine.state.player_x_velocity + (carry as u8);
    }
}

/// Chooses the player's animation pose byte (`player_pose`, 0x56) and facing
/// flip (`player_facing`, 0x57) from the current motion. State 0 bails out while
/// the landing/pose lockouts are active or the action button (bit 7) is held,
/// then branches on vertical motion: upward (negative delta) keeps the jump
/// pose, downward picks falling, the down button (bit 2) sets the crouch/down
/// pose 13, and no vertical motion uses the walking selector. States 2/3 set the
/// walking pose (bits kept = low 3) and facing; states 4/5/6 set the
/// jump/fall/idle pose (bits kept = low 2) and facing. Facing 64 = flip left.
pub fn update_player_pose_from_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut a: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // During landing recovery, leave the pose alone.
                x = 61;
                if (engine.state.landing_timer != 0) {
                    engine.state.player_pose = (x as u8); // $D8E7 BNE $D92E: STX $56
                    return;
                }
                // A scripted/special pose is in effect: leave it alone.
                x = 9;
                if (engine.state.pose_state != 0) {
                    engine.state.player_pose = (x as u8); // $D8ED BNE $D92E: STX $56
                    return;
                }
                // Action button held (bit 7 only, bit 6 masked out): hold pose.
                if ((engine.state.buttons & ((crate::bits::CLEAR_BIT6) as u8)) == 128) {
                    engine.state.player_pose = (x as u8); // $D8F5 BEQ $D92E: STX $56
                    return;
                }
                // No vertical motion: use the horizontal walking selector.
                a = (engine.state.vertical_delta as i32);
                if (a == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Moving up (negative delta): keep the jump pose.
                if ((a & crate::bits::BIT7) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Moving down while falling: use the airborne pose selector.
                if (engine.state.fall_frames != 0) {
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                // Moving down without the down button: walking selector.
                if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) == 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Down button held: force the crouch/down pose (13).
                x = 13;
                engine.state.player_pose = (x as u8);
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Rising: only the jump pose if a jump is actually in progress.
                if (engine.state.jump_timer == 0) {
                    engine.state.player_pose = (x as u8); // $D90E BEQ $D92E: STX $56
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
                // Walking selector: pose base 1, facing right; flip left if moving
                // left (X velocity negative) or facing chosen from sub-tile delta.
                x = 1;
                y = 0;
                if ((engine.state.player_x_velocity & ((crate::bits::BIT7) as u8)) != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.horizontal_subtile_delta == 0) {
                    return; // standing still: keep current pose.
                }
                y = 64; // moving right via sub-tile delta: face left flag.
                state = 3;
                continue 'dispatch;
            }
            3 => {
                // Commit walking pose: keep low 3 bits, OR in the new base.
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
                // Airborne selector: pose base 57 (jump/fall); if no horizontal
                // motion, use idle-air base 9 instead.
                x = 57;
                y = 0;
                a = ((engine.state.player_x_velocity | engine.state.horizontal_subtile_delta)
                    as i32);
                if ((a & crate::bits::BIT7) != 0) {
                    {
                        state = 6; // moving left: face right (y stays 0).
                        continue 'dispatch;
                    }
                }
                if (a != 0) {
                    {
                        state = 5; // moving right: set flip flag.
                        continue 'dispatch;
                    }
                }
                x = 9; // no horizontal motion.
                state = 5;
                continue 'dispatch;
            }
            5 => {
                y = 64; // face-left flip flag.
                state = 6;
                continue 'dispatch;
            }
            6 => {
                // Commit airborne/idle pose: keep low 2 bits, OR in the new base.
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

/// Advances the player's walk-cycle animation. While not landing and in a
/// ground pose (< 32), the attack/action button (bit 6) toggles the attack pose
/// bit (bit 4). Animation only steps when a D-pad direction is held (low nibble)
/// and the player is grounded (not jumping/falling), and only every 8th tick
/// (low 3 bits of the step counter). On a step it either flips facing (bit 6 of
/// facing) for the "axis" pose (pose bit 3 set) or toggles the walk frame
/// (pose bit 2) otherwise.
pub fn tick_player_walk_animation(engine: &mut Engine, r: &mut RoutineContext) {
    // Fold the attack button into the pose while grounded (pose < 32).
    if (engine.state.landing_timer == 0) {
        if (engine.state.player_pose < 32) {
            if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                engine.state.player_pose = engine.state.player_pose | ((crate::bits::BIT4) as u8); // set attack bit.
            } else {
                engine.state.player_pose =
                    engine.state.player_pose & ((crate::bits::CLEAR_BIT4) as u8); // clear attack bit.
            }
        }
    }
    // No direction held: nothing to animate.
    if ((engine.state.buttons & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    // Airborne: don't run the ground walk cycle.
    if ((engine.state.jump_timer | engine.state.fall_frames) != 0) {
        return;
    }
    // Advance the step counter; only act on every 8th tick.
    engine.state.anim_step_counter = engine.state.anim_step_counter + 1;
    if ((engine.state.anim_step_counter & ((crate::bits::LOW_3_BITS) as u8)) != 0) {
        return;
    }
    // Pose bit 3 selects between flipping facing vs. toggling the walk frame.
    if ((engine.state.player_pose & ((crate::bits::BIT3) as u8)) != 0) {
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8);
    } else {
        engine.state.player_pose = engine.state.player_pose ^ ((crate::bits::BIT2) as u8);
    }
}

/// Attempts to move the player by the current motion deltas, resolving all
/// collisions, and reports whether the move succeeded (carry set = clear path).
/// State 1 projects the position, then: out of bounds -> room transition; a
/// blocking tile action -> retry with reduced delta; object overlap -> classify
/// it (state 0..8 in scratch0). Doors (object_state 1) unlock with a key,
/// event/shop rewards (scratch0 >= 9) and pickups (< 9) apply their reward,
/// magic-contact actors are triggered. States 5/6 implement the "displaced"
/// speed-boost retry: when blocked, shrink the horizontal then vertical delta by
/// 2 and re-attempt the move so the player still slides part-way. State
/// 7 = success, state 8 = restore the saved deltas and exit.
/// The original deltas (0x49/0x4B) are saved on entry and always restored.
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
                // Project the move and test it against the room bounds.
                project_player_position(engine, r);
                check_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    // Out of bounds: try to cross into the adjacent room.
                    handle_player_room_transition(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 7; // transition succeeded.
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 5; // blocked at edge: try a reduced nudge.
                        continue 'dispatch;
                    }
                }
                // Run any tile action at the projected cell (solid/hazard/etc.);
                // carry set means the tile blocks the move.
                dispatch_projected_tile_actions(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                // Check for overlap with a room object; carry clear = none.
                find_player_object_overlap(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 8;
                        continue 'dispatch;
                    }
                }
                // scratch0 holds the contact class. 9 = solid (blocks), < 9 =
                // pickup-style object, >= 9 (else) = event/shop reward object.
                a = (engine.state.scratch0 as i32);
                if (a == 9) {
                    {
                        state = 5; // solid object: nudge.
                        continue 'dispatch;
                    }
                }
                if (a < 9) {
                    {
                        state = 2; // pickup/actor contact.
                        continue 'dispatch;
                    }
                }
                // Event/shop reward object: a locked door (state 1) needs a key,
                // otherwise grant the event reward and clear the room flag.
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
                // Pickup/actor contact branch (scratch0 < 9).
                x = (engine.state.scratch1 as i32);
                r.index = (x as u8);
                v = engine.state.object_state(x);
                r.value = (v as u8);
                if (v == 1) {
                    {
                        state = 3; // locked door object: magic-contact path.
                        continue 'dispatch;
                    }
                }
                if (v >= 26) {
                    {
                        state = 4; // hostile actor: no pickup, just pass through.
                        continue 'dispatch;
                    }
                }
                // Collectible room object: pick it up and apply its reward.
                collect_room_pickup_object(engine, r);
                {
                    state = 7;
                    continue 'dispatch;
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                // Possibly trigger the contacted actor's magic behavior.
                try_trigger_magic_contact_actor(engine, r);
                state = 4;
                continue 'dispatch;
            }
            4 => {
                // Contact handled but the move itself is blocked.
                r.carry = 0;
                {
                    state = 8;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                // Speed-boost ("displaced") slide retry on the horizontal delta.
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
                // Reduce |delta| by 2 toward zero (bit 3 = sign of the nibble),
                // keep it in 4 bits, and re-try the move if anything remains.
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
                // Restore the horizontal delta, then do the same slide retry on
                // the vertical delta.
                engine.state.horizontal_subtile_delta = (saved_horizontal_subtile_delta as u8);
                x = (engine.state.vertical_delta as i32);
                if (x == 0) {
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                }
                // Reduce |delta| by 2 toward zero (bit 7 = sign), then re-try.
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
                // Success: report a clear move.
                r.carry = 1;
                state = 8;
                continue 'dispatch;
            }
            8 => {
                // Always restore the caller's original deltas before returning.
                engine.state.horizontal_subtile_delta = (saved_horizontal_subtile_delta as u8);
                engine.state.vertical_delta = (saved_vertical_delta as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// When the player touches an actor while a magic-contact effect is active
/// (e.g. a damaging aura) and magic remains, flags the touched actor's slot with
/// the high bit (state 0x80) so its high-bit/destruction behavior runs. Only
/// applies in the normal-tileset rooms (CHR bank slot 3 < 48).
pub fn try_trigger_magic_contact_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.chr_bank(3) < 48) // only in standard (non-special) tilesets.
        && (engine.state.magic_contact_flag != 0)
        && (engine.state.player_magic != 0))
    {
        let hit_slot: i32 = (engine.state.scratch1 as i32);
        engine.state.set_object_state(hit_slot, 128); // 0x80: trigger high-bit behavior.
    }
}

/// Applies a collectible reward from an event/shop object (no room object slot
/// needs hiding here, unlike `collect_room_pickup_object`). The object's tile id
/// is in A; `reward_id = A - 2`. reward_id >= 24 is invalid (sets error prompt
/// 6). reward_id 0..7 are the "big" stat rewards (full heal/refill, large
/// coins/keys, long buffs, room clear) with a text pointer chosen from a table;
/// reward_id 8..23 are inventory items (id = reward_id - 8), capped at 11 (full
/// prompt 29). Picking up the fragment item (inventory id 14) chains to the
/// fragment-shrine warp.
pub fn apply_event_collectible_reward(engine: &mut Engine, r: &mut RoutineContext) {
    let reward_id: i32 = (((r.value - 2) as u8) as i32); // tile id 2 == reward 0.
    engine.state.set_object_state(160, 0); // clear the event object slot (0xA0).
    // Out of range: show the generic prompt and bail.
    if (reward_id >= 24) {
        engine.state.prompt_state = 6;
        return;
    }
    if (reward_id < 8) {
        // "Big" stat rewards 0..7, each with its own message pointer (ROM addrs).
        const EVENT_REWARD_TEXT: [i32; 8] = [
            0xD16A, 0xD199, 0xDB47, 0xDB52, 0xDB66, 0xDB7B, 0xDBB7, 0xDB9B,
        ];
        engine.state.data_ptr_lo =
            ((EVENT_REWARD_TEXT[reward_id as usize] & crate::bits::BYTE_MASK) as u8);
        engine.state.data_ptr_hi = ((EVENT_REWARD_TEXT[reward_id as usize] >> 8) as u8);
        r.value = ((reward_id << 1) as u8); // *2: word index for the consumer.
        r.index = r.value;
        match reward_id {
            0 => {
                animate_health_refill_to_cap(engine, r); // full heal.
            }
            1 => {
                animate_magic_refill_to_cap(engine, r); // full magic.
            }
            2 => {
                collect_large_coin_reward(engine, r); // big coins.
            }
            3 => {
                trigger_damage_pickup(engine, r); // trap/damage.
            }
            4 => {
                collect_key_bundle_reward(engine, r); // key bundle.
            }
            5 => {
                grant_long_invulnerability(engine, r); // long invuln.
            }
            6 => {
                defeat_active_room_actors(engine, r); // clear room.
            }
            7 => {
                grant_long_speed_boost(engine, r); // long speed boost.
            }
            _ => {}
        }
        return;
    }
    {
        // Inventory item reward (reward_id 8..23 -> item id 0..15).
        let inventory_item_id: i32 = ((reward_id - 8) as u8 as i32);
        // Already at the max count (11): show the "full" prompt, no increment.
        if (engine.state.inventory_item(inventory_item_id) >= 11) {
            engine.state.prompt_state = 29;
            return;
        }
        engine.state.set_inventory_item(
            inventory_item_id,
            (engine.state.inventory_item(inventory_item_id) + 1) & crate::bits::BYTE_MASK,
        );
        engine.state.prompt_state = 19; // "got item" prompt.
        // Fragment item (id 14) triggers the fragment-shrine warp.
        if (inventory_item_id == 14) {
            clear_room_persistent_flag(engine, r);
            enter_fragment_pickup_room(engine, r);
        }
    }
}

/// Collects a touched room pickup object: hides its object slot and OAM sprite,
/// then applies its reward. Mirrors `apply_event_collectible_reward` but for
/// in-room pickups, so it grants the "small" reward variants (small heal/magic/
/// coins, single key, short buffs). `reward_id = A - 2`; >= 24 is ignored.
/// X (r.index) is the contacted object slot offset; scratch0 is the OAM sprite
/// index used for that object. Inventory items (reward_id 8..23) behave as in
/// the event path, including the fragment (id 14) warp.
pub fn collect_room_pickup_object(engine: &mut Engine, r: &mut RoutineContext) {
    let reward_id: i32 = (((r.value - 2) as u8) as i32); // tile id 2 == reward 0.
    if (reward_id >= 24) {
        return; // out of range: ignore.
    }
    {
        // Deactivate the object slot and set its respawn/cleanup timer (0xF0).
        let object_slot_offset: i32 = (r.index as u8 as i32);
        engine.state.set_object_state(object_slot_offset, 0);
        engine.state.set_object_timer(object_slot_offset, 240);
    }
    {
        // Hide the object's two OAM sprite rows (Y=239). OAM offset = sprite
        // index * 8 (two 4-byte entries) with bit 7 set to land in the object
        // sprite half of OAM.
        let oam_offset: i32 = ((((engine.state.scratch0 as i32) << 3)
            | (((crate::bits::BIT7) as u8) as i32)) as u8 as i32);
        engine.state.set_oam_y(oam_offset, 239);
        engine.state.set_oam_y(4 + oam_offset, 239); // second 4-byte entry.
        r.index = (oam_offset as u8);
    }
    if (reward_id < 8) {
        // "Small" pickup rewards 0..7, each with its own message pointer (ROM addrs).
        const PICKUP_REWARD_TEXT: [i32; 8] = [
            0xDB26, 0xDB31, 0xDB3C, 0xDB52, 0xDB5D, 0xDB71, 0xDBB7, 0xDB85,
        ];
        engine.state.data_ptr_lo =
            ((PICKUP_REWARD_TEXT[reward_id as usize] & crate::bits::BYTE_MASK) as u8);
        engine.state.data_ptr_hi = ((PICKUP_REWARD_TEXT[reward_id as usize] >> 8) as u8);
        r.value = ((reward_id << 1) as u8); // *2 word index.
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
        // Inventory item reward (reward_id 8..23 -> item id 0..15).
        let inventory_item_id: i32 = ((reward_id - 8) as u8 as i32);
        if (engine.state.inventory_item(inventory_item_id) >= 11) {
            engine.state.prompt_state = 29; // already at max count.
            return;
        }
        engine.state.set_inventory_item(
            inventory_item_id,
            (engine.state.inventory_item(inventory_item_id) + 1) & crate::bits::BYTE_MASK,
        );
        engine.state.prompt_state = 19; // "got item".
        // Fragment item (id 14) triggers the fragment-shrine warp.
        if (inventory_item_id == 14) {
            clear_room_persistent_flag(engine, r);
            enter_fragment_pickup_room(engine, r);
        }
    }
}

/// Grants the small health pickup: sets the health-pickup prompt and adds 5
/// health points.
pub fn collect_small_health_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 30; // health-pickup prompt/sound.
    r.value = 5; // health points to add.
    add_health_points(engine, r);
}

/// Pickup handler: grants a small magic reward (+5 magic) and queues the
/// pickup prompt/sound. Sets `prompt_state` 17 (item pickup chime) and
/// delegates the magic-meter increment to `add_magic_points` with A=5.
pub fn collect_small_magic_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17; // 17 = item-pickup prompt/sound
    r.value = 5; // +5 magic points
    add_magic_points(engine, r);
}

/// Pickup handler: grants the small coin reward (+2 coins) and queues the
/// pickup prompt/sound. `add_coins` reads the amount from A=r.value.
pub fn collect_small_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17; // 17 = item-pickup prompt/sound
    r.value = 2; // +2 coins
    add_coins(engine, r);
}

/// Pickup handler: grants the large coin reward (+50 coins) and queues the
/// pickup prompt/sound. Same path as the small reward with a larger amount.
pub fn collect_large_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 17; // 17 = item-pickup prompt/sound
    r.value = 50; // +50 coins
    add_coins(engine, r);
}

/// Trap pickup handler: damages the player (-5 health) and queues the damage
/// prompt/sound. `subtract_health_points` reads the amount from A=r.value.
pub fn trigger_damage_pickup(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 29; // 29 = damage prompt/sound
    r.value = 5; // -5 health points
    subtract_health_points(engine, r);
}

/// Pickup handler: grants a single key. Sets `prompt_state` 21 (key pickup)
/// and increments the key count via `add_key` (which adds one).
pub fn collect_single_key_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 21; // 21 = key-pickup prompt/sound
    add_key(engine, r);
}

/// Pickup handler: grants the large key bundle (+20 keys) and queues the key
/// pickup prompt. `add_keys` reads the count from A=r.value.
pub fn collect_key_bundle_reward(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 21; // 21 = key-pickup prompt/sound
    r.value = 20; // +20 keys
    add_keys(engine, r);
}

/// Pickup handler: grants short (10-frame) sprite-blink invulnerability.
/// Sets the generic pickup prompt and arms the blink/i-frame countdown.
pub fn grant_short_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 19; // 19 = generic pickup prompt/sound
    engine.state.sprite_blink_timer = 10; // 10 frames of blink/invulnerability
    r.value = 10;
}

/// Pickup handler: grants long (30-frame) sprite-blink invulnerability.
/// Same as the short variant with a longer countdown.
pub fn grant_long_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 19; // 19 = generic pickup prompt/sound
    engine.state.sprite_blink_timer = 30; // 30 frames of blink/invulnerability
    r.value = 30;
}

/// Pickup handler: starts (or queues) a short speed/action boost. The boost
/// timers form a three-deep stack `0x88` (displaced/front), `0x89` (boost),
/// `0x8A` (short). A new boost always loads the front timer `0x88`; if it was
/// already running the previous value cascades into the next slot so a second
/// pickup extends rather than overwrites. Returns the previously-front timer
/// value in A=r.value and the loaded duration in X=r.index.
pub fn grant_short_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
    let boost_duration: i32 = 30; // boost length in frames
    let mut displaced_timer: i32 = 0;
    engine.state.prompt_state = 19; // 19 = generic pickup prompt/sound
    // If the front timer is already running, push it down the stack.
    displaced_timer = (engine.state.displaced_timer as i32);
    if (displaced_timer != 0) {
        // And if the second slot is also busy, push that into the short slot too.
        displaced_timer = (engine.state.boost_timer as i32);
        if (displaced_timer != 0) {
            engine.state.short_boost_timer = (boost_duration as u8);
        }
        engine.state.boost_timer = (boost_duration as u8);
    }
    // The newest boost always occupies the front timer slot.
    engine.state.displaced_timer = (boost_duration as u8);
    r.value = (displaced_timer as u8);
    r.index = (boost_duration as u8);
}

/// Pickup handler: starts (or queues) a long speed/action boost. Like
/// `grant_short_speed_boost` but with a four-deep stack `0x88`,`0x89`,`0x8A`,
/// `0x8B` (long) and a 60-frame duration: a new boost cascades busy timers one
/// slot deeper so stacked pickups accumulate. Returns previously-front timer in
/// A=r.value and the loaded duration in X=r.index.
pub fn grant_long_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
    let boost_duration: i32 = 60; // boost length in frames
    let mut displaced_timer: i32 = 0;
    engine.state.prompt_state = 19; // 19 = generic pickup prompt/sound
    // Cascade busy timers down the stack: front -> boost -> short -> long.
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
    // The newest boost always occupies the front timer slot.
    engine.state.displaced_timer = (boost_duration as u8);
    r.value = (displaced_timer as u8);
    r.index = (boost_duration as u8);
}

/// "Kill all" pickup effect: marks every active room actor as defeated and
/// triggers the screen palette flash. Walks the 9 object slots (16 bytes each)
/// and, for any whose state byte == 1 (alive/active), sets it to 128 (bit7 =
/// dying/defeated). Then queues the flash prompt and runs the palette flash.
pub fn defeat_active_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
    // Scan the 9 object slots; slots are 16 bytes apart in zero page.
    let mut slot_offset: i32 = 0;
    for _ in 0..9 {
        // State 1 = active enemy; flag it as defeated (bit7 set).
        if (engine.state.object_state(slot_offset) == 1) {
            engine.state.set_object_state(slot_offset, 128);
        }
        slot_offset = ((slot_offset + 16) as u8 as i32); // next slot (+16 bytes)
    }
    engine.state.prompt_state = 24; // 24 = flash/effect prompt
    engine.state.prompt_argument = 255; // flash all sprites
    r.index = 2; // flash-buffer mode/index
    flash_palette_buffer(engine, r);
}

/// Returns carry set when the tile above the top screen edge is empty (exit
/// clear) and also carry set while airborne/jumping ($DCA6 SEC); returns carry
/// clear when the player is not at the top sub-tile row ($DCA4 CLC).
pub fn check_top_boundary_exit_clear(engine: &mut Engine, r: &mut RoutineContext) {
    // Airborne/mid-jump: $DC8B BNE $DCA6 (SEC).
    if engine.state.airborne_flag != 0 || engine.state.jump_timer != 0 {
        r.carry = 1;
        return;
    }
    // Only at the topmost sub-tile (fine-X low byte == 0) is an up-exit possible
    // ($DC8F BNE $DCA4 CLC).
    if engine.state.indirect_ptr_lo != 0 {
        r.carry = 0;
        return;
    }
    // Build a tile pointer to the very top row (row 0) at the player's column.
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi; // column / tile-x
    engine.state.data_ptr_hi = 0; // row 0 (top edge)
    resolve_room_tile_pointer(engine, r);
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    // Mask off attribute bits; only the low 6 bits are the tile id.
    let tile = engine.state.byte(tile_ptr) & crate::bits::LOW_6_BITS;
    r.carry = ((tile == 0) as u8); // carry set => empty tile, exit is clear
}

/// Applies tile `0x30` (48) hazard contact at `tile_ptr + Y=r.offset`, e.g.
/// spikes/lava. Returns carry clear when the sampled tile is not the hazard.
/// On contact: arms a short recoil/jump timer, and if not currently
/// invulnerable, deducts one health point, queues the hurt prompt, and arms a
/// minimal blink so the hit only registers once. Returns carry set on contact.
pub fn apply_hazard_tile_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    // Sample the footprint tile (offset Y) and strip attribute bits.
    let tile = engine
        .state
        .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS;
    if tile != 48 {
        // 48 = 0x30, the hazard tile id
        r.carry = 0; // not a hazard tile
        return;
    }
    // Arm a short recoil/hop (10 frames) if not already jumping.
    if engine.state.jump_timer == 0 {
        engine.state.jump_timer = 10;
    }
    // Only take damage when not in i-frames; latch a 1-frame blink so the
    // same contact does not drain health every frame.
    if engine.state.sprite_blink_timer == 0 {
        consume_health_point(engine, r);
        engine.state.prompt_state = 10; // 10 = hurt prompt/sound
        engine.state.sprite_blink_timer = 1; // minimal i-frame latch
    }
    r.carry = 1; // hazard contact handled
}

/// Reports (in carry) whether a player footprint sample at `tile_ptr +
/// Y=r.offset` collides with terrain. Tile id 2 and any id >= 48 (0x30, the
/// solid/hazard range) are always solid. The empty tile (id 0) only counts as
/// contact when the player is exactly tile-aligned (fine-X == 0), which keeps
/// the player snapped to the boundary while straddling two columns.
pub fn probe_player_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    // Sample the footprint tile (offset Y) and strip attribute bits.
    let tile = engine
        .state
        .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS;
    if tile == 0 {
        // Empty tile: solid only when perfectly column-aligned.
        if engine.state.player_x_fine == 0 {
            r.carry = 1;
        } else {
            r.carry = 0;
        }
    } else if tile == 2 {
        // Tile id 2 is always solid.
        r.carry = 1;
    } else {
        // Ids >= 48 (0x30) are the solid/hazard range.
        r.carry = ((tile >= 48) as u8);
    }
}

/// Handles Up-button interactions with the tile directly above the player.
/// Samples the tile one row up at the player's column, then (if the player
/// straddles two columns, fine-X != 0) the adjacent column. Tile id 5
/// enters the character-select room, id 4 the shop, id 3 the four-fragment
/// portal (gated on the selected `0x0E` item and all four fragments held).
pub fn dispatch_overhead_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    let player_y = engine.state.player_y;
    if player_y == 0 {
        return; // already at the top row; nothing above
    }

    // Point at the tile one row above the player's current column.
    engine.state.data_ptr_hi = player_y - 1; // one row up
    engine.state.data_ptr_lo = engine.state.player_x_tile;
    resolve_room_tile_pointer(engine, r);

    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    // Check the primary column; bail if it consumed the action.
    if dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 0) {
        return;
    }
    // Straddling columns: also check the adjacent tile (+12 byte offset).
    if engine.state.player_x_fine != 0 {
        dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 12);
    }
}

/// Dispatches the overhead tile at `tile_ptr + offset`. Returns true when the
/// tile triggered a room transition (and set the non-local handoff flag).
fn dispatch_overhead_tile_at_offset(
    engine: &mut Engine,
    r: &mut RoutineContext,
    tile_ptr: i32,
    offset: i32,
) -> bool {
    r.offset = (offset as u8);
    // Strip attribute bits and dispatch on the tile id.
    match engine
        .state
        .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS
    {
        5 => {
            // Tile id 5 = character-select room entrance.
            run_character_select_room_flow(engine, r);
            engine.lotw_nonlocal_handoff = 1; // long jump out to room flow
            true
        }
        4 => {
            // Tile id 4 = shop room entrance.
            run_shop_room_flow(engine, r);
            engine.lotw_nonlocal_handoff = 1; // long jump out to room flow
            true
        }
        3 => {
            // Tile id 3 = four-fragment portal (gated inside the helper).
            dispatch_four_fragment_overhead_tile(engine, r);
            true
        }
        _ => false,
    }
}

/// Four-fragment portal gate (tile id 3). Requires the currently selected item
/// slot to hold item 14 (a Dragon Slayer fragment) and the total fragment
/// count (saved counter `0x..` plus any held in the 3 carried slots) to equal
/// 4. On success, follows the room link to the destination. Returns true only
/// when the transition fired.
fn dispatch_four_fragment_overhead_tile(engine: &mut Engine, r: &mut RoutineContext) -> bool {
    // The selected item must itself be a fragment (item id 14).
    let selected_slot = engine.state.selected_item_slot;
    if engine.state.item_slot(selected_slot as i32) != 14 {
        return false;
    }

    // Tally fragments held across the 3 carried-item slots on top of the
    // persistent fragment counter.
    let mut fragment_count = engine.state.fragment_count;
    for slot in 0..=2 {
        // 3 carried-item slots
        if engine.state.item_slot(slot) == 14 {
            fragment_count = ((fragment_count + 1) as u8);
        }
    }
    if fragment_count != 4 {
        return false; // need all 4 fragments
    }

    // Portal unlocked: jump to the linked destination room.
    enter_room_link_destination(engine, r);
    engine.lotw_nonlocal_handoff = 1; // long jump out to room flow
    true
}

/// Checks the projected player footprint (candidate next position) for room
/// tile actions, sampling up to four tiles: the primary column (+0) and its
/// adjacent column (+12) when straddling, then the row below (+1 / +13) when
/// the projection lies within the play area (pixel-Y < 176) and is not row
/// aligned. The first action to fire wins. Returns carry = handled. The
/// projection scratch (`0xFA` tile-x, `0xF9` sub-tile, `0x..` pixel-Y in
/// scratch2) is saved and restored so the caller can keep resolving collision
/// with the same candidate position.
pub fn dispatch_projected_tile_actions(engine: &mut Engine, r: &mut RoutineContext) {
    // Object slot pointer = $0490 (the projection/object scratch area).
    engine.state.obj_slot_ptr_lo = 144; // 0x90
    engine.state.obj_slot_ptr_hi = 4; // 0x04

    // Save the projection coordinates so we can restore them on exit.
    let saved_subtile_x = engine.state.indirect_ptr_lo;
    let saved_tile_x = engine.state.indirect_ptr_hi;
    let saved_pixel_y = engine.state.scratch2;

    // Resolve a tile pointer for (tile-x = indirect_ptr_hi, pixel-Y = scratch2).
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);

    // Primary column; if straddling (sub-tile != 0) also try the adjacent column (+12).
    let mut handled = dispatch_projected_tile_action_at_offset(engine, r, 0);
    if !handled && engine.state.indirect_ptr_lo != 0 {
        handled = dispatch_projected_tile_action_at_offset(engine, r, 12);
    }

    // Within the playfield (pixel-Y < 176) and not row-aligned (low nibble set):
    // also probe the row below (+1) and its adjacent column (+13).
    let projected_y = engine.state.scratch2;
    if !handled && projected_y < 176 && (projected_y & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
        // 176 = playfield bottom in pixels
        handled = dispatch_projected_tile_action_at_offset(engine, r, 1);
        if !handled && engine.state.indirect_ptr_lo != 0 {
            handled = dispatch_projected_tile_action_at_offset(engine, r, 13);
        }
    }

    r.carry = (handled as u8);
    // Restore the saved projection coordinates.
    engine.state.scratch2 = saved_pixel_y;
    engine.state.indirect_ptr_hi = saved_tile_x;
    engine.state.indirect_ptr_lo = saved_subtile_x;
}

/// Runs the room tile-action dispatcher for the tile at byte offset `offset`
/// (Y register) and reports whether it fired (carry set).
fn dispatch_projected_tile_action_at_offset(
    engine: &mut Engine,
    r: &mut RoutineContext,
    offset: i32,
) -> bool {
    r.offset = (offset as u8);
    dispatch_room_tile_action(engine, r);
    ((r.carry) != 0)
}

/// Converts the tile-sample byte offset in `0x0B` (scratch3) plus the
/// projected tile coordinates into an object spawn position written to the
/// object scratch (`0xF9..0xFC`). Offsets >= 12 select the adjacent column, so
/// the tile-x is bumped and the offset reduced by one column (12 bytes). A
/// non-zero remaining offset means the sample was the lower row, so pixel-Y is
/// advanced one tile (+16). The pixel-Y is snapped to a tile boundary (high
/// nibble) and X is placed at the tile's left edge (sub-tile 0).
pub fn seed_object_position_from_tile_offset(engine: &mut Engine, r: &mut RoutineContext) {
    let mut tile_offset: i32 = (engine.state.scratch3 as i32);
    // Offset into the second column (+12 bytes): advance tile-x by one.
    if (tile_offset >= 12) {
        tile_offset = ((tile_offset - 12) as u8 as i32); // back into first column
        engine.state.indirect_ptr_hi =
            (engine.state.indirect_ptr_hi + 1) & ((crate::bits::BYTE_MASK) as u8);
    }
    // Remaining non-zero offset = lower of the two rows: drop down one tile.
    if (tile_offset != 0) {
        engine.state.scratch2 = engine.state.scratch2 + 16; // +16 px = one tile row
    }
    // Snap Y to the tile's top edge and place X at the tile's left edge.
    engine.state.obj_y_pixel = engine.state.scratch2 & ((crate::bits::HIGH_NIBBLE) as u8);
    engine.state.obj_y_extra = 0;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_x_sub = 0;
    r.value = 0;
    r.offset = (tile_offset as u8);
}

/// Rebuilds the background tile column at object-scratch tile-x `0xFA`. Each
/// map tile is two PPU tiles wide, so the column's nametable X = tile_x*2; the
/// low 5 bits give the X within a 32-tile nametable, and bit4 of tile_x selects
/// which nametable half (left/right), contributing 0x04 to the high address
/// byte. The base nametable is at $2000, so the high byte starts at 0x20 (32).
pub fn redraw_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_x: i32 = (engine.state.obj_x_tile as i32);
    engine.state.data_ptr_lo = (tile_x as u8); // map column index for the redraw helper
    // PPU column X = tile_x*2, wrapped to a 32-wide nametable (low 5 bits).
    engine.state.vram_addr_lo = (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
    // bit4 of tile_x picks the nametable half -> high byte +0x04.
    engine.state.vram_addr_hi = (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
    engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
    engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // 32 = 0x20, $2000 nametable base
    farcall_bank_09_r7(engine, r);
}

/// Reads the room-map tile at the 16-bit room pointer (`0x10` low / `0x11`
/// high) plus the byte offset in `0x0B` (scratch3). Returns the raw tile byte
/// in A=r.value, the masked tile id (low 6 bits) in X=r.index, and the offset
/// in Y=r.offset. Tile id 62 (0x3E) is a placeholder that resolves to the
/// per-room replacement value held in `0x74` (room_tile_action).
pub fn read_room_tile_action_value(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_offset: i32 = (engine.state.scratch3 as i32);
    // Assemble the 16-bit room-map pointer from its low/high bytes.
    let room_ptr: i32 = (((engine.state.tile_fetch_counter as i32)
        | ((engine.state.aux_ptr_hi as i32) << 8)) as u16 as i32);
    let room_tile: i32 = engine.state.byte((room_ptr + tile_offset) as u16 as i32);
    let tile_id: i32 = room_tile & crate::bits::LOW_6_BITS; // strip attribute bits
    r.index = (tile_id as u8);
    r.offset = (tile_offset as u8);
    if (tile_id == 62) {
        // 62 = 0x3E placeholder -> use the room's replacement tile/action.
        r.value = (engine.state.room_tile_action as u8);
    } else {
        r.value = (room_tile as u8);
    }
}

/// After a blocked move, attempts a one-step nudge so the player slips around
/// a corner toward the nearest tile boundary. State 0 considers a vertical
/// nudge when there was horizontal intent (`horizontal_subtile_delta != 0`):
/// if the player's Y is within 5 px of a tile edge above/below it nudges that
/// way, unless the player is actively pressing away (Down/Up). State 1 is the
/// mirror case: a horizontal nudge driven by vertical intent, snapping toward
/// the nearer column unless pressing away (Right/Left). State 2 retries the
/// move with collision; state 3 returns carry=1 (no nudge applied). The retained
/// 6502 dead `continue`s after the early returns are preserved verbatim.
pub fn try_nudge_player_to_tile_boundary(engine: &mut Engine, r: &mut RoutineContext) {
    let horizontal_delta: i32 = (engine.state.horizontal_subtile_delta as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Clear any horizontal motion; we may convert it to a vertical nudge.
                engine.state.horizontal_subtile_delta = 0;
                engine.state.player_x_velocity = 0;
                if (horizontal_delta == 0) {
                    // No horizontal intent -> consider a horizontal nudge instead.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    // Y position within its tile (low nibble = sub-tile offset).
                    let mut a: i32 =
                        ((engine.state.player_y & ((crate::bits::LOW_NIBBLE) as u8)) as u8 as i32);
                    if (a == 0) {
                        // Already tile-aligned vertically: nothing to nudge.
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if (a < 6) {
                        // Near the top of the tile: nudge up unless pressing Down.
                        if ((engine.state.buttons & ((crate::bits::BIT2) as u8)) != 0) {
                            // BIT2 = Down pressed -> don't nudge up.
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.vertical_delta = 255; // -1 (move up)
                        engine.state.nudge_pending = 255; // direction = up
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (a >= 11) {
                        // Near the bottom of the tile: nudge down unless pressing Up.
                        if ((engine.state.buttons & ((crate::bits::BIT3) as u8)) != 0) {
                            // BIT3 = Up pressed -> don't nudge down.
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.vertical_delta = 1; // +1 (move down)
                        engine.state.nudge_pending = 0; // direction = down
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        // Mid-tile (6..=10): too far to snap, give up.
                        state = 3;
                        continue 'dispatch;
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                {
                    // A pending vertical nudge from state 0 enables the horizontal case.
                    let mut v4B: i32 = (engine.state.vertical_delta as i32);
                    engine.state.vertical_delta = 0;
                    engine.state.nudge_pending = 0;
                    if (v4B == 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    // X position within its tile (fine-X = sub-tile offset).
                    let mut a: i32 = (engine.state.player_x_fine as i32);
                    if (a == 0) {
                        // Already column-aligned: nothing to nudge.
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if (a < 6) {
                        // Near the left of the tile: nudge left unless pressing Right.
                        if ((engine.state.buttons & ((crate::bits::BIT0) as u8)) != 0) {
                            // BIT0 = Right pressed -> don't nudge left.
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.horizontal_subtile_delta = 15; // -1 sub-tile (wrap to 15)
                        engine.state.player_x_velocity = 255; // -1 (move left)
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (a >= 11) {
                        // Near the right of the tile: nudge right unless pressing Left.
                        if ((engine.state.buttons & ((crate::bits::BIT1) as u8)) != 0) {
                            // BIT1 = Left pressed -> don't nudge right.
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.state.horizontal_subtile_delta = 1; // +1 sub-tile (move right)
                        engine.state.player_x_velocity = 0;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        // Mid-tile (6..=10): too far to snap, give up.
                        state = 3;
                        continue 'dispatch;
                    }
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Re-attempt the move now that a nudge offset has been applied.
                try_move_player_with_collision(engine, r);
                return;
                state = 3;
                continue 'dispatch;
            }
            3 => {
                r.carry = 1; // no nudge applied / move stays blocked
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Attempts to close the item menu. First validates/decodes the on-screen item
/// list snapshot at pointer $B577 (the password-derived item list); if that
/// helper returns carry set the list is invalid and the close is aborted.
/// Otherwise it restores the pre-menu gameplay snapshot, re-syncs the key/coin
/// HUD, restages and uploads the room view, and restores the status sprites.
pub fn close_inventory_item_menu(engine: &mut Engine, r: &mut RoutineContext) {
    // Point at the item-list snapshot source ($B577).
    engine.state.indirect_ptr_lo = 119; // 0x77
    engine.state.indirect_ptr_hi = 181; // 0xB5
    decode_inventory_item_list_snapshot(engine, r);
    if ((r.carry) != 0) {
        return; // invalid list -> abort the close
    }
    engine.state.prompt_state = 16; // 16 = return-to-gameplay prompt
    restore_inventory_state_snapshot(engine, r);
    sync_key_hud(engine, r);
    sync_coin_hud(engine, r);
    engine.state.scroll_tile_x = 32; // restage from the second nametable column
    upload_staged_room_columns(engine, r);
    refresh_scroll_register_shadows(engine, r);
    restore_status_sprite_template(engine, r);
}

/// Selects the entry under the cursor in the item-input grid. The grid is 7
/// columns wide (and 5 rows tall), so the linear cell value = column*5 +
/// column*... ; specifically here `column*4 + column` (= column*5) plus the
/// row gives the cell index. Three cells are menu controls: 32 (0x20) and 33
/// (0x21) move the list cursor right/left, 34 (0x22) closes the menu. Any other
/// cell value is stored into the scrolling list buffer (password nibbles), then
/// the list cursor advances; reaching list slot 31 also closes the menu.
pub fn select_inventory_grid_entry(engine: &mut Engine, r: &mut RoutineContext) {
    let grid_column: i32 = (engine.state.obj_x_vel_lo as i32);
    // column*5: (column<<2)+column, byte-wrapped at each step.
    let mut grid_value: i32 = ((((grid_column << 2) as u8 as i32) + grid_column) as u8 as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Add the row to get the linear cell index.
                grid_value = ((grid_value + (engine.state.obj_y_vel as i32)) as u8 as i32);
                if (grid_value == 32) {
                    // 0x20 = "cursor right" control cell.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (grid_value == 33) {
                    // 0x21 = "cursor left" control cell.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (grid_value == 34) {
                    // 0x22 = "done/close" control cell.
                    close_inventory_item_menu(engine, r);
                    return;
                }
                // Normal entry: store its value at the current list slot.
                r.value = (grid_value as u8);
                set_inventory_list_buffer_index(engine, r);
                engine
                    .state
                    .set_password_nibbles_a((r.index as i32), grid_value);
                if (r.index == 31) {
                    // Filled the last (32nd) slot -> auto-close.
                    close_inventory_item_menu(engine, r);
                    return;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Advance the scrolling list cursor one slot to the right.
                engine.state.obj_x_sub =
                    (engine.state.obj_x_sub + 1) & ((crate::bits::BYTE_MASK) as u8);
                update_inventory_list_cursor_sprites(engine, r);
                return;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Move the scrolling list cursor one slot to the left.
                engine.state.obj_x_sub =
                    (engine.state.obj_x_sub - 1) & ((crate::bits::BYTE_MASK) as u8);
                update_inventory_list_cursor_sprites(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Moves the inventory grid cursor right across the 7 columns, wrapping past
/// column 6 back to column 0. Updates the grid cursor sprites afterward.
pub fn move_inventory_cursor_right(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_x_vel_lo + 1) as u8 as i32);
    if (x >= 7) {
        x = 0; // wrap past the 7th column (index 6) to column 0
    }
    engine.state.obj_x_vel_lo = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor left across the 7 columns, wrapping past
/// column 0 (which underflows, setting bit7) back to column 6.
pub fn move_inventory_cursor_left(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_x_vel_lo - 1) as u8 as i32);
    if ((x & crate::bits::BIT7) != 0) {
        // underflowed below 0
        x = 6; // wrap to the last column (index 6)
    }
    engine.state.obj_x_vel_lo = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor up across the 5 rows, wrapping past row 0
/// (underflow sets bit7) back to row 4.
pub fn move_inventory_cursor_up(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_y_vel - 1) as u8 as i32);
    if ((x & crate::bits::BIT7) != 0) {
        // underflowed below 0
        x = 4; // wrap to the last row (index 4)
    }
    engine.state.obj_y_vel = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Moves the inventory grid cursor down across the 5 rows, wrapping past row 4
/// back to row 0.
pub fn move_inventory_cursor_down(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = ((engine.state.obj_y_vel + 1) as u8 as i32);
    if (x >= 5) {
        x = 0; // wrap past the 5th row (index 4) to row 0
    }
    engine.state.obj_y_vel = (x as u8);
    update_inventory_grid_cursor_sprites(engine, r);
}

/// Positions the two arrow sprites (OAM entries 128 and 132) that point at the
/// scrolling selected item-list slot (`0xF9` masked to its low 5 bits = 0..31).
/// Slots 0..15 use cursor tile 97; slots 16..31 reuse the row by subtracting 16
/// and using tile 105 (a second row of arrows). The X position is slot*9 (i.e.
/// slot + slot*8 via shift, with the carry out of bit5 folded back in) offset by
/// 54 px to the playfield origin; the left arrow sits 8 px to the left.
pub fn update_inventory_list_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut list_slot: i32 = ((engine.state.obj_x_sub & ((crate::bits::LOW_5_BITS) as u8)) as i32);
    let mut cursor_tile: i32 = 97; // tile for slots 0..15
    if (list_slot >= 16) {
        list_slot = ((list_slot - 16) as u8 as i32); // fold the second half onto the same row
        cursor_tile = 105; // tile for slots 16..31
    }
    engine.state.set_oam_y(128, cursor_tile);
    engine.state.set_oam_y(132, cursor_tile);
    engine.state.scratch0 = (list_slot as u8);

    // slot*9 spacing: slot + slot*8, then re-add the carry out of bit5.
    let scaled_slot: i32 = (((list_slot >> 2) + list_slot) as u8 as i32);
    let carry: i32 = (((scaled_slot >> 5) & 1) as u8 as i32);
    let right_x: i32 = ((((scaled_slot << 3) as u8 as i32) + 54 + carry) as u8 as i32); // +54 = playfield left margin
    engine.state.set_oam_x(132, right_x);
    let left_x: i32 = ((right_x - 8) as u8 as i32); // left arrow is one tile (8 px) earlier
    engine.state.set_oam_x(128, left_x);
    r.index = (cursor_tile as u8);
    r.value = (left_x as u8);
}

/// Scales a grid index to a pixel offset: returns `(value*8, carry)` where the
/// carry is the bit shifted out of the 8-bit byte (bit5 of the scaled value),
/// matching the 6502 `ASL` carry that the callers fold back into the position.
fn scale_grid_coordinate(value: i32) -> (i32, i32) {
    (
        ((value << 3) as u8 as i32),       // value * 8 (tile width), byte-wrapped
        (((value >> 5) & 1) as u8 as i32), // carry out of the high bit
    )
}

/// Positions the 2x2 cursor (OAM entries 144/148) around the active item-grid
/// cell. The cell's column and row indices are each scaled to pixels (8 px per
/// cell), offset by 54 px (left margin) and 129 px (top margin) respectively;
/// the left/right and both cursor sprites share Y, with the left sprite 8 px in.
pub fn update_inventory_grid_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // Horizontal: column index -> pixel X, plus the playfield left margin.
    let (column_pixels, column_carry) = scale_grid_coordinate(engine.state.obj_x_vel_lo as i32);
    let right_x: i32 = ((column_pixels + 54 + column_carry) as u8 as i32); // +54 = left margin
    engine.state.set_oam_x(148, right_x);
    let left_x: i32 = ((right_x - 8) as u8 as i32); // left sprite one tile earlier
    engine.state.set_oam_x(144, left_x);

    // Vertical: row index -> pixel Y, plus the grid top margin.
    let (row_pixels, row_carry) = scale_grid_coordinate(engine.state.obj_y_vel as i32);
    let y: i32 = ((row_pixels + 129 + row_carry) as u8 as i32); // +129 = grid top margin
    engine.state.set_oam_y(144, y);
    engine.state.set_oam_y(148, y);
    r.value = (y as u8);
}

/// Converts the scrolling item-list cursor (`0xF9`) into a 0..31 buffer index
/// (the low 5 bits) returned in X=r.index.
pub fn set_inventory_list_buffer_index(engine: &mut Engine, r: &mut RoutineContext) {
    r.index = ((engine.state.obj_x_sub & ((crate::bits::LOW_5_BITS) as u8)) as u8);
}

/// Pops the saved gameplay-room checkpoint (taken when entering a temporary
/// room such as a shop) and fully rebuilds the room: fades out, clears the
/// temporary sprites, restores the saved song, rebuilds room metadata/palette,
/// re-uploads the room view, redraws the player and object sprites, refreshes
/// the scroll shadows, fades back in, and re-establishes the player pose and
/// walk animation.
pub fn restore_room_from_checkpoint(engine: &mut Engine, r: &mut RoutineContext) {
    // Restore the saved room state captured before the temporary room.
    pop_room_checkpoint(engine, r);
    fade_room_palette_out_reset_audio(engine, r);
    clear_temporary_room_sprites(engine, r);
    // The saved song id was stashed in the restore scratch byte.
    r.value = (engine.state.room_restore_scratch as u8);
    switch_song_if_needed(engine, r);
    // Rebuild the room graphics and sprites.
    prepare_room_metadata_and_palette(engine, r);
    upload_current_room_view(engine, r);
    draw_player_sprites(engine, r);
    draw_room_object_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
    fade_room_palette_in(engine, r);
    // Re-establish the player's animated pose.
    update_player_pose_from_motion(engine, r);
    tick_player_walk_animation(engine, r);
}

/// Enters a temporary room page selected by A=r.value, using the full
/// transition fade that also resets active music channel state. The page id is
/// unpacked into the map screen X (bits 2-3) and the room's scroll/tile column
/// (bits 0-1 scaled by 16), and the player is placed at a fixed spawn near the
/// bottom-center. Page 4 patches the tile-table high pointer to a special
/// graphics bank.
pub fn enter_temporary_room_page(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32); // page id
    fade_room_palette_out_reset_audio(engine, r);
    engine.state.scratch0 = (a as u8);
    // Unpack the page id: bits 2-3 -> map screen X, bits 0-1 -> tile column.
    engine.state.map_screen_x = (((a & crate::bits::BITS_2_3) >> 2) as u8);
    engine.state.scroll_tile_x = (((a & crate::bits::LOW_2_BITS) << 4) as u8); // *16 tiles per screen
    engine.state.player_x_tile = engine.state.scroll_tile_x + 7; // center of the screen
    engine.state.map_screen_y = 16;
    engine.state.player_x_fine = 8;
    engine.state.player_y = 160; // near the bottom of the playfield
    engine.state.jump_timer = 0;
    engine.state.fall_frames = 0;
    engine.state.scroll_fine_x = 0;
    clear_gameplay_object_sprites(engine, r);
    prepare_room_metadata_and_palette(engine, r);
    if (a == 4) {
        // Page 4 uses a special tile graphics bank.
        engine.state.tile_table_ptr_hi = 31 + 160; // 0x1F + 0xA0 = 0xBF
    }
    upload_staged_room_view(engine, r);
    update_player_pose_from_motion(engine, r);
    draw_player_sprites(engine, r);
    refresh_scroll_register_shadows(engine, r);
}

/// Rebuilds the temporary room page selected by A=r.value while preserving the
/// currently playing audio (unlike `enter_temporary_room_page`, which resets
/// the music channels). All other unpacking and spawn placement is identical.
pub fn refresh_temporary_room_page(engine: &mut Engine, r: &mut RoutineContext) {
    let mut a: i32 = (r.value as u8 as i32); // page id
    fade_room_palette_out_keep_audio(engine, r);
    engine.state.scratch0 = (a as u8);
    // Unpack the page id: bits 2-3 -> map screen X, bits 0-1 -> tile column.
    engine.state.map_screen_x = (((a & crate::bits::BITS_2_3) >> 2) as u8);
    engine.state.scroll_tile_x = (((a & crate::bits::LOW_2_BITS) << 4) as u8); // *16 tiles per screen
    engine.state.player_x_tile = engine.state.scroll_tile_x + 7; // center of the screen
    engine.state.map_screen_y = 16;
    engine.state.player_x_fine = 8;
    engine.state.player_y = 160; // near the bottom of the playfield
    engine.state.jump_timer = 0;
    engine.state.fall_frames = 0;
    engine.state.scroll_fine_x = 0;
    clear_gameplay_object_sprites(engine, r);
    prepare_room_metadata_and_palette(engine, r);
    if (a == 4) {
        // Page 4 uses a special tile graphics bank.
        engine.state.tile_table_ptr_hi = 31 + 160; // 0x1F + 0xA0 = 0xBF
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
    let mut y: i32 = 16; // OAM byte offset of the bottom (slot 2) item pair
    let mut a: i32 = 0;
    engine.state.scratch0 = 88; // running X position of the leftmost sprite in a pair
    {
        // Walk the three item slots from index 2 down to 0.
        x = 2;
        while (x >= 0) {
            let mut item: i32 = engine.state.item_slot(x);
            if ((item & crate::bits::BIT7) != 0) {
                // High bit set => empty slot; hide both sprites at Y=239 (off-screen).
                a = 239;
            } else {
                // Item id picks a 4-tile group: tile base = item*4 + 161.
                let mut t: i32 = ((((item << 2) as u8 as i32) + 161) as u8 as i32);
                engine.state.set_oam_tile(64 + y, t);
                engine.state.set_oam_tile(68 + y, t + 2); // right half is two tiles further
                a = 187; // on-screen Y for a visible item row
            }
            // Place the left (64+y) and right (68+y) halves of this item pair.
            engine.state.set_oam_y(64 + y, a);
            engine.state.set_oam_y(68 + y, a);
            engine
                .state
                .set_oam_x(64 + y, (engine.state.scratch0 as i32));
            engine
                .state
                .set_oam_x(68 + y, ((engine.state.scratch0 + 8) as i32)); // right half 8px over
            // Advance X by 8 then back by 40, i.e. -32, to the next slot column.
            engine.state.scratch0 = ((((engine.state.scratch0 + 8) as u8 as i32) - 40) as u8);
            engine.state.set_oam_attr(64 + y, 1); // palette 1, no flip
            engine.state.set_oam_attr(68 + y, 1);
            y = ((y - 8) as u8 as i32); // move up one item row (4 sprites = 16 bytes? two pairs)
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
///
/// Each slot byte holds an item id, or has bit7 set to mean "no item". When
/// the player already owns 11+ of an item the slot is cleared to 239 (the
/// off-screen Y / "unavailable" sentinel) so it cannot be bought again.
/// State machine: phase 0 draws the left slot, phase 1 the right slot, phase
/// 2 finalizes the right slot's Y/attr.
pub fn draw_shop_item_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    let mut a: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                a = 239; // default Y = off-screen / hidden
                x = engine.state.temp_save(0); // left shop slot item id
                if ((x & crate::bits::BIT7) != 0) {
                    // bit7 set => empty slot, leave hidden
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.inventory_item(x) >= 11) {
                    // Already owns the max stock: mark slot unavailable.
                    engine.state.set_temp_save(0, 239);
                    a = 239;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Visible item: tile base = item*4 + 161, right half +2.
                a = ((x << 2) as u8 as i32);
                a = ((a + 161) as u8 as i32);
                engine.state.set_oam_tile(64, a);
                a = ((a + 2) as u8 as i32);
                engine.state.set_oam_tile(68, a);
                engine.state.set_oam_x(64, 64); // left slot at X=64
                engine.state.set_oam_x(68, 72); // right half 8px over
                a = 164; // on-screen Y for left shop item
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Commit the left slot's Y/attr, then begin the right slot.
                engine.state.set_oam_y(64, a);
                engine.state.set_oam_y(68, a);
                engine.state.set_oam_attr(64, 1); // palette 1
                engine.state.set_oam_attr(68, 1);
                a = 239;
                x = engine.state.temp_save(2); // right shop slot item id
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
                engine.state.set_oam_x(72, 176); // right slot at X=176
                engine.state.set_oam_x(76, 184); // right half 8px over
                a = 160; // on-screen Y for right shop item
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Commit the right slot's Y/attr.
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
/// The pair sits centered (X=120/128) on the cost row (Y=152) using the coin
/// glyph tiles 241/243 with palette 2.
pub fn draw_coin_cost_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_oam_y(80, 152); // cost row Y
    engine.state.set_oam_y(84, 152);
    engine.state.set_oam_tile(80, 241); // left coin glyph
    engine.state.set_oam_tile(84, 243); // right coin glyph
    engine.state.set_oam_attr(80, 2); // palette 2
    engine.state.set_oam_attr(84, 2);
    engine.state.set_oam_x(80, 120); // centered horizontally
    engine.state.set_oam_x(84, 128);
    r.value = 128;
}

/// Hides the temporary room item and coin/cost sprites in OAM by moving the
/// six sprites at OAM bytes 64..84 to Y=239 (off the bottom of the screen).
pub fn clear_temporary_room_sprites(engine: &mut Engine, r: &mut RoutineContext) {
    // Item slots (64/68/72/76) and the coin/cost pair (80/84).
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
///
/// Copies the 56-byte status sprite Y template from ROM (`SPRITE_Y_TABLE_G`)
/// into OAM starting at byte 128, then reloads the four CHR banks (52..55)
/// that hold the status/menu glyphs.
pub fn restore_status_sprite_template(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    {
        // Copy 56 sprite-Y bytes (indices 55..0) into the status OAM region.
        x = 55;
        while (x >= 0) {
            engine.state.set_oam_y(
                128 + x, // status sprites live at OAM byte 128+
                engine.state.byte((SPRITE_Y_TABLE_G + x) as u16 as i32),
            );
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    // Reload the four CHR banks holding the status/menu tiles.
    engine.state.set_chr_bank(2, 52);
    engine.state.set_chr_bank(3, 53);
    engine.state.set_chr_bank(4, 54);
    engine.state.set_chr_bank(5, 55);
    r.index = 255;
    r.value = 55;
}

/// Spends one health point, returning carry set when health was already
/// empty. `r.value` returns the pre-decrement health.
pub fn consume_health_point(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = engine.state.player_health;
    if (r.value == 0) {
        // Nothing to spend: signal failure via carry.
        r.carry = 1;
        return;
    }
    engine.state.player_health = engine.state.player_health - 1;
    sync_health_hud(engine, r);
    r.carry = 0; // success
}

/// Subtracts `r.value` health (the damage amount), saturating at zero. Carry
/// is set when the player had enough health (no underflow), i.e. survived.
pub fn subtract_health_points(engine: &mut Engine, r: &mut RoutineContext) {
    let damage: i32 = (r.value as u8 as i32);
    engine.state.scratch0 = (damage as u8); // stash damage for callers
    let health: i32 = engine.state.player_health as i32;
    let enough_health: i32 = ((health >= damage) as u8 as i32);
    if ((enough_health) != 0) {
        engine.state.player_health = (health - damage) as u8;
    } else {
        // Underflow: clamp to zero (player would die).
        engine.state.player_health = 0;
    }
    sync_health_hud(engine, r);
    r.carry = (enough_health as u8); // carry = survived
}

/// Spends one magic point and preserves the caller's `r.index` (X). Carry is
/// set when no magic was available (failure), clear on success.
pub fn consume_magic_point(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_index: i32 = (r.index as u8 as i32); // preserve X across the call
    r.value = engine.state.player_magic;
    r.carry = 1; // default: failure
    if (engine.state.player_magic != 0) {
        engine.state.player_magic = engine.state.player_magic - 1;
        sync_magic_hud(engine, r);
        r.carry = 0; // success
    }
    r.index = (saved_index as u8);
}

/// Adds `r.value` health and clamps it to the HUD/resource maximum (109).
/// The HUD displays values up to 109, so any sum that reaches the 110 cap or
/// overflows a byte is pinned at 109.
pub fn add_health_points(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + engine.state.player_health as i32) as i32);
    let capped_total: i32 = if (total > 255) {
        109 // byte overflow -> max
    } else if ((total as u8 as i32) >= 110) {
        109 // at/over cap (110) -> max value 109
    } else {
        (total as u8 as i32)
    };
    engine.state.player_health = capped_total as u8;
    sync_health_hud(engine, r);
}

/// Adds `r.value` magic and clamps it to the HUD/resource maximum (109).
pub fn add_magic_points(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + engine.state.player_magic as i32) as i32);
    let capped_total: i32 = if (total > 255) {
        109 // byte overflow -> max
    } else if ((total as u8 as i32) >= 110) {
        109 // at/over cap (110) -> max value 109
    } else {
        (total as u8 as i32)
    };
    engine.state.player_magic = capped_total as u8;
    sync_magic_hud(engine, r);
}

/// Adds `r.value` coins and clamps them to the HUD/resource maximum (109).
/// Note the sum is reduced to a byte first, so `total > 255` can never be
/// true here; the cap-at-110 check is what actually limits the value.
pub fn add_coins(engine: &mut Engine, r: &mut RoutineContext) {
    // Keep the full (un-truncated) sum so the ADC overflow stays visible: the
    // original clamps to 109 when the add overflows (carry set) or the result
    // reaches the cap (110). Truncating to u8 first made the overflow branch
    // dead and let large adds wrap below the cap.
    let total: i32 = (r.value as i32) + (engine.state.coins as i32);
    let capped_total: i32 = if (total > 255) {
        109 // ADC overflow (carry set) -> clamp
    } else if (total >= 110) {
        109 // at/over cap (110) -> max value 109
    } else {
        total
    };
    engine.state.coins = (capped_total as u8);
    sync_coin_hud(engine, r);
}

/// Spends `r.value` coins. Carry is set on success and clear when the
/// player cannot afford the cost. `r.value` returns the remaining balance.
pub fn spend_coins(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch0 = (r.value as u8); // cost
    // 16-bit subtract so a borrow shows up as bit8 (the 6502 carry-clear case).
    let remaining_coins: i32 =
        (engine.state.coins as u16 as i32) - (engine.state.scratch0 as u16 as i32);
    r.value = (remaining_coins as u8);
    if ((remaining_coins & crate::bits::BIT8) != 0) {
        // Borrow occurred => cannot afford; leave coins unchanged.
        r.carry = 0;
        return;
    }
    engine.state.coins = (r.value as u8);
    sync_coin_hud(engine, r);
    r.carry = 1; // success
}

/// Adds one key and refreshes the key HUD digits. Carry cleared on return.
/// (Unlike `add_keys`, this single-key add is not clamped.)
pub fn add_key(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.keys = engine.state.keys + 1;
    sync_key_hud(engine, r);
    r.carry = 0;
}

/// Adds `r.value` keys and clamps them to the HUD/resource maximum (109).
pub fn add_keys(engine: &mut Engine, r: &mut RoutineContext) {
    let total: i32 = (((r.value as u16 as i32) + (engine.state.keys as i32)) as u8 as i32);
    let capped_total: i32 = if (total > 255) {
        109 // unreachable after the `as u8` above; kept to match original
    } else if ((total as u8 as i32) >= 110) {
        109 // at/over cap (110) -> max value 109
    } else {
        (total as u8 as i32)
    };
    engine.state.keys = (capped_total as u8);
    sync_key_hud(engine, r);
}

/// Spends one key, returning carry set when no key was available (failure).
/// `r.value` returns the pre-decrement key count.
pub fn consume_key(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = (engine.state.keys as u8);
    if (r.value == 0) {
        // No keys to spend.
        r.carry = 1;
        return;
    }
    engine.state.keys = engine.state.keys - 1;
    sync_key_hud(engine, r);
    r.carry = 0; // success
}

/// Updates live room objects by copying each 16-byte object slot into
/// scratch RAM `0xED..0xFC`, running the correct actor state path, then
/// copying the scratch state back to the slot.
///
/// There are two layouts. In a normal room (CHR bank 3 < 48) the small
/// actors are processed three slots per frame, round-robined across three
/// `scheduler_phase` values (0/1/2) so the full set is serviced every three
/// frames. In a boss room (CHR bank 3 >= 48) one large/boss actor (slot 0)
/// is ticked on even phases, and the secondary actors (slots 4..8) every
/// frame, with `scheduler_phase` toggling its low bit as the phase counter.
///
/// Each slot's `obj_state` selects the tick path: 0 = inactive (spawn),
/// bit7 set = defeated (reward drop / large-actor animation), 1 =
/// behavior-dispatched, >=24 = materialize delay, else standard actor.
pub fn update_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Y=16 is the title/menu screen marker: no room actors there.
                if (engine.state.map_screen_y == 16) {
                    return;
                }
                // CHR bank 3 >= 48 selects a boss room => use the large-actor path.
                if (engine.state.chr_bank(3) >= 48) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    // Round-robin: phase*3 picks the first of 3 slots this frame.
                    let mut scheduler_phase: i32 = (engine.state.scheduler_phase as i32);
                    let mut first_actor_slot: i32 =
                        (((scheduler_phase << 1) + scheduler_phase) as u8 as i32); // phase*3
                    engine.state.slot_index = (first_actor_slot as u8);
                    engine.state.slot_index_limit = ((first_actor_slot + 3) as u8);
                    // Object slots are 16 bytes each at $04xx; slot_index<<4 = byte offset.
                    let mut object_slot_lo: i32 =
                        (((engine.state.slot_index as i32) << 4) as u8 as i32);
                    engine.state.obj_slot_ptr_lo = (object_slot_lo as u8);
                    engine.state.actor_record_ptr_lo = ((object_slot_lo + 32) as u8); // record area +$20
                    engine.state.obj_slot_ptr_hi = 4; // object slots live in page $04
                    engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                }
                // Tick this frame's group of 3 small-actor slots.
                loop {
                    let mut actor_state: i32 = 0;
                    load_object_slot_scratch(engine, r);
                    actor_state = (engine.state.obj_state as i32);
                    if (actor_state == 0) {
                        tick_inactive_actor_slot(engine, r); // empty slot: try to spawn
                    } else if ((actor_state & crate::bits::BIT7) != 0) {
                        tick_defeated_actor_reward_drop(engine, r); // bit7: dying/reward
                    } else if (actor_state == 1) {
                        dispatch_actor_behavior(engine, r); // active: run behavior id
                    } else if (actor_state >= 24) {
                        tick_actor_materialize_delay(engine, r); // appearing
                    } else {
                        tick_standard_actor(engine, r); // generic motion
                    }
                    store_object_slot_scratch(engine, r);
                    engine.state.slot_index =
                        (engine.state.slot_index + 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.obj_slot_ptr_lo = engine.state.obj_slot_ptr_lo + 16; // next 16-byte slot
                    engine.state.actor_record_ptr_lo =
                        ((engine.state.actor_record_ptr_lo + 16) as u8);
                    if !(engine.state.slot_index < engine.state.slot_index_limit) {
                        break;
                    }
                }
                {
                    // Advance the round-robin phase, wrapping 0->1->2->0.
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
                // Boss room: tick the large actor (slot 0) only on even phases.
                if ((engine.state.scheduler_phase & ((crate::bits::BIT0) as u8)) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_slot_ptr_lo = 0; // slot 0
                engine.state.obj_slot_ptr_hi = 4;
                engine.state.slot_index = 0;
                engine.state.actor_record_ptr_lo = 32; // record area +$20
                engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                load_object_slot_scratch(engine, r);
                {
                    let mut actor_state: i32 = (engine.state.obj_state as i32);
                    if (actor_state == 0) {
                        initialize_large_actor_slot(engine, r); // empty: spawn boss
                    } else if ((actor_state & crate::bits::BIT7) != 0) {
                        // Defeated/idle: just update facing and body animation.
                        update_large_actor_facing_from_velocity(engine, r);
                        animate_large_actor_body_tiles(engine, r);
                    } else {
                        tick_large_chasing_actor(engine, r); // active boss AI
                    }
                }
                store_object_slot_scratch(engine, r);
                compose_large_actor_body_slots(engine, r); // assemble multi-tile body sprites
                {
                    state = 3;
                    continue 'dispatch;
                }
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Secondary boss-room actors: slots 4..8, ticked every frame.
                engine.state.slot_index = 4;
                engine.state.obj_slot_ptr_lo = 64; // slot 4 = byte 4*16
                engine.state.obj_slot_ptr_hi = 4;
                engine.state.actor_record_ptr_lo = 96; // 64 + $20 record offset
                engine.state.actor_record_ptr_hi = engine.state.palette_src_ptr_hi;
                loop {
                    let mut actor_state: i32 = 0;
                    load_object_slot_scratch(engine, r);
                    actor_state = (engine.state.obj_state as i32);
                    if ((actor_state == 0) || ((actor_state & crate::bits::BIT7) != 0)) {
                        // Empty or dead slot: maybe respawn a pursuer.
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
                        // process slots 4..8 inclusive
                        break;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                // Boss-room phase counter is just the toggling low bit.
                engine.state.scheduler_phase =
                    engine.state.scheduler_phase ^ ((crate::bits::BIT0) as u8);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Copies the 16-byte object slot addressed by `obj_slot_ptr` (`0xE5..0xE6`)
/// into scratch RAM `0xED..0xFC`. Counts down from 15 (the 6502 copies
/// high-index-first) and leaves `r.offset` (Y) at 255 on return.
pub fn load_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
    let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
    for slot_offset in (0..=15).rev() {
        // 16 bytes per object slot.
        engine.state.set_obj_scratch_byte(
            slot_offset,
            engine.state.byte((slot_ptr + slot_offset) as u16 as i32),
        );
    }
    r.offset = 255;
}

/// Writes the 16-byte scratch RAM `0xED..0xFC` back to the object slot
/// addressed by `obj_slot_ptr` (`0xE5..0xE6`). Inverse of
/// `load_object_slot_scratch`; leaves `r.offset` (Y) at 255.
pub fn store_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
    let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
    for slot_offset in (0..=15).rev() {
        // 16 bytes per object slot.
        engine.state.set_byte(
            ((slot_ptr + slot_offset) as u16 as i32),
            engine.state.obj_scratch_byte(slot_offset),
        );
    }
    r.offset = 255;
}

/// Initializes an inactive scratch slot from the room actor record at
/// `actor_record_ptr` (`0xE7..0xE8`). A nonzero timer leaves the actor
/// materializing (blinking sentinel sprite); a zero timer promotes it to the
/// normal active state with sprite bytes from the record.
///
/// Record layout used here: +0 tile, +1 attr, +2/+3 spawn X-tile/Y (0,0 =>
/// pick a random position), +4 health, +5 contact damage. If the current
/// family member is not in the actor's allowed-member mask, the contact
/// damage is doubled (saturating at 255) to make off-character actors harsher.
pub fn tick_inactive_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
    // Count down the per-slot spawn timer.
    engine.state.obj_timer = engine.state.obj_timer - 1;
    // Only attempt a spawn during the final 60 frames of the countdown
    // ($E9A9 CPX #$3C; BCS $E9E6). Without this guard the spawn probe runs and
    // consumes RNG every frame, desyncing the RNG stream and spawn placement.
    if engine.state.obj_timer >= 60 {
        return;
    }
    let actor_timer: i32 = (engine.state.obj_timer as i32);
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    if ((engine.state.byte((actor_data_ptr + 2) as u16 as i32)
        | engine.state.byte((actor_data_ptr + 3) as u16 as i32))
        == 0)
    {
        // No fixed spawn position (+2/+3 both 0): pick a random one.
        r.value = 12; // random Y in [0,11]
        rng_update(engine, r);
        engine.state.scratch2 = (((r.value as i32) << 4) as u8); // *16 = pixel row
        r.value = 64; // random X-tile in [0,63]
        rng_update(engine, r);
        engine.state.indirect_ptr_hi = (r.value as u8);
    } else {
        // Use the fixed spawn position from the record.
        engine.state.scratch2 = ((engine.state.byte((actor_data_ptr + 3) as u16 as i32)) as u8);
        engine.state.indirect_ptr_hi =
            ((engine.state.byte((actor_data_ptr + 2) as u16 as i32)) as u8);
    }
    engine.state.indirect_ptr_lo = 0; // sub-pixel X = 0
    engine.state.scratch3 = 0;
    // Probe checks (carry results are ignored here, matching the original).
    check_player_overlap(engine, r);
    if ((r.carry) != 0) {}
    check_projected_terrain_collision(engine, r);
    if ((r.carry) != 0) {}
    // Commit position and reset all motion/state fields.
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    engine.state.obj_y_extra = 0;
    engine.state.obj_health = ((engine.state.byte((actor_data_ptr + 4) as u16 as i32)) as u8);
    engine.state.obj_damage = ((engine.state.byte((actor_data_ptr + 5) as u16 as i32)) as u8);
    {
        // Build a 1-bit-per-member mask shifted to the current character's
        // position (character_index counts down to set that bit), then AND
        // with the actor's allowed-member mask.
        let mut current_member_bit: i32 = 0;
        let mut carry_bit: i32 = 1;
        let mut member_index: i32 = (engine.state.character_index as i32);
        loop {
            // Rotate-left through carry, seeding a 1 at the bottom.
            let mut next_carry_bit: i32 = (((current_member_bit >> 7) & 1) as u8 as i32);
            current_member_bit = (((current_member_bit << 1) | carry_bit) as u8 as i32);
            carry_bit = next_carry_bit;
            member_index = ((member_index - 1) as u8 as i32);
            if !((member_index & crate::bits::BIT7) == 0) {
                // loop until member_index goes negative (high bit set)
                break;
            }
        }
        current_member_bit =
            ((current_member_bit & (engine.state.family_member_mask as i32)) as u8 as i32);
        if (current_member_bit == 0) {
            // This character is not the actor's intended target: double damage.
            let mut contact_damage: i32 = (engine.state.obj_damage as i32);
            let mut damage_overflow: i32 = (((contact_damage >> 7) & 1) as u8 as i32);
            engine.state.obj_damage = ((contact_damage << 1) as u8); // *2
            if ((damage_overflow) != 0) {
                engine.state.obj_damage = 255; // saturate
            }
        }
    }
    // Default to the materializing state with the blinking sentinel sprite.
    engine.state.obj_state = 127; // materializing (high-ish, not active=1)
    engine.state.obj_tile = 249; // sentinel/appearing tile
    engine.state.obj_attr = 1;
    if (actor_timer == 0) {
        // Timer expired: become active with the record's real tile/attr.
        engine.state.obj_state = 1;
        engine.state.obj_tile = ((engine.state.byte((actor_data_ptr + 0) as u16 as i32)) as u8);
        engine.state.obj_attr = ((engine.state.byte((actor_data_ptr + 1) as u16 as i32)) as u8);
    } else {
        // Still materializing: blink by toggling the H-flip bit every 4 frames.
        if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
            engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
        }
    }
}

/// Counts down a materializing actor. When the timer reaches zero, the slot
/// becomes behavior-dispatched state `0x01` with sprite bytes (tile +0, attr
/// +1) from the room actor record; otherwise it blinks every 4 frames by
/// toggling the H-flip attribute bit.
pub fn tick_actor_materialize_delay(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_timer: i32 = ((engine.state.obj_timer - 1) as u8 as i32);
    engine.state.obj_timer = (actor_timer as u8);
    if (actor_timer == 0) {
        // Finished appearing: switch to the active behavior state.
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        engine.state.obj_state = 1;
        engine.state.obj_tile = ((engine.state.byte(actor_data_ptr)) as u8);
        engine.state.obj_attr = ((engine.state.byte((actor_data_ptr + 1) as u16 as i32)) as u8);
    } else if ((actor_timer & crate::bits::LOW_2_BITS) == 0) {
        // Every 4th frame: toggle the H-flip bit (bit6) to blink.
        engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
    }
}

/// Some late-game (boss) rooms periodically seed extra pursuer actors based
/// on the large actor's position. The 1-in-30 roll keeps empty secondary
/// slots from respawning every frame.
///
/// Copies 4 bytes of position from the large actor (slot 0) into the new
/// actor's scratch position. The source offset depends on slot 0's facing
/// (attr bit6): one of two body anchor points is used. Health/damage come
/// from the room actor record (+4/+5); the actor spawns active (state 1) with
/// tile 129, a random attribute, and a long cooldown (128).
pub fn maybe_spawn_pursuer_actor(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 30; // 1-in-30 spawn chance
    rng_update(engine, r);
    if (r.value != 0) {
        // Roll failed: leave the slot empty this frame.
        r.index = r.value;
        return;
    }
    r.index = 0;
    let mut scratch_offset: i32 = 3; // copy 4 position bytes (3..0)
    let mut source_slot_offset: i32 = 3; // default body anchor in slot 0
    if ((engine.state.object_attr(0) & crate::bits::BIT6) != 0) {
        // Slot 0 is H-flipped: read from the mirrored anchor.
        source_slot_offset = 19;
    }
    // Copy the 4 spawn-position bytes into the pursuer's scratch ($99+offset).
    loop {
        engine.state.set_inventory_item(
            153 + scratch_offset, // $99 base + offset
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
    // Reset motion state and pull combat stats from the room record.
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    engine.state.obj_health = ((engine.state.byte((actor_data_ptr + 4) as u16 as i32)) as u8);
    engine.state.obj_damage = ((engine.state.byte((actor_data_ptr + 5) as u16 as i32)) as u8);
    engine.state.obj_state = 1; // active
    engine.state.obj_tile = 129; // pursuer sprite tile
    r.value = 4; // random attribute in [0,3]
    rng_update(engine, r);
    engine.state.obj_attr = (r.value as u8);
    engine.state.obj_cooldown = 128; // initial cooldown before it acts
    r.offset = (source_slot_offset as u8);
    r.index = (scratch_offset as u8);
}

// Original 6502 jump-table of behavior-handler addresses, indexed by the
// behavior id in actor record byte 8. The Rust port dispatches via the match
// below; these addresses are only mirrored into the indirect-pointer scratch
// ($0E/$0F) so traces line up with the original ROM.
const ACTOR_BEHAVIOR_HANDLERS: [i32; 9] = [
    0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
];

/// Dispatches the behavior id stored at room actor record byte 8 to the
/// matching `tick_*` handler. Ids >= 9 fall back to behavior 0. The original
/// handler address is mirrored into `indirect_ptr` (`0x0E/0x0F`) for
/// trace-compatible scratch.
pub fn dispatch_actor_behavior(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    let mut behavior_id: i32 = engine.state.byte((actor_data_ptr + 8) as u16 as i32); // record +8
    if (behavior_id >= 9) {
        // Out of range: default to behavior 0.
        behavior_id = 0;
    }
    // Mirror the original handler pointer into scratch (trace compatibility).
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

/// Generic non-boss actor tick: keep existing movement going, try terrain
/// response, expire the actor when its lifetime timer reaches zero, then
/// update the terrain probe for the next frame.
///
/// Phase 0 advances motion: an in-progress jump arc (`obj_move_scratch != 0`)
/// or, otherwise, a queued jump (cooldown) followed by gravity. Each motion
/// helper sets carry to mean "blocked, position already handled". Phase 1
/// ages the actor: at timer 0 it despawns (state 0, timer 240). In the last
/// 60 ticks it swaps the live Y with the stashed off-screen Y (`obj_y_extra`)
/// to make the sprite flash on/off as it expires.
pub fn tick_standard_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut actor_timer: i32 = 0;
    let mut saved_tile_y: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_scratch == 0) {
                    // No jump arc active.
                    if (engine.state.obj_cooldown == 0) {
                        // Nothing queued: skip straight to aging.
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    // Try a queued jump; carry set => handled, go to aging.
                    try_actor_jump_arc_motion(engine, r);
                    if ((r.carry) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    commit_actor_projected_position(engine, r);
                }
                // Apply gravity; carry set => blocked/landed, go to aging.
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
                    // Lifetime over: despawn and arm respawn timer.
                    engine.state.obj_state = 0;
                    engine.state.obj_timer = 240; // respawn delay
                    r.index = (actor_timer as u8);
                    return;
                }
                engine.state.obj_timer = (actor_timer as u8);
                if (actor_timer < 60) {
                    // Final 60 ticks: blink by swapping live Y with the
                    // stashed Y (239 = off-screen) each frame.
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

/// Wanders horizontally, occasionally starts a jump arc, then falls under
/// the shared gravity helper until terrain accepts the projected position.
/// Behavior id 0 (the default handler).
///
/// Phase 0 decides whether to keep the current motion or pick a new one. A
/// new direction is chosen randomly (cardinal), with a random speed (1..6)
/// and a 1-in-4 chance of arming the jump bit (`0x80`). Phases 1-3 run the
/// jump-arc / terrain-move / commit path; phase 4 updates the terrain probe
/// and animation and restores the saved horizontal speed.
pub fn tick_wandering_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut saved_tile_dx: i32 = 0;
    let mut keep_existing_motion: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Keep current motion if recently turned (timer>=32), still
                // cooling down, or already moving on either axis.
                if (engine.state.obj_timer >= 32) {
                } else if (engine.state.obj_cooldown != 0) {
                    keep_existing_motion = 1;
                } else if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) != 0) {
                    keep_existing_motion = 1;
                }
                if ((keep_existing_motion) == 0) {
                    // Pick a fresh random direction, speed, and maybe a jump.
                    engine.state.obj_timer = 0;
                    choose_random_cardinal_actor_direction(engine, r);
                    r.value = 6; // random speed factor in [0,5]
                    rng_update(engine, r);
                    engine.state.obj_x_vel_hi = ((r.value + 1) as u8); // speed 1..6
                    r.value = 4; // 1-in-4 jump chance
                    rng_update(engine, r);
                    r.index = r.value;
                    if (r.value == 0) {
                        engine.state.obj_move_state = 128 | engine.state.obj_move_state; // arm jump bit
                    }
                }
                saved_tile_dx = (engine.state.obj_x_vel_hi as i32); // preserve speed across helpers
                r.offset = (engine.state.obj_x_vel_hi as u8);
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    // Jump arc in progress: keep falling under gravity.
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
                    // Still cooling down: go (re)start a jump arc.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::BIT7) as u8)) == 0) {
                    // No jump armed: just walk along terrain.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Start/continue a jump arc; carry clear => committed, animate.
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
                // Walk: move with terrain collision. Blocked => stop.
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
                engine.state.obj_x_vel_hi = (saved_tile_dx as u8); // restore saved speed
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Chooses a random direction when stationary, then moves without terrain
/// collision. Bounds/player contact can stop the motion. Behavior id 1.
/// Used by free-floating actors (e.g. flying enemies) that ignore walls.
/// Speed comes from room actor record byte 9.
pub fn tick_random_floating_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        // Stationary: pick a new random direction.
        choose_random_actor_direction(engine, r);
    }
    {
        // Build a velocity vector from direction bits + record speed (+9).
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        let mut speed: i32 = engine.state.byte((actor_data_ptr + 9) as u16 as i32);
        r.offset = (speed as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        // Blocked (bounds/player): halt.
        stop_actor_motion(engine, r);
    } else {
        commit_actor_projected_position(engine, r);
    }
    update_actor_animation(engine, r);
}

/// Walks along terrain ledges: blocked movement stops motion, supported
/// projections commit, and unsupported projections fall through gravity.
/// Behavior id 2. Speed comes from room actor record byte 9.
///
/// When stationary it reverses horizontal direction (paces back and forth).
/// While mid-fall (`obj_move_scratch != 0`) it just applies gravity. While
/// walking it moves with terrain collision, then probes the tile under the
/// next step (offsets 1 and 13 = the two foot positions); if there is no
/// floor it stops rather than walking off the ledge.
pub fn tick_ledge_walking_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut should_commit_position: i32 = 0;
    let mut should_stop_motion: i32 = 0;
    let mut skip_resolution: i32 = 0;
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        // Stopped at an edge/wall: turn around.
        reverse_actor_horizontal_direction(engine, r);
    }
    if (engine.state.obj_move_scratch != 0) {
        // Falling: gravity only.
        try_actor_gravity_motion(engine, r);
        if (r.carry == 0) {
            should_commit_position = 1;
        } else {
            skip_resolution = 1; // helper already resolved position
        }
    } else {
        // Walking: build velocity from record speed (+9) and direction.
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
        try_move_actor_with_terrain(engine, r);
        if ((r.carry) != 0) {
            should_stop_motion = 1; // hit a wall
        } else {
            // Ledge check: probe the floor under each foot before stepping.
            r.offset = 1; // first foot tile offset
            probe_object_solid_tile(engine, r);
            if (r.carry == 0) {
                should_stop_motion = 1; // no floor: don't walk off
            } else if (engine.state.indirect_ptr_lo == 0) {
                should_commit_position = 1; // aligned, floor present
            } else {
                r.offset = 13; // second foot tile offset
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

/// Re-aims toward the player, marks the direction as jump-capable with
/// `0x80`, then uses the same jump/gravity movement path as wanderers.
/// Behavior id 3. Speed comes from room actor record byte 9.
///
/// Phase 0 decides when to re-aim: if already moving, it keeps going unless
/// the turn timer has expired; if stopped and standing on a non-solid floor
/// tile it falls. The low 2 bits of `obj_move_state` are the horizontal
/// direction, which it flips when the turn timer hits zero. After aiming it
/// arms the jump bit and runs the shared jump/gravity/terrain path
/// (phases 2-6), identical in structure to `tick_wandering_jump_actor`.
pub fn tick_chasing_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Drop the jump bit; keep only the 4 direction bits.
                engine.state.obj_move_state =
                    engine.state.obj_move_state & ((crate::bits::LOW_NIBBLE) as u8);
                if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) != 0) {
                    // Already moving.
                    if (engine.state.obj_timer < 16) {
                        // Recently turned: just keep current motion.
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 1; // time to re-aim at the player
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_x_sub == 0) {
                    // Tile-aligned & stopped: check the floor under both feet.
                    let mut room_tile_ptr: i32 = 0;
                    engine.state.data_ptr_lo = engine.state.obj_x_tile;
                    engine.state.data_ptr_hi = engine.state.obj_y_pixel;
                    resolve_room_tile_pointer(engine, r);
                    room_tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
                    if ((engine.state.byte(room_tile_ptr) & crate::bits::LOW_6_BITS) == 0) {
                        // No floor tile here: fall (re-aim path).
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if ((engine.state.byte((room_tile_ptr + 1) as u16 as i32)
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
                    // No horizontal direction set: default to left (bit0).
                    engine.state.obj_move_state = 1;
                }
                {
                    // Turn timer expired => flip horizontal direction.
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
                            engine.state.obj_move_state ^ ((crate::bits::LOW_2_BITS) as u8); // swap L/R
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                }
                // Aim at the player and arm a jump.
                aim_actor_toward_player(engine, r);
                engine.state.obj_move_state = 128 | engine.state.obj_move_state; // jump bit
                {
                    state = 2;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Re-aim: reset turn timer and point at the player.
                engine.state.obj_timer = 0;
                aim_actor_toward_player(engine, r);
                state = 2;
                continue 'dispatch;
            }
            2 => {
                {
                    // Build velocity from record speed (+9) and direction bits.
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
                }
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    // Mid-jump: apply gravity.
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
                    // No jump armed: walk along terrain.
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                // Start a jump arc; carry clear => committed.
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
                // Walk with terrain collision; blocked => stop.
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

/// Aims from player overlap, moves without terrain, and asks the velocity
/// reflection helper to bounce away when blocked. Behavior id 4. Speed comes
/// from room actor record byte 9.
///
/// Keeps its current direction while it is moving and the turn timer is below
/// 32; otherwise re-aims at the player. When the move is blocked it tries to
/// reflect (bounce) its velocity, only stopping if reflection also fails.
pub fn tick_reflecting_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
    // Keep going if already moving and recently turned (timer < 32).
    let mut keep_current_direction: i32 = ((((engine.state.obj_x_vel_lo | engine.state.obj_y_vel)
        != 0)
        && (engine.state.obj_timer < 32)) as u8 as i32);
    if ((keep_current_direction) == 0) {
        aim_actor_from_player_overlap(engine, r);
    }
    {
        // Build velocity from record speed (+9) and direction bits.
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        // Blocked: try to bounce off the obstacle.
        try_reflect_object_velocity(engine, r);
        if ((r.carry) != 0) {
            // Couldn't reflect: halt.
            stop_actor_motion(engine, r);
            update_actor_animation(engine, r);
            return;
        }
    }
    commit_actor_projected_position(engine, r);
    update_actor_animation(engine, r);
}

/// Alternates between overhead probing, falling, and a jump arc. This is the
/// only normal behavior that asks `probe_actor_overhead_step` before moving.
/// Behavior id 5. Speed comes from room actor record byte 9.
///
/// Phase 0: if not already falling/cooling, probe the tile overhead; if blocked
/// (ceiling) it walks (phase 1), otherwise it begins a fall (phase 3, with the
/// fall counter bumped). Phase 1 walks with terrain collision and ledge/overhead
/// probing. Phase 3 applies gravity, and once it lands it sets a cooldown based
/// on how far it fell before its next action.
pub fn tick_overhead_probe_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_scratch != 0) {
                    // Already falling: continue gravity.
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_cooldown != 0) {
                    // Cooling down after a fall: do a jump arc.
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                // Probe the tile directly overhead from the current position.
                engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
                engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
                engine.state.scratch2 = engine.state.obj_y_pixel;
                probe_actor_overhead_step(engine, r);
                if ((r.carry) != 0) {
                    // Ceiling above: walk instead of rising.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Open above: start falling (bump fall counter by 2).
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
                    // Stopped: turn around.
                    reverse_actor_horizontal_direction(engine, r);
                }
                check_player_x_overlap(engine, r);
                if ((r.carry) != 0) {
                    // Lined up with player horizontally: stop and reconsider.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                {
                    // Build velocity from record speed (+9) and direction.
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
                }
                r.value = (engine.state.obj_move_state as u8);
                build_direction_velocity(engine, r);
                try_move_actor_with_terrain(engine, r);
                if ((r.carry) != 0) {
                    // Wall: stop.
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                }
                probe_actor_overhead_step(engine, r);
                if ((r.carry) == 0) {
                    // No ceiling at new spot: stop (would expose to a drop).
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
                // Halt horizontal motion and re-probe terrain.
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
                // Falling: apply gravity and commit.
                try_actor_gravity_motion(engine, r);
                commit_actor_projected_position(engine, r);
                {
                    let mut saved_fall_counter: i32 = (engine.state.obj_move_scratch as i32);
                    update_object_terrain_probe(engine, r);
                    if ((r.carry) == 0) {
                        // Still falling.
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    // Landed: set cooldown = fall distance + 6 before next action.
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
                // Jump arc after cooldown.
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

/// Sits inert until the player overlaps a one-step projection in any
/// cardinal direction, then switches into the chasing jump behavior.
/// Behavior id 6 (a trap/ambush enemy).
///
/// Once `obj_move_state` is nonzero it has been triggered and simply forwards
/// to `tick_chasing_jump_actor` forever after. While untriggered it probes
/// each of the four cardinal directions (bits 1/2/4/8); a player overlap in
/// any one wakes it (phase 1 sets direction bit0). If nothing touches it, it
/// refreshes its health from the record and stays dormant.
pub fn tick_contact_trigger_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                if (engine.state.obj_move_state != 0) {
                    // Already triggered: behave like a chasing-jump enemy.
                    tick_chasing_jump_actor(engine, r);
                    return;
                }
                // Probe left/right/up/down (direction bits 1,2,4,8).
                r.value = 1; // left
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 2; // right
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 4; // up
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                r.value = 8; // down
                check_actor_direction_contact(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                {
                    // Untouched: refresh health from record (+4), stay dormant.
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    let mut actor_type: i32 =
                        engine.state.byte((actor_data_ptr + 4) as u16 as i32);
                    engine.state.obj_health = (actor_type as u8);
                    r.value = 0;
                    engine.state.obj_y_extra = 0;
                }
                return;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Triggered: set a horizontal direction so the chase begins.
                r.value = 1;
                engine.state.obj_move_state = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Projects a one-step move in the direction bits in `r.value` and reports
/// whether the projected position overlaps the player; if it does, applies
/// the actor's contact damage and returns carry set. Helper for
/// `tick_contact_trigger_actor`.
pub fn check_actor_direction_contact(engine: &mut Engine, r: &mut RoutineContext) {
    r.offset = 1; // one-tile step
    build_direction_velocity(engine, r);
    project_actor_position(engine, r);
    check_player_overlap(engine, r);
    if (r.carry == 0) {
        // No contact: leave carry clear.
        return;
    }
    apply_actor_player_contact_damage(engine, r);
    r.carry = 1; // signal contact to the caller
}

/// Random floating behavior that turns into a high-bit/contact recoil state
/// when movement was blocked specifically by player overlap. Behavior id 7.
/// Speed comes from room actor record byte 9.
///
/// Floats in a random direction like `tick_random_floating_actor`, but when a
/// move is blocked it checks `overlap_flag`: if the block was the player, it
/// flips to the defeated/recoil state (`obj_state = 0x80`) instead of merely
/// stopping. Used by enemies that die or recoil on contact.
pub fn tick_contact_recoil_actor(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
        // Stationary: pick a random direction.
        choose_random_actor_direction(engine, r);
    }
    {
        // Build velocity from record speed (+9) and direction bits.
        let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
        r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
        r.value = (engine.state.obj_move_state as u8);
        build_direction_velocity(engine, r);
    }
    try_move_actor_without_terrain(engine, r);
    if ((r.carry) != 0) {
        // Blocked.
        if (engine.state.overlap_flag != 0) {
            // Blocked by the player: enter recoil/defeated state (bit7).
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

/// Chases for a limited time (cooldown counts down from `0xF1`). Behavior id
/// 8. Speed comes from room actor record byte 9.
///
/// Each frame it re-aims at the player via `aim_actor_from_player_overlap`,
/// but once it already has a direction it rejects abrupt multi-axis turns:
/// it compares the new direction bits against the old ones and, if more than
/// one of the four direction bits changed, keeps the old direction. This
/// damps jittery diagonal flip-flopping. When the cooldown reaches zero the
/// actor despawns (state 0).
pub fn tick_timed_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut chase_timer: i32 = ((engine.state.obj_cooldown - 1) as u8 as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                engine.state.obj_cooldown = (chase_timer as u8);
                if (chase_timer == 0) {
                    // Chase time up: despawn.
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_move_state == 0) {
                    // No direction yet: aim freely.
                    aim_actor_from_player_overlap(engine, r);
                } else {
                    if (engine.state.obj_timer >= 8) {
                        // Settled for >=8 frames: allow re-aim, but reject it
                        // if it would change more than one direction bit.
                        let mut direction_diff: i32 = 0;
                        let mut bit_count: i32 = 0;
                        let mut changed_bits: i32 = 0;
                        engine.state.scratch0 = engine.state.obj_move_state; // save old direction
                        aim_actor_from_player_overlap(engine, r);
                        direction_diff =
                            ((engine.state.obj_move_state ^ engine.state.scratch0) as u8 as i32);
                        // Count how many of the 4 direction bits changed.
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
                            // changed_bits-1 != 0 means it wasn't a single-bit
                            // change, so revert to the old direction.
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
                    // Build velocity from record speed (+9) and direction.
                    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
                    r.offset = ((engine.state.byte((actor_data_ptr + 9) as u16 as i32)) as u8);
                    r.value = (engine.state.obj_move_state as u8);
                    build_direction_velocity(engine, r);
                }
                try_move_actor_without_terrain(engine, r);
                if ((r.carry) != 0) {
                    // Blocked: despawn.
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
                engine.state.obj_state = 0; // despawn
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Probes the projected tile one row above the actor when the projected Y
/// position is tile-aligned. Carry is left from the solid-tile probe (set =>
/// the overhead tile(s) are solid). Inputs: `indirect_ptr_hi` = X-tile,
/// `scratch2` = Y-pixel of the projected position.
///
/// Only meaningful on a tile boundary (low nibble of Y == 0); otherwise it
/// returns immediately leaving carry unchanged. Checks the tile 16px above at
/// the left edge (offset 0) and, if the actor straddles two columns
/// (`indirect_ptr_lo != 0`), also the right edge (offset 12).
pub fn probe_actor_overhead_step(engine: &mut Engine, r: &mut RoutineContext) {
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) != 0) {
        // Not tile-aligned vertically: nothing to probe ($EDF0 BNE $EE17 CLC).
        r.carry = 0;
        return;
    }
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2 - 16; // one tile (16px) up
    resolve_room_tile_pointer(engine, r);
    r.offset = 0; // left-edge tile
    probe_projected_solid_tile(engine, r);
    if (r.carry == 0) {
        return;
    }
    if (engine.state.indirect_ptr_lo == 0) {
        // Column-aligned: left-edge result is the answer.
        return;
    }
    r.offset = 12; // right-edge tile (actor spans two columns)
    probe_projected_solid_tile(engine, r);
    if (r.carry == 0) {
        return;
    }
}

/// Sets direction bits in `obj_move_state` (`0xF4`) so an actor tends toward
/// the player. Room actor data byte 9 (when nonzero) allows an occasional
/// upward bias when the actor is below the player.
///
/// Horizontal: compute dx = actor.x_tile - player.x_tile (16-bit, so a borrow
/// shows as bit8). If dx != 0 set bit0 (move left); if dx is non-negative
/// (actor is right of player) also set bit1 => value 2 (move right). Vertical:
/// dy = actor.y - player.y; if the actor is above the player (dy non-negative)
/// and vertical bias is enabled, a 1-in-3 roll sets the jump bit (`0x80`). If
/// the actor is below the player, a 1-in-3 roll forces a pure "up" direction.
pub fn aim_actor_toward_player(engine: &mut Engine, r: &mut RoutineContext) {
    let mut direction_bits: i32 = 0;
    let mut dx: i32 = (((engine.state.obj_x_tile as u16 as i32)
        - (engine.state.player_x_tile as i32)) as u16 as i32);
    if ((dx as u8 as i32) != 0) {
        {
            direction_bits += 1; // bit0 = horizontal movement (left)
            direction_bits
        };
        if ((dx & crate::bits::BIT8) == 0) {
            // No borrow => actor is right of player => bit1 = move right.
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
            // Actor at/above player: maybe arm a jump (vertical bias).
            let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
            let mut vertical_bias_enabled: i32 =
                engine.state.byte((actor_data_ptr + 9) as u16 as i32);
            if (vertical_bias_enabled != 0) {
                r.value = 3; // 1-in-3 chance
                rng_update(engine, r);
                r.index = r.value;
                if (r.index == 0) {
                    engine.state.obj_move_state =
                        engine.state.obj_move_state | ((crate::bits::BIT7) as u8); // jump bit
                }
            }
        } else {
            // Actor below player: 1-in-3 chance to move straight up (bit2).
            r.value = 3;
            rng_update(engine, r);
            r.index = r.value;
            if (r.index == 0) {
                engine.state.obj_move_state = 4; // direction = up
            }
        }
    }
}

/// Builds direction bits in `obj_move_state` by checking whether the actor
/// already overlaps the player on each axis. Unlike `aim_actor_toward_player`,
/// an axis the actor already overlaps contributes no movement on that axis
/// (so it stops sliding once aligned).
///
/// X axis: if not overlapping, set bit0 (left) when the actor is left of the
/// player, or bit1 (right) when it is at/right of the player. Y axis: if not
/// overlapping, set bit2 (up) when above, or bit3 (down) when below. Resets
/// the turn timer to 0 so the caller knows it just re-aimed.
pub fn aim_actor_from_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut direction_bits: i32 = 0;
    // Seed the overlap probe with the actor's current position.
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    check_player_x_overlap(engine, r);
    direction_bits = 0;
    if (r.carry == 0) {
        // Not aligned horizontally: aim left (1) or right (2).
        let mut actor_is_right_of_player: i32 =
            ((if (engine.state.obj_x_tile >= engine.state.player_x_tile) {
                1
            } else {
                0
            }) as u8 as i32);
        direction_bits = 1; // default: move left
        if ((actor_is_right_of_player) != 0) {
            direction_bits = 2; // actor is right of player: move right
        }
    }
    engine.state.obj_move_state = (direction_bits as u8);
    check_player_y_overlap(engine, r);
    direction_bits = 0;
    if (r.carry == 0) {
        // Not aligned vertically: aim up (4) or down (8).
        let mut actor_is_below_player: i32 =
            ((if (engine.state.obj_y_pixel >= engine.state.player_y) {
                1
            } else {
                0
            }) as u8 as i32);
        direction_bits = 4; // default: move up
        if ((actor_is_below_player) != 0) {
            direction_bits = 8; // actor is below player: move down
        }
    }
    // Combine the X and Y direction bits.
    engine.state.obj_move_state = ((direction_bits | (engine.state.obj_move_state as i32)) as u8);
    engine.state.obj_timer = 0; // mark "just re-aimed"
}

/// Flips the actor's horizontal heading (left<->right) in `obj_move_state`.
///
/// Reads the low two move-state bits (bit0 = move right, bit1 = move left),
/// defaults an idle actor to "right" if neither is set, then XORs with 3 to
/// swap the two bits. The new move state is stored and also returned in
/// `r.value` (A).
pub fn reverse_actor_horizontal_direction(engine: &mut Engine, r: &mut RoutineContext) {
    // Isolate the horizontal heading bits (bit0=right, bit1=left).
    let mut direction_bits: i32 =
        ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) as i32);
    // An actor with no horizontal heading is treated as moving right.
    if (direction_bits == 0) {
        direction_bits = 1;
    }
    // XOR with 0b11 swaps the right (bit0) and left (bit1) heading bits.
    direction_bits ^= 3;
    engine.state.obj_move_state = (direction_bits as u8);
    r.value = (direction_bits as u8);
}

/// Picks one of the eight direction-bit patterns from the ROM move-state
/// table at `OBJ_MOVE_STATE_TABLE` (0xEEB3) and stores it as the actor's
/// `obj_move_state`.
///
/// `rng_update` is asked for a value in `[0, 8)` (modulus passed in A), then
/// that index reads the corresponding 8-direction bit pattern.
pub fn choose_random_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
    // Roll a random index in 0..8 (one entry per compass direction).
    r.value = 8;
    rng_update(engine, r);
    r.index = r.value;
    // Look up the direction-bit pattern for that index in the ROM table.
    engine.state.obj_move_state = ((engine
        .state
        .byte((OBJ_MOVE_STATE_TABLE + (r.index as i32)) as u16 as i32))
        as u8);
}

// Eight direction-bit patterns (bit0=right, bit1=left, bit2=up, bit3=down);
// indices 0/2/4/6 are the pure cardinal headings reached below.
const DIRECTION_LOOKUP: [i32; 8] = [1, 5, 4, 6, 2, 10, 8, 9];

/// Picks one of the four cardinal headings used by wandering actors.
///
/// `rng_update` returns a value in `[0, 3)`; doubling it (`<< 1`) selects one
/// of the even slots {0,2,4,6} of `DIRECTION_LOOKUP`, i.e. a pure
/// right/up/left direction, which becomes the actor's `obj_move_state`.
pub fn choose_random_cardinal_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
    // Roll a random value in 0..3.
    r.value = 3;
    rng_update(engine, r);
    // Double it to land on an even (pure-cardinal) table slot: 0, 2, or 4.
    let direction_index: i32 = (((r.value as i32) << 1) as u8 as i32);
    engine.state.obj_move_state = ((DIRECTION_LOOKUP[direction_index as usize]) as u8);
}

/// Advances a falling actor under gravity. If the diagonal move is blocked,
/// horizontal velocity is dropped and the straight-down move is retried;
/// if that is also blocked the actor stops falling.
///
/// Fall speed grows with the terrain-probe counter `obj_move_scratch` (0xF0):
/// downward velocity = `(counter >> 1) + 2`. Returns early (carry clear) once
/// a move succeeds.
pub fn try_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Downward speed accelerates as the fall counter rises; +2 is the minimum.
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 1) + 2;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Diagonal fall blocked: drop horizontal velocity and try falling straight down.
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Straight-down move also blocked: the actor has landed, cancel fall speed.
    engine.state.obj_y_vel = 0;
}

/// Advances an actor along its jump arc, using the cooldown byte `obj_cooldown`
/// (0xF1) as a jump-phase countdown.
///
/// The counter starts at 15 and counts down each frame; upward velocity is the
/// two's-complement negation of `counter >> 1`, so the actor rises fast early
/// in the jump and decelerates near the apex. If the diagonal move is blocked
/// it retries straight up; if that fails the counter is restored and the actor
/// attempts to reflect its velocity off the obstacle.
pub fn try_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Restart the jump countdown at 15 frames when it has run out.
    let mut jump_counter: i32 = (engine.state.obj_cooldown as i32);
    if (jump_counter == 0) {
        jump_counter = 15;
    }
    jump_counter = ((jump_counter - 1) as u8 as i32);
    engine.state.obj_cooldown = (jump_counter as u8);
    r.index = (jump_counter as u8);
    // Upward velocity = -(counter >> 1) as a signed byte (XOR 0xFF then +1).
    engine.state.obj_y_vel = ((((jump_counter >> 1) ^ crate::bits::BYTE_MASK) + 1) as u8);
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Diagonal jump blocked: drop horizontal speed and try moving straight up.
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Both moves blocked: undo this frame's countdown and bounce off the wall.
    engine.state.obj_cooldown = engine.state.obj_cooldown + 1;
    try_reflect_object_velocity(engine, r);
}

/// Commits the projected position (zero-page scratch `0x0E/0x0F` = x subtile/
/// tile and `0x0A` = y pixel) back into the live actor coordinates after a
/// movement attempt succeeds. Leaves the new y pixel in `r.value` (A).
pub fn commit_actor_projected_position(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy projected x (subtile + tile) and y back to the actor's position.
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    r.value = (engine.state.scratch2 as u8);
}

/// Brings an actor to a full stop: clears horizontal/vertical velocity, the
/// jump-arc cooldown (0xF1), and the terrain-probe/fall counter (0xF0).
pub fn stop_actor_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Zero velocity components and both motion counters.
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_y_vel = 0;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
}

/// Projects the actor's current position (`obj_x_sub`/`obj_x_tile` = 0xF9/0xFA
/// and `obj_y_pixel` = 0xFB) one frame forward through its velocity
/// (`obj_y_vel`, `obj_x_vel_lo`/`obj_x_vel_hi`), leaving the candidate
/// position in zero-page scratch `0x0E/0x0F` (x subtile/tile) and `0x0A`
/// (y pixel) without committing it.
///
/// The x coordinate is fixed-point: low nibble = subtile (0..15), high byte =
/// tile. Adding the signed x velocity may carry into the tile byte.
pub fn project_actor_position(engine: &mut Engine, r: &mut RoutineContext) {
    let mut subtile_dx: i32 = 0;
    let mut subtile_sum: i32 = 0;
    let mut tile_carry: i32 = 0;
    // Seed the projection with the actor's current position.
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.scratch2 = engine.state.obj_y_pixel;
    // Apply vertical velocity (y is a single pixel byte) when nonzero.
    if (engine.state.obj_y_vel != 0) {
        engine.state.scratch2 = engine.state.obj_y_vel + engine.state.scratch2;
    }
    // Apply horizontal velocity to the fixed-point x coordinate.
    subtile_dx = (engine.state.obj_x_vel_lo as i32);
    if (subtile_dx != 0) {
        // Add the subtile delta; keep the low nibble as the new subtile.
        subtile_sum = ((subtile_dx + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((subtile_sum & crate::bits::LOW_NIBBLE) as u8);
        // Bit 4 of the sum is the carry from subtile into the tile column.
        tile_carry = (((subtile_sum >> 4) & 1) as u8 as i32);
        engine.state.indirect_ptr_hi =
            engine.state.indirect_ptr_hi + engine.state.obj_x_vel_hi + (tile_carry as u8);
    }
}

// Original ROM addresses of the four actor animation handlers, indexed by the
// 2-bit animation mode; mirrored here so the C/Rust port can set the original
// indirect-jump pointer before dispatching to the equivalent function.
const ANIMATION_HANDLERS: [i32; 4] = [0xF03B, 0xF04B, 0xF071, 0xF0B9];

/// Dispatches the actor's animation routine based on the 2-bit animation mode
/// stored in room actor-record byte 7. Reproduces the original indirect jump:
/// it loads the matching ROM handler address into the indirect pointer and
/// then calls the equivalent Rust routine (flip / walk / directional walk /
/// tile cycle).
pub fn update_actor_animation(engine: &mut Engine, r: &mut RoutineContext) {
    // Byte 7 of the actor record holds the animation mode in its low 2 bits.
    let mut actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    let mut animation_id: i32 = ((engine.state.byte((actor_data_ptr + 7) as u16 as i32)
        & crate::bits::LOW_2_BITS) as u8 as i32);
    // Reconstruct the original handler's indirect-jump pointer (lo/hi bytes).
    let mut original_handler: i32 = ANIMATION_HANDLERS[animation_id as usize];
    engine.state.indirect_ptr_lo = ((original_handler & crate::bits::BYTE_MASK) as u8);
    engine.state.indirect_ptr_hi = ((original_handler >> 8) as u8);
    // X/A carry the doubled mode index (original table-of-words convention).
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

/// Animation mode 0: flips the actor's sprite horizontally every four frames.
///
/// Advances the actor frame timer and, once every 4 ticks (low 2 bits == 0),
/// toggles the horizontal-flip bit (bit6) of the sprite attribute `obj_attr`.
/// Leaves the resulting value in `r.value` (A).
pub fn animate_actor_flip_toggle(engine: &mut Engine, r: &mut RoutineContext) {
    let mut animation_phase: i32 = 0;
    // Advance the frame timer (wrap at 256).
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    // Phase 0 occurs once every 4 frames; only then do we flip.
    animation_phase = ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) as i32);
    if (animation_phase == 0) {
        // Toggle the horizontal-flip attribute bit (bit6).
        animation_phase = ((engine.state.obj_attr ^ ((crate::bits::BIT6) as u8)) as i32);
        engine.state.obj_attr = (animation_phase as u8);
    }
    r.value = (animation_phase as u8);
}

/// Animation mode 1: sets the sprite's horizontal facing from velocity and
/// toggles its walk frame every four ticks.
///
/// When moving horizontally, the flip bit (bit6) of `obj_attr` is set for
/// rightward motion (x velocity high byte non-negative) and cleared for
/// leftward motion (high byte negative, i.e. bit7 set). The walk frame is the
/// tile bit2 of `obj_tile`, toggled once per 4 frames.
pub fn animate_actor_walk_toggle(engine: &mut Engine, r: &mut RoutineContext) {
    // Update facing only while moving horizontally.
    if (engine.state.obj_x_vel_lo != 0) {
        // High-byte bit7 set => velocity negative (left) => no flip; else flip (bit6=64).
        let mut facing_bit: i32 =
            (if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
                0
            } else {
                64
            });
        engine.state.scratch0 = (facing_bit as u8);
        // Replace the flip bit (bit6) while preserving the other attribute bits.
        engine.state.obj_attr =
            (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
    }
    // Advance the frame timer and flip the walk-frame tile bit (bit2) every 4 frames.
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
        engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
    }
}

/// Animation mode 2: like `animate_actor_walk_toggle`, but selects a separate
/// set of vertical-movement frames so climbing/falling looks different from
/// horizontal walking.
///
/// Horizontal motion sets the flip bit (bit6) and clears tile bit3 (horizontal
/// frame set). Pure vertical motion sets tile bit3 (vertical frame set). Every
/// 4 ticks the animation alternates: when in the vertical set (bit3) it toggles
/// the flip bit (bit6) for an up/down swap; otherwise it toggles the walk frame
/// (tile bit2).
pub fn animate_actor_directional_walk(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_x_vel_lo != 0) {
        // Moving horizontally: set facing flip bit (bit6) from velocity sign.
        let mut facing_bit: i32 =
            (if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
                0
            } else {
                64
            });
        engine.state.scratch0 = (facing_bit as u8);
        engine.state.obj_attr =
            (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
        // Select the horizontal frame set by clearing tile bit3.
        engine.state.obj_tile = engine.state.obj_tile & ((crate::bits::CLEAR_BIT3) as u8);
    } else {
        // Pure vertical motion: select the vertical frame set (clear bits2-3, set bit3).
        if (engine.state.obj_y_vel != 0) {
            engine.state.obj_tile = (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8))
                | ((crate::bits::BIT3) as u8);
        }
    }
    // Advance frame timer; alternate the animation once every 4 frames.
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
        if ((engine.state.obj_tile & ((crate::bits::BIT3) as u8)) != 0) {
            // Vertical frame set: toggle facing flip (bit6) for the up/down swap.
            engine.state.obj_attr = engine.state.obj_attr ^ ((crate::bits::BIT6) as u8);
        } else {
            // Horizontal frame set: toggle the walk-frame tile bit (bit2).
            engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
        }
    }
}

/// Animation mode 3: cycles through four sprite-tile frames driven by the
/// frame timer.
///
/// Horizontal facing is set from velocity (flip bit6) as in the other modes.
/// The two-frame counter comes from timer bits1-2, shifted up by one so it
/// lands on tile bits2-3, giving a 0/4/8/0xC cycle that walks through four
/// adjacent tiles.
pub fn animate_actor_cycle_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Update horizontal facing flip bit (bit6) while moving horizontally.
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
    // Advance the frame timer.
    engine.state.obj_timer = (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
    // Derive frame tile bits from timer bits1-2 shifted into tile bits2-3.
    let animation_tile_bits: i32 =
        (((engine.state.obj_timer & ((crate::bits::BITS_1_2) as u8)) << 1) as u8 as i32);
    engine.state.scratch0 = (animation_tile_bits as u8);
    // Replace tile bits2-3 with the current frame index.
    engine.state.obj_tile = (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8))
        | (animation_tile_bits as u8);
}

/// Attempts to move the actor one frame, honoring terrain. Returns carry set
/// (`r.carry != 0`) when the move was ultimately blocked, carry clear when it
/// succeeded and the projected position is left in scratch ready to commit.
///
/// Each iteration: projects the move, despawns the actor if it left the room,
/// applies contact damage when an active actor overlaps the player, then tests
/// projected terrain. On a terrain collision it "shaves" vertical velocity
/// toward zero (magnitude -2 toward 0, biased by +1) and retries, so a falling
/// actor can slide down to rest against the floor. The original vertical
/// velocity is restored before returning so the caller's velocity is intact.
pub fn try_move_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    // Preserve the caller's vertical velocity; it is mutated during retries.
    let mut saved_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
    let mut blocked: i32 = 0;
    loop {
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            // Left the room: despawn the actor (state 0) with a 240-frame timer.
            engine.state.obj_state = 0;
            engine.state.obj_timer = 240;
            blocked = 1;
            break;
        }
        // Only active actors (state == 1) can hit the player.
        if (((engine.state.obj_state - 1) as u8 as i32) == 0) {
            check_player_overlap(engine, r);
            if ((r.carry) != 0) {
                apply_actor_player_contact_damage(engine, r);
            }
        }
        check_projected_terrain_collision(engine, r);
        if (r.carry == 0) {
            // Path is clear: success.
            blocked = 0;
            break;
        }
        {
            // Terrain blocked: shave vertical velocity toward zero and retry.
            let mut adjusted_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
            // For downward (positive) velocity, reduce magnitude by 2 first.
            if ((adjusted_vertical_velocity & crate::bits::BIT7) == 0) {
                adjusted_vertical_velocity = ((adjusted_vertical_velocity - 2) as u8 as i32);
            }
            // Step one unit toward zero (works for both up and down velocity).
            adjusted_vertical_velocity = ((adjusted_vertical_velocity + 1) as u8 as i32);
            engine.state.obj_y_vel = (adjusted_vertical_velocity as u8);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
        }
    }
    // Restore the original vertical velocity and report blocked status in carry.
    engine.state.obj_y_vel = (saved_vertical_velocity as u8);
    r.carry = (blocked as u8);
}

/// Moves an actor that ignores terrain (e.g. flying/ghost types) one frame.
/// Still applies player contact damage and despawns the actor if it leaves the
/// room. Carry is set when the actor either hit the player or went out of
/// bounds; carry clear means the move succeeded normally.
pub fn try_move_actor_without_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    project_actor_position(engine, r);
    // Player overlap: deal contact damage and report blocked (carry set).
    check_player_overlap(engine, r);
    if ((r.carry) != 0) {
        apply_actor_player_contact_damage(engine, r);
        r.carry = 1;
        return;
    }
    // No overlap: a clear move returns carry clear.
    check_position_out_of_bounds(engine, r);
    if (r.carry == 0) {
        return;
    }
    // Left the room: despawn the actor (state 0) with a 240-frame timer.
    engine.state.obj_state = 0;
    engine.state.obj_timer = 240;
}

/// Applies an actor's contact damage to the player, unless suppressed.
///
/// Bails out when the player is in post-hit invulnerability
/// (`sprite_blink_timer != 0`) or the actor is not active (state != 1).
/// Two contexts then gate the hit:
/// - In one context (CHR bank slot 3 >= 48): if the player has a slot active
///   and the selected item is item id 10 (a shield-like item), the hit is
///   absorbed and only a prompt is raised.
/// - Otherwise: character index 4 (a special character) is immune.
/// On a successful hit it subtracts `obj_damage` health, fires the hit
/// prompt/sound, starts the invulnerability blink, and clears the actor's
/// bit5 attribute.
pub fn apply_actor_player_contact_damage(engine: &mut Engine, r: &mut RoutineContext) {
    // Player is currently invulnerable (blinking after a hit): no damage.
    if (engine.state.sprite_blink_timer != 0) {
        return;
    }
    // Only an active actor (state == 1) deals contact damage.
    if (((engine.state.obj_state - 1) as u8 as i32) != 0) {
        return;
    }
    if (engine.state.chr_bank(3) >= 48) {
        // CHR bank context where a held shield item (id 10) can block the hit.
        if (engine.state.slot_index != 0) {
            let mut selected_item_slot: i32 = (engine.state.selected_item_slot as i32);
            if (engine.state.item_slot(selected_item_slot) == 10) {
                engine.state.prompt_state = 1;
                return;
            }
        }
    } else {
        // Special character index 4 is immune to contact damage.
        if (engine.state.character_index == 4) {
            return;
        }
    }
    // Deal the actor's damage and trigger the hit feedback.
    r.value = (engine.state.obj_damage as u8);
    subtract_health_points(engine, r);
    engine.state.prompt_state = 33; // hit prompt / sound id
    engine.state.prompt_argument = 1;
    engine.state.sprite_blink_timer = 1; // begin post-hit invulnerability blink
    engine.state.obj_attr = engine.state.obj_attr & ((crate::bits::CLEAR_BIT5) as u8);
}

/// Records that a terrain probe found the path clear: increments the object's
/// fall/probe counter `obj_move_scratch` (0xF0) and clears carry to signal
/// "no obstruction".
fn mark_probe_clear(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
    r.carry = 0;
}

/// Shared "object rests / lands" tail for the terrain probes ($F1D3 / $F223):
/// if the fall counter reached 0x0C, latch the jump cooldown to counter-4, then
/// reset the fall counter and report carry set (supported). Branches that the
/// 6502 routed to this tail were mistranslated as bare `return;`, dropping both
/// stores and the carry result.
fn object_probe_rest(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.obj_move_scratch >= 0x0C {
        engine.state.obj_cooldown = engine.state.obj_move_scratch - 4;
    }
    engine.state.obj_move_scratch = 0;
    r.carry = 1;
}

/// Updates the normal one-tile-wide terrain probe for the current object.
///
/// While the object is mid-jump (`obj_cooldown != 0`) the probe is skipped.
/// Otherwise it samples the tile(s) directly below the object's footprint:
/// for an active actor (state 1) it first checks for player overlap and bails
/// near the room floor (y >= 176); for inactive objects it remaps the sentinel
/// y 239 to the stored extra y. When the sampled tiles are all clear it calls
/// `mark_probe_clear`, advancing the fall counter `0xF0` and clearing carry.
/// A solid tile or any early bail leaves the counter unchanged (object rests).
pub fn update_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
    // Mid-jump objects do not run the floor probe ($F17B BNE $F1D3 rest tail).
    if (engine.state.obj_cooldown != 0) {
        object_probe_rest(engine, r);
        return;
    }
    // Seed the tile pointer with the object's tile-column / subtile position.
    engine.state.data_ptr_lo = engine.state.obj_x_tile;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    let mut tile_y: i32 = (engine.state.obj_y_pixel as i32);
    let active_state: i32 = ((engine.state.obj_state - 1) as u8 as i32);
    if (active_state == 0) {
        // Active actor: past the playfield floor row (176 px) keep falling.
        if (tile_y >= 176) {
            mark_probe_clear(engine, r); // $F19B BCS $F1CF
            return;
        }
        engine.state.data_ptr_hi = (tile_y as u8);
        // Probe the row one pixel below; rest if it overlaps the player.
        tile_y = ((tile_y + 1) as u8 as i32);
        engine.state.scratch2 = (tile_y as u8);
        check_player_overlap(engine, r);
        if ((r.carry) != 0) {
            object_probe_rest(engine, r); // $F1A5 BCS $F1D3
            return;
        }
    } else {
        // Inactive object: y 239 is a sentinel meaning "use the stored extra y".
        if (tile_y == 239) {
            tile_y = (engine.state.obj_y_extra as i32);
        }
        engine.state.data_ptr_hi = (tile_y as u8);
    }
    resolve_room_tile_pointer(engine, r);
    // When tile-aligned (no subtile offset), an empty tile (low 6 bits clear)
    // under either half of the footprint rests the object.
    if (engine.state.obj_x_sub == 0) {
        let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
        if ((engine.state.byte(tile_ptr) & crate::bits::LOW_6_BITS) == 0) {
            object_probe_rest(engine, r); // $F1B4 BEQ $F1D3
            return;
        }
        if ((engine.state.byte((tile_ptr + 1) as u16 as i32) & crate::bits::LOW_6_BITS) == 0) {
            object_probe_rest(engine, r); // $F1BB BEQ $F1D3
            return;
        }
    }
    // Probe the tile one row below (offset 1 = next map row in this layout).
    r.offset = 1;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        object_probe_rest(engine, r); // $F1C2 BCS $F1D3
        return;
    }
    // Tile-aligned objects only need that single column ($F1C6 BEQ $F1CF).
    if (engine.state.obj_x_sub == 0) {
        mark_probe_clear(engine, r);
        return;
    }
    // Straddling a tile boundary: also probe the adjacent column (offset 13).
    r.offset = 13;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        object_probe_rest(engine, r); // $F1CD BCS $F1D3
        return;
    }
    // Nothing solid below: the object keeps falling ($F1CF fall-through).
    mark_probe_clear(engine, r);
}

/// Updates the wider (two-tile-wide) terrain probe used by large objects.
///
/// Like `update_object_terrain_probe` but samples the wide footprint below the
/// object. Skipped while mid-jump (`obj_cooldown != 0`). Near the lower screen
/// (y >= 160) it simply advances the fall counter. Otherwise it checks for a
/// wide player overlap and probes the row beneath the left and right columns
/// (offsets 2 and 14), plus a third column (offset 26) when the object
/// straddles a tile boundary. With nothing solid below, the fall counter
/// `obj_move_scratch` (0xF0) advances so the object keeps falling.
pub fn update_wide_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
    // Mid-jump objects do not run the floor probe ($F1E6 BNE $F223 rest tail).
    if (engine.state.obj_cooldown != 0) {
        object_probe_rest(engine, r);
        return;
    }
    // Seed the tile pointer with the object's position; probe one row below.
    engine.state.data_ptr_lo = engine.state.obj_x_tile;
    engine.state.indirect_ptr_hi = engine.state.obj_x_tile;
    engine.state.indirect_ptr_lo = engine.state.obj_x_sub;
    engine.state.data_ptr_hi = engine.state.obj_y_pixel;
    engine.state.scratch2 = engine.state.obj_y_pixel + 1;
    resolve_room_tile_pointer(engine, r);
    // Below the bottom of the playfield (160 px): keep falling ($F200 BCS $F220
    // INC $F0; RTS — note this path does not touch carry).
    if (engine.state.obj_y_pixel >= 160) {
        engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
        return;
    }
    // Wide player overlap rests the object.
    check_player_overlap_wide(engine, r);
    if ((r.carry) != 0) {
        object_probe_rest(engine, r); // $F205 BCS $F223
        return;
    }
    // Probe the row below the left column (offset 2).
    r.offset = 2;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        object_probe_rest(engine, r); // $F20C BCS $F223
        return;
    }
    // Probe the row below the right column (offset 14).
    r.offset = 14;
    probe_object_solid_tile(engine, r);
    if ((r.carry) != 0) {
        object_probe_rest(engine, r); // $F213 BCS $F223
        return;
    }
    // Straddling a boundary: probe the third overlapping column (offset 26).
    if (engine.state.obj_x_sub != 0) {
        r.offset = 26;
        probe_object_solid_tile(engine, r);
        if ((r.carry) != 0) {
            object_probe_rest(engine, r); // $F21E BCS $F223
            return;
        }
    }
    // Nothing solid below the wide footprint: keep falling ($F220 INC $F0).
    engine.state.obj_move_scratch = engine.state.obj_move_scratch + 1;
}

/// Probes the room tile at `data_ptr + r.offset` (Y). Reads its low six tile
/// bits and sets carry when the tile id is in the solid range (>= 0x30), i.e.
/// a wall/floor the object cannot pass through.
pub fn probe_object_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    // Tile id = low 6 bits of the map byte at data_ptr + offset.
    let tile_id: i32 = ((engine
        .state
        .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS) as u8 as i32);
    // Tile ids 0x30 (48) and above are solid terrain.
    r.carry = (((tile_id >= 48) as u8) as u8);
}

/// Tests the projected one-tile-wide object footprint (x tile `0x0F`, x subtile
/// `0x0E`, y `0x0A`) for collision with solid terrain.
///
/// Probes up to four tile cells covering the destination footprint: the top
/// cell, the cell to its right (only when the subtile x is nonzero, i.e. the
/// footprint straddles two columns), then the bottom row cells (skipped when
/// the destination is below the floor at y 176 or aligned to a tile row). Any
/// solid tile leaves carry set; if every probe is clear carry is explicitly
/// cleared to report a free move.
pub fn check_projected_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
    // Build the map tile pointer from the projected position.
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);
    // Top-left cell.
    r.offset = 0;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    // Top-right cell, only when straddling two tile columns.
    if (engine.state.indirect_ptr_lo != 0) {
        r.offset = 12;
        probe_projected_solid_tile(engine, r);
        if ((r.carry) != 0) {
            return;
        }
    }
    // Below the floor row (176 px): no bottom cells to test.
    if (engine.state.scratch2 >= 176) {
        return;
    }
    // Row-aligned (no vertical subpixel): no bottom row to test.
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    // Bottom-left cell (offset 1 = next map row).
    r.offset = 1;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    // Bottom-right cell, only when straddling two columns.
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    r.offset = 13;
    probe_projected_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    // All probed cells clear: free move.
    r.carry = 0;
}

/// Convenience wrapper: probes the projected footprint cell at `tile_offset`
/// (loaded into Y) and returns the resulting carry (1 = solid) as an `i32`,
/// keeping the wide-collision check below compact.
fn probe(engine: &mut Engine, r: &mut RoutineContext, tile_offset: i32) -> i32 {
    r.offset = (tile_offset as u8);
    probe_projected_solid_tile(engine, r);
    return (r.carry as i32);
}

/// Tests the projected two-tile-wide object footprint (x tile `0x0F`, x subtile
/// `0x0E`, y `0x0A`) for collision with solid terrain.
///
/// Wide analogue of `check_projected_terrain_collision`: it probes the top row
/// across both columns (offsets 0,1 and adjacent 12,13), an extra pair (24,25)
/// when straddling a tile boundary, then the bottom row (2,14,26) unless the
/// destination is below the floor (y 176) or row-aligned. Any solid cell returns
/// with carry set; an all-clear path explicitly clears carry.
pub fn check_projected_wide_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
    // Build the map tile pointer from the projected position.
    engine.state.data_ptr_lo = engine.state.indirect_ptr_hi;
    engine.state.data_ptr_hi = engine.state.scratch2;
    resolve_room_tile_pointer(engine, r);
    // Top row: two columns of the wide footprint plus their right neighbors.
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
    // Extra column pair when the footprint straddles a tile boundary.
    if (engine.state.indirect_ptr_lo != 0) {
        if ((probe(engine, r, 24)) != 0) {
            return;
        }
        if ((probe(engine, r, 25)) != 0) {
            return;
        }
    }
    // Below the floor row (176 px): skip the bottom row.
    if (engine.state.scratch2 >= 176) {
        return;
    }
    // Row-aligned: no bottom row to test.
    if ((engine.state.scratch2 & ((crate::bits::LOW_NIBBLE) as u8)) == 0) {
        return;
    }
    // Bottom row of the wide footprint (offsets 2 and 14 = next map row).
    if ((probe(engine, r, 2)) != 0) {
        return;
    }
    if ((probe(engine, r, 14)) != 0) {
        return;
    }
    // Extra bottom column only when straddling a tile boundary.
    if (engine.state.indirect_ptr_lo == 0) {
        return;
    }
    if ((probe(engine, r, 26)) != 0) {
        return;
    }
    // Every probed cell clear: free move.
    r.carry = 0;
}

/// Probes a projected footprint tile at `data_ptr + r.offset` (Y). Identical
/// logic to `probe_object_solid_tile`: carry is set when the tile id (low six
/// bits) is in the solid range (>= 0x30).
pub fn probe_projected_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
    // Tile id = low 6 bits of the map byte at data_ptr + offset.
    let tile_id: i32 = ((engine
        .state
        .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS) as u8 as i32);
    // Tile ids 0x30 (48) and above are solid terrain.
    r.carry = (((tile_id >= 48) as u8) as u8);
}

/// Attempts to deflect a blocked object so it slides along the wall it hit,
/// turning its blocked horizontal motion into vertical motion (or vice versa)
/// based on which subtile edge it is nearest, then re-runs movement
/// validation. Carry remains set when no deflection was possible.
///
/// The logic is a flattened state machine (`state`): state 0 decides the
/// deflection, state 1 retries the move via `try_move_actor_with_terrain` and
/// returns, state 2 reports failure (carry = 1). It first clears the high x
/// velocity byte. If the object had horizontal velocity, it converts to
/// vertical: nearest the top edge of its tile (subpixel y nibble < 6) it nudges
/// up (y vel 0xFF = -1), nearest the bottom (nibble >= 11) it nudges down
/// (y vel 1), provided the corresponding move-state bit doesn't forbid it.
/// Otherwise (it had vertical velocity) it symmetrically converts to a small
/// horizontal nudge based on the subtile x position.
pub fn try_reflect_object_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let mut edge_nibble: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Always clear the high x velocity byte first.
                engine.state.obj_x_vel_hi = 0;
                if (engine.state.obj_x_vel_lo != 0) {
                    // Was moving horizontally: drop x speed, deflect vertically.
                    engine.state.obj_x_vel_lo = 0;
                    // Vertical subtile position within the tile (0..15).
                    edge_nibble =
                        ((engine.state.obj_y_pixel & ((crate::bits::LOW_NIBBLE) as u8)) as i32);
                    if (edge_nibble == 0) {
                        // Exactly tile-aligned: nothing to deflect toward.
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if (edge_nibble < 6) {
                        // Near the top edge: nudge upward unless up (bit2) is blocked.
                        if ((engine.state.obj_move_state & ((crate::bits::BIT2) as u8)) != 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.state.obj_y_vel = 255; // -1: move up one pixel
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if (edge_nibble >= 11) {
                        // Near the bottom edge: nudge down unless down (bit3) is blocked.
                        if ((engine.state.obj_move_state & ((crate::bits::BIT3) as u8)) != 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.state.obj_y_vel = 1; // +1: move down one pixel
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    // Mid-tile: no clear edge to slide toward.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (engine.state.obj_y_vel == 0) {
                    // Neither horizontal nor vertical velocity: cannot deflect.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Was moving vertically: drop y speed, deflect horizontally.
                engine.state.obj_y_vel = 0;
                // Horizontal subtile position within the tile (0..15).
                edge_nibble = (engine.state.obj_x_sub as i32);
                if (edge_nibble == 0) {
                    // Exactly tile-aligned: nothing to deflect toward.
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                if (edge_nibble < 6) {
                    // Near the left edge: nudge left unless right (bit0) is blocked.
                    if ((engine.state.obj_move_state & ((crate::bits::BIT0) as u8)) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    // x velocity 0xFF:0x0F = -1 in fixed-point (move left).
                    engine.state.obj_x_vel_lo = 15;
                    engine.state.obj_x_vel_hi = 255;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                if (edge_nibble >= 11) {
                    // Near the right edge: nudge right unless left (bit1) is blocked.
                    if ((engine.state.obj_move_state & ((crate::bits::BIT1) as u8)) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    // x velocity 0x00:0x01 = +1 subtile (move right).
                    engine.state.obj_x_vel_lo = 1;
                    engine.state.obj_x_vel_hi = 0;
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Mid-tile: no clear edge to slide toward.
                {
                    state = 2;
                    continue 'dispatch;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Re-run the move with the deflected velocity and return.
                try_move_actor_with_terrain(engine, r);
                return;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // No deflection possible: report still-blocked.
                r.carry = 1;
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Initializes the special large (boss-sized) actor slot from its room actor
/// record.
///
/// Large actors occupy slot `0x0400` as their logical/damage anchor and slots
/// `0x0410`, `0x0420`, `0x0430` as the four linked body pieces forming a 2x2
/// sprite. Spawn position comes from record bytes 2 (x tile) and 3 (y pixel);
/// the routine rejects a blocked spawn (wide terrain collision) before seeding
/// position, motion counters, sprite tile/attr, damage (byte 5) and health
/// (byte 4) across the anchor and the three body pieces. Finally it loads the
/// large-actor OAM template and the alternate health-meter tiles via the
/// banked-asset helper.
pub fn initialize_large_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
    let actor_data_ptr: i32 = ((engine.state.actor_record_ptr()) as u16 as i32);
    engine.state.set_chr_bank(4, 61); // CHR bank holding the large-actor graphics
    // Candidate spawn: record byte 3 = y pixel, byte 2 = x tile, subtile 0.
    engine.state.scratch2 = ((engine.state.byte((actor_data_ptr + 3) as u16 as i32)) as u8);
    engine.state.indirect_ptr_hi =
        ((engine.state.byte((actor_data_ptr + 2) as u16 as i32)) as u8);
    engine.state.indirect_ptr_lo = 0;
    engine.state.scratch3 = 0;
    // Abort the spawn if the wide footprint would overlap solid terrain.
    check_projected_wide_terrain_collision(engine, r);
    if ((r.carry) != 0) {
        return;
    }
    // Commit position and reset motion state for a freshly spawned actor.
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
    engine.state.obj_cooldown = 0;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_move_state = 0;
    engine.state.obj_state = 1; // active
    engine.state.obj_tile = 129; // initial body base tile id
    engine.state.obj_attr = 2; // initial sprite attribute (palette 2)
    engine.state.obj_damage = ((engine.state.byte((actor_data_ptr + 5) as u16 as i32)) as u8);
    {
        // Record byte 4 = starting health, copied to anchor + 3 body pieces.
        let actor_health: i32 = engine.state.byte((actor_data_ptr + 4) as u16 as i32);
        engine.state.obj_health = (actor_health as u8);
        engine.state.set_object_health(16, actor_health); // slot 0x0410
        engine.state.set_object_health(32, actor_health); // slot 0x0420
        engine.state.set_object_health(48, actor_health); // slot 0x0430
    }
    // Load the OAM sprite template from ROM 0xA7E1 (lo 225, hi 167).
    engine.state.indirect_ptr_lo = 225;
    engine.state.indirect_ptr_hi = 167;
    with_large_actor_asset_banks(engine, r, load_large_actor_oam_template);
    // Build the alternate health-meter tiles from ROM 0xCB53 (lo 83, hi 203).
    engine.state.indirect_ptr_lo = 83;
    engine.state.indirect_ptr_hi = 203;
    with_large_actor_asset_banks(engine, r, build_object_health_meter_alt_tiles);
}

/// Per-frame AI/movement tick for the active large actor (the boss that chases
/// the player).
///
/// Flattened state machine. States 0/1 decide heading: when idle it (re)picks
/// a chase direction, and on a turn-timer expiry either reverses its current
/// horizontal heading or re-aims toward the player (setting bit7 to request a
/// jump). State 2 builds velocity from the heading and chooses the motion path:
/// falling (`obj_move_scratch` set), a jump arc, or plain walking. States 3/4
/// run jump/walk movement with wide terrain collision (stopping on a block),
/// state 5 commits the new position, and state 6 always runs the floor probe,
/// updates facing, and advances the body animation.
pub fn tick_large_chasing_actor(engine: &mut Engine, r: &mut RoutineContext) {
    let mut horizontal_direction: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Keep only the low-nibble direction bits of the move state.
                engine.state.obj_move_state =
                    engine.state.obj_move_state & ((crate::bits::LOW_NIBBLE) as u8);
                if ((engine.state.obj_x_vel_lo | engine.state.obj_y_vel) == 0) {
                    // Currently motionless: ensure it has a horizontal heading.
                    if ((engine.state.obj_move_state & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        engine.state.obj_move_state = 1; // default: move right
                    }
                    {
                        // Count down the turn/decision timer.
                        let mut turn_timer: i32 = (engine.state.obj_timer as i32);
                        engine.state.obj_timer = 0;
                        turn_timer = ((turn_timer - 1) as u8 as i32);
                        if (turn_timer == 0) {
                            // Timer elapsed: reverse the current horizontal heading.
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
                            // No heading to reverse: fall through to re-aim.
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        // Timer still running: aim at the player and request a jump (bit7).
                        aim_actor_toward_player(engine, r);
                        engine.state.obj_move_state = 128 | engine.state.obj_move_state;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                } else {
                    // Already moving: keep going until the timer reaches 50.
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
                // Reset the timer and aim a fresh chase direction at the player.
                engine.state.obj_timer = 0;
                aim_actor_toward_player(engine, r);
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Turn the heading bits into x/y velocity (offset 2 = large-actor speed).
                r.value = (engine.state.obj_move_state as u8);
                r.offset = 2;
                build_direction_velocity(engine, r);
                if (engine.state.obj_move_scratch != 0) {
                    // Fall counter set: apply gravity motion.
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
                // Mid-jump (cooldown set), or jump requested (bit7): jump path.
                if (engine.state.obj_cooldown != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                if ((engine.state.obj_move_state & ((crate::bits::BIT7) as u8)) == 0) {
                    // No jump requested: plain ground movement.
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                state = 3;
                continue 'dispatch;
            }
            3 => {
                // Advance the jump arc; success -> commit, block -> walk fallback.
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
                // Plain horizontal walk; clear cooldown so jump can restart later.
                engine.state.obj_cooldown = 0;
                try_move_large_actor_with_terrain(engine, r);
                if ((r.carry) == 0) {
                    {
                        state = 5;
                        continue 'dispatch;
                    }
                }
                // Blocked: stop dead and skip the position commit.
                stop_actor_motion(engine, r);
                {
                    state = 6;
                    continue 'dispatch;
                }
                state = 5;
                continue 'dispatch;
            }
            5 => {
                // Movement succeeded: write the projected position back.
                commit_actor_projected_position(engine, r);
                state = 6;
                continue 'dispatch;
            }
            6 => {
                // Per-frame upkeep: floor probe, facing, and body animation.
                update_wide_object_terrain_probe(engine, r);
                update_large_actor_facing_from_velocity(engine, r);
                animate_large_actor_body_tiles(engine, r);
                break 'dispatch;
            }
            _ => break 'dispatch,
        }
    }
}

/// Applies the large actor's falling motion under gravity. If the wide
/// diagonal move is blocked it retries straight down; if that is also blocked
/// the actor stops falling.
///
/// Fall speed scales more gently than small actors: velocity =
/// `(obj_move_scratch >> 2) + 1`. Returns early (carry clear) on the first move
/// that succeeds.
pub fn try_large_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Downward speed grows slowly with the fall counter; +1 is the minimum.
    let fall_velocity: i32 = (((engine.state.obj_move_scratch >> 2) + 1) as u8 as i32);
    engine.state.obj_y_vel = (fall_velocity as u8);
    r.value = (fall_velocity as u8);
    try_move_large_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Diagonal fall blocked: drop horizontal velocity and fall straight down.
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    r.value = 0;
    try_move_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Straight-down move also blocked: landed, cancel fall speed.
    engine.state.obj_y_vel = 0;
    r.value = 0;
}

/// Advances the large actor's jump arc, using `obj_cooldown` (0xF1) as the jump
/// countdown, and retries straight-up movement when horizontal motion collides
/// with terrain.
///
/// The counter starts at 25 (a longer, higher arc than small actors) and counts
/// down; upward velocity is the two's-complement negation of `counter >> 2`, so
/// the actor rises fast early and slows near the apex.
pub fn try_large_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
    // Restart the jump countdown at 25 frames when it has run out.
    let mut jump_counter: i32 = (engine.state.obj_cooldown as i32);
    if (jump_counter == 0) {
        jump_counter = 25;
    }
    jump_counter = ((jump_counter - 1) as u8 as i32);
    engine.state.obj_cooldown = (jump_counter as u8);
    r.index = (jump_counter as u8);
    // Upward velocity = -(counter >> 2) as a signed byte (XOR 0xFF then +1).
    engine.state.obj_y_vel = ((((jump_counter >> 2) ^ crate::bits::BYTE_MASK) + 1) as u8);
    try_move_large_actor_with_terrain(engine, r);
    if ((r.carry) == 0) {
        return;
    }
    // Diagonal jump blocked: drop horizontal speed and try straight up.
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_x_vel_hi = 0;
    try_move_large_actor_with_terrain(engine, r);
}

/// Wide-footprint counterpart of `try_move_actor_with_terrain`: moves the large
/// actor one frame, applying wide player-overlap contact damage and rejecting
/// solid terrain via the wide collision check. On a terrain collision it shaves
/// vertical velocity toward zero and retries so the actor can settle against a
/// floor. Carry is set when the move was ultimately blocked, clear on success
/// (projected position left ready to commit).
pub fn try_move_large_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
    // Preserve the caller's vertical velocity; it is mutated during retries.
    let saved_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
    let mut blocked: i32 = 0;
    loop {
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            // Left the room: despawn (state 0) with a 240-frame timer.
            engine.state.obj_state = 0;
            engine.state.obj_timer = 240;
            blocked = 1;
            break;
        }
        // Wide overlap with the player: deal contact damage.
        check_player_overlap_wide(engine, r);
        if ((r.carry) != 0) {
            apply_actor_player_contact_damage(engine, r);
        }
        check_projected_wide_terrain_collision(engine, r);
        if (r.carry == 0) {
            // Path clear: success.
            blocked = 0;
            break;
        }
        {
            // Terrain blocked: shave vertical velocity toward zero and retry.
            let mut adjusted_vertical_velocity: i32 = (engine.state.obj_y_vel as i32);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
            // For downward (positive) velocity, reduce magnitude by 2 first.
            if ((adjusted_vertical_velocity & crate::bits::BIT7) == 0) {
                adjusted_vertical_velocity = ((adjusted_vertical_velocity - 2) as u8 as i32);
            }
            // Step one unit toward zero (works for up and down velocity).
            adjusted_vertical_velocity = ((adjusted_vertical_velocity + 1) as u8 as i32);
            engine.state.obj_y_vel = (adjusted_vertical_velocity as u8);
            if (adjusted_vertical_velocity == 0) {
                blocked = 1;
                break;
            }
        }
    }
    // Restore the original vertical velocity and report blocked status in carry.
    engine.state.obj_y_vel = (saved_vertical_velocity as u8);
    r.carry = (blocked as u8);
}

/// Sets the large actor's horizontal facing (flip bit6 of `obj_attr`) from its
/// x velocity: leftward (high byte negative) clears the flip bit, rightward
/// sets it (64). When there is no horizontal velocity the facing is left
/// unchanged.
pub fn update_large_actor_facing_from_velocity(engine: &mut Engine, r: &mut RoutineContext) {
    let mut facing_bit: i32 = 0;
    if ((engine.state.obj_x_vel_hi & ((crate::bits::BIT7) as u8)) != 0) {
        // x velocity high byte negative => moving left => no flip (facing_bit 0).
    } else if (engine.state.obj_x_vel_lo == 0) {
        // No horizontal motion: keep the current facing.
        return;
    } else {
        // Moving right: set the flip bit (bit6 = 64).
        facing_bit = 64;
    }
    engine.state.scratch0 = (facing_bit as u8);
    // Replace bit6 while preserving the other attribute bits.
    engine.state.obj_attr =
        (engine.state.obj_attr & ((crate::bits::LOW_6_BITS) as u8)) | (facing_bit as u8);
}

/// Advances the large actor's animation timer and derives the base body tile id
/// used as the top-left piece of the 2x2 body (the other pieces are computed
/// from it in `compose_large_actor_body_slots`).
///
/// The animation frame comes from timer bits2-3 shifted into tile bits3-4,
/// OR'd with the fixed base-tile mask `BITS_0_6` (0x41). The id is stored in
/// `obj_tile` and returned in `r.value` (A).
pub fn animate_large_actor_body_tiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Advance the frame timer (wrap at 256).
    let animation_timer: i32 =
        (((engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8)) as i32);
    engine.state.obj_timer = (animation_timer as u8);
    // Frame bits (timer bits2-3) shift up into tile bits3-4, plus the base mask.
    let body_tile_id: i32 =
        ((((animation_timer & crate::bits::BITS_2_3) << 1) | crate::bits::BITS_0_6) as u8 as i32);
    engine.state.obj_tile = (body_tile_id as u8);
    r.value = (body_tile_id as u8);
}

/// Swaps the bytes at object-table addresses `a` and `b`. Used to reorder the
/// large actor's four body-piece sprite ids so the visible 2x2 arrangement
/// stays consistent when the sprite is flipped horizontally/vertically.
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
    // Body pieces share the anchor's extra-y value.
    // Slot offsets: +16 = 0x0410 (top-right), +32 = 0x0420 (bottom-left),
    // +48 = 0x0430 (bottom-right); the anchor at +0 = 0x0400 is the top-left.
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
        // Top row keeps the anchor y; bottom row is 16 px (one tile) below.
        let tile_y: i32 = (engine.state.obj_y_pixel as i32);
        engine.state.set_object_y_pixel(16, tile_y);
        engine.state.set_object_y_pixel(32, tile_y + 16);
        engine.state.set_object_y_pixel(48, tile_y + 16);
    }
    // All four pieces share the anchor's x subtile.
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
        // Left column keeps the anchor x tile; right column is one tile over.
        let tile_x: i32 = (engine.state.obj_x_tile as i32);
        engine.state.set_object_x_tile(32, tile_x); // bottom-left
        engine.state.set_object_x_tile(16, tile_x + 1); // top-right
        engine.state.set_object_x_tile(48, tile_x + 1); // bottom-right
    }
    {
        // Propagate a "killed" state: if any body piece is dead (bit7 set),
        // mark the whole actor (and all pieces) as state 0x80.
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
        // The anchor's displayed health is the minimum across all body pieces.
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
        // Derive the three other body tiles from the top-left base tile:
        // top-right = base | bit2, bottom-right = top-right | bit5,
        // bottom-left = bottom-right with bit2 cleared.
        let body_tile_id: i32 = (engine.state.obj_tile as i32);
        let upper_right_tile: i32 = ((body_tile_id | crate::bits::BIT2) as u8 as i32);
        engine.state.set_object_tile(16, upper_right_tile);
        let lower_right_tile: i32 = ((upper_right_tile | crate::bits::BIT5) as u8 as i32);
        engine.state.set_object_tile(48, lower_right_tile);
        let lower_left_tile: i32 = ((lower_right_tile & crate::bits::CLEAR_BIT2) as u8 as i32);
        engine.state.set_object_tile(32, lower_left_tile);
    }
    {
        // All pieces share the anchor's sprite attribute (palette/flip bits).
        let sprite_attrs: i32 = (engine.state.obj_attr as i32);
        engine.state.set_object_attr(16, sprite_attrs);
        engine.state.set_object_attr(32, sprite_attrs);
        engine.state.set_object_attr(48, sprite_attrs);
        // Horizontal flip (bit6): swap left/right columns so tiles stay correct.
        if ((sprite_attrs & crate::bits::BIT6) != 0) {
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 0, OBJECT_TABLE_BASE + 16);
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 32, OBJECT_TABLE_BASE + 48);
        }
        // Vertical flip (bit7): swap top/bottom rows.
        if ((sprite_attrs & crate::bits::BIT7) != 0) {
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 0, OBJECT_TABLE_BASE + 32);
            swap_slot_sprite_id(engine, OBJECT_TABLE_BASE + 16, OBJECT_TABLE_BASE + 48);
        }
    }
    // Rebuild the alternate health-meter tiles from ROM 0xCB53 (lo 83, hi 203).
    with_large_actor_asset_banks(engine, r, |engine, r| {
        engine.state.indirect_ptr_lo = 83;
        engine.state.indirect_ptr_hi = 203;
        build_object_health_meter_alt_tiles(engine, r);
    });
}

/// Services the pool of player projectile object slots starting at `0x04B0`.
///
/// Iterates `projectile_count` slots (logical indices 11..). For each slot: if
/// the slot is live (its lifetime byte at +1 is nonzero) it advances that shot
/// via `update_player_projectile_slot`; if the slot is free and the fire button
/// (controller bit6) is freshly pressed (not yet latched in `direction_latch`),
/// it spawns a new projectile. The slot pointer advances 16 bytes (one object
/// record) per iteration with manual lo/hi carry.
pub fn update_player_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
    // Start at logical slot 11, object pointer 0x04B0 (lo 176, hi 4).
    engine.state.slot_index = 11;
    engine.state.obj_slot_ptr_lo = 176;
    engine.state.obj_slot_ptr_hi = 4;
    loop {
        let slot_ptr: i32 = ((engine.state.obj_slot_ptr()) as u16 as i32);
        // Byte +1 of the slot is the projectile's remaining lifetime/state.
        let active_lifetime: i32 = engine.state.byte((slot_ptr + 1) as u16 as i32);
        if (active_lifetime != 0) {
            // Live shot: advance it one frame.
            r.value = (active_lifetime as u8);
            r.offset = 1;
            update_player_projectile_slot(engine, r);
        } else {
            // Free slot: spawn only on a fresh fire-button press (bit6 edge).
            if ((engine.state.buttons & ((crate::bits::BIT6) as u8)) != 0) {
                if ((engine.state.direction_latch & ((crate::bits::BIT6) as u8)) == 0) {
                    r.value = 0;
                    r.offset = 1;
                    spawn_player_projectile(engine, r);
                }
            }
        }
        // Advance to the next logical slot and object record.
        engine.state.slot_index = (engine.state.slot_index + 1) & ((crate::bits::BYTE_MASK) as u8);
        {
            // Step the object pointer forward one 16-byte record, carrying into hi.
            let next_slot_lo: i32 = ((16 + engine.state.obj_slot_ptr_lo as i32) as u16 as i32);
            engine.state.obj_slot_ptr_lo = (next_slot_lo as u8);
            engine.state.obj_slot_ptr_hi =
                engine.state.obj_slot_ptr_hi + ((next_slot_lo >> 8) as u8);
        }
        // Continue while the relative slot index is below projectile_count.
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
                // Load the current slot into the working object scratch.
                load_object_slot_scratch(engine, r);
                // Latch the fire button so this press cannot re-fire next frame.
                engine.state.direction_latch = (engine.state.buttons & ((crate::bits::BIT6) as u8))
                    | engine.state.direction_latch;
                // Speed index: 4 when the player is displaced/knocked back, else 2.
                r.offset = ((if (engine.state.displaced_timer != 0) {
                    4
                } else {
                    2
                }) as u8);
                // Direction comes from the latched facing bits.
                r.value = (engine.state.direction_latch as u8);
                build_direction_velocity(engine, r);
                project_player_projectile_position(engine, r);
                // Abort the spawn if the muzzle position is off the playfield.
                check_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Abort if the player has no magic to pay for the base shot.
                consume_magic_point(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Commit the projectile's starting position.
                engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
                engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
                engine.state.obj_y_pixel = engine.state.scratch2;
                // Lifetime depends on equipped item; a longer lifetime costs extra magic.
                load_effective_projectile_lifetime(engine, r);
                engine.state.obj_state = (r.value as u8);
                if (r.carry == 0) {
                    consume_magic_point(engine, r);
                }
                // Damage likewise depends on the item; a stronger shot costs extra magic.
                load_effective_projectile_damage(engine, r);
                engine.state.obj_damage = (r.value as u8);
                if (r.carry == 0) {
                    consume_magic_point(engine, r);
                }
                engine.state.obj_attr = 0;
                engine.state.obj_tile = 33; // base projectile sprite tile
                // Fire the per-character shoot prompt/sound (base id 34 + character).
                engine.state.prompt_state = 34 + engine.state.character_index;
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Finalize: orient the projectile sprite, then write the slot back.
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

/// Commits the projected projectile coordinates (held in the shared
/// `indirect_ptr`/`scratch2` collision registers by
/// `project_player_projectile_position`) back into the object slot's
/// position fields.
fn store_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
    // Copy the projected X subtile/tile and Y pixel into the slot's position.
    engine.state.obj_x_sub = engine.state.indirect_ptr_lo;
    engine.state.obj_x_tile = engine.state.indirect_ptr_hi;
    engine.state.obj_y_pixel = engine.state.scratch2;
}

/// Shared tail used by every exit of `update_player_projectile_slot`: if the
/// projectile is still alive, refresh its sprite direction bits, then write the
/// scratch object back into the live object slot.
fn finish_projectile_slot_update(engine: &mut Engine, r: &mut RoutineContext) {
    // Only living projectiles (nonzero state) need direction bits refreshed.
    if (engine.state.obj_state != 0) {
        apply_projectile_direction_bits(engine, r);
    }
    // Persist the scratch object copy back into its slot in RAM.
    store_object_slot_scratch(engine, r);
}

/// Advances one active player projectile, applying terrain collision,
/// actor hits, damage, and expiry back into the object slot.
///
/// Loads the current slot into scratch, decrements its lifetime counter, then
/// projects its next position and resolves collisions in priority order: out of
/// bounds -> no actor overlap -> special "destructible" actor -> regular
/// damageable actor. Every path funnels through `finish_projectile_slot_update`
/// to write the result back.
pub fn update_player_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
    // Pull the live object slot into the scratch object registers.
    load_object_slot_scratch(engine, r);
    // Tick the projectile's remaining lifetime; expire it when it reaches 0.
    engine.state.obj_state = engine.state.obj_state - 1;
    if (engine.state.obj_state == 0) {
        finish_projectile_slot_update(engine, r);
        return;
    }
    // Compute the projectile's next position into the collision scratch regs.
    project_actor_position(engine, r);
    // Off the playfield -> kill the projectile (state 0) and finish.
    check_position_out_of_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.obj_state = 0;
        finish_projectile_slot_update(engine, r);
        return;
    }
    // No overlap with a damageable actor -> just move it to the new position.
    find_damageable_actor_overlap(engine, r);
    if ((r.carry) == 0) {
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
        return;
    }
    // Special case: when the active CHR bank is >= 48 (a boss/special graphics
    // set) and scratch0 (direction class) >= 4, kill the overlapped actor
    // outright, freeze this projectile (state 1), and raise prompt 12.
    if ((engine.state.chr_bank(3) >= 48) && (engine.state.scratch0 >= 4)) {
        let hit_slot: i32 = (engine.state.scratch1 as i32);
        engine.state.set_object_state(hit_slot, 128); // 0x80 = death/despawn flag
        engine.state.obj_state = 1;
        engine.state.prompt_state = 12; // request sfx/event 12
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
        return;
    }
    {
        // Regular damageable actor hit. The hit slot index is in scratch1.
        let mut hit_slot: i32 = (engine.state.scratch1 as i32);
        // Only actors whose state == 1 (normal/alive) take a hit; others are
        // ignored and the projectile merely advances.
        if (((engine.state.object_state(hit_slot) - 1) as u8 as i32) != 0) {
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        hit_slot = (engine.state.scratch1 as i32);
        {
            // Apply horizontal knockback in the projectile's travel direction:
            // +2 if BIT0 of state (moving right), else -2 (254 = -2 as u8).
            let knockback: i32 = (if ((engine.state.obj_state & ((crate::bits::BIT0) as u8)) != 0) {
                2
            } else {
                254
            });
            engine.state.set_object_y_extra(hit_slot, knockback);
        }
        {
            // Subtract projectile damage from the actor's health, mirroring the
            // new value into the password/health nibble store at 0xE3 + slot.
            let target_health: i32 = engine.state.object_health(hit_slot);
            let projectile_damage: i32 = (engine.state.obj_damage as i32);
            engine.state.set_password_nibbles_a(
                227 + hit_slot, // 0xE3 + slot: per-actor health nibble entry
                ((target_health - projectile_damage) as u8 as i32),
            );
            if (target_health >= projectile_damage) {
                // Survived: request the hit-flash/sfx prompt 6.
                engine.state.prompt_state = 6;
            } else {
                // Killed: set the death flag and zero out its health.
                engine.state.set_object_state(hit_slot, 128); // 0x80 = death flag
                engine.state.set_object_health(hit_slot, 0);
            }
        }
        store_projectile_position(engine, r);
        finish_projectile_slot_update(engine, r);
    }
}

/// Projects player position plus projectile velocity into the shared
/// collision-coordinate scratch registers.
///
/// Seeds `indirect_ptr` (X subtile/tile) and `scratch2` (Y pixel) from the
/// player's current position, then offsets them by the projectile's velocity so
/// downstream collision routines test where the shot will spawn/travel.
pub fn project_player_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
    // Start from the player's fine/tile X and Y position.
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.scratch2 = engine.state.player_y;
    // Apply vertical velocity (Y vel * 4 pixels) to the projected Y.
    if (engine.state.obj_y_vel != 0) {
        let mut a: i32 = (((engine.state.obj_y_vel as i32) << 2) as u8 as i32); // *4
        a = ((a + (engine.state.scratch2 as i32)) as u8 as i32);
        engine.state.scratch2 = (a as u8);
    }
    // Apply horizontal velocity. (X vel_lo * 4) is masked to the low nibble
    // (subtile resolution), added to the fine X, and any carry out of the
    // nibble (bit 4) promotes into the tile column along with vel_hi.
    if (engine.state.obj_x_vel_lo != 0) {
        let projected_subtile: i32 =
            ((((((engine.state.obj_x_vel_lo as i32) << 2) // *4
                & (((crate::bits::LOW_NIBBLE) as u8) as i32)) as u8 as i32)
                + (engine.state.indirect_ptr_lo as i32)) as u8 as i32);
        engine.state.indirect_ptr_lo = ((projected_subtile & crate::bits::LOW_NIBBLE) as u8);
        engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi
            + engine.state.obj_x_vel_hi
            + (((projected_subtile >> 4) & 1) as u8); // nibble carry -> tile
    }
}

/// Copies the projectile direction bits from its lifetime/state byte into
/// the sprite/object descriptor used by later render and collision code.
///
/// Bits 2-3 of `obj_state` encode the facing/travel direction; they are
/// stashed in `scratch0` and merged into the same bit positions of `obj_tile`
/// (the sprite tile/attribute byte). Returns the updated `obj_tile` in A.
pub fn apply_projectile_direction_bits(engine: &mut Engine, r: &mut RoutineContext) {
    // Extract direction bits (bits 2-3) from the state byte.
    let direction_bits: i32 = ((engine.state.obj_state & ((crate::bits::BITS_2_3) as u8)) as i32);
    engine.state.scratch0 = (direction_bits as u8);
    // Replace bits 2-3 of obj_tile with the extracted direction bits.
    engine.state.obj_tile =
        (engine.state.obj_tile & ((crate::bits::CLEAR_BITS_2_3) as u8)) | (direction_bits as u8);
    r.value = (engine.state.obj_tile as u8);
}

/// Updates the singleton tile-removal projectile stored at `0x0490` and
/// restores its saved background tile when it expires.
///
/// This is the "magic"/tile-eating shot occupying object slot 9 (base
/// `0x0490`). It ticks the projectile timer and, while alive, defers motion to
/// `update_tile_projectile_motion`. On expiry it writes the saved background
/// tile (`obj_move_scratch`) back into the room map and, if that tile is on
/// screen, queues a VRAM update for it.
pub fn update_tile_projectile(engine: &mut Engine, r: &mut RoutineContext) {
    // Nothing to do unless slot 9's state byte is set (projectile is active).
    if (engine.state.object_state(144) == 0) {
        // 0x0490 state byte
        return;
    }
    // Point the slot loader at slot 9 (0x0490) and pull it into scratch.
    engine.state.obj_slot_ptr_lo = 144; // 0x90
    engine.state.obj_slot_ptr_hi = 4; // 0x04 -> 0x0490
    load_object_slot_scratch(engine, r);
    // Tick the projectile's travel timer; while nonzero keep moving.
    engine.state.obj_timer = engine.state.obj_timer - 1;
    if (engine.state.obj_timer != 0) {
        update_tile_projectile_motion(engine, r);
        return;
    }
    // Timer hit 0. If the projectile is not yet aligned to a tile boundary
    // (low nibble of Y pixel or any X subtile bits set) and bit0 is clear,
    // give it one more tick so it lands exactly on a tile before resolving.
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
    // Expire the projectile and restore the background tile it had overwritten.
    engine.state.obj_state = 0;
    if (engine.state.obj_move_scratch != 0) {
        // Resolve the room-map address of the tile under the projectile and
        // write the saved original tile id back into it.
        engine.state.data_ptr_lo = engine.state.obj_x_tile;
        engine.state.data_ptr_hi = engine.state.obj_y_pixel;
        resolve_room_tile_pointer(engine, r);
        let tile_ptr: i32 = ((engine.state.data_ptr()) as u16 as i32);
        engine
            .state
            .set_byte(tile_ptr, (engine.state.obj_move_scratch as i32));
        // If the restored tile column is within the visible window (within 17
        // tiles ahead, or just behind: 254/255 wrap), build its VRAM address
        // and queue the bank-09 nametable update.
        let screen_diff: i32 =
            ((engine.state.obj_x_tile - engine.state.scroll_tile_x) as u8 as i32);
        if ((screen_diff < 17) || (screen_diff >= 254)) {
            // 17 tiles visible; 254..=255 = behind by 1-2
            let tile_x: i32 = (engine.state.obj_x_tile as i32);
            engine.state.data_ptr_lo = (tile_x as u8);
            // VRAM column = (tile_x*2) within a nametable, bit4 selects the
            // horizontally adjacent nametable (>>2 into the high byte).
            engine.state.vram_addr_lo = (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
            engine.state.vram_addr_hi =
                (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
            engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
            engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // 0x20xx nametable base
            farcall_bank_09_r7(engine, r);
        }
    }
    store_object_slot_scratch(engine, r);
}

/// Advances the tile-removal projectile, including collision checks,
/// bouncing, contact damage, and final tile replacement.
///
/// State machine: state 0 projects the next position and resolves collisions
/// (out of bounds / terrain / player / actor); state 1 handles player contact
/// damage; state 2 reverses velocity to "bounce"; state 3 checks for tile
/// alignment and, when aligned, restores the saved background tile; state 4
/// writes the scratch object back to its slot.
pub fn update_tile_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // (No work-RAM clear here: the original routine at $F7F7 begins
                // directly with the obj_tile bit0 test below. The previously
                // injected 0x0800..0xA000 clear loop had no 6502 counterpart.)
                // Bit0 of obj_tile means the projectile is in its "resting on a
                // wall" phase: every 4 frames toggle the bit2 animation flag,
                // then skip straight to write-back.
                if ((engine.state.obj_tile & ((crate::bits::BIT0) as u8)) != 0) {
                    if ((engine.state.obj_timer & ((crate::bits::LOW_2_BITS) as u8)) == 0) {
                        // every 4th frame
                        engine.state.obj_tile = engine.state.obj_tile ^ ((crate::bits::BIT2) as u8);
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                }
                // Moving phase: project from slot 9 and test collisions.
                engine.state.slot_index = 9;
                project_actor_position(engine, r);
                // Out of bounds -> bounce (state 2).
                check_actor_position_out_of_bounds(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Hit solid terrain -> bounce (state 2).
                check_projected_terrain_collision(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Overlaps the player -> contact damage (state 1).
                check_player_overlap(engine, r);
                if ((r.carry) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Overlaps a damageable actor -> kill it (death flag 0x80).
                find_damageable_actor_overlap(engine, r);
                if ((r.carry) != 0) {
                    let hit_slot: i32 = (engine.state.scratch1 as i32);
                    engine.state.set_object_state(hit_slot, 128); // 0x80 death flag
                }
                // No blocking collision: commit the projected position and
                // clear the bounce/move sub-state, then write back.
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
                // Player contact. If already bounced this frame, go align/land.
                if (engine.state.obj_move_state != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                // Respect player invulnerability (blink timer running).
                if (engine.state.sprite_blink_timer != 0) {
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                }
                // Deal one point of damage, raise the damage prompt, and start
                // the invulnerability blink window.
                consume_health_point(engine, r);
                engine.state.prompt_state = 10; // damage sfx/event
                engine.state.sprite_blink_timer = 2;
                state = 2;
                continue 'dispatch;
            }
            2 => {
                // Bounce. Skip if we have already bounced this frame.
                if (engine.state.obj_move_state != 0) {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                }
                engine.state.obj_move_state = engine.state.obj_move_state + 1;
                // Negate horizontal velocity (two's complement of the 4-bit
                // fine part, invert the high/sign byte).
                if (engine.state.obj_x_vel_lo != 0) {
                    engine.state.obj_x_vel_lo =
                        (0 - engine.state.obj_x_vel_lo) & ((crate::bits::LOW_NIBBLE) as u8);
                    engine.state.obj_x_vel_hi =
                        engine.state.obj_x_vel_hi ^ ((crate::bits::BYTE_MASK) as u8);
                }
                // Negate vertical velocity (two's complement: ~v + 1).
                engine.state.obj_y_vel = ((((!engine.state.obj_y_vel) as u8 as i32) + 1) as u8);
                // Raise the generic bounce prompt unless one is already pending.
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
                // Landing check after a bounce: if not yet tile-aligned (any
                // low-nibble Y or X subtile bits set), tick the timer and write
                // back so it keeps drifting toward alignment next frame.
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
                // Aligned: expire the projectile and restore its saved tile
                // (same map write + on-screen VRAM update as in
                // `update_tile_projectile`'s expiry path).
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
                            // visible window / behind by 1-2
                            let tile_x: i32 = (engine.state.obj_x_tile as i32);
                            engine.state.data_ptr_lo = (tile_x as u8);
                            // VRAM column = tile_x*2 within nametable; bit4 picks
                            // the adjacent nametable into the high byte.
                            engine.state.vram_addr_lo =
                                (((tile_x << 1) & crate::bits::LOW_5_BITS) as u8);
                            engine.state.vram_addr_hi =
                                (engine.state.obj_x_tile & ((crate::bits::BIT4) as u8)) >> 2;
                            engine.state.vram_addr_lo = 0 + engine.state.vram_addr_lo;
                            engine.state.vram_addr_hi = 32 + engine.state.vram_addr_hi; // 0x20xx base
                            farcall_bank_09_r7(engine, r);
                        }
                    }
                }
                state = 4;
                continue 'dispatch;
            }
            4 => {
                // Persist the scratch object back into slot 9.
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
/// Silences APU pulse channel 1: writes a duty-only volume byte (duty from the
/// channel's shadow byte 6, constant-volume + 0-volume bits 4-5 set) and clears
/// the pulse-1 active bit (bit0) of `sound_status_flags`.
fn silence_pulse1(engine: &mut Engine, _r: &mut RoutineContext) {
    // Preserve the duty bits (top 2) of the shadow byte; set bits 4-5 to mute.
    engine.device_write(
        crate::engine::reg::SQ1_VOL,
        (engine.state.sound_channel_byte(6, 0) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    // Mark pulse 1 inactive.
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT0) as u8);
}

/// Per-frame service for music pulse channel 1 (lane offset 0x00).
///
/// State 0 fetches and plays the next note/rest/command from the channel's
/// stream once the note-duration counter (byte 0) reaches 0; state 1 advances
/// the volume envelope each frame and silences the channel when the envelope
/// terminates. A 0x00 stream byte loops/stops; a 0xFF byte is a control
/// command; the high bit of a note byte marks a rest.
pub fn tick_pulse1_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Channel enable lives in bit7 of shadow byte 1; if off, mute.
                if ((engine.state.sound_channel_byte(1, 0) & crate::bits::BIT7) == 0) {
                    silence_pulse1(engine, r);
                    return;
                }
                // Decrement the note-duration counter (byte 0). While it is
                // still nonzero, only run the envelope (state 1).
                if (((engine.state.dec_sound_channel_byte(0, 0)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                // Duration elapsed: fetch the next stream event(s).
                loop {
                    // Stream pointer = bytes 2 (lo) | 3 (hi).
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 0)
                        | (engine.state.sound_channel_byte(3, 0) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    // 0x00 = end-of-stream: loop or stop, then mute.
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_pulse1(engine, r);
                        return;
                    }
                    // 0xFF = embedded control command; process and re-read.
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    // Store this note's duration (low 7 bits) into byte 0.
                    engine
                        .state
                        .set_sound_channel_byte(0, 0, note_byte & crate::bits::LOW_7_BITS);
                    // Bit7 set -> rest (silent timed envelope); else play note.
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        // Compute the period, mark pulse 1 active, and push the
                        // sweep / period-lo / length-counter+period-hi regs.
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
                        // SQ1_HI: low 3 bits = period high, bits 3-4 set the
                        // length-counter load (and high period bit).
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
                // Skip the envelope entirely if the channel went inactive.
                if ((engine.state.sound_status_flags & ((crate::bits::BIT0) as u8)) == 0) {
                    return;
                }
                // Envelope phase timer (byte 10) hit 0 -> recompute volume.
                if (((engine.state.dec_sound_channel_byte(10, 0)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ1_VOL, (r.value as i32));
                }
                // Advance to the next envelope phase; carry set = envelope done.
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

/// Silences APU pulse channel 2 (music): duty-only volume byte plus mute bits,
/// and clears the pulse-2 active bit (bit1) of `sound_status_flags`.
fn silence_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
    // Keep the duty bits of shadow byte 6 (lane 0x10); set bits 4-5 to mute.
    engine.device_write(
        crate::engine::reg::SQ2_VOL,
        (engine.state.sound_channel_byte(6, 16) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    // Mark pulse 2 inactive.
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT1) as u8);
}

/// Per-frame service for music pulse channel 2 (lane offset 0x10).
///
/// Like `tick_pulse1_channel`, but bit6 of `sound_channel_flags` indicates the
/// sound-effect overlay owns the physical APU channel: when set, the music
/// ticker keeps reading/advancing its stream (to stay in sync) but must not
/// touch the APU registers, so it skips note playback and envelope output.
pub fn tick_pulse2_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_flags: i32 = (engine.state.sound_channel_flags as i32);
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Channel disabled (enable bit7 clear): mute unless the sfx
                // overlay (bit6) currently owns the hardware channel.
                if ((channel_flags & crate::bits::BIT7) == 0) {
                    if ((channel_flags & crate::bits::BIT6) != 0) {
                        return;
                    }
                    silence_pulse2(engine, r);
                    return;
                }
                // Tick note-duration; while nonzero just run the envelope.
                if (((engine.state.dec_sound_channel_byte(0, 16)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                loop {
                    // Stream pointer = bytes 2|3 of lane 0x10.
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 16)
                        | (engine.state.sound_channel_byte(3, 16) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    // End-of-stream: loop/stop then mute.
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_pulse2(engine, r);
                        return;
                    }
                    // Control command, then re-read.
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 16, note_byte & crate::bits::LOW_7_BITS);
                    // Rest note (bit7): start a silent envelope, but if the sfx
                    // overlay owns the channel just keep timing without sound.
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
                    // Audible note but sfx overlay owns the channel: advance the
                    // stream once more and bail without writing the APU.
                    if ((engine.state.sound_channel_flags & ((crate::bits::BIT6) as u8)) != 0) {
                        increment_selected_music_stream_pointer(engine, r);
                        return;
                    }
                    // Normal note: compute period, mark active, push the APU regs.
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
                    // SQ2_HI: low 3 bits period-high | bits 3-4 length load.
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
                // Envelope output is suppressed whenever the sfx overlay (bit6)
                // owns the channel or the channel is no longer active (bit1).
                if ((engine.state.sound_channel_flags & ((crate::bits::BIT6) as u8)) != 0) {
                    return;
                }
                if ((engine.state.sound_status_flags & ((crate::bits::BIT1) as u8)) == 0) {
                    return;
                }
                // Phase timer expired -> recompute and write volume.
                if (((engine.state.dec_sound_channel_byte(10, 16)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ2_VOL, (r.value as i32));
                }
                // Advance envelope phase; carry = envelope terminated.
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

/// Silences the APU triangle channel by clearing its linear-counter register
/// and the triangle active bit (bit2) of `sound_status_flags`. Leaves the
/// flags byte in A.
fn silence_triangle(engine: &mut Engine, r: &mut RoutineContext) {
    r.value = 0;
    // Linear counter = 0 halts the triangle output.
    engine.device_write(crate::engine::reg::TRI_LINEAR, 0);
    // Mark triangle inactive.
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT2) as u8);
    r.value = (engine.state.sound_status_flags as u8);
}

/// Per-frame service for the music triangle channel (lane offset 0x20).
///
/// The triangle has no envelope; it uses a single `triangle_timer` countdown
/// for note duration. When the timer expires it pulls the next stream event:
/// 0x00 loops/stops, 0xFF is a command, the high bit marks a rest, otherwise it
/// loads the note period and triggers the linear/period registers.
pub fn tick_triangle_channel(engine: &mut Engine, r: &mut RoutineContext) {
    // Channel disabled (enable bit7 of shadow byte 1 clear) -> mute.
    if ((engine.state.sound_channel_byte(1, 32) & crate::bits::BIT7) == 0) {
        silence_triangle(engine, r);
        return;
    }
    // Duration counter still running: decrement and exit (note keeps playing).
    if (((engine.state.triangle_timer - 1) as u8 as i32) != 0) {
        engine.state.triangle_timer = engine.state.triangle_timer - 1;
        return;
    }
    // Expired: consume the last tick, then fetch the next stream event.
    engine.state.triangle_timer = engine.state.triangle_timer - 1;
    loop {
        // Stream pointer = bytes 2|3 of lane 0x20.
        let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 32)
            | (engine.state.sound_channel_byte(3, 32) << 8))
            as u16 as i32);
        let mut note_byte: i32 = engine.state.byte(stream_ptr);
        // End-of-stream: loop/stop then mute.
        if (note_byte == 0) {
            rewind_or_stop_audio_stream(engine, r);
            silence_triangle(engine, r);
            return;
        }
        if (note_byte != 255) {
            // A note or rest. Bit7 = rest, low 7 bits = duration.
            let mut is_rest: i32 = ((note_byte & crate::bits::BIT7) as u8 as i32);
            r.value = (note_byte as u8);
            increment_selected_music_stream_pointer(engine, r);
            r.value = ((note_byte & crate::bits::LOW_7_BITS) as u8);
            engine.state.triangle_timer = (r.value as u8);
            // Rest: just leave it silent for the duration.
            if ((is_rest) != 0) {
                silence_triangle(engine, r);
                return;
            }
            // Audible note: compute period, mark active, push the registers.
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
            // TRI_HI: low 3 bits period-high | high 5 bits length-counter load.
            r.value = (((engine.state.sound_length & ((crate::bits::LOW_3_BITS) as u8))
                | ((crate::bits::HIGH_5_BITS) as u8)) as u8);
            engine.device_write(crate::engine::reg::TRI_HI, (r.value as i32));
            return;
        }
        // 0xFF control command, then re-read the stream.
        dispatch_audio_stream_command(engine, r);
    }
}

/// Silences the APU noise channel: writes 0x30 (constant volume, level 0) to
/// the volume register and clears the noise active bit (bit3) of
/// `sound_status_flags`.
fn silence_noise(engine: &mut Engine, _r: &mut RoutineContext) {
    // 0x30 = constant-volume flag + envelope-disable, output level 0.
    engine.device_write(crate::engine::reg::NOISE_VOL, 48);
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT3) as u8);
}

/// Per-frame service for the music noise channel (lane offset 0x30).
///
/// Same stream/envelope structure as the pulse channels, but a "note" writes
/// the noise period (shadow byte 7) and 0x80 length-load instead of a pitch.
pub fn tick_noise_channel(engine: &mut Engine, r: &mut RoutineContext) {
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Channel disabled (enable bit7 of shadow byte 1 clear) -> mute.
                if ((engine.state.sound_channel_byte(1, 48) & crate::bits::BIT7) == 0) {
                    silence_noise(engine, r);
                    return;
                }
                // Note-duration counter still running -> envelope only.
                if (((engine.state.dec_sound_channel_byte(0, 48)) as u8 as i32) != 0) {
                    {
                        state = 1;
                        continue 'dispatch;
                    }
                }
                loop {
                    // Stream pointer = bytes 2|3 of lane 0x30.
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 48)
                        | (engine.state.sound_channel_byte(3, 48) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    // End-of-stream: loop/stop then mute.
                    if (note_byte == 0) {
                        rewind_or_stop_audio_stream(engine, r);
                        silence_noise(engine, r);
                        return;
                    }
                    // Control command, then re-read.
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 48, note_byte & crate::bits::LOW_7_BITS);
                    // Rest (bit7) vs. noise hit.
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        // Mark noise active, set the period (shadow byte 7) and
                        // trigger the length counter (0x80), then start envelope.
                        engine.state.sound_status_flags =
                            engine.state.sound_status_flags | ((crate::bits::BIT3) as u8);
                        engine.device_write(
                            crate::engine::reg::NOISE_LO,
                            engine.state.sound_channel_byte(7, 48),
                        );
                        engine.device_write(crate::engine::reg::NOISE_HI, 128); // length load
                        start_note_envelope(engine, r);
                    }
                    break;
                }
                state = 1;
                continue 'dispatch;
            }
            1 => {
                // Skip envelope if the channel is no longer active.
                if ((engine.state.sound_status_flags & ((crate::bits::BIT3) as u8)) == 0) {
                    return;
                }
                // Phase timer expired -> recompute and write volume.
                if (((engine.state.dec_sound_channel_byte(10, 48)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::NOISE_VOL, (r.value as i32));
                }
                // Advance phase; carry = envelope terminated -> mute.
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

/// Reads the byte currently pointed at by the stream pointer (shadow bytes 2/3)
/// of the channel lane selected by X (`r.index`).
fn deref_stream(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    let mut channel_offset: i32 = (r.index as u8 as i32);
    // 16-bit stream pointer = byte 2 (lo) | byte 3 (hi).
    let mut lo: i32 = engine.state.sound_channel_byte(2, channel_offset);
    let mut hi: i32 = engine.state.sound_channel_byte(3, channel_offset);
    return engine.state.byte((lo | (hi << 8)) as u16 as i32);
}

// A 0xFF stream byte is followed by command id and value bytes. The command
// updates per-channel playback state, then leaves the stream pointer at the
// next note/rest/control byte.
pub fn dispatch_audio_stream_command(engine: &mut Engine, r: &mut RoutineContext) {
    // Select the active channel lane and step over the 0xFF marker.
    r.index = (engine.state.sound_channel_offset as u8);
    increment_selected_music_stream_pointer(engine, r);
    // Read the command id byte.
    {
        let __v = deref_stream(engine, r);
        engine.state.sound_command = (__v as u8);
    }
    increment_selected_music_stream_pointer(engine, r);
    // Read the command argument byte.
    {
        let __v = deref_stream(engine, r);
        engine.state.sound_length = (__v as u8);
    }
    increment_selected_music_stream_pointer(engine, r);
    // Only command ids 0..=4 are defined; anything else is ignored.
    let mut command_id: i32 = (engine.state.sound_command as i32);
    if (command_id >= 5) {
        return;
    }
    // Original 6502 handler addresses preserved for the saved-handler shadow
    // (used by the engine's far-call/return bookkeeping).
    const ORIGINAL_COMMAND_HANDLERS: [i32; 5] = [0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05];
    let mut original_handler: i32 = ORIGINAL_COMMAND_HANDLERS[command_id as usize];
    engine.state.saved_audio_handler_lo = ((original_handler & crate::bits::BYTE_MASK) as u8);
    engine.state.saved_audio_handler_hi = ((original_handler >> 8) as u8);
    // Pass the argument in A and the lane in X to the chosen handler.
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

/// Audio command 0: select pulse duty and envelope "instrument".
///
/// `r.value` (A) is the argument byte: high nibble = pulse duty, low nibble =
/// instrument/envelope id. X holds the channel lane. The duty bits are placed
/// at bits 6-7 of shadow byte 6; the low nibble is scaled to a 16-byte envelope
/// table offset (stored in byte 15) and used to fetch the sustain/sweep value
/// (byte 7) from `SUSTAIN_TABLE`.
pub fn audio_cmd_set_duty_instrument(engine: &mut Engine, r: &mut RoutineContext) {
    let mut command_value: i32 = (r.value as u8 as i32);
    let mut channel_offset: i32 = (r.index as u8 as i32);
    // Duty = high nibble shifted up by 2 to land in bits 6-7.
    let mut duty_bits: i32 =
        ((((command_value & crate::bits::HIGH_NIBBLE) as u8 as i32) << 2) as u8 as i32);
    engine.state.audio_duty_work = (duty_bits as u8);
    // Merge duty into the top 2 bits of shadow byte 6 of the active channel lane
    // ($FBD2 STA $99,X — X = channel_offset, not lane 0).
    engine.state.set_sound_channel_byte(
        6,
        channel_offset,
        (((engine.state.sound_channel_byte(6, channel_offset) & crate::bits::LOW_6_BITS)
            | duty_bits) as u8 as i32),
    );
    // Whole argument << 4 yields the 16-byte-aligned envelope table offset.
    let mut envelope_offset: i32 = ((command_value << 4) as u8 as i32);
    engine
        .state
        .set_sound_channel_byte(15, channel_offset, envelope_offset);
    // Cache the instrument's sustain/sweep byte into shadow byte 7 of the active
    // channel lane ($FBDF STA $9A,X — X = channel_offset, not lane 0).
    engine.state.set_sound_channel_byte(
        7,
        channel_offset,
        engine
            .state
            .byte((SUSTAIN_TABLE + envelope_offset) as u16 as i32),
    );
    // Return the sustain value (A), envelope offset (Y) and lane (X).
    r.value = ((engine
        .state
        .byte((SUSTAIN_TABLE + envelope_offset) as u16 as i32)) as u8);
    r.offset = (envelope_offset as u8);
    r.index = (channel_offset as u8);
}

/// Audio command 1: set the per-channel volume scale (shadow byte 13).
///
/// For music channels (lane != 0x40, the sfx lane), a nonzero
/// `music_volume_override` short-circuits and is returned directly. Otherwise
/// the argument is mapped through `(arg-1) - 8` (clamped at 0) then `*2 + 1` to
/// produce an odd multiplier applied to the raw 0..15 envelope accumulator.
pub fn audio_cmd_set_volume_scale(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (r.index as u8 as i32);
    // Music channels honor a global volume override (sfx lane 0x40 ignores it).
    if !(engine.state.sound_channel_offset == 64) {
        // 0x40 = sfx overlay lane
        let mut music_volume_override: i32 = (engine.state.music_volume_override as i32);
        if (music_volume_override != 0) {
            r.value = (music_volume_override as u8);
            r.index = (channel_offset as u8);
            return;
        }
    }
    {
        // arg + 15 == arg - 1 (mod 256): reduce the 1-based argument by one.
        let mut adjusted_command: i32 = ((15 + engine.state.sound_length) as u8 as i32);
        // Bias by 8 (values < 8 collapse to 0 = silent).
        let mut scale: i32 = if (adjusted_command >= 8) {
            ((adjusted_command - 8) as u8 as i32)
        } else {
            0
        };
        // Make it an odd multiplier: *2 + 1.
        scale = ((scale << 1) as u8 as i32);
        scale = ((scale + 1) as u8 as i32);
        engine
            .state
            .set_sound_channel_byte(13, channel_offset, scale);
        r.value = (scale as u8);
    }
    r.index = (channel_offset as u8);
}

/// Audio command 2: overwrite the channel's duty/volume-flag shadow byte
/// (byte 6, at RAM 0x99 + lane). Argument in A, lane in X.
pub fn audio_cmd_set_channel_flags(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(6, r.index as i32, r.value as i32);
}

/// Audio command 3: store a fine pitch offset (shadow byte 14) that
/// `load_note_period` subtracts from the looked-up note period. Arg in A,
/// lane in X.
pub fn audio_cmd_set_pitch_offset(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(14, r.index as i32, r.value as i32);
}

/// Audio command 4: overwrite the channel's sweep (pulse) / period (noise)
/// shadow byte 7. Arg in A, lane in X.
pub fn audio_cmd_set_sweep_value(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_sound_channel_byte(7, r.index as i32, r.value as i32);
}

/// Decodes a note byte into an 11-bit APU period in `sound_command` (lo) /
/// `sound_length` (hi).
///
/// The note byte's low nibble indexes a 12-entry, 2-byte `NOTE_PERIOD_TABLE`
/// (base pitch), the channel's fine pitch offset (shadow byte 14) is subtracted
/// from it, and the high nibble gives an octave count: each octave shifts the
/// 16-bit period right by one (halving the period = raising one octave). The
/// stream pointer is advanced past the consumed note byte.
pub fn load_note_period(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    // Re-read the note byte from the current stream pointer (bytes 2|3).
    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, channel_offset)
        | (engine.state.sound_channel_byte(3, channel_offset) << 8))
        as u16 as i32);
    let mut note_byte: i32 = engine.state.byte(stream_ptr);
    increment_selected_music_stream_pointer(engine, r);
    {
        // Low nibble * 2 = byte offset into the 2-byte-per-entry pitch table.
        let mut pitch_index: i32 = (((note_byte & crate::bits::LOW_NIBBLE) << 1) as u8 as i32);
        let mut lo: i32 = engine
            .state
            .byte((NOTE_PERIOD_TABLE + pitch_index) as u16 as i32);
        let mut hi: i32 = engine
            .state
            .byte(((NOTE_PERIOD_TABLE + 1) + pitch_index) as u16 as i32);
        channel_offset = (engine.state.sound_channel_offset as i32);
        {
            // Subtract the fine pitch offset (byte 14) from the 16-bit period,
            // borrowing into the high byte on underflow (bit8 of the 9-bit sub).
            let mut sub: i32 = (((lo as u16 as i32)
                - engine.state.sound_channel_byte(14, channel_offset))
                as u16 as i32);
            lo = (sub as u8 as i32);
            if ((sub & crate::bits::BIT8) != 0) {
                hi = ((hi - 1) as u8 as i32);
            }
        }
        {
            // High nibble = octave count; each step halves the period (>>1),
            // carrying the low bit of hi into bit7 of lo.
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
        // Final period: lo -> sound_command (period low), hi -> sound_length.
        engine.state.sound_command = (lo as u8);
        engine.state.sound_length = (hi as u8);
    }
}

/// Scales the raw envelope volume accumulator (`audio_duty_work`) by the volume
/// multiplier passed in Y (`r.offset`), returning the APU's 4-bit volume in A.
///
/// Implements `(accumulator * (Y + 1)) >> 4` via repeated addition (the
/// original 6502 had no multiply). Writes the result back to `audio_duty_work`
/// and clears Y.
pub fn scale_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
    let mut scaled_volume: i32 = 0;
    let mut multiplier: i32 = (((r.offset + 1) as u8) as i32); // Y + 1
    // Repeated addition implements accumulator * multiplier (byte-wrapping).
    loop {
        scaled_volume = ((scaled_volume + (engine.state.audio_duty_work as i32)) as u8 as i32);
        multiplier = ((multiplier - 1) as u8 as i32);
        if (multiplier == 0) {
            break;
        }
    }
    // Divide by 16 to map the product back into the APU's 0..15 volume range.
    scaled_volume >>= 4;
    engine.state.audio_duty_work = (scaled_volume as u8);
    r.value = (scaled_volume as u8);
    r.offset = 0;
}

/// Loads the first envelope phase for an audible note into the channel lane.
///
/// The instrument's base envelope offset is in shadow byte 15. Each envelope
/// phase is a 4-byte record `{delta, ?, duration, ...}` in `ENVELOPE_TABLE`:
/// byte 8 = current phase offset, byte 9 = delta, byte 10 = ?, byte 11 = phase
/// duration, byte 12 = accumulator seed. Returns lane in X, offset in Y.
pub fn start_note_envelope(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    // Instrument's base envelope record offset (shadow byte 15).
    let mut envelope_offset: i32 = engine.state.sound_channel_byte(15, channel_offset);
    // Byte 8 tracks the current phase offset within the envelope table.
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, envelope_offset);
    // Byte 9 = phase delta (+0 of the record).
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte((ENVELOPE_TABLE + envelope_offset) as u16 as i32),
    );
    // Byte 10 = note-tick reload (+1 of the record).
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 1) + envelope_offset) as u16 as i32),
    );
    // Byte 11 = phase duration (+2 of the record).
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 2) + envelope_offset) as u16 as i32),
    );
    // Byte 12 = accumulator seed (+3 of the record).
    engine.state.set_sound_channel_byte(
        12,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 3) + envelope_offset) as u16 as i32),
    );
    r.index = (channel_offset as u8);
    r.offset = (envelope_offset as u8);
}

/// Loads the rest (silent) envelope for the channel lane.
///
/// Rest notes reuse the same per-instrument envelope record but offset by
/// +0x0C (12) bytes, which selects a timed silent phase. Only bytes 8-11 are
/// loaded (no accumulator seed); returns lane in X, offset in Y, byte 11
/// (phase duration) in A.
pub fn start_rest_envelope(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    // Rest envelope lives at the instrument's base + 0x0C.
    let mut rest_envelope_offset: i32 =
        ((engine.state.sound_channel_byte(15, channel_offset) + 12) as u8 as i32);
    // Byte 8 = current phase offset.
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, rest_envelope_offset);
    // Byte 9 = phase delta (+0).
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte((ENVELOPE_TABLE + rest_envelope_offset) as u16 as i32),
    );
    // Byte 10 = note-tick reload (+1).
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 1) + rest_envelope_offset) as u16 as i32),
    );
    // Byte 11 = phase duration (+2).
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 2) + rest_envelope_offset) as u16 as i32),
    );
    r.index = (channel_offset as u8);
    r.offset = (rest_envelope_offset as u8);
    r.value = ((engine
        .state
        .byte(((ENVELOPE_TABLE + 2) + rest_envelope_offset) as u16 as i32)) as u8);
}

/// Handles a 0x00 (end) stream byte: rewind to the channel's saved loop
/// pointer, or stop the channel if no loop is set.
///
/// Shadow bytes 4/5 hold the loop pointer (lo/hi); it is copied into the live
/// stream pointer (bytes 2/3). A nonzero loop-high byte means a real loop
/// address exists, so the duration counter (byte 0) is set to 1 to immediately
/// fetch from the loop point next tick. A zero loop-high means no loop: clear
/// the enable byte (byte 1) keeping only bit6 (sfx-overlay ownership).
pub fn rewind_or_stop_audio_stream(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut loop_pointer_hi: i32 = 0;
    // Copy loop pointer low (byte 4) into the live stream pointer low (byte 2).
    engine.state.set_sound_channel_byte(
        2,
        channel_offset,
        engine.state.sound_channel_byte(4, channel_offset),
    );
    // Copy loop pointer high (byte 5) into stream pointer high (byte 3).
    loop_pointer_hi = engine.state.sound_channel_byte(5, channel_offset);
    engine
        .state
        .set_sound_channel_byte(3, channel_offset, loop_pointer_hi);
    if (loop_pointer_hi != 0) {
        // Valid loop: arm the duration counter to refetch next tick.
        engine.state.set_sound_channel_byte(0, channel_offset, 1);
    } else {
        // No loop: disable the channel but keep bit6 (sfx overlay) intact.
        engine.state.set_sound_channel_byte(
            1,
            channel_offset,
            engine.state.sound_channel_byte(1, channel_offset) & crate::bits::BIT6,
        );
    }
    r.index = (channel_offset as u8);
}

/// Steps the envelope accumulator one tick and composes the APU volume byte
/// (returned in A).
///
/// Adds the phase delta (byte 9) to the accumulator (byte 12). If the delta's
/// sign bit is set (decreasing), underflow past 0 wraps to 0; otherwise
/// overflow past 15 clamps to 15. The clamped 0..15 value is scaled by the
/// channel volume multiplier (byte 13) via `scale_envelope_volume`, then merged
/// with the duty bits (byte 6) and the constant-volume/no-length bits 4-5.
pub fn next_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut envelope_phase: i32 = engine.state.sound_channel_byte(8, channel_offset);
    // Stash the phase's reload byte (+1) into the per-channel work area at
    // 0x9D + lane for the channel ticker.
    engine.state.set_byte(
        ((157 + channel_offset) as u8 as i32), // 0x9D + lane
        engine
            .state
            .byte(((ENVELOPE_TABLE + 1) + envelope_phase) as u16 as i32),
    );
    {
        // Advance the accumulator by the signed phase delta (byte 9).
        let mut envelope_delta: i32 = engine.state.sound_channel_byte(9, channel_offset);
        let mut accumulator: i32 =
            ((envelope_delta + engine.state.sound_channel_byte(12, channel_offset)) as u8 as i32);
        if ((envelope_delta & crate::bits::BIT7) != 0) {
            // Decreasing delta: wrapping past 0 (>=16 after wrap) -> floor at 0.
            if (accumulator >= 16) {
                accumulator = 0;
            }
        } else {
            // Increasing delta: clamp at the 4-bit max of 15.
            if (accumulator >= 16) {
                accumulator = 15;
            }
        }
        engine
            .state
            .set_sound_channel_byte(12, channel_offset, accumulator);
        engine.state.audio_duty_work = (accumulator as u8);
    }
    // Apply the per-channel volume multiplier (byte 13).
    r.offset = ((engine.state.sound_channel_byte(13, channel_offset)) as u8);
    scale_envelope_volume(engine, r);
    {
        // Compose the APU volume register: duty (top 2 bits of byte 6) |
        // scaled volume | constant-volume + length-disable (bits 4-5).
        let mut volume_register: i32 = (((engine.state.sound_channel_byte(6, channel_offset)
            & crate::bits::HIGH_2_BITS)
            | (engine.state.audio_duty_work as i32)
            | crate::bits::BITS_4_5) as u8 as i32);
        r.value = (volume_register as u8);
    }
}

/// Ticks the current envelope phase's duration; when it expires, advances to
/// the next 4-byte phase record. Sets carry when the envelope has terminated
/// (caller then silences the channel).
///
/// Carry semantics: carry=0 means keep playing (phase still running, or a new
/// phase was loaded); carry=1 means the next phase's low-nibble offset reached
/// >= 0x0C, the terminal/silence marker. Returns lane in X.
pub fn advance_envelope_phase(engine: &mut Engine, r: &mut RoutineContext) {
    let mut channel_offset: i32 = (engine.state.sound_channel_offset as i32);
    let mut phase_low_nibble: i32 = 0;
    let mut next_phase: i32 = 0;
    // Decrement the phase duration counter (byte 11, byte-wrapping).
    let phase_timer =
        (engine.state.sound_channel_byte(11, channel_offset) - 1) & crate::bits::BYTE_MASK;
    engine
        .state
        .set_sound_channel_byte(11, channel_offset, phase_timer);
    // Phase still has time left: keep playing.
    if (phase_timer != 0) {
        r.index = (channel_offset as u8);
        r.carry = 0;
        return;
    }
    // Phase elapsed. The low nibble of the phase offset (byte 8) reaching the
    // 0x0C terminal marker means the envelope is finished -> signal stop.
    phase_low_nibble = engine.state.sound_channel_byte(8, channel_offset) & crate::bits::LOW_NIBBLE;
    if (phase_low_nibble >= 12) {
        r.index = (channel_offset as u8);
        r.value = (phase_low_nibble as u8);
        r.carry = 1;
        return;
    }
    // Advance to the next 4-byte phase record and reload bytes 9/10/11.
    next_phase = ((engine.state.sound_channel_byte(8, channel_offset) + 4) as u8 as i32); // +4 = next record
    engine
        .state
        .set_sound_channel_byte(8, channel_offset, next_phase);
    // Byte 9 = new phase delta (+0).
    engine.state.set_sound_channel_byte(
        9,
        channel_offset,
        engine
            .state
            .byte((ENVELOPE_TABLE + next_phase) as u16 as i32),
    );
    // Byte 10 = new note-tick reload (+1).
    engine.state.set_sound_channel_byte(
        10,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 1) + next_phase) as u16 as i32),
    );
    // Byte 11 = new phase duration (+2).
    engine.state.set_sound_channel_byte(
        11,
        channel_offset,
        engine
            .state
            .byte(((ENVELOPE_TABLE + 2) + next_phase) as u16 as i32),
    );
    r.index = (channel_offset as u8);
    r.offset = (next_phase as u8);
    r.carry = 0;
}

/// Assembles a room into the working buffers ready for display: selects the
/// room's data bank/pointers, copies its tile pages, builds the attribute
/// table, and builds the palette buffer.
pub fn scene_assemble(engine: &mut Engine, r: &mut RoutineContext) {
    // Map in the room's data bank and set up the room pointers.
    select_room_data_bank_and_pointers(engine, r);
    // Copy the room's tile/metatile pages into the working buffer.
    copy_room_tile_pages(engine, r);
    // Carry passed to text_attr_build = whether the room metadef high byte + 3
    // overflows a byte (selects the attribute-build variant/page boundary).
    r.carry = (((if ((engine.state.room_metadef_hi as i32 + 3) > 255) {
        1
    } else {
        0
    }) as u8) as u8);
    text_attr_build(engine, r);
    // Build the palette buffer for the assembled room.
    build_room_palette_buffer(engine, r);
}

/// Silences the sound-effect overlay voice on APU pulse 2 (lane offset 0x40):
/// duty-only mute byte plus clears the pulse-2 active bit (bit1). Shares the
/// physical SQ2 hardware with the music pulse-2 channel.
fn silence_sfx_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
    // Keep duty bits of the sfx lane's shadow byte 6; set bits 4-5 to mute.
    engine.device_write(
        crate::engine::reg::SQ2_VOL,
        (engine.state.sound_channel_byte(6, 64) & crate::bits::HIGH_2_BITS) | crate::bits::BITS_4_5,
    );
    engine.state.sound_status_flags =
        engine.state.sound_status_flags & ((crate::bits::CLEAR_BIT1) as u8);
}

/// Per-frame service for the sound-effect overlay (lane offset 0x40), which
/// borrows APU pulse channel 2 from the music.
///
/// State 0: if a new sfx is requested (`prompt_state != 0`) and it outranks any
/// playing sfx (`prompt_argument >= sfx_priority` or none active), start it by
/// loading its stream pointer from `SFX_POINTER_TABLE[prompt_state]`, marking
/// the overlay active (`sfx_voice_active` bit7) and claiming SQ2 (set bit6 of
/// `sound_channel_flags`). Otherwise continue any active sfx, fetching the next
/// note/rest/command exactly like a music channel. State 1: advances the
/// envelope and silences SQ2 + releases the channel (clear bit6) when the sfx
/// stream ends or its envelope terminates.
pub fn sfx_overlay_voice(engine: &mut Engine, r: &mut RoutineContext) {
    let mut start: i32 = 0;
    let mut state: i32 = 0;
    'dispatch: loop {
        match state {
            0 => {
                // Decide whether a newly-requested sfx should start.
                if (engine.state.prompt_state != 0) {
                    if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
                        // Nothing playing -> start.
                        start = 1;
                    } else if (engine.state.prompt_argument >= engine.state.sfx_priority) {
                        // Request priority >= current -> preempt and start.
                        start = 1;
                    } else {
                        // Lower priority than the active sfx -> drop the request.
                        engine.state.prompt_argument = 0;
                        engine.state.prompt_state = 0;
                    }
                }
                if ((start) == 0) {
                    // Not starting a new sfx: if nothing is active, nothing to do.
                    if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
                        return;
                    }
                    // Active sfx: tick its note-duration; if still running, just
                    // service the envelope (state 1).
                    if (((engine.state.dec_sound_channel_byte(0, 64)) as u8 as i32) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                } else {
                    // Start the requested sfx.
                    let mut sfx_table_index: i32 = 0;
                    engine.state.sfx_priority = engine.state.prompt_argument;
                    // Each sfx pointer is 2 bytes -> id * 2 = table offset.
                    sfx_table_index = (((engine.state.prompt_state as i32) << 1) as u8 as i32);
                    engine.state.set_sound_channel_byte(
                        2,
                        64,
                        engine
                            .state
                            .byte((SFX_POINTER_TABLE + sfx_table_index) as u16 as i32),
                    );
                    engine.state.set_sound_channel_byte(
                        3,
                        64,
                        engine
                            .state
                            .byte(((SFX_POINTER_TABLE + 1) + sfx_table_index) as u16 as i32),
                    );
                    engine.state.sfx_voice_active = 128; // 0x80 = overlay active
                    // Claim SQ2 from the music (bit6 of channel flags).
                    engine.state.sound_channel_flags = ((engine.state.sound_channel_flags
                        | ((crate::bits::BIT6) as u8))
                        as u8 as u8);
                    engine.state.prompt_state = 0;
                    engine.state.prompt_argument = 0;
                }
                loop {
                    // Stream pointer = bytes 2|3 of lane 0x40.
                    let mut stream_ptr: i32 = ((engine.state.sound_channel_byte(2, 64)
                        | (engine.state.sound_channel_byte(3, 64) << 8))
                        as u16 as i32);
                    let mut note_byte: i32 = engine.state.byte(stream_ptr);
                    // End-of-stream: release the overlay back to the music.
                    if (note_byte == 0) {
                        engine.state.sfx_voice_active = 0;
                        engine.state.sfx_priority = 0;
                        engine.state.sound_channel_flags =
                            engine.state.sound_channel_flags & ((crate::bits::CLEAR_BIT6) as u8);
                        silence_sfx_pulse2(engine, r);
                        return;
                    }
                    // 0xFF control command, then re-read.
                    if (note_byte == 255) {
                        dispatch_audio_stream_command(engine, r);
                        continue;
                    }
                    increment_selected_music_stream_pointer(engine, r);
                    engine
                        .state
                        .set_sound_channel_byte(0, 64, note_byte & crate::bits::LOW_7_BITS);
                    // Rest (bit7) vs. audible note.
                    if ((note_byte & crate::bits::BIT7) != 0) {
                        start_rest_envelope(engine, r);
                    } else {
                        // Compute period, mark pulse 2 active (bit1), push regs.
                        load_note_period(engine, r);
                        engine.state.sound_status_flags = 2 | engine.state.sound_status_flags; // 2 = BIT1
                        engine.device_write(
                            crate::engine::reg::SQ2_SWEEP,
                            engine.state.sound_channel_byte(7, 64),
                        );
                        engine.device_write(
                            crate::engine::reg::SQ2_LO,
                            (engine.state.sound_command as i32),
                        );
                        // SQ2_HI: low 3 bits period-high | high 2 bits length load.
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
                // Envelope service for the overlay (only if pulse 2 active).
                if ((engine.state.sound_status_flags & ((crate::bits::BIT1) as u8)) == 0) {
                    return;
                }
                // Phase timer expired -> recompute and write volume.
                if (((engine.state.dec_sound_channel_byte(10, 64)) as u8 as i32) == 0) {
                    next_envelope_volume(engine, r);
                    engine.device_write(crate::engine::reg::SQ2_VOL, (r.value as i32));
                }
                // Advance phase; carry = envelope terminated -> silence overlay.
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

/// Initializes the music engine state for the song id in `engine.state.song`.
///
/// Songs 0-9 live in PRG bank pair 10/11; songs 10+ in 12/13. After mapping the
/// banks it looks up the song's channel-stream header via `SONG_POINTER_TABLE`
/// (2 bytes per song) and unpacks it into the four music channel lanes at
/// 0x0093 (`data_ptr` base): for each of the 4 channels it copies the 8-byte
/// header block, then zero-fills the following 8-byte runtime block.
pub fn song_init(engine: &mut Engine, r: &mut RoutineContext) {
    let mut song: i32 = (engine.state.song as i32);
    let mut idx: i32 = 0;
    let mut x: i32 = 0;
    let mut blk: i32 = 0;
    // Pick the PRG bank pair holding the song data: 10/11 or 12/13.
    x = ((if (song < 10) { 10 } else { 12 }) as u8 as i32);
    engine.state.song_ptr_lo = (x as u8);
    engine.state.song_ptr_hi = ((x + 1) as u8);
    sound_set_song_banks(engine, r);
    engine.state.music_volume_override = 0;
    engine.state.prompt_state = 0;
    // Compute the song index within its bank (mod 10), * 2 for the 2-byte
    // pointer table entry.
    idx = ((if (song < 10) {
        song
    } else {
        ((song - 10) as u8 as i32)
    }) as u8 as i32);
    idx = ((idx << 1) as u8 as i32);
    {
        // Read the song's channel-header source pointer (lo/hi).
        engine.state.indirect_ptr_lo = ((engine
            .state
            .byte((SONG_POINTER_TABLE + idx) as u16 as i32))
            as u8);
        engine.state.indirect_ptr_hi = ((engine
            .state
            .byte(((SONG_POINTER_TABLE + 1) + idx) as u16 as i32))
            as u8);
    }
    // Destination = music channel lane storage at 0x0093.
    engine.state.data_ptr_lo = 147; // 0x93
    engine.state.data_ptr_hi = 0;
    {
        // Unpack all four channel lanes.
        blk = 0;
        while (blk < 4) {
            let mut y: i32 = 0;
            let mut s: i32 = ((engine.state.indirect_ptr()) as u16 as i32);
            let mut d: i32 = ((engine.state.data_ptr()) as u16 as i32);
            {
                // Copy the 8-byte channel header (bytes 7..0).
                y = 7;
                while (y >= 0) {
                    engine.state.set_byte(
                        ((d + y) as u16 as i32),
                        engine.state.byte((s + y) as u16 as i32),
                    );
                    {
                        let __old = y;
                        y -= 1;
                        __old
                    };
                }
            }
            // Advance destination by 8 to the runtime block (16-bit add).
            d = ((engine.state.data_ptr_lo as i32 + 8) as u16 as i32);
            engine.state.data_ptr_lo = (d as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((d >> 8) as u8);
            d = ((engine.state.data_ptr()) as u16 as i32);
            {
                // Zero the 8-byte runtime block.
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
            // Advance destination past the runtime block (next lane).
            d = ((engine.state.data_ptr_lo as i32 + 8) as u16 as i32);
            engine.state.data_ptr_lo = (d as u8);
            engine.state.data_ptr_hi = engine.state.data_ptr_hi + ((d >> 8) as u8);
            // Advance source by 8 to the next channel header.
            s = ((engine.state.indirect_ptr_lo as i32 + 8) as u16 as i32);
            engine.state.indirect_ptr_lo = (s as u8);
            engine.state.indirect_ptr_hi = engine.state.indirect_ptr_hi + ((s >> 8) as u8);
            {
                let __old = blk;
                blk += 1;
                __old
            };
        }
    }
    // Restore the regular PPU/PRG bank mapping after the song load.
    ppu_commit_banks(engine, r);
}

/// Restores the MMC3 PRG banks the game was using before the sound code
/// remapped them: reloads slots 6 (`0x8000`) and 7 (`0xA000`) from the saved
/// `prg_bank_8000`/`prg_bank_a000` shadows.
pub fn sound_restore_game_banks(engine: &mut Engine, r: &mut RoutineContext) {
    // MMC3 register 6 selects the 0x8000 swappable PRG bank.
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.prg_bank_8000 as i32),
    );
    // MMC3 register 7 selects the 0xA000 swappable PRG bank.
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.prg_bank_a000 as i32),
    );
}

/// Maps the default sound PRG banks (10/11) into MMC3 slots 6/7 (`0x8000` /
/// `0xA000`) for the per-frame audio tick. Returns the final select reg in X
/// (7) and bank in Y (11).
pub fn sound_set_default_banks(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 6; // MMC3 select reg 6 -> 0x8000
    let mut y: i32 = 10; // PRG bank 10 (sound code)
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, x);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, y);
    x = ((x + 1) as u8 as i32); // reg 7 -> 0xA000
    y = ((y + 1) as u8 as i32); // PRG bank 11
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, x);
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, y);
    r.index = (x as u8);
    r.offset = (y as u8);
}

/// Maps the currently-selected song's PRG banks (`song_ptr_lo`/`song_ptr_hi`,
/// set by `song_init`) into MMC3 slots 6/7 so the song's note streams are
/// reachable at `0x8000`/`0xA000`.
pub fn sound_set_song_banks(engine: &mut Engine, r: &mut RoutineContext) {
    // MMC3 register 6 -> 0x8000 = song low bank.
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 6);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.song_ptr_lo as i32),
    );
    // MMC3 register 7 -> 0xA000 = song high bank.
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 7);
    engine.device_write(
        crate::engine::reg::MMC3_BANK_DATA,
        (engine.state.song_ptr_hi as i32),
    );
}

/// Per-frame APU driver tick: services the sound-effect overlay voice and then
/// either freezes the channels (when paused) or advances all four music channels.
///
/// Channels use a 16-byte-strided layout; `sound_channel_offset` selects the
/// active channel (0=pulse1, 16=pulse2, 32=triangle, 48=noise, 64=sfx overlay).
/// Banks are swapped to the sound driver's data, the tick runs, and the game's
/// banks are restored on exit.
pub fn sound_tick(engine: &mut Engine, r: &mut RoutineContext) {
    // Map in the default (driver) PRG/CHR banks and service the SFX overlay,
    // which sits at channel offset 64 and can override the music voices.
    sound_set_default_banks(engine, r);
    engine.state.sound_channel_offset = 64; // sfx overlay channel slot
    r.value = 64;
    sfx_overlay_voice(engine, r);
    if (engine.state.sound_paused != 0) {
        // Paused: hold the pulse channels silent-ish by rewriting their volume
        // registers from the stored duty bits, and quiet triangle/noise.
        if ((engine.state.sfx_voice_active & ((crate::bits::BIT7) as u8)) == 0) {
            // Only refresh pulse 2 when the sfx overlay is not commandeering it.
            engine.device_write(
                crate::engine::reg::SQ2_VOL,
                // Keep the stored duty (top 2 bits) and force constant-volume +
                // halt-length flags (bits 4,5) so the note holds while paused.
                (engine.state.sound_channel_byte(6, 16) & crate::bits::HIGH_2_BITS)
                    | crate::bits::BITS_4_5,
            );
        }
        engine.device_write(
            crate::engine::reg::SQ1_VOL,
            // Same duty-preserve + constant-volume/halt for pulse 1.
            (engine.state.sound_channel_byte(6, 0) & crate::bits::HIGH_2_BITS)
                | crate::bits::BITS_4_5,
        );
        engine.device_write(crate::engine::reg::TRI_LINEAR, 0); // halt triangle
        engine.device_write(crate::engine::reg::NOISE_VOL, 48); // noise: constant-vol, halt, vol 0
        r.value = 48;
    } else {
        // Playing: map in the current song's banks and advance each channel's
        // sequencer in turn (offsets 0/16/32/48 = pulse1/pulse2/triangle/noise).
        sound_set_song_banks(engine, r);
        engine.state.sound_channel_offset = 0; // pulse 1
        r.value = 0;
        tick_pulse1_channel(engine, r);
        engine.state.sound_channel_offset = 16; // pulse 2
        r.value = 16;
        tick_pulse2_channel(engine, r);
        engine.state.sound_channel_offset = 32; // triangle
        r.value = 32;
        tick_triangle_channel(engine, r);
        engine.state.sound_channel_offset = 48; // noise
        r.value = 48;
        tick_noise_channel(engine, r);
    }
    // Restore the gameplay PRG/CHR banks the caller expects.
    sound_restore_game_banks(engine, r);
}

/// NMI-time scroll/bank fix-up that produces the split between the scrolling
/// play field and the fixed status bar at the bottom of the screen.
///
/// Restores the rendering registers from the shadow copies, and when the split
/// is enabled, programs a hard scroll/bank switch (via an MMC3 sprite-0/IRQ-less
/// mid-frame technique) so the status panel renders from a fixed nametable and
/// CHR banks. It then runs `sound_tick` (which consumes the part of the frame the
/// old NES used as timing margin) and, after the split point, reprograms the CHR
/// banks back to the room's banks.
pub fn statusbar_split(engine: &mut Engine, r: &mut RoutineContext) {
    // Reload PPU mask, then rebuild PPU_CTRL from its shadow with the current
    // nametable-select bit, and reapply the X/Y scroll.
    engine.device_write(
        crate::engine::reg::PPU_MASK,
        (engine.state.ppu_mask_shadow as i32),
    );
    engine.state.ppu_ctrl_shadow = (engine.state.ppu_ctrl_shadow
        & ((crate::bits::CLEAR_BIT0) as u8)) // clear base-nametable low bit
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
        // Program the status-bar region: reset the PPU latch, force base
        // nametable, scroll X=0 / Y=196 (the split scanline), and swap in the
        // status-panel CHR banks via MMC3 slots 1/4/5.
        let _ = engine.device_read(crate::engine::reg::PPU_STATUS);
        engine.device_write(
            crate::engine::reg::PPU_CTRL,
            ((engine.state.ppu_ctrl_shadow & ((crate::bits::CLEAR_BIT0) as u8)) as i32),
        );
        engine.device_write(crate::engine::reg::PPU_SCROLL, 0); // status bar X scroll = 0
        engine.device_write(crate::engine::reg::PPU_SCROLL, 196); // split Y scanline
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 1); // CHR slot 1
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 22); // -> bank 22
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 4); // CHR slot 4
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 62); // -> bank 62
        engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 5); // CHR slot 5
        engine.device_write(crate::engine::reg::MMC3_BANK_DATA, 63); // -> bank 63
    }
    // Run the APU driver (also burns the cycles that approximate waiting for the
    // split scanline before reprogramming the play-field banks).
    sound_tick(engine, r);
    if (engine.state.statusbar_split_flag == 0) {
        return;
    }
    // Past the split: restore the play-field scroll and the room's own CHR banks
    // into MMC3 slots 1/4/5.
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
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 4); // CHR slot 4
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, engine.state.chr_bank(4));
    engine.device_write(crate::engine::reg::MMC3_BANK_SELECT, 5); // CHR slot 5
    engine.device_write(crate::engine::reg::MMC3_BANK_DATA, engine.state.chr_bank(5));
}

/// Builds the per-room rendering/metadata block from the room descriptor pointed
/// to by `palette_src_ptr`. Reads the descriptor fields (tile/attribute table
/// pointers, CHR banks, room action, family-member mask, temp-save bytes) and,
/// based on a per-screen save-data flag, conditionally spawns the special room
/// object in slot 160 and triggers the room's music.
///
/// `r.carry` on entry adds 1 to the tile-table high page (selects an alternate
/// tile set). `r.value`/`r.carry` are clobbered by the bit-rotate that extracts
/// the screen's save flag.
pub fn text_attr_build(engine: &mut Engine, r: &mut RoutineContext) {
    let mut p: i32 = ((engine.state.palette_src_ptr()) as u16 as i32);
    let mut carry_in: i32 = (r.carry as u8 as i32);
    let mut b: i32 = 0;
    // Field +0: tile-table high page = base 0xA0 + descriptor byte (+1 if carry).
    b = engine.state.byte(p);
    engine.state.tile_table_ptr_hi = ((b + 160 + carry_in) as u8); // 0xA0 base page
    engine.state.tile_table_ptr_lo = 0;
    // Field +1: CHR bank slot 3.
    engine
        .state
        .set_chr_bank(3, engine.state.byte((p + 1) as u16 as i32));
    // Fields +2/+3: attribute-table pointer; +4: room tile action; +5/+6: CHR
    // bank slots 0/1.
    engine.state.text_attr_ptr_lo = ((engine.state.byte((p + 2) as u16 as i32)) as u8);
    engine.state.text_attr_ptr_hi = ((engine.state.byte((p + 3) as u16 as i32)) as u8);
    engine.state.room_tile_action = ((engine.state.byte((p + 4) as u16 as i32)) as u8);
    engine
        .state
        .set_chr_bank(0, engine.state.byte((p + 5) as u16 as i32));
    engine
        .state
        .set_chr_bank(1, engine.state.byte((p + 6) as u16 as i32));
    {
        // Index the per-screen save-flag bitfield: byte = (map_screen_y>>1)
        // selected from a row of save bytes whose base index folds the y bit 0
        // and the x coordinate together; then rotate left (y>>1)+1 times to land
        // this screen's flag in carry.
        let mut ms_y: i32 = (engine.state.map_screen_y as i32);
        let mut ms_x: i32 = (engine.state.map_screen_x as i32);
        let mut idx: i32 = ((((ms_y << 2) & crate::bits::BIT2) | ms_x) as u8 as i32);
        let mut a: i32 = engine.state.save_payload(idx);
        let mut cnt: i32 = (((ms_y >> 1) + 1) as u8 as i32);
        let mut c: i32 = 0;
        loop {
            c = (((a >> 7) & 1) as u8 as i32); // bit shifted out -> carry
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
        // If this screen's save flag is set, descriptor field +7 names the
        // special object's state (0 = none). Slot 160 is the dedicated
        // room-special object slot.
        let mut y: i32 = 7; // descriptor field index +7
        let mut a: i32 = 0;
        if ((r.carry) != 0) {
            a = engine.state.byte((p + y) as u16 as i32);
        } else {
            a = 0;
        }
        engine.state.set_object_state(160, a);
        if (a != 0) {
            // Spawn the special object: attr=1, x from +8, y from +9, and pick
            // its tile from the type byte at +10.
            engine.state.set_object_attr(160, 1);
            {
                let __old = y;
                y += 1;
                __old
            };
            engine
                .state
                .set_object_x_tile(160, engine.state.byte((p + y) as u16 as i32));
            engine.state.set_object_x_sub(160, 0);
            {
                let __old = y;
                y += 1;
                __old
            };
            engine
                .state
                .set_object_y_pixel(160, engine.state.byte((p + y) as u16 as i32));
            {
                let __old = y;
                y += 1;
                __old
            };
            b = engine.state.byte((p + y) as u16 as i32);
            if (b == 23) {
                // Type 23: stair/door special -> state 25, tile 0xDD.
                engine.state.set_object_state(160, 25);
                engine.state.set_object_tile(160, 221);
            } else {
                engine.state.set_object_tile(160, 233); // default special tile 0xE9
            }
        }
    }
    {
        // Decide whether to (re)start the room's music. For the first five songs
        // (x<5), rotate a 1 left x+1 times to build a one-hot mask and AND it with
        // descriptor field +21 (per-song suppress bitmask); a nonzero result
        // suppresses the music change.
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
            a = ((a & engine.state.byte((p + 21) as u16 as i32)) as u8 as i32);
            if (a != 0) {
                do_d02e = 0;
            }
        }
        if ((do_d02e) != 0) {
            // Field +11 holds the room's song id.
            r.value = ((engine.state.byte((p + 11) as u16 as i32)) as u8);
            switch_song_if_needed(engine, r);
        }
    }
    // Fields +16..+19: four temp-save bytes; field +20: family-member mask
    // (which of the five family members may enter this room).
    engine
        .state
        .set_temp_save(0, engine.state.byte((p + 16) as u16 as i32));
    engine
        .state
        .set_temp_save(1, engine.state.byte((p + 17) as u16 as i32));
    engine
        .state
        .set_temp_save(2, engine.state.byte((p + 18) as u16 as i32));
    engine
        .state
        .set_temp_save(3, engine.state.byte((p + 19) as u16 as i32));
    engine.state.family_member_mask = ((engine.state.byte((p + 20) as u16 as i32)) as u8);
}

/// The NMI (vertical-blank) handler. Latches PPU status, runs the OAM DMA, and
/// then dispatches the one pending VRAM upload job requested in `nmi_vram_req`
/// (1..=6) before handing off to `vblank_commit_tail` for the per-frame bank /
/// scroll / counter bookkeeping. The 6502 register context is saved on entry and
/// restored on exit so the interrupted foreground code sees no change.
pub fn vblank_commit(engine: &mut Engine, r: &mut RoutineContext) {
    let save = *r;
    {
        // Model the PPU side-effects this handler relies on: set the vblank flag,
        // set sprite-0 (only meaningful when sprites+background are enabled, i.e.
        // mask bits 3,4), and evaluate sprite overflow.
        engine.ppu.set_vblank((1) != 0);
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
        // Reading PPU_STATUS latches+clears vblank and resets the addr latch.
        let __v = engine.device_read(crate::engine::reg::PPU_STATUS);
        engine.state.frame_status = (__v as u8);
    }
    // Copy the OAM shadow page ($0200) to PPU OAM via DMA.
    engine.device_write(crate::engine::reg::OAM_ADDR, 0);
    engine.device_write(crate::engine::reg::OAM_DMA, 2); // DMA from page $02
    let mut req: i32 = (engine.state.nmi_vram_req as i32);
    if (req == 0) {
        // No upload requested: just do the per-frame tail work.
        vblank_commit_tail(engine, r);
        {
            *r = save;
            return;
        }
    }
    engine.state.nmi_vram_req = 0;
    if (req >= 7) {
        // req 7+ (e.g. 255) is a "frame only" request with no VRAM job.
        vblank_commit_tail(engine, r);
        {
            *r = save;
            return;
        }
    }
    {
        // Stash the original jump-table target for this job id (1..=6 -> index
        // 0..5; the 7th entry is unused here). These mirror the ROM's per-job
        // handler addresses ($D3xx pages) for fidelity, though dispatch below is
        // done by the match.
        const jt_lo: [i32; 7] = [81, 82, 95, 144, 229, 52, 68];
        const jt_hi: [i32; 7] = [211, 210, 210, 210, 210, 211, 211];
        engine.state.saved_audio_handler_lo = ((jt_lo[req as usize]) as u8);
        engine.state.saved_audio_handler_hi = ((jt_hi[req as usize]) as u8);
    }
    // Point the PPU address latch at the upload destination and set the
    // increment mode (CTRL bit 2: +1 vs +32) for the job.
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
        ((engine.state.ppu_ctrl_shadow & ((crate::bits::BIT2) as u8)) as u8 as i32), // VRAM increment bit only
    );
    // Dispatch the requested upload job; each handler ends by calling
    // vblank_commit_tail itself.
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
    // Restore the saved foreground register context.
    *r = save;
}

/// Shared tail of every NMI path: commits the CHR/PRG bank shadows, runs the
/// status-bar split + APU tick, decrements the foreground frame counter (which
/// the foreground busy-waits on), advances the rest of the per-frame timers, and
/// restores the MMC3 bank-select latch.
pub fn vblank_commit_tail(engine: &mut Engine, r: &mut RoutineContext) {
    ppu_commit_banks(engine, r);
    // $D354 LDA $2002: read PPU_STATUS to reset the PPUSCROLL/PPUADDR write
    // toggle before statusbar_split rewrites the scroll/address registers.
    let _ = engine.device_read(crate::engine::reg::PPU_STATUS);
    statusbar_split(engine, r);
    // Tick down the foreground frame counter (the value wait_for_frame_counter
    // spins on), saturating at 0.
    if (engine.state.frame_counter != 0) {
        engine.state.frame_counter =
            (engine.state.frame_counter - 1) & ((crate::bits::BYTE_MASK) as u8);
    }
    frame_counters(engine, r);
    // Leave the MMC3 bank-select register where the foreground expects it.
    engine.device_write(
        crate::engine::reg::MMC3_BANK_SELECT,
        (engine.state.mmc3_bank_select as i32),
    );
}

/// VRAM job 4: streams a fixed 64-byte block from the inventory staging area
/// (base 160) to PPU_DATA at the pre-latched address. Used to push the inventory
/// menu tilemap.
pub fn vram_blit_stack(engine: &mut Engine, r: &mut RoutineContext) {
    {
        let mut i: i32 = 0;
        while (i < 64) {
            // 64-byte run
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.inventory_item(160 + i), // staging base offset 160
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

/// VRAM job 5: copies `inventory_upload_col` bytes from CPU memory pointed to by
/// `vram_addr2` to PPU_DATA at the pre-latched address (a general indirect blit).
pub fn vram_copy_indirect(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (engine.state.inventory_upload_col as i32); // byte count
    let mut src: i32 = (((engine.state.vram_addr2_lo as i32)
        | ((engine.state.vram_addr2_hi as i32) << 8)) as u16 as i32); // 16-bit source pointer
    let mut y: i32 = 0;
    loop {
        engine.device_write(
            crate::engine::reg::PPU_DATA,
            engine.state.byte((src + y) as u16 as i32),
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

/// VRAM job 1: writes the constant byte in `vram_addr2_lo` to PPU_DATA
/// `inventory_upload_col` times (a run-length fill, e.g. clearing a row of tiles).
pub fn vram_fill_run(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = (engine.state.inventory_upload_col as i32); // run length
    let mut a: i32 = (engine.state.vram_addr2_lo as i32); // fill byte
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

/// VRAM job 6: writes exactly two bytes (`vram_addr2_hi` then `vram_addr2_lo`)
/// to PPU_DATA at the pre-latched address — a minimal two-tile poke.
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

/// VRAM job 3: uploads a HUD/column update from the VRAM staging buffer. Pushes
/// two adjacent 24-tile columns (using the +32 vertical VRAM increment) and then
/// performs up to six read-modify-write updates to attribute-table bytes so that
/// only the masked bits change. Used when scrolling exposes a new tile column and
/// for HUD redraws.
pub fn vram_upload_hud(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x: i32 = 0;
    // Select vertical (+32) VRAM increment so writes step down a column.
    engine.device_write(
        crate::engine::reg::PPU_CTRL,
        ((engine.state.ppu_ctrl_shadow | ((crate::bits::BIT2) as u8)) as u8 as i32), // set increment-by-32
    );
    {
        // First column: 24 tiles from staging[0..23] (written high index first).
        x = 23; // 24 tiles, top of column last
        while (x >= 0) {
            engine.device_write(crate::engine::reg::PPU_DATA, engine.state.vram_stage(x));
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    // Re-latch the address one tile to the right for the adjacent column.
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        (engine.state.vram_addr_hi as i32),
    );
    engine.device_write(
        crate::engine::reg::PPU_ADDR,
        ((engine.state.vram_addr_lo + 1) as u8 as i32), // +1 = next column
    );
    {
        // Second column: 24 tiles from staging[24..47].
        x = 23;
        while (x >= 0) {
            engine.device_write(
                crate::engine::reg::PPU_DATA,
                engine.state.vram_stage(24 + x), // second-column staging base 24
            );
            {
                let __old = x;
                x -= 1;
                __old
            };
        }
    }
    {
        // Attribute-table fix-up: for up to 6 entries (stepping x by 2), read the
        // existing attribute byte, keep the bits in the AND-mask (vram_addr2_lo),
        // OR in the new bits from staging[49+x], and write it back. staging[48+x]
        // holds each target's low address byte; vram_addr2_hi is the shared high
        // byte (attribute-table page).
        x = 10; // 6 RMW iterations: 10,8,6,4,2,0
        while (x >= 0) {
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                (engine.state.vram_addr2_hi as i32),
            );
            engine.device_write(
                crate::engine::reg::PPU_ADDR,
                engine.state.vram_stage(48 + x), // attribute addr-lo for this entry
            );
            let _ = engine.device_read(crate::engine::reg::PPU_DATA); // dummy read (PPU read latency)
            {
                let mut v: i32 = (((engine.device_read(crate::engine::reg::PPU_DATA)
                    & (engine.state.vram_addr2_lo as i32)) // keep-mask
                    | engine.state.vram_stage(49 + x)) as u8 // new attribute bits
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

/// VRAM job 2: uploads the full 32-byte palette buffer to PPU palette RAM at
/// $3F00, then resets the PPU address back to $3F00 (and to $0000) so the dim
/// universal-background color is used during rendering rather than a stale
/// palette-RAM address.
pub fn vram_upload_palette(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 0;
    // Latch PPU address $3F00 (palette RAM base).
    engine.device_write(crate::engine::reg::PPU_ADDR, 63); // $3F
    engine.device_write(crate::engine::reg::PPU_ADDR, 0); // $00
    {
        // Write all 32 palette bytes.
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
    // Re-point to $3F00 then $0000 so rendering reads the intended backdrop color.
    engine.device_write(crate::engine::reg::PPU_ADDR, 63); // $3F
    engine.device_write(crate::engine::reg::PPU_ADDR, 0); // $00
    engine.device_write(crate::engine::reg::PPU_ADDR, 0); // $00
    engine.device_write(crate::engine::reg::PPU_ADDR, 0); // $00 -> address now $0000
    vblank_commit_tail(engine, r);
}

// ---------------------------------------------------------------------------
// Hand-written native runtime glue and high-level flows (formerly native.rs).
// Merged here: it is mutually recursive with the translated routines above and
// shares their namespace, so a separate module bought nothing.
// ---------------------------------------------------------------------------

/// Restores the saved gameplay PRG banks into the swappable $8000/$A000 windows
/// and seeds the indirect-return pointer to (`lo`,`hi`). Counterpart to
/// `leave_return_home`; models the ROM's "return home" bank trampoline used
/// around far calls that run in the fixed driver banks.
fn enter_return_home(engine: &mut Engine, lo: i32, hi: i32) {
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    engine.state.prg_bank_8000 = engine.state.saved_prg_bank_8000;
    engine.state.prg_bank_a000 = engine.state.saved_prg_bank_a000;
    engine.state.mmc3_bank_select = 6; // MMC3 R6: bank for $8000 window
    engine.prg_map_shadow();
}

/// Maps the fixed driver banks (12/13) into the $8000/$A000 windows. Used to
/// leave a "return home" far-call region and re-enter the common driver code.
fn leave_return_home(engine: &mut Engine) {
    engine.state.prg_bank_8000 = 12; // driver bank for $8000
    engine.state.prg_bank_a000 = 13; // driver bank for $A000
    engine.state.mmc3_bank_select = 7; // MMC3 R7: bank for $A000 window
    engine.prg_map_shadow();
}

/// Far-call wrapper around the "return home" trampoline: maps the gameplay banks
/// (with return pointer lo/hi), invokes `target`, then maps the driver banks back.
/// Named for the ROM call site at $CCE4.
fn farcall_cce4(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    enter_return_home(engine, lo, hi);
    target(engine, r);
    leave_return_home(engine);
}

/// Far-call wrapper that saves the current $8000/$A000 banks, maps in the fixed
/// driver banks (12/13) to run `target`, then restores the caller's banks.
/// Named for the ROM call site at $0C0D.
fn farcall_0c0d(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    // Snapshot the caller's banks (so far-called code can re-enter them via
    // enter_return_home) and stash the return pointer.
    let old6 = engine.state.prg_bank_8000;
    let old7 = engine.state.prg_bank_a000;
    engine.state.saved_prg_bank_8000 = old6;
    engine.state.saved_prg_bank_a000 = old7;
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    // Map the fixed driver banks (12/13) and run the target.
    engine.state.prg_bank_8000 = 12;
    engine.state.prg_bank_a000 = 13;
    engine.state.mmc3_bank_select = 7; // R7 -> $A000 window
    engine.prg_map_shadow();
    target(engine, r);
    // Restore the caller's banks.
    engine.state.prg_bank_a000 = old7;
    engine.state.prg_bank_8000 = old6;
    engine.state.mmc3_bank_select = 6; // R6 -> $8000 window
    engine.prg_map_shadow();
}

/// Convenience wrapper that sets up an indirect VRAM copy (job 5): destination
/// address (`dlo`/`dhi`), source pointer (`slo`/`shi`), `len` bytes, then queues
/// it and waits for the NMI to perform the upload.
fn vram_blit(
    engine: &mut Engine,
    r: &mut RoutineContext,
    dlo: i32,
    dhi: i32,
    slo: i32,
    shi: i32,
    len: i32,
) {
    engine.state.vram_addr_lo = (dlo as u8);
    engine.state.vram_addr_hi = (dhi as u8);
    engine.state.vram_addr2_lo = (slo as u8);
    engine.state.vram_addr2_hi = (shi as u8);
    engine.state.inventory_upload_col = (len as u8);
    r.value = 5; // VRAM job id 5 = vram_copy_indirect
    queue_ppu_job_and_wait(engine, r);
}

/// Initializes the current object slot scratch as a spawned pickup/item from the
/// item id `x`: derives its state and tile from `x`, drops it at the stored
/// y-extra, gives it a 240-frame lifetime, and probes the terrain under it.
fn item_spawn_setup(engine: &mut Engine, r: &mut RoutineContext, x: i32) {
    engine.state.obj_state = ((x + 2) as u8); // item states begin at 2
    engine.state.obj_tile = (((x << 2) | crate::bits::BITS_0_7) as u8); // tile = id*4 with low pattern bits set
    engine.state.obj_attr = 1;
    engine.state.obj_y_pixel = engine.state.obj_y_extra;
    engine.state.obj_timer = 240; // ~4s lifetime before despawn
    engine.state.obj_move_scratch = 0;
    engine.state.obj_cooldown = 0;
    update_object_terrain_probe(engine, r);
}

/// Queues the VRAM job id in `r.value` and waits until the NMI-side upload has
/// consumed it.
pub fn queue_ppu_job_and_wait(engine: &mut Engine, r: &mut RoutineContext) {
    frame::wait_for_ppu_job_idle(engine, r);
    engine.state.nmi_vram_req = (r.value as u8);
    frame::wait_for_ppu_job_idle(engine, r);
}

/// Shows the start-button prompt and waits for release, press, and release so a
/// held Start does not leak into the next menu/gameplay state.
pub fn wait_for_start_button_prompt(engine: &mut Engine, r: &mut RoutineContext) {
    // Show the "Press Start" prompt and pause music (refcount-style increment).
    engine.state.prompt_state = 3;
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);
    // Wait for any held buttons to be released first.
    loop {
        let buttons = frame::read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Wait for Start (bit 4) to be pressed.
    loop {
        let buttons = frame::read_buttons(engine, r);
        if (buttons & crate::bits::BIT4) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Wait for Start to be released so it does not leak into the next state.
    loop {
        let buttons = frame::read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Clear the prompt and resume music.
    engine.state.prompt_state = 4;
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);
}

/// The top-level in-room game loop. Each iteration: handles death/continue,
/// reads input and runs the full update pipeline (player, projectiles, actors,
/// tile projectile, camera), draws sprites, and commits one frame. The
/// final-exit item diverts into the scripted ending sequence. Returns when the
/// frame runner is asked to stop or when control passes back to `main_init`.
pub fn main_loop_dispatch(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        if frame::frame_runner_stop_requested() {
            return;
        }
        if engine.state.player_health == 0 {
            // Player died: show them, then run the death/continue flow in the
            // death-handler bank ($B307). r.index==0 means resume the same game;
            // otherwise re-init (decrement first, mirroring the ROM's path).
            engine.state.sprite_blink_timer = 0;
            draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 7, 179, run_player_death_or_continue_flow);
            if r.index == 0 {
                continue;
            }
            r.index = ((r.index - 1) as u8);
            main_init(engine, r);
            return;
        }

        // Start a one-frame window, snapshot the scroll column for change
        // detection, read input, and run the per-frame game logic update.
        engine.state.frame_counter = 1;
        engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
        frame::read_buttons(engine, r);
        game_update(engine, r);

        if engine.state.final_exit_flag != 0 {
            // The final-exit item diverts the normal room loop into a scripted
            // sequence that still reuses the player/object update helpers.
            farcall_0c0d(engine, r, 235, 162, setup_final_exit_sequence);
            loop {
                frame::read_buttons(engine, r);
                farcall_0c0d(engine, r, 188, 171, tick_scripted_player_motion);
                farcall_0c0d(engine, r, 230, 165, update_final_exit_projectiles);
                farcall_0c0d(engine, r, 93, 167, rotate_sprite_zero_from_scripted_oam);
                farcall_0c0d(engine, r, 227, 163, tick_final_exit_sequence);
                if engine.state.player_health == 0 {
                    break; // $C105 BNE $C0D4: loop while alive, exit on death
                }
            }

            // Final exit done: fold the player fine-x back into tile/fine, hide
            // sprite 0, then fall through the normal death/continue path to reset.
            engine.state.player_x_tile = engine.state.player_x_fine >> 4; // high nibble = tile
            engine.state.player_x_fine =
                engine.state.player_x_fine & ((crate::bits::LOW_NIBBLE) as u8); // low nibble = sub-tile
            engine.state.set_oam_y(0, 239); // park sprite 0 off-screen
            engine.state.sprite_blink_timer = 0;
            draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 7, 179, run_player_death_or_continue_flow);
            r.index = ((r.index - 1) as u8);
            main_init(engine, r);
            return;
        }

        // Normal frame: advance projectiles, room actors, the tile projectile,
        // and the camera. The actor/draw helpers clobber carry, so it is saved
        // across the draw calls because the scroll-change check below needs it.
        update_player_projectiles(engine, r);
        update_room_actors(engine, r);
        update_tile_projectile(engine, r);
        update_camera_scroll_from_player(engine, r);
        let saved_c = r.carry;
        draw_player_sprites(engine, r);
        draw_room_object_sprites(engine, r);
        r.carry = saved_c;
        // When the scroll column advanced (and carry clear), bump the main-loop
        // phase used to stagger the column-upload work.
        if ((r.carry) == 0) && engine.state.saved_scroll_tile != engine.state.scroll_tile_x {
            engine.state.main_loop_phase =
                (engine.state.main_loop_phase + 1) & ((crate::bits::BYTE_MASK) as u8);
        }

        // Push the frame and wait for the NMI to tick the frame counter to 0.
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Sets up the final-exit sequence after the final item trigger: flash the
/// current scene, switch to the scripted room, and seed the special object/player
/// state used by `tick_final_exit_sequence`.
pub fn setup_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    // Freeze the player and flash the current scene twice (with the room objects
    // reset between flashes), then start fading out the lower palette.
    engine.state.prompt_state = 24;
    engine.state.sprite_blink_timer = 0;
    draw_player_sprites(engine, r);

    r.index = 2; // 2 flash cycles
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    reset_room_object_slots(engine, r);
    draw_room_object_sprites(engine, r);
    r.index = 3; // 3 flash cycles
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    fade_partial_palette_buffer_out(engine, r);

    // Hold a blank frame for ~1 second (60 frames).
    engine.state.prompt_state = 32;
    engine.state.frame_counter = 60;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    // Switch to the scripted ending room (map cell 2,19) and clear the screen.
    engine.state.map_screen_y = 19;
    engine.state.map_screen_x = 2;
    farcall_cce4(engine, r, 242, 200, scene_assemble);
    clear_name_tables_to_blank_tiles(engine, r);

    // Position the camera/player for the scripted scene and upload the room view.
    engine.state.set_oam_y(0, 239); // hide sprite 0
    engine.state.scroll_y = 34;
    engine.state.scroll_fine_x = 0;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 16;
    farcall_cce4(engine, r, 203, 197, upload_current_room_view);
    r.index = 4; // 4 flash cycles
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    engine.state.scroll_tile_x = 0;
    farcall_cce4(engine, r, 108, 199, upload_room_columns_from_bank9);
    engine.state.set_chr_bank(3, 61);

    // Scroll the screen vertically up to the split scanline (194), uploading a
    // dummy job each frame; the nametable-select bit tracks scroll_y bit 3.
    loop {
        let mut x = engine.state.scroll_y;
        if x == 0 {
            x = 240; // wrap 0 -> 240 (scanline count)
        }
        if x == 194 {
            break; // reached split row
        }
        x = ((x - 1) as u8);
        engine.state.scroll_y = x;
        engine.state.nametable_select = (x & ((crate::bits::BIT3) as u8)) >> 3; // bit 3 -> NT select
        r.value = 255; // frame-only NMI request (no VRAM job)
        queue_ppu_job_and_wait(engine, r);
    }

    // Flash twice more and refresh the scroll-register shadows.
    r.index = 2;
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    farcall_cce4(engine, r, 199, 193, refresh_scroll_register_shadows);

    // Seed the scripted "player object" in slot 0 and the special object state.
    engine.state.set_object_x_sub(0, 0);
    engine.state.set_object_x_tile(0, 0);
    engine.state.set_object_timer(0, 0);
    engine.state.scheduler_phase = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;
    engine.state.set_object_health(0, 100); // scripted-object health/progress = 100
    engine.state.sprite_index = 8;
    // Pack player_x_tile (high nibble) and fine-x (low nibble) into player_x_fine
    // so the scripted code can address the player by pixel.
    engine.state.player_x_fine =
        ((((engine.state.player_x_tile as i32) << 4) | (engine.state.player_x_fine as i32)) as u8);
    draw_scripted_player_sprites(engine, r);
    // Hide the two object sprite slots and load the OAM templates for the
    // scripted object and player.
    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    load_final_exit_object_oam_template(engine, r);
    load_final_exit_player_oam_template(engine, r);
}

/// Runs the one-shot final-exit cutscene path that is entered before the special
/// object latch at `0xF2` is set.
fn run_final_exit_cutscene(engine: &mut Engine, r: &mut RoutineContext) {
    // Initialize the boss/object health meter and clear the three body-segment
    // object slots (16/32/48) and the scripted-object health.
    build_object_health_meter_standard_tiles(engine, r);
    engine.state.set_object_state(16, 0);
    engine.state.set_object_state(32, 0);
    engine.state.set_object_state(48, 0);
    engine.state.obj_x_tile = 0; // $A80D STA $FA (obj_x_tile, reused as a loop counter), not obj_health
    engine.state.sprite_blink_timer = 0;
    engine.state.displaced_timer = 0;
    draw_scripted_player_sprites(engine, r);
    draw_final_exit_projectile_sprites(engine, r);
    engine.state.set_oam_y(0, 239); // hide sprite 0

    // Drop the player downward one pixel per frame until y reaches 160.
    while engine.state.player_y < 160 {
        engine.state.player_y = (engine.state.player_y + 1) & ((crate::bits::BYTE_MASK) as u8);
        draw_scripted_player_sprites(engine, r);
        engine.state.frame_counter = 1;
        frame::wait_for_frame_counter(engine, r);
    }

    // Settle the landing pose and set up the first scripted scroll segment
    // (tile table page 182, nametable 1).
    engine.state.fall_frames = 0;
    engine.state.jump_timer = 0;
    update_scripted_player_pose_from_motion(engine, r);
    draw_scripted_player_sprites(engine, r);
    engine.state.scroll_tile_x = 32;
    engine.state.nametable_select = 1;
    engine.state.prompt_state = 32;
    engine.state.prompt_argument = 128;
    engine.state.tile_table_ptr_hi = 182; // tile-table page $B6

    // Scroll two full slices of the first background section.
    loop {
        advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }
    loop {
        advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }

    // Switch to the next tile-table page (183) and scroll two more slices.
    engine.state.prompt_state = 32;
    engine.state.prompt_argument = 128;
    engine.state.tile_table_ptr_hi = 183; // tile-table page $B7
    loop {
        advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }
    loop {
        advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }

    // Animated waiting loop: alternate the nametable every 8 frames, and on each
    // sprite-0 hit deal 5 damage to the scripted player and redraw its meter.
    engine.state.tile_fetch_counter = 0;
    loop {
        if (engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0 {
            // every 8th frame
            engine.state.nametable_select =
                engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
            engine.state.prompt_state = 32;
            engine.state.prompt_argument = 128;
        }

        r.value = 255; // frame-only NMI
        queue_ppu_job_and_wait(engine, r);
        if engine.state.sprite0_hit() {
            r.value = 5; // 5 damage
            subtract_scripted_player_health(engine, r);
            build_player_health_meter_sprites(engine, r);
        }

        if engine.state.sprite_index == 0 {
            engine.state.sprite_index = 2;
        }

        draw_scripted_player_sprites(engine, r);
        rotate_sprite_zero_from_scripted_oam(engine, r);
        engine.state.tile_fetch_counter =
            (engine.state.tile_fetch_counter - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.tile_fetch_counter == 0 {
            break; // wrapped 0->255->...->0 (256 frames)
        }
    }

    engine.state.nametable_select = 1;
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    if engine.state.player_health == 0 {
        return; // died during the waiting loop
    }

    // Float the player up off the top of the screen with an accelerating step
    // (scratch0 grows each frame); stop once y+43 wraps past the bottom (239).
    engine.state.set_oam_y(0, 239);
    engine.state.prompt_state = 24;
    engine.state.prompt_argument = 255;
    engine.state.scratch0 = 1; // initial rise step
    loop {
        let prev = engine.state.player_y;
        let ny = ((prev - engine.state.scratch0) as u8 as i32);
        engine.state.player_y = (ny as u8);
        let c = if prev >= engine.state.scratch0 { 1 } else { 0 }; // borrow -> carry
        let t = ((ny + 43 + c) as u8 as i32);
        if t >= 239 {
            break;
        }
        draw_scripted_player_sprites(engine, r);
        engine.state.scratch0 = (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
        r.value = 255;
        queue_ppu_job_and_wait(engine, r);
    }

    // Transition to the final cutscene room (map cell 3,16): clear sprites,
    // drain audio, fade out, rebuild the scene.
    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    engine.state.sprite_index = 0;
    engine.state.oam_cursor = 128; // OAM write cursor -> sprite slot 32 ($80)
    reset_room_object_slots(engine, r);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    clear_name_tables_to_blank_tiles(engine, r);
    clear_oam_with_sprite_zero_template(engine, r);
    engine.state.map_screen_y = 16;
    engine.state.map_screen_x = 3;
    farcall_cce4(engine, r, 242, 200, scene_assemble);
    // Place the player at the room entrance for the walk-in animation.
    engine.state.scroll_tile_x = 18;
    engine.state.player_y = 192;
    engine.state.player_x_tile = 26;
    engine.state.player_x_fine = 1;
    engine.state.scroll_fine_x = 1;
    engine.state.player_pose = 9;
    engine.state.set_chr_bank(2, 53);
    engine.state.set_chr_bank(3, 52);
    engine.state.set_chr_bank(4, 54);
    engine.state.set_chr_bank(5, 55);
    // Activate the four scripted object slots (16/32/48 = body segments,
    // 64 = the companion object) and place them.
    engine.state.set_object_state(16, 1);
    engine.state.set_object_state(32, 1);
    engine.state.set_object_state(48, 1);
    engine.state.set_object_state(64, 1);
    engine.state.set_object_y_pixel(16, 160);
    engine.state.set_object_y_pixel(32, 160);
    engine.state.set_object_y_pixel(48, 160);
    engine.state.set_object_y_pixel(64, 112);
    engine.state.set_object_x_tile(64, 51);
    sync_final_exit_body_slots_from_player(engine, r);
    // Body segment tiles are spaced 32 apart (0x2D, 0x4D, 0x6D); companion tile
    // is 0x81. All use attribute 64 (palette/flip bits).
    let mut v = 45; // 0x2D
    engine.state.set_object_tile(16, v);
    v = ((v + 32) as u8 as i32);
    engine.state.set_object_tile(32, v);
    v = ((v + 32) as u8 as i32);
    engine.state.set_object_tile(48, v);
    engine.state.set_object_tile(64, 129); // 0x81
    engine.state.set_object_attr(16, 64);
    engine.state.set_object_attr(32, 64);
    engine.state.set_object_attr(48, 64);
    engine.state.set_object_attr(64, 64);
    upload_status_panel_template(engine, r);
    farcall_cce4(engine, r, 203, 197, upload_current_room_view);
    // Resync the HUD, draw everything, and fade the room palette in (character
    // index 7 selects the ending palette set).
    sync_health_hud(engine, r);
    sync_magic_hud(engine, r);
    sync_coin_hud(engine, r);
    sync_key_hud(engine, r);
    refresh_scroll_register_shadows(engine, r);
    clear_gameplay_object_sprites(engine, r);
    draw_player_sprites(engine, r);
    draw_status_item_sprites(engine, r);
    draw_room_object_sprites(engine, r);
    engine.state.character_index = 7;
    farcall_cce4(engine, r, 146, 196, fade_room_palette_in);
    engine.state.countdown_timer = 5;
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

    // Walk the player up to y=160, flipping the facing bit every other step so it
    // appears to alternate between two animation frames.
    loop {
        if engine.state.player_y == 160 {
            break;
        }
        engine.state.player_y = (engine.state.player_y - 1) & ((crate::bits::BYTE_MASK) as u8);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        if engine.state.player_y == 160 {
            break;
        }
        engine.state.player_y = (engine.state.player_y - 1) & ((crate::bits::BYTE_MASK) as u8);
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8); // toggle H-flip
        draw_player_sprites(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }

    // Pause in the idle pose briefly.
    engine.state.player_pose = 13;
    draw_player_sprites(engine, r);
    engine.state.countdown_timer = 3;
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

    // Auto-walk right (forced button=1) until the player reaches tile column 55,
    // keeping the body-segment slots synced behind the player.
    loop {
        engine.state.frame_counter = 1;
        engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
        engine.state.buttons = 1; // forced "right" input
        farcall_cce4(engine, r, 43, 212, game_update);
        farcall_cce4(engine, r, 93, 193, update_camera_scroll_from_player);
        sync_final_exit_body_slots_from_player(engine, r);
        draw_player_sprites(engine, r);
        draw_room_object_sprites(engine, r);
        if engine.state.saved_scroll_tile != engine.state.scroll_tile_x {
            engine.state.main_loop_phase =
                (engine.state.main_loop_phase + 1) & ((crate::bits::BYTE_MASK) as u8);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        if engine.state.player_x_tile == 55 {
            break;
        }
    }

    // Final flashing celebration: toggle the player pose and all four object
    // tiles' bit-2 every 8 frames, for 20 cycles, then roll the story text.
    engine.state.player_pose = 25;
    engine.state.set_object_tile(16, 57); // 0x39
    engine.state.set_object_tile(32, 89); // 0x59
    engine.state.set_object_tile(48, 121); // 0x79
    engine.state.set_object_tile(64, 145); // 0x91
    engine.state.countdown_timer = 20;
    while engine.state.countdown_timer_active() {
        engine.state.player_pose = engine.state.player_pose ^ ((crate::bits::BIT2) as u8);
        engine
            .state
            .set_object_tile(16, engine.state.object_tile(16) ^ crate::bits::BIT2);
        engine
            .state
            .set_object_tile(32, engine.state.object_tile(32) ^ crate::bits::BIT2);
        engine
            .state
            .set_object_tile(48, engine.state.object_tile(48) ^ crate::bits::BIT2);
        engine
            .state
            .set_object_tile(64, engine.state.object_tile(64) ^ crate::bits::BIT2);
        for _ in 0..8 {
            draw_scene_and_wait_one_frame(engine, r);
        }
    }

    run_story_text_sequence(engine, r);
}

/// Ticks the final-exit scripted object state machine and stores the updated
/// scratch slot back into the active object slot.
pub fn tick_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    // Load the scripted object's scratch state from slot $0400.
    engine.state.obj_slot_ptr_lo = 0;
    engine.state.obj_slot_ptr_hi = 4; // object slot page $04
    load_object_slot_scratch(engine, r);
    if engine.state.obj_health == 0 {
        // Boss defeated: hand off to the one-shot ending cutscene.
        run_final_exit_cutscene(engine, r);
        return;
    }

    // On a sprite-0 hit, see which of the two damage zones the player's projectile
    // is in (sprite_index+2 masked to bits 1,2 selects object slot 8 or 16).
    if engine.state.sprite0_hit() {
        let t = (engine.state.sprite_index + 2) & ((crate::bits::BITS_1_2) as u8);
        if t != 0 {
            let x = ((t << 3) as u8 as i32); // *8 -> object-slot byte offset
            if engine.state.object_state(x) != 0 {
                engine.state.set_object_state(x, 0);
                // If the hit landed in the vulnerable x window [176,208), deal 2
                // damage and flag a hit; otherwise just play the blocked sound.
                let sum = ((engine.state.scroll_pixel_x + ((engine.state.object_x_sub(x)) as u8))
                    as u8 as i32);
                if sum >= 176 && sum < 208 {
                    let bl = engine.state.obj_health;
                    engine.state.obj_health = if bl < 2 { 0 } else { ((bl - 2) as u8) }; // -2, floor 0
                    build_object_health_meter_standard_tiles(engine, r);
                    engine.state.prompt_state = 32;
                    engine.state.prompt_argument = 1;
                } else {
                    engine.state.prompt_state = 1; // blocked-hit sfx
                }
            }
        }
    }

    // Background scroll/animation state machine, advanced once each slice
    // completes (obj_x_tile reaches 0). obj_timer is the phase selector.
    if engine.state.obj_x_tile == 0 {
        match engine.state.obj_timer {
            4 => {
                // Phase 4: hold the open mouth (page 181) at the split row for
                // scheduler_phase frames, then fall back to the rest page (179).
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase != 0 {
                    if engine.state.scheduler_phase == 4 {
                        engine.state.prompt_state = 32;
                    }
                    engine.state.tile_table_ptr_hi = 181; // page $B5
                    engine.state.scroll_y = 194; // split scanline
                } else {
                    engine.state.tile_table_ptr_hi = 179; // page $B3 (idle)
                    engine.state.obj_timer = 0;
                }
            }
            3 => {
                // Phase 3: shrinking-mouth animation (page 178) that recedes the
                // X scroll by 4/frame and steers Y toward the split row.
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase != 0 {
                    engine.state.tile_table_ptr_hi = 178; // page $B2
                    if engine.state.scroll_pixel_x != 0 {
                        let v = if engine.state.scroll_pixel_x < 4 {
                            0
                        } else {
                            ((engine.state.scroll_pixel_x - 4) as u8 as i32) // X -= 4
                        };
                        engine.state.scroll_pixel_x = (v as u8);
                        if v >= 17 {
                            if engine.state.scroll_y < 210 {
                                engine.state.scroll_y =
                                    (engine.state.scroll_y + 4) & ((crate::bits::BYTE_MASK) as u8); // Y += 4
                            } else if engine.state.scroll_pixel_x != 0 {
                                engine.state.scroll_pixel_x = (engine.state.scroll_pixel_x - 4)
                                    & ((crate::bits::BYTE_MASK) as u8);
                            }
                        } else if engine.state.scroll_y >= 195 {
                            engine.state.scroll_y =
                                (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8); // Y -= 4
                        }
                    } else if engine.state.scroll_y >= 195 {
                        engine.state.scroll_y =
                            (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8);
                    }
                } else if engine.state.scroll_pixel_x != 0 {
                    engine.state.obj_timer = 0;
                } else {
                    // Fully closed: switch to page 176 and re-arm phase 4.
                    engine.state.tile_table_ptr_hi = 176; // page $B0
                    engine.state.obj_timer =
                        (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8); // -> 4
                    engine.state.scheduler_phase = 4;
                }
            }
            2 => {
                // Phase 2: hold page 180 and pull Y back toward the split row.
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase != 0 {
                    engine.state.tile_table_ptr_hi = 180; // page $B4
                    if engine.state.scroll_y >= 195 {
                        engine.state.scroll_y =
                            (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8);
                    }
                } else {
                    engine.state.tile_table_ptr_hi = 179; // page $B3 (idle)
                    engine.state.obj_timer = 0;
                }
            }
            1 => {
                // Phase 1: growing-mouth animation that pushes X scroll out by
                // 4/frame; tile page alternates 176/177 by scheduler_phase parity.
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase == 0 {
                    engine.state.obj_timer = 0;
                } else {
                    let a = (((((engine.state.scheduler_phase as i32) << 1)
                        & (((crate::bits::BIT0) as u8) as i32)) // parity bit
                        + 176) as u8 as i32); // base page $B0
                    engine.state.tile_table_ptr_hi = (a as u8);
                    engine.state.scroll_pixel_x =
                        (engine.state.scroll_pixel_x + 4) & ((crate::bits::BYTE_MASK) as u8); // X += 4
                    if engine.state.scroll_pixel_x >= 64 {
                        engine.state.obj_timer = 0;
                    } else {
                        engine.state.scroll_y = 194; // split scanline
                    }
                }
            }
            _ => {
                // Phase 0 (idle): decide which animation to begin based on the
                // combined player + scroll X position relative to the mouth zone.
                let sum = ((engine.state.scroll_pixel_x + engine.state.player_x_fine) as u8 as i32);
                let carry = sum < (engine.state.scroll_pixel_x as i32); // add overflowed
                let close = carry || sum >= 192 || engine.state.scroll_pixel_x >= 64;
                let delayed_grow = sum < 128 || sum >= 160;
                if close || (delayed_grow && engine.state.scroll_y >= 195) {
                    // Begin the closing (phase 3) animation and run its first step
                    // inline (the body below mirrors the phase-3 arm).
                    engine.state.obj_timer = 3;
                    engine.state.scheduler_phase = 2;
                    engine.state.scheduler_phase =
                        (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                    if engine.state.scheduler_phase != 0 {
                        engine.state.tile_table_ptr_hi = 178;
                        if engine.state.scroll_pixel_x != 0 {
                            let v = if engine.state.scroll_pixel_x < 4 {
                                0
                            } else {
                                ((engine.state.scroll_pixel_x - 4) as u8 as i32)
                            };
                            engine.state.scroll_pixel_x = (v as u8);
                            if v >= 17 {
                                if engine.state.scroll_y < 210 {
                                    engine.state.scroll_y = (engine.state.scroll_y + 4)
                                        & ((crate::bits::BYTE_MASK) as u8);
                                } else if engine.state.scroll_pixel_x != 0 {
                                    engine.state.scroll_pixel_x = (engine.state.scroll_pixel_x - 4)
                                        & ((crate::bits::BYTE_MASK) as u8);
                                }
                            } else if engine.state.scroll_y >= 195 {
                                engine.state.scroll_y =
                                    (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8);
                            }
                        } else if engine.state.scroll_y >= 195 {
                            engine.state.scroll_y =
                                (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8);
                        }
                    } else if engine.state.scroll_pixel_x != 0 {
                        engine.state.obj_timer = 0;
                    } else {
                        engine.state.tile_table_ptr_hi = 176; // page $B0
                        engine.state.obj_timer =
                            (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
                        engine.state.scheduler_phase = 4;
                    }
                } else if !delayed_grow {
                    // Mid zone: just rest in phase 2 (page 179) for 8 frames.
                    engine.state.obj_timer = 2;
                    engine.state.scheduler_phase = 8;
                    engine.state.tile_table_ptr_hi = 179; // page $B3
                } else {
                    // Begin the opening (phase 1) animation and run its first step
                    // inline (mirrors the phase-1 arm).
                    engine.state.obj_timer = 1;
                    engine.state.scheduler_phase = 4;
                    engine.state.scheduler_phase =
                        (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                    if engine.state.scheduler_phase == 0 {
                        engine.state.obj_timer = 0;
                    } else {
                        let a = (((((engine.state.scheduler_phase as i32) << 1)
                            & (((crate::bits::BIT0) as u8) as i32)) // parity bit
                            + 176) as u8 as i32); // base page $B0
                        engine.state.tile_table_ptr_hi = (a as u8);
                        engine.state.scroll_pixel_x =
                            (engine.state.scroll_pixel_x + 4) & ((crate::bits::BYTE_MASK) as u8);
                        if engine.state.scroll_pixel_x >= 64 {
                            engine.state.obj_timer = 0;
                        } else {
                            engine.state.scroll_y = 194; // split scanline
                        }
                    }
                }
            }
        }
    }

    // Advance the background one scroll slice and persist the updated scratch.
    advance_scripted_scroll_slice(engine, r);
    store_object_slot_scratch(engine, r);
}

/// Fades the first 13 palette-buffer entries toward black over four timed
/// foreground frames.
pub fn fade_partial_palette_buffer_out(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 4; // 4 dim steps
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Darken entries 0..12: drop the brightness nibble (high) by one level
        // (subtract 16), clamping the darkest level to 0x0F (black).
        for x in (0..=12).rev() {
            let lo = engine.state.palette_buffer(x) & crate::bits::LOW_NIBBLE; // hue
            let hi = engine.state.palette_buffer(x) & crate::bits::HIGH_NIBBLE; // brightness
            engine.state.scratch0 = (lo as u8);
            let out = if hi < 16 {
                15 // already darkest -> NES black ($0F)
            } else {
                (((hi - 16) | lo) as u8 as i32) // one brightness level darker
            };
            engine.state.set_palette_buffer(x, out);
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
}

/// Redraws player/object sprites, commits one foreground frame, and waits for
/// the frame counter to expire.
pub fn draw_scene_and_wait_one_frame(engine: &mut Engine, r: &mut RoutineContext) {
    draw_player_sprites(engine, r);
    draw_room_object_sprites(engine, r);
    engine.state.frame_counter = 1;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Clears title/demo screen state so `main_init` can seed the first playable
/// room immediately after returning from the title loop.
pub fn clear_title_screen_for_new_game(engine: &mut Engine, r: &mut RoutineContext) {
    // Fade out, blank the nametables, and rebuild the status panel/OAM/menu state.
    fade_palette_buffer_out(engine, r);
    farcall_cce4(engine, r, 139, 195, clear_name_tables_to_blank_tiles);
    upload_status_panel_template(engine, r);
    clear_oam_with_sprite_zero_template(engine, r);
    reset_menu_state_and_palette(engine, r);
    // Refresh the HUD counters (coin sync is intentionally issued twice, as in
    // the ROM, to also cover the second digit row).
    sync_health_hud(engine, r);
    sync_coin_hud(engine, r);
    sync_key_hud(engine, r);
    sync_coin_hud(engine, r);

    // Run one frame in the gameplay banks so the uploads commit.
    engine.state.frame_counter = 1;
    enter_return_home(engine, 53, 193);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
    leave_return_home(engine);
}

/// Runs the scrolling story-text sequence shared by the title-screen chord and
/// the final-exit cutscene.
pub fn run_story_text_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    // Bump the music-volume override, drain audio, fade out, and clear the
    // screen/sprites. Load the text font CHR banks and enable rendering.
    engine.state.music_volume_override =
        (engine.state.music_volume_override + 1) & ((crate::bits::BYTE_MASK) as u8);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    clear_name_tables_to_blank_tiles(engine, r);
    hide_all_sprite_y_positions(engine, r);
    engine.state.set_chr_bank(0, 32);
    engine.state.set_chr_bank(1, 34);
    engine.state.ppu_mask_shadow = engine.state.ppu_mask_shadow | ((crate::bits::BITS_3_4) as u8); // enable bg+spr

    r.value = 255; // frame-only NMI
    queue_ppu_job_and_wait(engine, r);

    // Start the story/ending theme (song 10).
    engine.state.song = 10;
    song_init(engine, r);

    // Reset scroll and load the intro-text palette.
    engine.state.scroll_pixel_x = 0;
    engine.state.nametable_select = 0;
    engine.state.scratch2 = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;
    load_intro_text_palette(engine, r);

    // Point the text source at the story script ($B79C) and configure a 32-byte
    // upload to nametable address $0140.
    engine.state.vram_addr2_lo = 64; // dest addr lo $40
    engine.state.vram_addr2_hi = 1; // dest addr hi $01
    engine.state.inventory_upload_col = 32; // 32 columns/bytes per line
    engine.state.data_ptr_lo = 156; // script ptr lo $9C
    engine.state.data_ptr_hi = 183; // script ptr hi $B7

    // Scroll the story text upward, staging static then scrolling lines until a
    // staging routine signals end-of-script via carry.
    loop {
        advance_intro_text_scroll(engine, r);
        stage_intro_text_line(engine, r);
        if ((r.carry) != 0) {
            break;
        }
        advance_intro_text_scroll(engine, r);
        stage_scrolling_intro_text_line(engine, r);
        if ((r.carry) != 0) {
            break;
        }
    }

    // Trigger the closing sfx and wait for it to start and then finish.
    engine.state.prompt_state = 32;
    while engine.state.sfx_voice_active == 0 {
        frame::wait_frame(engine, r);
    }
    while engine.state.sfx_voice_active != 0 {
        frame::wait_frame(engine, r);
    }

    engine.state.frame_counter = 60; // ~1 second pause
    frame::wait_for_frame_counter(engine, r);

    // Silence channels 1 (pulse2)/2 (triangle? offsets 0/32/48) and clear flags.
    engine.state.set_sound_channel_byte(1, 0, 0);
    engine.state.sound_channel_flags = 0;
    engine.state.set_sound_channel_byte(1, 32, 0);
    engine.state.set_sound_channel_byte(1, 48, 0);
    engine.state.prompt_state = 24;

    // Flash the text: alternate all 32 palette entries to white (0x30) for one
    // frame, then back to the real palette for two, 10 times.
    let mut cnt = 10;
    loop {
        for x in (0..=31).rev() {
            engine.state.set_palette_buffer(x, 48); // 0x30 = white
        }
        upload_palette_buffer(engine, r);
        engine.state.frame_counter = 1;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        load_intro_text_palette(engine, r);
        upload_palette_buffer(engine, r);
        engine.state.frame_counter = 2;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        cnt = ((cnt - 1) as u8 as i32);
        if cnt == 0 {
            break;
        }
    }
}

/// Runs the title screen, timeout-driven attract/demo loop, and title shortcuts.
/// Returning normally means the user pressed Start and startup should enter the
/// first playable room.
pub fn run_title_screen_loop(engine: &mut Engine, r: &mut RoutineContext) {
    'restart: loop {
        // --- Title screen setup ---
        // Reset menu state, disable rendering, blank the palette, then build the
        // title nametables/OAM and start the title song (9).
        reset_menu_state_and_palette(engine, r);
        engine.state.set_chr_bank(2, 55);
        engine.state.statusbar_split_flag = 0;
        engine.state.ppu_ctrl_shadow = 160; // $A0: NMI on, 8x16 sprites
        engine.device_write(crate::engine::reg::PPU_CTRL, 160);
        engine.state.ppu_mask_shadow = 0;
        engine.device_write(crate::engine::reg::PPU_MASK, 0); // rendering off
        engine.state.scroll_pixel_x = 0;
        engine.state.nametable_select = 0;
        engine.state.scroll_y = 232;
        for x in (0..=31).rev() {
            engine.state.set_palette_buffer(x, 15); // all entries -> black ($0F)
        }
        farcall_cce4(engine, r, 105, 197, upload_palette_buffer);
        reset_room_object_slots(engine, r);
        clear_oam_with_sprite_zero_template(engine, r);
        load_title_oam_template(engine, r);
        engine.state.set_chr_bank(2, 21);
        engine.state.song = 9; // title theme
        song_init(engine, r);
        upload_title_screen_nametables(engine, r);
        engine.state.ppu_mask_shadow = 30; // $1E: show bg+sprites, no clip
        engine.device_write(crate::engine::reg::PPU_MASK, 30);
        engine.state.frame_counter = 120; // ~2s before fade-in
        frame::wait_for_frame_counter(engine, r);
        fade_title_palette_in(engine, r);
        engine.state.countdown_timer = 20; // attract timeout

        // --- Title idle loop ---
        // Wait for Start (-> new game) or the special password chord (-> story),
        // pulsing the "Press Start" color, until the attract timer expires.
        loop {
            engine.state.frame_counter = 1;
            let pad = frame::read_buttons(engine, r);
            if pad == 255 {
                // All buttons held: arm the password/continue entry.
                engine.state.prompt_state = 26;
                engine.state.continue_timer = 26;
            }
            if (engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return; // Start pressed -> begin a new game
            }
            if engine.state.button_chord == 131 {
                run_story_text_sequence(engine, r); // chord $83 -> show the story
                return;
            }
            // Every 8th frame, cycle the brightness of palette entry 2 (and mirror
            // it to entry 19) to make the prompt pulse.
            if (engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0 {
                let lo = engine.state.palette_buffer(2) & crate::bits::LOW_NIBBLE;
                let mut hi = engine.state.palette_buffer(2) & crate::bits::HIGH_NIBBLE;
                engine.state.scratch0 = (lo as u8);
                if hi < 16 {
                    hi = 48; // wrap dark -> brightest ($30 brightness band)
                } else {
                    hi = ((hi - 16) as u8 as i32); // one level dimmer
                }
                engine.state.set_palette_buffer(19, hi);
                engine
                    .state
                    .set_palette_buffer(2, hi | (engine.state.scratch0 as i32));
            }
            enter_return_home(engine, 53, 193);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            leave_return_home(engine);
            if engine.state.countdown_timer_active() {
                continue;
            }
            break; // attract timeout expired -> run a demo
        }

        // --- Attract/demo setup ---
        // Fade out and pick a random room (x in 0..4, y in 0..16).
        fade_palette_buffer_out(engine, r);
        clear_oam_with_sprite_zero_template(engine, r);
        reset_room_object_slots(engine, r);
        load_demo_oam_template(engine, r);
        r.value = 4; // RNG range 0..3 for map x
        rng_update(engine, r);
        engine.state.map_screen_x = (r.value as u8);
        r.value = 16; // RNG range 0..15 for map y
        rng_update(engine, r);
        engine.state.map_screen_y = (r.value as u8);
        farcall_cce4(engine, r, 242, 200, scene_assemble);

        // Pick a random standable spawn tile: x in 0..63, y aligned to 16px rows,
        // rejecting solid/special tiles (this tile and the next must be valid).
        loop {
            r.value = 64; // x range 0..63
            rng_update(engine, r);
            engine.state.player_x_tile = (r.value as u8);
            engine.state.data_ptr_lo = (r.value as u8);
            engine.state.player_x_fine = 0;
            r.value = 11; // y range 0..10
            rng_update(engine, r);
            r.value = (((r.value as i32) << 4) as u8); // *16 -> pixel y
            engine.state.player_y = (r.value as u8);
            engine.state.data_ptr_hi = (r.value as u8);
            resolve_room_tile_pointer(engine, r);
            let p = ((engine.state.data_ptr()) as u16 as i32);
            let mut t = engine.state.byte(p) & crate::bits::LOW_6_BITS; // tile id (low 6 bits)
            if t >= 48 {
                continue; // solid/blocking tile -> retry
            }
            if t == 2 {
                continue; // reject the hazard tile
            }
            if t == (engine.state.text_attr_ptr_lo as i32) {
                continue; // reject the room's special tile
            }
            t = engine.state.byte((p + 1) as u16 as i32) & crate::bits::LOW_6_BITS; // tile below
            if t < 48 {
                continue; // floor must be solid (>=48)
            }
            if t == 48 {
                continue;
            }
            break;
        }

        // Center the camera on the spawn column, clamped to [0,48].
        let mut x = engine.state.player_x_tile;
        if x < 8 {
            x = 0;
        } else {
            x = ((x - 8) as u8); // center: tile - 8 columns
        }
        if x >= 48 {
            x = 48; // clamp to rightmost scroll
        }
        engine.state.scroll_tile_x = x;
        engine.state.scroll_fine_x = 0;

        // Pick a random family member (0..4) that is allowed in this room: build a
        // one-hot mask from the rolled index and require it to intersect the room's
        // family-member mask.
        let chr = loop {
            r.value = 5; // 5 family members
            rng_update(engine, r);
            let chr = r.value;
            let mut a = 0;
            let mut c = 1;
            for _ in 0..=chr {
                let nc = (a >> 7) & 1;
                a = (((a << 1) | c) as u8 as i32); // rotate-left building a one-hot bit
                c = nc;
            }
            let mask = a;
            if (mask & (engine.state.family_member_mask as i32)) != 0 {
                break chr;
            }
        };
        // Give the chosen character their starting item and their three stat-derived
        // inventory slots (slots 11..14) from the character stats table.
        engine.state.set_item_slot(
            0,
            engine
                .state
                .byte((START_ITEM_TABLE + (chr as i32)) as u16 as i32),
        );
        engine.state.selected_item_slot = 0;
        engine.state.character_index = (chr as u8);
        let mut y = ((CHARACTER_STATS_TABLE + (((chr << 2) + 3) as i32)) as u16 as i32); // stats row end (chr*4 + 3)
        for i in (0..=3).rev() {
            engine.state.set_item_slot(11 + i, engine.state.byte(y)); // slots 11..14
            y = ((y - 1) as u16 as i32);
        }
        engine
            .state
            .set_chr_bank(2, ((engine.state.character_index + 56) as i32)); // per-character sprite bank base 56
        engine.state.set_chr_bank(4, 62);
        engine.state.set_chr_bank(5, 32);
        engine.state.player_pose = 13;
        engine.state.player_facing = 0;
        engine.state.title_timer = 1;
        engine.state.player_health = 100;
        engine.state.player_magic = 100;
        farcall_cce4(engine, r, 139, 195, clear_name_tables_to_blank_tiles);
        upload_status_panel_template(engine, r);
        farcall_cce4(engine, r, 203, 197, upload_current_room_view);
        sync_health_hud(engine, r);
        sync_magic_hud(engine, r);
        sync_coin_hud(engine, r);
        sync_key_hud(engine, r);
        refresh_scroll_register_shadows(engine, r);
        clear_gameplay_object_sprites(engine, r);
        draw_player_sprites(engine, r);
        draw_status_item_sprites(engine, r);
        farcall_cce4(engine, r, 146, 196, fade_room_palette_in);
        engine.state.countdown_timer = 10; // demo runtime

        // --- Demo play loop ---
        // Drive the real game update pipeline with synthesized random inputs;
        // Start jumps straight to a new game. Runs until the demo timer expires.
        loop {
            engine.state.frame_counter = 1;
            engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
            blink_demo_oam_sprites(engine, r);
            frame::read_buttons(engine, r);
            if (engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return; // Start during demo -> new game
            }

            // Reuse the last synthesized input; pick a new random one when the
            // hold timer elapses or the player is idle (no movement deltas).
            engine.state.buttons = engine.state.room_restore_scratch;
            let mut do_b044 = true;
            if (engine.state.horizontal_subtile_delta | engine.state.vertical_delta) != 0 {
                engine.state.title_timer =
                    (engine.state.title_timer - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.title_timer != 0 {
                    do_b044 = false;
                }
            }
            if do_b044 {
                engine.state.title_timer = 128; // input hold duration
                choose_random_demo_input(engine, r);
                engine.state.room_restore_scratch = engine.state.buttons;
            }

            // Run the full per-frame update pipeline (each in its gameplay bank).
            farcall_cce4(engine, r, 43, 212, game_update);
            farcall_cce4(engine, r, 40, 246, update_player_projectiles);
            farcall_cce4(engine, r, 124, 232, update_room_actors);
            farcall_cce4(engine, r, 130, 247, update_tile_projectile);
            farcall_cce4(engine, r, 93, 193, update_camera_scroll_from_player);
            draw_player_sprites(engine, r);
            draw_room_object_sprites(engine, r);
            if engine.state.saved_scroll_tile != engine.state.scroll_tile_x {
                engine.state.main_loop_phase =
                    (engine.state.main_loop_phase + 1) & ((crate::bits::BYTE_MASK) as u8);
            }
            enter_return_home(engine, 53, 193);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            leave_return_home(engine);
            if engine.state.countdown_timer_active() {
                continue;
            }
            break;
        }

        // Demo finished: fade out and restart the whole title sequence.
        fade_palette_buffer_out(engine, r);
        continue 'restart;
    }
}

/// Waits through a fixed object-render loop while draining active audio/sfx
/// timers used by the story/final-exit transitions.
pub fn drain_audio_timers_with_object_frames(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_sound_channel_byte(1, 32, 0); // silence triangle channel slot
    engine.state.data_ptr_hi = 16; // outer count: 16 groups
    loop {
        // Decrement the note-length timers (channel byte 13) of pulse1/pulse2/noise
        // (offsets 0/16/48) so any tail notes wind down.
        if engine.state.sound_channel_byte(13, 0) != 0 {
            engine.state.dec_sound_channel_byte(13, 0);
        }
        if engine.state.sound_channel_byte(13, 16) != 0 {
            engine.state.dec_sound_channel_byte(13, 16);
        }
        if engine.state.sound_channel_byte(13, 48) != 0 {
            engine.state.dec_sound_channel_byte(13, 48);
        }
        // Inner loop: render 20 object frames per group.
        engine.state.data_ptr_lo = 20;
        loop {
            draw_room_object_sprites(engine, r);
            engine.state.frame_counter = 1;
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            engine.state.data_ptr_lo =
                (engine.state.data_ptr_lo - 1) & ((crate::bits::BYTE_MASK) as u8);
            if engine.state.data_ptr_lo == 0 {
                break;
            }
        }
        engine.state.data_ptr_hi =
            (engine.state.data_ptr_hi - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.data_ptr_hi == 0 {
            break;
        }
    }
}

/// Runs the player death animation, extra-life recovery, and game-over/continue
/// screen. `r.index` returns `0` for immediate resume; nonzero values are
/// decremented by the caller before re-entering `main_init`.
pub fn run_player_death_or_continue_flow(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_song = engine.state.song;

    // Pause music and show the death pose (pose 53) for 8 frames + ~1s.
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);
    clear_gameplay_object_sprites(engine, r);
    r.index = 53; // death pose
    r.offset = 0;
    show_player_pose_for_eight_frames(engine, r);

    engine.state.frame_counter = 60;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    // Play the death jingle (song 8) and unpause.
    r.value = 8;
    switch_song_if_needed(engine, r);
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);

    // Spin the death tumble animation: cycle four poses, 5 times.
    engine.state.scratch2 = 5;
    loop {
        r.index = 13; // pose 13 facing 0
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 1; // pose 1
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 9; // pose 9
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 1; // pose 1, facing 64 (H-flip)
        r.offset = 64;
        show_player_pose_for_eight_frames(engine, r);
        engine.state.scratch2 = (engine.state.scratch2 - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.scratch2 == 0 {
            break;
        }
    }

    // Final collapsed pose (49).
    engine.state.frame_counter = 1;
    engine.state.player_pose = 49;
    draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    // Decide between resume and the game-over screen. The final-exit path always
    // goes to game over.
    let mut use_game_over_screen = engine.state.final_exit_flag != 0;
    if !use_game_over_screen {
        if (engine.state.continue_timer & ((crate::bits::BIT7) as u8)) == 0 {
            // $935C BPL $9363: bit7 CLEAR routes to the extra-life-token check.
            // An extra-life token (item id 12) in the selected slot lets the player
            // resume; consume it. Otherwise go to game over.
            let x = engine.state.selected_item_slot;
            if engine.state.item_slot(x as i32) == 12 {
                engine.state.set_item_slot((x as i32), 255); // remove extra life
                draw_status_item_sprites(engine, r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            engine.state.continue_timer =
                (engine.state.continue_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
        }

        if !use_game_over_screen {
            // Resume: refill health, restore the room song, return r.index=0.
            animate_health_refill_to_cap(engine, r);
            engine.state.player_pose = 25;
            read_debounced_buttons(engine, r);
            r.value = (saved_song as u8);
            switch_song_if_needed(engine, r);
            r.index = 0; // 0 -> caller resumes the same game
            return;
        }
    }

    // --- Game-over / continue screen ---
    fade_palette_buffer_out(engine, r);
    engine.state.final_exit_flag = 0;
    engine.state.sprite_index = 0;
    engine.state.oam_cursor = 128; // OAM cursor -> sprite slot 32 ($80)
    clear_name_tables_to_blank_tiles(engine, r);
    reset_room_object_slots(engine, r);
    draw_room_object_sprites(engine, r);
    engine.state.set_chr_bank(1, 22);
    engine.state.set_chr_bank(2, 54);
    engine.state.scroll_pixel_x = 0;
    engine.state.nametable_select = 0;
    engine.state.scroll_y = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;

    // Blit the three words of the "CONTINUE / END" prompt from CHR text data
    // (source page $B4) into the nametable.
    vram_blit(engine, r, 107, 33, 175, 180, 9); // 9-tile word
    vram_blit(engine, r, 76, 34, 184, 180, 5); // 5-tile word
    vram_blit(engine, r, 140, 34, 189, 180, 8); // 8-tile word

    // Place the selector sprite (pose 57) at the first option (y=112).
    engine.state.player_x_tile = 5;
    engine.state.player_x_fine = 0;
    engine.state.player_y = 112;
    engine.state.player_pose = 57;
    clear_oam_with_sprite_zero_template(engine, r);
    draw_player_sprites(engine, r);
    farcall_cce4(engine, r, 224, 196, fade_two_room_palette_rows_in);

    // Toggle the selector between the two menu rows (y bit 4) until Start.
    loop {
        read_debounced_buttons(engine, r);
        if (r.value & ((crate::bits::BIT4) as u8)) != 0 {
            break;
        }
        engine.state.player_y = engine.state.player_y ^ ((crate::bits::BIT4) as u8); // flip option
        engine.state.prompt_state = 12;
    }

    engine.state.prompt_state = 24;
    if engine.state.player_y != 112 {
        // "End" chosen (selector moved off the top row): fade out, pause, and
        // return r.index=2 so the caller re-inits to the title screen.
        fade_palette_buffer_out(engine, r);
        engine.state.frame_counter = 120;
        enter_return_home(engine, 53, 193);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);
        r.index = 2;
        return;
    }

    // "Continue" chosen: restore the saved inventory snapshot but strip the first
    // three item slots, reset to the home character/room (character 6, cell 3,16).
    restore_inventory_state_snapshot(engine, r);
    engine.state.set_item_slot(0, 255);
    engine.state.set_item_slot(1, 255);
    engine.state.set_item_slot(2, 255);
    engine.state.selected_item_slot = 3;
    engine.state.character_index = 6;
    engine.state.map_screen_x = 3;
    engine.state.map_screen_y = 16;
    fade_palette_buffer_out(engine, r);
    engine.state.song = 2; // home/overworld song
    clear_name_tables_to_blank_tiles(engine, r);
    upload_status_panel_template(engine, r);
    sync_health_hud(engine, r);
    sync_magic_hud(engine, r);
    sync_key_hud(engine, r);
    sync_coin_hud(engine, r);
    farcall_cce4(engine, r, 242, 200, scene_assemble);

    r.value = 15;
    for x in (0..=31).rev() {
        engine.state.set_palette_buffer(x, 15); // start palette at black ($0F)
    }
    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    farcall_cce4(engine, r, 180, 196, fade_room_palette_row_in);
    r.index = 1; // 1 -> caller re-inits gameplay at the home room
}

/// Shows the player sprite pose in `r.index`/`r.offset` for eight foreground
/// frames.
pub fn show_player_pose_for_eight_frames(engine: &mut Engine, r: &mut RoutineContext) {
    // Apply pose (r.index) and facing (r.offset), redraw, and hold 8 frames.
    engine.state.player_pose = (r.index as u8);
    engine.state.player_facing = (r.offset as u8);
    engine.state.frame_counter = 8;
    draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Fades the title-screen palette from black to its ROM palette in five steps.
pub fn fade_title_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch1 = 64; // dim amount, 64 -> 0 in steps of 16
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Reload the full title palette, then dim all 32 entries by the current
        // step amount (scratch1) read inside dim_palette_range_by_step.
        load_title_palette_buffer(engine, r);
        r.index = 4; // start palette index (preserve the 4 fixed entries 0-3)
        r.offset = 28; // 28 entries (4..31)
        dim_palette_range_by_step(engine, r);

        enter_return_home(engine, 53, 193);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);

        // Step the dim amount down by 16; stop once it underflows (bit 7 set).
        engine.state.scratch1 = (engine.state.scratch1 - 16) & ((crate::bits::BYTE_MASK) as u8);
        if (engine.state.scratch1 & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    upload_palette_buffer(engine, r);
}

/// Commits pending foreground frame work and waits for the active frame counter
/// to expire.
pub fn commit_foreground_frame_and_wait(engine: &mut Engine, r: &mut RoutineContext) {
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Fades the base palette buffer at `0x0180..0x01A0` toward black over four
/// frame-counter steps.
pub fn fade_palette_buffer_out(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 4; // 4 dim steps
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Darken all 33 palette entries (0..=32): drop the brightness nibble by
        // one level (subtract 16), clamping to NES black ($0F).
        for x in (0..=32).rev() {
            let v = engine.state.palette_buffer(x);
            let lo = v & crate::bits::LOW_NIBBLE; // hue
            let hi = v & crate::bits::HIGH_NIBBLE; // brightness
            engine.state.scratch0 = (lo as u8);
            engine.state.set_palette_buffer(
                x,
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32) // one level dimmer
                } else {
                    15 // already darkest -> $0F (black)
                },
            );
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
}

/// Fades in the first room palette row from the active room data pointer.
pub fn fade_room_palette_row_in(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = ((engine.state.palette_src_ptr()) as u16 as i32);
    let mut v = 64; // dim amount 64 -> 0 in steps of 16
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Reload the 4-entry palette row from the room data (offsets 224..227 of
        // the room descriptor) into the palette buffer, then dim it by the step.
        for y in 224..228 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte((ptr + y) as u16 as i32));
        }
        r.index = 0; // palette index 0
        r.offset = 4; // 4 entries (one row)
        dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8); // step down by 16
        engine.state.scratch1 = v;
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break; // underflowed -> fully bright
        }
    }
    upload_palette_buffer(engine, r);
}

/// Fades in the first two room palette rows from the active room data pointer.
pub fn fade_two_room_palette_rows_in(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = ((engine.state.palette_src_ptr()) as u16 as i32);
    let mut v = 64; // dim amount 64 -> 0 in steps of 16
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Reload two 4-entry palette rows from room data (descriptor offsets
        // 224..227 and 240..243) into the palette buffer.
        for y in 224..228 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte((ptr + y) as u16 as i32));
        }
        for y in 240..244 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte((ptr + y) as u16 as i32));
        }
        // Dim both rows (palette index 0 and 16) by the current step.
        r.index = 0; // first row
        r.offset = 4;
        dim_palette_range_by_step(engine, r);
        r.index = 16; // second row
        r.offset = 4;
        dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8); // step down by 16
        engine.state.scratch1 = v;
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break; // underflowed -> fully bright
        }
    }
    upload_palette_buffer(engine, r);
}

/// Advances frames until all controller buttons are released.
///
/// Spins redrawing the scene each frame and reading the controller; exits once
/// no button is held (or the frame runner asks to stop). Used to debounce a
/// press before sampling the next one.
pub fn wait_for_buttons_released(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        // Redraw the current scene and sample the controller for this frame.
        let buttons = frame::redraw_scene_and_read_buttons(engine, r);
        // No buttons held -> done debouncing.
        if buttons == 0 {
            break;
        }
        // Allow the frame runner to abort the busy-wait (e.g. shutdown).
        if frame::frame_runner_stop_requested() {
            break;
        }
    }
}

/// Advances frames until any controller button is pressed, then stores that
/// button byte in `r.value` (A) and the global button latch `buttons` (`0x20`).
pub fn wait_for_button_press(engine: &mut Engine, r: &mut RoutineContext) {
    // Spin until a button is held, redrawing the scene each frame.
    let buttons = loop {
        let buttons = frame::redraw_scene_and_read_buttons(engine, r);
        if buttons != 0 {
            break buttons;
        }
        // Aborted by the frame runner: report "no buttons".
        if frame::frame_runner_stop_requested() {
            break 0;
        }
    };
    // Publish the pressed buttons in A and the global latch for callers.
    r.value = (buttons as u8);
    engine.state.buttons = (buttons as u8);
}

/// Scans live object slots for a damageable actor overlapping the projected
/// position in `0x0E/0x0F/0x0A` (`indirect_ptr_hi` = target tile X,
/// `indirect_ptr_lo` = target X sub-pixel, `scratch2` = target Y).
///
/// Iterates logical slots 9..=0 (byte offsets 144,128,...,0, stepping by 16).
/// Slots are skipped when they are the current slot (`slot_index`), are
/// inactive (high state bit set), are not the right object class (state != 1
/// and < 26), are the removal-projectile tile (225), or are flagged
/// non-collidable (attr bit5). The overlap test passes when the Y delta is
/// within 16px and the X tile/sub-pixel deltas indicate the boxes touch. On a
/// hit, `scratch0` (`0x08`) receives the logical slot index and `scratch1`
/// (`0x09`) the object-slot byte offset, and carry (`r.carry`) is set; carry is
/// cleared when no overlap is found.
pub fn find_damageable_actor_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 9; // logical slot index, scanning high -> low
    let mut x = 144; // object-slot byte offset for slot 9 (9 * 16)
    loop {
        // Skip the slot owned by the caller (self-collision guard).
        let mut skip = (y as u8 as i32) == (engine.state.slot_index as i32);
        // Skip inactive slots (high state bit marks "dying"/empty).
        if !skip && (engine.state.object_state(x) & crate::bits::BIT7) != 0 {
            skip = true;
        }
        // Only actors qualify: state == 1, or a damageable-actor state (>= 26).
        if !skip && engine.state.object_state(x) != 1 && engine.state.object_state(x) < 26 {
            skip = true;
        }
        // Skip the tile-removal projectile placeholder (tile id 225).
        if !skip && (engine.state.object_tile(x) & crate::bits::CLEAR_BITS_1_2) == 225 {
            skip = true;
        }
        // Skip slots flagged as non-collidable (attribute bit5).
        if !skip && (engine.state.object_attr(x) & crate::bits::BIT5) != 0 {
            skip = true;
        }
        if !skip {
            // Vertical overlap: |target_Y - object_Y| must be < 16 px. The
            // wrapped subtraction yields a small value (<16) above or a large
            // near-256 value (>=241) below to pass the band test.
            let mut d =
                ((engine.state.scratch2 - ((engine.state.object_y_pixel(x)) as u8)) as u8 as i32);
            if !(d < 16) && d < 241 {
                skip = true;
            }
            if !skip {
                // Horizontal tile delta between target and object.
                d = ((engine.state.indirect_ptr_hi - ((engine.state.object_x_tile(x)) as u8)) as u8
                    as i32);
                if d == 0 {
                    // Same tile column -> definite overlap.
                    engine.state.scratch0 = (y as u8);
                    engine.state.scratch1 = (x as u8);
                    r.carry = 1;
                    return;
                }
                if d < 2 {
                    // One tile to the right: overlap only if the sub-pixel
                    // difference wraps negative (object straddles into target).
                    d = ((engine.state.indirect_ptr_lo - ((engine.state.object_x_sub(x)) as u8))
                        as u8 as i32);
                    if (d & crate::bits::BIT7) != 0 {
                        engine.state.scratch0 = (y as u8);
                        engine.state.scratch1 = (x as u8);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 255 {
                    // More than one tile away (and not wrapped) -> no overlap.
                    skip = true;
                } else {
                    // One tile to the left (delta wrapped to 255): overlap when
                    // the sub-pixel delta is positive and nonzero.
                    d = ((engine.state.indirect_ptr_lo - ((engine.state.object_x_sub(x)) as u8))
                        as u8 as i32);
                    if d != 0 && (d & crate::bits::BIT7) == 0 {
                        engine.state.scratch0 = (y as u8);
                        engine.state.scratch1 = (x as u8);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                }
            }
        }
        let _ = skip;
        // Step to the next lower object slot (16 bytes per slot).
        x = ((x - 16) as u8 as i32);
        y -= 1;
        if y < 0 {
            break;
        }
    }
    // No overlapping actor found.
    r.carry = 0;
}

/// Scans live object slots for any nonempty, non-high-bit object overlapping
/// the projected player position in `0x0E/0x0F/0x0A` (`indirect_ptr_hi` = tile
/// X, `indirect_ptr_lo` = X sub-pixel, `scratch2` = Y).
///
/// Mirrors [`find_damageable_actor_overlap`] but starts one slot higher
/// (logical slot 10, byte offset 160) and accepts any active object: it skips
/// only the current slot, empty slots (state == 0), inactive slots (high state
/// bit), the removal-projectile tile (225), and non-collidable slots (attr
/// bit5). On a hit, `scratch0`/`scratch1` get the logical slot and byte offset
/// and carry is set; otherwise carry is cleared.
pub fn find_player_object_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 10; // logical slot index (includes the player slot)
    let mut x = 160; // byte offset for slot 10 (10 * 16)
    loop {
        // Skip the caller's own slot.
        let mut skip = (y as u8 as i32) == (engine.state.slot_index as i32);
        // Skip empty slots.
        if !skip && engine.state.object_state(x) == 0 {
            skip = true;
        }
        // Skip inactive slots (high state bit set).
        if !skip && (engine.state.object_state(x) & crate::bits::BIT7) != 0 {
            skip = true;
        }
        // Skip the tile-removal projectile placeholder (tile id 225).
        if !skip && (engine.state.object_tile(x) & crate::bits::CLEAR_BITS_1_2) == 225 {
            skip = true;
        }
        // Skip non-collidable slots (attribute bit5).
        if !skip && (engine.state.object_attr(x) & crate::bits::BIT5) != 0 {
            skip = true;
        }
        if !skip {
            // Vertical band test: within 16 px (small delta or near-256 wrap).
            let mut d =
                ((engine.state.scratch2 - ((engine.state.object_y_pixel(x)) as u8)) as u8 as i32);
            if !(d < 16) && d < 241 {
                skip = true;
            }
            if !skip {
                // Horizontal tile delta.
                d = ((engine.state.indirect_ptr_hi - ((engine.state.object_x_tile(x)) as u8)) as u8
                    as i32);
                if d == 0 {
                    // Same tile column -> overlap.
                    engine.state.scratch0 = (y as u8);
                    engine.state.scratch1 = (x as u8);
                    r.carry = 1;
                    return;
                }
                if d < 2 {
                    // One tile right: overlap if sub-pixel delta wrapped negative.
                    d = ((engine.state.indirect_ptr_lo - ((engine.state.object_x_sub(x)) as u8))
                        as u8 as i32);
                    if (d & crate::bits::BIT7) != 0 {
                        engine.state.scratch0 = (y as u8);
                        engine.state.scratch1 = (x as u8);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 255 {
                    // More than one tile away -> no overlap.
                    skip = true;
                } else {
                    // One tile left (delta == 255): overlap if sub-pixel delta
                    // is positive and nonzero.
                    d = ((engine.state.indirect_ptr_lo - ((engine.state.object_x_sub(x)) as u8))
                        as u8 as i32);
                    if d != 0 && (d & crate::bits::BIT7) == 0 {
                        engine.state.scratch0 = (y as u8);
                        engine.state.scratch1 = (x as u8);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                }
            }
        }
        let _ = skip;
        // Step to the next lower slot.
        x = ((x - 16) as u8 as i32);
        y -= 1;
        if y < 0 {
            break;
        }
    }
    // No overlapping object found.
    r.carry = 0;
}

/// Tracks the player's floor/contact state after movement has either committed
/// or failed. `fall_frames` (`0x4E`) is the falling/contact frame counter and
/// `pose_state` (`0x4D`) records whether the player is tile-aligned. While
/// airborne (`airborne_flag`, `0x4F`) or mid-jump (`jump_timer`, `0x86`) the
/// probe is suppressed and both counters reset.
///
/// When grounded, it resolves the room tile beneath the player and probes for a
/// supporting solid tile (and, in cave/spell modes, a slot that can be stood
/// on). Finding support routes to [`resolve_player_landing_or_hazard_contact`];
/// otherwise `fall_frames` increments to keep accumulating fall height.
pub fn update_player_terrain_contact(engine: &mut Engine, r: &mut RoutineContext) {
    // Airborne or jumping: no ground contact this frame, reset both counters.
    if engine.state.airborne_flag != 0 || engine.state.jump_timer != 0 {
        engine.state.pose_state = 0;
        engine.state.fall_frames = 0;
        return;
    }

    // Point the room-tile resolver at the tile just below the player's feet.
    engine.state.data_ptr_lo = engine.state.player_x_tile;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.data_ptr_hi = engine.state.player_y;
    engine.state.scratch2 = engine.state.player_y + 1; // probe one pixel down
    resolve_room_tile_pointer(engine, r);

    // When tile-aligned horizontally, test the single tile directly underfoot.
    if engine.state.player_x_fine == 0 {
        engine.state.pose_state = 1; // aligned pose
        r.offset = 0;
        let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
        // A floor tile (low-6 collision bits == 0 means solid floor here).
        if (engine
            .state
            .byte((tile_ptr + (r.offset as i32)) as u16 as i32)
            & crate::bits::LOW_6_BITS)
            == 0
        {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
    }

    engine.state.pose_state = 0;
    // Below the playfield bottom (y >= 176): keep falling.
    if engine.state.player_y >= 176 {
        engine.state.fall_frames =
            (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
        return;
    }

    // Check whether the player is standing on a damageable actor.
    find_damageable_actor_overlap(engine, r);
    if ((r.carry) != 0) {
        // In cave/spell CHR mode (bank3 >= 48) any actor counts as ground.
        if engine.state.chr_bank(3) >= 48 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let selected_slot = engine.state.selected_item_slot;
        let selected_item = engine.state.item_slot(selected_slot as i32);
        // Item 5 (the stomp/jump item) while falling kills the actor instead of
        // standing on it; any other case lands normally.
        if selected_item != 5 || engine.state.fall_frames == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let hit_slot = engine.state.scratch1;
        engine.state.set_object_state((hit_slot as i32), 128); // mark actor dying
    }

    // Probe the supporting tile under the player's left foot (offset 1).
    r.offset = 1;
    probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    // Tile-aligned with no solid under the left foot: keep falling.
    if engine.state.player_x_fine == 0 {
        engine.state.fall_frames =
            (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
        return;
    }

    // Otherwise probe the right foot's supporting tile (offset 13).
    r.offset = 13;
    probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    // No support found under either foot: still falling.
    engine.state.fall_frames = (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
}

/// Converts a just-detected floor/object/hazard contact into damage, recoil,
/// hazard invulnerability, or a reset of the fall counter.
///
/// If the accumulated fall height (`fall_frames`, `0x4E`) reached the safe-fall
/// threshold (`jump_strength`, `0x49`), the excess becomes recoil: a bounce
/// timer (`jump_timer`) and landing-stun timer (`landing_timer = bounce + 10`)
/// are set, the fall-damage prompt (state 10) is shown, and one health point is
/// spent. On a clean landing (`fall_frames == 0`) the under-foot tiles are
/// checked for spike/lava hazard contact. Either way `fall_frames` is cleared.
fn resolve_player_landing_or_hazard_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let mut fall_frames = engine.state.fall_frames;
    // Fell far enough to take fall damage (>= safe-fall threshold).
    if fall_frames >= engine.state.jump_strength {
        // Compute the rebound height from the overshoot, clamped to the
        // threshold, minus one frame.
        fall_frames = ((fall_frames - 7) as u8); // subtract the free 7-frame margin
        if fall_frames >= engine.state.jump_strength {
            fall_frames = engine.state.jump_strength; // clamp the bounce height
        }
        fall_frames = ((fall_frames - 1) as u8);
        engine.state.jump_timer = fall_frames; // schedule the bounce
        engine.state.landing_timer = fall_frames + 10; // stun = bounce + 10 frames
        engine.state.prompt_state = 10; // fall-damage prompt
        consume_health_point(engine, r);
    }
    // A soft landing (no accumulated fall) can still touch a hazard tile.
    if engine.state.fall_frames == 0 {
        r.offset = 1; // left foot
        apply_hazard_tile_contact(engine, r);
        // If the left foot found no hazard and the player straddles tiles,
        // check the right foot too.
        if ((r.carry) == 0) && engine.state.player_x_fine != 0 {
            r.offset = 13; // right foot
            apply_hazard_tile_contact(engine, r);
        }
    }
    // Reset the fall counter now that contact is resolved.
    engine.state.fall_frames = 0;
}

/// Handles the room tile sampled at the current projected player footprint.
/// Special tiles can spend keys/magic, spawn transient objects, or launch the
/// tile-removal projectile; ordinary tiles return carry for solid terrain.
///
/// `r.offset` (Y) is the byte offset into the resolved tile column at
/// `data_ptr()`; the low-6 collision bits select a behavior:
/// - tile == `text_attr_ptr_lo`: the "secret" / scriptable tile — spawns the
///   removal projectile in slot 144 carrying `text_attr_ptr_hi`, and returns
///   carry if that action value indicates solid terrain (>= 48).
/// - tile == 2: a locked tile — spends magic (selected item 7) or a key, then
///   spawns the removal projectile carrying `room_tile_action`.
/// - tile == 62: an item-interactable tile — dispatches on the selected
///   carried item (1 = lamp/light spell, 2 = thrown weapon, 3 = magic blast).
/// - otherwise: carry set when the tile is solid (>= 48).
///
/// On success a transient object is seeded in slot 144 (`obj_*` scratch) and
/// the prompt/sound state is poked. Carry (`r.carry`) reports solid terrain to
/// the movement caller.
pub fn dispatch_room_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    // Read the collision class of the tile at column+offset (low-6 bits).
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    let tile_offset = r.offset;
    let tile = engine
        .state
        .byte((tile_ptr + (tile_offset as i32)) as u16 as i32)
        & crate::bits::LOW_6_BITS;
    // Scriptable "secret" tile whose id is stored in text_attr_ptr_lo.
    if tile == (engine.state.text_attr_ptr_lo as i32) {
        // Only spawn the removal projectile if slot 144 is free.
        if engine.state.object_state(144) == 0 {
            engine.state.scratch3 = (tile_offset as u8);
            engine.state.obj_tile = 225; // tile-removal projectile placeholder
            engine.state.obj_state = 1;
            engine.state.obj_attr = 1;
            engine.state.obj_move_scratch = engine.state.text_attr_ptr_hi; // carried action value
            engine.state.obj_timer = 10; // projectile lifetime (frames)
            seed_object_position_from_tile_offset(engine, r);
            store_object_slot_scratch(engine, r);
            engine.state.prompt_state = 6; // "blocked"/interaction sound
        }
        // Report solidity from the action value's collision class.
        let v = engine.state.text_attr_ptr_hi & ((crate::bits::LOW_6_BITS) as u8);
        r.value = (v as u8);
        r.carry = ((v >= 48) as u8); // >= 48 => solid terrain
        return;
    }
    // Locked tile: requires a key, or magic when the lockpick item is equipped.
    if tile == 2 {
        if engine.state.object_state(144) == 0 {
            engine.state.scratch3 = (tile_offset as u8);
            r.index = (engine.state.selected_item_slot as u8);
            let item = engine.state.item_slot(r.index as i32);
            r.value = (item as u8);
            // Item 7 (lockpick) tries magic first and unlocks without a key on
            // success. On magic failure the ROM falls through ($DDF2 BCC not
            // taken -> $DDF4) to spend a key — as does any other item; with no
            // key the tile stays solid ($DDF7 BCS $DE18).
            let mut need_key = true;
            if item == 7 {
                r.index = (engine.state.selected_item_slot as u8);
                consume_magic_point(engine, r);
                if ((r.carry) == 0) {
                    need_key = false; // magic spent: skip the key ($DDF2 BCC $DDF9)
                }
            }
            if need_key {
                consume_key(engine, r);
                if ((r.carry) != 0) {
                    r.carry = 1; // no key: stay solid ($DDF7 BCS $DE18)
                    return;
                }
            }
            // Unlocked: launch the removal projectile carrying the tile action.
            engine.state.obj_tile = 225;
            engine.state.obj_state = 1;
            engine.state.obj_attr = 1;
            engine.state.obj_move_scratch = engine.state.room_tile_action;
            engine.state.obj_timer = 15; // projectile lifetime
            seed_object_position_from_tile_offset(engine, r);
            store_object_slot_scratch(engine, r);
            engine.state.prompt_state = 6;
        }
        r.carry = 1; // locked tile is solid until consumed
        return;
    }
    // Item-interactable tile (id 62): only when the action button (bit7) is held
    // and the projectile slot is free.
    if tile == 62 {
        if (engine.state.buttons & ((crate::bits::BIT7) as u8)) != 0
            && engine.state.object_state(144) == 0
        {
            engine.state.scratch3 = (tile_offset as u8);
            engine.state.obj_move_state = 1;
            r.offset = (engine.state.selected_item_slot as u8);
            r.index = ((engine.state.item_slot(r.offset as i32)) as u8);
            let idx = r.index; // selected carried-item id
            // Item 2: the light spell / lamp that converts a tile in place
            // ($DE2D DEX;DEX; $DE2F BEQ $DE3F).
            if idx == 2 {
                if engine.state.player_magic != 0 {
                    // Only act when the player is tile-aligned (low Y nibble and
                    // X sub-pixel both zero).
                    let mut t = engine.state.player_y & ((crate::bits::LOW_NIBBLE) as u8);
                    t |= engine.state.player_x_fine;
                    if t == 0 {
                        // Pick the spawn offset for the facing direction; each
                        // direction has a 2-byte (X,Y) entry in the spawn tables.
                        let x2 = (((engine.state.direction_latch
                            & ((crate::bits::LOW_NIBBLE) as u8))
                            << 1) as u8 as i32);
                        // Place the projectile one tile ahead in X.
                        let lo = ((engine.state.player_x_tile
                            + ((engine
                                .state
                                .byte((SPAWN_OFFSET_X_TABLE + x2) as u16 as i32))
                                as u8)) as u8 as i32);
                        engine.state.set_object_x_tile(144, lo);
                        engine.state.data_ptr_lo = (lo as u8);
                        engine.state.set_object_x_sub(144, 0);
                        // ... and the corresponding Y offset.
                        let hi = ((engine.state.player_y
                            + ((engine
                                .state
                                .byte((SPAWN_OFFSET_Y_TABLE + x2) as u16 as i32))
                                as u8)) as u8 as i32);
                        engine.state.set_object_y_pixel(144, hi);
                        engine.state.data_ptr_hi = (hi as u8);
                        // Resolve the tile at that target position.
                        resolve_room_tile_pointer(engine, r);
                        r.offset = 0;
                        engine.state.scratch3 = 0;
                        let p = ((engine.state.data_ptr()) as u16 as i32);
                        let b = engine.state.byte(p) & crate::bits::LOW_6_BITS;
                        // Only fire if the targeted tile is also interactable (62).
                        if b == 62 {
                            engine.state.set_object_tile(144, 225); // removal projectile
                            engine.state.set_object_state(144, 1);
                            engine.state.set_object_attr(144, 1);
                            engine.state.set_object_timer(144, 15); // lifetime frames
                            read_room_tile_action_value(engine, r);
                            engine.state.set_object_move_scratch(144, (r.value as i32));
                            consume_magic_point(engine, r);
                            engine.state.prompt_state = 20; // cast-spell prompt
                        }
                    }
                }
                r.carry = 1; // tile remains solid
                return;
            }
            // Item 3: thrown weapon — launches a moving projectile in the faced
            // direction ($DE31 DEX; $DE32 BEQ $DE39 -> $DE9F).
            if idx == 3 {
                if (engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
                    r.offset = 1; // speed/profile selector
                    build_direction_velocity(engine, r);
                    r.offset = 248; // tile-table index for the projectile sprite
                    // Projectile sprite tile from the tile table at index 248,
                    // forced even (CLEAR_BIT0) for the 2-tile sprite pair.
                    let p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
                    engine.state.obj_tile = ((engine.state.byte((p79 + 248) as u16 as i32)
                        & crate::bits::CLEAR_BIT0)
                        as u8);
                    engine.state.obj_state = 1;
                    engine.state.obj_attr = 3;
                    // Replace the source tile with its post-action value.
                    r.offset = (engine.state.scratch3 as u8);
                    let b = engine
                        .state
                        .byte((tile_ptr + (r.offset as i32)) as u16 as i32);
                    engine.state.obj_move_scratch = (b as u8);
                    engine.state.obj_timer = 16; // projectile lifetime
                    read_room_tile_action_value(engine, r);
                    engine.state.set_byte(
                        ((tile_ptr + (r.offset as i32)) as u16 as i32),
                        (r.value as i32),
                    );
                    seed_object_position_from_tile_offset(engine, r);
                    redraw_room_tile_column(engine, r);
                    update_tile_projectile_motion(engine, r);
                    engine.state.slot_index = 255; // no owning slot
                    if engine.state.object_state(144) != 0 {
                        engine.state.prompt_state = 6;
                    }
                }
                // Cancel residual vertical motion on a throw.
                engine.state.vertical_delta = 0;
                engine.state.fall_frames = 0;
                r.carry = 1;
                return;
            }
            // Item 4: magic blast — like the throw but spends magic and uses a
            // higher launch speed ($DE34 DEX; $DE35 BEQ $DE3C -> $DEE8).
            if idx == 4 {
                if engine.state.player_magic != 0 {
                    if (engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
                        r.offset = 8; // faster velocity profile
                        build_direction_velocity(engine, r);
                        r.offset = 248; // projectile sprite tile index
                        let p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
                        engine.state.obj_tile = ((engine.state.byte((p79 + 248) as u16 as i32)
                            & crate::bits::CLEAR_BIT0)
                            as u8);
                        engine.state.obj_state = 1;
                        engine.state.obj_attr = 3;
                        // Consume the source tile, storing its post-action value.
                        r.offset = (engine.state.scratch3 as u8);
                        let b = engine
                            .state
                            .byte((tile_ptr + (r.offset as i32)) as u16 as i32);
                        engine.state.obj_move_scratch = (b as u8);
                        engine.state.obj_timer = 0; // unlimited lifetime for the blast
                        read_room_tile_action_value(engine, r);
                        engine.state.set_byte(
                            ((tile_ptr + (r.offset as i32)) as u16 as i32),
                            (r.value as i32),
                        );
                        seed_object_position_from_tile_offset(engine, r);
                        redraw_room_tile_column(engine, r);
                        update_tile_projectile_motion(engine, r);
                        engine.state.slot_index = 255;
                        if engine.state.obj_state != 0 {
                            engine.state.prompt_state = 20; // cast-spell prompt
                            consume_magic_point(engine, r);
                        }
                        engine.state.vertical_delta = 0;
                        engine.state.fall_frames = 0;
                        r.carry = 1;
                        return;
                    }
                    // No facing direction: nothing fired, still solid.
                    engine.state.vertical_delta = 0;
                    engine.state.fall_frames = 0;
                    r.carry = 1;
                    return;
                }
                r.carry = 1; // out of magic
                return;
            }
        }
        r.carry = 1; // interactable tile is solid until acted on
        return;
    }
    // Ordinary tile: solid when its collision class is >= 48.
    r.carry = ((tile >= 48) as u8);
}

/// Fades the room palette out over 4 steps and resets active audio channel
/// state. Each step darkens every background palette entry by one luminance
/// level (one nibble down) and halves the music channel volumes, so the screen
/// and music fade together; afterwards the song is cleared.
pub fn fade_room_palette_out_reset_audio(engine: &mut Engine, r: &mut RoutineContext) {
    // Suppress normal music-volume handling while we fade it out manually.
    engine.state.music_volume_override =
        (engine.state.music_volume_override + 1) & ((crate::bits::BYTE_MASK) as u8);
    let mut y = 4; // number of dim steps
    loop {
        engine.state.frame_counter = 5; // 5 frames per fade step
        // Darken the 29 background palette entries (indices 4..=32).
        for x in (0..=28).rev() {
            let v = engine.state.palette_buffer(4 + x); // 4 = first BG palette byte
            let lo = v & crate::bits::LOW_NIBBLE; // hue
            let hi = v & crate::bits::HIGH_NIBBLE; // luminance
            engine.state.scratch0 = (lo as u8);
            engine.state.set_vram_stage(
                68 + x, // 68 = VRAM stage offset for BG palette
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32) // drop one luminance level
                } else {
                    15 // already darkest: black (color 0x0F)
                },
            );
        }
        // Halve the three music volume bytes for channel group 13.
        engine
            .state
            .set_sound_channel_byte(13, 0, engine.state.sound_channel_byte(13, 0) >> 1);
        engine
            .state
            .set_sound_channel_byte(13, 16, engine.state.sound_channel_byte(13, 16) >> 1);
        engine
            .state
            .set_sound_channel_byte(13, 48, engine.state.sound_channel_byte(13, 48) >> 1);
        engine.state.set_sound_channel_byte(1, 32, 0); // mute one channel byte
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
    // Fully silence audio and mark "no song playing".
    engine.state.song = 255;
    engine.state.set_sound_channel_byte(1, 0, 0);
    engine.state.sound_channel_flags = 0;
    engine.state.set_sound_channel_byte(1, 48, 0);
    engine.state.music_volume_override = 0;
}

/// Fades the room palette out over 4 steps while preserving active audio
/// channel state. Identical to [`fade_room_palette_out_reset_audio`] minus the
/// volume halving/song reset, used when the music should keep playing across the
/// fade (e.g. entering a shop).
pub fn fade_room_palette_out_keep_audio(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 4; // number of dim steps
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        // Darken the 29 background palette entries (indices 4..=32).
        for x in (0..=28).rev() {
            let v = engine.state.palette_buffer(4 + x); // 4 = first BG palette byte
            let lo = v & crate::bits::LOW_NIBBLE; // hue
            let hi = v & crate::bits::HIGH_NIBBLE; // luminance
            engine.state.scratch0 = (lo as u8);
            engine.state.set_vram_stage(
                68 + x, // VRAM stage offset for BG palette
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32) // drop one luminance level
                } else {
                    15 // already darkest: black
                },
            );
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
}

/// Rebuilds room palette attributes and fades the gameplay palette back in.
///
/// `scratch1` (`0x09`) is the dim amount, started at 64 and decremented by 16
/// each step (64,48,32,16,0) until it underflows negative (bit7 set) after the
/// final fully-bright step. Each step rebuilds the true palette then dims it by
/// the current amount, so the room brightens smoothly to full.
pub fn fade_room_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    let mut v = 64; // initial dim amount (4 steps of 16)
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5; // 5 frames per step
        build_room_palette_buffer(engine, r); // rebuild full-brightness palette
        r.index = 4; // first palette entry to dim
        r.offset = 28; // count of entries to dim (29 entries)
        dim_palette_range_by_step(engine, r); // apply current dim amount
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8); // brighten by one step
        engine.state.scratch1 = v;
        // Stop once the amount underflows past 0 (final step was full brightness).
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    upload_palette_buffer(engine, r);
}

/// Flashes the screen `r.index` (X) times by alternating a 1-frame all-white
/// palette fill with a 2-frame rebuilt-palette display. Used for the
/// character-select reveal and special-exit effects. Returns with `r.index` == 0.
pub fn flash_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x = r.index; // flash count
    loop {
        // Fill all 32 palette entries with the bright color 48 (0x30 = white).
        for i in (0..=31).rev() {
            engine.state.set_palette_buffer(i, 48);
        }
        upload_palette_buffer(engine, r);
        engine.state.frame_counter = 1; // white flash held 1 frame
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        // Restore the true room palette for 2 frames.
        build_room_palette_buffer(engine, r);
        upload_palette_buffer(engine, r);
        engine.state.frame_counter = 2;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        x = ((x - 1) as u8);
        if x == 0 {
            break;
        }
    }
    r.index = x;
}

/// Animates the player's health counting up to the cap (99) one point per two
/// frames, syncing the HUD and playing the refill prompt sound each tick. The
/// sprite blink timer is forced off during the animation and restored after.
pub fn animate_health_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count health up one point at a time so the HUD and prompt animation match
    // the original refill reward pacing.
    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0; // no flicker while refilling
    draw_player_sprites(engine, r);
    loop {
        // Add one health point and update the on-screen counter.
        engine.state.player_health = engine.state.player_health.wrapping_add(1);
        sync_health_hud(engine, r);
        engine.state.prompt_state = 22; // refill "tick" sound/prompt
        engine.state.frame_counter = 2; // 2 frames per point
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_health;
        if engine.state.player_health >= 99 {
            break; // reached the 99 cap
        }
    }
    // Final "done" prompt and restore the saved blink state.
    engine.state.prompt_state = 23;
    engine.state.frame_counter = 16; // $D18E LDA #$10: hold the done prompt 16 frames
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r); // $D192 JSR $C135 waits for the counter
    engine.state.sprite_blink_timer = saved_blink;
}

/// Animates the player's magic counting up to the cap (99), sharing the exact
/// prompt/blink/pacing of [`animate_health_refill_to_cap`] but on the magic
/// counter and HUD.
pub fn animate_magic_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count magic up one point at a time, sharing the same prompt/blink pacing
    // as the health refill.
    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0; // no flicker while refilling
    draw_player_sprites(engine, r);
    loop {
        // Add one magic point and update the on-screen counter.
        engine.state.player_magic = engine.state.player_magic.wrapping_add(1);
        sync_magic_hud(engine, r);
        engine.state.prompt_state = 22; // refill "tick" sound/prompt
        engine.state.frame_counter = 2; // 2 frames per point
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_magic;
        if engine.state.player_magic >= 99 {
            break; // reached the 99 cap
        }
    }
    // Final "done" prompt and restore the saved blink state.
    engine.state.prompt_state = 23;
    engine.state.frame_counter = 16; // $D18E LDA #$10: hold the done prompt 16 frames
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r); // $D192 JSR $C135 waits for the counter
    engine.state.sprite_blink_timer = saved_blink;
}

/// Spends a key and runs the door-unlock prompt/music sequence. Carry is set
/// only when a key was available and the door event completed.
///
/// On no key, plays the "blocked" prompt (state 6) and returns carry clear.
/// Otherwise it reveals the door object in slot 160 (state/tile derived from
/// the door id at room metadata offset 10), plays the unlock jingle (song 14)
/// for 120 frames, restores the previous song, and returns carry set.
pub fn unlock_door_with_key(engine: &mut Engine, r: &mut RoutineContext) {
    // consume_key sets carry when the player has no keys.
    consume_key(engine, r);
    if ((r.carry) != 0) {
        engine.state.prompt_state = 6; // "blocked" prompt
        r.carry = 0;
        return;
    }

    // Read the door type id from room metadata (offset 10) and configure the
    // door object in slot 160.
    let ptr = ((engine.state.palette_src_ptr()) as u16 as i32);
    let door = engine.state.byte((ptr + 10) as u16 as i32);
    if door < 8 {
        engine.state.set_object_attr(160, 0); // low door ids use palette 0
    }
    engine.state.set_object_state(160, door + 2); // door open-animation state
    engine
        .state
        // base door tile 129, +4 tiles per door id
        .set_object_tile(160, ((door << 2) & crate::bits::BYTE_MASK) + 129);
    engine.state.prompt_state = 31; // door-open prompt
    draw_room_object_sprites(engine, r);

    // Freeze sprite blink while the unlock animation plays.
    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0;
    draw_player_sprites(engine, r);

    // Play the door-unlock jingle (song 14).
    let saved_song = engine.state.song;
    engine.state.song = 14;
    r.value = 14;
    song_init(engine, r);

    engine.state.frame_counter = 120; // hold for 120 frames (~2 seconds)
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    // Restore the prior room song and blink state.
    engine.state.song = saved_song;
    r.value = (saved_song as u8);
    song_init(engine, r);

    engine.state.sprite_blink_timer = saved_blink;
    r.carry = 1; // door unlocked
}

/// Opens the in-game character-select overlay (the loadout/status page reached
/// by the select button), waits for a press/release of the select button
/// (bit4), then restores the gameplay room.
///
/// While in overworld CHR mode (bank3 < 48) it checkpoints the room, swaps in
/// the loadout page (page 8) showing carried items/counts/stats, fades in, and
/// on exit fades out, restores the room metadata/palette/view, and fades back
/// in. The `sound_paused` counter brackets the whole overlay so music pauses.
pub fn run_character_select_overlay(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 3; // menu/overlay sound
    // Pause audio for the duration of the overlay (bracketed by a decrement).
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);

    // Only build the overlay page in the overworld (cave/spell mode skips it).
    if engine.state.chr_bank(3) < 48 {
        push_room_checkpoint(engine, r);
        r.value = 8; // loadout/status room page
        enter_temporary_room_page(engine, r);
        draw_carried_item_sprites(engine, r);
        upload_inventory_count_tiles(engine, r);
        upload_equipped_item_stat_tiles(engine, r);
        engine.state.scroll_fine_x = 8; // page-aligned scroll
        refresh_scroll_register_shadows(engine, r);
        // Pose was already computed by enter_temporary_room_page (mirrors $E6AD
        // JSR $D8E3); the original overlay ($E030) only refreshes and draws here.
        draw_player_sprites(engine, r);
        fade_room_palette_in(engine, r);
    }

    // Debounce: wait for all buttons released before reading the select press.
    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Wait for the select button (bit4) to be pressed to close the overlay.
    loop {
        if (frame::read_buttons(engine, r) & crate::bits::BIT4) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Debounce that press before returning to gameplay.
    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    engine.state.prompt_state = 4; // close-overlay sound

    // Rebuild the original gameplay room (overworld only).
    if engine.state.chr_bank(3) < 48 {
        pop_room_checkpoint(engine, r);
        fade_room_palette_out_reset_audio(engine, r);
        clear_temporary_room_sprites(engine, r);
        r.value = (engine.state.room_restore_scratch as u8); // saved song id
        switch_song_if_needed(engine, r);
        prepare_room_metadata_and_palette(engine, r);
        upload_current_room_view(engine, r);
        draw_player_sprites(engine, r);
        draw_room_object_sprites(engine, r);
        refresh_scroll_register_shadows(engine, r);
        fade_room_palette_in(engine, r);
    }

    // Resume audio (un-pause matching the entry increment).
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);
}

/// Shows the read-only inventory item-list page until the player presses a
/// button, then scrolls back to the character-selection room page.
pub fn show_inventory_item_list_screen(engine: &mut Engine, r: &mut RoutineContext) {
    // Scroll the nametable to the item-list page (tile column 16) and show it.
    engine.state.scroll_tile_x = 16;
    upload_staged_room_columns(engine, r);
    refresh_scroll_register_shadows(engine, r);

    // Build the item-list text into the buffer at screen pos 0xB4D4 (180:212).
    engine.state.indirect_ptr_lo = 212; // low byte of list draw address
    engine.state.indirect_ptr_hi = 180; // high byte of list draw address
    encode_inventory_snapshot_item_list(engine, r);
    upload_inventory_item_list(engine, r);

    // Wait for any held buttons to be released first.
    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    // Then wait for a fresh press to dismiss the list.
    loop {
        if frame::read_buttons(engine, r) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    // Scroll back to the character-select room page (tile column 32).
    engine.state.scroll_tile_x = 32;
    upload_staged_room_columns(engine, r);
    refresh_scroll_register_shadows(engine, r);
}

/// Runs the interactive inventory item-grid editor from the character-selection
/// room. The player moves a cursor over the item grid (D-pad), toggles the
/// selected entry (A/bit7), and exits with select (bit4) or B (bit5). Each
/// frame the two-sprite cursor pair (OAM 128/132 top, 144/148 bottom) is
/// redrawn over the current grid and list positions.
pub fn run_inventory_item_grid_menu(engine: &mut Engine, r: &mut RoutineContext) {
    // Scroll to the item-grid page (tile column 48) and clear the list area.
    engine.state.scroll_tile_x = 48;
    upload_staged_room_columns(engine, r);
    clear_inventory_item_list_buffer(engine, r);
    upload_inventory_item_list(engine, r);
    refresh_scroll_register_shadows(engine, r);

    // Debounce any held buttons before entering the menu loop.
    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    // Set up the cursor sprite pair: tiles 245 (top) / 247 (bottom), palette 0.
    engine.state.obj_x_sub = 0;
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_y_vel = 0;
    engine.state.set_oam_tile(128, 245); // top-left cursor tile
    engine.state.set_oam_tile(144, 245); // bottom-left cursor tile
    engine.state.set_oam_tile(132, 247); // top-right cursor tile
    engine.state.set_oam_tile(148, 247); // bottom-right cursor tile
    engine.state.set_oam_attr(128, 0);
    engine.state.set_oam_attr(132, 0);
    engine.state.set_oam_attr(144, 0);
    engine.state.set_oam_attr(148, 0);
    update_inventory_list_cursor_sprites(engine, r);
    update_inventory_grid_cursor_sprites(engine, r);

    loop {
        engine.state.frame_counter = 1;
        let b = frame::read_buttons(engine, r);
        r.value = (b as u8);

        // Dispatch on the highest-priority pressed button.
        if (b & crate::bits::BIT7) != 0 {
            // A: toggle/select the entry under the cursor.
            select_inventory_grid_entry(engine, r);
            upload_inventory_item_list(engine, r);
        } else if (b & crate::bits::BIT6) != 0 {
            // B: no-op in this menu.
        } else if (b & crate::bits::BIT0) != 0 {
            move_inventory_cursor_right(engine, r);
        } else if (b & crate::bits::BIT1) != 0 {
            move_inventory_cursor_left(engine, r);
        } else if (b & crate::bits::BIT2) != 0 {
            move_inventory_cursor_down(engine, r);
        } else if (b & crate::bits::BIT3) != 0 {
            move_inventory_cursor_up(engine, r);
            upload_inventory_item_list(engine, r);
        } else if (b & crate::bits::BIT4) != 0 {
            // Select: close the item menu (stay on the page).
            close_inventory_item_menu(engine, r);
        } else if (b & crate::bits::BIT5) != 0 {
            // Start: leave the menu back to the character-select page (col 32).
            engine.state.scroll_tile_x = 32;
            upload_staged_room_columns(engine, r);
            refresh_scroll_register_shadows(engine, r);
            restore_status_sprite_template(engine, r);
            return;
        }

        // Any non-(select|start) press plays the cursor-move tick prompt.
        if (engine.state.buttons & ((crate::bits::CLEAR_BITS_4_5) as u8)) != 0 {
            engine.state.prompt_state = 12; // cursor/coin tick prompt
            engine.state.frame_counter = 10; // brief debounce window
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special room flow used to refill resources, return carried items,
/// pick a family member, and optionally visit the inventory item pages.
///
/// Two distinct rooms share this entry, keyed on `map_screen_y` (`0xEC`):
/// - `map_screen_y != 16`: the refill/loadout room. The player can pay 10 coins
///   to fully refill health and magic and then re-equip carried items.
/// - `map_screen_y == 16`: the title/character-select room. Carried items are
///   returned to inventory, then the player walks to and selects one of the
///   five playable family members (or visits the item-list/grid pages, or uses
///   the cheat code), after which the chosen character's stats/CHR are loaded
///   and the game starts.
pub fn run_character_select_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    // --- Refill / loadout room (any room except map_screen_y == 16) ---
    if engine.state.map_screen_y != 16 {
        push_room_checkpoint(engine, r);
        r.value = 4; // refill room page
        enter_temporary_room_page(engine, r);
        draw_coin_cost_sprites(engine, r);
        fade_room_palette_in(engine, r);

        loop {
            // Walk until the player acts on a tile or exits the room.
            walk_purchase_room_until_action_or_exit(engine, r);
            if ((r.carry) != 0) {
                restore_room_from_checkpoint(engine, r);
                return; // exited the room
            }
            // The refill costs 10 coins; reject if the player can't afford it.
            if engine.state.coins < 10 {
                engine.state.prompt_state = 6; // "blocked" prompt
                continue;
            }

            // Animate spending 10 coins, one per 10 frames.
            let mut x = 10;
            loop {
                engine.state.coins = (engine.state.coins - 1) & ((crate::bits::BYTE_MASK) as u8);
                sync_coin_hud(engine, r);
                engine.state.prompt_state = 12; // coin-tick prompt
                engine.state.frame_counter = 10;
                frame::commit_frame_work(engine, r);
                frame::wait_for_frame_counter(engine, r);
                x = ((x - 1) as u8 as i32);
                if x == 0 {
                    break;
                }
            }
            // Refill health and magic on the loadout page, then re-equip items.
            fade_room_palette_out_keep_audio(engine, r);
            animate_health_refill_to_cap(engine, r);
            animate_magic_refill_to_cap(engine, r);
            r.value = 8; // loadout page
            refresh_temporary_room_page(engine, r);
            draw_carried_item_sprites(engine, r);
            upload_inventory_count_tiles(engine, r);
            upload_equipped_item_stat_tiles(engine, r);
            engine.state.scroll_fine_x = 8;
            refresh_scroll_register_shadows(engine, r);
            draw_player_sprites(engine, r);
            fade_room_palette_in(engine, r);
            run_carried_item_loadout_flow(engine, r);
            // Return to the refill page and loop for another purchase.
            r.value = 4;
            refresh_temporary_room_page(engine, r);
            clear_temporary_room_sprites(engine, r);
            draw_coin_cost_sprites(engine, r);
            fade_room_palette_in(engine, r);
        }
    }

    // --- Title / character-select room (map_screen_y == 16) ---
    engine.state.player_health = 0;
    engine.state.player_magic = 0;
    // If a real character was active (< 6), return their three carried items to
    // inventory and clear the carried-item slots.
    if engine.state.character_index < 6 {
        for y in (0..=2).rev() {
            let x = engine.state.item_slot(y);
            if (x & crate::bits::BIT7) == 0 {
                // Valid item id: increment its inventory count.
                engine.state.set_inventory_item(
                    x,
                    (engine.state.inventory_item(x) + 1) & crate::bits::BYTE_MASK,
                );
            }
            engine.state.set_item_slot(y, 255); // empty the carried slot
        }
        snapshot_inventory_state(engine, r);
    }

    push_room_checkpoint(engine, r);
    engine.state.character_index = 6; // 6 = "no character" / on the title screen
    r.value = 6; // character-select room page
    enter_temporary_room_page(engine, r);
    sync_health_hud(engine, r);
    sync_magic_hud(engine, r);
    engine.state.selected_item_slot = 3;
    draw_status_item_sprites(engine, r);
    engine.state.player_pose = 241; // title-screen pose
    engine.state.player_facing = 0;
    draw_player_sprites(engine, r);
    restore_status_sprite_template(engine, r);
    reset_room_object_slots(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        // Walk until A is pressed; the tile under the player identifies the menu
        // choice via its Y position (scratch2 high nibble) and column low nibble.
        walk_character_select_room_until_action(engine, r);
        let hi = engine.state.scratch2 & ((crate::bits::HIGH_NIBBLE) as u8);
        let mut chosen: Option<i32> = None;
        if hi == 80 {
            // Top row tile column 5: the music/cheat tile (only with a save).
            if (engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8)) == 5
                && engine.state.continue_timer != 0
            {
                // Cycle the demo song (0..15 wrapping).
                let mut x = ((engine.state.song + 1) as u8 as i32);
                if x >= 16 {
                    x = 0;
                }
                engine.state.song = (x as u8);
                song_init(engine, r);
                // Cheat: with the save flag's high bit set and buttons == 195
                // (A+B+Select+Start, 0xC3), max out all items/resources.
                if (engine.state.continue_timer & ((crate::bits::BIT7) as u8)) != 0
                    && engine.state.buttons == 195
                {
                    for x in (0..=13).rev() {
                        engine.state.set_inventory_item(x, 16); // 14 item slots -> 16 each
                    }
                    engine.state.continue_timer = 128;
                    engine.state.coins = 128;
                    engine.state.keys = 128;
                    engine.state.prompt_state = 26; // cheat-activated prompt
                }
            }
            continue;
        } else if hi == 112 {
            // Row at y nibble 0x70: family members 0 and 1.
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo == 6 {
                chosen = Some(0);
            } else if lo == 8 {
                chosen = Some(1);
            }
        } else if hi == 128 {
            // Row at y nibble 0x80: member 2, item-list page, item-grid page.
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo == 4 {
                chosen = Some(2);
            } else if lo == 10 {
                engine.state.prompt_state = 3;
                show_inventory_item_list_screen(engine, r);
                continue;
            } else if lo == 12 {
                engine.state.prompt_state = 3;
                run_inventory_item_grid_menu(engine, r);
                continue;
            }
        } else if hi == 144 {
            // Row at y nibble 0x90: family members 3 and 4.
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo == 6 {
                chosen = Some(3);
            } else if lo == 10 {
                chosen = Some(4);
            }
        }

        // Nothing selectable was under the player: keep walking.
        let Some(x) = chosen else {
            continue;
        };

        // A family member (0..4) was chosen; copy their four starting items from
        // the stats table into carried-item slots 11..14.
        engine.state.character_index = (x as u8);
        r.offset = (((x << 2) + 3) as u8); // table base: 4 entries per character, last byte
        for xi in (0..=3).rev() {
            engine.state.set_item_slot(
                11 + xi, // slots 11..=14
                engine
                    .state
                    .byte((CHARACTER_STATS_TABLE + (r.offset as i32)) as u16 as i32),
            );
            r.offset = ((r.offset - 1) as u8);
        }
        // Play the selection reveal (flash + pose change).
        engine.state.prompt_state = 24; // selection prompt
        engine.state.prompt_argument = 255;
        engine.state.frame_counter = 4;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 5; // flash 5 times
        flash_palette_buffer(engine, r);
        // Switch the character CHR banks: bank2 = per-character sprite bank
        // (56 + index), banks 3..5 = the shared 61/62/63 gameplay tiles.
        engine
            .state
            .set_chr_bank(2, ((engine.state.character_index + 56) as i32));
        engine.state.set_chr_bank(3, 61);
        engine.state.set_chr_bank(4, 62);
        engine.state.set_chr_bank(5, 63);
        engine.state.player_pose = 13; // standing pose
        engine.state.player_facing = 0;
        engine.state.player_y = engine.state.player_y & ((crate::bits::HIGH_NIBBLE) as u8); // tile-align Y
        engine.state.player_x_fine = 4;
        clear_gameplay_object_sprites(engine, r);
        draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        // Second flash, then hold the reveal for 120 frames.
        r.index = 5;
        flash_palette_buffer(engine, r);
        engine.state.frame_counter = 120;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        // Fade out the title room and set up the chosen character for gameplay
        // at full health/magic.
        fade_room_palette_out_reset_audio(engine, r);
        engine.state.player_pose = 8; // gameplay idle pose
        engine.state.player_facing = 0;
        engine.state.player_health = 99;
        engine.state.player_magic = 99;
        sync_health_hud(engine, r);
        sync_magic_hud(engine, r);
        engine.state.selected_item_slot = 2;
        draw_status_item_sprites(engine, r);
        r.value = 8; // loadout page for the initial item-pick
        enter_temporary_room_page(engine, r);
        // Show the loadout page so the player picks their starting carried
        // items, then enter the real game room.
        draw_carried_item_sprites(engine, r);
        upload_inventory_count_tiles(engine, r);
        upload_equipped_item_stat_tiles(engine, r);
        engine.state.scroll_fine_x = 8;
        refresh_scroll_register_shadows(engine, r);
        // Pose was already computed by enter_temporary_room_page ($E6AD JSR
        // $D8E3); the loadout page just redraws the player here.
        draw_player_sprites(engine, r);
        fade_room_palette_in(engine, r);
        run_carried_item_loadout_flow(engine, r);
        restore_room_from_checkpoint(engine, r);
        return;
    }
}

/// Runs the overhead-tile shop room. The caller enters through room tile `0x04`;
/// this flow preserves the current room, stages the shop room, sells the two
/// visible shop items, and restores gameplay when the player reaches the exit.
pub fn run_shop_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    push_room_checkpoint(engine, r);

    // Stage the shop room page (selected by map_screen_x). The two shop item ids
    // live in temp_save[0]/[2]; preserve all four temp_save bytes across the
    // page swap (enter_temporary_room_page clobbers them).
    let s80 = engine.state.temp_save(0);
    let s81 = engine.state.temp_save(1);
    let s82 = engine.state.temp_save(2);
    let s83 = engine.state.temp_save(3);
    r.value = (engine.state.map_screen_x as u8);
    enter_temporary_room_page(engine, r);
    engine.state.set_temp_save(3, s83);
    engine.state.set_temp_save(2, s82);
    engine.state.set_temp_save(1, s81);
    engine.state.set_temp_save(0, s80);

    // Draw the shop contents and prices, then fade in.
    draw_shop_item_sprites(engine, r);
    upload_shop_price_tiles(engine, r);
    draw_coin_cost_sprites(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        // Walk until the player acts on a counter tile or reaches the exit.
        walk_purchase_room_until_action_or_exit(engine, r);
        if ((r.carry) != 0) {
            restore_room_from_checkpoint(engine, r);
            return; // left the shop
        }

        // Map the player's tile column to which item is in front of them:
        // columns 3..4 -> item slot 0, columns 10..11 -> item slot 2.
        let nib = engine.state.player_x_tile & ((crate::bits::LOW_NIBBLE) as u8);
        let x = if nib < 3 {
            continue; // not standing at a counter
        } else if nib < 5 {
            0 // left item
        } else {
            if nib < 10 || nib >= 12 {
                continue; // gap between counters
            }
            2 // right item
        };

        let item = engine.state.temp_save(x); // item id (high bit = already bought)
        if (item & crate::bits::BIT7) != 0 {
            engine.state.prompt_state = 6; // sold out: "blocked" prompt
        } else {
            // Price lives at inventory pseudo-slot 33+x.
            let price = engine.state.inventory_item(33 + x);
            r.value = (price as u8);
            spend_coins(engine, r);
            if ((r.carry) != 0) {
                // Purchase succeeded: mark the item sold, redraw, add to inventory.
                engine.state.set_temp_save(x, 255);
                draw_shop_item_sprites(engine, r);
                engine.state.set_inventory_item(
                    item,
                    (engine.state.inventory_item(item) + 1) & crate::bits::BYTE_MASK,
                );
                engine.state.prompt_state = 16; // purchase jingle
            } else {
                // Not enough coins. Buying item 13 with a save active flags the
                // hidden shop (continue/cheat) behavior.
                if item == 13 && engine.state.continue_timer != 0 {
                    engine.state.shop_active = 1;
                }
                engine.state.prompt_state = 6; // "blocked" prompt
            }
        }

        // Debounce the action button before the next purchase.
        loop {
            if frame::read_buttons(engine, r) == 0 {
                break;
            }
            frame::wait_frame(engine, r);
        }
    }
}

/// Walks the player around the character-select room one frame at a time until
/// the A button (bit7) is pressed. The player's projected position is committed
/// only while it stays inside the selectable area, so the player can never walk
/// off the menu; on return the committed tile (`indirect_ptr_hi`/`scratch2`)
/// identifies the option chosen, and `r.value` (A) is 128.
pub fn walk_character_select_room_until_action(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.frame_counter = 1;
        let buttons = frame::read_buttons(engine, r);
        // A pressed: stop and report the action.
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            return;
        }

        // Build a movement delta from the D-pad (low nibble) and project the
        // would-be new player position.
        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        build_input_movement_delta(engine, r);
        project_player_position(engine, r);

        // Commit the move only if the target Y is in the menu band (48..161)
        // and the target tile column is selectable (>= 2, and either < 13 or
        // exactly tile-aligned at the right edge).
        let ty = engine.state.scratch2;
        if ty >= 48 && ty < 161 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo >= 2 {
                let store = lo < 13 || engine.state.indirect_ptr_lo == 0;
                if store {
                    engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                    engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                    engine.state.player_y = engine.state.scratch2;
                }
            }
        }

        draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special-exit actor animation. After the rising/falling sequence
/// collides with the playfield, the actor clears itself and raises `0xEB` so
/// the foreground loop enters the pending special-exit room.
pub fn tick_special_exit_actor_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    // First tick (low 7 bits of state still 0): play the triple-flash intro and
    // initialize the rise/fall motion.
    if (engine.state.obj_state & ((crate::bits::LOW_7_BITS) as u8)) == 0 {
        engine.state.prompt_state = 24; // reveal prompt
        engine.state.prompt_argument = 255;
        r.index = 3; // 3 flashes
        flash_palette_buffer(engine, r);

        engine.state.frame_counter = 2;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 3;
        flash_palette_buffer(engine, r);

        engine.state.frame_counter = 5;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 3;
        flash_palette_buffer(engine, r);

        engine.state.obj_state = (engine.state.obj_state + 1) & ((crate::bits::BYTE_MASK) as u8); // mark initialized
        engine.state.prompt_state = 2;
        engine.state.obj_cooldown = 15; // rising phase duration
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_move_scratch = 0; // 0 = rising phase
        engine.state.obj_y_extra = engine.state.obj_y_pixel; // remember start Y
    }

    // Rising phase (move_scratch == 0): the actor decelerates upward.
    if engine.state.obj_move_scratch == 0 {
        engine.state.obj_cooldown =
            (engine.state.obj_cooldown - 1) & ((crate::bits::BYTE_MASK) as u8);
        // Cooldown expired: switch to the falling phase.
        if engine.state.obj_cooldown == 0 {
            engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
            engine.state.obj_move_scratch = 1; // 1 = falling phase
            return;
        }
        // Upward velocity = -((cooldown/4)+1) (two's-complement via XOR 0xFF +1).
        let a = ((((engine.state.obj_cooldown >> 2) ^ ((crate::bits::BYTE_MASK) as u8)) + 1) as u8
            as i32);
        engine.state.obj_y_vel = (a as u8);
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        // Hit a wall while rising: start falling.
        if ((r.carry) != 0) {
            engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
            engine.state.obj_move_scratch = 1;
            return;
        }
        engine.state.obj_y_pixel = engine.state.scratch2;
        return;
    }

    // Falling phase: accelerate downward (velocity = scratch/4 + 1).
    engine.state.obj_move_scratch =
        (engine.state.obj_move_scratch + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 2) + 1;
    project_actor_position(engine, r);
    check_position_out_of_bounds(engine, r);
    // Landed back in the playfield: clear the actor and request the special exit.
    if ((r.carry) != 0) {
        engine.state.obj_state = 0;
        engine.state.obj_timer = 240; // exit transition delay
        engine.state.pending_special_exit = 1; // 0xEB: foreground loop enters exit room
        return;
    }
    engine.state.obj_y_pixel = engine.state.scratch2;
}

/// Walks a purchase/refill room until the player presses action or reaches the
/// exit tile. Carry set means exit; carry clear means action on the current
/// selectable tile.
pub fn walk_purchase_room_until_action_or_exit(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.frame_counter = 1;
        let buttons = frame::read_buttons(engine, r);
        // A pressed: action on the current tile (carry clear).
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            r.carry = 0;
            return;
        }

        // Project the player's movement from the D-pad.
        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        build_input_movement_delta(engine, r);
        project_player_position(engine, r);

        // Reaching the bottom exit row (Y >= 161) leaves the room (carry set).
        let ty = engine.state.scratch2;
        if ty >= 161 {
            r.value = (ty as u8);
            r.carry = 1;
            return;
        }
        // Commit moves only in the counter band (Y >= 140) on selectable tile
        // columns (2..12).
        if ty >= 140 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo >= 2 && lo < 13 {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                engine.state.player_y = engine.state.scratch2;
            }
        }

        // Animate the walking player and advance the frame.
        update_player_pose_from_motion(engine, r);
        tick_player_walk_animation(engine, r);
        draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Walks the carried-item loadout room until action or exit. This variant keeps
/// the tile cursor over a wider range than the purchase-room movement loop so
/// empty carried-item slots can be selected too.
pub fn walk_loadout_room_until_action_or_exit(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.frame_counter = 1;
        let buttons = frame::read_buttons(engine, r);
        // A pressed: action on the current tile (carry clear).
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            r.carry = 0;
            return;
        }

        // Project the player's movement from the D-pad.
        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        build_input_movement_delta(engine, r);
        project_player_position(engine, r);

        // Bottom exit row (Y >= 161): leave the room (carry set).
        let ty = engine.state.scratch2;
        if ty >= 161 {
            r.value = (ty as u8);
            r.carry = 1;
            return;
        }
        // Commit moves over a wide band (Y >= 32) and a wider column range than
        // the purchase room (1..15, plus the tile-aligned right edge), so empty
        // carried-item slots can also be reached.
        if ty >= 32 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            let mut store = false;
            if lo >= 1 {
                if lo < 15 {
                    store = true;
                } else if engine.state.indirect_ptr_lo == 0 {
                    store = true; // right edge, only when tile-aligned
                }
            }
            if store {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                engine.state.player_y = engine.state.scratch2;
            }
        }

        // Animate the walking player and advance the frame.
        update_player_pose_from_motion(engine, r);
        tick_player_walk_animation(engine, r);
        draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Lets the player refill the active family member's carried-item queue. A
/// selected inventory item is consumed, the previous front carried item is
/// returned to inventory, and the queue shifts left before the new item is
/// appended.
pub fn run_carried_item_loadout_flow(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        // Walk until the player exits or selects a tile.
        walk_loadout_room_until_action_or_exit(engine, r);
        if ((r.carry) != 0) {
            // On exit, if the equipped slot points at the empty marker (13),
            // reset the selection to the default slot 3.
            let e = engine.state.selected_item_slot;
            if engine.state.item_slot(e as i32) == 13 {
                engine.state.selected_item_slot = 3;
                draw_status_item_sprites(engine, r);
            }
            return;
        }
        let mut x = 255;
        let py = engine.state.player_y;
        // Decide whether the chosen tile yields an item to enqueue. Standing at
        // or below Y 88 means the "remove front item" tile (push the empty
        // marker, x stays 255); above that, the row selects an inventory page
        // (0 for the top row, 8 for the middle) and the tile column picks the
        // item index within it.
        let flow_0441 = if py >= 88 {
            true
        } else {
            x = if py < 56 { 0 } else { 8 }; // page base by row
            engine.state.scratch0 = x;
            x = (((engine.state.player_x_tile >> 1) | engine.state.scratch0) as u8); // item index
            if engine.state.inventory_item(x as i32) != 0 {
                // Item available: verify the active family member may carry it.
                r.value = (x as u8);
                load_family_item_permission_bits(engine, r);
                if ((r.carry) != 0) {
                    // Permitted: spend one from inventory.
                    engine.state.set_inventory_item(
                        (x as i32),
                        (engine.state.inventory_item(x as i32) - 1) & crate::bits::BYTE_MASK,
                    );
                    true
                } else {
                    false // not allowed for this character
                }
            } else {
                false // none in inventory
            }
        };
        // Selection invalid: play the "blocked" prompt and retry.
        if !flow_0441 {
            engine.state.prompt_state = 6;
            continue;
        }
        engine.state.scratch0 = x; // new item id (or 255 = empty)
        // Return the current front carried item (slot 0) to inventory if valid.
        let ci0 = engine.state.item_slot(0);
        if (ci0 & crate::bits::BIT7) == 0 {
            engine.state.set_inventory_item(
                ci0,
                (engine.state.inventory_item(ci0) + 1) & crate::bits::BYTE_MASK,
            );
        }
        // Shift the carried-item queue left and append the new item at the back.
        engine.state.set_item_slot(0, engine.state.item_slot(1));
        engine.state.set_item_slot(1, engine.state.item_slot(2));
        engine
            .state
            .set_item_slot(2, (engine.state.scratch0 as i32));
        engine.state.prompt_state = 18; // equip jingle
        draw_carried_item_sprites(engine, r);
        draw_status_item_sprites(engine, r);
        upload_inventory_count_tiles(engine, r);
        upload_equipped_item_stat_tiles(engine, r);
    }
}

/// Saves the current gameplay room state before entering a temporary room such
/// as a shop or character-select room. The current song is mirrored in `0xFE`
/// so the restore path can restart it after rebuilding the room.
pub fn push_room_checkpoint(engine: &mut Engine, _r: &mut RoutineContext) {
    // Mirror the current song so the restore path can restart it (0xFE).
    engine.state.room_restore_scratch = engine.state.song;
    // Push position/scroll/room-id onto the checkpoint stack if there's room.
    if engine.room_ckpt_sp < engine.room_ckpt_stack.len() {
        let c = [
            engine.state.player_x_fine as u8,
            engine.state.player_x_tile as u8,
            engine.state.player_y as u8,
            engine.state.scroll_fine_x as u8,
            engine.state.scroll_tile_x as u8,
            engine.state.map_screen_x as u8,
            engine.state.map_screen_y as u8,
        ];
        engine.room_ckpt_stack[engine.room_ckpt_sp] = c;
        engine.room_ckpt_sp += 1;
    }
}

/// Restores the most recently saved gameplay room position, scroll, and room
/// identity fields.
pub fn pop_room_checkpoint(engine: &mut Engine, _r: &mut RoutineContext) {
    // Pop the most recent checkpoint and restore the saved fields (in the same
    // order they were pushed by push_room_checkpoint).
    if engine.room_ckpt_sp > 0 {
        engine.room_ckpt_sp -= 1;
        let c = engine.room_ckpt_stack[engine.room_ckpt_sp];
        engine.state.player_x_fine = (c[0] as u8);
        engine.state.player_x_tile = (c[1] as u8);
        engine.state.player_y = (c[2] as u8);
        engine.state.scroll_fine_x = (c[3] as u8);
        engine.state.scroll_tile_x = (c[4] as u8);
        engine.state.map_screen_x = (c[5] as u8);
        engine.state.map_screen_y = (c[6] as u8);
    }
}

/// Runs the high-bit defeated-actor reward drop sequence. The actor rises,
/// falls back into the playfield, then turns into a pickup chosen from current
/// resource needs and the drop table.
pub fn tick_defeated_actor_reward_drop(engine: &mut Engine, r: &mut RoutineContext) {
    // Random drop table indexed by rng result 0..8: mostly health (3), some
    // magic (4), then keys (5), coins (6), and the rare item (7).
    const DROP_ITEM_TABLE: [i32; 9] = [3, 3, 3, 3, 4, 4, 5, 6, 7];
    // First tick: initialize the bounce animation and load the actor's sprite.
    if (engine.state.obj_state & ((crate::bits::LOW_7_BITS) as u8)) == 0 {
        engine.state.obj_state = (engine.state.obj_state + 1) & ((crate::bits::BYTE_MASK) as u8); // mark initialized
        engine.state.prompt_state = 14; // defeat prompt
        engine.state.obj_cooldown = 8; // rising phase duration
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_move_scratch = 0; // 0 = rising phase
        engine.state.obj_y_extra = engine.state.obj_y_pixel; // remember start Y
        // Read the death sprite tile from the actor record (offset 6).
        let ptr = (((engine.state.actor_record_ptr_lo as i32)
            | ((engine.state.actor_record_ptr_hi as i32) << 8)) as u16 as i32);
        engine.state.obj_tile = ((engine.state.byte((ptr + 6) as u16 as i32)) as u8);
        engine.state.obj_attr = engine.state.obj_attr & ((crate::bits::LOW_2_BITS) as u8); // keep only palette bits
    }
    // Rising phase: the corpse pops upward, decelerating.
    if engine.state.obj_move_scratch == 0 {
        engine.state.obj_cooldown =
            (engine.state.obj_cooldown - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.obj_cooldown != 0 {
            engine.state.obj_y_vel = 0 - engine.state.obj_cooldown; // upward (negative) velocity
            project_actor_position(engine, r);
            check_position_out_of_bounds(engine, r);
            if ((r.carry) == 0) {
                engine.state.obj_y_pixel = engine.state.scratch2;
                return; // still rising in-bounds
            }
        }
        // Cooldown done or hit something: switch to falling.
        engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
        engine.state.obj_move_scratch = 1; // 1 = falling phase
        return;
    }
    // Falling phase: accelerate downward (velocity = scratch/2 + 2).
    engine.state.obj_move_scratch =
        (engine.state.obj_move_scratch + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 1) + 2;
    project_actor_position(engine, r);
    check_position_out_of_bounds(engine, r);
    if ((r.carry) == 0) {
        engine.state.obj_y_pixel = engine.state.scratch2;
        return; // still falling in-bounds
    }

    // Landed: choose which pickup to spawn (x = item id passed to spawn setup).
    // Priority: fix the most urgent resource shortage first.
    let mut x = 0; // 0 = health pickup
    if engine.state.player_health < 20 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 1; // 1 = magic pickup
    if engine.state.player_magic < 30 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 4; // 4 = key pickup
    if engine.state.keys < 2 {
        item_spawn_setup(engine, r, x);
        return;
    }
    // No shortage: roll a random number in 0..19.
    r.value = 20;
    rng_update(engine, r);
    if r.value >= 9 {
        // High roll: top up whichever of health/magic is lowest, falling back to
        // coins (item 2) when both already exceed the coin count.
        x = 0; // health
        if engine.state.player_health < engine.state.player_magic {
            if (engine.state.player_health as i32) < (engine.state.coins as i32) {
                item_spawn_setup(engine, r, x);
                return;
            }
            x = 2; // coin pickup
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 1; // magic
        if (engine.state.player_magic as i32) < (engine.state.coins as i32) {
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 2; // coin pickup
        item_spawn_setup(engine, r, x);
        return;
    }
    // Low roll (0..8): use the weighted random drop table.
    x = DROP_ITEM_TABLE[r.value as usize];
    item_spawn_setup(engine, r, x);
}
