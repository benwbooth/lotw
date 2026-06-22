use lotw::{Engine, RoutineContext, game};

#[test]
fn selected_item_effect_starts_magic_timer_when_magic_is_available() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x00);
    engine.set_mem(0x51, 0x00);
    engine.set_mem(0x59, 0x01);

    game::tick_selected_item_effect(&mut engine, &mut r);

    assert_eq!(engine.mem(0x59), 0x00);
    assert_eq!(engine.mem(0x86), 0x02);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.carry, 0);
}

#[test]
fn selected_item_effect_reports_missing_magic_once_continue_timer_is_active() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x00);
    engine.set_mem(0x51, 0x01);
    engine.set_mem(0x59, 0x00);
    engine.set_mem(0x37, 0x01);

    game::tick_selected_item_effect(&mut engine, &mut r);

    assert_eq!(engine.mem(0x87), 0x00);
    assert_eq!(engine.mem(0x37), 0xFD);
    assert_eq!(engine.mem(0x8F), 0x1A);
}

#[test]
fn final_exit_trigger_requires_selected_item_and_exact_position() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x00);
    engine.set_mem(0x51, 0x0F);
    engine.set_mem(0x47, 0x01);
    engine.set_mem(0x48, 0x05);
    engine.set_mem(0x7C, 0x10);
    engine.set_mem(0x7B, 0x00);
    engine.set_mem(0x45, 0xA0);

    game::check_final_exit_trigger(&mut engine, &mut r);
    assert_eq!(engine.mem(0xEC), 0x01);

    engine.set_mem(0xEC, 0x00);
    engine.set_mem(0x45, 0x90);
    game::check_final_exit_trigger(&mut engine, &mut r);
    assert_eq!(engine.mem(0xEC), 0x00);
}
