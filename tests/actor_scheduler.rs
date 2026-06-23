use lotw::{Engine, RoutineContext, game};

#[test]
fn update_room_actors_skips_when_room_y_marker_blocks_actor_tick() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.map_screen_y = 0x10;
    engine.state.scheduler_phase = 0x02;

    game::update_room_actors(&mut engine, &mut r);

    assert_eq!(engine.state.scheduler_phase, 0x02);
    assert_eq!(engine.state.slot_index, 0x00);
}

#[test]
fn tick_inactive_actor_slot_promotes_ready_actor_from_room_data() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.family_member_mask = 0x01;
    engine.state.actor_record_ptr_lo = 0x00;
    engine.state.actor_record_ptr_hi = 0x20;
    engine.state.obj_timer = 0x01;
    engine.state.set_byte(0x2000, 0x81);
    engine.state.set_byte(0x2001, 0x02);
    engine.state.set_byte(0x2002, 0x20);
    engine.state.set_byte(0x2003, 0x40);
    engine.state.set_byte(0x2004, 0x05);
    engine.state.set_byte(0x2005, 0x06);

    game::tick_inactive_actor_slot(&mut engine, &mut r);

    assert_eq!(engine.state.obj_state, 0x01);
    assert_eq!(engine.state.obj_tile, 0x81);
    assert_eq!(engine.state.obj_attr, 0x02);
    assert_eq!(engine.state.obj_health, 0x05);
    assert_eq!(engine.state.obj_damage, 0x06);
    assert_eq!(engine.state.obj_x_tile, 0x20);
    assert_eq!(engine.state.obj_y_pixel, 0x40);
}

#[test]
fn tick_actor_materialize_delay_activates_when_timer_expires() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.actor_record_ptr_lo = 0x00;
    engine.state.actor_record_ptr_hi = 0x20;
    engine.state.obj_timer = 0x01;
    engine.state.set_byte(0x2000, 0x84);
    engine.state.set_byte(0x2001, 0x45);

    game::tick_actor_materialize_delay(&mut engine, &mut r);

    assert_eq!(engine.state.obj_timer, 0x00);
    assert_eq!(engine.state.obj_state, 0x01);
    assert_eq!(engine.state.obj_tile, 0x84);
    assert_eq!(engine.state.obj_attr, 0x45);
}

#[test]
fn tick_standard_actor_expires_when_lifetime_timer_reaches_zero() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.obj_state = 0x02;
    engine.state.obj_move_scratch = 0x00;
    engine.state.obj_cooldown = 0x00;
    engine.state.obj_timer = 0x01;

    game::tick_standard_actor(&mut engine, &mut r);

    assert_eq!(engine.state.obj_state, 0x00);
    assert_eq!(engine.state.obj_timer, 0xF0);
    assert_eq!(r.index, 0x00);
}
