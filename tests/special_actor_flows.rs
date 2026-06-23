use lotw::{Engine, RoutineContext, native};

#[test]
fn defeated_actor_reward_drop_spawns_needed_health_pickup() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_state(0x81);
    engine.state.set_obj_move_scratch(0x01);
    engine.state.set_obj_y_pixel(0xBD);
    engine.state.set_obj_y_extra(0x44);
    engine.state.player_health = 0x13;

    native::tick_defeated_actor_reward_drop(&mut engine, &mut r);

    assert_eq!(engine.state.obj_state(), 0x02);
    assert_eq!(engine.state.obj_tile(), 0x81);
    assert_eq!(engine.state.obj_attr(), 0x01);
    assert_eq!(engine.state.obj_y_pixel(), 0x44);
    assert_eq!(engine.state.obj_timer(), 0xF0);
}

#[test]
fn special_exit_actor_sets_pending_exit_when_fall_hits_bounds() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_state(0x81);
    engine.state.set_obj_move_scratch(0x01);
    engine.state.set_obj_y_pixel(0xBF);

    native::tick_special_exit_actor_sequence(&mut engine, &mut r);

    assert_eq!(engine.state.obj_state(), 0x00);
    assert_eq!(engine.state.pending_special_exit(), 0x01);
    assert_eq!(engine.state.obj_timer(), 0xF0);
}
