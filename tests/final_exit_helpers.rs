use lotw::{Engine, RoutineContext, game};

#[test]
fn final_exit_projectile_velocity_accumulates_direction_steps() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        offset: 0x02,
        ..RoutineContext::default()
    };

    engine.set_mem(0xFE8D, 0x03);
    engine.set_mem(0xFE8E, 0xFE);

    game::build_final_exit_projectile_velocity(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF5), 0x06);
    assert_eq!(engine.mem(0xF7), 0xFC);
}

#[test]
fn final_exit_projectile_projection_uses_spawn_and_motion_scales() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x20);
    engine.set_mem(0x45, 0x40);
    engine.set_mem(0xF5, 0x02);
    engine.set_mem(0xF7, 0xFF);

    game::project_final_exit_projectile_spawn(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0E), 0x28);
    assert_eq!(engine.mem(0x0A), 0x3C);

    engine.set_mem(0xF9, 0x30);
    engine.set_mem(0xFB, 0x50);

    game::project_final_exit_projectile_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0E), 0x32);
    assert_eq!(engine.mem(0x0A), 0x4F);
}

#[test]
fn final_exit_projectile_bounds_only_raise_carry_on_right_edge() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0A, 0x80);
    engine.set_mem(0x0E, 0xF1);

    game::check_final_exit_projectile_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    r.carry = 0;
    engine.set_mem(0x0A, 0xA1);

    game::check_final_exit_projectile_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);
}

#[test]
fn final_exit_projectile_animation_and_sprites_follow_slot_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xED, 0xFF);
    engine.set_mem(0xEE, 0x18);

    game::update_final_exit_projectile_animation_bits(&mut engine, &mut r);

    assert_eq!(engine.mem(0xED), 0xFB);

    engine.set_mem(0x0F, 0x88);
    engine.set_mem(0x0E, 0x10);
    engine.set_mem(0x0411, 0x01);
    engine.set_mem(0x0410, 0x50);
    engine.set_mem(0x0412, 0x40);
    engine.set_mem(0x041C, 0x30);
    engine.set_mem(0x041E, 0x20);

    game::draw_final_exit_projectile_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0288), 0x4B);
    assert_eq!(engine.mem(0x028C), 0x4B);
    assert_eq!(engine.mem(0x028B), 0x30);
    assert_eq!(engine.mem(0x028F), 0x38);
    assert_eq!(engine.mem(0x0289), 0x52);
    assert_eq!(engine.mem(0x028D), 0x50);

    engine.set_mem(0x0411, 0x00);
    game::draw_final_exit_projectile_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0288), 0xEF);
    assert_eq!(engine.mem(0x028C), 0xEF);
}

#[test]
fn sprite_zero_rotation_copies_and_hides_scripted_oam_entry() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x3E, 0x01);
    engine.set_mem(0x0210, 0x22);
    engine.set_mem(0x0211, 0x33);
    engine.set_mem(0x0212, 0x44);
    engine.set_mem(0x0213, 0x55);

    game::rotate_sprite_zero_from_scripted_oam(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0200), 0x22);
    assert_eq!(engine.mem(0x0201), 0x33);
    assert_eq!(engine.mem(0x0202), 0x44);
    assert_eq!(engine.mem(0x0203), 0x55);
    assert_eq!(engine.mem(0x0210), 0xEF);
    assert_eq!(engine.mem(0x3E), 0x00);
}

#[test]
fn final_exit_body_slots_mirror_player_pose_and_position() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x56, 0x3F);
    engine.set_mem(0x0410, 0xE0);
    engine.set_mem(0x0420, 0xC0);
    engine.set_mem(0x0430, 0xA0);
    engine.set_mem(0x43, 0x20);
    engine.set_mem(0x44, 0x10);

    game::sync_final_exit_body_slots_from_player(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0410), 0xFF);
    assert_eq!(engine.mem(0x0420), 0xDF);
    assert_eq!(engine.mem(0x0430), 0xBF);
    assert_eq!(engine.mem(0x041C), 0x20);
    assert_eq!(engine.mem(0x042C), 0x20);
    assert_eq!(engine.mem(0x043C), 0x20);
    assert_eq!(engine.mem(0x042D), 0x11);
    assert_eq!(engine.mem(0x043D), 0x0E);
    assert_eq!(engine.mem(0x041D), 0x0D);
}
