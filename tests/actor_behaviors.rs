use lotw::{Engine, RoutineContext, game};

#[test]
fn check_actor_direction_contact_reports_player_contact() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        ..RoutineContext::default()
    };

    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x10;
    engine.state.player_y = 0x50;
    engine.state.player_health = 0x0A;
    engine.state.obj_state = 0x01;
    engine.state.obj_damage = 0x01;
    engine.state.obj_x_sub = 0x00;
    engine.state.obj_x_tile = 0x10;
    engine.state.obj_y_pixel = 0x50;

    game::check_actor_direction_contact(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.overlap_flag, 0x01);
    assert_eq!(engine.state.player_health, 0x09);
}

#[test]
fn tick_contact_trigger_actor_resets_when_no_direction_contacts_player() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x80;
    engine.state.player_y = 0xC0;
    engine.state.actor_record_ptr_lo = 0x00;
    engine.state.actor_record_ptr_hi = 0x20;
    engine.state.set_byte(0x2004, 0x33);
    engine.state.obj_move_state = 0x00;
    engine.state.obj_x_sub = 0x00;
    engine.state.obj_x_tile = 0x10;
    engine.state.obj_y_pixel = 0x50;

    game::tick_contact_trigger_actor(&mut engine, &mut r);

    assert_eq!(engine.state.obj_move_state, 0x00);
    assert_eq!(engine.state.obj_health, 0x33);
    assert_eq!(engine.state.obj_y_extra, 0x00);
    assert_eq!(r.value, 0x00);
}

#[test]
fn tick_timed_chase_actor_clears_actor_when_timer_expires() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.obj_state = 0x01;
    engine.state.obj_cooldown = 0x01;

    game::tick_timed_chase_actor(&mut engine, &mut r);

    assert_eq!(engine.state.obj_cooldown, 0x00);
    assert_eq!(engine.state.obj_state, 0x00);
    assert_eq!(r.value, 0x00);
}
