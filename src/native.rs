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

pub fn queue_ppu_job_and_wait(engine: &mut Engine, r: &mut RoutineContext) {
    frame::wait_for_ppu_job_idle(engine, r);
    engine.set_mem(0x28, r.value);
    frame::wait_for_ppu_job_idle(engine, r);
}

pub fn routine_0029(engine: &mut Engine, r: &mut RoutineContext) {
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
            crate::game::routine_0061(engine, r);
            farcall_0c0d(engine, r, 0x07, 0xb3, routine_0049);
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
            farcall_0c0d(engine, r, 0xeb, 0xa2, routine_0001);
            loop {
                frame::read_buttons(engine, r);
                farcall_0c0d(engine, r, 0xbc, 0xab, crate::game::routine_0021);
                farcall_0c0d(engine, r, 0xe6, 0xa5, crate::game::routine_0005);
                farcall_0c0d(engine, r, 0x5d, 0xa7, crate::game::routine_0014);
                farcall_0c0d(engine, r, 0xe3, 0xa3, routine_0002);
                if player_health(engine) != 0 {
                    break;
                }
            }

            engine.set_mem(0x44, engine.mem(0x43) >> 4);
            engine.and_mem(0x43, 0x0f);
            engine.set_mem(0x0200, 0xef);
            set_sprite_blink_timer(engine, 0x00);
            crate::game::routine_0061(engine, r);
            farcall_0c0d(engine, r, 0x07, 0xb3, routine_0049);
            r.index = u8v(r.index - 1);
            crate::game::main_init(engine, r);
            return;
        }

        crate::game::update_player_projectiles(engine, r);
        crate::game::update_room_actors(engine, r);
        crate::game::update_tile_projectile(engine, r);
        crate::game::routine_0059(engine, r);
        let saved_c = r.carry;
        crate::game::routine_0061(engine, r);
        crate::game::routine_0063(engine, r);
        r.carry = saved_c;
        if !cbool(r.carry) && engine.mem(0x7e) != engine.mem(0x7c) {
            engine.inc_mem(0x3d);
        }

        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

pub fn routine_0001(engine: &mut Engine, r: &mut RoutineContext) {
    set_prompt_state(engine, 0x18);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::routine_0061(engine, r);

    r.index = 0x02;
    farcall_cce4(engine, r, 0x40, 0xc5, routine_0074);
    crate::game::routine_0128(engine, r);
    crate::game::routine_0063(engine, r);
    r.index = 0x03;
    farcall_cce4(engine, r, 0x40, 0xc5, routine_0074);
    routine_0004(engine, r);

    set_prompt_state(engine, 0x20);
    set_frame_counter(engine, 0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    engine.set_mem(0x48, 0x13);
    engine.set_mem(0x47, 0x02);
    farcall_cce4(engine, r, 0xf2, 0xc8, crate::game::scene_assemble);
    crate::game::routine_0066(engine, r);

    engine.set_mem(0x0200, 0xef);
    engine.set_mem(0x1e, 0x22);
    engine.set_mem(0x7b, 0x00);
    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x7c, 0x10);
    farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::routine_0077);
    r.index = 0x04;
    farcall_cce4(engine, r, 0x40, 0xc5, routine_0074);
    engine.set_mem(0x7c, 0x00);
    farcall_cce4(engine, r, 0x6c, 0xc7, crate::game::routine_0080);
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
    farcall_cce4(engine, r, 0x40, 0xc5, routine_0074);
    farcall_cce4(engine, r, 0xc7, 0xc1, crate::game::routine_0060);

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

fn routine_0002_cutscene(engine: &mut Engine, r: &mut RoutineContext) {
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
    crate::game::routine_0128(engine, r);
    routine_0045(engine, r);
    routine_0069(engine, r);
    crate::game::routine_0066(engine, r);
    crate::game::routine_0065(engine, r);
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
    crate::game::routine_0076(engine, r);
    farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::routine_0077);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    crate::game::sync_coin_hud(engine, r);
    crate::game::sync_key_hud(engine, r);
    crate::game::routine_0060(engine, r);
    crate::game::routine_0127(engine, r);
    crate::game::routine_0061(engine, r);
    crate::game::routine_0062(engine, r);
    crate::game::routine_0063(engine, r);
    engine.set_mem(0x40, 0x07);
    farcall_cce4(engine, r, 0x92, 0xc4, routine_0070);
    set_countdown_timer(engine, 0x05);
    while countdown_timer_active(engine) {
        routine_0020(engine, r);
    }

    loop {
        if engine.mem(0x45) == 0xa0 {
            break;
        }
        engine.dec_mem(0x45);
        routine_0020(engine, r);
        routine_0020(engine, r);
        if engine.mem(0x45) == 0xa0 {
            break;
        }
        engine.dec_mem(0x45);
        engine.xor_mem(0x57, 0x40);
        crate::game::routine_0061(engine, r);
        routine_0020(engine, r);
        routine_0020(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }

    engine.set_mem(0x56, 0x0d);
    crate::game::routine_0061(engine, r);
    set_countdown_timer(engine, 0x03);
    while countdown_timer_active(engine) {
        routine_0020(engine, r);
    }

    loop {
        set_frame_counter(engine, 0x01);
        engine.set_mem(0x7e, engine.mem(0x7c));
        set_buttons(engine, 0x01);
        farcall_cce4(engine, r, 0x2b, 0xd4, crate::game::game_update);
        farcall_cce4(engine, r, 0x5d, 0xc1, crate::game::routine_0059);
        crate::game::routine_0019(engine, r);
        crate::game::routine_0061(engine, r);
        crate::game::routine_0063(engine, r);
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
            routine_0020(engine, r);
        }
    }

    routine_0039(engine, r);
}

pub fn routine_0002(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0xe5, 0x00);
    engine.set_mem(0xe6, 0x04);
    crate::game::load_object_slot_scratch(engine, r);
    if engine.mem(0x00f2) == 0 {
        routine_0002_cutscene(engine, r);
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

pub fn routine_0004(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0020(engine: &mut Engine, r: &mut RoutineContext) {
    crate::game::routine_0061(engine, r);
    crate::game::routine_0063(engine, r);
    set_frame_counter(engine, 0x01);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

pub fn routine_0034(engine: &mut Engine, r: &mut RoutineContext) {
    routine_0069(engine, r);
    farcall_cce4(engine, r, 0x8b, 0xc3, crate::game::routine_0066);
    crate::game::routine_0076(engine, r);
    crate::game::routine_0065(engine, r);
    crate::game::routine_0053(engine, r);
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

pub fn routine_0039(engine: &mut Engine, r: &mut RoutineContext) {
    engine.inc_mem(0x92);
    routine_0045(engine, r);
    routine_0069(engine, r);
    crate::game::routine_0066(engine, r);
    crate::game::routine_0047(engine, r);
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
    crate::game::routine_0046(engine, r);

    engine.set_mem(0x18, 0x40);
    engine.set_mem(0x19, 0x01);
    engine.set_mem(0x1a, 0x20);
    engine.set_mem(0x0c, 0x9c);
    engine.set_mem(0x0d, 0xb7);

    loop {
        crate::game::routine_0043(engine, r);
        crate::game::routine_0040(engine, r);
        if cbool(r.carry) {
            break;
        }
        crate::game::routine_0043(engine, r);
        crate::game::routine_0041(engine, r);
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
        crate::game::routine_0075(engine, r);
        set_frame_counter(engine, 0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::routine_0046(engine, r);
        crate::game::routine_0075(engine, r);
        set_frame_counter(engine, 0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        cnt = u8v(cnt - 1);
        if cnt == 0 {
            break;
        }
    }
}

pub fn routine_0033(engine: &mut Engine, r: &mut RoutineContext) {
    'restart: loop {
        crate::game::routine_0053(engine, r);
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
        farcall_cce4(engine, r, 0x69, 0xc5, crate::game::routine_0075);
        crate::game::routine_0128(engine, r);
        crate::game::routine_0065(engine, r);
        crate::game::routine_0036(engine, r);
        engine.set_mem(0x2c, 0x15);
        engine.set_mem(0x8e, 0x09);
        crate::game::song_init(engine, r);
        crate::game::routine_0054(engine, r);
        engine.set_mem(0x24, 0x1e);
        engine.device_write(0x2001, 0x1e);
        set_frame_counter(engine, 0x78);
        frame::wait_for_frame_counter(engine, r);
        routine_0055(engine, r);
        set_countdown_timer(engine, 0x14);

        loop {
            set_frame_counter(engine, 0x01);
            let pad = frame::read_buttons(engine, r);
            if pad == 0xff {
                set_prompt_state(engine, 0x1a);
                engine.set_mem(0x37, 0x1a);
            }
            if (buttons(engine) & 0x10) != 0 {
                routine_0034(engine, r);
                return;
            }
            if button_chord(engine) == 0x83 {
                routine_0039(engine, r);
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

        routine_0069(engine, r);
        crate::game::routine_0065(engine, r);
        crate::game::routine_0128(engine, r);
        crate::game::routine_0037(engine, r);
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
        farcall_cce4(engine, r, 0x8b, 0xc3, crate::game::routine_0066);
        crate::game::routine_0076(engine, r);
        farcall_cce4(engine, r, 0xcb, 0xc5, crate::game::routine_0077);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        crate::game::sync_coin_hud(engine, r);
        crate::game::sync_key_hud(engine, r);
        crate::game::routine_0060(engine, r);
        crate::game::routine_0127(engine, r);
        crate::game::routine_0061(engine, r);
        crate::game::routine_0062(engine, r);
        farcall_cce4(engine, r, 0x92, 0xc4, routine_0070);
        set_countdown_timer(engine, 0x0a);

        loop {
            set_frame_counter(engine, 0x01);
            engine.set_mem(0x7e, engine.mem(0x7c));
            crate::game::routine_0038(engine, r);
            frame::read_buttons(engine, r);
            if (buttons(engine) & 0x10) != 0 {
                routine_0034(engine, r);
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
                crate::game::routine_0035(engine, r);
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
            farcall_cce4(engine, r, 0x5d, 0xc1, crate::game::routine_0059);
            crate::game::routine_0061(engine, r);
            crate::game::routine_0063(engine, r);
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

        routine_0069(engine, r);
        continue 'restart;
    }
}

pub fn routine_0045(engine: &mut Engine, r: &mut RoutineContext) {
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
            crate::game::routine_0063(engine, r);
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

pub fn routine_0049(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_song = engine.mem(0x8e);

    engine.inc_mem(0x8d);
    crate::game::routine_0127(engine, r);
    r.index = 0x35;
    r.offset = 0x00;
    routine_0050(engine, r);

    set_frame_counter(engine, 0x3c);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    r.value = 0x08;
    crate::game::routine_0123(engine, r);
    engine.dec_mem(0x8d);

    engine.set_mem(0x0a, 0x05);
    loop {
        r.index = 0x0d;
        r.offset = 0x00;
        routine_0050(engine, r);
        r.index = 0x01;
        r.offset = 0x00;
        routine_0050(engine, r);
        r.index = 0x09;
        r.offset = 0x00;
        routine_0050(engine, r);
        r.index = 0x01;
        r.offset = 0x40;
        routine_0050(engine, r);
        engine.dec_mem(0x0a);
        if engine.mem(0x0a) == 0 {
            break;
        }
    }

    set_frame_counter(engine, 0x01);
    engine.set_mem(0x56, 0x31);
    crate::game::routine_0061(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);

    let mut use_game_over_screen = engine.mem(0xec) != 0;
    if !use_game_over_screen {
        if (engine.mem(0x37) & 0x80) != 0 {
            let x = engine.mem(0x55);
            if engine.mem(u16v(0x51 + x)) == 0x0c {
                engine.set_mem(u16v(0x51 + x), 0xff);
                crate::game::routine_0062(engine, r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            engine.inc_mem(0x37);
        }

        if !use_game_over_screen {
            routine_0133(engine, r);
            engine.set_mem(0x56, 0x19);
            crate::game::read_debounced_buttons(engine, r);
            r.value = saved_song;
            crate::game::routine_0123(engine, r);
            r.index = 0x00;
            return;
        }
    }

    routine_0069(engine, r);
    engine.set_mem(0xec, 0x00);
    engine.set_mem(0x3e, 0x00);
    engine.set_mem(0x3f, 0x80);
    crate::game::routine_0066(engine, r);
    crate::game::routine_0128(engine, r);
    crate::game::routine_0063(engine, r);
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
    crate::game::routine_0065(engine, r);
    crate::game::routine_0061(engine, r);
    farcall_cce4(engine, r, 0xe0, 0xc4, routine_0072);

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
        routine_0069(engine, r);
        set_frame_counter(engine, 0x78);
        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);
        r.index = 0x02;
        return;
    }

    crate::game::routine_0130(engine, r);
    engine.set_mem(0x51, 0xff);
    engine.set_mem(0x52, 0xff);
    engine.set_mem(0x53, 0xff);
    engine.set_mem(0x55, 0x03);
    engine.set_mem(0x40, 0x06);
    engine.set_mem(0x47, 0x03);
    engine.set_mem(0x48, 0x10);
    routine_0069(engine, r);
    engine.set_mem(0x8e, 0x02);
    crate::game::routine_0066(engine, r);
    crate::game::routine_0076(engine, r);
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
    farcall_cce4(engine, r, 0xb4, 0xc4, routine_0071);
    r.index = 0x01;
}

pub fn routine_0050(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x56, r.index);
    engine.set_mem(0x57, r.offset);
    set_frame_counter(engine, 0x08);
    crate::game::routine_0061(engine, r);
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

pub fn routine_0055(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x09, 0x40);
    loop {
        set_frame_counter(engine, 0x05);
        crate::game::routine_0057(engine, r);
        r.index = 0x00;
        r.offset = 0x20;
        crate::game::routine_0056(engine, r);

        enter_return_home(engine, 0x35, 0xc1);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        leave_return_home(engine);

        engine.sub_mem(0x09, 0x10);
        if (engine.mem(0x09) & 0x80) != 0 {
            break;
        }
    }
    crate::game::routine_0075(engine, r);
}

pub fn routine_0058(engine: &mut Engine, r: &mut RoutineContext) {
    frame::commit_frame_work(engine, r);
    frame::wait_for_frame_counter(engine, r);
}

pub fn routine_0069(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0071(engine: &mut Engine, r: &mut RoutineContext) {
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
        crate::game::routine_0073(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
        if (v & 0x80) != 0 {
            break;
        }
    }
    crate::game::routine_0075(engine, r);
}

pub fn routine_0072(engine: &mut Engine, r: &mut RoutineContext) {
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
        crate::game::routine_0073(engine, r);
        r.index = 0x10;
        r.offset = 0x04;
        crate::game::routine_0073(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
        if (v & 0x80) != 0 {
            break;
        }
    }
    crate::game::routine_0075(engine, r);
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

pub fn routine_0109(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0110(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0163(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.mem(0x86) == 0 && engine.mem(0x4f) == 0 {
        engine.set_mem(0x0c, engine.mem(0x44));
        engine.set_mem(0x0f, engine.mem(0x44));
        engine.set_mem(0x0e, engine.mem(0x43));
        engine.set_mem(0x0d, engine.mem(0x45));
        engine.set_mem(0x0a, u8v(engine.mem(0x45) + 1));
        crate::game::resolve_room_tile_pointer(engine, r);
        if engine.mem(0x43) == 0 {
            engine.set_mem(0x50, 0x01);
            r.offset = 0x00;
            let ptr = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
            if (engine.mem(u16v(ptr + r.offset)) & 0x3f) == 0 {
                return routine_0163_dc4d(engine, r);
            }
        }
        engine.set_mem(0x50, 0x00);
        if engine.mem(0x45) >= 0xb0 {
            engine.inc_mem(0x4e);
            return;
        }
        routine_0109(engine, r);
        if cbool(r.carry) {
            if engine.mem(0x2d) >= 0x30 {
                return routine_0163_dc4d(engine, r);
            }
            let y = engine.mem(0x55);
            let x = engine.mem(u16v(0x0051 + y));
            if x != 0x05 || engine.mem(0x4e) == 0 {
                return routine_0163_dc4d(engine, r);
            }
            let hit_x = engine.mem(0x09);
            engine.set_mem(u16v(0x0401 + hit_x), 0x80);
        }
        r.offset = 0x01;
        crate::game::routine_0166(engine, r);
        if cbool(r.carry) {
            return routine_0163_dc4d(engine, r);
        }
        if engine.mem(0x43) == 0 {
            engine.inc_mem(0x4e);
            return;
        }
        r.offset = 0x0d;
        crate::game::routine_0166(engine, r);
        if cbool(r.carry) {
            return routine_0163_dc4d(engine, r);
        }
        engine.inc_mem(0x4e);
        return;
    }
    engine.set_mem(0x50, 0x00);
    engine.set_mem(0x4e, 0x00);
}

fn routine_0163_dc4d(engine: &mut Engine, r: &mut RoutineContext) {
    let mut v = engine.mem(0x4e);
    if v >= engine.mem(0x5c) {
        v = u8v(v - 0x07);
        if v >= engine.mem(0x5c) {
            v = engine.mem(0x5c);
        }
        v = u8v(v - 0x01);
        engine.set_mem(0x4f, v);
        engine.set_mem(0x46, u8v(v + 0x0a));
        engine.set_mem(0x8f, 0x0a);
        crate::game::consume_health_point(engine, r);
    }
    if engine.mem(0x4e) == 0 {
        r.offset = 0x01;
        crate::game::routine_0165(engine, r);
        if !cbool(r.carry) && engine.mem(0x43) != 0 {
            r.offset = 0x0d;
            crate::game::routine_0165(engine, r);
        }
    }
    engine.set_mem(0x4e, 0x00);
}

pub fn routine_0169(engine: &mut Engine, r: &mut RoutineContext) {
    let ptr = u16v(engine.mem(0x0c) | (engine.mem(0x0d) << 8));
    let y = r.offset;
    let tile = engine.mem(u16v(ptr + y)) & 0x3f;
    if tile == engine.mem(0x70) {
        if engine.mem(0x0491) == 0 {
            engine.set_mem(0x0b, y);
            engine.set_mem(0xed, 0xe1);
            engine.set_mem(0xee, 0x01);
            engine.set_mem(0xef, 0x01);
            engine.set_mem(0xf0, engine.mem(0x71));
            engine.set_mem(0xf3, 0x0a);
            crate::game::routine_0170(engine, r);
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
            engine.set_mem(0x0b, y);
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
            crate::game::routine_0170(engine, r);
            crate::game::store_object_slot_scratch(engine, r);
            engine.set_mem(0x8f, 0x06);
        }
        r.carry = 1;
        return;
    }
    if tile == 0x3e {
        if (engine.mem(0x20) & 0x80) != 0 && engine.mem(0x0491) == 0 {
            engine.set_mem(0x0b, y);
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
                            crate::game::routine_0172(engine, r);
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
                    let b = engine.mem(u16v(ptr + r.offset));
                    engine.set_mem(0xf0, b);
                    engine.set_mem(0xf3, 0x10);
                    crate::game::routine_0172(engine, r);
                    engine.set_mem(u16v(ptr + r.offset), r.value);
                    crate::game::routine_0170(engine, r);
                    crate::game::routine_0171(engine, r);
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
                        let b = engine.mem(u16v(ptr + r.offset));
                        engine.set_mem(0xf0, b);
                        engine.set_mem(0xf3, 0x00);
                        crate::game::routine_0172(engine, r);
                        engine.set_mem(u16v(ptr + r.offset), r.value);
                        crate::game::routine_0170(engine, r);
                        crate::game::routine_0171(engine, r);
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

pub fn routine_0067(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0068(engine: &mut Engine, r: &mut RoutineContext) {
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

pub fn routine_0070(engine: &mut Engine, r: &mut RoutineContext) {
    let mut v = 0x40;
    engine.set_mem(0x09, v);
    loop {
        set_frame_counter(engine, 0x05);
        crate::game::routine_0087(engine, r);
        r.index = 0x04;
        r.offset = 0x1c;
        crate::game::routine_0073(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        v = u8v(engine.mem(0x09) - 0x10);
        engine.set_mem(0x09, v);
        if (v & 0x80) != 0 {
            break;
        }
    }
    crate::game::routine_0075(engine, r);
}

pub fn routine_0074(engine: &mut Engine, r: &mut RoutineContext) {
    let mut x = r.index;
    loop {
        for i in (0..=0x1f).rev() {
            engine.set_mem(u16v(0x0180 + i), 0x30);
        }
        crate::game::routine_0075(engine, r);
        set_frame_counter(engine, 0x01);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
        crate::game::routine_0087(engine, r);
        crate::game::routine_0075(engine, r);
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

pub fn routine_0133(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::routine_0061(engine, r);
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

pub fn routine_0134(engine: &mut Engine, r: &mut RoutineContext) {
    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0x00);
    crate::game::routine_0061(engine, r);
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

pub fn routine_0148(engine: &mut Engine, r: &mut RoutineContext) {
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
    crate::game::routine_0063(engine, r);

    let saved_blink = sprite_blink_timer(engine);
    set_sprite_blink_timer(engine, 0);
    crate::game::routine_0061(engine, r);

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

pub fn routine_0174(engine: &mut Engine, r: &mut RoutineContext) {
    set_prompt_state(engine, 0x03);
    engine.inc_mem(0x8d);

    if engine.mem(0x2d) < 0x30 {
        routine_0193(engine, r);
        r.value = 0x08;
        crate::game::routine_0195(engine, r);
        crate::game::routine_0197(engine, r);
        crate::game::routine_0117(engine, r);
        crate::game::routine_0119(engine, r);
        engine.set_mem(0x7b, 0x08);
        crate::game::routine_0060(engine, r);
        crate::game::routine_0061(engine, r);
        routine_0070(engine, r);
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
        routine_0194(engine, r);
        routine_0067(engine, r);
        crate::game::routine_0200(engine, r);
        r.value = engine.mem(0xfe);
        crate::game::routine_0123(engine, r);
        crate::game::routine_0084(engine, r);
        crate::game::routine_0077(engine, r);
        crate::game::routine_0061(engine, r);
        crate::game::routine_0063(engine, r);
        crate::game::routine_0060(engine, r);
        routine_0070(engine, r);
    }

    engine.dec_mem(0x8d);
}

pub fn routine_0176(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x7c, 0x10);
    crate::game::routine_0081(engine, r);
    crate::game::routine_0060(engine, r);

    engine.set_mem(0x0e, 0xd4);
    engine.set_mem(0x0f, 0xb4);
    crate::game::routine_0051(engine, r);
    crate::game::routine_0131(engine, r);

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
    crate::game::routine_0081(engine, r);
    crate::game::routine_0060(engine, r);
}

pub fn routine_0177(engine: &mut Engine, r: &mut RoutineContext) {
    engine.set_mem(0x7c, 0x30);
    crate::game::routine_0081(engine, r);
    crate::game::routine_0132(engine, r);
    crate::game::routine_0131(engine, r);
    crate::game::routine_0060(engine, r);

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
    crate::game::routine_0184(engine, r);
    crate::game::routine_0185(engine, r);

    loop {
        set_frame_counter(engine, 0x01);
        let b = frame::read_buttons(engine, r);
        r.value = b;

        if (b & 0x80) != 0 {
            crate::game::routine_0179(engine, r);
            crate::game::routine_0131(engine, r);
        } else if (b & 0x40) != 0 {
        } else if (b & 0x01) != 0 {
            crate::game::routine_0180(engine, r);
        } else if (b & 0x02) != 0 {
            crate::game::routine_0181(engine, r);
        } else if (b & 0x04) != 0 {
            crate::game::routine_0183(engine, r);
        } else if (b & 0x08) != 0 {
            crate::game::routine_0182(engine, r);
            crate::game::routine_0131(engine, r);
        } else if (b & 0x10) != 0 {
            crate::game::routine_0178(engine, r);
        } else if (b & 0x20) != 0 {
            engine.set_mem(0x7c, 0x20);
            crate::game::routine_0081(engine, r);
            crate::game::routine_0060(engine, r);
            crate::game::routine_0201(engine, r);
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

pub fn routine_0175(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.mem(0x48) != 0x10 {
        routine_0193(engine, r);
        r.value = 0x04;
        crate::game::routine_0195(engine, r);
        crate::game::routine_0199(engine, r);
        routine_0070(engine, r);

        loop {
            routine_0189(engine, r);
            if cbool(r.carry) {
                crate::game::routine_0192(engine, r);
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
            routine_0068(engine, r);
            routine_0133(engine, r);
            routine_0134(engine, r);
            r.value = 0x08;
            crate::game::routine_0196(engine, r);
            crate::game::routine_0197(engine, r);
            crate::game::routine_0117(engine, r);
            crate::game::routine_0119(engine, r);
            engine.set_mem(0x7b, 0x08);
            crate::game::routine_0060(engine, r);
            crate::game::routine_0061(engine, r);
            routine_0070(engine, r);
            routine_0188(engine, r);
            r.value = 0x04;
            crate::game::routine_0196(engine, r);
            crate::game::routine_0200(engine, r);
            crate::game::routine_0199(engine, r);
            routine_0070(engine, r);
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
        crate::game::routine_0129(engine, r);
    }

    routine_0193(engine, r);
    engine.set_mem(0x40, 0x06);
    r.value = 0x06;
    crate::game::routine_0195(engine, r);
    crate::game::sync_health_hud(engine, r);
    crate::game::sync_magic_hud(engine, r);
    engine.set_mem(0x55, 0x03);
    crate::game::routine_0062(engine, r);
    engine.set_mem(0x56, 0xf1);
    engine.set_mem(0x57, 0x00);
    crate::game::routine_0061(engine, r);
    crate::game::routine_0201(engine, r);
    crate::game::routine_0128(engine, r);
    routine_0070(engine, r);

    loop {
        routine_0191(engine, r);
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
                routine_0176(engine, r);
                continue;
            } else if lo == 0x0c {
                set_prompt_state(engine, 0x03);
                routine_0177(engine, r);
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
        routine_0074(engine, r);
        engine.set_mem(0x2c, u8v(engine.mem(0x40) + 0x38));
        engine.set_mem(0x2d, 0x3d);
        engine.set_mem(0x2e, 0x3e);
        engine.set_mem(0x2f, 0x3f);
        engine.set_mem(0x56, 0x0d);
        engine.set_mem(0x57, 0x00);
        engine.and_mem(0x45, 0xf0);
        engine.set_mem(0x43, 0x04);
        crate::game::routine_0127(engine, r);
        crate::game::routine_0061(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x05;
        routine_0074(engine, r);
        set_frame_counter(engine, 0x78);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        routine_0067(engine, r);
        engine.set_mem(0x56, 0x08);
        engine.set_mem(0x57, 0x00);
        set_player_health(engine, 0x63);
        set_player_magic(engine, 0x63);
        crate::game::sync_health_hud(engine, r);
        crate::game::sync_magic_hud(engine, r);
        engine.set_mem(0x55, 0x02);
        crate::game::routine_0062(engine, r);
        r.value = 0x08;
        crate::game::routine_0195(engine, r);
        crate::game::routine_0197(engine, r);
        crate::game::routine_0117(engine, r);
        crate::game::routine_0119(engine, r);
        engine.set_mem(0x7b, 0x08);
        crate::game::routine_0060(engine, r);
        crate::game::routine_0061(engine, r);
        routine_0070(engine, r);
        routine_0188(engine, r);
        crate::game::routine_0192(engine, r);
        return;
    }
}

pub fn routine_0187(engine: &mut Engine, r: &mut RoutineContext) {
    routine_0193(engine, r);

    let s80 = engine.mem(0x80);
    let s81 = engine.mem(0x81);
    let s82 = engine.mem(0x82);
    let s83 = engine.mem(0x83);
    r.value = engine.mem(0x47);
    crate::game::routine_0195(engine, r);
    engine.set_mem(0x83, s83);
    engine.set_mem(0x82, s82);
    engine.set_mem(0x81, s81);
    engine.set_mem(0x80, s80);

    crate::game::routine_0198(engine, r);
    crate::game::routine_0120(engine, r);
    crate::game::routine_0199(engine, r);
    routine_0070(engine, r);

    loop {
        routine_0189(engine, r);
        if cbool(r.carry) {
            crate::game::routine_0192(engine, r);
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
                crate::game::routine_0198(engine, r);
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

pub fn routine_0191(engine: &mut Engine, r: &mut RoutineContext) {
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
        crate::game::routine_0143(engine, r);

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

        crate::game::routine_0061(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

pub fn routine_0259(engine: &mut Engine, r: &mut RoutineContext) {
    if (engine.mem(0xee) & 0x7f) == 0 {
        set_prompt_state(engine, 0x18);
        set_prompt_argument(engine, 0xff);
        r.index = 0x03;
        routine_0074(engine, r);

        set_frame_counter(engine, 0x02);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        routine_0074(engine, r);

        set_frame_counter(engine, 0x05);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);

        r.index = 0x03;
        routine_0074(engine, r);

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
        crate::game::routine_0241(engine, r);
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
    crate::game::routine_0241(engine, r);
    crate::game::check_position_out_of_bounds(engine, r);
    if cbool(r.carry) {
        engine.set_mem(0xee, 0x00);
        engine.set_mem(0xf3, 0xf0);
        engine.set_mem(0xeb, 0x01);
        return;
    }
    engine.set_mem(0xfb, engine.mem(0x0a));
}

pub fn routine_0189(engine: &mut Engine, r: &mut RoutineContext) {
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
        crate::game::routine_0143(engine, r);

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

        crate::game::routine_0144(engine, r);
        crate::game::routine_0145(engine, r);
        crate::game::routine_0061(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

pub fn routine_0190(engine: &mut Engine, r: &mut RoutineContext) {
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
        crate::game::routine_0143(engine, r);

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

        crate::game::routine_0144(engine, r);
        crate::game::routine_0145(engine, r);
        crate::game::routine_0061(engine, r);
        frame::commit_frame_work(engine, r);
        frame::wait_for_frame_counter(engine, r);
    }
}

pub fn routine_0188(engine: &mut Engine, r: &mut RoutineContext) {
    loop {
        routine_0190(engine, r);
        if cbool(r.carry) {
            let e = engine.mem(0x55);
            if engine.mem(u16v(0x51 + e)) == 0x0d {
                engine.set_mem(0x55, 0x03);
                crate::game::routine_0062(engine, r);
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
                crate::game::routine_0122(engine, r);
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
        crate::game::routine_0197(engine, r);
        crate::game::routine_0062(engine, r);
        crate::game::routine_0117(engine, r);
        crate::game::routine_0119(engine, r);
    }
}

pub fn routine_0193(engine: &mut Engine, _r: &mut RoutineContext) {
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

pub fn routine_0194(engine: &mut Engine, _r: &mut RoutineContext) {
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

pub fn routine_0240(engine: &mut Engine, r: &mut RoutineContext) {
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
            crate::game::routine_0241(engine, r);
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
    crate::game::routine_0241(engine, r);
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
