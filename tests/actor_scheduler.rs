use lotw::{Engine, RoutineContext, game};

#[test]
fn update_room_actors_skips_when_room_y_marker_blocks_actor_tick() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x48, 0x10);
    engine.set_mem(0xE9, 0x02);

    game::update_room_actors(&mut engine, &mut r);

    assert_eq!(engine.mem(0xE9), 0x02);
    assert_eq!(engine.mem(0xE3), 0x00);
}

#[test]
fn tick_inactive_actor_slot_promotes_ready_actor_from_room_data() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x41, 0x01);
    engine.set_mem(0xE7, 0x00);
    engine.set_mem(0xE8, 0x20);
    engine.set_mem(0xF3, 0x01);
    engine.set_mem(0x2000, 0x81);
    engine.set_mem(0x2001, 0x02);
    engine.set_mem(0x2002, 0x20);
    engine.set_mem(0x2003, 0x40);
    engine.set_mem(0x2004, 0x05);
    engine.set_mem(0x2005, 0x06);

    game::tick_inactive_actor_slot(&mut engine, &mut r);

    assert_eq!(engine.mem(0xEE), 0x01);
    assert_eq!(engine.mem(0xED), 0x81);
    assert_eq!(engine.mem(0xEF), 0x02);
    assert_eq!(engine.mem(0xF2), 0x05);
    assert_eq!(engine.mem(0xF8), 0x06);
    assert_eq!(engine.mem(0xFA), 0x20);
    assert_eq!(engine.mem(0xFB), 0x40);
}

#[test]
fn tick_actor_materialize_delay_activates_when_timer_expires() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xE7, 0x00);
    engine.set_mem(0xE8, 0x20);
    engine.set_mem(0xF3, 0x01);
    engine.set_mem(0x2000, 0x84);
    engine.set_mem(0x2001, 0x45);

    game::tick_actor_materialize_delay(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF3), 0x00);
    assert_eq!(engine.mem(0xEE), 0x01);
    assert_eq!(engine.mem(0xED), 0x84);
    assert_eq!(engine.mem(0xEF), 0x45);
}

#[test]
fn tick_standard_actor_expires_when_lifetime_timer_reaches_zero() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xEE, 0x02);
    engine.set_mem(0xF0, 0x00);
    engine.set_mem(0xF1, 0x00);
    engine.set_mem(0xF3, 0x01);

    game::tick_standard_actor(&mut engine, &mut r);

    assert_eq!(engine.mem(0xEE), 0x00);
    assert_eq!(engine.mem(0xF3), 0xF0);
    assert_eq!(r.index, 0x00);
}
