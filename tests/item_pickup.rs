use lotw::{Engine, RoutineContext, game};

fn expect_u8(engine: &Engine, name: &str, addr: i32, want: i32) {
    assert_eq!(
        engine.state.byte(addr),
        want,
        "{name} ${addr:04X}: got {:02X}, expected {want:02X}",
        engine.state.byte(addr)
    );
}

#[test]
fn item_pickup_updates_inventory_and_clears_object() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x10;
    engine.state.player_y = 0x50;
    engine.state.horizontal_subtile_delta = 0x00;
    engine.state.vertical_delta = 0x00;
    engine.state.slot_index = 0xff;

    engine.state.set_object_tile(0x80, 0x02);
    engine.state.set_object_state(0x80, 0x0a);
    engine.state.set_object_attr(0x80, 0x00);
    engine.state.set_object_x_sub(0x80, 0x00);
    engine
        .state
        .set_object_x_tile(0x80, (engine.state.player_x_tile as i32));
    engine
        .state
        .set_object_y_pixel(0x80, (engine.state.player_y as i32));

    r.value = 0x00;
    game::try_move_player_with_collision(&mut engine, &mut r);

    expect_u8(&engine, "inventory count", 0x0060, 0x01);
    expect_u8(&engine, "object active", 0x0401 + 0x80, 0x00);
    expect_u8(&engine, "object y clear", 0x0406 + 0x80, 0xf0);
    expect_u8(&engine, "oam clear 0", 0x0200 + ((8 << 3) | 0x80), 0xef);
    expect_u8(&engine, "oam clear 1", 0x0204 + ((8 << 3) | 0x80), 0xef);
}
