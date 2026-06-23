use lotw::{Engine, RoutineContext, game};

#[test]
fn carried_item_sprites_show_owned_items_and_hide_empty_slots() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_item_slot(0, 0x01);
    engine.state.set_item_slot(1, 0xFF);
    engine.state.set_item_slot(2, 0x02);

    game::draw_carried_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x40), 0xBB);
    assert_eq!(engine.state.oam_y(0x44), 0xBB);
    assert_eq!(engine.state.oam_tile(0x40), 0xA5);
    assert_eq!(engine.state.oam_tile(0x44), 0xA7);
    assert_eq!(engine.state.oam_x(0x40), 0x18);
    assert_eq!(engine.state.oam_x(0x44), 0x20);

    assert_eq!(engine.state.oam_y(0x48), 0xEF);
    assert_eq!(engine.state.oam_y(0x4C), 0xEF);

    assert_eq!(engine.state.oam_y(0x50), 0xBB);
    assert_eq!(engine.state.oam_y(0x54), 0xBB);
    assert_eq!(engine.state.oam_tile(0x50), 0xA9);
    assert_eq!(engine.state.oam_tile(0x54), 0xAB);
    assert_eq!(engine.state.oam_x(0x50), 0x58);
    assert_eq!(engine.state.oam_x(0x54), 0x60);

    assert_eq!(r.index, 0xFF);
    assert_eq!(r.offset, 0xF8);
}

#[test]
fn shop_item_sprites_hide_overstocked_items() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_temp_save(0, 0x01);
    engine.state.set_temp_save(2, 0x02);
    engine.state.set_shop_active(0x0A);
    engine.state.set_inventory_item(2, 0x0B);

    game::draw_shop_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x40), 0xA4);
    assert_eq!(engine.state.oam_y(0x44), 0xA4);
    assert_eq!(engine.state.oam_tile(0x40), 0xA5);
    assert_eq!(engine.state.oam_tile(0x44), 0xA7);
    assert_eq!(engine.state.oam_x(0x40), 0x40);
    assert_eq!(engine.state.oam_x(0x44), 0x48);

    assert_eq!(engine.state.temp_save(2), 0xEF);
    assert_eq!(engine.state.oam_y(0x48), 0xEF);
    assert_eq!(engine.state.oam_y(0x4C), 0xEF);
}

#[test]
fn coin_cost_sprites_can_be_drawn_and_cleared() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::draw_coin_cost_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x50), 0x98);
    assert_eq!(engine.state.oam_y(0x54), 0x98);
    assert_eq!(engine.state.oam_tile(0x50), 0xF1);
    assert_eq!(engine.state.oam_tile(0x54), 0xF3);
    assert_eq!(engine.state.oam_x(0x50), 0x78);
    assert_eq!(engine.state.oam_x(0x54), 0x80);
    assert_eq!(r.value, 0x80);

    game::clear_temporary_room_sprites(&mut engine, &mut r);

    for addr in [0x0240, 0x0244, 0x0248, 0x024C, 0x0250, 0x0254] {
        assert_eq!(engine.state.byte(addr), 0xEF);
    }
    assert_eq!(r.value, 0xEF);
}

#[test]
fn status_sprite_template_restores_oam_and_bank_shadows() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x37 {
        engine.state.set_byte(
            (lotw::game::SPRITE_Y_TABLE_G + offset) as u16 as i32,
            0x80 + offset,
        );
    }

    game::restore_status_sprite_template(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x80), 0x80);
    assert_eq!(engine.state.oam_x(0xB4), 0xB7);
    assert_eq!(engine.state.chr_bank(2), 0x34);
    assert_eq!(engine.state.chr_bank(3), 0x35);
    assert_eq!(engine.state.chr_bank(4), 0x36);
    assert_eq!(engine.state.chr_bank(5), 0x37);
    assert_eq!(r.index, 0xFF);
    assert_eq!(r.value, 0x37);
}
