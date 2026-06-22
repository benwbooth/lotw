use lotw::{Engine, RoutineContext, game};

#[test]
fn snapshot_and_restore_inventory_state_round_trips_progress_inventory_and_currency() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..8 {
        engine.set_mem(0x0300 + offset, 0x20 + offset);
    }
    for offset in 0..16 {
        engine.set_mem(0x0060 + offset, 0x40 + offset);
    }
    engine.set_mem(0x5A, 0x12);
    engine.set_mem(0x5B, 0x34);

    game::snapshot_inventory_state(&mut engine, &mut r);

    for offset in 0..8 {
        engine.set_mem(0x0300 + offset, 0x00);
    }
    for offset in 0..16 {
        engine.set_mem(0x0060 + offset, 0x00);
    }
    engine.set_mem(0x5A, 0x00);
    engine.set_mem(0x5B, 0x00);

    game::restore_inventory_state_snapshot(&mut engine, &mut r);

    for offset in 0..8 {
        assert_eq!(engine.mem(0x0300 + offset), 0x20 + offset);
    }
    for offset in 0..16 {
        assert_eq!(engine.mem(0x0060 + offset), 0x40 + offset);
    }
    assert_eq!(engine.mem(0x5A), 0x12);
    assert_eq!(engine.mem(0x5B), 0x34);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn clear_inventory_item_list_buffer_blanks_all_item_name_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..32 {
        engine.set_mem(0x0322 + offset, 0x80 + offset);
    }

    game::clear_inventory_item_list_buffer(&mut engine, &mut r);

    for offset in 0..32 {
        assert_eq!(engine.mem(0x0322 + offset), 0x7F);
    }
    assert_eq!(r.value, 0x7F);
    assert_eq!(r.index, 0xFF);
}
