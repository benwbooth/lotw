use lotw::{Engine, RoutineContext, game};

#[test]
fn expired_player_projectile_clears_its_object_slot() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0xE5, 0xB0);
    engine.set_mem(0xE6, 0x04);
    engine.set_mem(0x04B0, 0x21);
    engine.set_mem(0x04B1, 0x01);

    game::routine_0268(&mut engine, &mut r);

    assert_eq!(engine.mem(0x04B1), 0x00);
}
