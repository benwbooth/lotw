use lotw::{Engine, RoutineContext, game, native};

#[test]
fn resolve_room_tile_pointer_populates_tile_and_room_offsets() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.data_ptr_lo = 0x02;
    engine.state.data_ptr_hi = 0x30;
    engine.state.room_metadef_lo = 0x40;
    engine.state.room_metadef_hi = 0x12;

    game::resolve_room_tile_pointer(&mut engine, &mut r);

    assert_eq!(engine.state.data_ptr_lo, 0x1B);
    assert_eq!(engine.state.data_ptr_hi, 0x05);
    assert_eq!(engine.state.tile_fetch_counter, 0x5B);
    assert_eq!(engine.state.aux_ptr_hi, 0x12);
}

#[test]
fn bounds_helpers_set_carry_when_projected_position_is_outside() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scratch2 = 0xBF;
    engine.state.indirect_ptr_hi = 0x3E;
    engine.state.indirect_ptr_lo = 0x01;
    game::check_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.state.indirect_ptr_hi = 0x3F;
    game::check_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.state.scratch2 = 0xB0;
    engine.state.indirect_ptr_hi = 0x3E;
    engine.state.indirect_ptr_lo = 0x01;
    game::check_actor_position_out_of_bounds(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn check_player_overlap_sets_collision_flag() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x00;
    engine.state.player_x_tile = 0x10;
    engine.state.player_y = 0x50;
    engine.state.indirect_ptr_lo = 0x00;
    engine.state.indirect_ptr_hi = 0x10;
    engine.state.scratch2 = 0x50;

    game::check_player_overlap(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.overlap_flag, 0x01);

    engine.state.scratch2 = 0x70;
    game::check_player_overlap(&mut engine, &mut r);

    assert_eq!(r.carry, 0);
    assert_eq!(engine.state.overlap_flag, 0x00);
}

#[test]
fn damageable_actor_overlap_skips_low_non_actor_states() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.indirect_ptr_lo = 0x00;
    engine.state.indirect_ptr_hi = 0x10;
    engine.state.scratch2 = 0x50;
    engine.state.set_object_state(0x90, 0x02);
    engine.state.set_object_x_sub(0x90, 0x00);
    engine.state.set_object_x_tile(0x90, 0x10);
    engine.state.set_object_y_pixel(0x90, 0x50);

    native::find_damageable_actor_overlap(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    native::find_player_object_overlap(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.scratch0, 0x09);
    assert_eq!(engine.state.scratch1, 0x90);
}

#[test]
fn damageable_actor_overlap_reports_slot_and_offset() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.indirect_ptr_lo = 0x00;
    engine.state.indirect_ptr_hi = 0x10;
    engine.state.scratch2 = 0x50;
    engine.state.set_object_state(0x90, 0x01);
    engine.state.set_object_x_sub(0x90, 0x00);
    engine.state.set_object_x_tile(0x90, 0x10);
    engine.state.set_object_y_pixel(0x90, 0x50);

    native::find_damageable_actor_overlap(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.scratch0, 0x09);
    assert_eq!(engine.state.scratch1, 0x90);
}

#[test]
fn build_direction_velocity_clears_velocity_for_zero_speed() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        value: 0x01,
        offset: 0x00,
        ..RoutineContext::default()
    };

    engine.state.obj_x_vel_lo = 0xAA;
    engine.state.obj_x_vel_hi = 0xBB;
    engine.state.obj_y_vel = 0xCC;

    game::build_direction_velocity(&mut engine, &mut r);

    assert_eq!(engine.state.obj_x_vel_lo, 0x00);
    assert_eq!(engine.state.obj_x_vel_hi, 0x00);
    assert_eq!(engine.state.obj_y_vel, 0x00);
}

#[test]
fn solid_tile_probes_set_carry_for_solid_range() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        offset: 0x01,
        ..RoutineContext::default()
    };

    engine.state.data_ptr_lo = 0x00;
    engine.state.data_ptr_hi = 0x02;
    engine.state.set_oam_tile(0x00, 0x30);

    game::probe_object_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.state.set_oam_tile(0x00, 0x2F);
    game::probe_projected_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 0);
}

#[test]
fn player_solid_tile_probe_handles_empty_alignment_and_solid_tiles() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        offset: 0x01,
        ..RoutineContext::default()
    };

    engine.state.data_ptr_lo = 0x00;
    engine.state.data_ptr_hi = 0x02;

    engine.state.set_oam_tile(0x00, 0x00);
    engine.state.player_x_fine = 0x00;
    game::probe_player_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.state.player_x_fine = 0x01;
    game::probe_player_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.state.set_oam_tile(0x00, 0x02);
    game::probe_player_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    engine.state.set_oam_tile(0x00, 0x30);
    game::probe_player_solid_tile(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn hazard_tile_contact_consumes_health_once_and_sets_recoil() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        offset: 0x01,
        ..RoutineContext::default()
    };

    engine.state.data_ptr_lo = 0x00;
    engine.state.data_ptr_hi = 0x02;
    engine.state.set_oam_tile(0x00, 0x30);
    engine.state.player_health = 0x02;

    game::apply_hazard_tile_contact(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.player_health, 0x01);
    assert_eq!(engine.state.jump_timer, 0x0A);
    assert_eq!(engine.state.sprite_blink_timer, 0x01);
    assert_eq!(engine.state.prompt_state, 0x0A);

    engine.state.data_ptr_lo = 0x00;
    engine.state.data_ptr_hi = 0x02;
    r.offset = 0x01;
    game::apply_hazard_tile_contact(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.player_health, 0x01);
}

#[test]
fn top_boundary_exit_probe_reports_empty_top_row_tile() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.indirect_ptr_lo = 0x00;
    engine.state.indirect_ptr_hi = 0x00;
    engine.state.set_room_buffer(0x00, 0x00);
    game::check_top_boundary_exit_clear(&mut engine, &mut r);
    assert_eq!(r.carry, 1);

    r.carry = 0;
    engine.state.indirect_ptr_lo = 0x00;
    engine.state.indirect_ptr_hi = 0x00;
    engine.state.set_room_buffer(0x00, 0x01);
    game::check_top_boundary_exit_clear(&mut engine, &mut r);
    assert_eq!(r.carry, 0);
}

#[test]
fn player_terrain_contact_resets_contact_state_while_locked() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.airborne_flag = 0x01;
    engine.state.pose_state = 0x02;
    engine.state.fall_frames = 0x20;

    native::update_player_terrain_contact(&mut engine, &mut r);

    assert_eq!(engine.state.pose_state, 0x00);
    assert_eq!(engine.state.fall_frames, 0x00);
}

#[test]
fn project_player_position_applies_subtile_carry_and_vertical_delta() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.player_x_fine = 0x0E;
    engine.state.player_x_tile = 0x10;
    engine.state.player_y = 0x50;
    engine.state.horizontal_subtile_delta = 0x05;
    engine.state.player_x_velocity = 0x00;
    engine.state.vertical_delta = 0xFE;

    game::project_player_position(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_lo, 0x03);
    assert_eq!(engine.state.indirect_ptr_hi, 0x11);
    assert_eq!(engine.state.scratch2, 0x4E);
}

#[test]
fn player_walk_animation_toggles_pose_every_eight_movement_ticks() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.buttons = 0x41;
    engine.state.player_pose = 0x01;
    engine.state.player_facing = 0x00;
    engine.state.anim_step_counter = 0x07;

    game::tick_player_walk_animation(&mut engine, &mut r);

    assert_eq!(engine.state.anim_step_counter, 0x08);
    assert_eq!(engine.state.player_pose, 0x15);
    assert_eq!(engine.state.player_facing, 0x00);
}

#[test]
fn magic_contact_actor_marks_hit_slot_when_timer_and_magic_are_active() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_chr_bank(3, 0x20);
    engine.state.magic_contact_flag = 0x01;
    engine.state.player_magic = 0x01;
    engine.state.scratch1 = 0x30;

    game::try_trigger_magic_contact_actor(&mut engine, &mut r);
    assert_eq!(engine.state.object_state(0x30), 0x80);

    engine.state.set_object_state(0x30, 0x00);
    engine.state.player_magic = 0x00;
    game::try_trigger_magic_contact_actor(&mut engine, &mut r);
    assert_eq!(engine.state.object_state(0x30), 0x00);
}

#[test]
fn seed_object_position_from_tile_offset_handles_lower_tile_sample() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.scratch3 = 0x0D;
    engine.state.scratch2 = 0x34;
    engine.state.indirect_ptr_hi = 0x12;

    game::seed_object_position_from_tile_offset(&mut engine, &mut r);

    assert_eq!(engine.state.indirect_ptr_hi, 0x13);
    assert_eq!(engine.state.obj_x_tile, 0x13);
    assert_eq!(engine.state.obj_y_pixel, 0x40);
    assert_eq!(engine.state.obj_x_sub, 0x00);
    assert_eq!(engine.state.obj_y_extra, 0x00);
    assert_eq!(r.offset, 0x01);
    assert_eq!(r.value, 0x00);
}

#[test]
fn read_room_tile_action_value_resolves_replacement_tile() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.tile_fetch_counter = 0x00;
    engine.state.aux_ptr_hi = 0x02;
    engine.state.scratch3 = 0x05;
    engine.state.set_oam_tile(0x04, 0xBE);
    engine.state.room_tile_action = 0x2A;

    game::read_room_tile_action_value(&mut engine, &mut r);

    assert_eq!(r.index, 0x3E);
    assert_eq!(r.offset, 0x05);
    assert_eq!(r.value, 0x2A);

    engine.state.set_oam_tile(0x04, 0x24);
    game::read_room_tile_action_value(&mut engine, &mut r);
    assert_eq!(r.index, 0x24);
    assert_eq!(r.value, 0x24);
}

#[test]
fn room_tile_action_default_path_reports_solid_range() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        offset: 0x01,
        ..RoutineContext::default()
    };

    engine.state.data_ptr_lo = 0x00;
    engine.state.data_ptr_hi = 0x02;
    engine.state.set_oam_tile(0x00, 0x2F);
    native::dispatch_room_tile_action(&mut engine, &mut r);
    assert_eq!(r.carry, 0);

    engine.state.set_oam_tile(0x00, 0x30);
    native::dispatch_room_tile_action(&mut engine, &mut r);
    assert_eq!(r.carry, 1);
}

#[test]
fn try_reflect_object_velocity_reports_no_reflection_when_stationary() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.obj_x_vel_lo = 0x00;
    engine.state.obj_y_vel = 0x00;

    game::try_reflect_object_velocity(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(engine.state.obj_x_vel_hi, 0x00);
}
