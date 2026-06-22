use lotw::{Engine, RoutineContext, game};

#[test]
fn scripted_player_projection_applies_x_and_y_deltas() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x20);
    engine.set_mem(0x45, 0x80);
    engine.set_mem(0x49, 0x05);
    engine.set_mem(0x4B, 0xFE);

    game::project_scripted_player_position(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0E), 0x25);
    assert_eq!(engine.mem(0x0A), 0x7E);
}

#[test]
fn scripted_player_input_delta_uses_controller_direction_table() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x20, 0x01);
    engine.set_mem(0xFE8D, 0x05);
    engine.set_mem(0xFE8E, 0xFD);

    game::build_scripted_player_input_delta(&mut engine, &mut r);

    assert_eq!(r.index, 0x02);
    assert_eq!(engine.mem(0x49), 0x05);
    assert_eq!(engine.mem(0x4B), 0xFD);
}

#[test]
fn scripted_player_bounds_reject_bottom_and_right_edges() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0A, 0xA0);
    engine.set_mem(0x0E, 0xF0);

    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x0A, 0xA1);
    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.set_mem(0x0A, 0x80);
    engine.set_mem(0x0E, 0xF1);
    game::check_scripted_player_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn scripted_player_move_retries_vertical_delta_toward_bounds() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x20);
    engine.set_mem(0x45, 0x9F);
    engine.set_mem(0x49, 0x00);
    engine.set_mem(0x4B, 0x02);

    game::try_move_scripted_player_in_bounds(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.mem(0x0A), 0xA0);
    assert_eq!(engine.mem(0x4B), 0x02);
}

#[test]
fn scripted_player_health_subtract_saturates_underflow() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x58, 0x10);
    r.value = 0x04;

    game::subtract_scripted_player_health(&mut engine, &mut r);

    assert_eq!(engine.mem(0x58), 0x0C);
    assert_eq!(r.carry, 1);

    r.value = 0x20;
    game::subtract_scripted_player_health(&mut engine, &mut r);

    assert_eq!(engine.mem(0x58), 0x00);
    assert_eq!(r.carry, 0);
}

#[test]
fn scripted_player_sprites_draw_flip_and_blink() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x20);
    engine.set_mem(0x45, 0x10);
    engine.set_mem(0x56, 0x30);
    engine.set_mem(0x57, 0x00);

    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0210), 0x3B);
    assert_eq!(engine.mem(0x0214), 0x3B);
    assert_eq!(engine.mem(0x0213), 0x20);
    assert_eq!(engine.mem(0x0217), 0x28);
    assert_eq!(engine.mem(0x0211), 0x30);
    assert_eq!(engine.mem(0x0215), 0x32);

    engine.set_mem(0x57, 0x40);
    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0211), 0x32);
    assert_eq!(engine.mem(0x0215), 0x30);
    assert_eq!(engine.mem(0x0212), 0x60);
    assert_eq!(engine.mem(0x0216), 0x60);

    engine.set_mem(0x85, 0x01);
    engine.set_mem(0x84, 0x00);
    game::draw_scripted_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0210), 0xEF);
    assert_eq!(engine.mem(0x0214), 0xEF);
}

#[test]
fn scripted_player_pose_uses_horizontal_delta_for_ground_flip() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x56, 0x30);
    engine.set_mem(0x49, 0x01);

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0x56), 0x01);
    assert_eq!(engine.mem(0x57), 0x40);
    assert_eq!(r.index, 0x01);

    engine.set_mem(0x56, 0x30);
    engine.set_mem(0x49, 0xFF);

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0x56), 0x01);
    assert_eq!(engine.mem(0x57), 0x00);
}

#[test]
fn scripted_player_pose_uses_airborne_pose_while_falling() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x56, 0x22);
    engine.set_mem(0x49, 0x01);
    engine.set_mem(0x4B, 0x01);
    engine.set_mem(0x4E, 0x01);

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0x56), 0x3B);
    assert_eq!(engine.mem(0x57), 0x40);
    assert_eq!(r.index, 0x39);
}

#[test]
fn scripted_player_pose_uses_jump_pose_for_unheld_upward_motion() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x56, 0x30);
    engine.set_mem(0x4B, 0xFF);
    engine.set_mem(0x4F, 0x00);

    game::update_scripted_player_pose_from_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0x56), 0x09);
    assert_eq!(r.index, 0x09);
}

#[test]
fn scripted_player_fall_state_counts_and_bounces_after_long_fall() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x45, 0x9F);
    game::update_scripted_player_fall_state(&mut engine, &mut r);

    assert_eq!(engine.mem(0x4E), 0x01);

    engine.set_mem(0x45, 0xA0);
    engine.set_mem(0x4E, 0x10);
    engine.set_mem(0x5C, 0x10);

    game::update_scripted_player_fall_state(&mut engine, &mut r);

    assert_eq!(engine.mem(0x4F), 0x08);
    assert_eq!(engine.mem(0x8F), 0x0A);
    assert_eq!(engine.mem(0x4E), 0x00);
}
