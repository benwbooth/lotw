use lotw::{Engine, RoutineContext, game};

#[test]
fn camera_scroll_tracks_player_edges_and_marks_column_direction() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scroll_fine_x = 0x00;
    engine.state.scroll_tile_x = 0x04;
    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x06;

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.state.scroll_fine_x, 0x00);
    assert_eq!(engine.state.scroll_tile_x, 0x00);
    assert_eq!(engine.state.camera_scroll_flag, 0xFF);
    assert_eq!(engine.state.scroll_pixel_x, 0x00);
    assert_eq!(engine.state.nametable_select, 0x00);
    assert_eq!(r.carry, 0);

    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x0A;

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.state.scroll_fine_x, 0x00);
    assert_eq!(engine.state.scroll_tile_x, 0x01);
    assert_eq!(engine.state.camera_scroll_flag, 0x01);
    assert_eq!(engine.state.scroll_pixel_x, 0x10);
    assert_eq!(engine.state.nametable_select, 0x00);
    assert_eq!(r.carry, 0);

    game::update_camera_scroll_from_player(&mut engine, &mut r);

    assert_eq!(engine.state.scroll_fine_x, 0x00);
    assert_eq!(engine.state.scroll_tile_x, 0x01);
    assert_eq!(r.carry, 1);
}

#[test]
fn scroll_register_shadows_split_pixel_scroll_and_nametable_bit() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scroll_fine_x = 0x05;
    engine.state.scroll_tile_x = 0x1A;

    game::refresh_scroll_register_shadows(&mut engine, &mut r);

    assert_eq!(engine.state.scroll_pixel_x, 0xA5);
    assert_eq!(engine.state.nametable_select, 0x01);
    assert_eq!(r.index, 0xA5);
    assert_eq!(r.value, 0x01);
}

#[test]
fn player_sprites_follow_camera_and_blink_timer_hides_them() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x04;
    engine.state.player_x_tile = 0x08;
    engine.state.player_y = 0x10;
    engine.state.player_pose = 0x20;
    engine.state.player_facing = 0x40;
    engine.state.scroll_fine_x = 0x01;
    engine.state.scroll_tile_x = 0x04;

    game::draw_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x10), 0x3B);
    assert_eq!(engine.state.oam_y(0x14), 0x3B);
    assert_eq!(engine.state.oam_x(0x10), 0x43);
    assert_eq!(engine.state.oam_x(0x14), 0x4B);
    assert_eq!(engine.state.oam_tile(0x10), 0x22);
    assert_eq!(engine.state.oam_tile(0x14), 0x20);

    engine.state.sprite_blink_timer = 0x01;
    engine.state.frame_prescaler = 0x00;

    game::draw_player_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x10), 0xEF);
    assert_eq!(engine.state.oam_y(0x14), 0xEF);
}

#[test]
fn status_item_sprites_draw_selection_and_hide_empty_item_slots() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.selected_item_slot = 0x01;
    engine.state.set_item_slot(0, 0x00);
    engine.state.set_item_slot(1, 0x81);
    engine.state.set_item_slot(2, 0x02);

    game::draw_status_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x38), 0x13);
    assert_eq!(engine.state.oam_y(0x3C), 0x13);
    assert_eq!(engine.state.oam_x(0x38), 0xD8);
    assert_eq!(engine.state.oam_x(0x3C), 0xE0);

    assert_eq!(engine.state.oam_y(0x30), 0x13);
    assert_eq!(engine.state.oam_tile(0x30), 0xA9);
    assert_eq!(engine.state.oam_x(0x30), 0xE8);

    assert_eq!(engine.state.oam_y(0x28), 0xEF);
    assert_eq!(engine.state.oam_y(0x2C), 0xEF);

    assert_eq!(engine.state.oam_y(0x20), 0x13);
    assert_eq!(engine.state.oam_tile(0x20), 0xA1);
    assert_eq!(engine.state.oam_x(0x20), 0xC8);
}

#[test]
fn object_slot_sprites_project_visible_slot_and_clear_one_shot_x_offset() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x80,
        offset: 0x00,
        ..RoutineContext::default()
    };

    engine.state.scroll_fine_x = 0x03;
    engine.state.scroll_tile_x = 0x02;
    engine.state.set_object_tile(0x00, 0x40);
    engine.state.set_object_state(0x00, 0x01);
    engine.state.set_object_attr(0x00, 0x00);
    engine.state.set_object_x_sub(0x00, 0x07);
    engine.state.set_object_x_tile(0x00, 0x05);
    engine.state.set_object_y_pixel(0x00, 0x20);
    engine.state.set_object_y_extra(0x00, 0x05);

    game::draw_object_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x80), 0x4B);
    assert_eq!(engine.state.oam_tile(0x80), 0x40);
    assert_eq!(engine.state.oam_attr(0x80), 0x00);
    assert_eq!(engine.state.oam_x(0x80), 0x39);
    assert_eq!(engine.state.oam_y(0x84), 0x4B);
    assert_eq!(engine.state.oam_tile(0x84), 0x42);
    assert_eq!(engine.state.oam_x(0x84), 0x41);
    assert_eq!(engine.state.object_y_extra(0x00), 0x00);

    engine.state.set_object_state(0x00, 0x00);

    game::draw_object_slot_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x80), 0xEF);
    assert_eq!(engine.state.oam_y(0x84), 0xEF);
}

#[test]
fn clear_oam_preserves_sprite_zero_template_and_hides_the_rest() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=3 {
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_F + offset) as u16 as i32,
            0x80 + offset,
        );
    }
    engine.state.set_oam_y(0x04, 0x11);
    engine.state.set_oam_x(0xFC, 0x22);

    game::clear_oam_with_sprite_zero_template(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x00), 0x80);
    assert_eq!(engine.state.oam_tile(0x00), 0x81);
    assert_eq!(engine.state.oam_attr(0x00), 0x82);
    assert_eq!(engine.state.oam_x(0x00), 0x83);
    assert_eq!(engine.state.oam_y(0x04), 0xF8);
    assert_eq!(engine.state.oam_x(0xFC), 0xF8);
    assert_eq!(r.index, 0x00);
}

#[test]
fn clear_name_tables_writes_blank_tiles_and_zero_attributes() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.ppu.mirror = 1;
    engine.ppu.vram.fill(0x77);
    engine.state.ppu_ctrl_shadow = 0xA8;
    engine.state.ppu_mask_shadow = 0x1E;

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
    assert_eq!(engine.state.ppu_ctrl_shadow, 0xA8);
    assert_eq!(engine.state.ppu_mask_shadow, 0x1E);
    assert_eq!(engine.state.statusbar_split_flag, 0x00);
    assert_eq!(engine.ppu.ctrl, 0xA8);
    assert_eq!(engine.ppu.mask, 0x06);
    assert_eq!(r.value, 0xA8);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.offset, 0x00);
}
