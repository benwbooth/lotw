use lotw::{Engine, RoutineContext, game};

fn expect_u8(engine: &Engine, name: &str, addr: i32, want: i32) {
    assert_eq!(
        engine.mem(addr),
        want,
        "{name} ${addr:04X}: got {:02X}, expected {want:02X}",
        engine.mem(addr)
    );
}

#[test]
fn item_pickup_updates_inventory_and_clears_object() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x10);
    engine.set_mem(0x45, 0x50);
    engine.set_mem(0x49, 0x00);
    engine.set_mem(0x4b, 0x00);
    engine.set_mem(0xe3, 0xff);

    engine.set_mem(0x0400 + 0x80, 0x02);
    engine.set_mem(0x0401 + 0x80, 0x0a);
    engine.set_mem(0x0402 + 0x80, 0x00);
    engine.set_mem(0x040c + 0x80, 0x00);
    engine.set_mem(0x040d + 0x80, engine.mem(0x44));
    engine.set_mem(0x040e + 0x80, engine.mem(0x45));

    r.value = 0x00;
    game::try_move_player_with_collision(&mut engine, &mut r);

    expect_u8(&engine, "inventory count", 0x0060, 0x01);
    expect_u8(&engine, "object active", 0x0401 + 0x80, 0x00);
    expect_u8(&engine, "object y clear", 0x0406 + 0x80, 0xf0);
    expect_u8(&engine, "oam clear 0", 0x0200 + ((8 << 3) | 0x80), 0xef);
    expect_u8(&engine, "oam clear 1", 0x0204 + ((8 << 3) | 0x80), 0xef);
}
