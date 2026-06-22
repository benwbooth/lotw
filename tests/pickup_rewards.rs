use lotw::{Engine, RoutineContext, game};

#[test]
fn collect_room_pickup_object_clears_slot_oam_and_adds_inventory_item() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x0A,
        index: 0x80,
        ..RoutineContext::default()
    };

    engine.set_mem(0x08, 0x02);
    engine.set_mem(0x0401 + 0x80, 0x0A);
    engine.set_mem(0x0406 + 0x80, 0x22);
    engine.set_mem(0x0290, 0x11);
    engine.set_mem(0x0294, 0x22);

    game::collect_room_pickup_object(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0401 + 0x80), 0x00);
    assert_eq!(engine.mem(0x0406 + 0x80), 0xF0);
    assert_eq!(engine.mem(0x0290), 0xEF);
    assert_eq!(engine.mem(0x0294), 0xEF);
    assert_eq!(engine.mem(0x60), 0x01);
    assert_eq!(engine.mem(0x8F), 0x13);
}

#[test]
fn resource_reward_helpers_apply_expected_amounts_and_sounds() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::collect_small_health_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x58), 0x05);
    assert_eq!(engine.mem(0x8F), 0x1E);

    game::collect_small_magic_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x59), 0x05);
    assert_eq!(engine.mem(0x8F), 0x11);

    game::collect_small_coin_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x5A), 0x02);

    game::collect_large_coin_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x5A), 0x34);

    game::collect_single_key_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x5B), 0x01);

    game::collect_key_bundle_reward(&mut engine, &mut r);
    assert_eq!(engine.mem(0x5B), 0x15);
    assert_eq!(engine.mem(0x8F), 0x15);
}

#[test]
fn trigger_damage_pickup_subtracts_health_and_sets_damage_sound() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x58, 0x0A);

    game::trigger_damage_pickup(&mut engine, &mut r);

    assert_eq!(engine.mem(0x58), 0x05);
    assert_eq!(engine.mem(0x8F), 0x1D);
}

#[test]
fn invulnerability_and_speed_boost_rewards_set_timers() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::grant_short_invulnerability(&mut engine, &mut r);
    assert_eq!(engine.mem(0x85), 0x0A);
    assert_eq!(r.value, 0x0A);

    game::grant_long_invulnerability(&mut engine, &mut r);
    assert_eq!(engine.mem(0x85), 0x1E);
    assert_eq!(r.value, 0x1E);

    engine.set_mem(0x88, 0x01);
    engine.set_mem(0x89, 0x02);
    game::grant_short_speed_boost(&mut engine, &mut r);
    assert_eq!(engine.mem(0x88), 0x1E);
    assert_eq!(engine.mem(0x89), 0x1E);
    assert_eq!(engine.mem(0x8A), 0x1E);
    assert_eq!(r.index, 0x1E);
    assert_eq!(r.value, 0x02);

    engine.set_mem(0x88, 0x01);
    engine.set_mem(0x89, 0x02);
    engine.set_mem(0x8A, 0x03);
    game::grant_long_speed_boost(&mut engine, &mut r);
    assert_eq!(engine.mem(0x88), 0x3C);
    assert_eq!(engine.mem(0x89), 0x3C);
    assert_eq!(engine.mem(0x8A), 0x3C);
    assert_eq!(engine.mem(0x8B), 0x3C);
    assert_eq!(r.index, 0x3C);
    assert_eq!(r.value, 0x03);
}
