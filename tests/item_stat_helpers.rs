use lotw::{Engine, RoutineContext, game};

#[test]
fn load_effective_jump_duration_boosts_selected_jump_item_when_magic_available() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x01);
    engine.set_mem(0x52, 0x06);
    engine.set_mem(0x59, 0x01);
    engine.set_mem(0x5C, 0x10);

    game::load_effective_jump_duration(&mut engine, &mut r);

    assert_eq!(r.index, 0x01);
    assert_eq!(r.value, 0x14);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x59, 0x00);
    game::load_effective_jump_duration(&mut engine, &mut r);

    assert_eq!(r.value, 0x10);
    assert_eq!(r.carry, 1);
}

#[test]
fn load_effective_projectile_stats_apply_magic_boost_items() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x55, 0x00);
    engine.set_mem(0x51, 0x08);
    engine.set_mem(0x59, 0x02);
    engine.set_mem(0x5D, 0x03);

    game::load_effective_projectile_damage(&mut engine, &mut r);

    assert_eq!(r.value, 0x0C);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x51, 0x09);
    engine.set_mem(0x5F, 0x12);
    game::load_effective_projectile_lifetime(&mut engine, &mut r);

    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0x24);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x59, 0x00);
    game::load_effective_projectile_lifetime(&mut engine, &mut r);

    assert_eq!(r.value, 0x12);
    assert_eq!(r.carry, 1);
}

#[test]
fn clear_gameplay_object_sprites_hides_only_object_half_of_oam() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x0200, 0x11);
    engine.set_mem(0x0280, 0x22);
    engine.set_mem(0x02FC, 0x33);

    game::clear_gameplay_object_sprites(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0200), 0x11);
    assert_eq!(engine.mem(0x0280), 0xEF);
    assert_eq!(engine.mem(0x02FC), 0xEF);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0xEF);
}

#[test]
fn reset_room_object_slots_clears_active_state_and_scheduler() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for slot in 0..16 {
        let slot_offset = slot * 0x10;
        engine.set_mem(0x0401 + slot_offset, 0x80 + slot);
        engine.set_mem(0x0406 + slot_offset, 0xF0);
    }
    engine.set_mem(0xE9, 0x02);

    game::reset_room_object_slots(&mut engine, &mut r);

    for slot in 0..16 {
        let slot_offset = slot * 0x10;
        assert_eq!(engine.mem(0x0401 + slot_offset), 0x00);
        assert_eq!(engine.mem(0x0406 + slot_offset), 0x02);
    }
    assert_eq!(engine.mem(0xE9), 0x00);
    assert_eq!(r.value, 0x00);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.offset, 0x00);
}
