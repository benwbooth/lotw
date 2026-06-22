use crate::{Engine, RoutineContext, cbool, engine::RoutineFn, frame, u8v, u16v};

fn buttons(engine: &Engine) -> i32 {
    engine.mem(0x20)
}

fn set_buttons(engine: &mut Engine, buttons: i32) {
    engine.set_mem(0x20, buttons);
}

fn button_chord(engine: &Engine) -> i32 {
    engine.mem(0x21)
}

fn set_frame_counter(engine: &mut Engine, frames: i32) {
    engine.set_mem(0x36, frames);
}

fn frame_counter_active(engine: &Engine) -> bool {
    engine.mem(0x36) != 0
}

fn set_prompt_state(engine: &mut Engine, state: i32) {
    engine.set_mem(0x8f, state);
}

fn set_prompt_argument(engine: &mut Engine, argument: i32) {
    engine.set_mem(0x90, argument);
}

fn set_countdown_timer(engine: &mut Engine, frames: i32) {
    engine.set_mem(0x8c, frames);
}

fn countdown_timer_active(engine: &Engine) -> bool {
    engine.mem(0x8c) != 0
}

fn frame_status_bit6_set(engine: &Engine) -> bool {
    (engine.mem(0x26) & 0x40) != 0
}

fn set_sprite_blink_timer(engine: &mut Engine, frames: i32) {
    engine.set_mem(0x85, frames);
}

fn sprite_blink_timer(engine: &Engine) -> i32 {
    engine.mem(0x85)
}

fn set_player_health(engine: &mut Engine, value: i32) {
    engine.set_mem(0x58, value);
}

fn player_health(engine: &Engine) -> i32 {
    engine.mem(0x58)
}

fn set_player_magic(engine: &mut Engine, value: i32) {
    engine.set_mem(0x59, value);
}

fn enter_return_home(engine: &mut Engine, lo: i32, hi: i32) {
    engine.set_mem(0x0e, lo);
    engine.set_mem(0x0f, hi);
    engine.set_mem(0x30, engine.mem(0x32));
    engine.set_mem(0x31, engine.mem(0x33));
    engine.set_mem(0x25, 0x06);
    engine.prg_map_shadow();
}

fn leave_return_home(engine: &mut Engine) {
    engine.set_mem(0x30, 0x0c);
    engine.set_mem(0x31, 0x0d);
    engine.set_mem(0x25, 0x07);
    engine.prg_map_shadow();
}

fn farcall_cce4(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    enter_return_home(engine, lo, hi);
    target(engine, r);
    leave_return_home(engine);
}

fn farcall_0c0d(engine: &mut Engine, r: &mut RoutineContext, lo: i32, hi: i32, target: RoutineFn) {
    let old6 = engine.mem(0x30);
    let old7 = engine.mem(0x31);
    engine.set_mem(0x32, old6);
    engine.set_mem(0x33, old7);
    engine.set_mem(0x0e, lo);
    engine.set_mem(0x0f, hi);
    engine.set_mem(0x30, 0x0c);
    engine.set_mem(0x31, 0x0d);
    engine.set_mem(0x25, 0x07);
    engine.prg_map_shadow();
    target(engine, r);
    engine.set_mem(0x31, old7);
    engine.set_mem(0x30, old6);
    engine.set_mem(0x25, 0x06);
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
    engine.set_mem(0x16, dlo);
    engine.set_mem(0x17, dhi);
    engine.set_mem(0x18, slo);
    engine.set_mem(0x19, shi);
    engine.set_mem(0x1a, len);
    r.value = 0x05;
    queue_ppu_job_and_wait(engine, r);
}

fn item_spawn_setup(engine: &mut Engine, r: &mut RoutineContext, x: i32) {
    engine.set_mem(0xee, u8v(x + 2));
    engine.set_mem(0xed, u8v((x << 2) | 0x81));
    engine.set_mem(0xef, 0x01);
    engine.set_mem(0xfb, engine.mem(0xfc));
    engine.set_mem(0xf3, 0xf0);
    engine.set_mem(0xf0, 0x00);
    engine.set_mem(0xf1, 0x00);
    crate::game::update_object_terrain_probe(engine, r);
}

/// Queues the VRAM job id in `r.value` and waits until the NMI-side upload has
/// consumed it.
pub fn queue_ppu_job_and_wait(engine: &mut Engine, r: &mut RoutineContext) {
    frame::wait_for_ppu_job_idle(engine, r);
    engine.set_mem(0x28, r.value);
    frame::wait_for_ppu_job_idle(engine, r);
}

/// Shows the start-button prompt and waits for release, press, and release so a
/// held Start does not leak into the next menu/gameplay state.
pub fn wait_for_start_button_prompt(engine: &mut Engine, r: &mut RoutineContext) {
    set_prompt_state(engine, 0x03);
    engine.inc_mem(0x8d);
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
    set_prompt_state(engine, 0x04);
    engine.dec_mem(0x8d);
}

pub fn main_loop_dispatch(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        if frame::frame_runner_stop_requested() {
            return;
        }
        if player_health(engine) == 0 {
            set_sprite_blink_timer(engine, 0x00);
            crate::game::draw_player_sprites(engine, r);
            farcall_0c0d(engine, r, 0x07, 0xb3, run_player_death_or_continue_flow);
            if r.index == 0 {
                continue;
            }
            r.index = u8v(r.index - 1);
            crate::game::main_init(engine, r);
            return;
        }

        set_frame_counter(engine, 0x01);
        engine.set_mem(0x7e, engine.mem(0x7c));
        frame::read_buttons(engine, r);
        crate::game::game_update(engine, r);

        if engine.mem(0xec) != 0 {
            // The final-exit item diverts the normal room loop into a scripted
            // sequence that still reuses the player/object update helpers.
            farcall_0c0d(engine, r, 0xeb, 0xa2, setup_final_exit_sequence);
            loop {
                frame::read_buttons(engine, r);
                farcall_0c0d(engine, r, 0xbc, 0xab, crate::game::routine_0021);
                farcall_0c0d(engine, r, 0xe6, 0xa5, crate::game::routine_0005);
                farcall_0c0d(engine, r, 0x5d, 0xa7, crate::game::routine_0014);
                farcall_0c0d(engine, r, 0xe3, 0xa3, tick_final_exit_sequence);
                if player_health(engine) != 0 {
                    break;
                }
            }

            engine.set_mem(0x44, engine.mem(0x43) >> 4);
            engine.and_mem(0x43, 0x0f);
            engine.set_mem(0x0200, 0xef);
            set_sprite_blink_timer(engine, 0x00);
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
        if !cbool(r.carry) && engine.mem(0x7e) != engine.mem(0x7c) {
            engine.inc_mem(0x3d);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Sets up the final-exit sequence after the final item trigger: flash the
/// current scene, switch to the scripted room, and seed the special object/player
/// state used by `tick_final_exit_sequence`.
pub fn setup_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    set_prompt_state(engine, 0x18);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::draw_player_sprites(engine, r);

    r.index = 0x02;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    r.index = 0x03;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    fade_partial_palette_buffer_out(engine, r);

    set_prompt_state(engine, 0x20);
    set_frame_counter(engine, 0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.set_mem(0x48, 0x13);
    engine.set_mem(0x47, 0x02);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);

    engine.set_mem(0x0200, 0xef);
    engine.set_mem(0x1e, 0x22);
    engine.set_mem(0x7b, 0x00);
    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x7c, 0x10);
    farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::upload_current_room_view);
    r.index = 0x04;
    farcall_cce4(engine, r, 0x40, 0xc5, flash_palette_buffer);
    engine.set_mem(0x7c, 0x00);
    farcall_cce4(
        engine,
        r,
        0x6c,
        0xc7,
        crate::game::upload_room_columns_from_bank9,
    );
    engine.set_mem(0x2d, 0x3d);

    loop {
        let mut x = engine.mem(0x1e);
        if x == 0 {
            x = 0xf0;
        }
        if x == 0xc2 {
            break;
        }
        x = u8v(x - 1);
        engine.set_mem(0x1e, x);
        engine.set_mem(0x1d, (x & 0x08) >> 3);
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

    engine.set_mem(0x040c, 0x00);
    engine.set_mem(0x040d, 0x00);
    engine.set_mem(0x0406, 0x00);
    engine.set_mem(0xe9, 0x00);
    engine.set_mem(0x7b, 0x00);
    engine.set_mem(0x7c, 0x00);
    engine.set_mem(0x0405, 0x64);
    engine.set_mem(0x3e, 0x08);
    engine.set_mem(0x43, u8v((engine.mem(0x44) << 4) | engine.mem(0x43)));
    crate::game::routine_0026(engine, r);
    engine.set_mem(0x0210, 0xef);
    engine.set_mem(0x0214, 0xef);
    crate::game::routine_0016(engine, r);
    crate::game::routine_0018(engine, r);
}

/// Runs the one-shot final-exit cutscene path that is entered before the special
/// object latch at `0xF2` is set.
fn run_final_exit_cutscene(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::build_object_health_meter_standard_tiles(engine, r);
    engine.set_mem(0x0411, 0x00);
    engine.set_mem(0x0421, 0x00);
    engine.set_mem(0x0431, 0x00);
    engine.set_mem(0x00f2, 0x00);
    set_sprite_blink_timer(engine, 0x00);
    engine.set_mem(0x88, 0x00);
    crate::game::routine_0026(engine, r);
    crate::game::routine_0012(engine, r);
    engine.set_mem(0x0200, 0xef);

    while engine.mem(0x45) < 0xa0 {
        engine.inc_mem(0x45);
        crate::game::routine_0026(engine, r);
        set_frame_counter(engine, 0x01);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.set_mem(0x4e, 0x00);
    engine.set_mem(0x4f, 0x00);
    crate::game::routine_0024(engine, r);
    crate::game::routine_0026(engine, r);
    engine.set_mem(0x7c, 0x20);
    engine.set_mem(0x1d, 0x01);
    set_prompt_state(engine, 0x20);
    set_prompt_argument(engine, 0x80);
    engine.set_mem(0x7a, 0xb6);

    loop {
        crate::game::routine_0003(engine, r);
        if engine.mem(0xfa) == 0 {
            break;
        }
    }
    loop {
        crate::game::routine_0003(engine, r);
        if engine.mem(0xfa) == 0 {
            break;
        }
    }

    set_prompt_state(engine, 0x20);
    set_prompt_argument(engine, 0x80);
    engine.set_mem(0x7a, 0xb7);
    loop {
        crate::game::routine_0003(engine, r);
        if engine.mem(0xfa) == 0 {
            break;
        }
    }
    loop {
        crate::game::routine_0003(engine, r);
        if engine.mem(0xfa) == 0 {
            break;
        }
    }

    engine.set_mem(0x10, 0x00);
    loop {
        if (engine.mem(0x84) & 0x07) == 0 {
            engine.xor_mem(0x1d, 0x01);
            set_prompt_state(engine, 0x20);
            set_prompt_argument(engine, 0x80);
        }

        r.value = 0xff;
        queue_ppu_job_and_wait(engine, r);
        if frame_status_bit6_set(engine) {
            r.value = 0x05;
            crate::game::routine_0030(engine, r);
            crate::game::build_player_health_meter_sprites(engine, r);
        }

        if engine.mem(0x3e) == 0 {
            engine.set_mem(0x3e, 0x02);
        }

        crate::game::routine_0026(engine, r);
        crate::game::routine_0014(engine, r);
        engine.dec_mem(0x10);
        if engine.mem(0x10) == 0 {
            break;
        }
    }

    engine.set_mem(0x1d, 0x01);
    r.value = 0xff;
    queue_ppu_job_and_wait(engine, r);
    if player_health(engine) == 0 {
        return;
    }

    engine.set_mem(0x0200, 0xef);
    set_prompt_state(engine, 0x18);
    set_prompt_argument(engine, 0xff);
    engine.set_mem(0x08, 0x01);
    loop {
        let prev = engine.mem(0x45);
        let ny = u8v(prev - engine.mem(0x08));
        engine.set_mem(0x45, ny);
        let c = if prev >= engine.mem(0x08) { 1 } else { 0 };
        let t = u8v(ny + 0x2b + c);
        if t >= 0xef {
            break;
        }
        crate::game::routine_0026(engine, r);
        engine.inc_mem(0x08);
        r.value = 0xff;
        queue_ppu_job_and_wait(engine, r);
    }

    engine.set_mem(0x0210, 0xef);
    engine.set_mem(0x0214, 0xef);
    engine.set_mem(0x3e, 0x00);
    engine.set_mem(0x3f, 0x80);
    crate::game::reset_room_object_slots(engine, r);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    engine.set_mem(0x48, 0x10);
    engine.set_mem(0x47, 0x03);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);
    engine.set_mem(0x7c, 0x12);
    engine.set_mem(0x45, 0xc0);
    engine.set_mem(0x44, 0x1a);
    engine.set_mem(0x43, 0x01);
    engine.set_mem(0x7b, 0x01);
    engine.set_mem(0x56, 0x09);
    engine.set_mem(0x2c, 0x35);
    engine.set_mem(0x2d, 0x34);
    engine.set_mem(0x2e, 0x36);
    engine.set_mem(0x2f, 0x37);
    engine.set_mem(0x0411, 0x01);
    engine.set_mem(0x0421, 0x01);
    engine.set_mem(0x0431, 0x01);
    engine.set_mem(0x0441, 0x01);
    engine.set_mem(0x041e, 0xa0);
    engine.set_mem(0x042e, 0xa0);
    engine.set_mem(0x043e, 0xa0);
    engine.set_mem(0x044e, 0x70);
    engine.set_mem(0x044d, 0x33);
    crate::game::routine_0019(engine, r);
    let mut v = 0x2d;
    engine.set_mem(0x0410, v);
    v = u8v(v + 0x20);
    engine.set_mem(0x0420, v);
    v = u8v(v + 0x20);
    engine.set_mem(0x0430, v);
    engine.set_mem(0x0440, 0x81);
    engine.set_mem(0x0412, 0x40);
    engine.set_mem(0x0422, 0x40);
    engine.set_mem(0x0432, 0x40);
    engine.set_mem(0x0442, 0x40);
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
    engine.set_mem(0x40, 0x07);
    farcall_cce4(engine, r, 0x92, 0xc4, fade_room_palette_in);
    set_countdown_timer(engine, 0x05);
    while countdown_timer_active(engine) {
        draw_scene_and_wait_one_frame(engine, r);
    }

    loop {
        if engine.mem(0x45) == 0xa0 {
            break;
        }
        engine.dec_mem(0x45);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        if engine.mem(0x45) == 0xa0 {
            break;
        }
        engine.dec_mem(0x45);
        engine.xor_mem(0x57, 0x40);
        crate::game::draw_player_sprites(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        draw_scene_and_wait_one_frame(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.set_mem(0x56, 0x0d);
    crate::game::draw_player_sprites(engine, r);
    set_countdown_timer(engine, 0x03);
    while countdown_timer_active(engine) {
        draw_scene_and_wait_one_frame(engine, r);
    }

    loop {
        set_frame_counter(engine, 0x01);
        engine.set_mem(0x7e, engine.mem(0x7c));
        set_buttons(engine, 0x01);
        farcall_cce4(engine, r, 0x2b, 0xd4, crate::game::game_update);
        farcall_cce4(
            engine,
            r,
            0x5d,
            0xc1,
            crate::game::update_camera_scroll_from_player,
        );
        crate::game::routine_0019(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        if engine.mem(0x7e) != engine.mem(0x7c) {
            engine.inc_mem(0x3d);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        if engine.mem(0x44) == 0x37 {
            break;
        }
    }

    engine.set_mem(0x56, 0x19);
    engine.set_mem(0x0410, 0x39);
    engine.set_mem(0x0420, 0x59);
    engine.set_mem(0x0430, 0x79);
    engine.set_mem(0x0440, 0x91);
    set_countdown_timer(engine, 0x14);
    while countdown_timer_active(engine) {
        engine.xor_mem(0x56, 0x04);
        engine.xor_mem(0x0410, 0x04);
        engine.xor_mem(0x0420, 0x04);
        engine.xor_mem(0x0430, 0x04);
        engine.xor_mem(0x0440, 0x04);
        for _ in 0..8 {
            draw_scene_and_wait_one_frame(engine, r);
        }
    }

    run_story_text_sequence(engine, r);
}

/// Ticks the final-exit scripted object state machine and stores the updated
/// scratch slot back into the active object slot.
pub fn tick_final_exit_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0xe5, 0x00);
    engine.set_mem(0xe6, 0x04);
    crate::game::load_object_slot_scratch(engine, r);
    if engine.mem(0x00f2) == 0 {
        run_final_exit_cutscene(engine, r);
        return;
    }

    if frame_status_bit6_set(engine) {
        let t = (engine.mem(0x3e) + 2) & 0x06;
        if t != 0 {
            let x = u8v(t << 3);
            if engine.mem(u16v(0x0401 + x)) != 0 {
                engine.set_mem(u16v(0x0401 + x), 0x00);
                let sum = u8v(engine.mem(0x1c) + engine.mem(u16v(0x040c + x)));
                if sum >= 0xb0 && sum < 0xd0 {
                    let bl = engine.mem(0x00f2);
                    engine.set_mem(0x00f2, if bl < 0x02 { 0x00 } else { u8v(bl - 0x02) });
                    crate::game::build_object_health_meter_standard_tiles(engine, r);
                    set_prompt_state(engine, 0x20);
                    set_prompt_argument(engine, 0x01);
                } else {
                    set_prompt_state(engine, 0x01);
                }
            }
        }
    }

    if engine.mem(0xfa) == 0 {
        match engine.mem(0xf3) {
            4 => {
                engine.dec_mem(0xe9);
                if engine.mem(0xe9) != 0 {
                    if engine.mem(0xe9) == 0x04 {
                        set_prompt_state(engine, 0x20);
                    }
                    engine.set_mem(0x7a, 0xb5);
                    engine.set_mem(0x1e, 0xc2);
                } else {
                    engine.set_mem(0x7a, 0xb3);
                    engine.set_mem(0xf3, 0x00);
                }
            }
            3 => {
                engine.dec_mem(0xe9);
                if engine.mem(0xe9) != 0 {
                    engine.set_mem(0x7a, 0xb2);
                    if engine.mem(0x1c) != 0 {
                        let v = if engine.mem(0x1c) < 0x04 {
                            0x00
                        } else {
                            u8v(engine.mem(0x1c) - 0x04)
                        };
                        engine.set_mem(0x1c, v);
                        if v >= 0x11 {
                            if engine.mem(0x1e) < 0xd2 {
                                engine.add_mem(0x1e, 0x04);
                            } else if engine.mem(0x1c) != 0 {
                                engine.sub_mem(0x1c, 0x04);
                            }
                        } else if engine.mem(0x1e) >= 0xc3 {
                            engine.sub_mem(0x1e, 0x04);
                        }
                    } else if engine.mem(0x1e) >= 0xc3 {
                        engine.sub_mem(0x1e, 0x04);
                    }
                } else if engine.mem(0x1c) != 0 {
                    engine.set_mem(0xf3, 0x00);
                } else {
                    engine.set_mem(0x7a, 0xb0);
                    engine.inc_mem(0xf3);
                    engine.set_mem(0xe9, 0x04);
                }
            }
            2 => {
                engine.dec_mem(0xe9);
                if engine.mem(0xe9) != 0 {
                    engine.set_mem(0x7a, 0xb4);
                    if engine.mem(0x1e) >= 0xc3 {
                        engine.sub_mem(0x1e, 0x04);
                    }
                } else {
                    engine.set_mem(0x7a, 0xb3);
                    engine.set_mem(0xf3, 0x00);
                }
            }
            1 => {
                engine.dec_mem(0xe9);
                if engine.mem(0xe9) == 0 {
                    engine.set_mem(0xf3, 0x00);
                } else {
                    let a = u8v(((engine.mem(0xe9) << 1) & 0x01) + 0xb0);
                    engine.set_mem(0x7a, a);
                    engine.add_mem(0x1c, 0x04);
                    if engine.mem(0x1c) >= 0x40 {
                        engine.set_mem(0xf3, 0x00);
                    } else {
                        engine.set_mem(0x1e, 0xc2);
                    }
                }
            }
            _ => {
                let sum = u8v(engine.mem(0x1c) + engine.mem(0x43));
                let carry = sum < engine.mem(0x1c);
                let close = carry || sum >= 0xc0 || engine.mem(0x1c) >= 0x40;
                let delayed_grow = sum < 0x80 || sum >= 0xa0;
                if close || (delayed_grow && engine.mem(0x1e) >= 0xc3) {
                    engine.set_mem(0xf3, 0x03);
                    engine.set_mem(0xe9, 0x02);
                    engine.dec_mem(0xe9);
                    if engine.mem(0xe9) != 0 {
                        engine.set_mem(0x7a, 0xb2);
                        if engine.mem(0x1c) != 0 {
                            let v = if engine.mem(0x1c) < 0x04 {
                                0x00
                            } else {
                                u8v(engine.mem(0x1c) - 0x04)
                            };
                            engine.set_mem(0x1c, v);
                            if v >= 0x11 {
                                if engine.mem(0x1e) < 0xd2 {
                                    engine.add_mem(0x1e, 0x04);
                                } else if engine.mem(0x1c) != 0 {
                                    engine.sub_mem(0x1c, 0x04);
                                }
                            } else if engine.mem(0x1e) >= 0xc3 {
                                engine.sub_mem(0x1e, 0x04);
                            }
                        } else if engine.mem(0x1e) >= 0xc3 {
                            engine.sub_mem(0x1e, 0x04);
                        }
                    } else if engine.mem(0x1c) != 0 {
                        engine.set_mem(0xf3, 0x00);
                    } else {
                        engine.set_mem(0x7a, 0xb0);
                        engine.inc_mem(0xf3);
                        engine.set_mem(0xe9, 0x04);
                    }
                } else if !delayed_grow {
                    engine.set_mem(0xf3, 0x02);
                    engine.set_mem(0xe9, 0x08);
                    engine.set_mem(0x7a, 0xb3);
                } else {
                    engine.set_mem(0xf3, 0x01);
                    engine.set_mem(0xe9, 0x04);
                    engine.dec_mem(0xe9);
                    if engine.mem(0xe9) == 0 {
                        engine.set_mem(0xf3, 0x00);
                    } else {
                        let a = u8v(((engine.mem(0xe9) << 1) & 0x01) + 0xb0);
                        engine.set_mem(0x7a, a);
                        engine.add_mem(0x1c, 0x04);
                        if engine.mem(0x1c) >= 0x40 {
                            engine.set_mem(0xf3, 0x00);
                        } else {
                            engine.set_mem(0x1e, 0xc2);
                        }
                    }
                }
            }
        }
    }

    crate::game::routine_0003(engine, r);
    crate::game::store_object_slot_scratch(engine, r);
}

/// Fades the first 13 palette-buffer entries toward black over four timed
/// foreground frames.
pub fn fade_partial_palette_buffer_out(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 0x04;
    loop {
        set_frame_counter(engine, 0x05);
        for x in (0..=0x0c).rev() {
            let lo = engine.mem(u16v(0x0180 + x)) & 0x0f;
            let hi = engine.mem(u16v(0x0180 + x)) & 0xf0;
            engine.set_mem(0x08, lo);
            let out = if hi < 0x10 {
                0x0f
            } else {
                u8v((hi - 0x10) | lo)
            };
            engine.set_mem(u16v(0x0180 + x), out);
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
    set_frame_counter(engine, 0x01);
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

    set_frame_counter(engine, 0x01);
    enter_return_home(engine, 0x35, 0xc1);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
    leave_return_home(engine);
}

/// Runs the scrolling story-text sequence shared by the title-screen chord and
/// the final-exit cutscene.
pub fn run_story_text_sequence(engine: &mut Engine, r: &mut RoutineContext) {
    engine.inc_mem(0x92);
    drain_audio_timers_with_object_frames(engine, r);
    fade_palette_buffer_out(engine, r);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::hide_all_sprite_y_positions(engine, r);
    engine.set_mem(0x2a, 0x20);
    engine.set_mem(0x2b, 0x22);
    engine.or_mem(0x24, 0x18);

    r.value = 0xff;
    queue_ppu_job_and_wait(engine, r);

    engine.set_mem(0x8e, 0x0a);
    crate::game::song_init(engine, r);

    engine.set_mem(0x1c, 0x00);
    engine.set_mem(0x1d, 0x00);
    engine.set_mem(0x0a, 0x00);
    engine.set_mem(0x7b, 0x00);
    engine.set_mem(0x7c, 0x00);
    crate::game::load_intro_text_palette(engine, r);

    engine.set_mem(0x18, 0x40);
    engine.set_mem(0x19, 0x01);
    engine.set_mem(0x1a, 0x20);
    engine.set_mem(0x0c, 0x9c);
    engine.set_mem(0x0d, 0xb7);

    loop {
        crate::game::advance_intro_text_scroll(engine, r);
        crate::game::stage_intro_text_line(engine, r);
        if cbool(r.carry) {
            break;
        }
        crate::game::advance_intro_text_scroll(engine, r);
        crate::game::stage_scrolling_intro_text_line(engine, r);
        if cbool(r.carry) {
            break;
        }
    }

    set_prompt_state(engine, 0x20);
    while engine.mem(0xd4) == 0 {
        frame::wait_frame(engine, r);
    }
    while engine.mem(0xd4) != 0 {
        frame::wait_frame(engine, r);
    }

    set_frame_counter(engine, 0x3c);
    frame::wait_for_frame_counter(engine, r);

    engine.set_mem(0x94, 0x00);
    engine.set_mem(0xa4, 0x00);
    engine.set_mem(0xb4, 0x00);
    engine.set_mem(0xc4, 0x00);
    set_prompt_state(engine, 0x18);

    let mut cnt = 0x0a;
    loop {
        for x in (0..=0x1f).rev() {
            engine.set_mem(u16v(0x0180 + x), 0x30);
        }
        crate::game::upload_palette_buffer(engine, r);
        set_frame_counter(engine, 0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::load_intro_text_palette(engine, r);
        crate::game::upload_palette_buffer(engine, r);
        set_frame_counter(engine, 0x02);
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
        engine.set_mem(0x2c, 0x37);
        engine.set_mem(0x29, 0x00);
        engine.set_mem(0x23, 0xa0);
        engine.device_write(0x2000, 0xa0);
        engine.set_mem(0x24, 0x00);
        engine.device_write(0x2001, 0x00);
        engine.set_mem(0x1c, 0x00);
        engine.set_mem(0x1d, 0x00);
        engine.set_mem(0x1e, 0xe8);
        for x in (0..=0x1f).rev() {
            engine.set_mem(u16v(0x0180 + x), 0x0f);
        }
        farcall_cce4(engine, r, 0x69, 0xc5, crate::game::upload_palette_buffer);
        crate::game::reset_room_object_slots(engine, r);
        crate::game::clear_oam_with_sprite_zero_template(engine, r);
        crate::game::load_title_oam_template(engine, r);
        engine.set_mem(0x2c, 0x15);
        engine.set_mem(0x8e, 0x09);
        crate::game::song_init(engine, r);
        crate::game::upload_title_screen_nametables(engine, r);
        engine.set_mem(0x24, 0x1e);
        engine.device_write(0x2001, 0x1e);
        set_frame_counter(engine, 0x78);
        frame::wait_for_frame_counter(engine, r);
        fade_title_palette_in(engine, r);
        set_countdown_timer(engine, 0x14);

        loop {
            set_frame_counter(engine, 0x01);
            let pad = frame::read_buttons(engine, r);
            if pad == 0xff {
                set_prompt_state(engine, 0x1a);
                engine.set_mem(0x37, 0x1a);
            }
            if (buttons(engine) & 0x10) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }
            if button_chord(engine) == 0x83 {
                run_story_text_sequence(engine, r);
                return;
            }
            if (engine.mem(0x84) & 0x07) == 0 {
                let lo = engine.mem(0x0182) & 0x0f;
                let mut hi = engine.mem(0x0182) & 0xf0;
                engine.set_mem(0x08, lo);
                if hi < 0x10 {
                    hi = 0x30;
                } else {
                    hi = u8v(hi - 0x10);
                }
                engine.set_mem(0x0193, hi);
                engine.set_mem(0x0182, hi | engine.mem(0x08));
            }
            enter_return_home(engine, 0x35, 0xc1);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            leave_return_home(engine);
            if countdown_timer_active(engine) {
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
        engine.set_mem(0x47, r.value);
        r.value = 0x10;
        crate::game::rng_update(engine, r);
        engine.set_mem(0x48, r.value);
        farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);

        loop {
            r.value = 0x40;
            crate::game::rng_update(engine, r);
            engine.set_mem(0x44, r.value);
            engine.set_mem(0x0c, r.value);
            engine.set_mem(0x43, 0x00);
            r.value = 0x0b;
            crate::game::rng_update(engine, r);
            r.value = u8v(r.value << 4);
            engine.set_mem(0x45, r.value);
            engine.set_mem(0x0d, r.value);
            crate::game::resolve_room_tile_pointer(engine, r);
            let p = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
            let mut t = engine.mem(p) & 0x3f;
            if t >= 0x30 {
                continue;
            }
            if t == 0x02 {
                continue;
            }
            if t == engine.mem(0x70) {
                continue;
            }
            t = engine.mem(u16v(p + 1)) & 0x3f;
            if t < 0x30 {
                continue;
            }
            if t == 0x30 {
                continue;
            }
            break;
        }

        let mut x = engine.mem(0x44);
        if x < 0x08 {
            x = 0x00;
        } else {
            x = u8v(x - 0x08);
        }
        if x >= 0x30 {
            x = 0x30;
        }
        engine.set_mem(0x7c, x);
        engine.set_mem(0x7b, 0x00);

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
            if (mask & engine.mem(0x41)) != 0 {
                break chr;
            }
        };
        engine.set_mem(0x51, engine.mem(u16v(0xb0ac + chr)));
        engine.set_mem(0x55, 0x00);
        engine.set_mem(0x40, chr);
        let mut y = u16v(0xffa7 + ((chr << 2) + 0x03));
        for i in (0..=3).rev() {
            engine.set_mem(u16v(0x5c + i), engine.mem(y));
            y = u16v(y - 1);
        }
        engine.set_mem(0x2c, u8v(engine.mem(0x40) + 0x38));
        engine.set_mem(0x2e, 0x3e);
        engine.set_mem(0x2f, 0x20);
        engine.set_mem(0x56, 0x0d);
        engine.set_mem(0x57, 0x00);
        engine.set_mem(0x42, 0x01);
        set_player_health(engine, 0x64);
        set_player_magic(engine, 0x64);
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
        set_countdown_timer(engine, 0x0a);

        loop {
            set_frame_counter(engine, 0x01);
            engine.set_mem(0x7e, engine.mem(0x7c));
            crate::game::blink_demo_oam_sprites(engine, r);
            frame::read_buttons(engine, r);
            if (buttons(engine) & 0x10) != 0 {
                clear_title_screen_for_new_game(engine, r);
                return;
            }

            set_buttons(engine, engine.mem(0xfe));
            let mut do_b044 = true;
            if (engine.mem(0x49) | engine.mem(0x4b)) != 0 {
                engine.dec_mem(0x42);
                if engine.mem(0x42) != 0 {
                    do_b044 = false;
                }
            }
            if do_b044 {
                engine.set_mem(0x42, 0x80);
                crate::game::choose_random_demo_input(engine, r);
                engine.set_mem(0xfe, buttons(engine));
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
            if engine.mem(0x7e) != engine.mem(0x7c) {
                engine.inc_mem(0x3d);
            }
            enter_return_home(engine, 0x35, 0xc1);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            leave_return_home(engine);
            if countdown_timer_active(engine) {
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
    engine.set_mem(0xb4, 0);
    engine.set_mem(0x0d, 0x10);
    loop {
        if engine.mem(0xa0) != 0 {
            engine.dec_mem(0xa0);
        }
        if engine.mem(0xb0) != 0 {
            engine.dec_mem(0xb0);
        }
        if engine.mem(0xd0) != 0 {
            engine.dec_mem(0xd0);
        }
        engine.set_mem(0x0c, 0x14);
        loop {
            crate::game::draw_room_object_sprites(engine, r);
            set_frame_counter(engine, 0x01);
            frame::commit_frame_work(engine, r);
            frame::wait_for_frame_counter(engine, r);
            engine.dec_mem(0x0c);
            if engine.mem(0x0c) == 0 {
                break;
            }
        }
        engine.dec_mem(0x0d);
        if engine.mem(0x0d) == 0 {
            break;
        }
    }
}

/// Runs the player death animation, extra-life recovery, and game-over/continue
/// screen. `r.index` returns `0` for immediate resume; nonzero values are
/// decremented by the caller before re-entering `main_init`.
pub fn run_player_death_or_continue_flow(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_song = engine.mem(0x8e);

    engine.inc_mem(0x8d);
    crate::game::clear_gameplay_object_sprites(engine, r);
    r.index = 0x35;
    r.offset = 0x00;
    show_player_pose_for_eight_frames(engine, r);

    set_frame_counter(engine, 0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    r.value = 0x08;
    crate::game::switch_song_if_needed(engine, r);
    engine.dec_mem(0x8d);

    engine.set_mem(0x0a, 0x05);
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
        engine.dec_mem(0x0a);
        if engine.mem(0x0a) == 0 {
            break;
        }
    }

    set_frame_counter(engine, 0x01);
    engine.set_mem(0x56, 0x31);
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    let mut use_game_over_screen = engine.mem(0xec) != 0;
    if !use_game_over_screen {
        if (engine.mem(0x37) & 0x80) != 0 {
            let x = engine.mem(0x55);
            if engine.mem(u16v(0x51 + x)) == 0x0c {
                engine.set_mem(u16v(0x51 + x), 0xff);
                crate::game::draw_status_item_sprites(engine, r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            engine.inc_mem(0x37);
        }

        if !use_game_over_screen {
            animate_health_refill_to_cap(engine, r);
            engine.set_mem(0x56, 0x19);
            crate::game::read_debounced_buttons(engine, r);
            r.value = saved_song;
            crate::game::switch_song_if_needed(engine, r);
            r.index = 0x00;
            return;
        }
    }

    fade_palette_buffer_out(engine, r);
    engine.set_mem(0xec, 0x00);
    engine.set_mem(0x3e, 0x00);
    engine.set_mem(0x3f, 0x80);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    engine.set_mem(0x2b, 0x16);
    engine.set_mem(0x2c, 0x36);
    engine.set_mem(0x1c, 0x00);
    engine.set_mem(0x1d, 0x00);
    engine.set_mem(0x1e, 0x00);
    engine.set_mem(0x7b, 0x00);
    engine.set_mem(0x7c, 0x00);

    vram_blit(engine, r, 0x6b, 0x21, 0xaf, 0xb4, 0x09);
    vram_blit(engine, r, 0x4c, 0x22, 0xb8, 0xb4, 0x05);
    vram_blit(engine, r, 0x8c, 0x22, 0xbd, 0xb4, 0x08);

    engine.set_mem(0x44, 0x05);
    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x45, 0x70);
    engine.set_mem(0x56, 0x39);
    crate::game::clear_oam_with_sprite_zero_template(engine, r);
    crate::game::draw_player_sprites(engine, r);
    farcall_cce4(engine, r, 0xe0, 0xc4, fade_two_room_palette_rows_in);

    loop {
        crate::game::read_debounced_buttons(engine, r);
        if (r.value & 0x10) != 0 {
            break;
        }
        engine.xor_mem(0x45, 0x10);
        set_prompt_state(engine, 0x0c);
    }

    set_prompt_state(engine, 0x18);
    if engine.mem(0x45) != 0x70 {
        fade_palette_buffer_out(engine, r);
        set_frame_counter(engine, 0x78);
        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);
        r.index = 0x02;
        return;
    }

    crate::game::restore_inventory_state_snapshot(engine, r);
    engine.set_mem(0x51, 0xff);
    engine.set_mem(0x52, 0xff);
    engine.set_mem(0x53, 0xff);
    engine.set_mem(0x55, 0x03);
    engine.set_mem(0x40, 0x06);
    engine.set_mem(0x47, 0x03);
    engine.set_mem(0x48, 0x10);
    fade_palette_buffer_out(engine, r);
    engine.set_mem(0x8e, 0x02);
    crate::game::clear_name_tables_to_blank_tiles(engine, r);
    crate::game::upload_status_panel_template(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);

    r.value = 0x0f;
    for x in (0..=0x1f).rev() {
        engine.set_mem(u16v(0x0180 + x), 0x0f);
    }
    engine.set_mem(0x0210, 0xef);
    engine.set_mem(0x0214, 0xef);
    farcall_cce4(engine, r, 0xb4, 0xc4, fade_room_palette_row_in);
    r.index = 0x01;
}

/// Shows the player sprite pose in `r.index`/`r.offset` for eight foreground
/// frames.
pub fn show_player_pose_for_eight_frames(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x56, r.index);
    engine.set_mem(0x57, r.offset);
    set_frame_counter(engine, 0x08);
    crate::game::draw_player_sprites(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

/// Fades the title-screen palette from black to its ROM palette in five steps.
pub fn fade_title_palette_in(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x09, 0x40);
    loop {
        set_frame_counter(engine, 0x05);
        crate::game::load_title_palette_buffer(engine, r);
        r.index = 0x00;
        r.offset = 0x20;
        crate::game::dim_palette_range_by_step(engine, r);

        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);

        engine.sub_mem(0x09, 0x10);
        if (engine.mem(0x09) & 0x80) != 0 {
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
        set_frame_counter(engine, 0x05);
        for x in (0..=0x20).rev() {
            let v = engine.mem(u16v(0x0180 + x));
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.set_mem(0x08, lo);
            engine.set_mem(
                u16v(0x0180 + x),
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
    let ptr = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
    let mut v = 0x40;
    engine.set_mem(0x09, v);
    loop {
        set_frame_counter(engine, 0x05);
        for y in 0xe0..0xe4 {
            engine.set_mem(u16v(0x00a0 + y), engine.mem(u16v(ptr + y)));
        }
        r.index = 0x00;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
        if (v & 0x80) != 0 {
            break;
        }
    }
    crate::game::upload_palette_buffer(engine, r);
}

/// Fades in the first two room palette rows from the active room data pointer.
pub fn fade_two_room_palette_rows_in(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
    let mut v = 0x40;
    engine.set_mem(0x09, v);
    loop {
        set_frame_counter(engine, 0x05);
        for y in 0xe0..0xe4 {
            engine.set_mem(u16v(0x00a0 + y), engine.mem(u16v(ptr + y)));
        }
        for y in 0xf0..0xf4 {
            engine.set_mem(u16v(0x00a0 + y), engine.mem(u16v(ptr + y)));
        }
        r.index = 0x00;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        r.index = 0x10;
        r.offset = 0x04;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
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
    engine.set_mem(0x20, buttons);
}

/// Scans live object slots for a damageable actor overlapping the projected
/// position in `0x0E/0x0F/0x0A`. On hit, `0x08` receives the logical slot and
/// `0x09` receives the object-slot byte offset.
pub fn find_damageable_actor_overlap(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 0x09;
    let mut x = 0x90;
    loop {
        let mut skip = u8v(y) == engine.mem(0xe3);
        if !skip && (engine.mem(u16v(0x0401 + x)) & 0x80) != 0 {
            skip = true;
        }
        if !skip && engine.mem(u16v(0x0401 + x)) != 0x01 && engine.mem(u16v(0x0401 + x)) < 0x1a {
            skip = true;
        }
        if !skip && (engine.mem(u16v(0x0400 + x)) & 0xf9) == 0xe1 {
            skip = true;
        }
        if !skip && (engine.mem(u16v(0x0402 + x)) & 0x20) != 0 {
            skip = true;
        }
        if !skip {
            let mut d = u8v(engine.mem(0x0a) - engine.mem(u16v(0x040e + x)));
            if !(d < 0x10) && d < 0xf1 {
                skip = true;
            }
            if !skip {
                d = u8v(engine.mem(0x0f) - engine.mem(u16v(0x040d + x)));
                if d == 0 {
                    engine.set_mem(0x08, u8v(y));
                    engine.set_mem(0x09, x);
                    r.carry = 1;
                    return;
                }
                if d < 0x02 {
                    d = u8v(engine.mem(0x0e) - engine.mem(u16v(0x040c + x)));
                    if (d & 0x80) != 0 {
                        engine.set_mem(0x08, u8v(y));
                        engine.set_mem(0x09, x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 0xff {
                    skip = true;
                } else {
                    d = u8v(engine.mem(0x0e) - engine.mem(u16v(0x040c + x)));
                    if d != 0 && (d & 0x80) == 0 {
                        engine.set_mem(0x08, u8v(y));
                        engine.set_mem(0x09, x);
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
        let mut skip = u8v(y) == engine.mem(0xe3);
        if !skip && engine.mem(u16v(0x0401 + x)) == 0 {
            skip = true;
        }
        if !skip && (engine.mem(u16v(0x0401 + x)) & 0x80) != 0 {
            skip = true;
        }
        if !skip && (engine.mem(u16v(0x0400 + x)) & 0xf9) == 0xe1 {
            skip = true;
        }
        if !skip && (engine.mem(u16v(0x0402 + x)) & 0x20) != 0 {
            skip = true;
        }
        if !skip {
            let mut d = u8v(engine.mem(0x0a) - engine.mem(u16v(0x040e + x)));
            if !(d < 0x10) && d < 0xf1 {
                skip = true;
            }
            if !skip {
                d = u8v(engine.mem(0x0f) - engine.mem(u16v(0x040d + x)));
                if d == 0 {
                    engine.set_mem(0x08, u8v(y));
                    engine.set_mem(0x09, x);
                    r.carry = 1;
                    return;
                }
                if d < 0x02 {
                    d = u8v(engine.mem(0x0e) - engine.mem(u16v(0x040c + x)));
                    if (d & 0x80) != 0 {
                        engine.set_mem(0x08, u8v(y));
                        engine.set_mem(0x09, x);
                        r.carry = 1;
                        return;
                    }
                    skip = true;
                } else if d < 0xff {
                    skip = true;
                } else {
                    d = u8v(engine.mem(0x0e) - engine.mem(u16v(0x040c + x)));
                    if d != 0 && (d & 0x80) == 0 {
                        engine.set_mem(0x08, u8v(y));
                        engine.set_mem(0x09, x);
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
    if engine.mem(0x86) != 0 || engine.mem(0x4f) != 0 {
        engine.set_mem(0x50, 0x00);
        engine.set_mem(0x4e, 0x00);
        return;
    }

    engine.set_mem(0x0c, engine.mem(0x44));
    engine.set_mem(0x0f, engine.mem(0x44));
    engine.set_mem(0x0e, engine.mem(0x43));
    engine.set_mem(0x0d, engine.mem(0x45));
    engine.set_mem(0x0a, u8v(engine.mem(0x45) + 1));
    crate::game::resolve_room_tile_pointer(engine, r);

    if engine.mem(0x43) == 0 {
        engine.set_mem(0x50, 0x01);
        r.offset = 0x00;
        let tile_ptr = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
        if (engine.mem(u16v(tile_ptr + r.offset)) & 0x3f) == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
    }

    engine.set_mem(0x50, 0x00);
    if engine.mem(0x45) >= 0xb0 {
        engine.inc_mem(0x4e);
        return;
    }

    find_damageable_actor_overlap(engine, r);
    if cbool(r.carry) {
        if engine.mem(0x2d) >= 0x30 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let selected_slot = engine.mem(0x55);
        let selected_item = engine.mem(u16v(0x0051 + selected_slot));
        if selected_item != 0x05 || engine.mem(0x4e) == 0 {
            return resolve_player_landing_or_hazard_contact(engine, r);
        }
        let hit_slot = engine.mem(0x09);
        engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
    }

    r.offset = 0x01;
    crate::game::probe_player_solid_tile(engine, r);
    if cbool(r.carry) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    if engine.mem(0x43) == 0 {
        engine.inc_mem(0x4e);
        return;
    }

    r.offset = 0x0d;
    crate::game::probe_player_solid_tile(engine, r);
    if cbool(r.carry) {
        return resolve_player_landing_or_hazard_contact(engine, r);
    }
    engine.inc_mem(0x4e);
}

/// Converts a just-detected floor/object/hazard contact into damage, recoil,
/// hazard invulnerability, or a reset of the fall counter.
fn resolve_player_landing_or_hazard_contact(engine: &mut Engine, r: &mut RoutineContext) {
    let mut fall_frames = engine.mem(0x4e);
    if fall_frames >= engine.mem(0x5c) {
        fall_frames = u8v(fall_frames - 0x07);
        if fall_frames >= engine.mem(0x5c) {
            fall_frames = engine.mem(0x5c);
        }
        fall_frames = u8v(fall_frames - 0x01);
        engine.set_mem(0x4f, fall_frames);
        engine.set_mem(0x46, u8v(fall_frames + 0x0a));
        engine.set_mem(0x8f, 0x0a);
        crate::game::consume_health_point(engine, r);
    }
    if engine.mem(0x4e) == 0 {
        r.offset = 0x01;
        crate::game::apply_hazard_tile_contact(engine, r);
        if !cbool(r.carry) && engine.mem(0x43) != 0 {
            r.offset = 0x0d;
            crate::game::apply_hazard_tile_contact(engine, r);
        }
    }
    engine.set_mem(0x4e, 0x00);
}

/// Handles the room tile sampled at the current projected player footprint.
/// Special tiles can spend keys/magic, spawn transient objects, or launch the
/// tile-removal projectile; ordinary tiles return carry for solid terrain.
pub fn dispatch_room_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
    let tile_ptr = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
    let tile_offset = r.offset;
    let tile = engine.mem(u16v(tile_ptr + tile_offset)) & 0x3f;
    if tile == engine.mem(0x70) {
        if engine.mem(0x0491) == 0 {
            engine.set_mem(0x0b, tile_offset);
            engine.set_mem(0xed, 0xe1);
            engine.set_mem(0xee, 0x01);
            engine.set_mem(0xef, 0x01);
            engine.set_mem(0xf0, engine.mem(0x71));
            engine.set_mem(0xf3, 0x0a);
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.set_mem(0x8f, 0x06);
        }
        let v = engine.mem(0x71) & 0x3f;
        r.value = v;
        r.carry = u8v(v >= 0x30);
        return;
    }
    if tile == 0x02 {
        if engine.mem(0x0491) == 0 {
            engine.set_mem(0x0b, tile_offset);
            r.index = engine.mem(0x55);
            let item = engine.mem(u16v(0x0051 + r.index));
            r.value = item;
            if item == 0x07 {
                r.index = engine.mem(0x55);
                crate::game::consume_magic_point(engine, r);
                if cbool(r.carry) {
                    r.carry = 1;
                    return;
                }
            } else {
                crate::game::consume_key(engine, r);
                if cbool(r.carry) {
                    r.carry = 1;
                    return;
                }
            }
            engine.set_mem(0xed, 0xe1);
            engine.set_mem(0xee, 0x01);
            engine.set_mem(0xef, 0x01);
            engine.set_mem(0xf0, engine.mem(0x74));
            engine.set_mem(0xf3, 0x0f);
            crate::game::seed_object_position_from_tile_offset(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.set_mem(0x8f, 0x06);
        }
        r.carry = 1;
        return;
    }
    if tile == 0x3e {
        if (engine.mem(0x20) & 0x80) != 0 && engine.mem(0x0491) == 0 {
            engine.set_mem(0x0b, tile_offset);
            engine.set_mem(0xf4, 0x01);
            r.offset = engine.mem(0x55);
            r.index = engine.mem(u16v(0x0051 + r.offset));
            let idx = r.index;
            if idx == 1 {
                if engine.mem(0x59) != 0 {
                    let mut t = engine.mem(0x45) & 0x0f;
                    t |= engine.mem(0x43);
                    if t == 0 {
                        let x2 = u8v((engine.mem(0xfd) & 0x0f) << 1);
                        let lo = u8v(engine.mem(0x44) + engine.mem(u16v(0xfeab + x2)));
                        engine.set_mem(0x049d, lo);
                        engine.set_mem(0x0c, lo);
                        engine.set_mem(0x049c, 0x00);
                        let hi = u8v(engine.mem(0x45) + engine.mem(u16v(0xfeac + x2)));
                        engine.set_mem(0x049e, hi);
                        engine.set_mem(0x0d, hi);
                        crate::game::resolve_room_tile_pointer(engine, r);
                        r.offset = 0x00;
                        engine.set_mem(0x0b, 0x00);
                        let p = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
                        let b = engine.mem(p) & 0x3f;
                        if b == 0x3e {
                            engine.set_mem(0x0490, 0xe1);
                            engine.set_mem(0x0491, 0x01);
                            engine.set_mem(0x0492, 0x01);
                            engine.set_mem(0x0496, 0x0f);
                            crate::game::read_room_tile_action_value(engine, r);
                            engine.set_mem(0x0493, r.value);
                            crate::game::consume_magic_point(engine, r);
                            engine.set_mem(0x8f, 0x14);
                        }
                    }
                }
                r.carry = 1;
                return;
            }
            if idx == 2 {
                if (engine.mem(0xfd) & 0x0f) != 0 {
                    r.offset = 0x01;
                    crate::game::build_direction_velocity(engine, r);
                    r.offset = 0xf8;
                    let p79 = u16v(engine.mem(0x79) | (engine.mem(0x7a) << 8));
                    engine.set_mem(0xed, engine.mem(u16v(p79 + 0xf8)) & 0xfe);
                    engine.set_mem(0xee, 0x01);
                    engine.set_mem(0xef, 0x03);
                    r.offset = engine.mem(0x0b);
                    let b = engine.mem(u16v(tile_ptr + r.offset));
                    engine.set_mem(0xf0, b);
                    engine.set_mem(0xf3, 0x10);
                    crate::game::read_room_tile_action_value(engine, r);
                    engine.set_mem(u16v(tile_ptr + r.offset), r.value);
                    crate::game::seed_object_position_from_tile_offset(engine, r);
                    crate::game::redraw_room_tile_column(engine, r);
                    crate::game::update_tile_projectile_motion(engine, r);
                    engine.set_mem(0xe3, 0xff);
                    if engine.mem(0x0491) != 0 {
                        engine.set_mem(0x8f, 0x06);
                    }
                }
                engine.set_mem(0x4b, 0x00);
                engine.set_mem(0x4e, 0x00);
                r.carry = 1;
                return;
            }
            if idx == 3 {
                if engine.mem(0x59) != 0 {
                    if (engine.mem(0xfd) & 0x0f) != 0 {
                        r.offset = 0x08;
                        crate::game::build_direction_velocity(engine, r);
                        r.offset = 0xf8;
                        let p79 = u16v(engine.mem(0x79) | (engine.mem(0x7a) << 8));
                        engine.set_mem(0xed, engine.mem(u16v(p79 + 0xf8)) & 0xfe);
                        engine.set_mem(0xee, 0x01);
                        engine.set_mem(0xef, 0x03);
                        r.offset = engine.mem(0x0b);
                        let b = engine.mem(u16v(tile_ptr + r.offset));
                        engine.set_mem(0xf0, b);
                        engine.set_mem(0xf3, 0x00);
                        crate::game::read_room_tile_action_value(engine, r);
                        engine.set_mem(u16v(tile_ptr + r.offset), r.value);
                        crate::game::seed_object_position_from_tile_offset(engine, r);
                        crate::game::redraw_room_tile_column(engine, r);
                        crate::game::update_tile_projectile_motion(engine, r);
                        engine.set_mem(0xe3, 0xff);
                        if engine.mem(0xee) != 0 {
                            engine.set_mem(0x8f, 0x14);
                            crate::game::consume_magic_point(engine, r);
                        }
                        engine.set_mem(0x4b, 0x00);
                        engine.set_mem(0x4e, 0x00);
                        r.carry = 1;
                        return;
                    }
                    engine.set_mem(0x4b, 0x00);
                    engine.set_mem(0x4e, 0x00);
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
    engine.inc_mem(0x92);
    let mut y = 0x04;
    loop {
        set_frame_counter(engine, 0x05);
        for x in (0..=0x1c).rev() {
            let v = engine.mem(u16v(0x0184 + x));
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.set_mem(0x08, lo);
            engine.set_mem(
                u16v(0x0184 + x),
                if hi >= 0x10 {
                    u8v((hi - 0x10) | lo)
                } else {
                    0x0f
                },
            );
        }
        engine.shr_mem(0xa0, 1);
        engine.shr_mem(0xb0, 1);
        engine.shr_mem(0xd0, 1);
        engine.set_mem(0xb4, 0);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        y -= 1;
        if y == 0 {
            break;
        }
    }
    engine.set_mem(0x8e, 0xff);
    engine.set_mem(0x94, 0);
    engine.set_mem(0xa4, 0);
    engine.set_mem(0xc4, 0);
    engine.set_mem(0x92, 0);
}

/// Fades the room palette out while preserving active audio channel state.
pub fn fade_room_palette_out_keep_audio(engine: &mut Engine, r: &mut RoutineContext) {
    let mut y = 0x04;
    loop {
        set_frame_counter(engine, 0x05);
        for x in (0..=0x1c).rev() {
            let v = engine.mem(u16v(0x0184 + x));
            let lo = v & 0x0f;
            let hi = v & 0xf0;
            engine.set_mem(0x08, lo);
            engine.set_mem(
                u16v(0x0184 + x),
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
    engine.set_mem(0x09, v);
    loop {
        set_frame_counter(engine, 0x05);
        crate::game::build_room_palette_buffer(engine, r);
        r.index = 0x04;
        r.offset = 0x1c;
        crate::game::dim_palette_range_by_step(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
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
            engine.set_mem(u16v(0x0180 + i), 0x30);
        }
        crate::game::upload_palette_buffer(engine, r);
        set_frame_counter(engine, 0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::build_room_palette_buffer(engine, r);
        crate::game::upload_palette_buffer(engine, r);
        set_frame_counter(engine, 0x02);
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
    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine.inc_mem(0x58);
        crate::game::sync_health_hud(engine, r);
        set_prompt_state(engine, 0x16);
        set_frame_counter(engine, 0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.mem(0x58);
        if engine.mem(0x58) >= 0x63 {
            break;
        }
    }
    set_prompt_state(engine, 0x17);
    set_frame_counter(engine, 0x00);
    frame::commit_frame_work(engine, r);
    set_sprite_blink_timer(engine, saved_blink);
}

pub fn animate_magic_refill_to_cap(engine: &mut Engine, r: &mut RoutineContext) {
    // Count magic up one point at a time, sharing the same prompt/blink pacing
    // as the health refill.
    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::draw_player_sprites(engine, r);
    loop {
        engine.inc_mem(0x59);
        crate::game::sync_magic_hud(engine, r);
        set_prompt_state(engine, 0x16);
        set_frame_counter(engine, 0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        r.index = engine.mem(0x59);
        if engine.mem(0x59) >= 0x63 {
            break;
        }
    }
    set_prompt_state(engine, 0x17);
    set_frame_counter(engine, 0x00);
    frame::commit_frame_work(engine, r);
    set_sprite_blink_timer(engine, saved_blink);
}

/// Spends a key and runs the door-unlock prompt/music sequence. Carry is set
/// only when a key was available and the door event completed.
pub fn unlock_door_with_key(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::consume_key(engine, r);
    if cbool(r.carry) {
        set_prompt_state(engine, 0x06);
        r.carry = 0;
        return;
    }

    let ptr = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
    let door = engine.mem(u16v(ptr + 0x0a));
    if door < 0x08 {
        engine.set_mem(0x04a2, 0x00);
    }
    engine.set_mem(0x04a1, u8v(door + 0x02));
    engine.set_mem(0x04a0, u8v(((door << 2) & 0xff) + 0x81));
    set_prompt_state(engine, 0x1f);
    crate::game::draw_room_object_sprites(engine, r);

    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0);
    crate::game::draw_player_sprites(engine, r);

    let saved_song = engine.mem(0x8e);
    engine.set_mem(0x8e, 0x0e);
    r.value = 0x0e;
    crate::game::song_init(engine, r);

    set_frame_counter(engine, 0x78);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.set_mem(0x8e, saved_song);
    r.value = saved_song;
    crate::game::song_init(engine, r);

    set_sprite_blink_timer(engine, saved_blink);
    r.carry = 1;
}

/// Opens the in-game character-select overlay, waits for a press/release of the
/// character-select button, then restores the gameplay room.
pub fn run_character_select_overlay(engine: &mut Engine, r: &mut RoutineContext) {
    set_prompt_state(engine, 0x03);
    engine.inc_mem(0x8d);

    if engine.mem(0x2d) < 0x30 {
        push_room_checkpoint(engine, r);
        r.value = 0x08;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.set_mem(0x7b, 0x08);
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

    set_prompt_state(engine, 0x04);

    if engine.mem(0x2d) < 0x30 {
        pop_room_checkpoint(engine, r);
        fade_room_palette_out_reset_audio(engine, r);
        crate::game::clear_temporary_room_sprites(engine, r);
        r.value = engine.mem(0xfe);
        crate::game::switch_song_if_needed(engine, r);
        crate::game::prepare_room_metadata_and_palette(engine, r);
        crate::game::upload_current_room_view(engine, r);
        crate::game::draw_player_sprites(engine, r);
        crate::game::draw_room_object_sprites(engine, r);
        crate::game::refresh_scroll_register_shadows(engine, r);
        fade_room_palette_in(engine, r);
    }

    engine.dec_mem(0x8d);
}

/// Shows the read-only inventory item-list page until the player presses a
/// button, then returns to the character-selection room page.
pub fn show_inventory_item_list_screen(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x7c, 0x10);
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);

    engine.set_mem(0x0e, 0xd4);
    engine.set_mem(0x0f, 0xb4);
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

    engine.set_mem(0x7c, 0x20);
    crate::game::upload_staged_room_columns(engine, r);
    crate::game::refresh_scroll_register_shadows(engine, r);
}

/// Runs the interactive inventory item-grid editor from the character-selection
/// room.
pub fn run_inventory_item_grid_menu(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x7c, 0x30);
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

    engine.set_mem(0xf9, 0);
    engine.set_mem(0xf5, 0);
    engine.set_mem(0xf7, 0);
    engine.set_mem(0x0281, 0xf5);
    engine.set_mem(0x0291, 0xf5);
    engine.set_mem(0x0285, 0xf7);
    engine.set_mem(0x0295, 0xf7);
    engine.set_mem(0x0282, 0x00);
    engine.set_mem(0x0286, 0x00);
    engine.set_mem(0x0292, 0x00);
    engine.set_mem(0x0296, 0x00);
    crate::game::update_inventory_list_cursor_sprites(engine, r);
    crate::game::update_inventory_grid_cursor_sprites(engine, r);

    loop {
        set_frame_counter(engine, 0x01);
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
            engine.set_mem(0x7c, 0x20);
            crate::game::upload_staged_room_columns(engine, r);
            crate::game::refresh_scroll_register_shadows(engine, r);
            crate::game::restore_status_sprite_template(engine, r);
            return;
        }

        if (buttons(engine) & 0xcf) != 0 {
            set_prompt_state(engine, 0x0c);
            set_frame_counter(engine, 0x0a);
        }
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

/// Runs the special room flow used to refill resources, return carried items,
/// pick a family member, and optionally visit the inventory item pages.
pub fn run_character_select_room_flow(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.mem(0x48) != 0x10 {
        push_room_checkpoint(engine, r);
        r.value = 0x04;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_coin_cost_sprites(engine, r);
        fade_room_palette_in(engine, r);

        loop {
            walk_purchase_room_until_action_or_exit(engine, r);
            if cbool(r.carry) {
                crate::game::restore_room_from_checkpoint(engine, r);
                return;
            }
            if engine.mem(0x5a) < 0x0a {
                set_prompt_state(engine, 0x06);
                continue;
            }

            let mut x = 0x0a;
            loop {
                engine.dec_mem(0x5a);
                crate::game::sync_coin_hud(engine, r);
                set_prompt_state(engine, 0x0c);
                set_frame_counter(engine, 0x0a);
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
            engine.set_mem(0x7b, 0x08);
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

    set_player_health(engine, 0x00);
    set_player_magic(engine, 0x00);
    if engine.mem(0x40) < 0x06 {
        for y in (0..=2).rev() {
            let x = engine.mem(u16v(0x51 + y));
            if (x & 0x80) == 0 {
                engine.inc_mem(u16v(0x60 + x));
            }
            engine.set_mem(u16v(0x51 + y), 0xff);
        }
        crate::game::snapshot_inventory_state(engine, r);
    }

    push_room_checkpoint(engine, r);
    engine.set_mem(0x40, 0x06);
    r.value = 0x06;
    crate::game::enter_temporary_room_page(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    engine.set_mem(0x55, 0x03);
    crate::game::draw_status_item_sprites(engine, r);
    engine.set_mem(0x56, 0xf1);
    engine.set_mem(0x57, 0x00);
    crate::game::draw_player_sprites(engine, r);
    crate::game::restore_status_sprite_template(engine, r);
    crate::game::reset_room_object_slots(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        walk_character_select_room_until_action(engine, r);
        let hi = engine.mem(0x0a) & 0xf0;
        let mut chosen: Option<i32> = None;
        if hi == 0x50 {
            if (engine.mem(0x0f) & 0x0f) == 0x05 && engine.mem(0x37) != 0 {
                let mut x = u8v(engine.mem(0x8e) + 1);
                if x >= 0x10 {
                    x = 0x00;
                }
                engine.set_mem(0x8e, x);
                crate::game::song_init(engine, r);
                if (engine.mem(0x37) & 0x80) != 0 && buttons(engine) == 0xc3 {
                    for x in (0..=0x0d).rev() {
                        engine.set_mem(u16v(0x60 + x), 0x10);
                    }
                    engine.set_mem(0x37, 0x80);
                    engine.set_mem(0x5a, 0x80);
                    engine.set_mem(0x5b, 0x80);
                    set_prompt_state(engine, 0x1a);
                }
            }
            continue;
        } else if hi == 0x70 {
            let lo = engine.mem(0x0f) & 0x0f;
            if lo == 0x06 {
                chosen = Some(0x00);
            } else if lo == 0x08 {
                chosen = Some(0x01);
            }
        } else if hi == 0x80 {
            let lo = engine.mem(0x0f) & 0x0f;
            if lo == 0x04 {
                chosen = Some(0x02);
            } else if lo == 0x0a {
                set_prompt_state(engine, 0x03);
                show_inventory_item_list_screen(engine, r);
                continue;
            } else if lo == 0x0c {
                set_prompt_state(engine, 0x03);
                run_inventory_item_grid_menu(engine, r);
                continue;
            }
        } else if hi == 0x90 {
            let lo = engine.mem(0x0f) & 0x0f;
            if lo == 0x06 {
                chosen = Some(0x03);
            } else if lo == 0x0a {
                chosen = Some(0x04);
            }
        }

        let Some(x) = chosen else {
            continue;
        };

        engine.set_mem(0x40, x);
        r.offset = u8v((x << 2) + 0x03);
        for xi in (0..=3).rev() {
            engine.set_mem(u16v(0x5c + xi), engine.mem(u16v(0xffa7 + r.offset)));
            r.offset = u8v(r.offset - 1);
        }
        set_prompt_state(engine, 0x18);
        set_prompt_argument(engine, 0xff);
        set_frame_counter(engine, 0x04);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x05;
        flash_palette_buffer(engine, r);
        engine.set_mem(0x2c, u8v(engine.mem(0x40) + 0x38));
        engine.set_mem(0x2d, 0x3d);
        engine.set_mem(0x2e, 0x3e);
        engine.set_mem(0x2f, 0x3f);
        engine.set_mem(0x56, 0x0d);
        engine.set_mem(0x57, 0x00);
        engine.and_mem(0x45, 0xf0);
        engine.set_mem(0x43, 0x04);
        crate::game::clear_gameplay_object_sprites(engine, r);
        crate::game::draw_player_sprites(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x05;
        flash_palette_buffer(engine, r);
        set_frame_counter(engine, 0x78);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        fade_room_palette_out_reset_audio(engine, r);
        engine.set_mem(0x56, 0x08);
        engine.set_mem(0x57, 0x00);
        set_player_health(engine, 0x63);
        set_player_magic(engine, 0x63);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        engine.set_mem(0x55, 0x02);
        crate::game::draw_status_item_sprites(engine, r);
        r.value = 0x08;
        crate::game::enter_temporary_room_page(engine, r);
        crate::game::draw_carried_item_sprites(engine, r);
        crate::game::upload_inventory_count_tiles(engine, r);
        crate::game::upload_equipped_item_stat_tiles(engine, r);
        engine.set_mem(0x7b, 0x08);
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

    let s80 = engine.mem(0x80);
    let s81 = engine.mem(0x81);
    let s82 = engine.mem(0x82);
    let s83 = engine.mem(0x83);
    r.value = engine.mem(0x47);
    crate::game::enter_temporary_room_page(engine, r);
    engine.set_mem(0x83, s83);
    engine.set_mem(0x82, s82);
    engine.set_mem(0x81, s81);
    engine.set_mem(0x80, s80);

    crate::game::draw_shop_item_sprites(engine, r);
    crate::game::upload_shop_price_tiles(engine, r);
    crate::game::draw_coin_cost_sprites(engine, r);
    fade_room_palette_in(engine, r);

    loop {
        walk_purchase_room_until_action_or_exit(engine, r);
        if cbool(r.carry) {
            crate::game::restore_room_from_checkpoint(engine, r);
            return;
        }

        let nib = engine.mem(0x44) & 0x0f;
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

        let item = engine.mem(u16v(0x80 + x));
        if (item & 0x80) != 0 {
            set_prompt_state(engine, 0x06);
        } else {
            let price = engine.mem(u16v(0x81 + x));
            r.value = price;
            crate::game::spend_coins(engine, r);
            if cbool(r.carry) {
                engine.set_mem(u16v(0x80 + x), 0xff);
                crate::game::draw_shop_item_sprites(engine, r);
                engine.inc_mem(u16v(0x60 + item));
                set_prompt_state(engine, 0x10);
            } else {
                if item == 0x0d && engine.mem(0x37) != 0 {
                    engine.set_mem(0x61, 0x01);
                }
                set_prompt_state(engine, 0x06);
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
        set_frame_counter(engine, 0x01);
        let buttons = frame::read_buttons(engine, r);
        if (buttons & 0x80) != 0 {
            r.value = 0x80;
            return;
        }

        r.value = buttons & 0x0f;
        r.offset = 0x01;
        crate::game::build_input_movement_delta(engine, r);
        crate::game::project_player_position(engine, r);

        let ty = engine.mem(0x0a);
        if ty >= 0x30 && ty < 0xa1 {
            let lo = engine.mem(0x0f) & 0x0f;
            if lo >= 0x02 {
                let store = lo < 0x0d || engine.mem(0x0e) == 0;
                if store {
                    engine.set_mem(0x43, engine.mem(0x0e));
                    engine.set_mem(0x44, engine.mem(0x0f));
                    engine.set_mem(0x45, engine.mem(0x0a));
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
    if (engine.mem(0xee) & 0x7f) == 0 {
        set_prompt_state(engine, 0x18);
        set_prompt_argument(engine, 0xff);
        r.index = 0x03;
        flash_palette_buffer(engine, r);

        set_frame_counter(engine, 0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        flash_palette_buffer(engine, r);

        set_frame_counter(engine, 0x05);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        flash_palette_buffer(engine, r);

        engine.inc_mem(0xee);
        set_prompt_state(engine, 0x02);
        engine.set_mem(0xf1, 0x0f);
        engine.set_mem(0xf5, 0x00);
        engine.set_mem(0xf6, 0x00);
        engine.set_mem(0xf0, 0x00);
        engine.set_mem(0xfc, engine.mem(0xfb));
    }

    if engine.mem(0xf0) == 0 {
        engine.dec_mem(0xf1);
        if engine.mem(0xf1) == 0 {
            engine.or_mem(0xef, 0x80);
            engine.set_mem(0xf0, 0x01);
            return;
        }
        let a = u8v(((engine.mem(0xf1) >> 2) ^ 0xff) + 1);
        engine.set_mem(0xf7, a);
        crate::game::project_actor_position(engine, r);
        crate::game::check_position_out_of_bounds(engine, r);
        if cbool(r.carry) {
            engine.or_mem(0xef, 0x80);
            engine.set_mem(0xf0, 0x01);
            return;
        }
        engine.set_mem(0xfb, engine.mem(0x0a));
        return;
    }

    engine.inc_mem(0xf0);
    engine.set_mem(0xf7, u8v((engine.mem(0xf0) >> 2) + 1));
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if cbool(r.carry) {
        engine.set_mem(0xee, 0x00);
        engine.set_mem(0xf3, 0xf0);
        engine.set_mem(0xeb, 0x01);
        return;
    }
    engine.set_mem(0xfb, engine.mem(0x0a));
}

/// Walks a purchase/refill room until the player presses action or reaches the
/// exit tile. Carry set means exit; carry clear means action on the current
/// selectable tile.
pub fn walk_purchase_room_until_action_or_exit(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        set_frame_counter(engine, 0x01);
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

        let ty = engine.mem(0x0a);
        if ty >= 0xa1 {
            r.value = ty;
            r.carry = 1;
            return;
        }
        if ty >= 0x8c {
            let lo = engine.mem(0x0f) & 0x0f;
            if lo >= 0x02 && lo < 0x0d {
                engine.set_mem(0x43, engine.mem(0x0e));
                engine.set_mem(0x44, engine.mem(0x0f));
                engine.set_mem(0x45, engine.mem(0x0a));
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
        set_frame_counter(engine, 0x01);
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

        let ty = engine.mem(0x0a);
        if ty >= 0xa1 {
            r.value = ty;
            r.carry = 1;
            return;
        }
        if ty >= 0x20 {
            let lo = engine.mem(0x0f) & 0x0f;
            let mut store = false;
            if lo >= 0x01 {
                if lo < 0x0f {
                    store = true;
                } else if engine.mem(0x0e) == 0 {
                    store = true;
                }
            }
            if store {
                engine.set_mem(0x43, engine.mem(0x0e));
                engine.set_mem(0x44, engine.mem(0x0f));
                engine.set_mem(0x45, engine.mem(0x0a));
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
        if cbool(r.carry) {
            let e = engine.mem(0x55);
            if engine.mem(u16v(0x51 + e)) == 0x0d {
                engine.set_mem(0x55, 0x03);
                crate::game::draw_status_item_sprites(engine, r);
            }
            return;
        }
        let mut x = 0xff;
        let py = engine.mem(0x45);
        let flow_0441 = if py >= 0x58 {
            true
        } else {
            x = if py < 0x38 { 0x00 } else { 0x08 };
            engine.set_mem(0x08, x);
            x = u8v((engine.mem(0x44) >> 1) | engine.mem(0x08));
            if engine.mem(u16v(0x60 + x)) != 0 {
                r.value = x;
                crate::game::load_family_item_permission_bits(engine, r);
                if cbool(r.carry) {
                    engine.dec_mem(u16v(0x60 + x));
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if !flow_0441 {
            engine.set_mem(0x8f, 0x06);
            continue;
        }
        engine.set_mem(0x08, x);
        let ci0 = engine.mem(0x51);
        if (ci0 & 0x80) == 0 {
            engine.inc_mem(u16v(0x60 + ci0));
        }
        engine.set_mem(0x51, engine.mem(0x52));
        engine.set_mem(0x52, engine.mem(0x53));
        engine.set_mem(0x53, engine.mem(0x08));
        engine.set_mem(0x8f, 0x12);
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
    engine.set_mem(0xfe, engine.mem(0x8e));
    if engine.room_ckpt_sp < engine.room_ckpt_stack.len() {
        let c = [
            engine.mem(0x43) as u8,
            engine.mem(0x44) as u8,
            engine.mem(0x45) as u8,
            engine.mem(0x7b) as u8,
            engine.mem(0x7c) as u8,
            engine.mem(0x47) as u8,
            engine.mem(0x48) as u8,
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
        engine.set_mem(0x43, c[0] as i32);
        engine.set_mem(0x44, c[1] as i32);
        engine.set_mem(0x45, c[2] as i32);
        engine.set_mem(0x7b, c[3] as i32);
        engine.set_mem(0x7c, c[4] as i32);
        engine.set_mem(0x47, c[5] as i32);
        engine.set_mem(0x48, c[6] as i32);
    }
}

/// Runs the high-bit defeated-actor reward drop sequence. The actor rises,
/// falls back into the playfield, then turns into a pickup chosen from current
/// resource needs and the drop table.
pub fn tick_defeated_actor_reward_drop(engine: &mut Engine, r: &mut RoutineContext) {
    const DROP_ITEM_TABLE: [i32; 9] = [0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x05, 0x06, 0x07];
    if (engine.mem(0xee) & 0x7f) == 0 {
        engine.inc_mem(0xee);
        engine.set_mem(0x8f, 0x0e);
        engine.set_mem(0xf1, 0x08);
        engine.set_mem(0xf5, 0x00);
        engine.set_mem(0xf6, 0x00);
        engine.set_mem(0xf0, 0x00);
        engine.set_mem(0xfc, engine.mem(0xfb));
        let ptr = u16v(engine.mem(0xe7) | (engine.mem(0xe8) << 8));
        engine.set_mem(0xed, engine.mem(u16v(ptr + 6)));
        engine.and_mem(0xef, 0x03);
    }
    if engine.mem(0xf0) == 0 {
        engine.dec_mem(0xf1);
        if engine.mem(0xf1) != 0 {
            engine.set_mem(0xf7, u8v(0 - engine.mem(0xf1)));
            crate::game::project_actor_position(engine, r);
            crate::game::check_position_out_of_bounds(engine, r);
            if !cbool(r.carry) {
                engine.set_mem(0xfb, engine.mem(0x0a));
                return;
            }
        }
        engine.or_mem(0xef, 0x80);
        engine.set_mem(0xf0, 0x01);
        return;
    }
    engine.inc_mem(0xf0);
    engine.set_mem(0xf7, u8v((engine.mem(0xf0) >> 1) + 2));
    crate::game::project_actor_position(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if !cbool(r.carry) {
        engine.set_mem(0xfb, engine.mem(0x0a));
        return;
    }
    let mut x = 0x00;
    if engine.mem(0x58) < 0x14 {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 0x01;
    if engine.mem(0x59) < 0x1e {
        item_spawn_setup(engine, r, x);
        return;
    }
    x = 0x04;
    if engine.mem(0x5b) < 0x02 {
        item_spawn_setup(engine, r, x);
        return;
    }
    r.value = 0x14;
    crate::game::rng_update(engine, r);
    if r.value >= 0x09 {
        x = 0x00;
        if engine.mem(0x58) < engine.mem(0x59) {
            if engine.mem(0x58) < engine.mem(0x5a) {
                item_spawn_setup(engine, r, x);
                return;
            }
            x = 0x02;
            item_spawn_setup(engine, r, x);
            return;
        }
        x = 0x01;
        if engine.mem(0x59) < engine.mem(0x5a) {
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
