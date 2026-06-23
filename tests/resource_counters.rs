use lotw::{Engine, RoutineContext, game};

#[test]
fn add_health_points_clamps_and_marks_hud_dirty() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x05,
        ..RoutineContext::default()
    };

    engine.state.player_health = 0x6C;

    game::add_health_points(&mut engine, &mut r);

    assert_eq!(engine.state.player_health, 0x6D);
    assert_eq!(engine.state.hud_refresh_flag, 0x01);
}

#[test]
fn subtract_health_points_saturates_underflow() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x05,
        ..RoutineContext::default()
    };

    engine.state.player_health = 0x03;

    game::subtract_health_points(&mut engine, &mut r);

    assert_eq!(engine.state.player_health, 0x00);
    assert_eq!(r.carry, 0);
}

#[test]
fn consume_magic_point_preserves_index_and_reports_empty() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x42,
        ..RoutineContext::default()
    };

    engine.state.player_magic = 0x02;
    game::consume_magic_point(&mut engine, &mut r);

    assert_eq!(engine.state.player_magic, 0x01);
    assert_eq!(r.index, 0x42);
    assert_eq!(r.carry, 0);

    engine.state.player_magic = 0x00;
    r.index = 0x77;
    game::consume_magic_point(&mut engine, &mut r);

    assert_eq!(engine.state.player_magic, 0x00);
    assert_eq!(r.index, 0x77);
    assert_eq!(r.carry, 1);
}

#[test]
fn spend_coins_updates_balance_only_when_affordable() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x06,
        ..RoutineContext::default()
    };

    engine.state.coins = 0x0A;
    game::spend_coins(&mut engine, &mut r);

    assert_eq!(engine.state.coins, 0x04);
    assert_eq!(r.carry, 1);

    r.value = 0x05;
    game::spend_coins(&mut engine, &mut r);

    assert_eq!(engine.state.coins, 0x04);
    assert_eq!(r.carry, 0);
}

#[test]
fn consume_key_decrements_or_reports_empty() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.keys = 0x01;
    game::consume_key(&mut engine, &mut r);

    assert_eq!(engine.state.keys, 0x00);
    assert_eq!(r.carry, 0);

    game::consume_key(&mut engine, &mut r);

    assert_eq!(engine.state.keys, 0x00);
    assert_eq!(r.carry, 1);
}
