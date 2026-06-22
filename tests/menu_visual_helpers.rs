use lotw::{Engine, RoutineContext, game};

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
