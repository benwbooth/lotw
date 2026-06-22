use lotw::{Engine, RoutineContext, game};

fn encoded_intro_tile(source_byte: i32) -> i32 {
    (((source_byte & 0xF0) << 1) | (source_byte & 0x0F)) & 0xFF
}

#[test]
fn title_oam_template_copies_full_sprite_block() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x7F {
        engine.set_mem(0xB71C + offset, 0x80 + offset);
    }

    game::load_title_oam_template(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0240), 0x80);
    assert_eq!(engine.mem(0x02BF), 0xFF);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn demo_oam_template_copies_small_sprite_block() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.set_mem(0xB6FC + offset, 0x40 + offset);
    }

    game::load_demo_oam_template(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0240), 0x40);
    assert_eq!(engine.mem(0x025F), 0x5F);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn demo_oam_blink_toggles_first_eight_sprite_y_positions() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::blink_demo_oam_sprites(&mut engine, &mut r);

    for offset in (0..=0x1C).step_by(4) {
        assert_eq!(engine.mem(0x0240 + offset), 0xEF);
    }
    assert_eq!(r.index, 0xEF);

    engine.set_mem(0x84, 0x10);
    game::blink_demo_oam_sprites(&mut engine, &mut r);

    for offset in (0..=0x1C).step_by(4) {
        assert_eq!(engine.mem(0x0240 + offset), 0x80);
    }
    assert_eq!(r.index, 0x80);
}

#[test]
fn sprite_y_clear_leaves_other_oam_bytes_untouched() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0xFF {
        engine.set_mem(0x0200 + offset, offset);
    }

    game::hide_all_sprite_y_positions(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0200), 0xEF);
    assert_eq!(engine.mem(0x0204), 0xEF);
    assert_eq!(engine.mem(0x02FC), 0xEF);
    assert_eq!(engine.mem(0x0201), 0x01);
    assert_eq!(engine.mem(0x02FF), 0xFF);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0xEF);
}

#[test]
fn text_staging_buffer_clears_to_blank_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.set_mem(0x0140 + offset, offset);
    }

    game::clear_text_staging_buffer(&mut engine, &mut r);

    for offset in 0..=0x1F {
        assert_eq!(engine.mem(0x0140 + offset), 0xC0);
    }
    assert_eq!(r.value, 0xC0);
    assert_eq!(r.offset, 0xFF);
}

#[test]
fn intro_text_vram_address_tracks_scroll_offset() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0A, 0x40);

    game::set_intro_text_vram_address(&mut engine, &mut r);

    assert_eq!(engine.mem(0x17), 0x21);
    assert_eq!(engine.mem(0x16), 0x00);
    assert_eq!(r.value, 0x00);

    engine.set_mem(0x0A, 0xF0);

    game::set_intro_text_vram_address(&mut engine, &mut r);

    assert_eq!(engine.mem(0x17), 0x23);
    assert_eq!(engine.mem(0x16), 0xC0);
    assert_eq!(r.value, 0xC0);
}

#[test]
fn intro_text_line_terminator_sets_carry_without_upload() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.set_mem(0x0140 + offset, 0x55);
    }
    engine.set_mem(0x0C, 0x00);
    engine.set_mem(0x0D, 0xB0);
    engine.set_mem(0xB000, 0x00);

    game::stage_intro_text_line(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.mem(0x28), 0);
    for offset in 0..=0x1F {
        assert_eq!(engine.mem(0x0140 + offset), 0xC0);
    }
}

#[test]
fn intro_text_line_stages_encoded_tiles_until_terminator() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0C, 0x00);
    engine.set_mem(0x0D, 0xB0);
    engine.set_mem(0xB000, 0x12);
    engine.set_mem(0xB001, 0x34);
    engine.set_mem(0xB002, 0x00);

    game::stage_intro_text_line(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0140), encoded_intro_tile(0x12));
    assert_eq!(engine.mem(0x0141), encoded_intro_tile(0x34));
    assert_eq!(engine.mem(0x0142), 0xC0);
    assert_eq!(engine.mem(0x28), 0x00);
    assert_eq!(engine.mem(0x0C), 0x00);
    assert_eq!(engine.mem(0x0D), 0xB0);
    assert_eq!(r.carry, 1);
}

#[test]
fn scrolling_intro_text_line_offsets_staged_tiles_until_terminator() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0C, 0xFE);
    engine.set_mem(0x0D, 0xB0);
    engine.set_mem(0xB0FE, 0x12);
    engine.set_mem(0xB0FF, 0x00);

    game::stage_scrolling_intro_text_line(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0140), (encoded_intro_tile(0x12) + 0x10) & 0xFF);
    assert_eq!(engine.mem(0x0141), 0xC0);
    assert_eq!(engine.mem(0x0C), 0xFE);
    assert_eq!(engine.mem(0x0D), 0xB0);
    assert_eq!(engine.mem(0x28), 0x00);
    assert_eq!(r.carry, 1);
}
