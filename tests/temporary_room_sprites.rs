use lotw::{Engine, RoutineContext, game};

#[test]
fn carried_item_sprites_show_owned_items_and_hide_empty_slots() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x51, 0x01);
    engine.set_mem(0x52, 0xFF);
    engine.set_mem(0x53, 0x02);

    game::draw_carried_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0240), 0xBB);
    assert_eq!(engine.mem(0x0244), 0xBB);
    assert_eq!(engine.mem(0x0241), 0xA5);
    assert_eq!(engine.mem(0x0245), 0xA7);
    assert_eq!(engine.mem(0x0243), 0x18);
    assert_eq!(engine.mem(0x0247), 0x20);

    assert_eq!(engine.mem(0x0248), 0xEF);
    assert_eq!(engine.mem(0x024C), 0xEF);

    assert_eq!(engine.mem(0x0250), 0xBB);
    assert_eq!(engine.mem(0x0254), 0xBB);
    assert_eq!(engine.mem(0x0251), 0xA9);
    assert_eq!(engine.mem(0x0255), 0xAB);
    assert_eq!(engine.mem(0x0253), 0x58);
    assert_eq!(engine.mem(0x0257), 0x60);

    assert_eq!(r.index, 0xFF);
    assert_eq!(r.offset, 0xF8);
}

#[test]
fn shop_item_sprites_hide_overstocked_items() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x80, 0x01);
    engine.set_mem(0x82, 0x02);
    engine.set_mem(0x61, 0x0A);
    engine.set_mem(0x62, 0x0B);

    game::draw_shop_item_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0240), 0xA4);
    assert_eq!(engine.mem(0x0244), 0xA4);
    assert_eq!(engine.mem(0x0241), 0xA5);
    assert_eq!(engine.mem(0x0245), 0xA7);
    assert_eq!(engine.mem(0x0243), 0x40);
    assert_eq!(engine.mem(0x0247), 0x48);

    assert_eq!(engine.mem(0x82), 0xEF);
    assert_eq!(engine.mem(0x0248), 0xEF);
    assert_eq!(engine.mem(0x024C), 0xEF);
}

#[test]
fn coin_cost_sprites_can_be_drawn_and_cleared() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    game::draw_coin_cost_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0250), 0x98);
    assert_eq!(engine.mem(0x0254), 0x98);
    assert_eq!(engine.mem(0x0251), 0xF1);
    assert_eq!(engine.mem(0x0255), 0xF3);
    assert_eq!(engine.mem(0x0253), 0x78);
    assert_eq!(engine.mem(0x0257), 0x80);
    assert_eq!(r.value, 0x80);

    game::clear_temporary_room_sprites(&mut engine, &mut r);

    for addr in [0x0240, 0x0244, 0x0248, 0x024C, 0x0250, 0x0254] {
        assert_eq!(engine.mem(addr), 0xEF);
    }
    assert_eq!(r.value, 0xEF);
}

#[test]
fn status_sprite_template_restores_oam_and_bank_shadows() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x37 {
        engine.set_mem(0xFF6F + offset, 0x80 + offset);
    }

    game::restore_status_sprite_template(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0280), 0x80);
    assert_eq!(engine.mem(0x02B7), 0xB7);
    assert_eq!(engine.mem(0x2C), 0x34);
    assert_eq!(engine.mem(0x2D), 0x35);
    assert_eq!(engine.mem(0x2E), 0x36);
    assert_eq!(engine.mem(0x2F), 0x37);
    assert_eq!(r.index, 0xFF);
    assert_eq!(r.value, 0x37);
}
