use crate::game::{
    CHARACTER_STATS_TABLE, SPAWN_OFFSET_X_TABLE, SPAWN_OFFSET_Y_TABLE, START_ITEM_TABLE,
};
use crate::{Engine, RoutineContext, engine::RoutineFn, frame, u8v, u16v};

fn enter_return_home(engine: &mut Engine, lo: i32, hi: i32) {
    engine.state.set_indirect_ptr_lo(lo);
    engine.state.set_indirect_ptr_hi(hi);
    engine
        .state
        .set_prg_bank_8000(engine.state.saved_prg_bank_8000());
    engine
        .state
        .set_prg_bank_a000(engine.state.saved_prg_bank_a000());
    engine.state.set_mmc3_bank_select(0x06);
    engine.prg_map_shadow();
}

fn leave_return_home(engine: &mut Engine) {
    engine.state.set_prg_bank_8000(0x0c);
    engine.state.set_prg_bank_a000(0x0d);
    engine.state.set_mmc3_bank_select(0x07);
    engine.prg_map_shadow();
}

fn farcall_cce4(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    enter_return_home(engine, lo, hi);
    target(engine, r);
    leave_return_home(engine);
}

fn farcall_0c0d(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    let old6 = engine.state.prg_bank_8000();
    let old7 = engine.state.prg_bank_a000();
    engine.state.set_saved_prg_bank_8000(old6);
    engine.state.set_saved_prg_bank_a000(old7);
    engine.state.set_indirect_ptr_lo(lo);
    engine.state.set_indirect_ptr_hi(hi);
    engine.state.set_prg_bank_8000(0x0c);
    engine.state.set_prg_bank_a000(0x0d);
    engine.state.set_mmc3_bank_select(0x07);
    engine.prg_map_shadow();
    target(engine, r);
    engine.state.set_prg_bank_a000(old7);
    engine.state.set_prg_bank_8000(old6);
    engine.state.set_mmc3_bank_select(0x06);
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
    engine.state.set_vram_addr_lo(dlo);
    engine.state.set_vram_addr_hi(dhi);
    engine.state.set_vram_addr2_lo(slo);
    engine.state.set_vram_addr2_hi(shi);
    engine.state.set_inventory_upload_col(len);
    r.value = 0x05;
    queue_ppu_job_and_wait(engine, r);
}

fn item_spawn_setup(engine: &mut Engine, r: &mut RoutineContext, x: i32) {
    engine.state.set_obj_state(u8v(x + 2));
    engine.state.set_obj_tile(u8v((x << 2) | 0x81));
    engine.state.set_obj_attr(0x01);
    engine.state.set_obj_y_pixel(engine.state.obj_y_extra());
    engine.state.set_obj_timer(0xf0);
    engine.state.set_obj_move_scratch(0x00);
    engine.state.set_obj_cooldown(0x00);
    crate::game::update_object_terrain_probe(engine, r);
}

/// Queues the VRAM job id in `r.value` and waits until the NMI-side upload has
/// consumed it.
pub fn queue_ppu_job_and_wait(engine: &mut Engine, r: &mut RoutineContext) {
    frame::wait_for_ppu_job_idle(engine, r);
    engine.state.set_nmi_vram_req(r.value);
    frame::wait_for_ppu_job_idle(engine, r);
}

/// Shows the start-button prompt and waits for release, press, and release so a
/// held Start does not leak into the next menu/gameplay state.
pub fn wait_for_start_button_prompt(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_prompt_state(0x03);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() + 1) & 0xFF);
    loop {
        let buttons = frame::read_buttons(engine, r);
        if buttons == 0 {
            break;
        }
        frame::wait_frame(engine, r);
    }
    loop {
        let buttons = frame::read_buttons(engine, r);
        if (buttons & 0x10) != 0 {
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
    engine.state.set_prompt_state(0x04);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() - 1) & 0xFF);
}

pub fn main_loop_dispatch(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        if frame::frame_runner_stop_requested() {
            return;
        }
        if engine.state.player_health() == 0 {
            engine.state.set_sprite_blink_timer(0x00);
            crate::game::draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 0x07, 0xb3, run_player_death_or_continue_flow);
            if r.index == 0 {
                continue;
            }
            r.index = u8v(r.index - 1);
            crate::game::main_init(engine, r);
            return;
        }

        engine.state.set_frame_counter(0x01);
        engine
            .state
            .set_saved_scroll_tile(engine.state.scroll_tile_x());
        frame::read_buttons(engine, r);
        crate::game::game_update(engine, r);

        if engine.state.final_exit_flag() != 0 {
            // The final-exit item diverts the normal room loop into a scripted
            // sequence that still reuses the player/object update helpers.
            farcall_0c0d(engine, r, 0xeb, 0xa2, setup_final_exit_sequence);
            loop {
                frame::read_buttons(engine, r);
                farcall_0c0d(
                    engine,
                    r,
                    0xbc,
                    0xab,
                    crate::game::tick_scripted_player_motion,
                );
                farcall_0c0d(
                    engine,
                    r,
                    0xe6,
                    0xa5,
                    crate::game::update_final_exit_projectiles,
                );
                farcall_0c0d(
                    engine,
                    r,
                    0x5d,
                    0xa7,
                    crate::game::rotate_sprite_zero_from_scripted_oam,
                );
                farcall_0c0d(engine, r, 0xe3, 0xa3, tick_final_exit_sequence);
                if engine.state.player_health() != 0 {
                    break;
                }
            }

            engine
                .state
                .set_player_x_tile(engine.state.player_x_fine() >> 4);
            engine
                .state
                .set_player_x_fine(engine.state.player_x_fine() & 0x0f);
            engine.state.set_oam_y(0x00, 0xef);
            engine.state.set_sprite_blink_timer(0x00);
            crate::game::draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 0x07, 0xb3, run_player_death_or_continue_flow);
            r.index = u8v(r.index - 1);
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
        if !((r.carry) != 0) && engine.state.saved_scroll_tile() != engine.state.scroll_tile_x() {
            engine
                .state
                .set_main_loop_phase((engine.state.main_loop_phase() + 1) & 0xFF);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Sets up the final-exit sequence after the final item trigger: flash the
/// current scene, switch to the scripted room, and seed the special object/player
/// state used by `tick_final_exit_sequence`.
pub fn setup_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_prompt_state(0x18);
    engine.state.set_sprite_blink_timer(0x00);
    crate::game::draw_player_sprites(engine, r);

    r.index = 0x02;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    r.index = 0x03;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    fade_partial_palette_buffer_out(engine, r);

    engine.state.set_prompt_state(0x20);
    engine.state.set_frame_counter(0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.state.set_map_screen_y(0x13);
    engine.state.set_map_screen_x(0x02);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);

    engine.state.set_oam_y(0x00, 0xef);
    engine.state.set_scroll_y(0x22);
    engine.state.set_scroll_fine_x(0x00);
    engine.state.set_player_x_fine(0x00);
    engine.state.set_scroll_tile_x(0x10);
    farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::upload_current_room_view);
    r.index = 0x04;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    engine.state.set_scroll_tile_x(0x00);
    farcall_cce4(
        engine,
        r,
        0x6c,
        0xc7,
        crate::game::upload_room_columns_from_bank9,
    );
    engine.state.set_chr_bank(3, 0x3d);

    loop {
        let mut x = engine.state.scroll_y();
        if x == 0 {
            x = 0xf0;
        }
        if x == 0xc2 {
            break;
        }
        x = u8v(x - 1);
        engine.state.set_scroll_y(x);
        engine.state.set_nametable_select((x & 0x08) >> 3);
        r.value = 0xff;
        queue_ppu_job_and_wait(engine, r);
    }

    r.index = 0x02;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    farcall_cce4(
        engine,
        r,
        0xc7,
        0xc1,
        crate::game::refresh_scroll_register_shadows,
    );

    engine.state.set_object_x_sub(0x00, 0x00);
    engine.state.set_object_x_tile(0x00, 0x00);
    engine.state.set_object_timer(0x00, 0x00);
    engine.state.set_scheduler_phase(0x00);
    engine.state.set_scroll_fine_x(0x00);
    engine.state.set_scroll_tile_x(0x00);
    engine.state.set_object_health(0x00, 0x64);
    engine.state.set_sprite_index(0x08);
    engine.state.set_player_x_fine(u8v(
        (engine.state.player_x_tile() << 4) | engine.state.player_x_fine()
    ));
    crate::game::draw_scripted_player_sprites(engine, r);
    engine.state.set_oam_y(0x10, 0xef);
    engine.state.set_oam_y(0x14, 0xef);
    crate::game::load_final_exit_object_oam_template(engine, r);
    crate::game::load_final_exit_player_oam_template(engine, r);
}

/// Runs the one-shot final-exit cutscene path that is entered before the special
/// object latch at `0xF2` is set.
fn run_final_exit_cutscene(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::build_object_health_meter_standard_tiles(engine, r);
    engine.state.set_object_state(0x10, 0x00);
    engine.state.set_object_state(0x20, 0x00);
    engine.state.set_object_state(0x30, 0x00);
    engine.state.set_obj_health(0x00);
    engine.state.set_sprite_blink_timer(0x00);
    engine.state.set_displaced_timer(0x00);
    crate::game::draw_scripted_player_sprites(engine, r);
    crate::game::draw_final_exit_projectile_sprites(engine, r);
    engine.state.set_oam_y(0x00, 0xef);

    while engine.state.player_y() < 0xa0 {
        engine
            .state
            .set_player_y((engine.state.player_y() + 1) & 0xFF);
        crate::game::draw_scripted_player_sprites(engine, r);
        engine.state.set_frame_counter(0x01);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.state.set_fall_frames(0x00);
    engine.state.set_jump_timer(0x00);
    crate::game::update_scripted_player_pose_from_motion(engine, r);
    crate::game::draw_scripted_player_sprites(engine, r);
    engine.state.set_scroll_tile_x(0x20);
    engine.state.set_nametable_select(0x01);
    engine.state.set_prompt_state(0x20);
    engine.state.set_prompt_argument(0x80);
    engine.state.set_tile_table_ptr_hi(0xb6);

    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile() == 0 {
            break;
        }
    }
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile() == 0 {
            break;
        }
    }

    engine.state.set_prompt_state(0x20);
    engine.state.set_prompt_argument(0x80);
    engine.state.set_tile_table_ptr_hi(0xb7);
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile() == 0 {
            break;
        }
    }
    loop {
        crate::game::advance_scripted_scroll_slice(engine, r);
        if engine.state.obj_x_tile() == 0 {
            break;
        }
    }

    engine.state.set_tile_fetch_counter(0x00);
    loop {
        if (engine.state.frame_prescaler() & 0x07) == 0 {
            engine
                .state
                .set_nametable_select(engine.state.nametable_select() ^ 0x01);
            engine.state.set_prompt_state(0x20);
            engine.state.set_prompt_argument(0x80);
        }

        r.value = 0xff;
        queue_ppu_job_and_wait(engine, r);
        if engine.state.sprite0_hit() {
            r.value = 0x05;
            crate::game::subtract_scripted_player_health(engine, r);
            crate::game::build_player_health_meter_sprites(engine, r);
        }

        if engine.state.sprite_index() == 0 {
            engine.state.set_sprite_index(0x02);
        }

        crate::game::draw_scripted_player_sprites(engine, r);
        crate::game::rotate_sprite_zero_from_scripted_oam(engine, r);
        engine
            .state
            .set_tile_fetch_counter((engine.state.tile_fetch_counter() - 1) & 0xFF);
        if engine.state.tile_fetch_counter() == 0 {
            break;
        }
    }

    engine.state.set_nametable_select(0x01);
    r.value = 0xff;
    queue_ppu_job_and_wait(engine, r);
    if engine.state.player_health() == 0 {
        return;
    }

    engine.state.set_oam_y(0x00, 0xef);
    engine.state.set_prompt_state(0x18);
    engine.state.set_prompt_argument(0xff);
    engine.state.set_scratch0(0x01);
    loop {
        let prev = engine.state.player_y();
        let ny = u8v(prev - engine.state.scratch0());
        engine.state.set_player_y(ny);
        let c = if prev >= engine.state.scratch0() {
            1
        } else {
            0
        };
        let t = u8v(ny + 0x2b + c);
        if t >= 0xef {
            break;
        }
        crate::game::draw_scripted_player_sprites(engine, r);
        engine
            .state
            .set_scratch0((engine.state.scratch0() + 1) & 0xFF);
        r.value = 0xff;
        queue_ppu_job_and_wait(engine, r);
    }

    engine.state.set_oam_y(0x10, 0xef);
    engine.state.set_oam_y(0x14, 0xef);
    engine.state.set_sprite_index(0x00);
    engine.state.set_oam_cursor(0x80);
    crate::game::reset_room_object_slots(engine, r);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    engine.state.set_map_screen_y(0x10);
    engine.state.set_map_screen_x(0x03);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);
    engine.state.set_scroll_tile_x(0x12);
    engine.state.set_player_y(0xc0);
    engine.state.set_player_x_tile(0x1a);
    engine.state.set_player_x_fine(0x01);
    engine.state.set_scroll_fine_x(0x01);
    engine.state.set_player_pose(0x09);
    engine.state.set_chr_bank(2, 0x35);
    engine.state.set_chr_bank(3, 0x34);
    engine.state.set_chr_bank(4, 0x36);
    engine.state.set_chr_bank(5, 0x37);
    engine.state.set_object_state(0x10, 0x01);
    engine.state.set_object_state(0x20, 0x01);
    engine.state.set_object_state(0x30, 0x01);
    engine.state.set_object_state(0x40, 0x01);
    engine.state.set_object_y_pixel(0x10, 0xa0);
    engine.state.set_object_y_pixel(0x20, 0xa0);
    engine.state.set_object_y_pixel(0x30, 0xa0);
    engine.state.set_object_y_pixel(0x40, 0x70);
    engine.state.set_object_x_tile(0x40, 0x33);
    crate::game::sync_final_exit_body_slots_from_player(engine, r);
    let mut v = 0x2d;
    engine.state.set_object_tile(0x10, v);
    v = u8v(v + 0x20);
    engine.state.set_object_tile(0x20, v);
    v = u8v(v + 0x20);
    engine.state.set_object_tile(0x30, v);
    engine.state.set_object_tile(0x40, 0x81);
    engine.state.set_object_attr(0x10, 0x40);
    engine.state.set_object_attr(0x20, 0x40);
    engine.state.set_object_attr(0x30, 0x40);
    engine.state.set_object_attr(0x40, 0x40);
    crate::game::upload_status_panel_template(engine, r);
    farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::upload_current_room_view);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);
    crate::game::clear_gameplay_object_sprites(engine, r);
    crate::game::draw_player_sprites(engine, r);
    crate::game::draw_status_item_sprites(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.state.set_character_index(0x07);
    farcall_cce4(engine, r, 0x92, 0xc4, fade_room_palette_in);
    engine.state.set_countdown_timer(0x05);
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

    loop {
        if engine.state.player_y() == 0xa0 {
            break;
        }
        engine
            .state
            .set_player_y((engine.state.player_y() - 1) & 0xFF);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        if engine.state.player_y() == 0xa0 {
            break;
        }
        engine
            .state
            .set_player_y((engine.state.player_y() - 1) & 0xFF);
        engine
            .state
            .set_player_facing(engine.state.player_facing() ^ 0x40);
        crate::game::draw_player_sprites(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.state.set_player_pose(0x0d);
    crate::game::draw_player_sprites(engine, r);
    engine.state.set_countdown_timer(0x03);
    while engine.state.countdown_timer_active() {
        draw_scene_and_wait_one_frame(engine, r);
    }

    loop {
        engine.state.set_frame_counter(0x01);
        engine
            .state
            .set_saved_scroll_tile(engine.state.scroll_tile_x());
        engine.state.set_buttons(0x01);
        farcall_cce4(engine, r, 0x2b, 0xd4, crate::game::game_update);
        farcall_cce4(
            engine,
            r,
            0x5d,
            0xc1,
            crate::game::update_camera_scroll_from_player,
        );
        crate::game::sync_final_exit_body_slots_from_player(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        if engine.state.saved_scroll_tile() != engine.state.scroll_tile_x() {
            engine
                .state
                .set_main_loop_phase((engine.state.main_loop_phase() + 1) & 0xFF);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        if engine.state.player_x_tile() == 0x37 {
            break;
        }
    }

    engine.state.set_player_pose(0x19);
    engine.state.set_object_tile(0x10, 0x39);
    engine.state.set_object_tile(0x20, 0x59);
    engine.state.set_object_tile(0x30, 0x79);
    engine.state.set_object_tile(0x40, 0x91);
    engine.state.set_countdown_timer(0x14);
    while engine.state.countdown_timer_active() {
        engine
            .state
            .set_player_pose(engine.state.player_pose() ^ 0x04);
        engine
            .state
            .set_object_tile(0x10, engine.state.object_tile(0x10) ^ 0x04);
        engine
            .state
            .set_object_tile(0x20, engine.state.object_tile(0x20) ^ 0x04);
        engine
            .state
            .set_object_tile(0x30, engine.state.object_tile(0x30) ^ 0x04);
        engine
            .state
            .set_object_tile(0x40, engine.state.object_tile(0x40) ^ 0x04);
        for _ in 0..8 {
            draw_scene_and_wait_one_frame(engine, r);
        }
    }

    run_story_text_sequence(engine, r);
}

/// Ticks the final-exit scripted object state machine and stores the updated
/// scratch slot back into the active object slot.
pub fn tick_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_obj_slot_ptr_lo(0x00);
    engine.state.set_obj_slot_ptr_hi(0x04);
    crate::game::load_object_slot_scratch(engine, r);
    if engine.state.obj_health() == 0 {
        run_final_exit_cutscene(engine, r);
        return;
    }

    if engine.state.sprite0_hit() {
        let t = (engine.state.sprite_index() + 2) & 0x06;
        if t != 0 {
            let x = u8v(t << 3);
            if engine.state.object_state(x) != 0 {
                engine.state.set_object_state(x, 0x00);
                let sum = u8v(engine.state.scroll_pixel_x() + engine.state.object_x_sub(x));
                if sum >= 0xb0 && sum < 0xd0 {
                    let bl = engine.state.obj_health();
                    engine
                        .state
                        .set_obj_health(if bl < 0x02 { 0x00 } else { u8v(bl - 0x02) });
                    crate::game::build_object_health_meter_standard_tiles(engine, r);
                    engine.state.set_prompt_state(0x20);
                    engine.state.set_prompt_argument(0x01);
                } else {
                    engine.state.set_prompt_state(0x01);
                }
            }
        }
    }

    if engine.state.obj_x_tile() == 0 {
        match engine.state.obj_timer() {
            4 => {
                engine
                    .state
                    .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                if engine.state.scheduler_phase() != 0 {
                    if engine.state.scheduler_phase() == 0x04 {
                        engine.state.set_prompt_state(0x20);
                    }
                    engine.state.set_tile_table_ptr_hi(0xb5);
                    engine.state.set_scroll_y(0xc2);
                } else {
                    engine.state.set_tile_table_ptr_hi(0xb3);
                    engine.state.set_obj_timer(0x00);
                }
            }
            3 => {
                engine
                    .state
                    .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                if engine.state.scheduler_phase() != 0 {
                    engine.state.set_tile_table_ptr_hi(0xb2);
                    if engine.state.scroll_pixel_x() != 0 {
                        let v = if engine.state.scroll_pixel_x() < 0x04 {
                            0x00
                        } else {
                            u8v(engine.state.scroll_pixel_x() - 0x04)
                        };
                        engine.state.set_scroll_pixel_x(v);
                        if v >= 0x11 {
                            if engine.state.scroll_y() < 0xd2 {
                                engine
                                    .state
                                    .set_scroll_y((engine.state.scroll_y() + 0x04) & 0xFF);
                            } else if engine.state.scroll_pixel_x() != 0 {
                                engine.state.set_scroll_pixel_x(
                                    (engine.state.scroll_pixel_x() - 0x04) & 0xFF,
                                );
                            }
                        } else if engine.state.scroll_y() >= 0xc3 {
                            engine
                                .state
                                .set_scroll_y((engine.state.scroll_y() - 0x04) & 0xFF);
                        }
                    } else if engine.state.scroll_y() >= 0xc3 {
                        engine
                            .state
                            .set_scroll_y((engine.state.scroll_y() - 0x04) & 0xFF);
                    }
                } else if engine.state.scroll_pixel_x() != 0 {
                    engine.state.set_obj_timer(0x00);
                } else {
                    engine.state.set_tile_table_ptr_hi(0xb0);
                    engine
                        .state
                        .set_obj_timer((engine.state.obj_timer() + 1) & 0xFF);
                    engine.state.set_scheduler_phase(0x04);
                }
            }
            2 => {
                engine
                    .state
                    .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                if engine.state.scheduler_phase() != 0 {
                    engine.state.set_tile_table_ptr_hi(0xb4);
                    if engine.state.scroll_y() >= 0xc3 {
                        engine
                            .state
                            .set_scroll_y((engine.state.scroll_y() - 0x04) & 0xFF);
                    }
                } else {
                    engine.state.set_tile_table_ptr_hi(0xb3);
                    engine.state.set_obj_timer(0x00);
                }
            }
            1 => {
                engine
                    .state
                    .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                if engine.state.scheduler_phase() == 0 {
                    engine.state.set_obj_timer(0x00);
                } else {
                    let a = u8v(((engine.state.scheduler_phase() << 1) & 0x01) + 0xb0);
                    engine.state.set_tile_table_ptr_hi(a);
                    engine
                        .state
                        .set_scroll_pixel_x((engine.state.scroll_pixel_x() + 0x04) & 0xFF);
                    if engine.state.scroll_pixel_x() >= 0x40 {
                        engine.state.set_obj_timer(0x00);
                    } else {
                        engine.state.set_scroll_y(0xc2);
                    }
                }
            }
            _ => {
                let sum = u8v(engine.state.scroll_pixel_x() + engine.state.player_x_fine());
                let carry = sum < engine.state.scroll_pixel_x();
                let close = carry || sum >= 0xc0 || engine.state.scroll_pixel_x() >= 0x40;
                let delayed_grow = sum < 0x80 || sum >= 0xa0;
                if close || (delayed_grow && engine.state.scroll_y() >= 0xc3) {
                    engine.state.set_obj_timer(0x03);
                    engine.state.set_scheduler_phase(0x02);
                    engine
                        .state
                        .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                    if engine.state.scheduler_phase() != 0 {
                        engine.state.set_tile_table_ptr_hi(0xb2);
                        if engine.state.scroll_pixel_x() != 0 {
                            let v = if engine.state.scroll_pixel_x() < 0x04 {
                                0x00
                            } else {
                                u8v(engine.state.scroll_pixel_x() - 0x04)
                            };
                            engine.state.set_scroll_pixel_x(v);
                            if v >= 0x11 {
                                if engine.state.scroll_y() < 0xd2 {
                                    engine
                                        .state
                                        .set_scroll_y((engine.state.scroll_y() + 0x04) & 0xFF);
                                } else if engine.state.scroll_pixel_x() != 0 {
                                    engine.state.set_scroll_pixel_x(
                                        (engine.state.scroll_pixel_x() - 0x04) & 0xFF,
                                    );
                                }
                            } else if engine.state.scroll_y() >= 0xc3 {
                                engine
                                    .state
                                    .set_scroll_y((engine.state.scroll_y() - 0x04) & 0xFF);
                            }
                        } else if engine.state.scroll_y() >= 0xc3 {
                            engine
                                .state
                                .set_scroll_y((engine.state.scroll_y() - 0x04) & 0xFF);
                        }
                    } else if engine.state.scroll_pixel_x() != 0 {
                        engine.state.set_obj_timer(0x00);
                    } else {
                        engine.state.set_tile_table_ptr_hi(0xb0);
                        engine
                            .state
                            .set_obj_timer((engine.state.obj_timer() + 1) & 0xFF);
                        engine.state.set_scheduler_phase(0x04);
                    }
                } else if !delayed_grow {
                    engine.state.set_obj_timer(0x02);
                    engine.state.set_scheduler_phase(0x08);
                    engine.state.set_tile_table_ptr_hi(0xb3);
                } else {
                    engine.state.set_obj_timer(0x01);
                    engine.state.set_scheduler_phase(0x04);
                    engine
                        .state
                        .set_scheduler_phase((engine.state.scheduler_phase() - 1) & 0xFF);
                    if engine.state.scheduler_phase() == 0 {
                        engine.state.set_obj_timer(0x00);
                    } else {
                        let a = u8v(((engine.state.scheduler_phase() << 1) & 0x01) + 0xb0);
                        engine.state.set_tile_table_ptr_hi(a);
                        engine
                            .state
                            .set_scroll_pixel_x((engine.state.scroll_pixel_x() + 0x04) & 0xFF);
                        if engine.state.scroll_pixel_x() >= 0x40 {
                            engine.state.set_obj_timer(0x00);
                        } else {
                            engine.state.set_scroll_y(0xc2);
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
    let mut y = 0x04;
    loop {
        engine.state.set_frame_counter(0x05);
        for x in (0..=0x0c).rev() {
            let lo = engine.state.palette_buffer(x) & 0x0f;
            let hi = engine.state.palette_buffer(x) & 0xf0;
            engine.state.set_scratch0(lo);
            let out = if hi < 0x10 {
                0x0f
            } else {
                u8v((hi - 0x10) | lo)
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
    engine.state.set_frame_counter(0x01);
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
        0x8b,
        0xc3,
        crate::game::clear_name_tables_to_blank_tiles,
    );
    crate::game::upload_status_panel_template(engine, r);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    crate::game::reset_menu_state_and_palette(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);

    engine.state.set_frame_counter(0x01);
    enter_return_home(engine, 0x35, 0xc1);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
    leave_return_home(engine);
}

/// Runs the scrolling story-text sequence shared by the title-screen chord and
/// the final-exit cutscene.
pub fn run_story_text_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_music_volume_override((engine.state.music_volume_override() + 1) & 0xFF);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::hide_all_sprite_y_positions(engine, r);
    engine.state.set_chr_bank(0, 0x20);
    engine.state.set_chr_bank(1, 0x22);
    engine
        .state
        .set_ppu_mask_shadow(engine.state.ppu_mask_shadow() | 0x18);

    r.value = 0xff;
    queue_ppu_job_and_wait(engine, r);

    engine.state.set_song(0x0a);
    crate::game::song_init(engine, r);

    engine.state.set_scroll_pixel_x(0x00);
    engine.state.set_nametable_select(0x00);
    engine.state.set_scratch2(0x00);
    engine.state.set_scroll_fine_x(0x00);
    engine.state.set_scroll_tile_x(0x00);
    crate::game::load_intro_text_palette(engine, r);

    engine.state.set_vram_addr2_lo(0x40);
    engine.state.set_vram_addr2_hi(0x01);
    engine.state.set_inventory_upload_col(0x20);
    engine.state.set_data_ptr_lo(0x9c);
    engine.state.set_data_ptr_hi(0xb7);

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

    engine.state.set_prompt_state(0x20);
    while engine.state.sfx_voice_active() == 0 {
        frame::wait_frame(engine, r);
    }
    while engine.state.sfx_voice_active() != 0 {
        frame::wait_frame(engine, r);
    }

    engine.state.set_frame_counter(0x3c);
    frame::wait_for_frame_counter(engine, r);

    engine.state.set_sound_channel_byte(1, 0x00, 0x00);
    engine.state.set_sound_channel_flags(0x00);
    engine.state.set_sound_channel_byte(1, 0x20, 0x00);
    engine.state.set_sound_channel_byte(1, 0x30, 0x00);
    engine.state.set_prompt_state(0x18);

    let mut cnt = 0x0a;
    loop {
        for x in (0..=0x1f).rev() {
            engine.state.set_palette_buffer(x, 0x30);
        }
        crate::game::upload_palette_buffer(engine, r);
        engine.state.set_frame_counter(0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::load_intro_text_palette(engine, r);
        crate::game::upload_palette_buffer(engine, r);
        engine.state.set_frame_counter(0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        cnt = u8v(cnt - 1);
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
        engine.state.set_chr_bank(2, 0x37);
        engine.state.set_statusbar_split_flag(0x00);
        engine.state.set_ppu_ctrl_shadow(0xa0);
        engine.device_write(crate::engine::reg::PPU_CTRL, 0xa0);
        engine.state.set_ppu_mask_shadow(0x00);
        engine.device_write(crate::engine::reg::PPU_MASK, 0x00);
        engine.state.set_scroll_pixel_x(0x00);
        engine.state.set_nametable_select(0x00);
        engine.state.set_scroll_y(0xe8);
        for x in (0..=0x1f).rev() {
            engine.state.set_palette_buffer(x, 0x0f);
        }
        farcall_cce4(engine, r, 0x69, 0xc5, crate::game::upload_palette_buffer);
        crate::game::reset_room_object_slots(engine, r);
        crate::game::clear_oam_with_sprite_zero_template(engine, r);
        crate::game::load_title_oam_template(engine, r);
        engine.state.set_chr_bank(2, 0x15);
        engine.state.set_song(0x09);
        crate::game::song_init(engine, r);
        crate::game::upload_title_screen_nametables(engine, r);
        engine.state.set_ppu_mask_shadow(0x1e);
        engine.device_write(crate::engine::reg::PPU_MASK, 0x1e);
        engine.state.set_frame_counter(0x78);
        frame::wait_for_frame_counter(engine, r);
        fade_title_palette_in(engine, r);
        engine.state.set_countdown_timer(0x14);

        loop {
            engine.state.set_frame_counter(0x01);
            let pad = frame::read_buttons(engine, r);
            if pad == 0xff {
                engine.state.set_prompt_state(0x1a);
                engine.state.set_continue_timer(0x1a);
            }
            if (engine.state.buttons() & 0x10) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }
            if engine.state.button_chord() == 0x83 {
                run_story_text_sequence(engine, r);
                return;
            }
            if (engine.state.frame_prescaler() & 0x07) == 0 {
                let lo = engine.state.palette_buffer(0x02) & 0x0f;
                let mut hi = engine.state.palette_buffer(0x02) & 0xf0;
                engine.state.set_scratch0(lo);
                if hi < 0x10 {
                    hi = 0x30;
                } else {
                    hi = u8v(hi - 0x10);
                }
                engine.state.set_palette_buffer(0x13, hi);
                engine
                    .state
                    .set_palette_buffer(0x02, hi | engine.state.scratch0());
            }
            enter_return_home(engine, 0x35, 0xc1);
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
        r.value = 0x04;
        crate::game::rng_update(engine, r);
        engine.state.set_map_screen_x(r.value);
        r.value = 0x10;
        crate::game::rng_update(engine, r);
        engine.state.set_map_screen_y(r.value);
        farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);

        loop {
            r.value = 0x40;
            crate::game::rng_update(engine, r);
            engine.state.set_player_x_tile(r.value);
            engine.state.set_data_ptr_lo(r.value);
            engine.state.set_player_x_fine(0x00);
            r.value = 0x0b;
            crate::game::rng_update(engine, r);
            r.value = u8v(r.value << 4);
            engine.state.set_player_y(r.value);
            engine.state.set_data_ptr_hi(r.value);
            crate::game::resolve_room_tile_pointer(engine, r);
            let p = u16v(engine.state.data_ptr());
            let mut t = engine.state.byte(p) & 0x3f;
            if t >= 0x30 {
                continue;
            }
            if t == 0x02 {
                continue;
            }
            if t == engine.state.text_attr_ptr_lo() {
                continue;
            }
            t = engine.state.byte(u16v(p + 1)) & 0x3f;
            if t < 0x30 {
                continue;
            }
            if t == 0x30 {
                continue;
            }
            break;
        }

        let mut x = engine.state.player_x_tile();
        if x < 0x08 {
            x = 0x00;
        } else {
            x = u8v(x - 0x08);
        }
        if x >= 0x30 {
            x = 0x30;
        }
        engine.state.set_scroll_tile_x(x);
        engine.state.set_scroll_fine_x(0x00);

        let chr = loop {
            r.value = 0x05;
            crate::game::rng_update(engine, r);
            let chr = r.value;
            let mut a = 0x00;
            let mut c = 1;
            for _ in 0..=chr {
                let nc = (a >> 7) & 1;
                a = u8v((a << 1) | c);
                c = nc;
            }
            let mask = a;
            if (mask & engine.state.family_member_mask()) != 0 {
                break chr;
            }
        };
        engine
            .state
            .set_item_slot(0, engine.state.byte(u16v(START_ITEM_TABLE + chr)));
        engine.state.set_selected_item_slot(0x00);
        engine.state.set_character_index(chr);
        let mut y = u16v(0xffa7 + ((chr << 2) + 0x03));
        for i in (0..=3).rev() {
            engine.state.set_item_slot(0x0B + i, engine.state.byte(y));
            y = u16v(y - 1);
        }
        engine
            .state
            .set_chr_bank(2, u8v(engine.state.character_index() + 0x38));
        engine.state.set_chr_bank(4, 0x3e);
        engine.state.set_chr_bank(5, 0x20);
        engine.state.set_player_pose(0x0d);
        engine.state.set_player_facing(0x00);
        engine.state.set_title_timer(0x01);
        engine.state.set_player_health(0x64);
        engine.state.set_player_magic(0x64);
        farcall_cce4(
            engine,
            r,
            0x8b,
            0xc3,
            crate::game::clear_name_tables_to_blank_tiles,
        );
        crate::game::upload_status_panel_template(engine, r);
        farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::upload_current_room_view);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        crate::game::sync_coin_hud(engine, r);
        crate::game::sync_key_hud(engine, r);
        crate::game::refresh_scroll_register_shadows(engine, r);
        crate::game::clear_gameplay_object_sprites(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_status_item_sprites(engine, r);
        farcall_cce4(engine, r, 0x92, 0xc4, fade_room_palette_in);
        engine.state.set_countdown_timer(0x0a);

        loop {
            engine.state.set_frame_counter(0x01);
            engine
                .state
                .set_saved_scroll_tile(engine.state.scroll_tile_x());
            crate::game::blink_demo_oam_sprites(engine, r);
            frame::read_buttons(engine, r);
            if (engine.state.buttons() & 0x10) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }

            engine
                .state
                .set_buttons(engine.state.room_restore_scratch());
            let mut do_b044 = true;
            if (engine.state.horizontal_subtile_delta() | engine.state.vertical_delta()) != 0 {
                engine
                    .state
                    .set_title_timer((engine.state.title_timer() - 1) & 0xFF);
                if engine.state.title_timer() != 0 {
                    do_b044 = false;
                }
            }
            if do_b044 {
                engine.state.set_title_timer(0x80);
                crate::game::choose_random_demo_input(engine, r);
                engine
                    .state
                    .set_room_restore_scratch(engine.state.buttons());
            }

            farcall_cce4(engine, r, 0x2b, 0xd4, crate::game::game_update);
            farcall_cce4(
                engine,
                r,
                0x28,
                0xf6,
                crate::game::update_player_projectiles,
            );
            farcall_cce4(engine, r, 0x7c, 0xe8, crate::game::update_room_actors);
            farcall_cce4(engine, r, 0x82, 0xf7, crate::game::update_tile_projectile);
            farcall_cce4(
                engine,
                r,
                0x5d,
                0xc1,
                crate::game::update_camera_scroll_from_player,
            );
            crate::game::draw_player_sprites(engine, r);
            crate::game::draw_room_object_sprites(engine, r);
            if engine.state.saved_scroll_tile() != engine.state.scroll_tile_x() {
                engine
                    .state
                    .set_main_loop_phase((engine.state.main_loop_phase() + 1) & 0xFF);
            }
            enter_return_home(engine, 0x35, 0xc1);
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
    engine.state.set_sound_channel_byte(1, 0x20, 0);
    engine.state.set_data_ptr_hi(0x10);
    loop {
        if engine.state.sound_channel_byte(13, 0x00) != 0 {
            engine.state.dec_sound_channel_byte(13, 0x00);
        }
        if engine.state.sound_channel_byte(13, 0x10) != 0 {
            engine.state.dec_sound_channel_byte(13, 0x10);
        }
        if engine.state.sound_channel_byte(13, 0x30) != 0 {
            engine.state.dec_sound_channel_byte(13, 0x30);
        }
        engine.state.set_data_ptr_lo(0x14);
        loop {
            crate::game::draw_room_object_sprites(engine, r);
            engine.state.set_frame_counter(0x01);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            engine
                .state
                .set_data_ptr_lo((engine.state.data_ptr_lo() - 1) & 0xFF);
            if engine.state.data_ptr_lo() == 0 {
                break;
            }
        }
        engine
            .state
            .set_data_ptr_hi((engine.state.data_ptr_hi() - 1) & 0xFF);
        if engine.state.data_ptr_hi() == 0 {
            break;
        }
    }
}

/// Runs the player death animation, extra-life recovery, and game-over/continue
/// screen. `r.index` returns `0` for immediate resume; nonzero values are
/// decremented by the caller before re-entering `main_init`.
pub fn run_player_death_or_continue_flow(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_song = engine.state.song();

    engine
        .state
        .set_sound_paused((engine.state.sound_paused() + 1) & 0xFF);
    crate::game::clear_gameplay_object_sprites(engine, r);
    r.index = 0x35;
    r.offset = 0x00;
    show_player_pose_for_eight_frames(engine, r);

    engine.state.set_frame_counter(0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    r.value = 0x08;
    crate::game::switch_song_if_needed(engine, r);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() - 1) & 0xFF);

    engine.state.set_scratch2(0x05);
    loop {
        r.index = 0x0d;
        r.offset = 0x00;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 0x01;
        r.offset = 0x00;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 0x09;
        r.offset = 0x00;
        show_player_pose_for_eight_frames(engine, r);
        r.index = 0x01;
        r.offset = 0x40;
        show_player_pose_for_eight_frames(engine, r);
        engine
            .state
            .set_scratch2((engine.state.scratch2() - 1) & 0xFF);
        if engine.state.scratch2() == 0 {
            break;
        }
    }

    engine.state.set_frame_counter(0x01);
    engine.state.set_player_pose(0x31);
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    let mut use_game_over_screen = engine.state.final_exit_flag() != 0;
    if !use_game_over_screen {
        if (engine.state.continue_timer() & 0x80) != 0 {
            let x = engine.state.selected_item_slot();
            if engine.state.item_slot(x) == 0x0c {
                engine.state.set_item_slot(x, 0xff);
                crate::game::draw_status_item_sprites(engine, r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            engine
                .state
                .set_continue_timer((engine.state.continue_timer() + 1) & 0xFF);
        }

        if !use_game_over_screen {
            animate_health_refill_to_cap(engine, r);
            engine.state.set_player_pose(0x19);
            crate::game::read_debounced_buttons(engine, r);
            r.value = saved_song;
            crate::game::switch_song_if_needed(engine, r);
            r.index = 0x00;
            return;
        }
    }

    fade_palette_buffer_out(engine, r);
    engine.state.set_final_exit_flag(0x00);
    engine.state.set_sprite_index(0x00);
    engine.state.set_oam_cursor(0x80);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.state.set_chr_bank(1, 0x16);
    engine.state.set_chr_bank(2, 0x36);
    engine.state.set_scroll_pixel_x(0x00);
    engine.state.set_nametable_select(0x00);
    engine.state.set_scroll_y(0x00);
    engine.state.set_scroll_fine_x(0x00);
    engine.state.set_scroll_tile_x(0x00);

    vram_blit(engine, r, 0x6b, 0x21, 0xaf, 0xb4, 0x09);
    vram_blit(engine, r, 0x4c, 0x22, 0xb8, 0xb4, 0x05);
    vram_blit(engine, r, 0x8c, 0x22, 0xbd, 0xb4, 0x08);

    engine.state.set_player_x_tile(0x05);
    engine.state.set_player_x_fine(0x00);
    engine.state.set_player_y(0x70);
    engine.state.set_player_pose(0x39);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    crate::game::draw_player_sprites(engine, r);
    farcall_cce4(engine, r, 0xe0, 0xc4, fade_two_room_palette_rows_in);

    loop {
        crate::game::read_debounced_buttons(engine, r);
        if (r.value & 0x10) != 0 {
            break;
        }
        engine.state.set_player_y(engine.state.player_y() ^ 0x10);
        engine.state.set_prompt_state(0x0c);
    }

    engine.state.set_prompt_state(0x18);
    if engine.state.player_y() != 0x70 {
        fade_palette_buffer_out(engine, r);
        engine.state.set_frame_counter(0x78);
        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);
        r.index = 0x02;
        return;
    }

    crate::game::restore_inventory_state_snapshot(engine, r);
    engine.state.set_item_slot(0, 0xff);
    engine.state.set_item_slot(1, 0xff);
    engine.state.set_item_slot(2, 0xff);
    engine.state.set_selected_item_slot(0x03);
    engine.state.set_character_index(0x06);
    engine.state.set_map_screen_x(0x03);
    engine.state.set_map_screen_y(0x10);
    fade_palette_buffer_out(engine, r);
    engine.state.set_song(0x02);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::upload_status_panel_template(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);

    r.value = 0x0f;
    for x in (0..=0x1f).rev() {
        engine.state.set_palette_buffer(x, 0x0f);
    }
    engine.state.set_oam_y(0x10, 0xef);
    engine.state.set_oam_y(0x14, 0xef);
    farcall_cce4(engine, r, 0xb4, 0xc4, fade_room_palette_row_in);
    r.index = 0x01;
}

/// Shows the player sprite pose in `r.index`/`r.offset` for eight foreground
/// frames.
pub fn show_player_pose_for_eight_frames(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_player_pose(r.index);
    engine.state.set_player_facing(r.offset);
    engine.state.set_frame_counter(0x08);
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Fades the title-screen palette from black to its ROM palette in five steps.
pub fn fade_title_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_scratch1(0x40);
    loop {
        engine.state.set_frame_counter(0x05);
        crate::game::load_title_palette_buffer(engine, r);
        r.index = 0x00;
        r.offset = 0x20;
        crate::game::dim_palette_range_by_step(engine, r);

        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);

        engine
            .state
            .set_scratch1((engine.state.scratch1() - 0x10) & 0xFF);
        if (engine.state.scratch1() & 0x80) != 0 {
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
    let mut y = 0x04;
    loop {
        engine.state.set_frame_counter(0x05);
        for x in (0..=0x20).rev() {
            let v = engine.state.palette_buffer(x);
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.state.set_scratch0(lo);
            engine.state.set_palette_buffer(
                x,
                if hi >= 0x10 {
                    u8v((hi - 0x10) | lo)
                } else {
                    0x0f
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
    let ptr = u16v(engine.state.palette_src_ptr());
    let mut v = 0x40;
    engine.state.set_scratch1(v);
    loop {
        engine.state.set_frame_counter(0x05);
        for y in 0xe0..0xe4 {
            engine
                .state
                .set_inventory_item(0x40 + y, engine.state.byte(u16v(ptr + y)));
        }
        r.index = 0x00;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.state.scratch1() - 0x10);
        engine.state.set_scratch1(v);
        if (v & 0x80) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
}

/// Fades in the first two room palette rows from the active room data pointer.
pub fn fade_two_room_palette_rows_in(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = u16v(engine.state.palette_src_ptr());
    let mut v = 0x40;
    engine.state.set_scratch1(v);
    loop {
        engine.state.set_frame_counter(0x05);
        for y in 0xe0..0xe4 {
            engine
                .state
                .set_inventory_item(0x40 + y, engine.state.byte(u16v(ptr + y)));
        }
        for y in 0xf0..0xf4 {
            engine
                .state
                .set_inventory_item(0x40 + y, engine.state.byte(u16v(ptr + y)));
        }
        r.index = 0x00;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        r.index = 0x10;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.state.scratch1() - 0x10);
        engine.state.set_scratch1(v);
        if (v & 0x80) != 0 {
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
    r.value = buttons;
    engine.state.set_buttons(buttons);
}

/// Scans live object slots for a damageable actor overlapping the projected
/// position in `0x0E/0x0F/0x0A`. On hit, `0x08` receives the logical slot and
/// `0x09` receives the object-slot byte offset.
pub fn find_damageable_actor_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 0x09;
    let mut x = 0x90;
    loop {
        let mut skip = u8v(y) == engine.state.slot_index();
        if !skip && (engine.state.object_state(x) & 0x80) != 0 {
            skip = true;
        }
        if !skip && engine.state.object_state(x) != 0x01 && engine.state.object_state(x) < 0x1a {
            skip = true;
        }
        if !skip && (engine.state.object_tile(x) & 0xf9) == 0xe1 {
            skip = true;
        }
        if !skip && (engine.state.object_attr(x) & 0x20) != 0 {
            skip = true;
        }
        if !skip {
            let mut d = u8v(engine.state.scratch2() - engine.state.object_y_pixel(x));
            if !(d < 0x10) && d < 0xf1 {
                skip = true;
            }
            if !skip {
                d = u8v(engine.state.indirect_ptr_hi() - engine.state.object_x_tile(x));
                if d == 0 {
                    engine.state.set_scratch0(u8v(y));
                    engine.state.set_scratch1(x);
                    r.carry = 1;
                    return;
                }
                if d < 0x02 {
                    d = u8v(engine.state.indirect_ptr_lo() - engine.state.object_x_sub(x));
                    if (d & 0x80) != 0 {
                        engine.state.set_scratch0(u8v(y));
                        engine.state.set_scratch1(x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 0xff {
                    skip = true;
                } else {
                    d = u8v(engine.state.indirect_ptr_lo() - engine.state.object_x_sub(x));
                    if d != 0 && (d & 0x80) == 0 {
                        engine.state.set_scratch0(u8v(y));
                        engine.state.set_scratch1(x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                }
            }
        }
        let _ = skip;
        x = u8v(x - 0x10);
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
    let mut y = 0x0a;
    let mut x = 0xa0;
    loop {
        let mut skip = u8v(y) == engine.state.slot_index();
        if !skip && engine.state.object_state(x) == 0 {
            skip = true;
        }
        if !skip && (engine.state.object_state(x) & 0x80) != 0 {
            skip = true;
        }
        if !skip && (engine.state.object_tile(x) & 0xf9) == 0xe1 {
            skip = true;
        }
        if !skip && (engine.state.object_attr(x) & 0x20) != 0 {
            skip = true;
        }
        if !skip {
            let mut d = u8v(engine.state.scratch2() - engine.state.object_y_pixel(x));
            if !(d < 0x10) && d < 0xf1 {
                skip = true;
            }
            if !skip {
                d = u8v(engine.state.indirect_ptr_hi() - engine.state.object_x_tile(x));
                if d == 0 {
                    engine.state.set_scratch0(u8v(y));
                    engine.state.set_scratch1(x);
                    r.carry = 1;
                    return;
                }
                if d < 0x02 {
                    d = u8v(engine.state.indirect_ptr_lo() - engine.state.object_x_sub(x));
                    if (d & 0x80) != 0 {
                        engine.state.set_scratch0(u8v(y));
                        engine.state.set_scratch1(x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 0xff {
                    skip = true;
                } else {
                    d = u8v(engine.state.indirect_ptr_lo() - engine.state.object_x_sub(x));
                    if d != 0 && (d & 0x80) == 0 {
                        engine.state.set_scratch0(u8v(y));
                        engine.state.set_scratch1(x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                }
            }
        }
        let _ = skip;
        x = u8v(x - 0x10);
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
    if engine.state.airborne_flag() != 0 || engine.state.jump_timer() != 0 {
        engine.state.set_pose_state(0x00);
        engine.state.set_fall_frames(0x00);
        return;
    }

    engine.state.set_data_ptr_lo(engine.state.player_x_tile());
    engine
        .state
        .set_indirect_ptr_hi(engine.state.player_x_tile());
    engine
        .state
        .set_indirect_ptr_lo(engine.state.player_x_fine());
    engine.state.set_data_ptr_hi(engine.state.player_y());
    engine.state.set_scratch2(u8v(engine.state.player_y() + 1));
    crate::game::resolve_room_tile_pointer(engine, r);

    if engine.state.player_x_fine() == 0 {
        engine.state.set_pose_state(0x01);
        r.offset = 0x00;
        let tile_ptr = u16v(engine.state.data_ptr());
        if (engine.state.byte(u16v(tile_ptr + r.offset)) & 0x3f) == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
    }

    engine.state.set_pose_state(0x00);
    if engine.state.player_y() >= 0xb0 {
        engine
            .state
            .set_fall_frames((engine.state.fall_frames() + 1) & 0xFF);
        return;
    }

    find_damageable_actor_overlap(engine, r);
    if ((r.carry) != 0) {
        if engine.state.chr_bank(3) >= 0x30 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let selected_slot = engine.state.selected_item_slot();
        let selected_item = engine.state.item_slot(selected_slot);
        if selected_item != 0x05 || engine.state.fall_frames() == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let hit_slot = engine.state.scratch1();
        engine.state.set_object_state(hit_slot, 0x80);
    }

    r.offset = 0x01;
    crate::game::probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    if engine.state.player_x_fine() == 0 {
        engine
            .state
            .set_fall_frames((engine.state.fall_frames() + 1) & 0xFF);
        return;
    }

    r.offset = 0x0d;
    crate::game::probe_player_solid_tile(engine, r);
    if ((r.carry) != 0) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    engine
        .state
        .set_fall_frames((engine.state.fall_frames() + 1) & 0xFF);
}

/// Converts a just-detected floor/object/hazard contact into damage, recoil,
/// hazard invulnerability, or a reset of the fall counter.
fn resolve_player_landing_or_hazard_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let mut fall_frames = engine.state.fall_frames();
    if fall_frames >= engine.state.jump_strength() {
        fall_frames = u8v(fall_frames - 0x07);
        if fall_frames >= engine.state.jump_strength() {
            fall_frames = engine.state.jump_strength();
        }
        fall_frames = u8v(fall_frames - 0x01);
        engine.state.set_jump_timer(fall_frames);
        engine.state.set_landing_timer(u8v(fall_frames + 0x0a));
        engine.state.set_prompt_state(0x0a);
        crate::game::consume_health_point(engine, r);
    }
    if engine.state.fall_frames() == 0 {
        r.offset = 0x01;
        crate::game::apply_hazard_tile_contact(engine, r);
        if !((r.carry) != 0) && engine.state.player_x_fine() != 0 {
            r.offset = 0x0d;
            crate::game::apply_hazard_tile_contact(engine, r);
        }
    }
    engine.state.set_fall_frames(0x00);
}

/// Handles the room tile sampled at the current projected player footprint.
/// Special tiles can spend keys/magic, spawn transient objects, or launch the
/// tile-removal projectile; ordinary tiles return carry for solid terrain.
pub fn dispatch_room_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = u16v(engine.state.data_ptr());
    let tile_offset = r.offset;
    let tile = engine.state.byte(u16v(tile_ptr + tile_offset)) & 0x3f;
    if tile == engine.state.text_attr_ptr_lo() {
        if engine.state.object_state(0x90) == 0 {
            engine.state.set_scratch3(tile_offset);
            engine.state.set_obj_tile(0xe1);
            engine.state.set_obj_state(0x01);
            engine.state.set_obj_attr(0x01);
            engine
                .state
                .set_obj_move_scratch(engine.state.text_attr_ptr_hi());
            engine.state.set_obj_timer(0x0a);
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.state.set_prompt_state(0x06);
        }
        let v = engine.state.text_attr_ptr_hi() & 0x3f;
        r.value = v;
        r.carry = u8v(v >= 0x30);
        return;
    }
    if tile == 0x02 {
        if engine.state.object_state(0x90) == 0 {
            engine.state.set_scratch3(tile_offset);
            r.index = engine.state.selected_item_slot();
            let item = engine.state.item_slot(r.index);
            r.value = item;
            if item == 0x07 {
                r.index = engine.state.selected_item_slot();
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
            engine.state.set_obj_tile(0xe1);
            engine.state.set_obj_state(0x01);
            engine.state.set_obj_attr(0x01);
            engine
                .state
                .set_obj_move_scratch(engine.state.room_tile_action());
            engine.state.set_obj_timer(0x0f);
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.state.set_prompt_state(0x06);
        }
        r.carry = 1;
        return;
    }
    if tile == 0x3e {
        if (engine.state.buttons() & 0x80) != 0 && engine.state.object_state(0x90) == 0 {
            engine.state.set_scratch3(tile_offset);
            engine.state.set_obj_move_state(0x01);
            r.offset = engine.state.selected_item_slot();
            r.index = engine.state.item_slot(r.offset);
            let idx = r.index;
            if idx == 1 {
                if engine.state.player_magic() != 0 {
                    let mut t = engine.state.player_y() & 0x0f;
                    t |= engine.state.player_x_fine();
                    if t == 0 {
                        let x2 = u8v((engine.state.direction_latch() & 0x0f) << 1);
                        let lo = u8v(engine.state.player_x_tile()
                            + engine.state.byte(u16v(SPAWN_OFFSET_X_TABLE + x2)));
                        engine.state.set_object_x_tile(0x90, lo);
                        engine.state.set_data_ptr_lo(lo);
                        engine.state.set_object_x_sub(0x90, 0x00);
                        let hi = u8v(engine.state.player_y()
                            + engine.state.byte(u16v(SPAWN_OFFSET_Y_TABLE + x2)));
                        engine.state.set_object_y_pixel(0x90, hi);
                        engine.state.set_data_ptr_hi(hi);
                        crate::game::resolve_room_tile_pointer(engine, r);
                        r.offset = 0x00;
                        engine.state.set_scratch3(0x00);
                        let p = u16v(engine.state.data_ptr());
                        let b = engine.state.byte(p) & 0x3f;
                        if b == 0x3e {
                            engine.state.set_object_tile(0x90, 0xe1);
                            engine.state.set_object_state(0x90, 0x01);
                            engine.state.set_object_attr(0x90, 0x01);
                            engine.state.set_object_timer(0x90, 0x0f);
                            crate::game::read_room_tile_action_value(engine, r);
                            engine.state.set_object_move_scratch(0x90, r.value);
                            crate::game::consume_magic_point(engine, r);
                            engine.state.set_prompt_state(0x14);
                        }
                    }
                }
                r.carry = 1;
                return;
            }
            if idx == 2 {
                if (engine.state.direction_latch() & 0x0f) != 0 {
                    r.offset = 0x01;
                    crate::game::build_direction_velocity(engine, r);
                    r.offset = 0xf8;
                    let p79 = u16v(engine.state.tile_table_ptr());
                    engine
                        .state
                        .set_obj_tile(engine.state.byte(u16v(p79 + 0xf8)) & 0xfe);
                    engine.state.set_obj_state(0x01);
                    engine.state.set_obj_attr(0x03);
                    r.offset = engine.state.scratch3();
                    let b = engine.state.byte(u16v(tile_ptr + r.offset));
                    engine.state.set_obj_move_scratch(b);
                    engine.state.set_obj_timer(0x10);
                    crate::game::read_room_tile_action_value(engine, r);
                    engine.state.set_byte(u16v(tile_ptr + r.offset), r.value);
                    crate::game::seed_object_position_from_tile_offset(engine, r);
                    crate::game::redraw_room_tile_column(engine, r);
                    crate::game::update_tile_projectile_motion(engine, r);
                    engine.state.set_slot_index(0xff);
                    if engine.state.object_state(0x90) != 0 {
                        engine.state.set_prompt_state(0x06);
                    }
                }
                engine.state.set_vertical_delta(0x00);
                engine.state.set_fall_frames(0x00);
                r.carry = 1;
                return;
            }
            if idx == 3 {
                if engine.state.player_magic() != 0 {
                    if (engine.state.direction_latch() & 0x0f) != 0 {
                        r.offset = 0x08;
                        crate::game::build_direction_velocity(engine, r);
                        r.offset = 0xf8;
                        let p79 = u16v(engine.state.tile_table_ptr());
                        engine
                            .state
                            .set_obj_tile(engine.state.byte(u16v(p79 + 0xf8)) & 0xfe);
                        engine.state.set_obj_state(0x01);
                        engine.state.set_obj_attr(0x03);
                        r.offset = engine.state.scratch3();
                        let b = engine.state.byte(u16v(tile_ptr + r.offset));
                        engine.state.set_obj_move_scratch(b);
                        engine.state.set_obj_timer(0x00);
                        crate::game::read_room_tile_action_value(engine, r);
                        engine.state.set_byte(u16v(tile_ptr + r.offset), r.value);
                        crate::game::seed_object_position_from_tile_offset(engine, r);
                        crate::game::redraw_room_tile_column(engine, r);
                        crate::game::update_tile_projectile_motion(engine, r);
                        engine.state.set_slot_index(0xff);
                        if engine.state.obj_state() != 0 {
                            engine.state.set_prompt_state(0x14);
                            crate::game::consume_magic_point(engine, r);
                        }
                        engine.state.set_vertical_delta(0x00);
                        engine.state.set_fall_frames(0x00);
                        r.carry = 1;
                        return;
                    }
                    engine.state.set_vertical_delta(0x00);
                    engine.state.set_fall_frames(0x00);
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
    r.carry = u8v(tile >= 0x30);
}

/// Fades the room palette out and resets active audio channel state.
pub fn fade_room_palette_out_reset_audio(engine: &mut Engine, r: &mut RoutineContext) {
    engine
        .state
        .set_music_volume_override((engine.state.music_volume_override() + 1) & 0xFF);
    let mut y = 0x04;
    loop {
        engine.state.set_frame_counter(0x05);
        for x in (0..=0x1c).rev() {
            let v = engine.state.palette_buffer(0x04 + x);
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.state.set_scratch0(lo);
            engine.state.set_vram_stage(
                0x44 + x,
                if hi >= 0x10 {
                    u8v((hi - 0x10) | lo)
                } else {
                    0x0f
                },
            );
        }
        engine.state.set_sound_channel_byte(
            0x0D,
            0x00,
            engine.state.sound_channel_byte(0x0D, 0x00) >> 1,
        );
        engine.state.set_sound_channel_byte(
            0x0D,
            0x10,
            engine.state.sound_channel_byte(0x0D, 0x10) >> 1,
        );
        engine.state.set_sound_channel_byte(
            0x0D,
            0x30,
            engine.state.sound_channel_byte(0x0D, 0x30) >> 1,
        );
        engine.state.set_sound_channel_byte(1, 0x20, 0);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
    engine.state.set_song(0xff);
    engine.state.set_sound_channel_byte(1, 0x00, 0);
    engine.state.set_sound_channel_flags(0);
    engine.state.set_sound_channel_byte(1, 0x30, 0);
    engine.state.set_music_volume_override(0);
}

/// Fades the room palette out while preserving active audio channel state.
pub fn fade_room_palette_out_keep_audio(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 0x04;
    loop {
        engine.state.set_frame_counter(0x05);
        for x in (0..=0x1c).rev() {
            let v = engine.state.palette_buffer(0x04 + x);
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.state.set_scratch0(lo);
            engine.state.set_vram_stage(
                0x44 + x,
                if hi >= 0x10 {
                    u8v((hi - 0x10) | lo)
                } else {
                    0x0f
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
    let mut v = 0x40;
    engine.state.set_scratch1(v);
    loop {
        engine.state.set_frame_counter(0x05);
        crate::game::build_room_palette_buffer(engine, r);
        r.index = 0x04;
        r.offset = 0x1c;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.state.scratch1() - 0x10);
        engine.state.set_scratch1(v);
        if (v & 0x80) != 0 {
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
        for i in (0..=0x1f).rev() {
            engine.state.set_palette_buffer(i, 0x30);
        }
        crate::game::upload_palette_buffer(engine, r);
        engine.state.set_frame_counter(0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::build_room_palette_buffer(engine, r);
        crate::game::upload_palette_buffer(engine, r);
        engine.state.set_frame_counter(0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        x = u8v(x - 1);
        if x == 0 {
            break;
        }
    }
    r.index = x;
}

pub fn animate_health_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count health up one point at a time so the HUD and prompt animation match
    // the original refill reward pacing.
    let saved_blink = engine.state.sprite_blink_timer();
    engine.state.set_sprite_blink_timer(0x00);
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine
            .state
            .set_player_health((engine.state.player_health() + 1) & 0xFF);
        crate::game::sync_health_hud(engine, r);
        engine.state.set_prompt_state(0x16);
        engine.state.set_frame_counter(0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_health();
        if engine.state.player_health() >= 0x63 {
            break;
        }
    }
    engine.state.set_prompt_state(0x17);
    engine.state.set_frame_counter(0x00);
    frame::commit_frame_work(engine, r);
    engine.state.set_sprite_blink_timer(saved_blink);
}

pub fn animate_magic_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count magic up one point at a time, sharing the same prompt/blink pacing
    // as the health refill.
    let saved_blink = engine.state.sprite_blink_timer();
    engine.state.set_sprite_blink_timer(0x00);
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine
            .state
            .set_player_magic((engine.state.player_magic() + 1) & 0xFF);
        crate::game::sync_magic_hud(engine, r);
        engine.state.set_prompt_state(0x16);
        engine.state.set_frame_counter(0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.state.player_magic();
        if engine.state.player_magic() >= 0x63 {
            break;
        }
    }
    engine.state.set_prompt_state(0x17);
    engine.state.set_frame_counter(0x00);
    frame::commit_frame_work(engine, r);
    engine.state.set_sprite_blink_timer(saved_blink);
}

/// Spends a key and runs the door-unlock prompt/music sequence. Carry is set
/// only when a key was available and the door event completed.
pub fn unlock_door_with_key(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::consume_key(engine, r);
    if ((r.carry) != 0) {
        engine.state.set_prompt_state(0x06);
        r.carry = 0;
        return;
    }

    let ptr = u16v(engine.state.palette_src_ptr());
    let door = engine.state.byte(u16v(ptr + 0x0a));
    if door < 0x08 {
        engine.state.set_object_attr(0xA0, 0x00);
    }
    engine.state.set_object_state(0xA0, u8v(door + 0x02));
    engine
        .state
        .set_object_tile(0xA0, u8v(((door << 2) & 0xff) + 0x81));
    engine.state.set_prompt_state(0x1f);
    crate::game::draw_room_object_sprites(engine, r);

    let saved_blink = engine.state.sprite_blink_timer();
    engine.state.set_sprite_blink_timer(0);
    crate::game::draw_player_sprites(engine, r);

    let saved_song = engine.state.song();
    engine.state.set_song(0x0e);
    r.value = 0x0e;
    crate::game::song_init(engine, r);

    engine.state.set_frame_counter(0x78);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.state.set_song(saved_song);
    r.value = saved_song;
    crate::game::song_init(engine, r);

    engine.state.set_sprite_blink_timer(saved_blink);
    r.carry = 1;
}

/// Opens the in-game character-select overlay, waits for a press/release of the
/// character-select button, then restores the gameplay room.
pub fn run_character_select_overlay(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_prompt_state(0x03);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() + 1) & 0xFF);

    if engine.state.chr_bank(3) < 0x30 {
        push_room_checkpoint(engine, r);
        r.value = 0x08;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.state.set_scroll_fine_x(0x08);
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
        if (frame::read_buttons(engine, r) & 0x10) != 0 {
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

    engine.state.set_prompt_state(0x04);

    if engine.state.chr_bank(3) < 0x30 {
        pop_room_checkpoint(engine, r);
        fade_room_palette_out_reset_audio(engine, r);
        crate::game::clear_temporary_room_sprites(engine, r);
        r.value = engine.state.room_restore_scratch();
        crate::game::switch_song_if_needed(engine, r);
        crate::game::prepare_room_metadata_and_palette(engine, r);
        crate::game::upload_current_room_view(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        crate::game::refresh_scroll_register_shadows(engine, r);
        fade_room_palette_in(engine, r);
    }

    engine
        .state
        .set_sound_paused((engine.state.sound_paused() - 1) & 0xFF);
}

/// Shows the read-only inventory item-list page until the player presses a
/// button, then returns to the character-selection room page.
pub fn show_inventory_item_list_screen(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_scroll_tile_x(0x10);
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);

    engine.state.set_indirect_ptr_lo(0xd4);
    engine.state.set_indirect_ptr_hi(0xb4);
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

    engine.state.set_scroll_tile_x(0x20);
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);
}

/// Runs the interactive inventory item-grid editor from the character-selection
/// room.
pub fn run_inventory_item_grid_menu(engine: &mut Engine, r: &mut RoutineContext) {
    engine.state.set_scroll_tile_x(0x30);
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

    engine.state.set_obj_x_sub(0);
    engine.state.set_obj_x_vel_lo(0);
    engine.state.set_obj_y_vel(0);
    engine.state.set_oam_tile(0x80, 0xf5);
    engine.state.set_oam_tile(0x90, 0xf5);
    engine.state.set_oam_tile(0x84, 0xf7);
    engine.state.set_oam_tile(0x94, 0xf7);
    engine.state.set_oam_attr(0x80, 0x00);
    engine.state.set_oam_attr(0x84, 0x00);
    engine.state.set_oam_attr(0x90, 0x00);
    engine.state.set_oam_attr(0x94, 0x00);
    crate::game::update_inventory_list_cursor_sprites(engine, r);
    crate::game::update_inventory_grid_cursor_sprites(engine, r);

    loop {
        engine.state.set_frame_counter(0x01);
        let b = frame::read_buttons(engine, r);
        r.value = b;

        if (b & 0x80) != 0 {
            crate::game::select_inventory_grid_entry(engine, r);
            crate::game::upload_inventory_item_list(engine, r);
        } else if (b & 0x40) != 0 {
        } else if (b & 0x01) != 0 {
            crate::game::move_inventory_cursor_right(engine, r);
        } else if (b & 0x02) != 0 {
            crate::game::move_inventory_cursor_left(engine, r);
        } else if (b & 0x04) != 0 {
            crate::game::move_inventory_cursor_down(engine, r);
        } else if (b & 0x08) != 0 {
            crate::game::move_inventory_cursor_up(engine, r);
            crate::game::upload_inventory_item_list(engine, r);
        } else if (b & 0x10) != 0 {
            crate::game::close_inventory_item_menu(engine, r);
        } else if (b & 0x20) != 0 {
            engine.state.set_scroll_tile_x(0x20);
            crate::game::upload_staged_room_columns(engine, r);
            crate::game::refresh_scroll_register_shadows(engine, r);
            crate::game::restore_status_sprite_template(engine, r);
            return;
        }

        if (engine.state.buttons() & 0xcf) != 0 {
            engine.state.set_prompt_state(0x0c);
            engine.state.set_frame_counter(0x0a);
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special room flow used to refill resources, return carried items,
/// pick a family member, and optionally visit the inventory item pages.
pub fn run_character_select_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.map_screen_y() != 0x10 {
        push_room_checkpoint(engine, r);
        r.value = 0x04;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_coin_cost_sprites(engine, r);
        fade_room_palette_in(engine, r);

        loop {
            walk_purchase_room_until_action_or_exit(engine, r);
            if ((r.carry) != 0) {
                crate::game::restore_room_from_checkpoint(engine, r);
                return;
            }
            if engine.state.coins() < 0x0a {
                engine.state.set_prompt_state(0x06);
                continue;
            }

            let mut x = 0x0a;
            loop {
                engine.state.set_coins((engine.state.coins() - 1) & 0xFF);
                crate::game::sync_coin_hud(engine, r);
                engine.state.set_prompt_state(0x0c);
                engine.state.set_frame_counter(0x0a);
                frame::commit_frame_work(engine, r);
                frame::wait_for_frame_counter(engine, r);
                x = u8v(x - 1);
                if x == 0 {
                    break;
                }
            }
            fade_room_palette_out_keep_audio(engine, r);
            animate_health_refill_to_cap(engine, r);
            animate_magic_refill_to_cap(engine, r);
            r.value = 0x08;
            crate::game::refresh_temporary_room_page(engine, r);
            crate::game::draw_carried_item_sprites(engine, r);
            crate::game::upload_inventory_count_tiles(engine, r);
            crate::game::upload_equipped_item_stat_tiles(engine, r);
            engine.state.set_scroll_fine_x(0x08);
            crate::game::refresh_scroll_register_shadows(engine, r);
            crate::game::draw_player_sprites(engine, r);
            fade_room_palette_in(engine, r);
            run_carried_item_loadout_flow(engine, r);
            r.value = 0x04;
            crate::game::refresh_temporary_room_page(engine, r);
            crate::game::clear_temporary_room_sprites(engine, r);
            crate::game::draw_coin_cost_sprites(engine, r);
            fade_room_palette_in(engine, r);
        }
    }

    engine.state.set_player_health(0x00);
    engine.state.set_player_magic(0x00);
    if engine.state.character_index() < 0x06 {
        for y in (0..=2).rev() {
            let x = engine.state.item_slot(y);
            if (x & 0x80) == 0 {
                engine
                    .state
                    .set_inventory_item(x, (engine.state.inventory_item(x) + 1) & 0xFF);
            }
            engine.state.set_item_slot(y, 0xff);
        }
        crate::game::snapshot_inventory_state(engine, r);
    }

    push_room_checkpoint(engine, r);
    engine.state.set_character_index(0x06);
    r.value = 0x06;
    crate::game::enter_temporary_room_page(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    engine.state.set_selected_item_slot(0x03);
    crate::game::draw_status_item_sprites(engine, r);
    engine.state.set_player_pose(0xf1);
    engine.state.set_player_facing(0x00);
    crate::game::draw_player_sprites(engine, r);
    crate::game::restore_status_sprite_template(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        walk_character_select_room_until_action(engine, r);
        let hi = engine.state.scratch2() & 0xf0;
        let mut chosen: Option<i32> = None;
        if hi == 0x50 {
            if (engine.state.indirect_ptr_hi() & 0x0f) == 0x05 && engine.state.continue_timer() != 0
            {
                let mut x = u8v(engine.state.song() + 1);
                if x >= 0x10 {
                    x = 0x00;
                }
                engine.state.set_song(x);
                crate::game::song_init(engine, r);
                if (engine.state.continue_timer() & 0x80) != 0 && engine.state.buttons() == 0xc3 {
                    for x in (0..=0x0d).rev() {
                        engine.state.set_inventory_item(x, 0x10);
                    }
                    engine.state.set_continue_timer(0x80);
                    engine.state.set_coins(0x80);
                    engine.state.set_keys(0x80);
                    engine.state.set_prompt_state(0x1a);
                }
            }
            continue;
        } else if hi == 0x70 {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            if lo == 0x06 {
                chosen = Some(0x00);
            } else if lo == 0x08 {
                chosen = Some(0x01);
            }
        } else if hi == 0x80 {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            if lo == 0x04 {
                chosen = Some(0x02);
            } else if lo == 0x0a {
                engine.state.set_prompt_state(0x03);
                show_inventory_item_list_screen(engine, r);
                continue;
            } else if lo == 0x0c {
                engine.state.set_prompt_state(0x03);
                run_inventory_item_grid_menu(engine, r);
                continue;
            }
        } else if hi == 0x90 {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            if lo == 0x06 {
                chosen = Some(0x03);
            } else if lo == 0x0a {
                chosen = Some(0x04);
            }
        }

        let Some(x) = chosen else {
            continue;
        };

        engine.state.set_character_index(x);
        r.offset = u8v((x << 2) + 0x03);
        for xi in (0..=3).rev() {
            engine.state.set_item_slot(
                0x0B + xi,
                engine.state.byte(u16v(CHARACTER_STATS_TABLE + r.offset)),
            );
            r.offset = u8v(r.offset - 1);
        }
        engine.state.set_prompt_state(0x18);
        engine.state.set_prompt_argument(0xff);
        engine.state.set_frame_counter(0x04);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x05;
        flash_palette_buffer(engine, r);
        engine
            .state
            .set_chr_bank(2, u8v(engine.state.character_index() + 0x38));
        engine.state.set_chr_bank(3, 0x3d);
        engine.state.set_chr_bank(4, 0x3e);
        engine.state.set_chr_bank(5, 0x3f);
        engine.state.set_player_pose(0x0d);
        engine.state.set_player_facing(0x00);
        engine.state.set_player_y(engine.state.player_y() & 0xf0);
        engine.state.set_player_x_fine(0x04);
        crate::game::clear_gameplay_object_sprites(engine, r);
        crate::game::draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x05;
        flash_palette_buffer(engine, r);
        engine.state.set_frame_counter(0x78);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        fade_room_palette_out_reset_audio(engine, r);
        engine.state.set_player_pose(0x08);
        engine.state.set_player_facing(0x00);
        engine.state.set_player_health(0x63);
        engine.state.set_player_magic(0x63);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        engine.state.set_selected_item_slot(0x02);
        crate::game::draw_status_item_sprites(engine, r);
        r.value = 0x08;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.state.set_scroll_fine_x(0x08);
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
    r.value = engine.state.map_screen_x();
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

        let nib = engine.state.player_x_tile() & 0x0f;
        let x = if nib < 0x03 {
            continue;
        } else if nib < 0x05 {
            0x00
        } else {
            if nib < 0x0a || nib >= 0x0c {
                continue;
            }
            0x02
        };

        let item = engine.state.temp_save(x);
        if (item & 0x80) != 0 {
            engine.state.set_prompt_state(0x06);
        } else {
            let price = engine.state.inventory_item(0x21 + x);
            r.value = price;
            crate::game::spend_coins(engine, r);
            if ((r.carry) != 0) {
                engine.state.set_temp_save(x, 0xff);
                crate::game::draw_shop_item_sprites(engine, r);
                engine
                    .state
                    .set_inventory_item(item, (engine.state.inventory_item(item) + 1) & 0xFF);
                engine.state.set_prompt_state(0x10);
            } else {
                if item == 0x0d && engine.state.continue_timer() != 0 {
                    engine.state.set_shop_active(0x01);
                }
                engine.state.set_prompt_state(0x06);
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
        engine.state.set_frame_counter(0x01);
        let buttons = frame::read_buttons(engine, r);
        if (buttons & 0x80) != 0 {
            r.value = 0x80;
            return;
        }

        r.value = buttons & 0x0f;
        r.offset = 0x01;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.state.scratch2();
        if ty >= 0x30 && ty < 0xa1 {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            if lo >= 0x02 {
                let store = lo < 0x0d || engine.state.indirect_ptr_lo() == 0;
                if store {
                    engine
                        .state
                        .set_player_x_fine(engine.state.indirect_ptr_lo());
                    engine
                        .state
                        .set_player_x_tile(engine.state.indirect_ptr_hi());
                    engine.state.set_player_y(engine.state.scratch2());
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
    if (engine.state.obj_state() & 0x7f) == 0 {
        engine.state.set_prompt_state(0x18);
        engine.state.set_prompt_argument(0xff);
        r.index = 0x03;
        flash_palette_buffer(engine, r);

        engine.state.set_frame_counter(0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        flash_palette_buffer(engine, r);

        engine.state.set_frame_counter(0x05);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        flash_palette_buffer(engine, r);

        engine
            .state
            .set_obj_state((engine.state.obj_state() + 1) & 0xFF);
        engine.state.set_prompt_state(0x02);
        engine.state.set_obj_cooldown(0x0f);
        engine.state.set_obj_x_vel_lo(0x00);
        engine.state.set_obj_x_vel_hi(0x00);
        engine.state.set_obj_move_scratch(0x00);
        engine.state.set_obj_y_extra(engine.state.obj_y_pixel());
    }

    if engine.state.obj_move_scratch() == 0 {
        engine
            .state
            .set_obj_cooldown((engine.state.obj_cooldown() - 1) & 0xFF);
        if engine.state.obj_cooldown() == 0 {
            engine.state.set_obj_attr(engine.state.obj_attr() | 0x80);
            engine.state.set_obj_move_scratch(0x01);
            return;
        }
        let a = u8v(((engine.state.obj_cooldown() >> 2) ^ 0xff) + 1);
        engine.state.set_obj_y_vel(a);
        crate::game::project_actor_position(engine, r);
        crate::game::check_position_out_of_bounds(engine, r);
        if ((r.carry) != 0) {
            engine.state.set_obj_attr(engine.state.obj_attr() | 0x80);
            engine.state.set_obj_move_scratch(0x01);
            return;
        }
        engine.state.set_obj_y_pixel(engine.state.scratch2());
        return;
    }

    engine
        .state
        .set_obj_move_scratch((engine.state.obj_move_scratch() + 1) & 0xFF);
    engine
        .state
        .set_obj_y_vel(u8v((engine.state.obj_move_scratch() >> 2) + 1));
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if ((r.carry) != 0) {
        engine.state.set_obj_state(0x00);
        engine.state.set_obj_timer(0xf0);
        engine.state.set_pending_special_exit(0x01);
        return;
    }
    engine.state.set_obj_y_pixel(engine.state.scratch2());
}

/// Walks a purchase/refill room until the player presses action or reaches the
/// exit tile. Carry set means exit; carry clear means action on the current
/// selectable tile.
pub fn walk_purchase_room_until_action_or_exit(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        engine.state.set_frame_counter(0x01);
        let buttons = frame::read_buttons(engine, r);
        if (buttons & 0x80) != 0 {
            r.value = 0x80;
            r.carry = 0;
            return;
        }

        r.value = buttons & 0x0f;
        r.offset = 0x01;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.state.scratch2();
        if ty >= 0xa1 {
            r.value = ty;
            r.carry = 1;
            return;
        }
        if ty >= 0x8c {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            if lo >= 0x02 && lo < 0x0d {
                engine
                    .state
                    .set_player_x_fine(engine.state.indirect_ptr_lo());
                engine
                    .state
                    .set_player_x_tile(engine.state.indirect_ptr_hi());
                engine.state.set_player_y(engine.state.scratch2());
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
        engine.state.set_frame_counter(0x01);
        let buttons = frame::read_buttons(engine, r);
        if (buttons & 0x80) != 0 {
            r.value = 0x80;
            r.carry = 0;
            return;
        }

        r.value = buttons & 0x0f;
        r.offset = 0x01;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.state.scratch2();
        if ty >= 0xa1 {
            r.value = ty;
            r.carry = 1;
            return;
        }
        if ty >= 0x20 {
            let lo = engine.state.indirect_ptr_hi() & 0x0f;
            let mut store = false;
            if lo >= 0x01 {
                if lo < 0x0f {
                    store = true;
                } else if engine.state.indirect_ptr_lo() == 0 {
                    store = true;
                }
            }
            if store {
                engine
                    .state
                    .set_player_x_fine(engine.state.indirect_ptr_lo());
                engine
                    .state
                    .set_player_x_tile(engine.state.indirect_ptr_hi());
                engine.state.set_player_y(engine.state.scratch2());
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
            let e = engine.state.selected_item_slot();
            if engine.state.item_slot(e) == 0x0d {
                engine.state.set_selected_item_slot(0x03);
                crate::game::draw_status_item_sprites(engine, r);
            }
            return;
        }
        let mut x = 0xff;
        let py = engine.state.player_y();
        let flow_0441 = if py >= 0x58 {
            true
        } else {
            x = if py < 0x38 { 0x00 } else { 0x08 };
            engine.state.set_scratch0(x);
            x = u8v((engine.state.player_x_tile() >> 1) | engine.state.scratch0());
            if engine.state.inventory_item(x) != 0 {
                r.value = x;
                crate::game::load_family_item_permission_bits(engine, r);
                if ((r.carry) != 0) {
                    engine
                        .state
                        .set_inventory_item(x, (engine.state.inventory_item(x) - 1) & 0xFF);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if !flow_0441 {
            engine.state.set_prompt_state(0x06);
            continue;
        }
        engine.state.set_scratch0(x);
        let ci0 = engine.state.item_slot(0);
        if (ci0 & 0x80) == 0 {
            engine
                .state
                .set_inventory_item(ci0, (engine.state.inventory_item(ci0) + 1) & 0xFF);
        }
        engine.state.set_item_slot(0, engine.state.item_slot(1));
        engine.state.set_item_slot(1, engine.state.item_slot(2));
        engine.state.set_item_slot(2, engine.state.scratch0());
        engine.state.set_prompt_state(0x12);
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
    engine.state.set_room_restore_scratch(engine.state.song());
    if engine.room_ckpt_sp < engine.room_ckpt_stack.len() {
        let c = [
            engine.state.player_x_fine() as u8,
            engine.state.player_x_tile() as u8,
            engine.state.player_y() as u8,
            engine.state.scroll_fine_x() as u8,
            engine.state.scroll_tile_x() as u8,
            engine.state.map_screen_x() as u8,
            engine.state.map_screen_y() as u8,
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
        engine.state.set_player_x_fine(c[0] as i32);
        engine.state.set_player_x_tile(c[1] as i32);
        engine.state.set_player_y(c[2] as i32);
        engine.state.set_scroll_fine_x(c[3] as i32);
        engine.state.set_scroll_tile_x(c[4] as i32);
        engine.state.set_map_screen_x(c[5] as i32);
        engine.state.set_map_screen_y(c[6] as i32);
    }
}

/// Runs the high-bit defeated-actor reward drop sequence. The actor rises,
/// falls back into the playfield, then turns into a pickup chosen from current
/// resource needs and the drop table.
pub fn tick_defeated_actor_reward_drop(engine: &mut Engine, r: &mut RoutineContext) {
    const DROP_ITEM_TABLE: [i32; 9] = [0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x05, 0x06, 0x07];
    if (engine.state.obj_state() & 0x7f) == 0 {
        engine
            .state
            .set_obj_state((engine.state.obj_state() + 1) & 0xFF);
        engine.state.set_prompt_state(0x0e);
        engine.state.set_obj_cooldown(0x08);
        engine.state.set_obj_x_vel_lo(0x00);
        engine.state.set_obj_x_vel_hi(0x00);
        engine.state.set_obj_move_scratch(0x00);
        engine.state.set_obj_y_extra(engine.state.obj_y_pixel());
        let ptr =
            u16v(engine.state.actor_record_ptr_lo() | (engine.state.actor_record_ptr_hi() << 8));
        engine.state.set_obj_tile(engine.state.byte(u16v(ptr + 6)));
        engine.state.set_obj_attr(engine.state.obj_attr() & 0x03);
    }
    if engine.state.obj_move_scratch() == 0 {
        engine
            .state
            .set_obj_cooldown((engine.state.obj_cooldown() - 1) & 0xFF);
        if engine.state.obj_cooldown() != 0 {
            engine
                .state
                .set_obj_y_vel(u8v(0 - engine.state.obj_cooldown()));
            crate::game::project_actor_position(engine, r);
            crate::game::check_position_out_of_bounds(engine, r);
            if !((r.carry) != 0) {
                engine.state.set_obj_y_pixel(engine.state.scratch2());
                return;
            }
        }
        engine.state.set_obj_attr(engine.state.obj_attr() | 0x80);
        engine.state.set_obj_move_scratch(0x01);
        return;
    }
    engine
        .state
        .set_obj_move_scratch((engine.state.obj_move_scratch() + 1) & 0xFF);
    engine
        .state
        .set_obj_y_vel(u8v((engine.state.obj_move_scratch() >> 1) + 2));
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if !((r.carry) != 0) {
        engine.state.set_obj_y_pixel(engine.state.scratch2());
        return;
    }
    let mut x = 0x00;
    if engine.state.player_health() < 0x14 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 0x01;
    if engine.state.player_magic() < 0x1e {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 0x04;
    if engine.state.keys() < 0x02 {
        item_spawn_setup(engine, r, x);
        return;
    }
    r.value = 0x14;
    crate::game::rng_update(engine, r);
    if r.value >= 0x09 {
        x = 0x00;
        if engine.state.player_health() < engine.state.player_magic() {
            if engine.state.player_health() < engine.state.coins() {
                item_spawn_setup(engine, r, x);
                return;
            }
            x = 0x02;
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 0x01;
        if engine.state.player_magic() < engine.state.coins() {
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 0x02;
        item_spawn_setup(engine, r, x);
        return;
    }
    x = DROP_ITEM_TABLE[r.value as usize];
    item_spawn_setup(engine, r, x);
}
