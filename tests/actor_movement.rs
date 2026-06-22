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

#[test]
fn try_move_large_actor_with_terrain_uses_wide_probe_and_restores_vertical_velocity() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x80);
    engine.set_mem(0x45, 0xC0);
    engine.set_mem(0xEE, 0x01);
    engine.set_mem(0xF5, 0x02);
    engine.set_mem(0xF6, 0x00);
    engine.set_mem(0xF7, 0x04);
    engine.set_mem(0xF9, 0x00);
    engine.set_mem(0xFA, 0x20);
    engine.set_mem(0xFB, 0x50);

    game::try_move_large_actor_with_terrain(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.mem(0xF7), 0x04);
    assert_eq!(engine.mem(0x0E), 0x02);
    assert_eq!(engine.mem(0x0F), 0x20);
    assert_eq!(engine.mem(0x0A), 0x54);
}

#[test]
fn compose_large_actor_body_slots_syncs_linked_sprite_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x30, 0xAA);
    engine.set_mem(0x31, 0xBB);
    engine.set_mem(0xEE, 0x01);
    engine.set_mem(0xED, 0x41);
    engine.set_mem(0xEF, 0x02);
    engine.set_mem(0xF2, 0x09);
    engine.set_mem(0xF9, 0x03);
    engine.set_mem(0xFA, 0x20);
    engine.set_mem(0xFB, 0x50);
    engine.set_mem(0xFC, 0x77);
    engine.set_mem(0x0411, 0x80);
    engine.set_mem(0x0415, 0x08);
    engine.set_mem(0x0425, 0x06);
    engine.set_mem(0x0435, 0x07);

    game::compose_large_actor_body_slots(&mut engine, &mut r);

    assert_eq!(engine.mem(0x041C), 0x03);
    assert_eq!(engine.mem(0x042C), 0x03);
    assert_eq!(engine.mem(0x043C), 0x03);
    assert_eq!(engine.mem(0x042D), 0x20);
    assert_eq!(engine.mem(0x041D), 0x21);
    assert_eq!(engine.mem(0x043D), 0x21);
    assert_eq!(engine.mem(0x041E), 0x50);
    assert_eq!(engine.mem(0x042E), 0x60);
    assert_eq!(engine.mem(0x043E), 0x60);
    assert_eq!(engine.mem(0x041F), 0x77);
    assert_eq!(engine.mem(0x042F), 0x77);
    assert_eq!(engine.mem(0x043F), 0x77);

    assert_eq!(engine.mem(0x0401), 0x80);
    assert_eq!(engine.mem(0x0411), 0x80);
    assert_eq!(engine.mem(0x0421), 0x80);
    assert_eq!(engine.mem(0x0431), 0x80);
    assert_eq!(engine.mem(0x0405), 0x06);

    assert_eq!(engine.mem(0x0410), 0x45);
    assert_eq!(engine.mem(0x0420), 0x61);
    assert_eq!(engine.mem(0x0430), 0x65);
    assert_eq!(engine.mem(0x0412), 0x02);
    assert_eq!(engine.mem(0x0422), 0x02);
    assert_eq!(engine.mem(0x0432), 0x02);

    assert_eq!(engine.mem(0x30), 0xAA);
    assert_eq!(engine.mem(0x31), 0xBB);
}
