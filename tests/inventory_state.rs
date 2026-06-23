use lotw::{Engine, RoutineContext, game};

#[test]
fn snapshot_and_restore_inventory_state_round_trips_progress_inventory_and_currency() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..8 {
        engine.state.set_save_payload(offset, 0x20 + offset);
    }
    for offset in 0..16 {
        engine.state.set_inventory_item(offset, 0x40 + offset);
    }
    engine.state.coins = 0x12;
    engine.state.keys = 0x34;

    game::snapshot_inventory_state(&mut engine, &mut r);

    for offset in 0..8 {
        engine.state.set_save_payload(offset, 0x00);
    }
    for offset in 0..16 {
        engine.state.set_inventory_item(offset, 0x00);
    }
    engine.state.coins = 0x00;
    engine.state.keys = 0x00;

    game::restore_inventory_state_snapshot(&mut engine, &mut r);

    for offset in 0..8 {
        assert_eq!(engine.state.save_payload(offset), 0x20 + offset);
    }
    for offset in 0..16 {
        assert_eq!(engine.state.inventory_item(offset), 0x40 + offset);
    }
    assert_eq!(engine.state.coins, 0x12);
    assert_eq!(engine.state.keys, 0x34);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn clear_inventory_item_list_buffer_blanks_all_item_name_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..32 {
        engine.state.set_password_nibbles_a(offset, 0x80 + offset);
    }

    game::clear_inventory_item_list_buffer(&mut engine, &mut r);

    for offset in 0..32 {
        assert_eq!(engine.state.password_nibbles_a(offset), 0x7F);
    }
    assert_eq!(r.value, 0x7F);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn inventory_item_list_codec_round_trips_snapshot_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..8 {
        engine.state.set_save_progress(offset, 0x10 + offset);
    }
    for offset in 0..16 {
        engine.state.set_save_inventory(offset, offset);
    }
    engine.state.set_save_inventory(0x10, 0xA5);
    engine.state.set_save_inventory(0x11, 0x5A);

    game::encode_inventory_snapshot_item_list(&mut engine, &mut r);

    assert!((0..32).all(|offset| engine.state.password_nibbles_a(offset) < 0x20));

    for offset in 0..8 {
        engine.state.set_save_progress(offset, 0x00);
    }
    for offset in 0..16 {
        engine.state.set_save_inventory(offset, 0x00);
    }
    engine.state.set_save_inventory(0x10, 0x00);
    engine.state.set_save_inventory(0x11, 0x00);

    game::decode_inventory_item_list_snapshot(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    for offset in 0..8 {
        assert_eq!(engine.state.save_progress(offset), 0x10 + offset);
    }
    for offset in 0..16 {
        assert_eq!(engine.state.save_inventory(offset), offset);
    }
    assert_eq!(engine.state.save_inventory(0x10), 0xA5);
    assert_eq!(engine.state.save_inventory(0x11), 0x5A);
}

#[test]
fn inventory_item_list_decoder_rejects_checksum_mismatch() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..8 {
        engine.state.set_save_progress(offset, 0x20 + offset);
    }
    for offset in 0..16 {
        engine.state.set_save_inventory(offset, 0x0F - offset);
    }
    engine.state.set_save_inventory(0x10, 0x12);
    engine.state.set_save_inventory(0x11, 0x34);

    game::encode_inventory_snapshot_item_list(&mut engine, &mut r);
    engine
        .state
        .set_password_nibbles_a(0, engine.state.password_nibbles_a(0) ^ 0x01);

    game::decode_inventory_item_list_snapshot(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.prompt_state, 0x1C);
    assert_eq!(engine.state.prompt_argument, 0x1C);
}
