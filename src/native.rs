use crate::game::{
    CHARACTER_STATS_TABLE, SPAWN_OFFSET_X_TABLE, SPAWN_OFFSET_Y_TABLE, START_ITEM_TABLE,
};
use crate::{Engine, RoutineContext, engine::RoutineFn, frame};

fn enter_return_home(engine: &mut Engine, lo: i32, hi: i32) {
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    engine.state.prg_bank_8000 = engine.state.saved_prg_bank_8000;
    engine.state.prg_bank_a000 = engine.state.saved_prg_bank_a000;
    engine.state.mmc3_bank_select = 6;
    engine.prg_map_shadow();
}

fn leave_return_home(engine: &mut Engine) {
    engine.state.prg_bank_8000 = 12;
    engine.state.prg_bank_a000 = 13;
    engine.state.mmc3_bank_select = 7;
    engine.prg_map_shadow();
}

fn farcall_cce4(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    enter_return_home(engine, lo, hi);
    target(engine, r);
    leave_return_home(engine);
}

fn farcall_0c0d(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    let old6 = engine.state.prg_bank_8000;
    let old7 = engine.state.prg_bank_a000;
    engine.state.saved_prg_bank_8000 = old6;
    engine.state.saved_prg_bank_a000 = old7;
    engine.state.indirect_ptr_lo = (lo as u8);
    engine.state.indirect_ptr_hi = (hi as u8);
    engine.state.prg_bank_8000 = 12;
    engine.state.prg_bank_a000 = 13;
    engine.state.mmc3_bank_select = 7;
    engine.prg_map_shadow();
    target(engine, r);
    engine.state.prg_bank_a000 = old7;
    engine.state.prg_bank_8000 = old6;
    engine.state.mmc3_bank_select = 6;
    engine.prg_map_shadow();
}

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
    r.value = 5;
    queue_ppu_job_and_wait(engine, r);
}

fn item_spawn_setup(engine: &mut Engine, r: &mut RoutineContext, x: i32) {
    engine.state.obj_state = ((x + 2) as u8);
    engine.state.obj_tile = (((x << 2) | crate::bits::BITS_0_7) as u8);
    engine.state.obj_attr = 1;
    engine.state.obj_y_pixel = engine.state.obj_y_extra;
    engine.state.obj_timer = 240;
    engine.state.obj_move_scratch = 0;
    engine.state.obj_cooldown = 0;
    crate::game::update_object_terrain_probe(engine, r);
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
    engine.state.prompt_state = 3;
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);
    loop {
        let buttons = frame::read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        let buttons = frame::read_buttons(engine, r);
        if (buttons & crate::bits::BIT4) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        let buttons = frame::read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    engine.state.prompt_state = 4;
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);
}

pub fn main_loop_dispatch(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        if frame::frame_runner_stop_requested() {
            return;
        }
        if engine.state.player_health == 0 {
            engine.state.sprite_blink_timer = 0;
            crate::game::draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 7, 179, run_player_death_or_continue_flow);
            if r.index == 0 {
                continue;
            }
            r.index = ((r.index - 1) as u8);
            crate::game::main_init(engine, r);
            return;
        }

        engine.state.frame_counter = 1;
        engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
        frame::read_buttons(engine, r);
        crate::game::game_update(engine, r);

        if engine.state.final_exit_flag != 0 {
            // The final-exit item diverts the normal room loop into a scripted
            // sequence that still reuses the player/object update helpers.
            farcall_0c0d(engine, r, 235, 162, setup_final_exit_sequence);
            loop {
                frame::read_buttons(engine, r);
                farcall_0c0d(
                    engine,
                    r,
                    188,
                    171,
                    crate::game::tick_scripted_player_motion,
                );
                farcall_0c0d(
                    engine,
                    r,
                    230,
                    165,
                    crate::game::update_final_exit_projectiles,
                );
                farcall_0c0d(
                    engine,
                    r,
                    93,
                    167,
                    crate::game::rotate_sprite_zero_from_scripted_oam,
                );
                farcall_0c0d(engine, r, 227, 163, tick_final_exit_sequence);
                if engine.state.player_health != 0 {
                    break;
                }
            }

            engine.state.player_x_tile = engine.state.player_x_fine >> 4;
            engine.state.player_x_fine =
                engine.state.player_x_fine & ((crate::bits::LOW_NIBBLE) as u8);
            engine.state.set_oam_y(0, 239);
            engine.state.sprite_blink_timer = 0;
            crate::game::draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 7, 179, run_player_death_or_continue_flow);
            r.index = ((r.index - 1) as u8);
            crate::game::main_init(engine, r);
            return;
        }

        crate::game::update_player_projectiles(engine, r);
        crate::game::update_room_actors(engine, r);
        crate::game::update_tile_projectile(engine, r);
        crate::game::update_camera_scroll_from_player(engine, r);
        let saved_c = r.carry;
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        r.carry = saved_c;
        if ((r.carry) == 0) && engine.state.saved_scroll_tile != engine.state.scroll_tile_x {
            engine.state.main_loop_phase =
                (engine.state.main_loop_phase + 1) & ((crate::bits::BYTE_MASK) as u8);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Sets up the final-exit sequence after the final item trigger: flash the
/// current scene, switch to the scripted room, and seed the special object/player
/// state used by `tick_final_exit_sequence`.
pub fn setup_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 24;
    engine.state.sprite_blink_timer = 0;
    crate::game::draw_player_sprites(engine, r);

    r.index = 2;
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    r.index = 3;
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    fade_partial_palette_buffer_out(engine, r);

    engine.state.prompt_state = 32;
    engine.state.frame_counter = 60;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.state.map_screen_y = 19;
    engine.state.map_screen_x = 2;
    farcall_cce4(engine, r, 242, 200, crate::game::scene_assemble);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);

    engine.state.set_oam_y(0, 239);
    engine.state.scroll_y = 34;
    engine.state.scroll_fine_x = 0;
    engine.state.player_x_fine = 0;
    engine.state.scroll_tile_x = 16;
    farcall_cce4(engine, r, 203, 197, crate::game::upload_current_room_view);
    r.index = 4;
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    engine.state.scroll_tile_x = 0;
    farcall_cce4(
        engine,
        r,
        108,
        199,
        crate::game::upload_room_columns_from_bank9,
    );
    engine.state.set_chr_bank(3, 61);

    loop {
        let mut x = engine.state.scroll_y;
        if x == 0 {
            x = 240;
        }
        if x == 194 {
            break;
        }
        x = ((x - 1) as u8);
        engine.state.scroll_y = x;
        engine.state.nametable_select = (x & ((crate::bits::BIT3) as u8)) >> 3;
        r.value = 255;
        queue_ppu_job_and_wait(engine, r);
    }

    r.index = 2;
    farcall_cce4(engine, r, 64, 197, flash_palette_buffer);
    farcall_cce4(
        engine,
        r,
        199,
        193,
        crate::game::refresh_scroll_register_shadows,
    );

    engine.state.set_object_x_sub(0, 0);
    engine.state.set_object_x_tile(0, 0);
    engine.state.set_object_timer(0, 0);
    engine.state.scheduler_phase = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;
    engine.state.set_object_health(0, 100);
    engine.state.sprite_index = 8;
    engine.state.player_x_fine =
        ((((engine.state.player_x_tile as i32) << 4) | (engine.state.player_x_fine as i32)) as u8);
    crate::game::draw_scripted_player_sprites(engine, r);
    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    crate::game::load_final_exit_object_oam_template(engine, r);
    crate::game::load_final_exit_player_oam_template(engine, r);
}

/// Runs the one-shot final-exit cutscene path that is entered before the special
/// object latch at `0xF2` is set.
fn run_final_exit_cutscene(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::build_object_health_meter_standard_tiles(engine, r);
    engine.state.set_object_state(16, 0);
    engine.state.set_object_state(32, 0);
    engine.state.set_object_state(48, 0);
    engine.state.obj_health = 0;
    engine.state.sprite_blink_timer = 0;
    engine.state.displaced_timer = 0;
    crate::game::draw_scripted_player_sprites(engine, r);
    crate::game::draw_final_exit_projectile_sprites(engine, r);
    engine.state.set_oam_y(0, 239);

    while engine.state.player_y < 160 {
        engine.state.player_y = (engine.state.player_y + 1) & ((crate::bits::BYTE_MASK) as u8);
        crate::game::draw_scripted_player_sprites(engine, r);
        engine.state.frame_counter = 1;
        frame::wait_for_frame_counter(engine, r);
    }

    engine.state.fall_frames = 0;
    engine.state.jump_timer = 0;
    crate::game::update_scripted_player_pose_from_motion(engine, r);
    crate::game::draw_scripted_player_sprites(engine, r);
    engine.state.scroll_tile_x = 32;
    engine.state.nametable_select = 1;
    engine.state.prompt_state = 32;
    engine.state.prompt_argument = 128;
    engine.state.tile_table_ptr_hi = 182;

    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }

    engine.state.prompt_state = 32;
    engine.state.prompt_argument = 128;
    engine.state.tile_table_ptr_hi = 183;
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile == 0 {
            break;
        }
    }

    engine.state.tile_fetch_counter = 0;
    loop {
        if (engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0 {
            engine.state.nametable_select =
                engine.state.nametable_select ^ ((crate::bits::BIT0) as u8);
            engine.state.prompt_state = 32;
            engine.state.prompt_argument = 128;
        }

        r.value = 255;
        queue_ppu_job_and_wait(engine, r);
        if engine.state.sprite0_hit() {
            r.value = 5;
            crate::game::subtract_scripted_player_health(engine, r);
            crate::game::build_player_health_meter_sprites(engine, r);
        }

        if engine.state.sprite_index == 0 {
            engine.state.sprite_index = 2;
        }

        crate::game::draw_scripted_player_sprites(engine, r);
        crate::game::rotate_sprite_zero_from_scripted_oam(engine, r);
        engine.state.tile_fetch_counter =
            (engine.state.tile_fetch_counter - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.tile_fetch_counter == 0 {
            break;
        }
    }

    engine.state.nametable_select = 1;
    r.value = 255;
    queue_ppu_job_and_wait(engine, r);
    if engine.state.player_health == 0 {
        return;
    }

    engine.state.set_oam_y(0, 239);
    engine.state.prompt_state = 24;
    engine.state.prompt_argument = 255;
    engine.state.scratch0 = 1;
    loop {
        let prev = engine.state.player_y;
        let ny = ((prev - engine.state.scratch0) as u8 as i32);
        engine.state.player_y = (ny as u8);
        let c = if prev >= engine.state.scratch0 { 1 } else { 0 };
        let t = ((ny + 43 + c) as u8 as i32);
        if t >= 239 {
            break;
        }
        crate::game::draw_scripted_player_sprites(engine, r);
        engine.state.scratch0 = (engine.state.scratch0 + 1) & ((crate::bits::BYTE_MASK) as u8);
        r.value = 255;
        queue_ppu_job_and_wait(engine, r);
    }

    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    engine.state.sprite_index = 0;
    engine.state.oam_cursor = 128;
    crate::game::reset_room_object_slots(engine, r);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    engine.state.map_screen_y = 16;
    engine.state.map_screen_x = 3;
    farcall_cce4(engine, r, 242, 200, crate::game::scene_assemble);
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
    engine.state.set_object_state(16, 1);
    engine.state.set_object_state(32, 1);
    engine.state.set_object_state(48, 1);
    engine.state.set_object_state(64, 1);
    engine.state.set_object_y_pixel(16, 160);
    engine.state.set_object_y_pixel(32, 160);
    engine.state.set_object_y_pixel(48, 160);
    engine.state.set_object_y_pixel(64, 112);
    engine.state.set_object_x_tile(64, 51);
    crate::game::sync_final_exit_body_slots_from_player(engine, r);
    let mut v = 45;
    engine.state.set_object_tile(16, v);
    v = ((v + 32) as u8 as i32);
    engine.state.set_object_tile(32, v);
    v = ((v + 32) as u8 as i32);
    engine.state.set_object_tile(48, v);
    engine.state.set_object_tile(64, 129);
    engine.state.set_object_attr(16, 64);
    engine.state.set_object_attr(32, 64);
    engine.state.set_object_attr(48, 64);
    engine.state.set_object_attr(64, 64);
    crate::game::upload_status_panel_template(engine, r);
    farcall_cce4(engine, r, 203, 197, crate::game::upload_current_room_view);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);
    crate::game::clear_gameplay_object_sprites(engine, r);
    crate::game::draw_player_sprites(engine, r);
    crate::game::draw_status_item_sprites(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.state.character_index = 7;
    farcall_cce4(engine, r, 146, 196, fade_room_palette_in);
    engine.state.countdown_timer = 5;
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

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
        engine.state.player_facing = engine.state.player_facing ^ ((crate::bits::BIT6) as u8);
        crate::game::draw_player_sprites(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.state.player_pose = 13;
    crate::game::draw_player_sprites(engine, r);
    engine.state.countdown_timer = 3;
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

    loop {
        engine.state.frame_counter = 1;
        engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
        engine.state.buttons = 1;
        farcall_cce4(engine, r, 43, 212, crate::game::game_update);
        farcall_cce4(
            engine,
            r,
            93,
            193,
            crate::game::update_camera_scroll_from_player,
        );
        crate::game::sync_final_exit_body_slots_from_player(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
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

    engine.state.player_pose = 25;
    engine.state.set_object_tile(16, 57);
    engine.state.set_object_tile(32, 89);
    engine.state.set_object_tile(48, 121);
    engine.state.set_object_tile(64, 145);
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
    engine.state.obj_slot_ptr_lo = 0;
    engine.state.obj_slot_ptr_hi = 4;
    crate::game::load_object_slot_scratch(engine, r);
    if engine.state.obj_health == 0 {
        run_final_exit_cutscene(engine, r);
        return;
    }

    if engine.state.sprite0_hit() {
        let t = (engine.state.sprite_index + 2) & ((crate::bits::BITS_1_2) as u8);
        if t != 0 {
            let x = ((t << 3) as u8 as i32);
            if engine.state.object_state(x) != 0 {
                engine.state.set_object_state(x, 0);
                let sum = ((engine.state.scroll_pixel_x + ((engine.state.object_x_sub(x)) as u8))
                    as u8 as i32);
                if sum >= 176 && sum < 208 {
                    let bl = engine.state.obj_health;
                    engine.state.obj_health = if bl < 2 { 0 } else { ((bl - 2) as u8) };
                    crate::game::build_object_health_meter_standard_tiles(engine, r);
                    engine.state.prompt_state = 32;
                    engine.state.prompt_argument = 1;
                } else {
                    engine.state.prompt_state = 1;
                }
            }
        }
    }

    if engine.state.obj_x_tile == 0 {
        match engine.state.obj_timer {
            4 => {
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase != 0 {
                    if engine.state.scheduler_phase == 4 {
                        engine.state.prompt_state = 32;
                    }
                    engine.state.tile_table_ptr_hi = 181;
                    engine.state.scroll_y = 194;
                } else {
                    engine.state.tile_table_ptr_hi = 179;
                    engine.state.obj_timer = 0;
                }
            }
            3 => {
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
                                engine.state.scroll_y =
                                    (engine.state.scroll_y + 4) & ((crate::bits::BYTE_MASK) as u8);
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
                    engine.state.tile_table_ptr_hi = 176;
                    engine.state.obj_timer =
                        (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
                    engine.state.scheduler_phase = 4;
                }
            }
            2 => {
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase != 0 {
                    engine.state.tile_table_ptr_hi = 180;
                    if engine.state.scroll_y >= 195 {
                        engine.state.scroll_y =
                            (engine.state.scroll_y - 4) & ((crate::bits::BYTE_MASK) as u8);
                    }
                } else {
                    engine.state.tile_table_ptr_hi = 179;
                    engine.state.obj_timer = 0;
                }
            }
            1 => {
                engine.state.scheduler_phase =
                    (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                if engine.state.scheduler_phase == 0 {
                    engine.state.obj_timer = 0;
                } else {
                    let a = (((((engine.state.scheduler_phase as i32) << 1)
                        & (((crate::bits::BIT0) as u8) as i32))
                        + 176) as u8 as i32);
                    engine.state.tile_table_ptr_hi = (a as u8);
                    engine.state.scroll_pixel_x =
                        (engine.state.scroll_pixel_x + 4) & ((crate::bits::BYTE_MASK) as u8);
                    if engine.state.scroll_pixel_x >= 64 {
                        engine.state.obj_timer = 0;
                    } else {
                        engine.state.scroll_y = 194;
                    }
                }
            }
            _ => {
                let sum = ((engine.state.scroll_pixel_x + engine.state.player_x_fine) as u8 as i32);
                let carry = sum < (engine.state.scroll_pixel_x as i32);
                let close = carry || sum >= 192 || engine.state.scroll_pixel_x >= 64;
                let delayed_grow = sum < 128 || sum >= 160;
                if close || (delayed_grow && engine.state.scroll_y >= 195) {
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
                        engine.state.tile_table_ptr_hi = 176;
                        engine.state.obj_timer =
                            (engine.state.obj_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
                        engine.state.scheduler_phase = 4;
                    }
                } else if !delayed_grow {
                    engine.state.obj_timer = 2;
                    engine.state.scheduler_phase = 8;
                    engine.state.tile_table_ptr_hi = 179;
                } else {
                    engine.state.obj_timer = 1;
                    engine.state.scheduler_phase = 4;
                    engine.state.scheduler_phase =
                        (engine.state.scheduler_phase - 1) & ((crate::bits::BYTE_MASK) as u8);
                    if engine.state.scheduler_phase == 0 {
                        engine.state.obj_timer = 0;
                    } else {
                        let a = (((((engine.state.scheduler_phase as i32) << 1)
                            & (((crate::bits::BIT0) as u8) as i32))
                            + 176) as u8 as i32);
                        engine.state.tile_table_ptr_hi = (a as u8);
                        engine.state.scroll_pixel_x =
                            (engine.state.scroll_pixel_x + 4) & ((crate::bits::BYTE_MASK) as u8);
                        if engine.state.scroll_pixel_x >= 64 {
                            engine.state.obj_timer = 0;
                        } else {
                            engine.state.scroll_y = 194;
                        }
                    }
                }
            }
        }
    }

    crate::game::advance_scripted_scroll_slice(engine, r);
    crate::game::store_object_slot_scratch(engine, r);
}

/// Fades the first 13 palette-buffer entries toward black over four timed
/// foreground frames.
pub fn fade_partial_palette_buffer_out(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 4;
    loop {
        engine.state.frame_counter = 5;
        for x in (0..=12).rev() {
            let lo = engine.state.palette_buffer(x) & crate::bits::LOW_NIBBLE;
            let hi = engine.state.palette_buffer(x) & crate::bits::HIGH_NIBBLE;
            engine.state.scratch0 = (lo as u8);
            let out = if hi < 16 {
                15
            } else {
                (((hi - 16) | lo) as u8 as i32)
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
    crate::game::draw_player_sprites(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.state.frame_counter = 1;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Clears title/demo screen state so `main_init` can seed the first playable
/// room immediately after returning from the title loop.
pub fn clear_title_screen_for_new_game(engine: &mut Engine, r: &mut RoutineContext) {
    fade_palette_buffer_out(engine, r);
    farcall_cce4(
        engine,
        r,
        139,
        195,
        crate::game::clear_name_tables_to_blank_tiles,
    );
    crate::game::upload_status_panel_template(engine, r);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    crate::game::reset_menu_state_and_palette(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);

    engine.state.frame_counter = 1;
    enter_return_home(engine, 53, 193);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
    leave_return_home(engine);
}

/// Runs the scrolling story-text sequence shared by the title-screen chord and
/// the final-exit cutscene.
pub fn run_story_text_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.music_volume_override =
        (engine.state.music_volume_override + 1) & ((crate::bits::BYTE_MASK) as u8);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::hide_all_sprite_y_positions(engine, r);
    engine.state.set_chr_bank(0, 32);
    engine.state.set_chr_bank(1, 34);
    engine.state.ppu_mask_shadow = engine.state.ppu_mask_shadow | ((crate::bits::BITS_3_4) as u8);

    r.value = 255;
    queue_ppu_job_and_wait(engine, r);

    engine.state.song = 10;
    crate::game::song_init(engine, r);

    engine.state.scroll_pixel_x = 0;
    engine.state.nametable_select = 0;
    engine.state.scratch2 = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;
    crate::game::load_intro_text_palette(engine, r);

    engine.state.vram_addr2_lo = 64;
    engine.state.vram_addr2_hi = 1;
    engine.state.inventory_upload_col = 32;
    engine.state.data_ptr_lo = 156;
    engine.state.data_ptr_hi = 183;

    loop {
        crate::game::advance_intro_text_scroll(engine, r);
        crate::game::stage_intro_text_line(engine, r);
        if ((r.carry) != 0) {
            break;
        }
        crate::game::advance_intro_text_scroll(engine, r);
        crate::game::stage_scrolling_intro_text_line(engine, r);
        if ((r.carry) != 0) {
            break;
        }
    }

    engine.state.prompt_state = 32;
    while engine.state.sfx_voice_active == 0 {
        frame::wait_frame(engine, r);
    }
    while engine.state.sfx_voice_active != 0 {
        frame::wait_frame(engine, r);
    }

    engine.state.frame_counter = 60;
    frame::wait_for_frame_counter(engine, r);

    engine.state.set_sound_channel_byte(1, 0, 0);
    engine.state.sound_channel_flags = 0;
    engine.state.set_sound_channel_byte(1, 32, 0);
    engine.state.set_sound_channel_byte(1, 48, 0);
    engine.state.prompt_state = 24;

    let mut cnt = 10;
    loop {
        for x in (0..=31).rev() {
            engine.state.set_palette_buffer(x, 48);
        }
        crate::game::upload_palette_buffer(engine, r);
        engine.state.frame_counter = 1;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::load_intro_text_palette(engine, r);
        crate::game::upload_palette_buffer(engine, r);
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
        crate::game::reset_menu_state_and_palette(engine, r);
        engine.state.set_chr_bank(2, 55);
        engine.state.statusbar_split_flag = 0;
        engine.state.ppu_ctrl_shadow = 160;
        engine.device_write(crate::engine::reg::PPU_CTRL, 160);
        engine.state.ppu_mask_shadow = 0;
        engine.device_write(crate::engine::reg::PPU_MASK, 0);
        engine.state.scroll_pixel_x = 0;
        engine.state.nametable_select = 0;
        engine.state.scroll_y = 232;
        for x in (0..=31).rev() {
            engine.state.set_palette_buffer(x, 15);
        }
        farcall_cce4(engine, r, 105, 197, crate::game::upload_palette_buffer);
        crate::game::reset_room_object_slots(engine, r);
        crate::game::clear_oam_with_sprite_zero_template(engine, r);
        crate::game::load_title_oam_template(engine, r);
        engine.state.set_chr_bank(2, 21);
        engine.state.song = 9;
        crate::game::song_init(engine, r);
        crate::game::upload_title_screen_nametables(engine, r);
        engine.state.ppu_mask_shadow = 30;
        engine.device_write(crate::engine::reg::PPU_MASK, 30);
        engine.state.frame_counter = 120;
        frame::wait_for_frame_counter(engine, r);
        fade_title_palette_in(engine, r);
        engine.state.countdown_timer = 20;

        loop {
            engine.state.frame_counter = 1;
            let pad = frame::read_buttons(engine, r);
            if pad == 255 {
                engine.state.prompt_state = 26;
                engine.state.continue_timer = 26;
            }
            if (engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }
            if engine.state.button_chord == 131 {
                run_story_text_sequence(engine, r);
                return;
            }
            if (engine.state.frame_prescaler & ((crate::bits::LOW_3_BITS) as u8)) == 0 {
                let lo = engine.state.palette_buffer(2) & crate::bits::LOW_NIBBLE;
                let mut hi = engine.state.palette_buffer(2) & crate::bits::HIGH_NIBBLE;
                engine.state.scratch0 = (lo as u8);
                if hi < 16 {
                    hi = 48;
                } else {
                    hi = ((hi - 16) as u8 as i32);
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
            break;
        }

        fade_palette_buffer_out(engine, r);
        crate::game::clear_oam_with_sprite_zero_template(engine, r);
        crate::game::reset_room_object_slots(engine, r);
        crate::game::load_demo_oam_template(engine, r);
        r.value = 4;
        crate::game::rng_update(engine, r);
        engine.state.map_screen_x = (r.value as u8);
        r.value = 16;
        crate::game::rng_update(engine, r);
        engine.state.map_screen_y = (r.value as u8);
        farcall_cce4(engine, r, 242, 200, crate::game::scene_assemble);

        loop {
            r.value = 64;
            crate::game::rng_update(engine, r);
            engine.state.player_x_tile = (r.value as u8);
            engine.state.data_ptr_lo = (r.value as u8);
            engine.state.player_x_fine = 0;
            r.value = 11;
            crate::game::rng_update(engine, r);
            r.value = (((r.value as i32) << 4) as u8);
            engine.state.player_y = (r.value as u8);
            engine.state.data_ptr_hi = (r.value as u8);
            crate::game::resolve_room_tile_pointer(engine, r);
            let p = ((engine.state.data_ptr()) as u16 as i32);
            let mut t = engine.state.byte(p) & crate::bits::LOW_6_BITS;
            if t >= 48 {
                continue;
            }
            if t == 2 {
                continue;
            }
            if t == (engine.state.text_attr_ptr_lo as i32) {
                continue;
            }
            t = engine.state.byte(((p + 1) as u16 as i32)) & crate::bits::LOW_6_BITS;
            if t < 48 {
                continue;
            }
            if t == 48 {
                continue;
            }
            break;
        }

        let mut x = engine.state.player_x_tile;
        if x < 8 {
            x = 0;
        } else {
            x = ((x - 8) as u8);
        }
        if x >= 48 {
            x = 48;
        }
        engine.state.scroll_tile_x = x;
        engine.state.scroll_fine_x = 0;

        let chr = loop {
            r.value = 5;
            crate::game::rng_update(engine, r);
            let chr = r.value;
            let mut a = 0;
            let mut c = 1;
            for _ in 0..=chr {
                let nc = (a >> 7) & 1;
                a = (((a << 1) | c) as u8 as i32);
                c = nc;
            }
            let mask = a;
            if (mask & (engine.state.family_member_mask as i32)) != 0 {
                break chr;
            }
        };
        engine.state.set_item_slot(
            0,
            engine
                .state
                .byte(((START_ITEM_TABLE + (chr as i32)) as u16 as i32)),
        );
        engine.state.selected_item_slot = 0;
        engine.state.character_index = (chr as u8);
        let mut y = ((CHARACTER_STATS_TABLE + (((chr << 2) + 3) as i32)) as u16 as i32);
        for i in (0..=3).rev() {
            engine.state.set_item_slot(11 + i, engine.state.byte(y));
            y = ((y - 1) as u16 as i32);
        }
        engine
            .state
            .set_chr_bank(2, ((engine.state.character_index + 56) as i32));
        engine.state.set_chr_bank(4, 62);
        engine.state.set_chr_bank(5, 32);
        engine.state.player_pose = 13;
        engine.state.player_facing = 0;
        engine.state.title_timer = 1;
        engine.state.player_health = 100;
        engine.state.player_magic = 100;
        farcall_cce4(
            engine,
            r,
            139,
            195,
            crate::game::clear_name_tables_to_blank_tiles,
        );
        crate::game::upload_status_panel_template(engine, r);
        farcall_cce4(engine, r, 203, 197, crate::game::upload_current_room_view);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        crate::game::sync_coin_hud(engine, r);
        crate::game::sync_key_hud(engine, r);
        crate::game::refresh_scroll_register_shadows(engine, r);
        crate::game::clear_gameplay_object_sprites(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_status_item_sprites(engine, r);
        farcall_cce4(engine, r, 146, 196, fade_room_palette_in);
        engine.state.countdown_timer = 10;

        loop {
            engine.state.frame_counter = 1;
            engine.state.saved_scroll_tile = engine.state.scroll_tile_x;
            crate::game::blink_demo_oam_sprites(engine, r);
            frame::read_buttons(engine, r);
            if (engine.state.buttons & ((crate::bits::BIT4) as u8)) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }

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
                engine.state.title_timer = 128;
                crate::game::choose_random_demo_input(engine, r);
                engine.state.room_restore_scratch = engine.state.buttons;
            }

            farcall_cce4(engine, r, 43, 212, crate::game::game_update);
            farcall_cce4(engine, r, 40, 246, crate::game::update_player_projectiles);
            farcall_cce4(engine, r, 124, 232, crate::game::update_room_actors);
            farcall_cce4(engine, r, 130, 247, crate::game::update_tile_projectile);
            farcall_cce4(
                engine,
                r,
                93,
                193,
                crate::game::update_camera_scroll_from_player,
            );
            crate::game::draw_player_sprites(engine, r);
            crate::game::draw_room_object_sprites(engine, r);
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

        fade_palette_buffer_out(engine, r);
        continue 'restart;
    }
}

/// Waits through a fixed object-render loop while draining active audio/sfx
/// timers used by the story/final-exit transitions.
pub fn drain_audio_timers_with_object_frames(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_sound_channel_byte(1, 32, 0);
    engine.state.data_ptr_hi = 16;
    loop {
        if engine.state.sound_channel_byte(13, 0) != 0 {
            engine.state.dec_sound_channel_byte(13, 0);
        }
        if engine.state.sound_channel_byte(13, 16) != 0 {
            engine.state.dec_sound_channel_byte(13, 16);
        }
        if engine.state.sound_channel_byte(13, 48) != 0 {
            engine.state.dec_sound_channel_byte(13, 48);
        }
        engine.state.data_ptr_lo = 20;
        loop {
            crate::game::draw_room_object_sprites(engine, r);
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

    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);
    crate::game::clear_gameplay_object_sprites(engine, r);
    r.index = 53;
    r.offset = 0;
    show_player_pose_for_eight_frames(engine, r);

    engine.state.frame_counter = 60;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    r.value = 8;
    crate::game::switch_song_if_needed(engine, r);
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);

    engine.state.scratch2 = 5;
    loop {
        r.index = 13;
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 1;
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 9;
        r.offset = 0;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 1;
        r.offset = 64;
        show_player_pose_for_eight_frames(engine, r);
        engine.state.scratch2 = (engine.state.scratch2 - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.scratch2 == 0 {
            break;
        }
    }

    engine.state.frame_counter = 1;
    engine.state.player_pose = 49;
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    let mut use_game_over_screen = engine.state.final_exit_flag != 0;
    if !use_game_over_screen {
        if (engine.state.continue_timer & ((crate::bits::BIT7) as u8)) != 0 {
            let x = engine.state.selected_item_slot;
            if engine.state.item_slot((x as i32)) == 12 {
                engine.state.set_item_slot((x as i32), 255);
                crate::game::draw_status_item_sprites(engine, r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            engine.state.continue_timer =
                (engine.state.continue_timer + 1) & ((crate::bits::BYTE_MASK) as u8);
        }

        if !use_game_over_screen {
            animate_health_refill_to_cap(engine, r);
            engine.state.player_pose = 25;
            crate::game::read_debounced_buttons(engine, r);
            r.value = (saved_song as u8);
            crate::game::switch_song_if_needed(engine, r);
            r.index = 0;
            return;
        }
    }

    fade_palette_buffer_out(engine, r);
    engine.state.final_exit_flag = 0;
    engine.state.sprite_index = 0;
    engine.state.oam_cursor = 128;
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.state.set_chr_bank(1, 22);
    engine.state.set_chr_bank(2, 54);
    engine.state.scroll_pixel_x = 0;
    engine.state.nametable_select = 0;
    engine.state.scroll_y = 0;
    engine.state.scroll_fine_x = 0;
    engine.state.scroll_tile_x = 0;

    vram_blit(engine, r, 107, 33, 175, 180, 9);
    vram_blit(engine, r, 76, 34, 184, 180, 5);
    vram_blit(engine, r, 140, 34, 189, 180, 8);

    engine.state.player_x_tile = 5;
    engine.state.player_x_fine = 0;
    engine.state.player_y = 112;
    engine.state.player_pose = 57;
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    crate::game::draw_player_sprites(engine, r);
    farcall_cce4(engine, r, 224, 196, fade_two_room_palette_rows_in);

    loop {
        crate::game::read_debounced_buttons(engine, r);
        if (r.value & ((crate::bits::BIT4) as u8)) != 0 {
            break;
        }
        engine.state.player_y = engine.state.player_y ^ ((crate::bits::BIT4) as u8);
        engine.state.prompt_state = 12;
    }

    engine.state.prompt_state = 24;
    if engine.state.player_y != 112 {
        fade_palette_buffer_out(engine, r);
        engine.state.frame_counter = 120;
        enter_return_home(engine, 53, 193);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);
        r.index = 2;
        return;
    }

    crate::game::restore_inventory_state_snapshot(engine, r);
    engine.state.set_item_slot(0, 255);
    engine.state.set_item_slot(1, 255);
    engine.state.set_item_slot(2, 255);
    engine.state.selected_item_slot = 3;
    engine.state.character_index = 6;
    engine.state.map_screen_x = 3;
    engine.state.map_screen_y = 16;
    fade_palette_buffer_out(engine, r);
    engine.state.song = 2;
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::upload_status_panel_template(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    farcall_cce4(engine, r, 242, 200, crate::game::scene_assemble);

    r.value = 15;
    for x in (0..=31).rev() {
        engine.state.set_palette_buffer(x, 15);
    }
    engine.state.set_oam_y(16, 239);
    engine.state.set_oam_y(20, 239);
    farcall_cce4(engine, r, 180, 196, fade_room_palette_row_in);
    r.index = 1;
}

/// Shows the player sprite pose in `r.index`/`r.offset` for eight foreground
/// frames.
pub fn show_player_pose_for_eight_frames(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.player_pose = (r.index as u8);
    engine.state.player_facing = (r.offset as u8);
    engine.state.frame_counter = 8;
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Fades the title-screen palette from black to its ROM palette in five steps.
pub fn fade_title_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scratch1 = 64;
    loop {
        engine.state.frame_counter = 5;
        crate::game::load_title_palette_buffer(engine, r);
        r.index = 0;
        r.offset = 32;
        crate::game::dim_palette_range_by_step(engine, r);

        enter_return_home(engine, 53, 193);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);

        engine.state.scratch1 = (engine.state.scratch1 - 16) & ((crate::bits::BYTE_MASK) as u8);
        if (engine.state.scratch1 & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
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
    let mut y = 4;
    loop {
        engine.state.frame_counter = 5;
        for x in (0..=32).rev() {
            let v = engine.state.palette_buffer(x);
            let lo = v & crate::bits::LOW_NIBBLE;
            let hi = v & crate::bits::HIGH_NIBBLE;
            engine.state.scratch0 = (lo as u8);
            engine.state.set_palette_buffer(
                x,
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32)
                } else {
                    15
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
    let mut v = 64;
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5;
        for y in 224..228 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte(((ptr + y) as u16 as i32)));
        }
        r.index = 0;
        r.offset = 4;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8);
        engine.state.scratch1 = v;
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
}

/// Fades in the first two room palette rows from the active room data pointer.
pub fn fade_two_room_palette_rows_in(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = ((engine.state.palette_src_ptr()) as u16 as i32);
    let mut v = 64;
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5;
        for y in 224..228 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte(((ptr + y) as u16 as i32)));
        }
        for y in 240..244 {
            engine
                .state
                .set_inventory_item(64 + y, engine.state.byte(((ptr + y) as u16 as i32)));
        }
        r.index = 0;
        r.offset = 4;
        crate::game::dim_palette_range_by_step(engine, r);
        r.index = 16;
        r.offset = 4;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8);
        engine.state.scratch1 = v;
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
}

/// Advances frames until all controller buttons are released.
pub fn wait_for_buttons_released(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        let buttons = frame::redraw_scene_and_read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        if frame::frame_runner_stop_requested() {
            break;
        }
    }
}

/// Advances frames until any controller button is pressed, then stores that
/// button byte in `r.value` and `0x20`.
pub fn wait_for_button_press(engine: &mut Engine, r: &mut RoutineContext) {
    let buttons = loop {
        let buttons = frame::redraw_scene_and_read_buttons(engine, r);
        if buttons != 0 {
            break buttons;
        }
        if frame::frame_runner_stop_requested() {
            break 0;
        }
    };
    r.value = (buttons as u8);
    engine.state.buttons = (buttons as u8);
}

/// Scans live object slots for a damageable actor overlapping the projected
/// position in `0x0E/0x0F/0x0A`. On hit, `0x08` receives the logical slot and
/// `0x09` receives the object-slot byte offset.
pub fn find_damageable_actor_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 9;
    let mut x = 144;
    loop {
        let mut skip = (y as u8 as i32) == (engine.state.slot_index as i32);
        if !skip && (engine.state.object_state(x) & crate::bits::BIT7) != 0 {
            skip = true;
        }
        if !skip && engine.state.object_state(x) != 1 && engine.state.object_state(x) < 26 {
            skip = true;
        }
        if !skip && (engine.state.object_tile(x) & crate::bits::CLEAR_BITS_1_2) == 225 {
            skip = true;
        }
        if !skip && (engine.state.object_attr(x) & crate::bits::BIT5) != 0 {
            skip = true;
        }
        if !skip {
            let mut d =
                ((engine.state.scratch2 - ((engine.state.object_y_pixel(x)) as u8)) as u8 as i32);
            if !(d < 16) && d < 241 {
                skip = true;
            }
            if !skip {
                d = ((engine.state.indirect_ptr_hi - ((engine.state.object_x_tile(x)) as u8)) as u8
                    as i32);
                if d == 0 {
                    engine.state.scratch0 = (y as u8);
                    engine.state.scratch1 = (x as u8);
                    r.carry = 1;
                    return;
                }
                if d < 2 {
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
                    skip = true;
                } else {
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
        x = ((x - 16) as u8 as i32);
        y -= 1;
        if y < 0 {
            break;
        }
    }
    r.carry = 0;
}

/// Scans live object slots for any nonempty, non-high-bit object overlapping
/// the projected player position in `0x0E/0x0F/0x0A`.
pub fn find_player_object_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y: i32 = 10;
    let mut x = 160;
    loop {
        let mut skip = (y as u8 as i32) == (engine.state.slot_index as i32);
        if !skip && engine.state.object_state(x) == 0 {
            skip = true;
        }
        if !skip && (engine.state.object_state(x) & crate::bits::BIT7) != 0 {
            skip = true;
        }
        if !skip && (engine.state.object_tile(x) & crate::bits::CLEAR_BITS_1_2) == 225 {
            skip = true;
        }
        if !skip && (engine.state.object_attr(x) & crate::bits::BIT5) != 0 {
            skip = true;
        }
        if !skip {
            let mut d =
                ((engine.state.scratch2 - ((engine.state.object_y_pixel(x)) as u8)) as u8 as i32);
            if !(d < 16) && d < 241 {
                skip = true;
            }
            if !skip {
                d = ((engine.state.indirect_ptr_hi - ((engine.state.object_x_tile(x)) as u8)) as u8
                    as i32);
                if d == 0 {
                    engine.state.scratch0 = (y as u8);
                    engine.state.scratch1 = (x as u8);
                    r.carry = 1;
                    return;
                }
                if d < 2 {
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
                    skip = true;
                } else {
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
        x = ((x - 16) as u8 as i32);
        y -= 1;
        if y < 0 {
            break;
        }
    }
    r.carry = 0;
}

/// Tracks the player's floor/contact state after movement has either committed
/// or failed. `0x4E` is the falling/contact frame counter; a nonzero recoil
/// timer (`0x4F`) or scripted lock (`0x86`) suppresses this probe.
pub fn update_player_terrain_contact(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.airborne_flag != 0 || engine.state.jump_timer != 0 {
        engine.state.pose_state = 0;
        engine.state.fall_frames = 0;
        return;
    }

    engine.state.data_ptr_lo = engine.state.player_x_tile;
    engine.state.indirect_ptr_hi = engine.state.player_x_tile;
    engine.state.indirect_ptr_lo = engine.state.player_x_fine;
    engine.state.data_ptr_hi = engine.state.player_y;
    engine.state.scratch2 = engine.state.player_y + 1;
    crate::game::resolve_room_tile_pointer(engine, r);

    if engine.state.player_x_fine == 0 {
        engine.state.pose_state = 1;
        r.offset = 0;
        let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
        if (engine
            .state
            .byte(((tile_ptr + (r.offset as i32)) as u16 as i32))
            & crate::bits::LOW_6_BITS)
            == 0
        {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
    }

    engine.state.pose_state = 0;
    if engine.state.player_y >= 176 {
        engine.state.fall_frames =
            (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
        return;
    }

    find_damageable_actor_overlap(engine, r);
    if ((r.carry) != 0) {
        if engine.state.chr_bank(3) >= 48 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let selected_slot = engine.state.selected_item_slot;
        let selected_item = engine.state.item_slot((selected_slot as i32));
        if selected_item != 5 || engine.state.fall_frames == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let hit_slot = engine.state.scratch1;
        engine.state.set_object_state((hit_slot as i32), 128);
    }

    r.offset = 1;
    crate::game::probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    if engine.state.player_x_fine == 0 {
        engine.state.fall_frames =
            (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
        return;
    }

    r.offset = 13;
    crate::game::probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    engine.state.fall_frames = (engine.state.fall_frames + 1) & ((crate::bits::BYTE_MASK) as u8);
}

/// Converts a just-detected floor/object/hazard contact into damage, recoil,
/// hazard invulnerability, or a reset of the fall counter.
fn resolve_player_landing_or_hazard_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let mut fall_frames = engine.state.fall_frames;
    if fall_frames >= engine.state.jump_strength {
        fall_frames = ((fall_frames - 7) as u8);
        if fall_frames >= engine.state.jump_strength {
            fall_frames = engine.state.jump_strength;
        }
        fall_frames = ((fall_frames - 1) as u8);
        engine.state.jump_timer = fall_frames;
        engine.state.landing_timer = fall_frames + 10;
        engine.state.prompt_state = 10;
        crate::game::consume_health_point(engine, r);
    }
    if engine.state.fall_frames == 0 {
        r.offset = 1;
        crate::game::apply_hazard_tile_contact(engine, r);
        if ((r.carry) == 0) && engine.state.player_x_fine != 0 {
            r.offset = 13;
            crate::game::apply_hazard_tile_contact(engine, r);
        }
    }
    engine.state.fall_frames = 0;
}

/// Handles the room tile sampled at the current projected player footprint.
/// Special tiles can spend keys/magic, spawn transient objects, or launch the
/// tile-removal projectile; ordinary tiles return carry for solid terrain.
pub fn dispatch_room_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = ((engine.state.data_ptr()) as u16 as i32);
    let tile_offset = r.offset;
    let tile = engine
        .state
        .byte(((tile_ptr + (tile_offset as i32)) as u16 as i32))
        & crate::bits::LOW_6_BITS;
    if tile == (engine.state.text_attr_ptr_lo as i32) {
        if engine.state.object_state(144) == 0 {
            engine.state.scratch3 = (tile_offset as u8);
            engine.state.obj_tile = 225;
            engine.state.obj_state = 1;
            engine.state.obj_attr = 1;
            engine.state.obj_move_scratch = engine.state.text_attr_ptr_hi;
            engine.state.obj_timer = 10;
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.state.prompt_state = 6;
        }
        let v = engine.state.text_attr_ptr_hi & ((crate::bits::LOW_6_BITS) as u8);
        r.value = (v as u8);
        r.carry = ((v >= 48) as u8);
        return;
    }
    if tile == 2 {
        if engine.state.object_state(144) == 0 {
            engine.state.scratch3 = (tile_offset as u8);
            r.index = (engine.state.selected_item_slot as u8);
            let item = engine.state.item_slot((r.index as i32));
            r.value = (item as u8);
            if item == 7 {
                r.index = (engine.state.selected_item_slot as u8);
                crate::game::consume_magic_point(engine, r);
                if ((r.carry) != 0) {
                    r.carry = 1;
                    return;
                }
            } else {
                crate::game::consume_key(engine, r);
                if ((r.carry) != 0) {
                    r.carry = 1;
                    return;
                }
            }
            engine.state.obj_tile = 225;
            engine.state.obj_state = 1;
            engine.state.obj_attr = 1;
            engine.state.obj_move_scratch = engine.state.room_tile_action;
            engine.state.obj_timer = 15;
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.state.prompt_state = 6;
        }
        r.carry = 1;
        return;
    }
    if tile == 62 {
        if (engine.state.buttons & ((crate::bits::BIT7) as u8)) != 0
            && engine.state.object_state(144) == 0
        {
            engine.state.scratch3 = (tile_offset as u8);
            engine.state.obj_move_state = 1;
            r.offset = (engine.state.selected_item_slot as u8);
            r.index = ((engine.state.item_slot((r.offset as i32))) as u8);
            let idx = r.index;
            if idx == 1 {
                if engine.state.player_magic != 0 {
                    let mut t = engine.state.player_y & ((crate::bits::LOW_NIBBLE) as u8);
                    t |= engine.state.player_x_fine;
                    if t == 0 {
                        let x2 = (((engine.state.direction_latch
                            & ((crate::bits::LOW_NIBBLE) as u8))
                            << 1) as u8 as i32);
                        let lo = ((engine.state.player_x_tile
                            + ((engine
                                .state
                                .byte(((SPAWN_OFFSET_X_TABLE + x2) as u16 as i32)))
                                as u8)) as u8 as i32);
                        engine.state.set_object_x_tile(144, lo);
                        engine.state.data_ptr_lo = (lo as u8);
                        engine.state.set_object_x_sub(144, 0);
                        let hi = ((engine.state.player_y
                            + ((engine
                                .state
                                .byte(((SPAWN_OFFSET_Y_TABLE + x2) as u16 as i32)))
                                as u8)) as u8 as i32);
                        engine.state.set_object_y_pixel(144, hi);
                        engine.state.data_ptr_hi = (hi as u8);
                        crate::game::resolve_room_tile_pointer(engine, r);
                        r.offset = 0;
                        engine.state.scratch3 = 0;
                        let p = ((engine.state.data_ptr()) as u16 as i32);
                        let b = engine.state.byte(p) & crate::bits::LOW_6_BITS;
                        if b == 62 {
                            engine.state.set_object_tile(144, 225);
                            engine.state.set_object_state(144, 1);
                            engine.state.set_object_attr(144, 1);
                            engine.state.set_object_timer(144, 15);
                            crate::game::read_room_tile_action_value(engine, r);
                            engine.state.set_object_move_scratch(144, (r.value as i32));
                            crate::game::consume_magic_point(engine, r);
                            engine.state.prompt_state = 20;
                        }
                    }
                }
                r.carry = 1;
                return;
            }
            if idx == 2 {
                if (engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
                    r.offset = 1;
                    crate::game::build_direction_velocity(engine, r);
                    r.offset = 248;
                    let p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
                    engine.state.obj_tile = ((engine.state.byte(((p79 + 248) as u16 as i32))
                        & crate::bits::CLEAR_BIT0)
                        as u8);
                    engine.state.obj_state = 1;
                    engine.state.obj_attr = 3;
                    r.offset = (engine.state.scratch3 as u8);
                    let b = engine
                        .state
                        .byte(((tile_ptr + (r.offset as i32)) as u16 as i32));
                    engine.state.obj_move_scratch = (b as u8);
                    engine.state.obj_timer = 16;
                    crate::game::read_room_tile_action_value(engine, r);
                    engine.state.set_byte(
                        ((tile_ptr + (r.offset as i32)) as u16 as i32),
                        (r.value as i32),
                    );
                    crate::game::seed_object_position_from_tile_offset(engine, r);
                    crate::game::redraw_room_tile_column(engine, r);
                    crate::game::update_tile_projectile_motion(engine, r);
                    engine.state.slot_index = 255;
                    if engine.state.object_state(144) != 0 {
                        engine.state.prompt_state = 6;
                    }
                }
                engine.state.vertical_delta = 0;
                engine.state.fall_frames = 0;
                r.carry = 1;
                return;
            }
            if idx == 3 {
                if engine.state.player_magic != 0 {
                    if (engine.state.direction_latch & ((crate::bits::LOW_NIBBLE) as u8)) != 0 {
                        r.offset = 8;
                        crate::game::build_direction_velocity(engine, r);
                        r.offset = 248;
                        let p79 = ((engine.state.tile_table_ptr()) as u16 as i32);
                        engine.state.obj_tile = ((engine.state.byte(((p79 + 248) as u16 as i32))
                            & crate::bits::CLEAR_BIT0)
                            as u8);
                        engine.state.obj_state = 1;
                        engine.state.obj_attr = 3;
                        r.offset = (engine.state.scratch3 as u8);
                        let b = engine
                            .state
                            .byte(((tile_ptr + (r.offset as i32)) as u16 as i32));
                        engine.state.obj_move_scratch = (b as u8);
                        engine.state.obj_timer = 0;
                        crate::game::read_room_tile_action_value(engine, r);
                        engine.state.set_byte(
                            ((tile_ptr + (r.offset as i32)) as u16 as i32),
                            (r.value as i32),
                        );
                        crate::game::seed_object_position_from_tile_offset(engine, r);
                        crate::game::redraw_room_tile_column(engine, r);
                        crate::game::update_tile_projectile_motion(engine, r);
                        engine.state.slot_index = 255;
                        if engine.state.obj_state != 0 {
                            engine.state.prompt_state = 20;
                            crate::game::consume_magic_point(engine, r);
                        }
                        engine.state.vertical_delta = 0;
                        engine.state.fall_frames = 0;
                        r.carry = 1;
                        return;
                    }
                    engine.state.vertical_delta = 0;
                    engine.state.fall_frames = 0;
                    r.carry = 1;
                    return;
                }
                r.carry = 1;
                return;
            }
        }
        r.carry = 1;
        return;
    }
    r.carry = ((tile >= 48) as u8);
}

/// Fades the room palette out and resets active audio channel state.
pub fn fade_room_palette_out_reset_audio(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.music_volume_override =
        (engine.state.music_volume_override + 1) & ((crate::bits::BYTE_MASK) as u8);
    let mut y = 4;
    loop {
        engine.state.frame_counter = 5;
        for x in (0..=28).rev() {
            let v = engine.state.palette_buffer(4 + x);
            let lo = v & crate::bits::LOW_NIBBLE;
            let hi = v & crate::bits::HIGH_NIBBLE;
            engine.state.scratch0 = (lo as u8);
            engine.state.set_vram_stage(
                68 + x,
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32)
                } else {
                    15
                },
            );
        }
        engine
            .state
            .set_sound_channel_byte(13, 0, engine.state.sound_channel_byte(13, 0) >> 1);
        engine
            .state
            .set_sound_channel_byte(13, 16, engine.state.sound_channel_byte(13, 16) >> 1);
        engine
            .state
            .set_sound_channel_byte(13, 48, engine.state.sound_channel_byte(13, 48) >> 1);
        engine.state.set_sound_channel_byte(1, 32, 0);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
    engine.state.song = 255;
    engine.state.set_sound_channel_byte(1, 0, 0);
    engine.state.sound_channel_flags = 0;
    engine.state.set_sound_channel_byte(1, 48, 0);
    engine.state.music_volume_override = 0;
}

/// Fades the room palette out while preserving active audio channel state.
pub fn fade_room_palette_out_keep_audio(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 4;
    loop {
        engine.state.frame_counter = 5;
        for x in (0..=28).rev() {
            let v = engine.state.palette_buffer(4 + x);
            let lo = v & crate::bits::LOW_NIBBLE;
            let hi = v & crate::bits::HIGH_NIBBLE;
            engine.state.scratch0 = (lo as u8);
            engine.state.set_vram_stage(
                68 + x,
                if hi >= 16 {
                    (((hi - 16) | lo) as u8 as i32)
                } else {
                    15
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
pub fn fade_room_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    let mut v = 64;
    engine.state.scratch1 = v;
    loop {
        engine.state.frame_counter = 5;
        crate::game::build_room_palette_buffer(engine, r);
        r.index = 4;
        r.offset = 28;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = ((engine.state.scratch1 - 16) as u8);
        engine.state.scratch1 = v;
        if (v & ((crate::bits::BIT7) as u8)) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
}

/// Flashes the palette buffer `r.index` times by alternating a bright fill and
/// rebuilt palette upload.
pub fn flash_palette_buffer(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x = r.index;
    loop {
        for i in (0..=31).rev() {
            engine.state.set_palette_buffer(i, 48);
        }
        crate::game::upload_palette_buffer(engine, r);
        engine.state.frame_counter = 1;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::build_room_palette_buffer(engine, r);
        crate::game::upload_palette_buffer(engine, r);
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

pub fn animate_health_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count health up one point at a time so the HUD and prompt animation match
    // the original refill reward pacing.
    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0;
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine.state.player_health = engine.state.player_health.wrapping_add(1);
        crate::game::sync_health_hud(engine, r);
        engine.state.prompt_state = 22;
        engine.state.frame_counter = 2;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_health;
        if engine.state.player_health >= 99 {
            break;
        }
    }
    engine.state.prompt_state = 23;
    engine.state.frame_counter = 0;
    frame::commit_frame_work(engine, r);
    engine.state.sprite_blink_timer = saved_blink;
}

pub fn animate_magic_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count magic up one point at a time, sharing the same prompt/blink pacing
    // as the health refill.
    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0;
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine.state.player_magic = engine.state.player_magic.wrapping_add(1);
        crate::game::sync_magic_hud(engine, r);
        engine.state.prompt_state = 22;
        engine.state.frame_counter = 2;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_magic;
        if engine.state.player_magic >= 99 {
            break;
        }
    }
    engine.state.prompt_state = 23;
    engine.state.frame_counter = 0;
    frame::commit_frame_work(engine, r);
    engine.state.sprite_blink_timer = saved_blink;
}

/// Spends a key and runs the door-unlock prompt/music sequence. Carry is set
/// only when a key was available and the door event completed.
pub fn unlock_door_with_key(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::consume_key(engine, r);
    if ((r.carry) != 0) {
        engine.state.prompt_state = 6;
        r.carry = 0;
        return;
    }

    let ptr = ((engine.state.palette_src_ptr()) as u16 as i32);
    let door = engine.state.byte(((ptr + 10) as u16 as i32));
    if door < 8 {
        engine.state.set_object_attr(160, 0);
    }
    engine.state.set_object_state(160, door + 2);
    engine
        .state
        .set_object_tile(160, ((door << 2) & crate::bits::BYTE_MASK) + 129);
    engine.state.prompt_state = 31;
    crate::game::draw_room_object_sprites(engine, r);

    let saved_blink = engine.state.sprite_blink_timer;
    engine.state.sprite_blink_timer = 0;
    crate::game::draw_player_sprites(engine, r);

    let saved_song = engine.state.song;
    engine.state.song = 14;
    r.value = 14;
    crate::game::song_init(engine, r);

    engine.state.frame_counter = 120;
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.state.song = saved_song;
    r.value = (saved_song as u8);
    crate::game::song_init(engine, r);

    engine.state.sprite_blink_timer = saved_blink;
    r.carry = 1;
}

/// Opens the in-game character-select overlay, waits for a press/release of the
/// character-select button, then restores the gameplay room.
pub fn run_character_select_overlay(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.prompt_state = 3;
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);

    if engine.state.chr_bank(3) < 48 {
        push_room_checkpoint(engine, r);
        r.value = 8;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.state.scroll_fine_x = 8;
        crate::game::refresh_scroll_register_shadows(engine, r);
        crate::game::draw_player_sprites(engine, r);
        fade_room_palette_in(engine, r);
    }

    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        if (frame::read_buttons(engine, r) & crate::bits::BIT4) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    engine.state.prompt_state = 4;

    if engine.state.chr_bank(3) < 48 {
        pop_room_checkpoint(engine, r);
        fade_room_palette_out_reset_audio(engine, r);
        crate::game::clear_temporary_room_sprites(engine, r);
        r.value = (engine.state.room_restore_scratch as u8);
        crate::game::switch_song_if_needed(engine, r);
        crate::game::prepare_room_metadata_and_palette(engine, r);
        crate::game::upload_current_room_view(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        crate::game::refresh_scroll_register_shadows(engine, r);
        fade_room_palette_in(engine, r);
    }

    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);
}

/// Shows the read-only inventory item-list page until the player presses a
/// button, then returns to the character-selection room page.
pub fn show_inventory_item_list_screen(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scroll_tile_x = 16;
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);

    engine.state.indirect_ptr_lo = 212;
    engine.state.indirect_ptr_hi = 180;
    crate::game::encode_inventory_snapshot_item_list(engine, r);
    crate::game::upload_inventory_item_list(engine, r);

    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        if frame::read_buttons(engine, r) != 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    engine.state.scroll_tile_x = 32;
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);
}

/// Runs the interactive inventory item-grid editor from the character-selection
/// room.
pub fn run_inventory_item_grid_menu(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.scroll_tile_x = 48;
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::clear_inventory_item_list_buffer(engine, r);
    crate::game::upload_inventory_item_list(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);

    loop {
        if frame::read_buttons(engine, r) == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }

    engine.state.obj_x_sub = 0;
    engine.state.obj_x_vel_lo = 0;
    engine.state.obj_y_vel = 0;
    engine.state.set_oam_tile(128, 245);
    engine.state.set_oam_tile(144, 245);
    engine.state.set_oam_tile(132, 247);
    engine.state.set_oam_tile(148, 247);
    engine.state.set_oam_attr(128, 0);
    engine.state.set_oam_attr(132, 0);
    engine.state.set_oam_attr(144, 0);
    engine.state.set_oam_attr(148, 0);
    crate::game::update_inventory_list_cursor_sprites(engine, r);
    crate::game::update_inventory_grid_cursor_sprites(engine, r);

    loop {
        engine.state.frame_counter = 1;
        let b = frame::read_buttons(engine, r);
        r.value = (b as u8);

        if (b & crate::bits::BIT7) != 0 {
            crate::game::select_inventory_grid_entry(engine, r);
            crate::game::upload_inventory_item_list(engine, r);
        } else if (b & crate::bits::BIT6) != 0 {
        } else if (b & crate::bits::BIT0) != 0 {
            crate::game::move_inventory_cursor_right(engine, r);
        } else if (b & crate::bits::BIT1) != 0 {
            crate::game::move_inventory_cursor_left(engine, r);
        } else if (b & crate::bits::BIT2) != 0 {
            crate::game::move_inventory_cursor_down(engine, r);
        } else if (b & crate::bits::BIT3) != 0 {
            crate::game::move_inventory_cursor_up(engine, r);
            crate::game::upload_inventory_item_list(engine, r);
        } else if (b & crate::bits::BIT4) != 0 {
            crate::game::close_inventory_item_menu(engine, r);
        } else if (b & crate::bits::BIT5) != 0 {
            engine.state.scroll_tile_x = 32;
            crate::game::upload_staged_room_columns(engine, r);
            crate::game::refresh_scroll_register_shadows(engine, r);
            crate::game::restore_status_sprite_template(engine, r);
            return;
        }

        if (engine.state.buttons & ((crate::bits::CLEAR_BITS_4_5) as u8)) != 0 {
            engine.state.prompt_state = 12;
            engine.state.frame_counter = 10;
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special room flow used to refill resources, return carried items,
/// pick a family member, and optionally visit the inventory item pages.
pub fn run_character_select_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.map_screen_y != 16 {
        push_room_checkpoint(engine, r);
        r.value = 4;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_coin_cost_sprites(engine, r);
        fade_room_palette_in(engine, r);

        loop {
            walk_purchase_room_until_action_or_exit(engine, r);
            if ((r.carry) != 0) {
                crate::game::restore_room_from_checkpoint(engine, r);
                return;
            }
            if engine.state.coins < 10 {
                engine.state.prompt_state = 6;
                continue;
            }

            let mut x = 10;
            loop {
                engine.state.coins = (engine.state.coins - 1) & ((crate::bits::BYTE_MASK) as u8);
                crate::game::sync_coin_hud(engine, r);
                engine.state.prompt_state = 12;
                engine.state.frame_counter = 10;
                frame::commit_frame_work(engine, r);
                frame::wait_for_frame_counter(engine, r);
                x = ((x - 1) as u8 as i32);
                if x == 0 {
                    break;
                }
            }
            fade_room_palette_out_keep_audio(engine, r);
            animate_health_refill_to_cap(engine, r);
            animate_magic_refill_to_cap(engine, r);
            r.value = 8;
            crate::game::refresh_temporary_room_page(engine, r);
            crate::game::draw_carried_item_sprites(engine, r);
            crate::game::upload_inventory_count_tiles(engine, r);
            crate::game::upload_equipped_item_stat_tiles(engine, r);
            engine.state.scroll_fine_x = 8;
            crate::game::refresh_scroll_register_shadows(engine, r);
            crate::game::draw_player_sprites(engine, r);
            fade_room_palette_in(engine, r);
            run_carried_item_loadout_flow(engine, r);
            r.value = 4;
            crate::game::refresh_temporary_room_page(engine, r);
            crate::game::clear_temporary_room_sprites(engine, r);
            crate::game::draw_coin_cost_sprites(engine, r);
            fade_room_palette_in(engine, r);
        }
    }

    engine.state.player_health = 0;
    engine.state.player_magic = 0;
    if engine.state.character_index < 6 {
        for y in (0..=2).rev() {
            let x = engine.state.item_slot(y);
            if (x & crate::bits::BIT7) == 0 {
                engine.state.set_inventory_item(
                    x,
                    (engine.state.inventory_item(x) + 1) & crate::bits::BYTE_MASK,
                );
            }
            engine.state.set_item_slot(y, 255);
        }
        crate::game::snapshot_inventory_state(engine, r);
    }

    push_room_checkpoint(engine, r);
    engine.state.character_index = 6;
    r.value = 6;
    crate::game::enter_temporary_room_page(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    engine.state.selected_item_slot = 3;
    crate::game::draw_status_item_sprites(engine, r);
    engine.state.player_pose = 241;
    engine.state.player_facing = 0;
    crate::game::draw_player_sprites(engine, r);
    crate::game::restore_status_sprite_template(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        walk_character_select_room_until_action(engine, r);
        let hi = engine.state.scratch2 & ((crate::bits::HIGH_NIBBLE) as u8);
        let mut chosen: Option<i32> = None;
        if hi == 80 {
            if (engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8)) == 5
                && engine.state.continue_timer != 0
            {
                let mut x = ((engine.state.song + 1) as u8 as i32);
                if x >= 16 {
                    x = 0;
                }
                engine.state.song = (x as u8);
                crate::game::song_init(engine, r);
                if (engine.state.continue_timer & ((crate::bits::BIT7) as u8)) != 0
                    && engine.state.buttons == 195
                {
                    for x in (0..=13).rev() {
                        engine.state.set_inventory_item(x, 16);
                    }
                    engine.state.continue_timer = 128;
                    engine.state.coins = 128;
                    engine.state.keys = 128;
                    engine.state.prompt_state = 26;
                }
            }
            continue;
        } else if hi == 112 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo == 6 {
                chosen = Some(0);
            } else if lo == 8 {
                chosen = Some(1);
            }
        } else if hi == 128 {
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
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo == 6 {
                chosen = Some(3);
            } else if lo == 10 {
                chosen = Some(4);
            }
        }

        let Some(x) = chosen else {
            continue;
        };

        engine.state.character_index = (x as u8);
        r.offset = (((x << 2) + 3) as u8);
        for xi in (0..=3).rev() {
            engine.state.set_item_slot(
                11 + xi,
                engine
                    .state
                    .byte(((CHARACTER_STATS_TABLE + (r.offset as i32)) as u16 as i32)),
            );
            r.offset = ((r.offset - 1) as u8);
        }
        engine.state.prompt_state = 24;
        engine.state.prompt_argument = 255;
        engine.state.frame_counter = 4;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 5;
        flash_palette_buffer(engine, r);
        engine
            .state
            .set_chr_bank(2, ((engine.state.character_index + 56) as i32));
        engine.state.set_chr_bank(3, 61);
        engine.state.set_chr_bank(4, 62);
        engine.state.set_chr_bank(5, 63);
        engine.state.player_pose = 13;
        engine.state.player_facing = 0;
        engine.state.player_y = engine.state.player_y & ((crate::bits::HIGH_NIBBLE) as u8);
        engine.state.player_x_fine = 4;
        crate::game::clear_gameplay_object_sprites(engine, r);
        crate::game::draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 5;
        flash_palette_buffer(engine, r);
        engine.state.frame_counter = 120;
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        fade_room_palette_out_reset_audio(engine, r);
        engine.state.player_pose = 8;
        engine.state.player_facing = 0;
        engine.state.player_health = 99;
        engine.state.player_magic = 99;
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        engine.state.selected_item_slot = 2;
        crate::game::draw_status_item_sprites(engine, r);
        r.value = 8;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.state.scroll_fine_x = 8;
        crate::game::refresh_scroll_register_shadows(engine, r);
        crate::game::draw_player_sprites(engine, r);
        fade_room_palette_in(engine, r);
        run_carried_item_loadout_flow(engine, r);
        crate::game::restore_room_from_checkpoint(engine, r);
        return;
    }
}

/// Runs the overhead-tile shop room. The caller enters through room tile `0x04`;
/// this flow preserves the current room, stages the shop room, sells the two
/// visible shop items, and restores gameplay when the player reaches the exit.
pub fn run_shop_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    push_room_checkpoint(engine, r);

    let s80 = engine.state.temp_save(0);
    let s81 = engine.state.temp_save(1);
    let s82 = engine.state.temp_save(2);
    let s83 = engine.state.temp_save(3);
    r.value = (engine.state.map_screen_x as u8);
    crate::game::enter_temporary_room_page(engine, r);
    engine.state.set_temp_save(3, s83);
    engine.state.set_temp_save(2, s82);
    engine.state.set_temp_save(1, s81);
    engine.state.set_temp_save(0, s80);

    crate::game::draw_shop_item_sprites(engine, r);
    crate::game::upload_shop_price_tiles(engine, r);
    crate::game::draw_coin_cost_sprites(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        walk_purchase_room_until_action_or_exit(engine, r);
        if ((r.carry) != 0) {
            crate::game::restore_room_from_checkpoint(engine, r);
            return;
        }

        let nib = engine.state.player_x_tile & ((crate::bits::LOW_NIBBLE) as u8);
        let x = if nib < 3 {
            continue;
        } else if nib < 5 {
            0
        } else {
            if nib < 10 || nib >= 12 {
                continue;
            }
            2
        };

        let item = engine.state.temp_save(x);
        if (item & crate::bits::BIT7) != 0 {
            engine.state.prompt_state = 6;
        } else {
            let price = engine.state.inventory_item(33 + x);
            r.value = (price as u8);
            crate::game::spend_coins(engine, r);
            if ((r.carry) != 0) {
                engine.state.set_temp_save(x, 255);
                crate::game::draw_shop_item_sprites(engine, r);
                engine.state.set_inventory_item(
                    item,
                    (engine.state.inventory_item(item) + 1) & crate::bits::BYTE_MASK,
                );
                engine.state.prompt_state = 16;
            } else {
                if item == 13 && engine.state.continue_timer != 0 {
                    engine.state.shop_active = 1;
                }
                engine.state.prompt_state = 6;
            }
        }

        loop {
            if frame::read_buttons(engine, r) == 0 {
                break;
            }
            frame::wait_frame(engine, r);
        }
    }
}

/// Walks the character-select room until the action button is pressed, keeping
/// `0x43..0x45` pointed at the last selectable tile under the player.
pub fn walk_character_select_room_until_action(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.frame_counter = 1;
        let buttons = frame::read_buttons(engine, r);
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            return;
        }

        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

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

        crate::game::draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special-exit actor animation. After the rising/falling sequence
/// collides with the playfield, the actor clears itself and raises `0xEB` so
/// the foreground loop enters the pending special-exit room.
pub fn tick_special_exit_actor_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.state.obj_state & ((crate::bits::LOW_7_BITS) as u8)) == 0 {
        engine.state.prompt_state = 24;
        engine.state.prompt_argument = 255;
        r.index = 3;
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

        engine.state.obj_state = (engine.state.obj_state + 1) & ((crate::bits::BYTE_MASK) as u8);
        engine.state.prompt_state = 2;
        engine.state.obj_cooldown = 15;
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_move_scratch = 0;
        engine.state.obj_y_extra = engine.state.obj_y_pixel;
    }

    if engine.state.obj_move_scratch == 0 {
        engine.state.obj_cooldown =
            (engine.state.obj_cooldown - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.obj_cooldown == 0 {
            engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
            engine.state.obj_move_scratch = 1;
            return;
        }
        let a = ((((engine.state.obj_cooldown >> 2) ^ ((crate::bits::BYTE_MASK) as u8)) + 1) as u8
            as i32);
        engine.state.obj_y_vel = (a as u8);
        crate::game::project_actor_position(engine, r);
        crate::game::check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
            engine.state.obj_move_scratch = 1;
            return;
        }
        engine.state.obj_y_pixel = engine.state.scratch2;
        return;
    }

    engine.state.obj_move_scratch =
        (engine.state.obj_move_scratch + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 2) + 1;
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.obj_state = 0;
        engine.state.obj_timer = 240;
        engine.state.pending_special_exit = 1;
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
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            r.carry = 0;
            return;
        }

        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.state.scratch2;
        if ty >= 161 {
            r.value = (ty as u8);
            r.carry = 1;
            return;
        }
        if ty >= 140 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            if lo >= 2 && lo < 13 {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                engine.state.player_y = engine.state.scratch2;
            }
        }

        crate::game::update_player_pose_from_motion(engine, r);
        crate::game::tick_player_walk_animation(engine, r);
        crate::game::draw_player_sprites(engine, r);
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
        if (buttons & crate::bits::BIT7) != 0 {
            r.value = 128;
            r.carry = 0;
            return;
        }

        r.value = ((buttons & crate::bits::LOW_NIBBLE) as u8);
        r.offset = 1;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.state.scratch2;
        if ty >= 161 {
            r.value = (ty as u8);
            r.carry = 1;
            return;
        }
        if ty >= 32 {
            let lo = engine.state.indirect_ptr_hi & ((crate::bits::LOW_NIBBLE) as u8);
            let mut store = false;
            if lo >= 1 {
                if lo < 15 {
                    store = true;
                } else if engine.state.indirect_ptr_lo == 0 {
                    store = true;
                }
            }
            if store {
                engine.state.player_x_fine = engine.state.indirect_ptr_lo;
                engine.state.player_x_tile = engine.state.indirect_ptr_hi;
                engine.state.player_y = engine.state.scratch2;
            }
        }

        crate::game::update_player_pose_from_motion(engine, r);
        crate::game::tick_player_walk_animation(engine, r);
        crate::game::draw_player_sprites(engine, r);
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
        walk_loadout_room_until_action_or_exit(engine, r);
        if ((r.carry) != 0) {
            let e = engine.state.selected_item_slot;
            if engine.state.item_slot((e as i32)) == 13 {
                engine.state.selected_item_slot = 3;
                crate::game::draw_status_item_sprites(engine, r);
            }
            return;
        }
        let mut x = 255;
        let py = engine.state.player_y;
        let flow_0441 = if py >= 88 {
            true
        } else {
            x = if py < 56 { 0 } else { 8 };
            engine.state.scratch0 = x;
            x = (((engine.state.player_x_tile >> 1) | engine.state.scratch0) as u8);
            if engine.state.inventory_item((x as i32)) != 0 {
                r.value = (x as u8);
                crate::game::load_family_item_permission_bits(engine, r);
                if ((r.carry) != 0) {
                    engine.state.set_inventory_item(
                        (x as i32),
                        (engine.state.inventory_item((x as i32)) - 1) & crate::bits::BYTE_MASK,
                    );
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if !flow_0441 {
            engine.state.prompt_state = 6;
            continue;
        }
        engine.state.scratch0 = x;
        let ci0 = engine.state.item_slot(0);
        if (ci0 & crate::bits::BIT7) == 0 {
            engine.state.set_inventory_item(
                ci0,
                (engine.state.inventory_item(ci0) + 1) & crate::bits::BYTE_MASK,
            );
        }
        engine.state.set_item_slot(0, engine.state.item_slot(1));
        engine.state.set_item_slot(1, engine.state.item_slot(2));
        engine
            .state
            .set_item_slot(2, (engine.state.scratch0 as i32));
        engine.state.prompt_state = 18;
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::draw_status_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
    }
}

/// Saves the current gameplay room state before entering a temporary room such
/// as a shop or character-select room. The current song is mirrored in `0xFE`
/// so the restore path can restart it after rebuilding the room.
pub fn push_room_checkpoint(engine: &mut Engine, _r: &mut RoutineContext) {
    engine.state.room_restore_scratch = engine.state.song;
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
    const DROP_ITEM_TABLE: [i32; 9] = [3, 3, 3, 3, 4, 4, 5, 6, 7];
    if (engine.state.obj_state & ((crate::bits::LOW_7_BITS) as u8)) == 0 {
        engine.state.obj_state = (engine.state.obj_state + 1) & ((crate::bits::BYTE_MASK) as u8);
        engine.state.prompt_state = 14;
        engine.state.obj_cooldown = 8;
        engine.state.obj_x_vel_lo = 0;
        engine.state.obj_x_vel_hi = 0;
        engine.state.obj_move_scratch = 0;
        engine.state.obj_y_extra = engine.state.obj_y_pixel;
        let ptr = (((engine.state.actor_record_ptr_lo as i32)
            | ((engine.state.actor_record_ptr_hi as i32) << 8)) as u16 as i32);
        engine.state.obj_tile = ((engine.state.byte(((ptr + 6) as u16 as i32))) as u8);
        engine.state.obj_attr = engine.state.obj_attr & ((crate::bits::LOW_2_BITS) as u8);
    }
    if engine.state.obj_move_scratch == 0 {
        engine.state.obj_cooldown =
            (engine.state.obj_cooldown - 1) & ((crate::bits::BYTE_MASK) as u8);
        if engine.state.obj_cooldown != 0 {
            engine.state.obj_y_vel = 0 - engine.state.obj_cooldown;
            crate::game::project_actor_position(engine, r);
            crate::game::check_position_out_of_bounds(engine, r);
            if ((r.carry) == 0) {
                engine.state.obj_y_pixel = engine.state.scratch2;
                return;
            }
        }
        engine.state.obj_attr = engine.state.obj_attr | ((crate::bits::BIT7) as u8);
        engine.state.obj_move_scratch = 1;
        return;
    }
    engine.state.obj_move_scratch =
        (engine.state.obj_move_scratch + 1) & ((crate::bits::BYTE_MASK) as u8);
    engine.state.obj_y_vel = (engine.state.obj_move_scratch >> 1) + 2;
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if ((r.carry) == 0) {
        engine.state.obj_y_pixel = engine.state.scratch2;
        return;
    }
    let mut x = 0;
    if engine.state.player_health < 20 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 1;
    if engine.state.player_magic < 30 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 4;
    if engine.state.keys < 2 {
        item_spawn_setup(engine, r, x);
        return;
    }
    r.value = 20;
    crate::game::rng_update(engine, r);
    if r.value >= 9 {
        x = 0;
        if engine.state.player_health < engine.state.player_magic {
            if (engine.state.player_health as i32) < (engine.state.coins as i32) {
                item_spawn_setup(engine, r, x);
                return;
            }
            x = 2;
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 1;
        if (engine.state.player_magic as i32) < (engine.state.coins as i32) {
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 2;
        item_spawn_setup(engine, r, x);
        return;
    }
    x = DROP_ITEM_TABLE[r.value as usize];
    item_spawn_setup(engine, r, x);
}
