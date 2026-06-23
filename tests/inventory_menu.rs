use lotw::{Engine, RoutineContext, game};

#[test]
fn inventory_grid_cursor_wraps_horizontally_and_vertically() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_x_vel_lo(0x06);
    game::move_inventory_cursor_right(&mut engine, &mut r);
    assert_eq!(engine.state.obj_x_vel_lo(), 0x00);

    game::move_inventory_cursor_left(&mut engine, &mut r);
    assert_eq!(engine.state.obj_x_vel_lo(), 0x06);

    engine.state.set_obj_y_vel(0x04);
    game::move_inventory_cursor_down(&mut engine, &mut r);
    assert_eq!(engine.state.obj_y_vel(), 0x00);

    game::move_inventory_cursor_up(&mut engine, &mut r);
    assert_eq!(engine.state.obj_y_vel(), 0x04);
}

#[test]
fn inventory_grid_cursor_sprites_follow_grid_coordinates() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_x_vel_lo(0x02);
    engine.state.set_obj_y_vel(0x03);

    game::update_inventory_grid_cursor_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_x(0x94), 0x46);
    assert_eq!(engine.state.oam_x(0x90), 0x3E);
    assert_eq!(engine.state.oam_y(0x90), 0x99);
    assert_eq!(engine.state.oam_y(0x94), 0x99);
    assert_eq!(r.value, 0x99);
}

#[test]
fn inventory_list_cursor_uses_low_five_bits() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_x_sub(0x3F);
    game::set_inventory_list_buffer_index(&mut engine, &mut r);
    assert_eq!(r.index, 0x1F);

    game::update_inventory_list_cursor_sprites(&mut engine, &mut r);
    assert_eq!(engine.state.oam_y(0x80), 0x69);
    assert_eq!(engine.state.oam_y(0x84), 0x69);
    assert_eq!(engine.state.oam_x(0x80), 0xBE);
    assert_eq!(engine.state.oam_x(0x84), 0xC6);
}

#[test]
fn selecting_inventory_grid_entry_writes_to_scrolling_list() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_x_vel_lo(0x01);
    engine.state.set_obj_y_vel(0x02);
    engine.state.set_obj_x_sub(0x03);

    game::select_inventory_grid_entry(&mut engine, &mut r);

    assert_eq!(engine.state.password_nibbles_a(0x03), 0x07);
    assert_eq!(engine.state.obj_x_sub(), 0x04);
    assert_eq!(r.index, 0x61);
}
