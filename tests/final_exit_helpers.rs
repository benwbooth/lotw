use lotw::{Engine, RoutineContext, game};

#[test]
fn final_exit_projectile_velocity_accumulates_direction_steps() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        offset: 0x02,
        ..RoutineContext::default()
    };

    engine
        .state
        .set_byte(lotw::game::MOVE_DELTA_Y_TABLE + 1, 0x03);
    engine
        .state
        .set_byte(lotw::game::MOVE_DELTA_Y_TABLE + 2, 0xFE);

    game::build_final_exit_projectile_velocity(&mut engine, &mut r);

    assert_eq!(engine.state.obj_x_vel_lo, 0x06);
    assert_eq!(engine.state.obj_y_vel, 0xFC);
}

#[test]
fn final_exit_projectile_projection_uses_spawn_and_motion_scales() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x20;
    engine.state.player_y = 0x40;
    engine.state.obj_x_vel_lo = 0x02;
    engine.state.obj_y_vel = 0xFF;

    game::project_final_exit_projectile_spawn(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo, 0x28);
    assert_eq!(engine.state.scratch2, 0x3C);

    engine.state.obj_x_sub = 0x30;
    engine.state.obj_y_pixel = 0x50;

    game::project_final_exit_projectile_motion(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo, 0x32);
    assert_eq!(engine.state.scratch2, 0x4F);
}

#[test]
fn final_exit_projectile_bounds_only_raise_carry_on_right_edge() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scratch2 = 0x80;
    engine.state.indirect_ptr_lo = 0xF1;

    game::check_final_exit_projectile_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    r.carry = 0;
    engine.state.scratch2 = 0xA1;

    game::check_final_exit_projectile_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);
}

#[test]
fn final_exit_projectile_animation_and_sprites_follow_slot_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.obj_tile = 0xFF;
    engine.state.obj_state = 0x18;

    game::update_final_exit_projectile_animation_bits(&mut engine, &mut r);

    assert_eq!(engine.state.obj_tile, 0xFB);

    engine.state.indirect_ptr_hi = 0x88;
    engine.state.indirect_ptr_lo = 0x10;
    engine.state.set_object_state(0x10, 0x01);
    engine.state.set_object_tile(0x10, 0x50);
    engine.state.set_object_attr(0x10, 0x40);
    engine.state.set_object_x_sub(0x10, 0x30);
    engine.state.set_object_y_pixel(0x10, 0x20);

    game::draw_final_exit_projectile_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x88), 0x4B);
    assert_eq!(engine.state.oam_y(0x8C), 0x4B);
    assert_eq!(engine.state.oam_x(0x88), 0x30);
    assert_eq!(engine.state.oam_x(0x8C), 0x38);
    assert_eq!(engine.state.oam_tile(0x88), 0x52);
    assert_eq!(engine.state.oam_tile(0x8C), 0x50);

    engine.state.set_object_state(0x10, 0x00);
    game::draw_final_exit_projectile_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x88), 0xEF);
    assert_eq!(engine.state.oam_y(0x8C), 0xEF);
}

#[test]
fn sprite_zero_rotation_copies_and_hides_scripted_oam_entry() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.sprite_index = 0x01;
    engine.state.set_oam_y(0x10, 0x22);
    engine.state.set_oam_tile(0x10, 0x33);
    engine.state.set_oam_attr(0x10, 0x44);
    engine.state.set_oam_x(0x10, 0x55);

    game::rotate_sprite_zero_from_scripted_oam(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x00), 0x22);
    assert_eq!(engine.state.oam_tile(0x00), 0x33);
    assert_eq!(engine.state.oam_attr(0x00), 0x44);
    assert_eq!(engine.state.oam_x(0x00), 0x55);
    assert_eq!(engine.state.oam_y(0x10), 0xEF);
    assert_eq!(engine.state.sprite_index, 0x00);
}

#[test]
fn final_exit_body_slots_mirror_player_pose_and_position() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_pose = 0x3F;
    engine.state.set_object_tile(0x10, 0xE0);
    engine.state.set_object_tile(0x20, 0xC0);
    engine.state.set_object_tile(0x30, 0xA0);
    engine.state.player_x_fine = 0x20;
    engine.state.player_x_tile = 0x10;

    game::sync_final_exit_body_slots_from_player(&mut engine, &mut r);

    assert_eq!(engine.state.object_tile(0x10), 0xFF);
    assert_eq!(engine.state.object_tile(0x20), 0xDF);
    assert_eq!(engine.state.object_tile(0x30), 0xBF);
    assert_eq!(engine.state.object_x_sub(0x10), 0x20);
    assert_eq!(engine.state.object_x_sub(0x20), 0x20);
    assert_eq!(engine.state.object_x_sub(0x30), 0x20);
    assert_eq!(engine.state.object_x_tile(0x20), 0x11);
    assert_eq!(engine.state.object_x_tile(0x30), 0x0E);
    assert_eq!(engine.state.object_x_tile(0x10), 0x0D);
}

#[test]
fn final_exit_oam_templates_copy_expected_ranges() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x3F {
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_A + offset) as u16 as i32,
            0x10 + offset,
        );
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_B + offset) as u16 as i32,
            0x50 + offset,
        );
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_C + offset) as u16 as i32,
            0x90 + offset,
        );
    }

    game::load_final_exit_object_oam_template(&mut engine, &mut r);
    assert_eq!(engine.state.oam_y(0x40), 0x10);
    assert_eq!(engine.state.oam_x(0x7C), 0x4F);

    game::load_large_actor_oam_template(&mut engine, &mut r);
    assert_eq!(engine.state.oam_y(0x40), 0x50);
    assert_eq!(engine.state.oam_x(0x7C), 0x8F);

    game::load_final_exit_player_oam_template(&mut engine, &mut r);
    assert_eq!(engine.state.oam_y(0xC0), 0x90);
    assert_eq!(engine.state.oam_x(0xFC), 0xCF);
}
