use lotw::{Engine, RoutineContext, game};

#[test]
fn room_persistent_flag_read_and_clear_use_current_map_bit() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_map_screen_x(0x02);
    engine.state.set_map_screen_y(0x00);
    engine.state.set_save_payload(0x02, 0x40);

    game::read_room_persistent_flag(&mut engine, &mut r);
    assert_eq!(r.value, 0x80);

    engine.state.set_save_payload(0x02, 0xFF);
    game::clear_room_persistent_flag(&mut engine, &mut r);

    assert_eq!(engine.state.save_payload(0x02), 0x7F);
    assert_eq!(r.index, 0x02);
    assert_eq!(r.value, 0x7F);

    engine.state.set_map_screen_x(0x03);
    engine.state.set_map_screen_y(0x02);
    engine.state.set_save_payload(0x03, 0xFF);

    game::clear_room_persistent_flag(&mut engine, &mut r);

    assert_eq!(engine.state.save_payload(0x03), 0xBF);
    assert_eq!(r.index, 0x03);
    assert_eq!(r.value, 0xBF);
}

#[test]
fn family_item_permission_bits_return_shifted_value_and_carry() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x00,
        ..RoutineContext::default()
    };

    engine.state.set_character_index(0x00);
    engine
        .state
        .set_byte(lotw::game::MOVEMENT_PATTERN_TABLE, 0x80);

    game::load_family_item_permission_bits(&mut engine, &mut r);

    assert_eq!(r.value, 0x00);
    assert_eq!(r.carry, 0x01);

    engine
        .state
        .set_byte(lotw::game::MOVEMENT_PATTERN_TABLE, 0x40);
    game::load_family_item_permission_bits(&mut engine, &mut r);

    assert_eq!(r.value, 0x80);
    assert_eq!(r.carry, 0x00);
}
