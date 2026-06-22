use lotw::{Engine, RoutineContext, game};

#[test]
fn camera_scroll_tracks_player_edges_and_marks_column_direction() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x7B, 0x00);
    engine.set_mem(0x7C, 0x04);
    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x06);

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.mem(0x7B), 0x00);
    assert_eq!(engine.mem(0x7C), 0x00);
    assert_eq!(engine.mem(0x7F), 0xFF);
    assert_eq!(engine.mem(0x1C), 0x00);
    assert_eq!(engine.mem(0x1D), 0x00);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x43, 0x00);
    engine.set_mem(0x44, 0x0A);

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.mem(0x7B), 0x00);
    assert_eq!(engine.mem(0x7C), 0x01);
    assert_eq!(engine.mem(0x7F), 0x01);
    assert_eq!(engine.mem(0x1C), 0x10);
    assert_eq!(engine.mem(0x1D), 0x00);
    assert_eq!(r.carry, 0);

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.mem(0x7B), 0x00);
    assert_eq!(engine.mem(0x7C), 0x01);
    assert_eq!(r.carry, 1);
}

#[test]
fn scroll_register_shadows_split_pixel_scroll_and_nametable_bit() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x7B, 0x05);
    engine.set_mem(0x7C, 0x1A);

    game::refresh_scroll_register_shadows(&mut engine, &mut r);

    assert_eq!(engine.mem(0x1C), 0xA5);
    assert_eq!(engine.mem(0x1D), 0x01);
    assert_eq!(r.index, 0xA5);
    assert_eq!(r.value, 0x01);
}

#[test]
fn player_sprites_follow_camera_and_blink_timer_hides_them() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x43, 0x04);
    engine.set_mem(0x44, 0x08);
    engine.set_mem(0x45, 0x10);
    engine.set_mem(0x56, 0x20);
    engine.set_mem(0x57, 0x40);
    engine.set_mem(0x7B, 0x01);
    engine.set_mem(0x7C, 0x04);

    game::draw_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0210), 0x3B);
    assert_eq!(engine.mem(0x0214), 0x3B);
    assert_eq!(engine.mem(0x0213), 0x43);
    assert_eq!(engine.mem(0x0217), 0x4B);
    assert_eq!(engine.mem(0x0211), 0x22);
    assert_eq!(engine.mem(0x0215), 0x20);

    engine.set_mem(0x85, 0x01);
    engine.set_mem(0x84, 0x00);

    game::draw_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0210), 0xEF);
    assert_eq!(engine.mem(0x0214), 0xEF);
}

#[test]
fn status_item_sprites_draw_selection_and_hide_empty_item_slots() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x01);
    engine.set_mem(0x51, 0x00);
    engine.set_mem(0x52, 0x81);
    engine.set_mem(0x53, 0x02);

    game::draw_status_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0238), 0x13);
    assert_eq!(engine.mem(0x023C), 0x13);
    assert_eq!(engine.mem(0x023B), 0xD8);
    assert_eq!(engine.mem(0x023F), 0xE0);

    assert_eq!(engine.mem(0x0230), 0x13);
    assert_eq!(engine.mem(0x0231), 0xA9);
    assert_eq!(engine.mem(0x0233), 0xE8);

    assert_eq!(engine.mem(0x0228), 0xEF);
    assert_eq!(engine.mem(0x022C), 0xEF);

    assert_eq!(engine.mem(0x0220), 0x13);
    assert_eq!(engine.mem(0x0221), 0xA1);
    assert_eq!(engine.mem(0x0223), 0xC8);
}

#[test]
fn object_slot_sprites_project_visible_slot_and_clear_one_shot_x_offset() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x80,
        offset: 0x00,
        ..RoutineContext::default()
    };

    engine.set_mem(0x7B, 0x03);
    engine.set_mem(0x7C, 0x02);
    engine.set_mem(0x0400, 0x40);
    engine.set_mem(0x0401, 0x01);
    engine.set_mem(0x0402, 0x00);
    engine.set_mem(0x040C, 0x07);
    engine.set_mem(0x040D, 0x05);
    engine.set_mem(0x040E, 0x20);
    engine.set_mem(0x040F, 0x05);

    game::draw_object_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0280), 0x4B);
    assert_eq!(engine.mem(0x0281), 0x40);
    assert_eq!(engine.mem(0x0282), 0x00);
    assert_eq!(engine.mem(0x0283), 0x39);
    assert_eq!(engine.mem(0x0284), 0x4B);
    assert_eq!(engine.mem(0x0285), 0x42);
    assert_eq!(engine.mem(0x0287), 0x41);
    assert_eq!(engine.mem(0x040F), 0x00);

    engine.set_mem(0x0401, 0x00);

    game::draw_object_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0280), 0xEF);
    assert_eq!(engine.mem(0x0284), 0xEF);
}

#[test]
fn clear_oam_preserves_sprite_zero_template_and_hides_the_rest() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=3 {
        engine.set_mem(0xFF6B + offset, 0x80 + offset);
    }
    engine.set_mem(0x0204, 0x11);
    engine.set_mem(0x02FF, 0x22);

    game::clear_oam_with_sprite_zero_template(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0200), 0x80);
    assert_eq!(engine.mem(0x0201), 0x81);
    assert_eq!(engine.mem(0x0202), 0x82);
    assert_eq!(engine.mem(0x0203), 0x83);
    assert_eq!(engine.mem(0x0204), 0xF8);
    assert_eq!(engine.mem(0x02FF), 0xF8);
    assert_eq!(r.index, 0x00);
}

#[test]
fn clear_name_tables_writes_blank_tiles_and_zero_attributes() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.ppu.mirror = 1;
    engine.ppu.vram.fill(0x77);
    engine.set_mem(0x23, 0xA8);
    engine.set_mem(0x24, 0x1E);

    game::clear_name_tables_to_blank_tiles(&mut engine, &mut r);

    for base in [0x000, 0x400] {
        assert!(
            engine.ppu.vram[base..base + 0x3C0]
                .iter()
                .all(|tile| *tile == 0xC0)
        );
        assert!(
            engine.ppu.vram[base + 0x3C0..base + 0x400]
                .iter()
                .all(|attr| *attr == 0x00)
        );
    }
    assert_eq!(engine.mem(0x23), 0xA8);
    assert_eq!(engine.mem(0x24), 0x1E);
    assert_eq!(engine.mem(0x29), 0x00);
    assert_eq!(engine.ppu.ctrl, 0xA8);
    assert_eq!(engine.ppu.mask, 0x06);
    assert_eq!(r.value, 0xA8);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.offset, 0x00);
}
