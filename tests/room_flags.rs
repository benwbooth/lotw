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
}
