use lotw::{Engine, RoutineContext, native};

#[test]
fn defeated_actor_reward_drop_spawns_needed_health_pickup() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xEE, 0x81);
    engine.set_mem(0xF0, 0x01);
    engine.set_mem(0xFB, 0xBD);
    engine.set_mem(0xFC, 0x44);
    engine.set_mem(0x58, 0x13);

    native::tick_defeated_actor_reward_drop(&mut engine, &mut r);

    assert_eq!(engine.mem(0xEE), 0x02);
    assert_eq!(engine.mem(0xED), 0x81);
    assert_eq!(engine.mem(0xEF), 0x01);
    assert_eq!(engine.mem(0xFB), 0x44);
    assert_eq!(engine.mem(0xF3), 0xF0);
}

#[test]
fn special_exit_actor_sets_pending_exit_when_fall_hits_bounds() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xEE, 0x81);
    engine.set_mem(0xF0, 0x01);
    engine.set_mem(0xFB, 0xBF);

    native::tick_special_exit_actor_sequence(&mut engine, &mut r);

    assert_eq!(engine.mem(0xEE), 0x00);
    assert_eq!(engine.mem(0xEB), 0x01);
    assert_eq!(engine.mem(0xF3), 0xF0);
}
