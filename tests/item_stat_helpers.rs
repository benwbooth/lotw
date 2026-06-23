use lotw::{Engine, RoutineContext, game};

#[test]
fn load_effective_jump_duration_boosts_selected_jump_item_when_magic_available() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_selected_item_slot(0x01);
    engine.state.set_item_slot(1, 0x06);
    engine.state.player_magic = 0x01;
    engine.state.set_jump_strength(0x10);

    game::load_effective_jump_duration(&mut engine, &mut r);

    assert_eq!(r.index, 0x01);
    assert_eq!(r.value, 0x14);
    assert_eq!(r.carry, 0);

    engine.state.player_magic = 0x00;
    game::load_effective_jump_duration(&mut engine, &mut r);

    assert_eq!(r.value, 0x10);
    assert_eq!(r.carry, 1);
}

#[test]
fn load_effective_projectile_stats_apply_magic_boost_items() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_selected_item_slot(0x00);
    engine.state.set_item_slot(0, 0x08);
    engine.state.player_magic = 0x02;
    engine.state.set_projectile_damage(0x03);

    game::load_effective_projectile_damage(&mut engine, &mut r);

    assert_eq!(r.value, 0x0C);
    assert_eq!(r.carry, 0);

    engine.state.set_item_slot(0, 0x09);
    engine.state.set_projectile_lifetime(0x12);
    game::load_effective_projectile_lifetime(&mut engine, &mut r);

    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0x24);
    assert_eq!(r.carry, 0);

    engine.state.player_magic = 0x00;
    game::load_effective_projectile_lifetime(&mut engine, &mut r);

    assert_eq!(r.value, 0x12);
    assert_eq!(r.carry, 1);
}

#[test]
fn clear_gameplay_object_sprites_hides_only_object_half_of_oam() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_oam_y(0x00, 0x11);
    engine.state.set_oam_y(0x80, 0x22);
    engine.state.set_oam_y(0xFC, 0x33);

    game::clear_gameplay_object_sprites(&mut engine, &mut r);

    assert_eq!(engine.state.oam_y(0x00), 0x11);
    assert_eq!(engine.state.oam_y(0x80), 0xEF);
    assert_eq!(engine.state.oam_y(0xFC), 0xEF);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.value, 0xEF);
}

#[test]
fn reset_room_object_slots_clears_active_state_and_scheduler() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for slot in 0..16 {
        let slot_offset = slot * 0x10;
        engine.state.set_object_state(slot_offset, 0x80 + slot);
        engine.state.set_object_timer(slot_offset, 0xF0);
    }
    engine.state.set_scheduler_phase(0x02);

    game::reset_room_object_slots(&mut engine, &mut r);

    for slot in 0..16 {
        let slot_offset = slot * 0x10;
        assert_eq!(engine.state.object_state(slot_offset), 0x00);
        assert_eq!(engine.state.object_timer(slot_offset), 0x02);
    }
    assert_eq!(engine.state.scheduler_phase(), 0x00);
    assert_eq!(r.value, 0x00);
    assert_eq!(r.index, 0x00);
    assert_eq!(r.offset, 0x00);
}
