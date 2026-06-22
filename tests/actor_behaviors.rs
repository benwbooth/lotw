use lotw::{Engine, RoutineContext, game};

#[test]
fn check_actor_direction_contact_reports_player_contact() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        ..RoutineContext::default()
    };

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x10);
    engine.set_mem(0x45, 0x50);
    engine.set_mem(0x58, 0x0A);
    engine.set_mem(0xEE, 0x01);
    engine.set_mem(0xF8, 0x01);
    engine.set_mem(0xF9, 0x00);
    engine.set_mem(0xFA, 0x10);
    engine.set_mem(0xFB, 0x50);

    game::check_actor_direction_contact(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.mem(0xEA), 0x01);
    assert_eq!(engine.mem(0x58), 0x09);
}

#[test]
fn tick_contact_trigger_actor_resets_when_no_direction_contacts_player() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x80);
    engine.set_mem(0x45, 0xC0);
    engine.set_mem(0xE7, 0x00);
    engine.set_mem(0xE8, 0x20);
    engine.set_mem(0x2004, 0x33);
    engine.set_mem(0xF4, 0x00);
    engine.set_mem(0xF9, 0x00);
    engine.set_mem(0xFA, 0x10);
    engine.set_mem(0xFB, 0x50);

    game::tick_contact_trigger_actor(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF4), 0x00);
    assert_eq!(engine.mem(0xF2), 0x33);
    assert_eq!(engine.mem(0xFC), 0x00);
    assert_eq!(r.value, 0x00);
}

#[test]
fn tick_timed_chase_actor_clears_actor_when_timer_expires() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xEE, 0x01);
    engine.set_mem(0xF1, 0x01);

    game::tick_timed_chase_actor(&mut engine, &mut r);

    assert_eq!(engine.mem(0xF1), 0x00);
    assert_eq!(engine.mem(0xEE), 0x00);
    assert_eq!(r.value, 0x00);
}
