use lotw::{Engine, RoutineContext, game};

#[test]
fn project_actor_position_applies_subtile_carry_and_vertical_velocity() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xF9, 0x03);
    engine.set_mem(0xFA, 0x10);
    engine.set_mem(0xFB, 0x50);
    engine.set_mem(0xF5, 0x0E);
    engine.set_mem(0xF6, 0x00);
    engine.set_mem(0xF7, 0x02);

    game::project_actor_position(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0E), 0x01);
    assert_eq!(engine.mem(0x0F), 0x11);
    assert_eq!(engine.mem(0x0A), 0x52);
}

#[test]
fn commit_and_stop_actor_helpers_update_scratch_motion_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0E, 0x04);
    engine.set_mem(0x0F, 0x22);
    engine.set_mem(0x0A, 0x66);
    engine.set_mem(0xF5, 0x01);
    engine.set_mem(0xF7, 0x02);
    engine.set_mem(0xF1, 0x03);
    engine.set_mem(0xF0, 0x04);

    game::commit_actor_projected_position(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF9), 0x04);
    assert_eq!(engine.mem(0xFA), 0x22);
    assert_eq!(engine.mem(0xFB), 0x66);
    assert_eq!(r.value, 0x66);

    game::stop_actor_motion(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF5), 0x00);
    assert_eq!(engine.mem(0xF7), 0x00);
    assert_eq!(engine.mem(0xF1), 0x00);
    assert_eq!(engine.mem(0xF0), 0x00);
}

#[test]
fn reverse_actor_horizontal_direction_flips_low_direction_bits() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xF4, 0x01);
    game::reverse_actor_horizontal_direction(&mut engine, &mut r);
    assert_eq!(engine.mem(0xF4), 0x02);
    assert_eq!(r.value, 0x02);

    engine.set_mem(0xF4, 0x00);
    game::reverse_actor_horizontal_direction(&mut engine, &mut r);
    assert_eq!(engine.mem(0xF4), 0x02);
}

#[test]
fn update_actor_animation_dispatches_room_animation_mode() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xE7, 0x00);
    engine.set_mem(0xE8, 0x20);
    engine.set_mem(0x2007, 0x03);
    engine.set_mem(0xF3, 0x02);
    engine.set_mem(0xED, 0xF3);

    game::update_actor_animation(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0E), 0xB9);
    assert_eq!(engine.mem(0x0F), 0xF0);
    assert_eq!(engine.mem(0xF3), 0x03);
    assert_eq!(engine.mem(0xED) & 0x0C, 0x04);
    assert_eq!(r.index, 0x06);
    assert_eq!(r.value, 0x06);
}

#[test]
fn apply_actor_player_contact_damage_updates_health_and_hit_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x58, 0x0A);
    engine.set_mem(0xEE, 0x01);
    engine.set_mem(0xEF, 0xFF);
    engine.set_mem(0xF8, 0x02);

    game::apply_actor_player_contact_damage(&mut engine, &mut r);

    assert_eq!(engine.mem(0x58), 0x08);
    assert_eq!(engine.mem(0x85), 0x01);
    assert_eq!(engine.mem(0x8F), 0x21);
    assert_eq!(engine.mem(0x90), 0x01);
    assert_eq!(engine.mem(0xEF) & 0x20, 0x00);
}
