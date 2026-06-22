use lotw::{Engine, RoutineContext, game};

#[test]
fn read_controllers_uses_live_ppu_buttons_without_replay_override() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.ppu.set_buttons(0x08);
    game::read_controllers(&mut engine, &mut r);

    assert_eq!(engine.mem(0x20), 0x10);
}

#[test]
fn read_controllers_uses_replay_override_when_configured() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.ppu.set_buttons(0x00);
    engine.set_next_input(|| 0x08);
    game::read_controllers(&mut engine, &mut r);

    assert_eq!(engine.mem(0x20), 0x10);
}
