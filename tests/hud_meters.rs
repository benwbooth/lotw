use lotw::{Engine, RoutineContext, game};

#[test]
fn split_meter_value_returns_full_blocks_and_partial_block() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for (value, expected_partial, expected_blocks) in [
        (0x00, 0x01, 0x01),
        (0x09, 0x0A, 0x01),
        (0x0A, 0x01, 0x02),
        (0x17, 0x04, 0x03),
    ] {
        engine.state.scratch0 = value;
        game::split_meter_value(&mut engine, &mut r);

        assert_eq!(engine.state.scratch0, expected_partial, "value {value}");
        assert_eq!(r.value as i32, ((expected_partial) as i32), "value {value}");
        assert_eq!(r.offset as i32, expected_blocks, "value {value}");
    }
}

#[test]
fn decimal_digit_tiles_encode_ones_and_tens() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x0E,
        ..RoutineContext::default()
    };

    game::build_decimal_digit_tiles(&mut engine, &mut r);

    assert_eq!(engine.state.vram_addr2_lo, 0xD4);
    assert_eq!(engine.state.vram_addr2_hi, 0xD1);

    r.value = 0x09;
    game::build_decimal_digit_tiles(&mut engine, &mut r);

    assert_eq!(engine.state.vram_addr2_lo, 0xD9);
    assert_eq!(engine.state.vram_addr2_hi, 0xC0);
}

#[test]
fn build_health_meter_sprites_initializes_full_and_empty_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x65,
        offset: 0x6B,
        ..RoutineContext::default()
    };

    engine.state.scratch0 = 0x00;
    engine.state.scratch1 = 0x00;

    game::build_health_meter_sprites(&mut engine, &mut r);

    for addr in [0x0259, 0x025D, 0x0261, 0x0265, 0x0269] {
        assert_eq!(engine.state.byte(addr), 0x65);
    }
    for addr in [0x026D, 0x0271, 0x0275, 0x0279, 0x027D] {
        assert_eq!(engine.state.byte(addr), 0x6B);
    }
}
