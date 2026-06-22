use lotw::{Engine, RoutineContext, game};

#[test]
fn resolve_room_tile_pointer_populates_tile_and_room_offsets() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0C, 0x02);
    engine.set_mem(0x0D, 0x30);
    engine.set_mem(0x75, 0x40);
    engine.set_mem(0x76, 0x12);

    game::resolve_room_tile_pointer(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0C), 0x1B);
    assert_eq!(engine.mem(0x0D), 0x05);
    assert_eq!(engine.mem(0x10), 0x5B);
    assert_eq!(engine.mem(0x11), 0x12);
}

#[test]
fn bounds_helpers_set_carry_when_projected_position_is_outside() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0A, 0xBF);
    engine.set_mem(0x0F, 0x3E);
    engine.set_mem(0x0E, 0x01);
    game::check_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x0F, 0x3F);
    game::check_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.set_mem(0x0A, 0xB0);
    engine.set_mem(0x0F, 0x3E);
    engine.set_mem(0x0E, 0x01);
    game::check_actor_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn check_player_overlap_sets_collision_flag() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x10);
    engine.set_mem(0x45, 0x50);
    engine.set_mem(0x0E, 0x00);
    engine.set_mem(0x0F, 0x10);
    engine.set_mem(0x0A, 0x50);

    game::check_player_overlap(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.mem(0xEA), 0x01);

    engine.set_mem(0x0A, 0x70);
    game::check_player_overlap(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.mem(0xEA), 0x00);
}

#[test]
fn build_direction_velocity_clears_velocity_for_zero_speed() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        offset: 0x00,
        ..RoutineContext::default()
    };

    engine.set_mem(0xF5, 0xAA);
    engine.set_mem(0xF6, 0xBB);
    engine.set_mem(0xF7, 0xCC);

    game::build_direction_velocity(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF5), 0x00);
    assert_eq!(engine.mem(0xF6), 0x00);
    assert_eq!(engine.mem(0xF7), 0x00);
}

#[test]
fn solid_tile_probes_set_carry_for_solid_range() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        offset: 0x01,
        ..RoutineContext::default()
    };

    engine.set_mem(0x0C, 0x00);
    engine.set_mem(0x0D, 0x02);
    engine.set_mem(0x0201, 0x30);

    game::probe_object_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.set_mem(0x0201, 0x2F);
    game::probe_projected_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 0);
}

#[test]
fn try_reflect_object_velocity_reports_no_reflection_when_stationary() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xF5, 0x00);
    engine.set_mem(0xF7, 0x00);

    game::try_reflect_object_velocity(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.mem(0xF6), 0x00);
}
