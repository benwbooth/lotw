use lotw::{Engine, RoutineContext, game};

#[test]
fn scripted_player_projection_applies_x_and_y_deltas() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x20;
    engine.state.player_y = 0x80;
    engine.state.horizontal_subtile_delta = 0x05;
    engine.state.vertical_delta = 0xFE;

    game::project_scripted_player_position(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo, 0x25);
    assert_eq!(engine.state.scratch2, 0x7E);
}

#[test]
fn scripted_player_input_delta_uses_controller_direction_table() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.buttons = 0x01;
    engine
        .state
        .set_byte(lotw::game::MOVE_DELTA_Y_TABLE + 1, 0x05);
    engine
        .state
        .set_byte(lotw::game::MOVE_DELTA_Y_TABLE + 2, 0xFD);

    game::build_scripted_player_input_delta(&mut engine, &mut r);

    assert_eq!(r.index, 0x02);
    assert_eq!(engine.state.horizontal_subtile_delta, 0x05);
    assert_eq!(engine.state.vertical_delta, 0xFD);
}

#[test]
fn scripted_player_bounds_reject_bottom_and_right_edges() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scratch2 = 0xA0;
    engine.state.indirect_ptr_lo = 0xF0;

    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.state.scratch2 = 0xA1;
    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.state.scratch2 = 0x80;
    engine.state.indirect_ptr_lo = 0xF1;
    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn scripted_player_move_retries_vertical_delta_toward_bounds() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x20;
    engine.state.player_y = 0x9F;
    engine.state.horizontal_subtile_delta = 0x00;
    engine.state.vertical_delta = 0x02;

    game::try_move_scripted_player_in_bounds(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.state.scratch2, 0xA0);
    assert_eq!(engine.state.vertical_delta, 0x02);
}

#[test]
fn scripted_player_health_subtract_saturates_underflow() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_health = 0x10;
    r.value = 0x04;

    game::subtract_scripted_player_health(&mut engine, &mut r);

    assert_eq!(engine.state.player_health, 0x0C);
    assert_eq!(r.carry, 1);

    r.value = 0x20;
    game::subtract_scripted_player_health(&mut engine, &mut r);

    assert_eq!(engine.state.player_health, 0x00);
    assert_eq!(r.carry, 0);
}

#[test]
fn scripted_player_sprites_draw_flip_and_blink() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x20;
    engine.state.player_y = 0x10;
    engine.state.player_pose = 0x30;
    engine.state.player_facing = 0x00;

    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x10), 0x3B);
    assert_eq!(engine.state.oam_y(0x14), 0x3B);
    assert_eq!(engine.state.oam_x(0x10), 0x20);
    assert_eq!(engine.state.oam_x(0x14), 0x28);
    assert_eq!(engine.state.oam_tile(0x10), 0x30);
    assert_eq!(engine.state.oam_tile(0x14), 0x32);

    engine.state.player_facing = 0x40;
    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_tile(0x10), 0x32);
    assert_eq!(engine.state.oam_tile(0x14), 0x30);
    assert_eq!(engine.state.oam_attr(0x10), 0x60);
    assert_eq!(engine.state.oam_attr(0x14), 0x60);

    engine.state.sprite_blink_timer = 0x01;
    engine.state.frame_prescaler = 0x00;
    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x10), 0xEF);
    assert_eq!(engine.state.oam_y(0x14), 0xEF);
}

#[test]
fn scripted_player_pose_uses_horizontal_delta_for_ground_flip() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_pose = 0x30;
    engine.state.horizontal_subtile_delta = 0x01;

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.state.player_pose, 0x01);
    assert_eq!(engine.state.player_facing, 0x40);
    assert_eq!(r.index, 0x01);

    engine.state.player_pose = 0x30;
    engine.state.horizontal_subtile_delta = 0xFF;

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.state.player_pose, 0x01);
    assert_eq!(engine.state.player_facing, 0x00);
}

#[test]
fn scripted_player_pose_uses_airborne_pose_while_falling() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_pose = 0x22;
    engine.state.horizontal_subtile_delta = 0x01;
    engine.state.vertical_delta = 0x01;
    engine.state.fall_frames = 0x01;

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.state.player_pose, 0x3B);
    assert_eq!(engine.state.player_facing, 0x40);
    assert_eq!(r.index, 0x39);
}

#[test]
fn scripted_player_pose_uses_jump_pose_for_unheld_upward_motion() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_pose = 0x30;
    engine.state.vertical_delta = 0xFF;
    engine.state.jump_timer = 0x00;

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.state.player_pose, 0x09);
    assert_eq!(r.index, 0x09);
}

#[test]
fn scripted_player_fall_state_counts_and_bounces_after_long_fall() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_y = 0x9F;
    game::update_scripted_player_fall_state(&mut engine, &mut r);

    assert_eq!(engine.state.fall_frames, 0x01);

    engine.state.player_y = 0xA0;
    engine.state.fall_frames = 0x10;
    engine.state.jump_strength = 0x10;

    game::update_scripted_player_fall_state(&mut engine, &mut r);

    assert_eq!(engine.state.jump_timer, 0x08);
    assert_eq!(engine.state.prompt_state, 0x0A);
    assert_eq!(engine.state.fall_frames, 0x00);
}
