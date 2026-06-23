use lotw::{Engine, RoutineContext, game};

fn encoded_intro_tile(source_byte: i32) -> i32 {
    (((source_byte & 0xF0) << 1) | (source_byte & 0x0F)) & 0xFF
}

#[test]
fn title_oam_template_copies_full_sprite_block() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x7F {
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_D + offset) as u16 as i32,
            0x80 + offset,
        );
    }

    game::load_title_oam_template(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x40), 0x80);
    assert_eq!(engine.state.oam_x(0xBC), 0xFF);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn demo_oam_template_copies_small_sprite_block() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_E + offset) as u16 as i32,
            0x40 + offset,
        );
    }

    game::load_demo_oam_template(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x40), 0x40);
    assert_eq!(engine.state.oam_x(0x5C), 0x5F);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn demo_oam_blink_toggles_first_eight_sprite_y_positions() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::blink_demo_oam_sprites(&mut engine, &mut r);

    for offset in (0..=0x1C).step_by(4) {
        assert_eq!(engine.state.oam_y(0x40 + offset), 0xEF);
    }
    assert_eq!(r.index, 0xEF);

    engine.state.set_frame_prescaler(0x10);
    game::blink_demo_oam_sprites(&mut engine, &mut r);

    for offset in (0..=0x1C).step_by(4) {
        assert_eq!(engine.state.oam_y(0x40 + offset), 0x80);
    }
    assert_eq!(r.index, 0x80);
}

#[test]
fn sprite_y_clear_leaves_other_oam_bytes_untouched() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0xFF {
        engine.state.set_oam_y(offset, offset);
    }

    game::hide_all_sprite_y_positions(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x00), 0xEF);
    assert_eq!(engine.state.oam_y(0x04), 0xEF);
    assert_eq!(engine.state.oam_y(0xFC), 0xEF);
    assert_eq!(engine.state.oam_tile(0x00), 0x01);
    assert_eq!(engine.state.oam_x(0xFC), 0xFF);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0xEF);
}

#[test]
fn text_staging_buffer_clears_to_blank_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.state.set_vram_stage(offset, offset);
    }

    game::clear_text_staging_buffer(&mut engine, &mut r);

    for offset in 0..=0x1F {
        assert_eq!(engine.state.vram_stage(offset), 0xC0);
    }
    assert_eq!(r.value, 0xC0);
    assert_eq!(r.offset, 0xFF);
}

#[test]
fn intro_text_vram_address_tracks_scroll_offset() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_scratch2(0x40);

    game::set_intro_text_vram_address(&mut engine, &mut r);

    assert_eq!(engine.state.vram_addr_hi(), 0x21);
    assert_eq!(engine.state.vram_addr_lo(), 0x00);
    assert_eq!(r.value, 0x00);

    engine.state.set_scratch2(0xF0);

    game::set_intro_text_vram_address(&mut engine, &mut r);

    assert_eq!(engine.state.vram_addr_hi(), 0x23);
    assert_eq!(engine.state.vram_addr_lo(), 0xC0);
    assert_eq!(r.value, 0xC0);
}

#[test]
fn intro_text_line_terminator_sets_carry_without_upload() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.state.set_vram_stage(offset, 0x55);
    }
    engine.state.set_data_ptr_lo(0x00);
    engine.state.set_data_ptr_hi(0xB0);
    engine
        .state
        .set_byte(lotw::game::PALETTE_DATA_TABLE + 0, 0x00);

    game::stage_intro_text_line(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.nmi_vram_req(), 0);
    for offset in 0..=0x1F {
        assert_eq!(engine.state.vram_stage(offset), 0xC0);
    }
}

#[test]
fn intro_text_line_stages_encoded_tiles_until_terminator() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_data_ptr_lo(0x00);
    engine.state.set_data_ptr_hi(0xB0);
    engine
        .state
        .set_byte(lotw::game::PALETTE_DATA_TABLE + 0, 0x12);
    engine
        .state
        .set_byte(lotw::game::PALETTE_DATA_TABLE + 1, 0x34);
    engine
        .state
        .set_byte(lotw::game::PALETTE_DATA_TABLE + 2, 0x00);

    game::stage_intro_text_line(&mut engine, &mut r);

    assert_eq!(engine.state.vram_stage(0x00), encoded_intro_tile(0x12));
    assert_eq!(engine.state.vram_stage(0x01), encoded_intro_tile(0x34));
    assert_eq!(engine.state.vram_stage(0x02), 0xC0);
    assert_eq!(engine.state.nmi_vram_req(), 0x00);
    assert_eq!(engine.state.data_ptr_lo(), 0x00);
    assert_eq!(engine.state.data_ptr_hi(), 0xB0);
    assert_eq!(r.carry, 1);
}

#[test]
fn scrolling_intro_text_line_offsets_staged_tiles_until_terminator() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_data_ptr_lo(0xFE);
    engine.state.set_data_ptr_hi(0xB0);
    engine.state.set_byte(lotw::game::DEMO_INPUT_TABLE, 0x12);
    engine
        .state
        .set_byte(lotw::game::DEMO_INPUT_TABLE + 1, 0x00);

    game::stage_scrolling_intro_text_line(&mut engine, &mut r);

    assert_eq!(
        engine.state.vram_stage(0x00),
        (encoded_intro_tile(0x12) + 0x10) & 0xFF
    );
    assert_eq!(engine.state.vram_stage(0x01), 0xC0);
    assert_eq!(engine.state.data_ptr_lo(), 0xFE);
    assert_eq!(engine.state.data_ptr_hi(), 0xB0);
    assert_eq!(engine.state.nmi_vram_req(), 0x00);
    assert_eq!(r.carry, 1);
}
