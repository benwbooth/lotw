use lotw::{Engine, RoutineContext, game};

#[test]
fn room_persistent_flag_read_and_clear_use_current_map_bit() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x47, 0x02);
    engine.set_mem(0x48, 0x00);
    engine.set_mem(0x0302, 0x40);

    game::read_room_persistent_flag(&mut engine, &mut r);
    assert_eq!(r.value, 0x80);

    engine.set_mem(0x0302, 0xFF);
    game::clear_room_persistent_flag(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0302), 0x7F);
    assert_eq!(r.index, 0x02);
    assert_eq!(r.value, 0x7F);

    engine.set_mem(0x47, 0x03);
    engine.set_mem(0x48, 0x02);
    engine.set_mem(0x0303, 0xFF);

    game::clear_room_persistent_flag(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0303), 0xBF);
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

    engine.set_mem(0x40, 0x00);
    engine.set_mem(0xFFBB, 0x80);

    game::load_family_item_permission_bits(&mut engine, &mut r);

    assert_eq!(r.value, 0x00);
    assert_eq!(r.carry, 0x01);

    engine.set_mem(0xFFBB, 0x40);
    game::load_family_item_permission_bits(&mut engine, &mut r);

    assert_eq!(r.value, 0x80);
    assert_eq!(r.carry, 0x00);
}
