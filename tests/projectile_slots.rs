use lotw::{Engine, RoutineContext, game};

#[test]
fn expired_player_projectile_clears_its_object_slot() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.obj_slot_ptr_lo = 0xB0;
    engine.state.obj_slot_ptr_hi = 0x04;
    engine.state.set_object_tile(0xB0, 0x21);
    engine.state.set_object_state(0xB0, 0x01);

    game::update_player_projectile_slot(&mut engine, &mut r);

    assert_eq!(engine.state.object_state(0xB0), 0x00);
}
