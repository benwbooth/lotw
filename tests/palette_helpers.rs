use lotw::{Engine, RoutineContext, game};

#[test]
fn dim_palette_range_by_step_saturates_to_black() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x00,
        offset: 0x03,
        ..RoutineContext::default()
    };

    engine.set_mem(0x09, 0x20);
    engine.set_mem(0x0180, 0x3A);
    engine.set_mem(0x0181, 0x1B);
    engine.set_mem(0x0182, 0x0C);

    game::dim_palette_range_by_step(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0180), 0x1A);
    assert_eq!(engine.mem(0x0181), 0x0F);
    assert_eq!(engine.mem(0x0182), 0x0F);
    assert_eq!(r.index, 0x03);
    assert_eq!(r.offset, 0x00);
}
