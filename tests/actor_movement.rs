use lotw::{Engine, RoutineContext, game};

#[test]
fn project_actor_position_applies_subtile_carry_and_vertical_velocity() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_x_sub(0x03);
    engine.state.set_obj_x_tile(0x10);
    engine.state.set_obj_y_pixel(0x50);
    engine.state.set_obj_x_vel_lo(0x0E);
    engine.state.set_obj_x_vel_hi(0x00);
    engine.state.set_obj_y_vel(0x02);

    game::project_actor_position(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo(), 0x01);
    assert_eq!(engine.state.indirect_ptr_hi(), 0x11);
    assert_eq!(engine.state.scratch2(), 0x52);
}

#[test]
fn commit_and_stop_actor_helpers_update_scratch_motion_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_indirect_ptr_lo(0x04);
    engine.state.set_indirect_ptr_hi(0x22);
    engine.state.set_scratch2(0x66);
    engine.state.set_obj_x_vel_lo(0x01);
    engine.state.set_obj_y_vel(0x02);
    engine.state.set_obj_cooldown(0x03);
    engine.state.set_obj_move_scratch(0x04);

    game::commit_actor_projected_position(&mut engine, &mut r);

    assert_eq!(engine.state.obj_x_sub(), 0x04);
    assert_eq!(engine.state.obj_x_tile(), 0x22);
    assert_eq!(engine.state.obj_y_pixel(), 0x66);
    assert_eq!(r.value, 0x66);

    game::stop_actor_motion(&mut engine, &mut r);

    assert_eq!(engine.state.obj_x_vel_lo(), 0x00);
    assert_eq!(engine.state.obj_y_vel(), 0x00);
    assert_eq!(engine.state.obj_cooldown(), 0x00);
    assert_eq!(engine.state.obj_move_scratch(), 0x00);
}

#[test]
fn reverse_actor_horizontal_direction_flips_low_direction_bits() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_obj_move_state(0x01);
    game::reverse_actor_horizontal_direction(&mut engine, &mut r);
    assert_eq!(engine.state.obj_move_state(), 0x02);
    assert_eq!(r.value, 0x02);

    engine.state.set_obj_move_state(0x00);
    game::reverse_actor_horizontal_direction(&mut engine, &mut r);
    assert_eq!(engine.state.obj_move_state(), 0x02);
}

#[test]
fn update_actor_animation_dispatches_room_animation_mode() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_actor_record_ptr_lo(0x00);
    engine.state.set_actor_record_ptr_hi(0x20);
    engine.state.set_byte(0x2007, 0x03);
    engine.state.set_obj_timer(0x02);
    engine.state.set_obj_tile(0xF3);

    game::update_actor_animation(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo(), 0xB9);
    assert_eq!(engine.state.indirect_ptr_hi(), 0xF0);
    assert_eq!(engine.state.obj_timer(), 0x03);
    assert_eq!(engine.state.obj_tile() & 0x0C, 0x04);
    assert_eq!(r.index, 0x06);
    assert_eq!(r.value, 0x06);
}

#[test]
fn apply_actor_player_contact_damage_updates_health_and_hit_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_health = 0x0A;
    engine.state.set_obj_state(0x01);
    engine.state.set_obj_attr(0xFF);
    engine.state.set_obj_damage(0x02);

    game::apply_actor_player_contact_damage(&mut engine, &mut r);

    assert_eq!(engine.state.player_health, 0x08);
    assert_eq!(engine.state.sprite_blink_timer(), 0x01);
    assert_eq!(engine.state.prompt_state(), 0x21);
    assert_eq!(engine.state.prompt_argument(), 0x01);
    assert_eq!(engine.state.obj_attr() & 0x20, 0x00);
}

#[test]
fn try_move_large_actor_with_terrain_uses_wide_probe_and_restores_vertical_velocity() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_player_x_fine(0x00);
    engine.state.set_player_x_tile(0x80);
    engine.state.set_player_y(0xC0);
    engine.state.set_obj_state(0x01);
    engine.state.set_obj_x_vel_lo(0x02);
    engine.state.set_obj_x_vel_hi(0x00);
    engine.state.set_obj_y_vel(0x04);
    engine.state.set_obj_x_sub(0x00);
    engine.state.set_obj_x_tile(0x20);
    engine.state.set_obj_y_pixel(0x50);

    game::try_move_large_actor_with_terrain(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.state.obj_y_vel(), 0x04);
    assert_eq!(engine.state.indirect_ptr_lo(), 0x02);
    assert_eq!(engine.state.indirect_ptr_hi(), 0x20);
    assert_eq!(engine.state.scratch2(), 0x54);
}

#[test]
fn compose_large_actor_body_slots_syncs_linked_sprite_state() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_prg_bank_8000(0xAA);
    engine.state.set_prg_bank_a000(0xBB);
    engine.state.set_obj_state(0x01);
    engine.state.set_obj_tile(0x41);
    engine.state.set_obj_attr(0x02);
    engine.state.set_obj_health(0x09);
    engine.state.set_obj_x_sub(0x03);
    engine.state.set_obj_x_tile(0x20);
    engine.state.set_obj_y_pixel(0x50);
    engine.state.set_obj_y_extra(0x77);
    engine.state.set_object_state(0x10, 0x80);
    engine.state.set_object_health(0x10, 0x08);
    engine.state.set_object_health(0x20, 0x06);
    engine.state.set_object_health(0x30, 0x07);

    game::compose_large_actor_body_slots(&mut engine, &mut r);

    assert_eq!(engine.state.object_x_sub(0x10), 0x03);
    assert_eq!(engine.state.object_x_sub(0x20), 0x03);
    assert_eq!(engine.state.object_x_sub(0x30), 0x03);
    assert_eq!(engine.state.object_x_tile(0x20), 0x20);
    assert_eq!(engine.state.object_x_tile(0x10), 0x21);
    assert_eq!(engine.state.object_x_tile(0x30), 0x21);
    assert_eq!(engine.state.object_y_pixel(0x10), 0x50);
    assert_eq!(engine.state.object_y_pixel(0x20), 0x60);
    assert_eq!(engine.state.object_y_pixel(0x30), 0x60);
    assert_eq!(engine.state.object_y_extra(0x10), 0x77);
    assert_eq!(engine.state.object_y_extra(0x20), 0x77);
    assert_eq!(engine.state.object_y_extra(0x30), 0x77);

    assert_eq!(engine.state.object_state(0x00), 0x80);
    assert_eq!(engine.state.object_state(0x10), 0x80);
    assert_eq!(engine.state.object_state(0x20), 0x80);
    assert_eq!(engine.state.object_state(0x30), 0x80);
    assert_eq!(engine.state.object_health(0x00), 0x06);

    assert_eq!(engine.state.object_tile(0x10), 0x45);
    assert_eq!(engine.state.object_tile(0x20), 0x61);
    assert_eq!(engine.state.object_tile(0x30), 0x65);
    assert_eq!(engine.state.object_attr(0x10), 0x02);
    assert_eq!(engine.state.object_attr(0x20), 0x02);
    assert_eq!(engine.state.object_attr(0x30), 0x02);

    assert_eq!(engine.state.prg_bank_8000(), 0xAA);
    assert_eq!(engine.state.prg_bank_a000(), 0xBB);
}
