use lotw::{Engine, RoutineContext, game};

#[test]
fn selected_item_effect_starts_magic_timer_when_magic_is_available() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.selected_item_slot = 0x00;
    engine.state.set_item_slot(0, 0x00);
    engine.state.player_magic = 0x01;

    game::tick_selected_item_effect(&mut engine, &mut r);

    assert_eq!(engine.state.player_magic, 0x00);
    assert_eq!(engine.state.airborne_flag, 0x02);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.carry, 0);
}

#[test]
fn selected_item_effect_reports_missing_magic_once_continue_timer_is_active() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.selected_item_slot = 0x00;
    engine.state.set_item_slot(0, 0x01);
    engine.state.player_magic = 0x00;
    engine.state.continue_timer = 0x01;

    game::tick_selected_item_effect(&mut engine, &mut r);

    assert_eq!(engine.state.magic_contact_flag, 0x00);
    assert_eq!(engine.state.continue_timer, 0xFD);
    assert_eq!(engine.state.prompt_state, 0x1A);
}

#[test]
fn final_exit_trigger_requires_selected_item_and_exact_position() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.selected_item_slot = 0x00;
    engine.state.set_item_slot(0, 0x0F);
    engine.state.map_screen_x = 0x01;
    engine.state.map_screen_y = 0x05;
    engine.state.scroll_tile_x = 0x10;
    engine.state.scroll_fine_x = 0x00;
    engine.state.player_y = 0xA0;

    game::check_final_exit_trigger(&mut engine, &mut r);
    assert_eq!(engine.state.final_exit_flag, 0x01);

    engine.state.final_exit_flag = 0x00;
    engine.state.player_y = 0x90;
    game::check_final_exit_trigger(&mut engine, &mut r);
    assert_eq!(engine.state.final_exit_flag, 0x00);
}
