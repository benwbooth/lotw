// Rust game routine module. The functions here are the checked-in native game logic.
//
// Numbered `routine_####` names are retained as stable port labels while the
// original game systems are being identified. Keep semantic discoveries in
// `docs/routine_catalog.md` first, then rename or alias routines only after the
// dataflow is understood well enough to make the name useful.
use crate::engine::RoutineFn;
use crate::frame;
use crate::native::*;
use crate::{Engine, RoutineContext, cbool, not, u8v, u16v};

pub use add_coins::add_coins;
pub use add_health_points::add_health_points;
pub use add_key::add_key;
pub use add_keys::add_keys;
pub use add_magic_points::add_magic_points;
pub use advance_envelope_phase::advance_envelope_phase;
pub use aim_actor_from_player_overlap::aim_actor_from_player_overlap;
pub use aim_actor_toward_player::aim_actor_toward_player;
pub use animate_actor_cycle_tiles::animate_actor_cycle_tiles;
pub use animate_actor_directional_walk::animate_actor_directional_walk;
pub use animate_actor_flip_toggle::animate_actor_flip_toggle;
pub use animate_actor_walk_toggle::animate_actor_walk_toggle;
pub use animate_large_actor_body_tiles::animate_large_actor_body_tiles;
pub use apply_actor_player_contact_damage::apply_actor_player_contact_damage;
pub use apply_event_collectible_reward::apply_event_collectible_reward;
pub use apply_hazard_tile_contact::apply_hazard_tile_contact;
pub use apply_projectile_direction_bits::apply_projectile_direction_bits;
pub use audio_cmd_set_channel_flags::audio_cmd_set_channel_flags;
pub use audio_cmd_set_duty_instrument::audio_cmd_set_duty_instrument;
pub use audio_cmd_set_pitch_offset::audio_cmd_set_pitch_offset;
pub use audio_cmd_set_sweep_value::audio_cmd_set_sweep_value;
pub use audio_cmd_set_volume_scale::audio_cmd_set_volume_scale;
pub use build_direction_velocity::build_direction_velocity;
pub use build_health_meter_sprites::build_health_meter_sprites;
pub use build_input_movement_delta::build_input_movement_delta;
pub use build_object_health_meter_alt_tiles::build_object_health_meter_alt_tiles;
pub use build_object_health_meter_standard_tiles::build_object_health_meter_standard_tiles;
pub use build_player_health_meter_sprites::build_player_health_meter_sprites;
pub use build_status_resource_meter_tiles::build_status_resource_meter_tiles;
pub use check_actor_direction_contact::check_actor_direction_contact;
pub use check_actor_position_out_of_bounds::check_actor_position_out_of_bounds;
pub use check_final_exit_trigger::check_final_exit_trigger;
pub use check_player_overlap::check_player_overlap;
pub use check_player_overlap_wide::check_player_overlap_wide;
pub use check_player_x_overlap::check_player_x_overlap;
pub use check_player_y_overlap::check_player_y_overlap;
pub use check_position_out_of_bounds::check_position_out_of_bounds;
pub use check_projected_terrain_collision::check_projected_terrain_collision;
pub use check_projected_wide_terrain_collision::check_projected_wide_terrain_collision;
pub use check_top_boundary_exit_clear::check_top_boundary_exit_clear;
pub use choose_random_actor_direction::choose_random_actor_direction;
pub use choose_random_cardinal_actor_direction::choose_random_cardinal_actor_direction;
pub use clear_gameplay_object_sprites::clear_gameplay_object_sprites;
pub use clear_inventory_item_list_buffer::clear_inventory_item_list_buffer;
pub use clear_pending_vram_job::clear_pending_vram_job;
pub use close_inventory_item_menu::close_inventory_item_menu;
pub use collect_key_bundle_reward::collect_key_bundle_reward;
pub use collect_large_coin_reward::collect_large_coin_reward;
pub use collect_room_pickup_object::collect_room_pickup_object;
pub use collect_single_key_reward::collect_single_key_reward;
pub use collect_small_coin_reward::collect_small_coin_reward;
pub use collect_small_health_reward::collect_small_health_reward;
pub use collect_small_magic_reward::collect_small_magic_reward;
pub use commit_actor_projected_position::commit_actor_projected_position;
pub use compose_large_actor_body_slots::compose_large_actor_body_slots;
pub use consume_health_point::consume_health_point;
pub use consume_key::consume_key;
pub use consume_magic_point::consume_magic_point;
pub use defeat_active_room_actors::defeat_active_room_actors;
pub use dispatch_actor_behavior::dispatch_actor_behavior;
pub use dispatch_audio_stream_command::dispatch_audio_stream_command;
pub use dispatch_overhead_tile_action::dispatch_overhead_tile_action;
pub use dispatch_projected_tile_actions::dispatch_projected_tile_actions;
pub use enter_fragment_pickup_room::enter_fragment_pickup_room;
pub use enter_pending_special_exit_room::enter_pending_special_exit_room;
pub use enter_room_link_destination::enter_room_link_destination;
pub use farcall_bank_0C0D_seed::farcall_bank_0C0D_seed;
pub use farcall_bank_09_r7::farcall_bank_09_r7;
pub use farcall_return_home::farcall_return_home;
pub use frame_counters::frame_counters;
pub use game_update::game_update;
pub use grant_long_invulnerability::grant_long_invulnerability;
pub use grant_long_speed_boost::grant_long_speed_boost;
pub use grant_short_invulnerability::grant_short_invulnerability;
pub use grant_short_speed_boost::grant_short_speed_boost;
pub use handle_player_room_transition::handle_player_room_transition;
pub use inc16_95::inc16_95;
pub use initialize_large_actor_slot::initialize_large_actor_slot;
pub use load_effective_jump_duration::load_effective_jump_duration;
pub use load_effective_projectile_damage::load_effective_projectile_damage;
pub use load_effective_projectile_lifetime::load_effective_projectile_lifetime;
pub use load_note_period::load_note_period;
pub use load_object_slot_scratch::load_object_slot_scratch;
pub use main_init::main_init;
pub use maybe_spawn_pursuer_actor::maybe_spawn_pursuer_actor;
pub use metasprite_build::metasprite_build;
pub use move_inventory_cursor_down::move_inventory_cursor_down;
pub use move_inventory_cursor_left::move_inventory_cursor_left;
pub use move_inventory_cursor_right::move_inventory_cursor_right;
pub use move_inventory_cursor_up::move_inventory_cursor_up;
pub use next_envelope_volume::next_envelope_volume;
pub use ppu_commit_banks::ppu_commit_banks;
pub use probe_actor_overhead_step::probe_actor_overhead_step;
pub use probe_object_solid_tile::probe_object_solid_tile;
pub use probe_player_solid_tile::probe_player_solid_tile;
pub use probe_projected_solid_tile::probe_projected_solid_tile;
pub use project_actor_position::project_actor_position;
pub use project_player_position::project_player_position;
pub use project_player_projectile_position::project_player_projectile_position;
pub use ram_state_init::ram_state_init;
pub use read_controllers::read_controllers;
pub use read_debounced_buttons::read_debounced_buttons;
pub use read_room_tile_action_value::read_room_tile_action_value;
pub use redraw_room_tile_column::redraw_room_tile_column;
pub use reset::reset;
pub use reset_room_object_slots::reset_room_object_slots;
pub use resolve_room_tile_pointer::resolve_room_tile_pointer;
pub use restore_inventory_state_snapshot::restore_inventory_state_snapshot;
pub use reverse_actor_horizontal_direction::reverse_actor_horizontal_direction;
pub use rewind_or_stop_audio_stream::rewind_or_stop_audio_stream;
pub use rng_update::rng_update;
pub use routine_0003::routine_0003;
pub use routine_0005::routine_0005;
pub use routine_0006::routine_0006;
pub use routine_0007::routine_0007;
pub use routine_0008::routine_0008;
pub use routine_0009::routine_0009;
pub use routine_0010::routine_0010;
pub use routine_0011::routine_0011;
pub use routine_0012::routine_0012;
pub use routine_0013::routine_0013;
pub use routine_0014::routine_0014;
pub use routine_0015::routine_0015;
pub use routine_0016::routine_0016;
pub use routine_0017::routine_0017;
pub use routine_0018::routine_0018;
pub use routine_0019::routine_0019;
pub use routine_0021::routine_0021;
pub use routine_0022::routine_0022;
pub use routine_0023::routine_0023;
pub use routine_0024::routine_0024;
pub use routine_0025::routine_0025;
pub use routine_0026::routine_0026;
pub use routine_0027::routine_0027;
pub use routine_0028::routine_0028;
pub use routine_0030::routine_0030;
pub use routine_0031::routine_0031;
pub use routine_0032::routine_0032;
pub use routine_0035::routine_0035;
pub use routine_0036::routine_0036;
pub use routine_0037::routine_0037;
pub use routine_0038::routine_0038;
pub use routine_0040::routine_0040;
pub use routine_0041::routine_0041;
pub use routine_0042::routine_0042;
pub use routine_0043::routine_0043;
pub use routine_0044::routine_0044;
pub use routine_0046::routine_0046;
pub use routine_0047::routine_0047;
pub use routine_0048::routine_0048;
pub use routine_0051::routine_0051;
pub use routine_0052::routine_0052;
pub use routine_0053::routine_0053;
pub use routine_0054::routine_0054;
pub use routine_0056::routine_0056;
pub use routine_0057::routine_0057;
pub use routine_0059::routine_0059;
pub use routine_0060::routine_0060;
pub use routine_0061::routine_0061;
pub use routine_0062::routine_0062;
pub use routine_0063::routine_0063;
pub use routine_0064::routine_0064;
pub use routine_0065::routine_0065;
pub use routine_0066::routine_0066;
pub use routine_0073::routine_0073;
pub use routine_0075::routine_0075;
pub use routine_0076::routine_0076;
pub use routine_0077::routine_0077;
pub use routine_0078::routine_0078;
pub use routine_0079::routine_0079;
pub use routine_0080::routine_0080;
pub use routine_0081::routine_0081;
pub use routine_0082::routine_0082;
pub use routine_0083::routine_0083;
pub use routine_0084::routine_0084;
pub use routine_0085::routine_0085;
pub use routine_0086::routine_0086;
pub use routine_0087::routine_0087;
pub use routine_0088::routine_0088;
pub use routine_0089::routine_0089;
pub use routine_0117::routine_0117;
pub use routine_0118::routine_0118;
pub use routine_0119::routine_0119;
pub use routine_0120::routine_0120;
pub use routine_0121::routine_0121;
pub use routine_0122::routine_0122;
pub use routine_0123::routine_0123;
pub use routine_0192::routine_0192;
pub use routine_0195::routine_0195;
pub use routine_0196::routine_0196;
pub use routine_0197::routine_0197;
pub use routine_0198::routine_0198;
pub use routine_0199::routine_0199;
pub use routine_0200::routine_0200;
pub use routine_0201::routine_0201;
pub use run_warp_transition_effect::run_warp_transition_effect;
pub use scale_envelope_volume::scale_envelope_volume;
pub use scale_room_tile_column::scale_room_tile_column;
pub use scene_assemble::scene_assemble;
pub use seed_object_position_from_tile_offset::seed_object_position_from_tile_offset;
pub use select_inventory_grid_entry::select_inventory_grid_entry;
pub use set_inventory_list_buffer_index::set_inventory_list_buffer_index;
pub use sfx_overlay_voice::sfx_overlay_voice;
pub use snapshot_inventory_state::snapshot_inventory_state;
pub use song_init::song_init;
pub use sound_restore_game_banks::sound_restore_game_banks;
pub use sound_set_default_banks::sound_set_default_banks;
pub use sound_set_song_banks::sound_set_song_banks;
pub use sound_tick::sound_tick;
pub use spawn_player_projectile::spawn_player_projectile;
pub use spend_coins::spend_coins;
pub use split_meter_value::split_meter_value;
pub use start_note_envelope::start_note_envelope;
pub use start_rest_envelope::start_rest_envelope;
pub use statusbar_split::statusbar_split;
pub use stop_actor_motion::stop_actor_motion;
pub use store_object_slot_scratch::store_object_slot_scratch;
pub use subtract_health_points::subtract_health_points;
pub use sync_coin_hud::sync_coin_hud;
pub use sync_health_hud::sync_health_hud;
pub use sync_key_hud::sync_key_hud;
pub use sync_magic_hud::sync_magic_hud;
pub use text_attr_build::text_attr_build;
pub use tick_actor_materialize_delay::tick_actor_materialize_delay;
pub use tick_chasing_jump_actor::tick_chasing_jump_actor;
pub use tick_contact_recoil_actor::tick_contact_recoil_actor;
pub use tick_contact_trigger_actor::tick_contact_trigger_actor;
pub use tick_inactive_actor_slot::tick_inactive_actor_slot;
pub use tick_large_chasing_actor::tick_large_chasing_actor;
pub use tick_ledge_walking_actor::tick_ledge_walking_actor;
pub use tick_noise_channel::tick_noise_channel;
pub use tick_overhead_probe_actor::tick_overhead_probe_actor;
pub use tick_player_jump_action::tick_player_jump_action;
pub use tick_player_walk_animation::tick_player_walk_animation;
pub use tick_pulse1_channel::tick_pulse1_channel;
pub use tick_pulse2_channel::tick_pulse2_channel;
pub use tick_random_floating_actor::tick_random_floating_actor;
pub use tick_reflecting_chase_actor::tick_reflecting_chase_actor;
pub use tick_selected_item_effect::tick_selected_item_effect;
pub use tick_standard_actor::tick_standard_actor;
pub use tick_timed_chase_actor::tick_timed_chase_actor;
pub use tick_triangle_channel::tick_triangle_channel;
pub use tick_wandering_jump_actor::tick_wandering_jump_actor;
pub use trigger_damage_pickup::trigger_damage_pickup;
pub use try_actor_gravity_motion::try_actor_gravity_motion;
pub use try_actor_jump_arc_motion::try_actor_jump_arc_motion;
pub use try_large_actor_gravity_motion::try_large_actor_gravity_motion;
pub use try_large_actor_jump_arc_motion::try_large_actor_jump_arc_motion;
pub use try_move_actor_with_terrain::try_move_actor_with_terrain;
pub use try_move_actor_without_terrain::try_move_actor_without_terrain;
pub use try_move_large_actor_with_terrain::try_move_large_actor_with_terrain;
pub use try_move_player_with_collision::try_move_player_with_collision;
pub use try_nudge_player_to_tile_boundary::try_nudge_player_to_tile_boundary;
pub use try_reflect_object_velocity::try_reflect_object_velocity;
pub use try_trigger_magic_contact_actor::try_trigger_magic_contact_actor;
pub use update_actor_animation::update_actor_animation;
pub use update_inventory_grid_cursor_sprites::update_inventory_grid_cursor_sprites;
pub use update_inventory_list_cursor_sprites::update_inventory_list_cursor_sprites;
pub use update_large_actor_facing_from_velocity::update_large_actor_facing_from_velocity;
pub use update_object_terrain_probe::update_object_terrain_probe;
pub use update_player_pose_from_motion::update_player_pose_from_motion;
pub use update_player_projectile_slot::update_player_projectile_slot;
pub use update_player_projectiles::update_player_projectiles;
pub use update_room_actors::update_room_actors;
pub use update_tile_projectile::update_tile_projectile;
pub use update_tile_projectile_motion::update_tile_projectile_motion;
pub use update_wide_object_terrain_probe::update_wide_object_terrain_probe;
pub use upload_inventory_item_list::upload_inventory_item_list;
pub use upload_resource_hud::upload_resource_hud;
pub use vblank_commit::vblank_commit;
pub use vblank_commit_tail::vblank_commit_tail;
pub use vram_blit_stack::vram_blit_stack;
pub use vram_copy_indirect::vram_copy_indirect;
pub use vram_fill_run::vram_fill_run;
pub use vram_poke2::vram_poke2;
pub use vram_upload_hud::vram_upload_hud;
pub use vram_upload_palette::vram_upload_palette;

fn with_large_actor_asset_banks<F>(engine: &mut Engine, r: &mut RoutineContext, action: F)
where
    F: FnOnce(&mut Engine, &mut RoutineContext),
{
    let saved_bank6: i32 = engine.mem(0x30);
    let saved_bank7: i32 = engine.mem(0x31);
    engine.set_mem(0x32, saved_bank6);
    engine.set_mem(0x33, saved_bank7);
    engine.set_mem(0x30, 0x0C);
    engine.set_mem(0x31, 0x0D);
    engine.set_mem(0x25, 0x07);
    engine.prg_map_shadow();
    action(engine, r);
    engine.set_mem(0x31, saved_bank7);
    engine.set_mem(0x30, saved_bank6);
    engine.set_mem(0x25, 0x06);
    engine.prg_map_shadow();
}

mod farcall_bank_09_r7 {
    use super::*;
    pub fn farcall_bank_09_r7(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_r7: i32 = engine.mem(0x31);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, 0x09);
        engine.device_write(0x8001, 0x09);
        engine.set_mem(0x0D, 0x00);
        r.value = 0x00;
        resolve_room_tile_pointer(engine, r);
        metasprite_build(engine, r);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, saved_r7);
        engine.device_write(0x8001, saved_r7);
        r.value = saved_r7;
    }
}

mod farcall_bank_0C0D_seed {
    use super::*;
    pub fn farcall_bank_0C0D_seed(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x32, engine.mem(0x30));
        engine.set_mem(0x33, engine.mem(0x31));
        engine.set_mem(0x25, 0x06);
        engine.device_write(0x8000, 0x06);
        engine.set_mem(0x30, 0x0C);
        engine.device_write(0x8001, 0x0C);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, 0x0D);
        engine.device_write(0x8001, 0x0D);
        r.value = 0x0D;
        r.offset = 0x07;
    }
}

mod farcall_return_home {
    use super::*;
    pub fn farcall_return_home(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x31, engine.mem(0x33));
        engine.set_mem(0x30, engine.mem(0x32));
    }
}

mod frame_counters {
    use super::*;
    pub fn frame_counters(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.dec_mem(0x84) != 0) {
            return;
        }
        {
            let mut x: i32 = 7;
            while cbool(x >= 0) {
                if cbool(engine.mem((0x85 + x) & 0xFF) != 0) {
                    engine.dec_mem((0x85 + x) & 0xFF);
                }
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x84, 0x3C);
        r.index = 0xFF;
    }
}

mod game_update {
    use super::*;
    pub fn game_update(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut y: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xE3, 0xFF);
                    if cbool(engine.mem(0xEB) != 0) {
                        enter_pending_special_exit_room(engine, r);
                        return;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    check_final_exit_trigger(engine, r);
                    if cbool(engine.mem(0x20) & 0x10) {
                        routine_0174(engine, r);
                        return;
                    }
                    tick_selected_item_effect(engine, r);
                    if cbool(engine.mem(0x46) != 0) {
                        engine.dec_mem(0x46);
                        engine.set_mem(0x20, 0x00);
                    }
                    {
                        let mut clear_hi: i32 = 1;
                        if cbool(engine.mem(0x40) == 0x04) {
                            if cbool((engine.mem(0x84) & 0x07) == 0) {
                                clear_hi = 1;
                            } else {
                                clear_hi = (if cbool(engine.mem(0x20) & 0x40) { 0 } else { 1 });
                            }
                        } else {
                            clear_hi = (if cbool(engine.mem(0x20) & 0x40) { 0 } else { 1 });
                        }
                        if cbool(clear_hi) {
                            engine.and_mem(0xFD, 0x0F);
                        }
                    }
                    a = engine.mem(0x20) & 0x0F;
                    if cbool(a != 0) {
                        engine.set_mem(0x08, a);
                        engine.set_mem(0xFD, u8v((engine.mem(0xFD) & 0xF0) | a));
                    }
                    if cbool(engine.mem(0x20) & 0x20) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x20) & 0x08) {
                        dispatch_overhead_tile_action(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                    }
                    y = 0x01;
                    while cbool(engine.mem(u16v(0x0087 + y)) != 0) {
                        {
                            let __old = y;
                            y += 1;
                            __old
                        };
                        if cbool(y >= 0x05) {
                            y = 0x06;
                            break;
                        }
                    }
                    r.offset = y;
                    build_input_movement_delta(engine, r);
                    if cbool(engine.mem(0x4E) != 0) {
                        engine.set_mem(0x4B, u8v((engine.mem(0x4E) >> 2) + 1));
                        try_move_player_with_collision(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0x49, 0x00);
                        engine.set_mem(0x4A, 0x00);
                        try_move_player_with_collision(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4F) != 0) {
                        tick_player_jump_action(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                        engine.set_mem(0x4F, 0x00);
                    } else if cbool(engine.mem(0x20) & 0x80) {
                        tick_player_jump_action(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                        engine.set_mem(0x4F, 0x00);
                    } else {
                        engine.set_mem(0x22, 0x00);
                        engine.set_mem(0x4F, 0x00);
                    }
                    try_move_player_with_collision(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    try_nudge_player_to_tile_boundary(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x43, engine.mem(0x0E));
                    engine.set_mem(0x44, engine.mem(0x0F));
                    a = engine.mem(0x0A);
                    if cbool(a >= 0xEF) {
                        a = 0x00;
                    }
                    engine.set_mem(0x45, a);
                    update_player_terrain_contact(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0x4F, 0x00);
                    engine.set_mem(0x4E, 0x00);
                    update_player_terrain_contact(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    engine.set_mem(0x8F, 0x10);
                    loop {
                        read_debounced_buttons(engine, r);
                        if cbool(r.value & 0xF0) {
                            break;
                        }
                        if cbool((engine.mem(0x20) & 0x03) == 0) {
                            continue;
                        }
                        engine.shl_mem(0x20, 1);
                        engine.shl_mem(0x20, 1);
                        r.offset = 0x01;
                        build_input_movement_delta(engine, r);
                        {
                            let mut t: i32 = u8v(engine.mem(0x4B) + engine.mem(0x55));
                            let mut ni: i32 = 0;
                            if cbool(t & 0x80) {
                                ni = 0x03;
                            } else if cbool(t < 0x04) {
                                ni = t;
                            } else {
                                ni = 0x00;
                            }
                            engine.set_mem(0x55, ni);
                        }
                        engine.set_mem(0x8F, 0x0C);
                    }
                    engine.set_mem(0x8F, 0x10);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    update_player_pose_from_motion(engine, r);
                    tick_player_walk_animation(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod inc16_95 {
    use super::*;
    pub fn inc16_95(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        if cbool(engine.inc_mem((0x95 + x) & 0xFF) == 0) {
            engine.inc_mem((0x96 + x) & 0xFF);
        }
        r.index = x;
    }
}

mod main_init {
    use super::*;
    fn farcall_0C0D(
        engine: &mut Engine,
        r: &mut RoutineContext,
        mut lo: i32,
        mut hi: i32,
        target: RoutineFn,
    ) {
        let mut old6: i32 = engine.mem(0x30);
        let mut old7: i32 = engine.mem(0x31);
        engine.set_mem(0x32, old6);
        engine.set_mem(0x33, old7);
        engine.set_mem(0x0E, lo);
        engine.set_mem(0x0F, hi);
        engine.set_mem(0x30, 0x0C);
        engine.set_mem(0x31, 0x0D);
        engine.set_mem(0x25, 0x07);
        engine.prg_map_shadow();
        target(engine, r);
        engine.set_mem(0x31, old7);
        engine.set_mem(0x30, old6);
        engine.set_mem(0x25, 0x06);
        engine.prg_map_shadow();
    }

    pub fn main_init(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2000, 0x00);
        engine.device_write(0x2001, 0x00);
        engine.device_write(0x4010, 0x00);
        engine.set_mem(0x0027, 0x1F);
        engine.device_write(0x4015, 0x1F);
        engine.device_write(0x4017, 0xC0);
        engine.device_write(0xA000, 0x00);
        farcall_bank_0C0D_seed(engine, r);
        ram_state_init(engine, r);
        farcall_0C0D(engine, r, 0x64, 0xAE, routine_0033);
        engine.set_mem(0x46, 0x00);
        engine.set_mem(0x7B, 0x00);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7C, 0x30);
        engine.set_mem(0x44, 0x3C);
        engine.set_mem(0x45, 0xA0);
        scene_assemble(engine, r);
        engine.set_mem(0x20, 0x08);
        game_update(engine, r);
        main_loop_dispatch(engine, r);
    }
}

mod metasprite_build {
    use super::*;
    pub fn metasprite_build(engine: &mut Engine, r: &mut RoutineContext) {
        let mut p0C: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut p79: i32 = u16v(engine.mem(0x79) | (engine.mem(0x7A) << 8));
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut dst_lo: i32 = 0;
        let mut mask2: i32 = 0;
        engine.set_mem(0x0B, 0x00);
        {
            x = 0x16;
            while cbool(x >= 0) {
                let mut e: i32 = engine.mem(u16v(p0C + engine.mem(0x0B)));
                let mut ty: i32 = u16v(u8v(e << 2));
                engine.set_mem(u16v(0x0141 + x), engine.mem(u16v(p79 + ((ty + 0) & 0xFF))));
                engine.set_mem(u16v(0x0140 + x), engine.mem(u16v(p79 + ((ty + 1) & 0xFF))));
                engine.set_mem(u16v(0x0159 + x), engine.mem(u16v(p79 + ((ty + 2) & 0xFF))));
                engine.set_mem(u16v(0x0158 + x), engine.mem(u16v(p79 + ((ty + 3) & 0xFF))));
                engine.add_mem(0x0B, 1);
                x -= 2;
            }
        }
        engine.set_mem(0x19, u8v(engine.mem(0x17) + 0x03));
        dst_lo = engine.mem(0x16);
        engine.set_mem(0x0B, u8v((dst_lo >> 2) + 0xC0));
        mask2 = u8v(dst_lo & 0x02);
        engine.set_mem(0x18, (if cbool(mask2) { 0x33 } else { 0xCC }));
        y = 0x00;
        {
            x = 0x0A;
            while cbool(x >= 0) {
                let mut b0: i32 = 0;
                let mut b1: i32 = 0;
                let mut v: i32 = 0;
                engine.set_mem(u16v(0x0170 + x), engine.mem(0x0B));
                engine.set_mem(0x0B, u8v(engine.mem(0x0B) + 0x08));
                b0 = engine.mem(u16v(
                    p0C + ({
                        let __old = y;
                        y += 1;
                        __old
                    }),
                ));
                v = u8v((b0 & 0xC0) >> 4);
                engine.set_mem(u16v(0x0171 + x), v);
                b1 = engine.mem(u16v(
                    p0C + ({
                        let __old = y;
                        y += 1;
                        __old
                    }),
                ));
                v = u8v((b1 & 0xC0) | engine.mem(u16v(0x0171 + x)));
                engine.set_mem(u16v(0x0171 + x), v);
                if cbool(mask2 == 0) {
                    engine.set_mem(u16v(0x0171 + x), u8v(engine.mem(u16v(0x0171 + x)) >> 1));
                    engine.set_mem(u16v(0x0171 + x), u8v(engine.mem(u16v(0x0171 + x)) >> 1));
                }
                x -= 2;
            }
        }
        r.value = 0x03;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod ppu_commit_banks {
    use super::*;
    pub fn ppu_commit_banks(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 7;
            while cbool(x >= 0) {
                engine.device_write(0x8000, u8v(x));
                engine.device_write(0x8001, engine.mem(u16v(0x2A + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod ram_state_init {
    use super::*;
    pub fn ram_state_init(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut i: i32 = 0;
        x = 0;
        loop {
            engine.set_mem(0x0000 + x, engine.mem(0x9B9F + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
        {
            i = 0x3F;
            while cbool(i >= 0) {
                engine.set_mem(0x0100 + u8v(i), engine.mem(0x9C9E + u8v(i)));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        {
            i = 0x1F;
            while cbool(i >= 0) {
                engine.set_mem(0x0180 + u8v(i), 0x0F);
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        x = 0;
        loop {
            engine.set_mem(0x0300 + x, engine.mem(0x9D3E + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
        x = 0;
        loop {
            engine.set_mem(0x0400 + x, engine.mem(0x9DC9 + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
    }
}

mod read_controllers {
    use super::*;
    pub fn read_controllers(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        if let Some(__buttons) = engine.next_input() {
            engine.ppu.set_buttons(__buttons);
        }
        engine.device_write(0x4016, 0x01);
        engine.device_write(0x4016, 0x00);
        {
            x = 8;
            while cbool(x != 0) {
                a = u8v(engine.device_read(0x4016) | engine.device_read(0x4017));
                c = a & 1;
                a >>= 1;
                engine.set_mem(0x20, u8v((engine.mem(0x20) << 1) | c));
                c = a & 1;
                engine.set_mem(0x21, u8v((engine.mem(0x21) << 1) | c));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x20, engine.mem(0x20) | engine.mem(0x21));
        return;
        engine.set_mem(0x4016, 0x01);
        engine.set_mem(0x4016, 0x00);
        {
            x = 8;
            while cbool(x != 0) {
                a = engine.mem(0x4016) | engine.mem(0x4017);
                c = a & 1;
                a >>= 1;
                engine.set_mem(0x20, u8v((engine.mem(0x20) << 1) | c));
                c = a & 1;
                engine.set_mem(0x21, u8v((engine.mem(0x21) << 1) | c));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x20, engine.mem(0x20) | engine.mem(0x21));
    }
}

mod reset {
    use super::*;
    pub fn reset(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x00);
        engine.device_write(0xA001, 0x00);
        engine.device_write(0xE000, 0x00);
        main_init(engine, r);
    }
}

mod rng_update {
    use super::*;
    pub fn rng_update(engine: &mut Engine, r: &mut RoutineContext) {
        let mut count: i32 = u8v(r.value);
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: i32 = 0;
        engine.set_mem(0x38, count);
        if cbool(count == 0) {
            r.value = engine.mem(0x3B);
            return;
        }
        x = engine.mem(0x3B);
        y = engine.mem(0x3A);
        loop {
            let mut xy: i32 = 0;
            let mut s: i32 = 0;
            let mut carry: i32 = 0;
            engine.set_mem(0x39, y);
            xy = u16v((u16v((x << 8) | y) << 1) + 1);
            x = u8v(xy >> 8);
            y = u8v(xy);
            s = u16v(y + engine.mem(0x3A));
            y = u8v(s);
            carry = u8v(s >> 8);
            a = u8v(x + engine.mem(0x3B) + carry);
            a = u8v(a + engine.mem(0x39));
            a &= 0x7F;
            x = a;
            engine.set_mem(0x3B, a);
            engine.set_mem(0x3A, y);
            if !cbool(a >= count) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0003 {
    use super::*;
    pub fn routine_0003(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xFA) == 0) {
            engine.set_mem(0x16, 0x0E);
            engine.set_mem(0x17, 0x20);
            engine.set_mem(
                0x17,
                u8v((u8v((engine.mem(0x1D) ^ 0x01) << 2)) | engine.mem(0x17)),
            );
            engine.set_mem(
                0xF9,
                u8v((u8v((((engine.mem(0x1D) ^ 0x01) << 4) + 0x07))) | engine.mem(0x7C)),
            );
            engine.set_mem(0xFA, 0x09);
        }
        engine.set_mem(0x0C, engine.mem(0xF9));
        farcall_bank_09_r7(engine, r);
        engine.set_mem(0x16, u8v(engine.mem(0x16) + 1));
        engine.set_mem(0x16, u8v(engine.mem(0x16) + 1));
        engine.set_mem(0xF9, u8v(engine.mem(0xF9) + 1));
        engine.set_mem(0xFA, u8v(engine.mem(0xFA) - 1));
        if cbool(engine.mem(0xFA) == 0) {
            engine.xor_mem(0x1D, 0x01);
        }
    }
}

mod routine_0005 {
    use super::*;
    pub fn routine_0005(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xE3, 0x01);
        engine.set_mem(0xE5, 0x10);
        engine.set_mem(0xE6, 0x04);
        loop {
            let mut p: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
            if cbool(engine.mem(u16v(p + 1)) != 0) {
                routine_0007(engine, r);
            } else if (cbool(engine.mem(0x20) & 0x40) && !cbool(engine.mem(0xFD) & 0x40)) {
                routine_0006(engine, r);
            }
            engine.set_mem(0xE3, u8v(engine.mem(0xE3) + 1));
            {
                let mut np: i32 = u16v(engine.mem(0xE5) + 0x10);
                engine.set_mem(0xE5, u8v(np));
                engine.set_mem(0xE6, u8v(engine.mem(0xE6) + (np >> 8)));
            }
            if !cbool(engine.mem(0xE3) < 0x04) {
                break;
            }
        }
        routine_0012(engine, r);
    }
}

mod routine_0006 {
    use super::*;
    pub fn routine_0006(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xFD, u8v((engine.mem(0x20) & 0x40) | engine.mem(0xFD)));
        r.value = engine.mem(0xFD);
        r.offset = 0x02;
        routine_0015(engine, r);
        routine_0008(engine, r);
        routine_0010(engine, r);
        if !cbool(r.carry) {
            engine.set_mem(0xF9, engine.mem(0x0E));
            engine.set_mem(0xFB, engine.mem(0x0A));
            engine.set_mem(0xEE, 0x18);
            engine.set_mem(0xEF, 0x00);
            engine.set_mem(0xED, 0x21);
            engine.set_mem(0x8F, 0x19);
        }
        if cbool(engine.mem(0xEE) != 0) {
            routine_0009(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }
}

mod routine_0007 {
    use super::*;
    pub fn routine_0007(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xEE, u8v(engine.mem(0xEE) - 1));
        if cbool(engine.mem(0xEE) != 0) {
            routine_0011(engine, r);
            routine_0010(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
            } else {
                engine.set_mem(0xF9, engine.mem(0x0E));
                engine.set_mem(0xFB, engine.mem(0x0A));
            }
        }
        if cbool(engine.mem(0xEE) != 0) {
            routine_0009(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }
}

mod routine_0008 {
    use super::*;
    pub fn routine_0008(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0xF7) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF7) << 2);
            engine.set_mem(0x0A, u8v(a + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0xF5) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF5) << 2);
            engine.set_mem(0x0E, u8v(a + engine.mem(0x0E)));
        }
    }
}

mod routine_0009 {
    use super::*;
    pub fn routine_0009(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x08, u8v(engine.mem(0xEE) & 0x0C));
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | engine.mem(0x08)));
    }
}

mod routine_0010 {
    use super::*;
    pub fn routine_0010(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xA1) {
            return;
        }
        if cbool(engine.mem(0x0E) < 0xF1) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0x00) {
            return;
        }
        r.carry = 1;
    }
}

mod routine_0011 {
    use super::*;
    pub fn routine_0011(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0A, engine.mem(0xFB));
        if cbool(engine.mem(0xF7) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0xF7) + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0xF5) != 0) {
            engine.set_mem(0x0E, u8v(engine.mem(0xF5) + engine.mem(0x0E)));
        }
    }
}

mod routine_0012 {
    use super::*;
    pub fn routine_0012(engine: &mut Engine, r: &mut RoutineContext) {
        let mut count: i32 = 0;
        engine.set_mem(0x0F, 0x88);
        engine.set_mem(0x0E, 0x10);
        count = 0x03;
        loop {
            routine_0013(engine, r);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + 0x08));
            engine.set_mem(0x0E, u8v(engine.mem(0x0E) + 0x10));
            count = u8v(count - 0x01);
            if !cbool(count != 0) {
                break;
            }
        }
    }
}

mod routine_0013 {
    use super::*;
    pub fn routine_0013(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x0F);
        let mut y: i32 = engine.mem(0x0E);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(u16v(0x0401 + y)) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x040E + y)) >= 0xBF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut attr: i32 = engine.mem(u16v(0x0402 + y));
                        engine.set_mem(u16v(0x0202 + x), attr);
                        engine.set_mem(u16v(0x0206 + x), attr);
                        if cbool(attr & 0x40) {
                            let mut t: i32 = engine.mem(u16v(0x0400 + y));
                            engine.set_mem(u16v(0x0205 + x), t);
                            engine.set_mem(u16v(0x0201 + x), u8v(t + 2));
                        } else {
                            let mut t: i32 = engine.mem(u16v(0x0400 + y));
                            engine.set_mem(u16v(0x0201 + x), t);
                            engine.set_mem(u16v(0x0205 + x), u8v(t + 2));
                        }
                    }
                    {
                        let mut px: i32 = engine.mem(u16v(0x040C + y));
                        engine.set_mem(u16v(0x0203 + x), px);
                        engine.set_mem(u16v(0x0207 + x), u8v(px + 8));
                        {
                            let mut py: i32 = u8v(engine.mem(u16v(0x040E + y)) + 0x2B);
                            engine.set_mem(u16v(0x0200 + x), py);
                            engine.set_mem(u16v(0x0204 + x), py);
                        }
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(u16v(0x0200 + x), 0xEF);
                    engine.set_mem(u16v(0x0204 + x), 0xEF);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0014 {
    use super::*;
    pub fn routine_0014(engine: &mut Engine, r: &mut RoutineContext) {
        let mut c: i32 = u8v(engine.mem(0x3E) - 1);
        if cbool(c & 0x80) {
            c = 0x07;
        }
        engine.set_mem(0x3E, c);
        let mut x: i32 = u8v(c << 2);
        let mut base: i32 = (if cbool(c & 0x06) { 0x0280 } else { 0x0210 });
        engine.set_mem(0x0200, engine.mem(u16v(base + 0 + x)));
        engine.set_mem(0x0201, engine.mem(u16v(base + 1 + x)));
        engine.set_mem(0x0202, engine.mem(u16v(base + 2 + x)));
        engine.set_mem(0x0203, engine.mem(u16v(base + 3 + x)));
        engine.set_mem(u16v(base + x), 0xEF);
    }
}

mod routine_0015 {
    use super::*;
    pub fn routine_0015(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        engine.set_mem(0x09, r.offset);
        x = u8v((r.value & 0x0F) << 1);
        a = 0x00;
        y = r.offset;
        loop {
            a = u8v(a + engine.mem(u16v(0xFE8B + x)));
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y != 0) {
                break;
            }
        }
        engine.set_mem(0xF5, a);
        y = engine.mem(0x09);
        a = 0x00;
        loop {
            a = u8v(a + engine.mem(u16v(0xFE8C + x)));
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y != 0) {
                break;
            }
        }
        engine.set_mem(0xF7, a);
    }
}

mod routine_0016 {
    use super::*;
    pub fn routine_0016(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0240 + x), engine.mem(u16v(0xAAFC + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        build_object_health_meter_standard_tiles(engine, r);
    }
}

mod routine_0017 {
    use super::*;
    pub fn routine_0017(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0240 + x), engine.mem(u16v(0xAB3C + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        build_object_health_meter_alt_tiles(engine, r);
    }
}

mod routine_0018 {
    use super::*;
    pub fn routine_0018(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x02C0 + x), engine.mem(u16v(0xAB7C + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        build_player_health_meter_sprites(engine, r);
    }
}

mod routine_0019 {
    use super::*;
    pub fn routine_0019(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = u8v(engine.mem(0x56) & 0x1F);
        engine.set_mem(0x08, v);
        engine.set_mem(0x0410, u8v((engine.mem(0x0410) & 0xE0) | v));
        engine.set_mem(0x0420, u8v((engine.mem(0x0420) & 0xE0) | v));
        engine.set_mem(0x0430, u8v((engine.mem(0x0430) & 0xE0) | v));
        let mut xf: i32 = engine.mem(0x43);
        engine.set_mem(0x041C, xf);
        engine.set_mem(0x042C, xf);
        engine.set_mem(0x043C, xf);
        let mut x: i32 = engine.mem(0x44);
        {
            let __old = x;
            x += 1;
            __old
        };
        engine.set_mem(0x042D, x);
        x -= 3;
        engine.set_mem(0x043D, x);
        {
            let __old = x;
            x -= 1;
            __old
        };
        engine.set_mem(0x041D, x);
    }
}

mod routine_0021 {
    use super::*;
    fn tail_acbb(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0024(engine, r);
        routine_0025(engine, r);
        routine_0026(engine, r);
    }

    fn tail_aca1(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x43, engine.mem(0x0E));
        engine.set_mem(0x45, engine.mem(0x0A));
        routine_0028(engine, r);
        tail_acbb(engine, r);
    }

    fn tail_acaf(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        routine_0028(engine, r);
        tail_acbb(engine, r);
    }

    pub fn routine_0021(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    r.value = engine.mem(0x20);
                    if cbool(r.value & 0x10) {
                        routine_0029(engine, r);
                        return;
                    }
                    if !cbool(engine.mem(0x20) & 0x40) {
                        engine.set_mem(0xFD, u8v(engine.mem(0xFD) & 0x0F));
                    }
                    r.value = u8v(engine.mem(0x20) & 0x0F);
                    if cbool(r.value != 0) {
                        engine.set_mem(0x08, r.value);
                        engine.set_mem(0xFD, u8v((engine.mem(0xFD) & 0xF0) | engine.mem(0x08)));
                    }
                    if cbool(engine.mem(0x85) == 0) {
                        if cbool((engine.mem(0x26) & 0x40) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.index = u8v(engine.mem(0x3E) + 1);
                        if cbool(((r.index) & 0x06) != 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        {
                            let mut sum: i32 =
                                u8v(u8v(engine.mem(0x1C) + engine.mem(u16v(0x040C + r.index))));
                            r.value = u8v((if cbool(sum < 0xB0) { 0x0A } else { 0x05 }));
                        }
                        routine_0030(engine, r);
                        engine.set_mem(0x4F, 0x0A);
                        engine.set_mem(0x8F, 0x21);
                        engine.set_mem(0x90, 0x02);
                        engine.set_mem(0x85, 0x01);
                        build_player_health_meter_sprites(engine, r);
                    }
                    if (cbool(engine.mem(0x4F) == 0) && cbool(engine.mem(0x4E) == 0)) {
                        engine.set_mem(0x85, 0x00);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x20, u8v((engine.mem(0x20) & 0xF0) | 0x02));
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0032(engine, r);
                    if cbool(engine.mem(0x4E) != 0) {
                        r.value = u8v(engine.mem(0x4E) >> 2);
                        r.value = u8v(r.value + 1);
                        engine.set_mem(0x4B, r.value);
                        routine_0027(engine, r);
                        if !cbool(r.carry) {
                            tail_aca1(engine, r);
                            return;
                        }
                        engine.set_mem(0x49, 0x00);
                        routine_0027(engine, r);
                        if !cbool(r.carry) {
                            return;
                        }
                        tail_acaf(engine, r);
                        return;
                    }
                    if cbool(engine.mem(0x4F) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0x20) & 0x80) {
                        engine.set_mem(0x22, 0x00);
                        r.value = 0x00;
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    routine_0022(engine, r);
                    r.value = 0x00;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x4F, r.value);
                    routine_0027(engine, r);
                    if cbool(r.carry) {
                        tail_acaf(engine, r);
                        return;
                    }
                    tail_aca1(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0022 {
    use super::*;
    pub fn routine_0022(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x4F);
        if cbool(x == 0) {
            if cbool(engine.mem(0x22) != 0) {
                return;
            }
            engine.set_mem(0x8F, 0x1B);
            engine.set_mem(0x4F, engine.mem(0x5C));
        }
        engine.set_mem(0x22, 0x01);
        engine.set_mem(0x4F, u8v(engine.mem(0x4F) - 1));
        engine.set_mem(0x4B, u8v((u8v(x >> 2) ^ 0xFF) + 1));
        routine_0027(engine, r);
        if cbool(r.carry) {
            engine.set_mem(0x49, 0x00);
            routine_0027(engine, r);
        }
        if !cbool(r.carry) {
            engine.set_mem(0x43, engine.mem(0x0E));
            engine.set_mem(0x45, engine.mem(0x0A));
            routine_0028(engine, r);
        } else {
            engine.set_mem(0x4F, 0x00);
            engine.set_mem(0x4E, 0x00);
            routine_0028(engine, r);
        }
        routine_0024(engine, r);
        routine_0025(engine, r);
        routine_0026(engine, r);
    }
}

mod routine_0023 {
    use super::*;
    pub fn routine_0023(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0x4B) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x4B) + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0x49) != 0) {
            engine.set_mem(0x0E, u8v(engine.mem(0x49) + engine.mem(0x0E)));
        }
    }
}

mod routine_0024 {
    use super::*;
    pub fn routine_0024(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    r.index = 0x09;
                    if cbool(u8v(engine.mem(0x20) & 0xBF) == 0x80) {
                        engine.set_mem(0x56, r.index);
                        return;
                    }
                    if cbool(engine.mem(0x4B) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4B) & 0x80) {
                        if cbool(engine.mem(0x4F) == 0) {
                            engine.set_mem(0x56, r.index);
                            return;
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4E) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool((engine.mem(0x20) & 0x04) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.index = 0x0D;
                    engine.set_mem(0x56, r.index);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.index = 0x01;
                    r.offset = 0x00;
                    if cbool(engine.mem(0x49) & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    r.offset = 0x40;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x08, r.index);
                    engine.set_mem(0x56, u8v((engine.mem(0x56) & 0x07) | engine.mem(0x08)));
                    engine.set_mem(0x57, r.offset);
                    return;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    r.index = 0x39;
                    r.offset = 0x00;
                    if cbool(engine.mem(0x49) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    r.offset = 0x40;
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0x08, r.index);
                    engine.set_mem(0x56, u8v((engine.mem(0x56) & 0x03) | engine.mem(0x08)));
                    engine.set_mem(0x57, r.offset);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0025 {
    use super::*;
    pub fn routine_0025(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x56) < 0x20) {
            let mut a: i32 = engine.mem(0x56);
            if cbool(engine.mem(0x20) & 0x40) {
                a = u8v(a | 0x10);
            } else {
                a = u8v(a & 0xEF);
            }
            engine.set_mem(0x56, a);
        }
        if cbool((engine.mem(0x20) & 0x0F) == 0) {
            return;
        }
        if cbool((engine.mem(0x4F) | engine.mem(0x4E)) != 0) {
            return;
        }
        engine.set_mem(0x4D, u8v(engine.mem(0x4D) + 1));
        if cbool((engine.mem(0x4D) & 0x07) != 0) {
            return;
        }
        if cbool(engine.mem(0x56) & 0x08) {
            engine.xor_mem(0x57, 0x40);
        } else {
            engine.xor_mem(0x56, 0x04);
        }
    }
}

mod routine_0026 {
    use super::*;
    pub fn routine_0026(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x85) != 0) {
            if cbool((engine.mem(0x84) & 0x01) == 0) {
                engine.set_mem(0x0210, 0xEF);
                engine.set_mem(0x0214, 0xEF);
                return;
            }
        }
        engine.set_mem(0x0210, u8v(engine.mem(0x45) + 0x2B));
        engine.set_mem(0x0214, u8v(engine.mem(0x45) + 0x2B));
        engine.set_mem(0x0213, engine.mem(0x43));
        engine.set_mem(0x0217, u8v(engine.mem(0x43) + 0x08));
        engine.set_mem(0x0212, u8v(engine.mem(0x57) | 0x20));
        engine.set_mem(0x0216, u8v(engine.mem(0x57) | 0x20));
        if cbool(engine.mem(0x57) & 0x40) {
            r.index = engine.mem(0x56);
            engine.set_mem(0x0215, r.index);
            r.index = u8v(r.index + 2);
            engine.set_mem(0x0211, r.index);
        } else {
            r.index = engine.mem(0x56);
            engine.set_mem(0x0211, r.index);
            r.index = u8v(r.index + 2);
            engine.set_mem(0x0215, r.index);
        }
    }
}

mod routine_0027 {
    use super::*;
    pub fn routine_0027(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved: i32 = engine.mem(0x4B);
        loop {
            routine_0023(engine, r);
            routine_0031(engine, r);
            if !cbool(r.carry) {
                break;
            }
            {
                let mut x: i32 = engine.mem(0x4B);
                if cbool(x == 0) {
                    r.carry = 1;
                    break;
                }
                if !cbool(x & 0x80) {
                    x = u8v(x - 1);
                    x = u8v(x - 1);
                }
                x = u8v(x + 1);
                engine.set_mem(0x4B, x);
                if cbool(x != 0) {
                    continue;
                }
                r.carry = 1;
                break;
            }
        }
        engine.set_mem(0x4B, saved);
    }
}

mod routine_0028 {
    use super::*;
    pub fn routine_0028(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x4F) != 0) {
            r.carry = 0;
            return;
        }
        if cbool(engine.mem(0x45) < 0xA0) {
            engine.set_mem(0x4E, u8v(engine.mem(0x4E) + 1));
            return;
        }
        {
            let mut a: i32 = engine.mem(0x4E);
            if cbool(a >= engine.mem(0x5C)) {
                a = u8v(a - 0x07);
                if cbool(a >= engine.mem(0x5C)) {
                    a = engine.mem(0x5C);
                }
                a = u8v(a - 0x01);
                engine.set_mem(0x4F, a);
                engine.set_mem(0x8F, 0x0A);
            }
        }
        engine.set_mem(0x4E, 0x00);
    }
}

mod routine_0030 {
    use super::*;
    pub fn routine_0030(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lhs: i32 = 0;
        let mut res: i32 = 0;
        engine.set_mem(0x08, r.value);
        lhs = engine.mem(0x58);
        {
            let mut t: i32 = u16v(lhs) - u16v(engine.mem(0x08));
            res = u8v(t);
            r.carry = (if cbool(t & 0x100) { 0 } else { 1 });
            r.zero = u8v((if cbool(res == 0) { 1 } else { 0 }));
            r.negative = (res >> 7) & 1;
        }
        engine.set_mem(0x58, res);
        if !cbool(r.carry) {
            engine.set_mem(0x58, 0x00);
        }
    }
}

mod routine_0031 {
    use super::*;
    pub fn routine_0031(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xA1) {
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(0x0E) >= 0xF1) {
            r.carry = 1;
            return;
        }
        r.carry = 0;
    }
}

mod routine_0032 {
    use super::*;
    pub fn routine_0032(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = u8v((engine.mem(0x20) & 0x0F) << 1);
        engine.set_mem(0x49, engine.mem(u16v(0xFE8B + r.index)));
        engine.set_mem(0x4B, engine.mem(u16v(0xFE8C + r.index)));
    }
}

mod routine_0035 {
    use super::*;
    pub fn routine_0035(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x04;
        rng_update(engine, r);
        r.index = r.value;
        engine.set_mem(0x20, engine.mem(u16v(0xB0FE + r.index)));
        r.value = 0x0A;
        rng_update(engine, r);
        r.index = r.value;
        if cbool(r.index == 0) {
            engine.set_mem(0x20, u8v(engine.mem(0x20) | 0x40));
        }
    }
}

mod routine_0036 {
    use super::*;
    pub fn routine_0036(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x7F;
            while cbool(x >= 0) {
                engine.set_mem(0x0240 + x, engine.mem(0xB71C + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0037 {
    use super::*;
    pub fn routine_0037(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x1F;
            while cbool(x >= 0) {
                engine.set_mem(0x0240 + x, engine.mem(0xB6FC + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0038 {
    use super::*;
    pub fn routine_0038(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0xEF;
        if cbool(engine.mem(0x84) & 0x30) {
            x = 0x80;
        }
        engine.set_mem(0x0240, x);
        engine.set_mem(0x0244, x);
        engine.set_mem(0x0248, x);
        engine.set_mem(0x024C, x);
        engine.set_mem(0x0250, x);
        engine.set_mem(0x0254, x);
        engine.set_mem(0x0258, x);
        engine.set_mem(0x025C, x);
        r.index = x;
    }
}

mod routine_0040 {
    use super::*;
    pub fn routine_0040(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = 0;
        let mut y: i32 = 0;
        routine_0048(engine, r);
        ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        {
            let mut i: i32 = 0;
            {
                i = 0;
                y = 0;
                while cbool(i < 256) {
                    let mut b: i32 = engine.mem(u16v(ptr + y));
                    if cbool(b == 0x00) {
                        r.carry = 1;
                        return;
                    }
                    if cbool(b == 0x0D) {
                        routine_0042(engine, r);
                        r.value = 0x05;
                        routine_0044(engine, r);
                        r.carry = 0;
                        return;
                    }
                    engine.set_mem(0x08, b & 0x0F);
                    engine.set_mem(u16v(0x0140 + y), u8v(((b & 0xF0) << 1) | engine.mem(0x08)));
                    {
                        i += 1;
                        i
                    };
                    {
                        y += 1;
                        y
                    };
                }
            }
        }
    }
}

mod routine_0041 {
    use super::*;
    pub fn routine_0041(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = 0;
        let mut y: i32 = 0;
        let mut c: i32 = 0;
        let mut lo: i32 = 0;
        let mut guard: i32 = 0;
        routine_0048(engine, r);
        ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        y = 0x00;
        {
            guard = 0;
            while cbool(guard < 256) {
                c = engine.mem(u16v(ptr + y));
                if cbool(c == 0x00) {
                    r.carry = 1;
                    return;
                }
                if cbool(c == 0x0D) {
                    let mut sum: i32 = 0;
                    {
                        let __old = y;
                        y += 1;
                        __old
                    };
                    sum = u16v(y + engine.mem(0x0C));
                    lo = u8v(sum);
                    engine.set_mem(0x0C, lo);
                    if cbool(sum > 0xFF) {
                        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 1));
                    }
                    r.value = 0x08;
                    routine_0042(engine, r);
                    r.value = 0x05;
                    routine_0044(engine, r);
                    r.carry = 0;
                    return;
                }
                {
                    let mut lonib: i32 = c & 0x0F;
                    let mut hi: i32 = 0;
                    let mut v: i32 = 0;
                    engine.set_mem(0x08, lonib);
                    hi = u8v((c & 0xF0) << 1);
                    v = u8v(hi | engine.mem(0x08));
                    v = u8v(v + 0x10);
                    engine.set_mem(u16v(0x0140 + y), v);
                }
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                {
                    let __old = guard;
                    guard += 1;
                    __old
                };
            }
        }
    }
}

mod routine_0042 {
    use super::*;
    pub fn routine_0042(engine: &mut Engine, r: &mut RoutineContext) {
        let mut hi: i32 = 0x08;
        let mut a: i32 = engine.mem(0x0A);
        let mut carry: i32 = 0;
        carry = a >> 7;
        a = u8v(a << 1);
        hi = u8v((hi << 1) | carry);
        carry = a >> 7;
        a = u8v(a << 1);
        hi = u8v((hi << 1) | carry);
        engine.set_mem(0x17, hi);
        engine.set_mem(0x16, a);
        r.value = a;
    }
}

mod routine_0043 {
    use super::*;
    pub fn routine_0043(engine: &mut Engine, r: &mut RoutineContext) {
        loop {
            engine.inc_mem(0x0A);
            if cbool((engine.mem(0x0A) & 0x07) == 0) {
                break;
            }
            r.value = 0xFF;
            routine_0044(engine, r);
        }
        if cbool(engine.mem(0x0A) == 0xF0) {
            engine.set_mem(0x0A, 0x00);
        }
    }
}

mod routine_0044 {
    use super::*;
    pub fn routine_0044(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_a: i32 = u8v(r.value);
        let mut v: i32 = u8v(engine.mem(0x0A) + 0x06);
        if cbool(v >= 0xF0) {
            v = u8v(v + 0x10);
        }
        engine.set_mem(0x1E, v);
        r.value = saved_a;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0046 {
    use super::*;
    pub fn routine_0046(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        engine.set_mem(0x0180, 0x0F);
        engine.set_mem(0x0181, 0x0C);
        engine.set_mem(0x0182, 0x10);
        engine.set_mem(0x0183, 0x30);
        {
            x = 0x1B;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0184 + x), 0x0F);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.value = 0x0F;
        routine_0075(engine, r);
    }
}

mod routine_0047 {
    use super::*;
    pub fn routine_0047(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x00;
        loop {
            engine.set_mem(u16v(0x0200 + x), 0xEF);
            x = u8v(x + 4);
            if !cbool(x != 0) {
                break;
            }
        }
        r.index = x;
        r.value = 0xEF;
    }
}

mod routine_0048 {
    use super::*;
    pub fn routine_0048(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        {
            y = 0x1F;
            while cbool(y >= 0) {
                engine.set_mem(0x0140 + y, 0xC0);
                {
                    y -= 1;
                    y
                };
            }
        }
        r.value = 0xC0;
        r.offset = 0xFF;
    }
}

mod routine_0051 {
    use super::*;
    pub fn routine_0051(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        x = 0x0F;
        y = 0x07;
        loop {
            let mut b: i32 = engine.mem(u16v(0x0308 + y));
            engine.set_mem(u16v(0x0322 + x), u8v(b >> 4));
            {
                let __old = x;
                x -= 1;
                __old
            };
            engine.set_mem(u16v(0x0322 + x), u8v(b & 0x0F));
            {
                let __old = x;
                x -= 1;
                __old
            };
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y >= 0) {
                break;
            }
        }
        {
            x = 0x0F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0332 + x), u8v(engine.mem(u16v(0x0310 + x)) & 0x0F));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            let mut a: i32 = engine.mem(0x0320);
            {
                x = 0x0F;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0322 + x));
                    engine.set_mem(u16v(0x0322 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = engine.mem(0x0321);
            {
                x = 0x0F;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0332 + x));
                    engine.set_mem(u16v(0x0332 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = 0x00;
            {
                x = 0x1F;
                while cbool(x >= 0) {
                    a = u8v(a + engine.mem(u16v(0x0322 + x)));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            engine.set_mem(0x0389, a);
        }
        {
            let mut a: i32 = 0x0A;
            {
                x = 0x1F;
                while cbool(x >= 0) {
                    a = u8v(a ^ engine.mem(u16v(0x0322 + x)));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            engine.set_mem(0x038A, a);
        }
        {
            let mut a: i32 = engine.mem(0x0389);
            {
                x = 0x0E;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0322 + x));
                    engine.set_mem(u16v(0x0322 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = engine.mem(0x038A);
            {
                x = 0x0E;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0332 + x));
                    engine.set_mem(u16v(0x0332 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        engine.set_mem(0x3A, engine.mem(0x0331));
        engine.set_mem(0x3B, engine.mem(0x0341));
        {
            x = 0x0E;
            while cbool(x >= 0) {
                engine.set_mem(0x08, u8v(x));
                r.value = 0x20;
                rng_update(engine, r);
                x = engine.mem(0x08);
                engine.set_mem(
                    u16v(0x0322 + x),
                    u8v(r.value ^ engine.mem(u16v(0x0322 + x))),
                );
                r.value = 0x20;
                rng_update(engine, r);
                x = engine.mem(0x08);
                engine.set_mem(
                    u16v(0x0332 + x),
                    u8v(r.value ^ engine.mem(u16v(0x0332 + x))),
                );
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
    }
}

mod routine_0052 {
    use super::*;
    pub fn routine_0052(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    {
                        x = 0x1F;
                        while cbool(x >= 0) {
                            engine.set_mem(u16v(0x0342 + x), engine.mem(u16v(0x0322 + x)));
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    engine.set_mem(0x3A, engine.mem(0x0351));
                    engine.set_mem(0x3B, engine.mem(0x0361));
                    {
                        x = 0x0E;
                        while cbool(x >= 0) {
                            engine.set_mem(0x08, u8v(x));
                            r.value = 0x20;
                            rng_update(engine, r);
                            x = engine.mem(0x08);
                            engine.xor_mem(u16v(0x0342 + x), r.value);
                            r.value = 0x20;
                            rng_update(engine, r);
                            x = engine.mem(0x08);
                            engine.xor_mem(u16v(0x0352 + x), r.value);
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0E;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0352 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0352 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x038A, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0E;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0342 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0342 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0389, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x1F;
                            while cbool(x >= 0) {
                                a = u8v(a + engine.mem(u16v(0x0342 + x)));
                                {
                                    let __old = x;
                                    x -= 1;
                                    __old
                                };
                            }
                        }
                        if cbool(a != engine.mem(0x0389)) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    {
                        let mut a: i32 = 0x0A;
                        {
                            x = 0x1F;
                            while cbool(x >= 0) {
                                a ^= engine.mem(u16v(0x0342 + x));
                                {
                                    let __old = x;
                                    x -= 1;
                                    __old
                                };
                            }
                        }
                        if cbool(a != engine.mem(0x038A)) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0F;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0342 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0342 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0320, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0F;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0352 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0352 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0321, a);
                    }
                    x = 0x0F;
                    y = 0x07;
                    loop {
                        let mut hi: i32 = engine.mem(u16v(0x0342 + x));
                        {
                            let __old = x;
                            x -= 1;
                            __old
                        };
                        let mut lo: i32 = engine.mem(u16v(0x0342 + x));
                        {
                            let __old = x;
                            x -= 1;
                            __old
                        };
                        engine.set_mem(u16v(0x0308 + y), u8v((hi << 4) | lo));
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                        if !cbool(y >= 0) {
                            break;
                        }
                    }
                    {
                        x = 0x0F;
                        while cbool(x >= 0) {
                            engine.set_mem(u16v(0x0310 + x), engine.mem(u16v(0x0352 + x)));
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    r.carry = 0;
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x8F, 0x1C);
                    engine.set_mem(0x90, 0x1C);
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0053 {
    use super::*;
    pub fn routine_0053(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x40;
        loop {
            engine.set_mem(u16v(0x00 + x), engine.mem(u16v(0x9B9F + x)));
            {
                let __old = x;
                x += 1;
                __old
            };
            if !cbool(x != 0x8C) {
                break;
            }
        }
        {
            x = 0x1F;
            while cbool((x & 0x80) == 0) {
                engine.set_mem(u16v(0x0180 + x), 0x0F);
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.value = 0x0F;
        r.index = 0xFF;
    }
}

mod routine_0054 {
    use super::*;
    pub fn routine_0054(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl: i32 = engine.mem(0x23);
        let mut mask: i32 = engine.mem(0x24);
        let mut i: i32 = 0;
        engine.device_write(0x2000, ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, mask & 0xE7);
        engine.device_write(0x2006, 0x20);
        engine.device_write(0x2006, 0x00);
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0x9EC9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0x9FC9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0xA0C9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0xA1C9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x2A, engine.mem(0xA2E9));
        engine.set_mem(0x2B, engine.mem(0xA2EA));
        engine.set_mem(0x24, mask);
        engine.set_mem(0x23, ctrl);
        engine.device_write(0x2000, ctrl);
        r.value = ctrl;
        r.index = 0;
    }
}

mod routine_0056 {
    use super::*;
    pub fn routine_0056(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        loop {
            let mut lo: i32 = engine.mem(u16v(0x0180 + x)) & 0x0F;
            engine.set_mem(0x08, lo);
            let mut hi: i32 = engine.mem(u16v(0x0180 + x)) & 0xF0;
            let mut sub: i32 = engine.mem(0x09);
            let mut res: i32 = 0;
            if cbool(hi >= sub) {
                res = u8v(u8v(hi - sub) | lo);
            } else {
                res = 0x0F;
            }
            engine.set_mem(u16v(0x0180 + x), res);
            {
                x += 1;
                x
            };
            {
                y -= 1;
                y
            };
            if !cbool(y != 0) {
                break;
            }
        }
        r.index = x;
        r.offset = y;
    }
}

mod routine_0057 {
    use super::*;
    pub fn routine_0057(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x1F;
            while cbool(x >= 0) {
                engine.set_mem(0x0180 + x, engine.mem(0xA2C9 + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0059 {
    use super::*;
    pub fn routine_0059(engine: &mut Engine, r: &mut RoutineContext) {
        let mut scrollpos: i32 = u8v((engine.mem(0x7C) << 4) | engine.mem(0x7B));
        let mut playerpos: i32 = u8v((engine.mem(0x44) << 4) | engine.mem(0x43));
        let mut delta: i32 = u8v(playerpos - scrollpos);
        let mut out_carry: i32 = 0;
        engine.set_mem(0x08, scrollpos);
        if cbool(delta < 0x60) {
            if cbool((engine.mem(0x7C) | engine.mem(0x7B)) == 0) {
                out_carry = 1;
            } else {
                let mut t: i32 = u8v(engine.mem(0x44) - 0x06);
                if cbool(engine.mem(0x44) < 0x06) {
                    engine.set_mem(0x7B, 0x00);
                    engine.set_mem(0x7C, 0x00);
                    out_carry = 0;
                } else {
                    engine.set_mem(0x7C, t);
                    engine.set_mem(0x7B, engine.mem(0x43));
                    engine.set_mem(0x7F, 0xFF);
                    out_carry = 0;
                }
            }
        } else if cbool(delta < 0x91) {
            out_carry = 1;
        } else {
            if cbool(engine.mem(0x7C) >= 0x30) {
                engine.set_mem(0x7C, 0x30);
                engine.set_mem(0x7B, 0x00);
                out_carry = 1;
            } else {
                engine.set_mem(0x7C, u8v(engine.mem(0x44) - 0x09));
                engine.set_mem(0x7B, engine.mem(0x43));
                engine.set_mem(0x7F, 0x01);
                out_carry = 0;
            }
        }
        routine_0060(engine, r);
        r.carry = u8v(out_carry);
    }
}

mod routine_0060 {
    use super::*;
    pub fn routine_0060(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x7C);
        let mut carry: i32 = 0;
        let mut i: i32 = 0;
        {
            i = 0;
            while cbool(i < 4) {
                carry = u8v(a >> 7);
                a = u8v(a << 1);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        a |= engine.mem(0x7B);
        r.index = a;
        a = u8v(0x00 << 1) | carry;
        engine.set_mem(0x1C, r.index);
        engine.set_mem(0x1D, a);
        r.value = a;
    }
}

mod routine_0061 {
    use super::*;
    pub fn routine_0061(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut world_x: i32 = 0;
        if (cbool(engine.mem(0x85) != 0) && cbool((engine.mem(0x84) & 0x01) == 0)) {
            engine.set_mem(0x0210, 0xEF);
            engine.set_mem(0x0214, 0xEF);
            return;
        }
        a = u8v(engine.mem(0x45) + 0x2B);
        engine.set_mem(0x0210, a);
        engine.set_mem(0x0214, a);
        world_x = u8v((engine.mem(0x7C) << 4) | engine.mem(0x7B));
        engine.set_mem(0x08, world_x);
        a = u8v((engine.mem(0x44) << 4) | engine.mem(0x43));
        a = u8v(a - world_x);
        engine.set_mem(0x0213, a);
        engine.set_mem(0x0217, u8v(a + 0x08));
        engine.set_mem(0x0212, engine.mem(0x57));
        engine.set_mem(0x0216, engine.mem(0x57));
        x = engine.mem(0x56);
        if cbool(engine.mem(0x57) & 0x40) {
            engine.set_mem(0x0215, x);
            engine.set_mem(0x0211, u8v(x + 2));
        } else {
            engine.set_mem(0x0211, x);
            engine.set_mem(0x0215, u8v(x + 2));
        }
    }
}

mod routine_0062 {
    use super::*;
    pub fn routine_0062(engine: &mut Engine, r: &mut RoutineContext) {
        let mut value: i32 = 0;
        let mut slot: i32 = 0;
        let mut offset: i32 = 0;
        value = engine.mem(0x0055);
        slot = 0x13;
        if cbool(value >= 0x03) {
            slot = 0xEF;
            engine.set_mem(0x0238, slot);
            engine.set_mem(0x023C, slot);
        } else {
            engine.set_mem(0x0238, slot);
            engine.set_mem(0x023C, slot);
            value = u8v(value << 4);
            value = u8v(value + 0xC8);
            engine.set_mem(0x023B, value);
            value = u8v(value + 0x08);
            engine.set_mem(0x023F, value);
            engine.set_mem(0x0239, 0xFF);
            engine.set_mem(0x023D, 0xFF);
            engine.set_mem(0x023A, 0x01);
            engine.set_mem(0x023E, 0x41);
        }
        slot = 0x02;
        offset = 0x10;
        loop {
            value = engine.mem(u16v((0x0051) + slot));
            if cbool(value & 0x80) {
                value = 0xEF;
            } else {
                value = u8v(value << 2);
                value = u8v(value + 0xA1);
                engine.set_mem(u16v(0x0221 + offset), value);
                value = u8v(value + 0x02);
                engine.set_mem(u16v(0x0225 + offset), value);
                value = u8v(offset << 1);
                value = u8v(value + 0xC8);
                engine.set_mem(u16v(0x0223 + offset), value);
                value = u8v(value + 0x08);
                engine.set_mem(u16v(0x0227 + offset), value);
                engine.set_mem(u16v(0x0222 + offset), 0x01);
                engine.set_mem(u16v(0x0226 + offset), 0x01);
                value = 0x13;
            }
            engine.set_mem(u16v(0x0220 + offset), value);
            engine.set_mem(u16v(0x0224 + offset), value);
            offset = u8v(offset - 0x08);
            if cbool(
                {
                    let __old = slot;
                    slot -= 1;
                    __old
                } == 0,
            ) {
                break;
            }
        }
    }
}

mod routine_0063 {
    use super::*;
    pub fn routine_0063(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        engine.set_mem(0x0A, 0x10);
        x = engine.mem(0x3F);
        y = engine.mem(0x3E);
        loop {
            r.index = x;
            r.offset = y;
            routine_0064(engine, r);
            x = u8v((u8v(x + 0x08)) | 0x80);
            y = u8v(y + 0x30);
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
            if !cbool(engine.mem(0x0A) != 0) {
                break;
            }
        }
        engine.set_mem(0x3F, u8v((u8v(x + 0x38)) | 0x80));
        engine.set_mem(0x3E, u8v(y + 0x10));
    }
}

mod routine_0064 {
    use super::*;
    pub fn routine_0064(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        let mut a: i32 = 0;
        let mut t: i32 = 0;
        let mut carry: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem((0x0400) + 1 + y) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem((0x0400) + 0x0E + y) >= 0xBF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem((0x0400) + 2 + y);
                    engine.set_mem(0x0202 + x, a);
                    engine.set_mem(0x0206 + x, a);
                    if cbool(a & 0x40) {
                        a = engine.mem((0x0400) + y);
                        engine.set_mem(0x0205 + x, a);
                        a = u8v(a + 0x02);
                        engine.set_mem(0x0201 + x, a);
                    } else {
                        a = engine.mem((0x0400) + y);
                        engine.set_mem(0x0201 + x, a);
                        a = u8v(a + 0x02);
                        engine.set_mem(0x0205 + x, a);
                    }
                    {
                        let mut d: i32 =
                            u16v(engine.mem((0x0400) + 0x0C + y)) + 0x100 - engine.mem(0x007B);
                        a = u8v(d) & 0x0F;
                        engine.set_mem(0x08, a);
                        carry = u8v(d >> 8);
                        d = u16v(engine.mem((0x0400) + 0x0D + y)) + carry - engine.mem(0x007C) - 1;
                        a = u8v(d);
                        if cbool(a >= 0x10) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        a = u8v((a << 4) | engine.mem(0x08));
                        engine.set_mem(0x08, a);
                    }
                    if cbool(engine.mem((0x0400) + 1 + y) == 0x01) {
                        if cbool(engine.mem((0x0400) + 0x0F + y) != 0) {
                            engine.set_mem(
                                0x08,
                                u8v(engine.mem(0x08) + engine.mem((0x0400) + 0x0F + y)),
                            );
                            engine.set_mem((0x0400) + 0x0F + y, 0x00);
                        }
                    }
                    a = engine.mem(0x08);
                    if cbool(a >= 0xEF) {
                        engine.set_mem(0x0203 + x, a);
                        t = u8v(engine.mem((0x0400) + 0x0E + y) + 0x2B);
                        engine.set_mem(0x0200 + x, t);
                        engine.set_mem(0x0204 + x, 0xEF);
                        return;
                    }
                    engine.set_mem(0x0203 + x, a);
                    a = u8v(a + 0x08);
                    engine.set_mem(0x0207 + x, a);
                    t = u8v(engine.mem((0x0400) + 0x0E + y) + 0x2B);
                    engine.set_mem(0x0200 + x, t);
                    engine.set_mem(0x0204 + x, t);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x0200 + x, 0xEF);
                    engine.set_mem(0x0204 + x, 0xEF);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0065 {
    use super::*;
    pub fn routine_0065(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 3;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0200 + x), engine.mem(u16v(0xFF6B + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            x = 4;
            while cbool(x <= 0xFF) {
                engine.set_mem(u16v(0x0200 + x), 0xF8);
                {
                    let __old = x;
                    x += 1;
                    __old
                };
            }
        }
        r.index = 0x00;
    }
}

mod routine_0066 {
    use super::*;
    pub fn routine_0066(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl: i32 = engine.mem(0x23);
        let mut mask: i32 = engine.mem(0x24);
        let mut i: i32 = 0;
        engine.device_write(0x2000, ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, mask & 0xE7);
        engine.device_write(0x2006, 0x20);
        engine.device_write(0x2006, 0x00);
        {
            i = 0;
            while cbool(i < 5 * 0xC0) {
                engine.device_write(0x2007, 0xC0);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, 0x00);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 5 * 0xC0) {
                engine.device_write(0x2007, 0xC0);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, 0x00);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x24, mask);
        engine.set_mem(0x23, ctrl);
        engine.device_write(0x2000, ctrl);
        r.value = ctrl;
        r.index = 0;
        r.offset = 0;
    }
}

mod routine_0073 {
    use super::*;
    pub fn routine_0073(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        loop {
            let mut lo: i32 = engine.mem(u16v(0x0180 + x)) & 0x0F;
            engine.set_mem(0x08, lo);
            let mut hi: i32 = engine.mem(u16v(0x0180 + x)) & 0xF0;
            let mut sub: i32 = engine.mem(0x09);
            let mut res: i32 = 0;
            if cbool(hi >= sub) {
                res = u8v(u8v(hi - sub) | lo);
            } else {
                res = 0x0F;
            }
            engine.set_mem(u16v(0x0180 + x), res);
            {
                x += 1;
                x
            };
            {
                y -= 1;
                y
            };
            if !cbool(y != 0) {
                break;
            }
        }
        r.index = x;
        r.offset = y;
    }
}

mod routine_0075 {
    use super::*;
    pub fn routine_0075(engine: &mut Engine, r: &mut RoutineContext) {
        clear_pending_vram_job(engine, r);
        engine.set_mem(0x16, 0x00);
        engine.set_mem(0x17, 0x3F);
        r.value = 0x02;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0076 {
    use super::*;
    pub fn routine_0076(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_ctrl: i32 = 0;
        let mut saved_mask: i32 = 0;
        let mut i: i32 = 0;
        clear_pending_vram_job(engine, r);
        saved_ctrl = engine.mem(0x23);
        engine.device_write(0x2000, saved_ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        saved_mask = engine.mem(0x24);
        engine.device_write(0x2001, saved_mask & 0xE7);
        engine.device_write(0x2006, 0x23);
        engine.device_write(0x2006, 0x20);
        {
            i = 0;
            while cbool(i < 0xA0) {
                engine.device_write(0x2007, engine.mem(u16v(0xFECB + i)));
                {
                    i += 1;
                    i
                };
            }
        }
        engine.device_write(0x2006, 0x23);
        engine.device_write(0x2006, 0xF0);
        {
            i = 0;
            while cbool(i < 0x10) {
                engine.device_write(0x2007, 0x00);
                {
                    i += 1;
                    i
                };
            }
        }
        engine.add_mem(0x29, 1);
        engine.set_mem(0x24, saved_mask);
        engine.set_mem(0x23, saved_ctrl);
        engine.device_write(0x2000, saved_ctrl);
        r.value = saved_ctrl;
        r.offset = 0x00;
    }
}

mod routine_0077 {
    use super::*;
    pub fn routine_0077(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x7C) & 0xFE);
        engine.set_mem(0x0D, 0x00);
        resolve_room_tile_pointer(engine, r);
        routine_0079(engine, r);
    }
}

mod routine_0078 {
    use super::*;
    pub fn routine_0078(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x7C) & 0xFE);
        engine.set_mem(0x0D, 0x00);
        resolve_room_tile_pointer(engine, r);
        engine.set_mem(0x0D, u8v((engine.mem(0x0D) - 0x05) + engine.mem(0x76)));
        routine_0079(engine, r);
    }
}

mod routine_0079 {
    use super::*;
    pub fn routine_0079(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl_save: i32 = engine.mem(0x23);
        let mut v29_save: i32 = engine.mem(0x29);
        let mut v24_save: i32 = engine.mem(0x24);
        let mut c0c_save: i32 = engine.mem(0x0C);
        let mut c0d_save: i32 = engine.mem(0x0D);
        let mut p0C: i32 = 0;
        let mut p79: i32 = 0;
        let mut outer: i32 = 0;
        engine.device_write(0x2000, (ctrl_save & 0x7F) | 0x04);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, v24_save & 0xE7);
        p79 = u16v(engine.mem(0x79) | (engine.mem(0x7A) << 8));
        {
            let mut sx: i32 = engine.mem(0x7C);
            let mut lo: i32 = u8v((sx << 1) & 0x1C);
            let mut hi: i32 = u8v((sx & 0x10) >> 2);
            let mut t: i32 = u16v(0x00 + lo);
            engine.set_mem(0x16, u8v(t));
            engine.set_mem(0x17, u8v(0x20 + hi + (t >> 8)));
        }
        engine.set_mem(0x0A, 0x12);
        p0C = u16v(c0c_save | (c0d_save << 8));
        {
            outer = 0;
            while cbool(outer < 0x12) {
                let mut inner: i32 = 0;
                engine.set_mem(0x0B, 0x0C);
                engine.device_write(0x2006, engine.mem(0x17));
                engine.device_write(0x2006, engine.mem(0x16));
                engine.set_mem(0x08, 0x00);
                loop {
                    let mut idx: i32 = engine.mem(u16v(p0C + engine.mem(0x08)));
                    let mut y: i32 = u8v(idx << 2);
                    engine.device_write(0x2007, engine.mem(u16v(p79 + y)));
                    engine.device_write(0x2007, engine.mem(u16v(p79 + u8v(y + 1))));
                    engine.inc_mem(0x08);
                    engine.dec_mem(0x0B);
                    if !cbool(engine.mem(0x0B) != 0) {
                        break;
                    }
                }
                engine.set_mem(0x0B, 0x0C);
                engine.device_write(0x2006, engine.mem(0x17));
                inner = u8v(engine.mem(0x16) + 1);
                engine.device_write(0x2006, inner);
                engine.set_mem(0x08, 0x00);
                loop {
                    let mut idx: i32 = engine.mem(u16v(p0C + engine.mem(0x08)));
                    let mut y: i32 = u8v((idx << 2) + 2);
                    engine.device_write(0x2007, engine.mem(u16v(p79 + y)));
                    engine.device_write(0x2007, engine.mem(u16v(p79 + u8v(y + 1))));
                    engine.inc_mem(0x08);
                    engine.dec_mem(0x0B);
                    if !cbool(engine.mem(0x0B) != 0) {
                        break;
                    }
                }
                engine.add_mem(0x16, 2);
                if cbool(engine.mem(0x16) & 0x20) {
                    engine.set_mem(0x16, 0x00);
                    engine.xor_mem(0x17, 0x04);
                }
                {
                    let mut t: i32 = u16v(0x0C + engine.mem(0x0C));
                    engine.set_mem(0x0C, u8v(t));
                    engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
                    p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                }
                engine.dec_mem(0x0A);
                {
                    let __old = outer;
                    outer += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x0D, c0d_save);
        engine.set_mem(0x0C, c0c_save);
        p0C = u16v(c0c_save | (c0d_save << 8));
        {
            let mut sx: i32 = engine.mem(0x7C);
            let mut lo: i32 = u8v((sx >> 1) & 0x07);
            let mut hi: i32 = u8v((sx & 0x10) >> 2);
            let mut t: i32 = u16v(0xC0 + lo);
            engine.set_mem(0x16, u8v(t));
            engine.set_mem(0x17, u8v(0x23 + hi + (t >> 8)));
        }
        engine.set_mem(0x0A, 0x09);
        loop {
            let mut x: i32 = 0;
            {
                x = 6;
                while cbool(x > 0) {
                    let mut a: i32 = 0;
                    a = engine.mem(u16v(p0C + 0x0D));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x01));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x0C));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x00));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    engine.device_write(0x2006, engine.mem(0x17));
                    engine.device_write(0x2006, engine.mem(0x16));
                    engine.device_write(0x2007, engine.mem(0x08));
                    {
                        let mut t: i32 = u16v(0x02 + engine.mem(0x0C));
                        engine.set_mem(0x0C, u8v(t));
                        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
                    }
                    {
                        let mut t: i32 = u16v(0x08 + engine.mem(0x16));
                        engine.set_mem(0x16, u8v(t));
                        engine.set_mem(0x17, u8v(engine.mem(0x17) + (t >> 8)));
                    }
                    p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            {
                let mut t: i32 = u16v(0x0C + engine.mem(0x0C));
                engine.set_mem(0x0C, u8v(t));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
            }
            {
                let mut t: i32 = u16v(0xD1 + engine.mem(0x16));
                engine.set_mem(0x16, u8v(t));
                engine.set_mem(0x17, u8v(engine.mem(0x17) + 0xFF + (t >> 8)));
            }
            p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            if cbool(engine.mem(0x16) & 0x08) {
                engine.set_mem(0x16, 0xC0);
                engine.xor_mem(0x17, 0x04);
            }
            engine.dec_mem(0x0A);
            if cbool(engine.mem(0x0A) == 0) {
                break;
            }
        }
        engine.set_mem(0x24, v24_save);
        engine.set_mem(0x29, v29_save);
        engine.set_mem(0x23, ctrl_save);
        engine.device_write(0x2000, ctrl_save);
        r.value = ctrl_save;
        r.index = 0;
    }
}

mod routine_0080 {
    use super::*;
    pub fn routine_0080(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sx: i32 = 0;
        clear_pending_vram_job(engine, r);
        sx = engine.mem(0x7C);
        engine.set_mem(0x16, u8v((sx << 1) & 0x1F));
        engine.set_mem(0x17, u8v((sx & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        engine.set_mem(0x08, sx);
        engine.set_mem(0x09, 0x10);
        loop {
            engine.set_mem(0x0C, engine.mem(0x08));
            farcall_bank_09_r7(engine, r);
            engine.set_mem(0x16, u8v(engine.mem(0x16) + 2));
            if cbool(engine.mem(0x16) & 0x20) {
                engine.set_mem(0x16, 0x00);
                engine.xor_mem(0x17, 0x04);
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            engine.set_mem(0x09, u8v(engine.mem(0x09) - 1));
            if !cbool(engine.mem(0x09) != 0) {
                break;
            }
        }
    }
}

mod routine_0081 {
    use super::*;
    pub fn routine_0081(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sx: i32 = 0;
        clear_pending_vram_job(engine, r);
        sx = engine.mem(0x7C);
        engine.set_mem(0x16, u8v((sx << 1) & 0x1F));
        engine.set_mem(0x17, u8v((sx & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        engine.set_mem(0x08, sx);
        engine.set_mem(0x09, 0x10);
        loop {
            engine.set_mem(0x0C, engine.mem(0x08));
            routine_0083(engine, r);
            engine.set_mem(0x16, u8v(engine.mem(0x16) + 2));
            if cbool(engine.mem(0x16) & 0x20) {
                engine.set_mem(0x16, 0x00);
                engine.xor_mem(0x17, 0x04);
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            engine.set_mem(0x09, u8v(engine.mem(0x09) - 1));
            if !cbool(engine.mem(0x09) != 0) {
                break;
            }
        }
    }
}

mod routine_0082 {
    use super::*;
    pub fn routine_0082(engine: &mut Engine, r: &mut RoutineContext) {
        let mut col: i32 = 0;
        clear_pending_vram_job(engine, r);
        if cbool(engine.mem(0x7F) & 0x80) {
            col = engine.mem(0x7C);
        } else {
            col = u8v(engine.mem(0x7C) + 0x10);
        }
        engine.set_mem(0x0C, col);
        engine.set_mem(0x16, u8v((col << 1) & 0x1F));
        engine.set_mem(0x17, u8v((col & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        farcall_bank_09_r7(engine, r);
    }
}

mod routine_0083 {
    use super::*;
    pub fn routine_0083(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0D, 0x00);
        resolve_room_tile_pointer(engine, r);
        engine.set_mem(0x0D, u8v(u8v(engine.mem(0x0D) - 0x05) + engine.mem(0x76)));
        metasprite_build(engine, r);
    }
}

mod routine_0084 {
    use super::*;
    pub fn routine_0084(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0086(engine, r);
        text_attr_build(engine, r);
        routine_0087(engine, r);
    }
}

mod routine_0085 {
    use super::*;
    pub fn routine_0085(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut ptr: i32 = 0;
        let mut i: i32 = 0;
        engine.set_mem(0x77, engine.mem(0x75));
        engine.set_mem(0x78, engine.mem(0x76));
        lo = engine.mem(0x77);
        hi = engine.mem(0x78);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0500 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0600 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0700 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        r.offset = 0;
    }
}

mod routine_0086 {
    use super::*;
    pub fn routine_0086(engine: &mut Engine, r: &mut RoutineContext) {
        let mut bank: i32 = u8v(engine.mem(0x48) >> 1);
        let mut t: i32 = 0;
        let mut lo: i32 = 0;
        if cbool(bank != engine.mem(0x30)) {
            engine.set_mem(0x30, bank);
            r.value = 0xFF;
            queue_ppu_job_and_wait(engine, r);
        }
        t = u8v(((engine.mem(0x48) & 0x01) << 2));
        t = u8v((t | engine.mem(0x47)) << 2);
        lo = u8v(t + 0x80);
        engine.set_mem(0x76, lo);
        engine.set_mem(0x78, u8v(lo + 0x03));
        engine.set_mem(0x77, 0x00);
        engine.set_mem(0x75, 0x00);
        r.carry = u8v((if cbool((lo + 0x03) > 0xFF) { 1 } else { 0 }));
    }
}

mod routine_0087 {
    use super::*;
    pub fn routine_0087(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        {
            y = 0xE0;
            while cbool(y <= 0xFF) {
                engine.set_mem(u16v(0x00A0 + u8v(y)), engine.mem(u16v(ptr + u8v(y))));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
            }
        }
        a = engine.mem(0x40);
        if cbool(a >= 0x06) {
            r.value = a;
            r.carry = 1;
            return;
        }
        a = u8v((a << 2) + 0x03);
        x = a;
        {
            y = 0x03;
            while cbool(y >= 0) {
                engine.set_mem(u16v(0x0190 + y), engine.mem(u16v(0xFFC5 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
                {
                    let __old = y;
                    y -= 1;
                    __old
                };
            }
        }
        r.value = a;
        r.index = x;
        r.offset = u8v(0xFF);
        r.carry = 0;
    }
}

mod routine_0088 {
    use super::*;
    pub fn routine_0088(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ms_y: i32 = engine.mem(0x48);
        let mut ms_x: i32 = engine.mem(0x47);
        let mut idx: i32 = u8v(((ms_y << 2) & 0x04) | ms_x);
        let mut a: i32 = engine.mem(u16v(0x0300 + idx));
        let mut cnt: i32 = u8v((ms_y >> 1) + 1);
        loop {
            a = u8v(a << 1);
            if !cbool(
                {
                    cnt -= 1;
                    cnt
                } != 0,
            ) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0089 {
    use super::*;
    pub fn routine_0089(engine: &mut Engine, r: &mut RoutineContext) {
        let mut msy: i32 = engine.mem(0x48);
        let mut x: i32 = u8v((msy >> 1) + 1);
        let mut a: i32 = 0xFF;
        let mut carry: i32 = 0;
        let mut idx: i32 = 0;
        loop {
            let mut newcarry: i32 = a & 1;
            a = u8v((carry << 7) | (a >> 1));
            carry = newcarry;
            {
                let __old = x;
                x -= 1;
                __old
            };
            if !cbool(x != 0) {
                break;
            }
        }
        idx = u8v(((u8v(msy << 2)) & 0x04) | engine.mem(0x47));
        engine.and_mem(u16v(0x0300 + idx), a);
        r.value = engine.mem(u16v(0x0300 + idx));
        r.index = idx;
    }
}

mod resolve_room_tile_pointer {
    use super::*;

    /// Converts tile coordinates in `0x0C/0x0D` into the current room tile
    /// pointer. `0x10/0x11` receives the same offset plus the room base pointer.
    pub fn resolve_room_tile_pointer(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_y: i32 = engine.mem(0x0D);
        scale_room_tile_column(engine, r);
        engine.set_mem(0x11, engine.mem(0x0D));
        {
            let tile_row: i32 = u8v(tile_y >> 4);
            let room_offset: i32 = u16v(tile_row + engine.mem(0x0C));
            engine.set_mem(0x0C, u8v(room_offset));
            engine.set_mem(0x10, u8v(room_offset));
            if cbool(room_offset & 0x100) {
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 1));
                engine.set_mem(0x11, u8v(engine.mem(0x11) + 1));
            }
        }
        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 0x05));
        {
            let room_ptr_lo: i32 = u16v(engine.mem(0x10) + engine.mem(0x75));
            let carry: i32 = u8v(room_ptr_lo >> 8);
            engine.set_mem(0x10, u8v(room_ptr_lo));
            engine.set_mem(0x11, u8v(engine.mem(0x11) + engine.mem(0x76) + carry));
        }
    }
}

mod scale_room_tile_column {
    use super::*;

    /// Multiplies the tile column in `0x0C` by the room-data stride of 12.
    pub fn scale_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
        let column_times_four: i32 = u16v(engine.mem(0x0C) << 2);
        let column_times_eight: i32 = u16v(engine.mem(0x0C) << 3);
        let column_offset: i32 = u16v(column_times_four + column_times_eight);
        engine.set_mem(0x0C, u8v(column_offset));
        engine.set_mem(0x0D, u8v(column_offset >> 8));
        r.index = u8v(column_times_four >> 8);
        r.offset = u8v(column_times_four);
        r.value = u8v(column_offset >> 8);
    }
}

mod upload_resource_hud {
    use super::*;

    /// Queues the resource HUD VRAM upload after resource counters changed.
    pub fn upload_resource_hud(engine: &mut Engine, r: &mut RoutineContext) {
        clear_pending_vram_job(engine, r);
        engine.set_mem(0x16, 0x60);
        engine.set_mem(0x17, 0x23);
        r.value = 0x04;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod sync_health_hud {
    use super::*;

    /// Clamps the health counter and queues the health HUD digits for redraw.
    pub fn sync_health_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut health: i32 = engine.mem(0x58);
        if cbool(health >= 0x6D) {
            health = 0x6D;
        }
        engine.set_mem(0x58, health);
        engine.set_mem(0x08, health);
        r.value = health;
        r.index = 0x00;
        build_status_resource_meter_tiles(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod sync_magic_hud {
    use super::*;

    /// Clamps the magic counter and queues the magic HUD digits for redraw.
    pub fn sync_magic_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut magic: i32 = engine.mem(0x59);
        if cbool(magic >= 0x6D) {
            magic = 0x6D;
        }
        engine.set_mem(0x59, magic);
        engine.set_mem(0x08, magic);
        r.value = magic;
        r.index = 0x06;
        build_status_resource_meter_tiles(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod sync_key_hud {
    use super::*;

    /// Clamps the key counter and queues the key HUD digits for redraw.
    pub fn sync_key_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut keys: i32 = engine.mem(0x5B);
        if cbool(keys >= 0x6D) {
            keys = 0x6D;
        }
        engine.set_mem(0x5B, keys);
        engine.set_mem(0x08, keys);
        r.value = keys;
        r.index = 0x0C;
        build_status_resource_meter_tiles(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod sync_coin_hud {
    use super::*;

    /// Clamps the coin counter and queues the coin HUD digits for redraw.
    pub fn sync_coin_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut coins: i32 = engine.mem(0x5A);
        if cbool(coins >= 0x6D) {
            coins = 0x6D;
        }
        engine.set_mem(0x5A, coins);
        engine.set_mem(0x08, coins);
        r.value = coins;
        r.index = 0x12;
        build_status_resource_meter_tiles(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod build_status_resource_meter_tiles {
    use super::*;

    /// Builds the two-row status resource meter in the VRAM staging buffers.
    /// `r.index` selects the meter column and `0x08` contains the resource value.
    pub fn build_status_resource_meter_tiles(engine: &mut Engine, r: &mut RoutineContext) {
        let base_slot: i32 = r.index;
        engine.set_mem(0x01FB, base_slot);
        for tile_offset in 0..5 {
            engine.set_mem(u16v(0x0101 + base_slot + tile_offset), 0xDC);
        }

        let base_slot: i32 = engine.mem(0x01FB);
        engine.set_mem(0x01FB, base_slot);
        for tile_offset in 0..5 {
            engine.set_mem(u16v(0x0121 + base_slot + tile_offset), 0xDF);
        }

        let base_slot: i32 = engine.mem(0x01FB);
        r.index = base_slot;
        split_meter_value(engine, r);

        let mut filled_blocks: i32 = r.offset;
        let mut tile_slot: i32 = base_slot;
        loop {
            filled_blocks = u8v(filled_blocks - 1);
            if cbool(filled_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0101 + tile_slot));
            filled_blocks = u8v(filled_blocks - 1);
            if cbool(filled_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0101 + tile_slot));
            tile_slot = u8v(tile_slot + 1);
        }

        tile_slot = base_slot;
        let mut partial_blocks: i32 = engine.mem(0x08);
        loop {
            partial_blocks = u8v(partial_blocks - 1);
            if cbool(partial_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0121 + tile_slot));
            partial_blocks = u8v(partial_blocks - 1);
            if cbool(partial_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0121 + tile_slot));
            tile_slot = u8v(tile_slot + 1);
        }
        r.offset = partial_blocks;
        r.index = tile_slot;
        r.value = base_slot;
    }
}

mod build_object_health_meter_alt_tiles {
    use super::*;

    /// Builds an object health meter using the alternate `0xA5/0xAB` sprite
    /// tile pair.
    pub fn build_object_health_meter_alt_tiles(engine: &mut Engine, r: &mut RoutineContext) {
        let mut health: i32 = engine.mem(0x0405);
        if cbool(health >= 0x6D) {
            health = 0x6D;
        }
        engine.set_mem(0x08, health);
        engine.set_mem(0x09, 0x00);
        r.index = 0xA5;
        r.offset = 0xAB;
        build_health_meter_sprites(engine, r);
    }
}

mod build_object_health_meter_standard_tiles {
    use super::*;

    /// Builds an object health meter using the standard `0x65/0x6B` sprite
    /// tile pair.
    pub fn build_object_health_meter_standard_tiles(engine: &mut Engine, r: &mut RoutineContext) {
        let mut health: i32 = engine.mem(0x0405);
        if cbool(health >= 0x6D) {
            health = 0x6D;
        }
        engine.set_mem(0x08, health);
        engine.set_mem(0x09, 0x00);
        let full_tile: i32 = 0x65;
        let empty_tile: i32 = 0x6B;
        let mut sprite_slot: i32 = engine.mem(0x09);
        engine.set_mem(u16v(0x0259 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x025D + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0261 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0265 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0269 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x026D + sprite_slot), empty_tile);
        engine.set_mem(u16v(0x0271 + sprite_slot), empty_tile);
        engine.set_mem(u16v(0x0275 + sprite_slot), empty_tile);
        engine.set_mem(u16v(0x0279 + sprite_slot), empty_tile);
        engine.set_mem(u16v(0x027D + sprite_slot), empty_tile);
        split_meter_value(engine, r);
        let mut filled_blocks: i32 = r.offset;
        sprite_slot = u8v(engine.mem(0x09) + 0x18);
        loop {
            filled_blocks = u8v(filled_blocks - 1);
            if cbool(filled_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            filled_blocks = u8v(filled_blocks - 1);
            if cbool(filled_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            sprite_slot = u8v(sprite_slot + 4);
        }

        sprite_slot = u8v(engine.mem(0x09) + 0x2C);
        let mut partial_blocks: i32 = engine.mem(0x08);
        loop {
            partial_blocks = u8v(partial_blocks - 1);
            if cbool(partial_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            partial_blocks = u8v(partial_blocks - 1);
            if cbool(partial_blocks == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            engine.dec_mem(u16v(0x0241 + sprite_slot));
            sprite_slot = u8v(sprite_slot + 4);
        }
        r.value = full_tile;
        r.index = sprite_slot;
        r.offset = partial_blocks;
    }
}

mod build_player_health_meter_sprites {
    use super::*;

    /// Builds the player health meter sprite strip at the second OAM meter
    /// slot.
    pub fn build_player_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
        let mut health: i32 = engine.mem(0x58);
        if cbool(health >= 0x6D) {
            health = 0x6D;
        }
        engine.set_mem(0x08, health);
        engine.set_mem(0x09, 0x80);
        r.index = 0x65;
        r.offset = 0x6B;
        build_health_meter_sprites(engine, r);
    }
}

mod build_health_meter_sprites {
    use super::*;

    /// Builds a ten-sprite two-row health meter. `0x09` selects the OAM slot,
    /// `r.index` is the full tile, `r.offset` is the empty tile, and `0x08`
    /// contains the value.
    pub fn build_health_meter_sprites(engine: &mut Engine, r: &mut RoutineContext) {
        let sprite_slot: i32 = engine.mem(0x09);
        let full_tile: i32 = u8v(r.index);
        engine.set_mem(u16v(0x0259 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x025D + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0261 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0265 + sprite_slot), full_tile);
        engine.set_mem(u16v(0x0269 + sprite_slot), full_tile);
        {
            let empty_tile: i32 = u8v(r.offset);
            engine.set_mem(u16v(0x026D + sprite_slot), empty_tile);
            engine.set_mem(u16v(0x0271 + sprite_slot), empty_tile);
            engine.set_mem(u16v(0x0275 + sprite_slot), empty_tile);
            engine.set_mem(u16v(0x0279 + sprite_slot), empty_tile);
            engine.set_mem(u16v(0x027D + sprite_slot), empty_tile);
        }
        split_meter_value(engine, r);
        {
            let mut filled_blocks: i32 = u8v(r.offset);
            let mut sprite_slot: i32 = u8v(engine.mem(0x09) + 0x18);
            loop {
                filled_blocks = u8v(filled_blocks - 1);
                if cbool(filled_blocks == 0) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + sprite_slot), 2);
                filled_blocks = u8v(filled_blocks - 1);
                if cbool(filled_blocks == 0) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + sprite_slot), 2);
                sprite_slot = u8v(sprite_slot + 4);
            }
        }
        {
            let mut partial_blocks: i32 = engine.mem(0x08);
            let mut sprite_slot: i32 = u8v(engine.mem(0x09) + 0x2C);
            loop {
                partial_blocks = u8v(partial_blocks - 1);
                if cbool(partial_blocks == 0) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + sprite_slot), 2);
                partial_blocks = u8v(partial_blocks - 1);
                if cbool(partial_blocks == 0) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + sprite_slot), 2);
                sprite_slot = u8v(sprite_slot + 4);
            }
        }
    }
}

mod split_meter_value {
    use super::*;

    /// Splits the value in `0x08` into full 10-point meter blocks (`r.offset`)
    /// and a one-based partial block (`0x08`/`r.value`).
    pub fn split_meter_value(engine: &mut Engine, r: &mut RoutineContext) {
        let mut remainder: i32 = engine.mem(0x08);
        let mut full_blocks: i32 = 0;
        let mut carry: i32 = 1;
        loop {
            full_blocks = u8v(full_blocks + 1);
            let trial: i32 = (remainder) - 0x0A - (1 - carry);
            remainder = u8v(trial);
            carry = u8v((if cbool(trial >= 0) { 1 } else { 0 }));
            if !cbool(carry) {
                break;
            }
        }
        remainder = u8v(remainder + 0x0B + carry);
        engine.set_mem(0x08, remainder);
        r.value = remainder;
        r.offset = full_blocks;
    }
}

mod read_debounced_buttons {
    use super::*;

    /// Waits for release, then press, then release again, returning the pressed
    /// button byte in `r.value` and `0x20`.
    pub fn read_debounced_buttons(engine: &mut Engine, r: &mut RoutineContext) {
        wait_for_buttons_released(engine, r);
        wait_for_button_press(engine, r);
        {
            let pressed_buttons: i32 = u8v(r.value);
            wait_for_buttons_released(engine, r);
            r.value = pressed_buttons;
            engine.set_mem(0x20, pressed_buttons);
        }
    }
}

mod clear_pending_vram_job {
    use super::*;

    /// Clears the deferred VRAM job selector at `0x28`.
    pub fn clear_pending_vram_job(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x28, 0);
    }
}

mod build_input_movement_delta {
    use super::*;

    /// Builds player movement deltas from current directional input and speed
    /// `r.offset`, storing them in `0x49..0x4B`.
    pub fn build_input_movement_delta(engine: &mut Engine, r: &mut RoutineContext) {
        let speed: i32 = u8v(r.offset);
        engine.set_mem(0x09, speed);
        if cbool(speed == 0) {
            engine.set_mem(0x49, 0);
            engine.set_mem(0x4A, 0);
            engine.set_mem(0x4B, 0);
            return;
        }
        let direction_index: i32 = u8v((engine.mem(0x20) & 0x0F) << 1);
        let mut horizontal_delta: i32 = 0;
        {
            let mut steps = speed;
            while cbool(steps != 0) {
                horizontal_delta = u8v(horizontal_delta + engine.mem(0xFE8B + direction_index));
                {
                    let __old = steps;
                    steps -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x49, horizontal_delta & 0x0F);
        let sign_fill: i32 = (if cbool(horizontal_delta & 0x80) {
            0xF0
        } else {
            0x00
        });
        engine.set_mem(0x08, sign_fill);
        engine.set_mem(0x4A, u8v(((horizontal_delta & 0xF0) >> 4) | sign_fill));
        let mut vertical_delta: i32 = 0;
        {
            let mut steps = speed;
            while cbool(steps != 0) {
                vertical_delta = u8v(vertical_delta + engine.mem(0xFE8C + direction_index));
                {
                    let __old = steps;
                    steps -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x4B, vertical_delta);
    }
}

mod build_direction_velocity {
    use super::*;

    /// Builds object/projectile velocity from direction bits in `r.value` and
    /// speed `r.offset`, storing it in `0xF5..0xF7`.
    pub fn build_direction_velocity(engine: &mut Engine, r: &mut RoutineContext) {
        let speed: i32 = u8v(r.offset);
        engine.set_mem(0x09, speed);
        if cbool(speed == 0) {
            engine.set_mem(0xF5, 0);
            engine.set_mem(0xF6, 0);
            engine.set_mem(0xF7, 0);
            return;
        }
        let direction_index: i32 = u8v((r.value & 0x0F) << 1);
        let mut horizontal_delta: i32 = 0;
        {
            let mut steps = speed;
            while cbool(steps != 0) {
                horizontal_delta = u8v(horizontal_delta + engine.mem(0xFE8B + direction_index));
                {
                    let __old = steps;
                    steps -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0xF5, horizontal_delta & 0x0F);
        let sign_fill: i32 = (if cbool(horizontal_delta & 0x80) {
            0xF0
        } else {
            0x00
        });
        engine.set_mem(0x08, sign_fill);
        engine.set_mem(0xF6, u8v(((horizontal_delta & 0xF0) >> 4) | sign_fill));
        let mut vertical_delta: i32 = 0;
        {
            let mut steps = speed;
            while cbool(steps != 0) {
                vertical_delta = u8v(vertical_delta + engine.mem(0xFE8C + direction_index));
                {
                    let __old = steps;
                    steps -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0xF7, vertical_delta);
    }
}

mod check_player_overlap {
    use super::*;

    /// Checks the projected object position against the player hitbox. Carry
    /// and `0xEA` are set on overlap.
    pub fn check_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xEA, 0x00);
        check_player_y_overlap(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        check_player_x_overlap(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        engine.set_mem(0xEA, 0x01);
        r.carry = 1;
    }
}

mod check_player_x_overlap {
    use super::*;

    /// Checks horizontal player overlap using projected tile/subtile position
    /// in `0x0E/0x0F`.
    pub fn check_player_x_overlap(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_delta: i32 = u8v(engine.mem(0x0F) - engine.mem(0x44));
        if cbool(tile_delta == 0) {
            return;
        }
        if cbool(tile_delta < 0x02) {
            let subtile_delta: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
            r.carry = (if cbool(subtile_delta & 0x80) { 1 } else { 0 });
            return;
        }
        if cbool(tile_delta < 0xFF) {
            return;
        }
        {
            let subtile_delta: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
            if cbool(subtile_delta == 0) {
                return;
            }
            if cbool(subtile_delta & 0x80) {
                return;
            }
            r.carry = 1;
        }
    }
}

mod check_player_y_overlap {
    use super::*;

    /// Checks vertical player overlap using projected y position in `0x0A`.
    pub fn check_player_y_overlap(engine: &mut Engine, r: &mut RoutineContext) {
        let y_delta: i32 = u8v(engine.mem(0x0A) - engine.mem(0x45));
        if cbool(y_delta < 0x10) {
            r.carry = 1;
        } else if cbool(y_delta < 0xF1) {
            r.carry = 0;
        } else {
            r.carry = 1;
        }
    }
}

mod check_player_overlap_wide {
    use super::*;

    /// Wider player-overlap test used by falling/large movement probes. Carry
    /// and `0xEA` are set on overlap.
    pub fn check_player_overlap_wide(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dy: i32 = 0;
        let mut dx: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xEA, 0x00);
                    dy = u8v(engine.mem(0x0A) - engine.mem(0x45));
                    if (cbool(dy >= 0x10) && cbool(dy < 0xE1)) {
                        r.carry = 0;
                        return;
                    }
                    dx = u8v(engine.mem(0x0F) - engine.mem(0x44));
                    if cbool(dx == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(dx == 0xFF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(dx < 0x02) {
                        let subtile_delta: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
                        if cbool(subtile_delta & 0x80) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.carry = 0;
                        return;
                    }
                    if cbool(dx < 0xFE) {
                        return;
                    }
                    {
                        let subtile_delta: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
                        if cbool(subtile_delta == 0) {
                            return;
                        }
                        if cbool(subtile_delta & 0x80) {
                            return;
                        }
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xEA, 0x01);
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod check_position_out_of_bounds {
    use super::*;

    /// Checks projected position against the general playfield bounds. Carry is
    /// set when the position is outside the allowed area.
    pub fn check_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xC0) {
            r.carry = 1;
        } else if cbool(engine.mem(0x0F) < 0x3F) {
            r.carry = 0;
        } else if cbool(engine.mem(0x0E) == 0) {
            r.carry = 0;
        } else {
            r.carry = 1;
        }
    }
}

mod check_actor_position_out_of_bounds {
    use super::*;

    /// Checks projected actor position against the tighter actor playfield
    /// bounds. Carry is set when the position is outside the allowed area.
    pub fn check_actor_position_out_of_bounds(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xB0) {
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(0x0F) < 0x3F) {
            r.carry = 0;
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            r.carry = 0;
            return;
        }
        r.carry = 1;
    }
}

mod routine_0117 {
    use super::*;
    pub fn routine_0117(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x0F;
            while cbool(x >= 0) {
                r.index = u8v(x);
                r.offset = engine.mem(u16v(0x0060 + x));
                routine_0118(engine, r);
                r.index = u8v(x);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0118 {
    use super::*;
    pub fn routine_0118(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut s: i32 = 0;
        lo = u8v((x & 0x07) << 2);
        lo = u8v(((x & 0x08) << 4) | lo);
        hi = 0x00;
        s = u16v(0xC2 + lo);
        engine.set_mem(0x16, u8v(s));
        engine.set_mem(0x17, u8v(0x20 + hi + (s >> 8)));
        r.value = r.offset;
        routine_0121(engine, r);
        {
            let mut in_: i32 = x;
            let mut dx: i32 = u8v(engine.mem(0x40) << 1);
            let mut yy: i32 = 0;
            let mut carry: i32 = 0;
            let mut v: i32 = 0;
            if cbool(in_ >= 0x08) {
                {
                    let __old = dx;
                    dx += 1;
                    __old
                };
            }
            yy = u8v((in_ & 0x07) + 1);
            v = engine.mem(u16v(0xFFBB + dx));
            carry = 0;
            loop {
                carry = u8v(v >> 7);
                v = u8v(v << 1);
                if !cbool(
                    {
                        yy -= 1;
                        yy
                    } != 0,
                ) {
                    break;
                }
            }
            r.carry = carry;
        }
        r.value = x;
        routine_0122(engine, r);
        if !cbool(r.carry) {
            engine.set_mem(0x18, u8v(engine.mem(0x18) - 0x40));
            engine.set_mem(0x19, u8v(engine.mem(0x19) - 0x40));
        }
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0119 {
    use super::*;
    pub fn routine_0119(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x16, 0xDE);
        engine.set_mem(0x17, 0x21);
        load_effective_projectile_damage(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x1E);
        engine.set_mem(0x17, 0x22);
        load_effective_jump_duration(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x5E);
        engine.set_mem(0x17, 0x22);
        load_effective_projectile_lifetime(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0120 {
    use super::*;
    pub fn routine_0120(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut c: i32 = 0;
        engine.set_mem(0x16, 0x47);
        engine.set_mem(0x17, 0x22);
        if cbool(engine.mem(0x7C) & 0x10) {
            let mut s: i32 = u16v(0x00 + engine.mem(0x16));
            engine.set_mem(0x16, u8v(s));
            engine.set_mem(0x17, u8v(0x04 + engine.mem(0x17) + (s >> 8)));
        }
        r.value = engine.mem(0x81);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        lo = engine.mem(0x16);
        c = u8v((0x0E + lo) >> 8);
        engine.set_mem(0x16, u8v(0x0E + lo));
        hi = engine.mem(0x17);
        engine.set_mem(0x17, u8v(0x00 + hi + c));
        r.value = engine.mem(0x83);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0121 {
    use super::*;
    pub fn routine_0121(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        let mut hi: i32 = 0xD0;
        while cbool(a >= 0x0A) {
            a = u8v(a - 0x0A);
            {
                hi += 1;
                hi
            };
        }
        a = u8v(a + 0xD0);
        engine.set_mem(0x18, a);
        if cbool(hi == 0xD0) {
            hi = 0xC0;
        }
        engine.set_mem(0x19, hi);
    }
}

mod routine_0122 {
    use super::*;
    pub fn routine_0122(engine: &mut Engine, r: &mut RoutineContext) {
        let mut in_: i32 = u8v(r.value);
        let mut x: i32 = u8v(engine.mem(0x40) << 1);
        if cbool(in_ >= 0x08) {
            {
                let __old = x;
                x += 1;
                __old
            };
        }
        let mut y: i32 = u8v((in_ & 0x07) + 1);
        let mut a: i32 = engine.mem(u16v(0xFFBB + x));
        loop {
            a = u8v(a << 1);
            if !cbool(
                {
                    y -= 1;
                    y
                } != 0,
            ) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0123 {
    use super::*;
    pub fn routine_0123(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(r.value == engine.mem(0x8E)) {
            return;
        }
        engine.set_mem(0x8E, r.value);
        song_init(engine, r);
    }
}

mod load_effective_jump_duration {
    use super::*;

    /// Loads the active character's jump duration. Carry is clear when the
    /// selected jump item is present and magic can pay for the boosted value.
    pub fn load_effective_jump_duration(engine: &mut Engine, r: &mut RoutineContext) {
        let selected_item_slot: i32 = engine.mem(0x55);
        let selected_item: i32 = engine.mem((0x51 + selected_item_slot) & 0xFF);
        r.index = selected_item_slot;
        if cbool(selected_item == 0x06) && cbool(engine.mem(0x59) != 0) {
            let base_jump_duration: i32 = engine.mem(0x5C);
            r.value = u8v((base_jump_duration >> 2) + base_jump_duration);
            r.carry = 0;
        } else {
            r.value = engine.mem(0x5C);
            r.carry = 1;
        }
    }
}

mod load_effective_projectile_damage {
    use super::*;

    /// Loads the projectile damage stat. Carry is clear when the selected
    /// projectile-power item is active and magic can pay for the boosted shot.
    pub fn load_effective_projectile_damage(engine: &mut Engine, r: &mut RoutineContext) {
        let selected_item_slot: i32 = engine.mem(0x55);
        let selected_item: i32 = engine.mem((0x51 + selected_item_slot) & 0xFF);
        if cbool(selected_item == 0x08) && cbool(engine.mem(0x59) != 0) {
            r.value = u8v(engine.mem(0x5D) << 2);
            r.carry = 0;
        } else {
            r.value = engine.mem(0x5D);
            r.carry = 1;
        }
    }
}

mod load_effective_projectile_lifetime {
    use super::*;

    /// Loads the projectile lifetime/state byte. Carry is clear when the
    /// selected projectile-range item is active and magic can pay for it.
    pub fn load_effective_projectile_lifetime(engine: &mut Engine, r: &mut RoutineContext) {
        let selected_item_slot: i32 = engine.mem(0x55);
        r.index = selected_item_slot;
        if cbool(engine.mem((0x51 + selected_item_slot) & 0xFF) == 0x09)
            && cbool(engine.mem(0x59) != 0)
        {
            r.value = u8v(engine.mem(0x5F) << 1);
            r.carry = 0;
            return;
        }
        r.value = engine.mem(0x5F);
        r.carry = 1;
    }
}

mod clear_gameplay_object_sprites {
    use super::*;

    /// Hides the gameplay-object half of OAM, leaving HUD sprites untouched.
    pub fn clear_gameplay_object_sprites(engine: &mut Engine, r: &mut RoutineContext) {
        let mut oam_offset: i32 = 0x80;
        loop {
            engine.set_mem(u16v(0x0200 + oam_offset), 0xEF);
            oam_offset = u8v(oam_offset + 4);
            if !cbool(oam_offset != 0) {
                break;
            }
        }
        r.index = oam_offset;
        r.value = 0xEF;
    }
}

mod reset_room_object_slots {
    use super::*;

    /// Clears all 16 object slots to inactive and resets the actor scheduler.
    pub fn reset_room_object_slots(engine: &mut Engine, r: &mut RoutineContext) {
        let mut slot_offset: i32 = 0x00;
        let mut slots_remaining: i32 = 0x10;
        loop {
            engine.set_mem(u16v(0x0401 + slot_offset), 0x00);
            engine.set_mem(u16v(0x0406 + slot_offset), 0x02);
            slot_offset = u8v(slot_offset + 0x10);
            if !cbool(
                {
                    slots_remaining -= 1;
                    slots_remaining
                } != 0,
            ) {
                break;
            }
        }
        engine.set_mem(0xE9, 0x00);
        r.value = 0x00;
        r.index = slot_offset;
        r.offset = 0x00;
    }
}

mod snapshot_inventory_state {
    use super::*;

    /// Saves mutable inventory/progress state before the status or inventory
    /// flows temporarily repurpose the same RAM range.
    pub fn snapshot_inventory_state(engine: &mut Engine, r: &mut RoutineContext) {
        for progress_offset in (0..8).rev() {
            engine.set_mem(
                0x0308 + progress_offset,
                engine.mem(0x0300 + progress_offset),
            );
        }
        for inventory_offset in (0..16).rev() {
            engine.set_mem(
                0x0310 + inventory_offset,
                engine.mem(0x0060 + inventory_offset),
            );
        }
        engine.set_mem(0x0321, engine.mem(0x5A));
        engine.set_mem(0x0320, engine.mem(0x5B));
        r.index = 0xFF;
    }
}

mod restore_inventory_state_snapshot {
    use super::*;

    /// Restores the progress, inventory counts, coins, and keys saved by
    /// `snapshot_inventory_state`.
    pub fn restore_inventory_state_snapshot(engine: &mut Engine, r: &mut RoutineContext) {
        for progress_offset in (0..8).rev() {
            engine.set_mem(
                0x0300 + progress_offset,
                engine.mem(0x0308 + progress_offset),
            );
        }
        for inventory_offset in (0..16).rev() {
            engine.set_mem(
                0x0060 + inventory_offset,
                engine.mem(0x0310 + inventory_offset),
            );
        }
        engine.set_mem(0x5A, engine.mem(0x0321));
        engine.set_mem(0x5B, engine.mem(0x0320));
        r.index = 0xFF;
    }
}

mod upload_inventory_item_list {
    use super::*;

    /// Converts the 32-byte item-list buffer at `0x0322` into the VRAM staging
    /// buffer at `0x0362`, then uploads the two visible nametable rows.
    pub fn upload_inventory_item_list(engine: &mut Engine, r: &mut RoutineContext) {
        let mut source_offset: i32 = 0x1F;
        let mut staging_offset: i32 = 0x26;
        loop {
            {
                let mut chars_in_column: i32 = 0;
                while cbool(chars_in_column < 4) {
                    let mut tile: i32 = u8v(engine.mem(u16v(0x0322 + source_offset)) | 0x80);
                    if cbool(tile >= 0xA0) {
                        tile = 0x7F;
                    }
                    engine.set_mem(u16v(0x0362 + (staging_offset & 0xFF)), tile);
                    staging_offset = (staging_offset - 1) & 0xFF;
                    source_offset = (source_offset - 1) & 0xFF;
                    {
                        chars_in_column += 1;
                        chars_in_column
                    };
                }
            }
            staging_offset = (staging_offset - 1) & 0xFF;
            if !cbool((staging_offset & 0x80) == 0) {
                break;
            }
        }
        engine.set_mem(0x1A, 0x13);
        engine.set_mem(0x1B, 0x00);
        engine.set_mem(0x16, 0xE6);
        engine.set_mem(0x17, 0x24);
        engine.set_mem(0x18, 0x62);
        engine.set_mem(0x19, 0x03);
        r.value = 0x05;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x06);
        engine.set_mem(0x17, 0x25);
        engine.set_mem(0x18, 0x76);
        engine.set_mem(0x19, 0x03);
        r.value = 0x05;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod clear_inventory_item_list_buffer {
    use super::*;

    /// Fills the item-list source buffer with blank tile ids.
    pub fn clear_inventory_item_list_buffer(engine: &mut Engine, r: &mut RoutineContext) {
        for item_list_offset in (0..32).rev() {
            engine.set_mem(0x0322 + item_list_offset, 0x7F);
        }
        r.value = 0x7F;
        r.index = 0xFF;
    }
}

mod tick_player_jump_action {
    use super::*;

    /// Starts or continues the player jump/action arc. `0x4F` is the active
    /// jump timer, `0x22` prevents a held button from restarting the jump, and
    /// selected item `0x06` extends the timer by spending magic.
    pub fn tick_player_jump_action(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x4F) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x22) != 0) {
                        return;
                    }
                    engine.set_mem(0x8F, 0x1B);
                    engine.set_mem(0x4F, engine.mem(0x5C));
                    {
                        let selected_slot: i32 = engine.mem(0x55);
                        if cbool(engine.mem(u16v(0x51 + selected_slot)) == 0x06) {
                            consume_magic_point(engine, r);
                            if !cbool(r.carry) {
                                let jump_timer: i32 = engine.mem(0x4F);
                                engine.set_mem(0x4F, u8v((jump_timer >> 2) + jump_timer));
                            }
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.lotw_nonlocal_handoff = 1;
                    engine.set_mem(0x22, 0x01);
                    {
                        let jump_timer: i32 = engine.mem(0x4F);
                        engine.set_mem(0x4F, u8v(jump_timer - 1));
                        let upward_speed: i32 = u8v(jump_timer >> 2);
                        engine.set_mem(0x4B, u8v((upward_speed ^ 0xFF) + 1));
                    }
                    try_move_player_with_collision(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x49, 0x00);
                    engine.set_mem(0x4A, 0x00);
                    try_move_player_with_collision(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.inc_mem(0x4F);
                    try_nudge_player_to_tile_boundary(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x43, engine.mem(0x0E));
                    engine.set_mem(0x44, engine.mem(0x0F));
                    {
                        let mut y: i32 = engine.mem(0x0A);
                        if cbool(y >= 0xEF) {
                            y = 0x00;
                        }
                        engine.set_mem(0x45, y);
                    }
                    update_player_terrain_contact(engine, r);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x4F, 0x00);
                    engine.set_mem(0x4E, 0x00);
                    update_player_terrain_contact(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    update_player_pose_from_motion(engine, r);
                    tick_player_walk_animation(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_selected_item_effect {
    use super::*;

    /// Applies the currently selected passive/consumable item effect. Item ids
    /// below `0x02` are magic-draining effect timers, `0x0B` refills magic when
    /// empty, and `0x0D` returns the player to the fixed safe room.
    pub fn tick_selected_item_effect(engine: &mut Engine, r: &mut RoutineContext) {
        let selected_slot: i32 = engine.mem(0x0055);
        let selected_item: i32 = engine.mem(u16v((0x0051) + selected_slot));
        if cbool(selected_item >= 0x02) {
            if cbool(selected_item == 0x0B) {
                if cbool(engine.mem(0x59) != 0) {
                    return;
                }
                engine.set_mem(u16v((0x0051) + selected_slot), 0xFF);
                routine_0062(engine, r);
                animate_magic_refill_to_cap(engine, r);
                return;
            }
            if cbool(selected_item != 0x0D) {
                return;
            }
            if cbool(engine.mem(0x0048) >= 0x11) {
                engine.set_mem(0x0055, 0x03);
                return;
            }
            engine.set_mem(u16v((0x0051) + selected_slot), 0xFF);
            routine_0062(engine, r);
            engine.set_mem(0x8F, 0x12);
            engine.set_mem(0x0048, 0x10);
            engine.set_mem(0x0047, 0x03);
            engine.set_mem(0x007C, 0x12);
            engine.set_mem(0x0045, 0xB0);
            engine.set_mem(0x0044, 0x1A);
            engine.set_mem(0x0043, 0x00);
            engine.set_mem(0x007B, 0x00);
            routine_0067(engine, r);
            reset_room_object_slots(engine, r);
            scene_assemble(engine, r);
            routine_0077(engine, r);
            clear_gameplay_object_sprites(engine, r);
            routine_0060(engine, r);
            routine_0061(engine, r);
            routine_0070(engine, r);
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(u16v(0x86 + selected_item)) != 0) {
            return;
        }
        r.index = selected_item;
        consume_magic_point(engine, r);
        if cbool(r.carry == 0) {
            engine.set_mem(u16v(0x86 + selected_item), 0x02);
            return;
        }
        {
            let continue_timer: i32 = engine.mem(0x37);
            if (cbool(continue_timer == 0) || cbool(continue_timer & 0x80)) {
                return;
            }
            engine.set_mem(0x37, 0xFD);
            engine.set_mem(0x8F, 0x1A);
        }
    }
}

mod enter_room_link_destination {
    use super::*;

    /// Enters the destination encoded in the active room link record at
    /// `0x77/0x78 + 0x0C..0x0F`.
    pub fn enter_room_link_destination(engine: &mut Engine, r: &mut RoutineContext) {
        let link_ptr: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        engine.set_mem(0x47, engine.mem(u16v(link_ptr + 0x0C)));
        engine.set_mem(0x48, engine.mem(u16v(link_ptr + 0x0D)));

        let player_tile_x: i32 = engine.mem(u16v(link_ptr + 0x0E));
        engine.set_mem(0x44, player_tile_x);
        let scroll_x: i32 = if cbool(player_tile_x >= 0x08) {
            u8v(player_tile_x - 0x08)
        } else {
            0x00
        };
        engine.set_mem(
            0x7C,
            if cbool(scroll_x >= 0x31) {
                0x30
            } else {
                scroll_x
            },
        );
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);

        r.value = engine.mem(u16v(link_ptr + 0x0F));
        engine.set_mem(0x45, r.value);
        routine_0067(engine, r);
        reset_room_object_slots(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        clear_gameplay_object_sprites(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod enter_fragment_pickup_room {
    use super::*;

    /// After collecting a `0x0E` fragment item, runs the warp transition and
    /// moves to the fragment-specific room selected by `0x6E`.
    pub fn enter_fragment_pickup_room(engine: &mut Engine, r: &mut RoutineContext) {
        run_warp_transition_effect(engine, r);
        engine.set_mem(0x48, 0x11);
        r.index = u8v(engine.mem(0x6E) - 1);
        engine.set_mem(0x47, r.index);
        engine.set_mem(0x7C, 0x12);
        engine.set_mem(0x45, 0x10);
        engine.set_mem(0x44, 0x1A);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);
        r.value = 0x00;
        routine_0067(engine, r);
        reset_room_object_slots(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        clear_gameplay_object_sprites(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod enter_pending_special_exit_room {
    use super::*;

    /// Consumes the pending special-exit flag set by the high-bit actor path
    /// and moves to its fixed destination room.
    pub fn enter_pending_special_exit_room(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xEB, 0x00);
        run_warp_transition_effect(engine, r);
        engine.set_mem(0x2E, 0x3E);
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x47, 0x03);
        engine.set_mem(0x7C, 0x12);
        engine.set_mem(0x45, 0xB0);
        engine.set_mem(0x44, 0x1A);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);
        r.value = 0x00;
        routine_0067(engine, r);
        reset_room_object_slots(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        clear_gameplay_object_sprites(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod check_final_exit_trigger {
    use super::*;

    /// Raises the final-exit flag when item `0x0F` is selected at the exact
    /// room/scroll/player position expected by the original game.
    pub fn check_final_exit_trigger(engine: &mut Engine, r: &mut RoutineContext) {
        let selected_slot: i32 = engine.mem(0x55);
        if (cbool(engine.mem(u16v(0x51 + selected_slot)) == 0x0F)
            && cbool(engine.mem(0x47) == 0x01)
            && cbool(engine.mem(0x48) == 0x05)
            && cbool(engine.mem(0x7C) == 0x10)
            && cbool(engine.mem(0x7B) == 0x00)
            && cbool(engine.mem(0x45) == 0xA0))
        {
            engine.set_mem(0xEC, 0x01);
        }
    }
}

mod run_warp_transition_effect {
    use super::*;

    /// Shared scroll/audio transition used before scripted room warps.
    pub fn run_warp_transition_effect(engine: &mut Engine, r: &mut RoutineContext) {
        let mut outer: i32 = 0;
        routine_0065(engine, r);
        engine.set_mem(0x85, 0x00);
        routine_0061(engine, r);
        routine_0062(engine, r);
        if cbool(engine.mem(0x7C) >= 0x21) {
            engine.set_mem(0x7C, 0x20);
        }
        routine_0080(engine, r);
        engine.set_mem(0x7C, u8v(engine.mem(0x7C) + 0x10));
        routine_0080(engine, r);
        engine.set_mem(0x08, 0x01);
        loop {
            let mut x: i32 = 0x0C;
            loop {
                let mut sum: i32 = u16v(engine.mem(0x1C) + engine.mem(0x08));
                engine.set_mem(0x1C, u8v(sum));
                if cbool(sum & 0x100) {
                    engine.set_mem(0x1D, u8v(engine.mem(0x1D) ^ 0x01));
                }
                r.value = 0xFF;
                queue_ppu_job_and_wait(engine, r);
                if !cbool(
                    {
                        x -= 1;
                        x
                    } != 0,
                ) {
                    break;
                }
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            outer = engine.mem(0x08);
            if !cbool(outer < 0x20) {
                break;
            }
        }
        engine.set_mem(0x8F, 0x18);
        engine.set_mem(0x90, 0xFF);
        r.index = 0x08;
        routine_0074(engine, r);
    }
}

mod handle_player_room_transition {
    use super::*;

    fn scene_rebuild_full(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0067(engine, r);
        reset_room_object_slots(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        clear_gameplay_object_sprites(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }

    fn scene_rebuild_vert(engine: &mut Engine, r: &mut RoutineContext) {
        reset_room_object_slots(engine, r);
        clear_gameplay_object_sprites(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0075(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }

    /// Handles player transitions across room edges. Vertical exits can rebuild
    /// a whole room or a vertical strip; horizontal exits play the side-scroll
    /// transition while moving the map-space room coordinate.
    pub fn handle_player_room_transition(engine: &mut Engine, r: &mut RoutineContext) {
        let player_y: i32 = engine.mem(0x0045);
        if cbool(player_y < 0x10) {
            check_top_boundary_exit_clear(engine, r);
            if cbool(r.carry == 0) {
                return;
            }
            if cbool(engine.mem(0x0048) == 0x00) {
                engine.set_mem(0x0048, 0x10);
                engine.set_mem(0x0047, 0x03);
                engine.set_mem(0x007C, 0x12);
                engine.set_mem(0x0045, 0xB0);
                engine.set_mem(0x0044, 0x1A);
                engine.set_mem(0x0043, 0x00);
                engine.set_mem(0x007B, 0x00);
                scene_rebuild_full(engine, r);
                return;
            }
            if cbool(engine.mem(0x0048) == 0x10) {
                return;
            }
            engine.set_mem(0x0048, u8v(engine.mem(0x0048) - 1));
            engine.set_mem(0x0045, 0xB0);
            scene_rebuild_vert(engine, r);
            return;
        }
        if cbool(player_y >= 0xA1) {
            if cbool(engine.mem(0x0048) == 0x10) {
                engine.set_mem(0x0048, 0x00);
                engine.set_mem(0x0047, 0x00);
                engine.set_mem(0x007C, 0x00);
                engine.set_mem(0x0045, 0x00);
                engine.set_mem(0x0043, 0x00);
                engine.set_mem(0x007B, 0x00);
                engine.set_mem(0x0044, 0x01);
                scene_rebuild_full(engine, r);
                return;
            }
            if cbool(u8v(engine.mem(0x0048) + 1) >= 0x10) {
                return;
            }
            engine.set_mem(0x0048, u8v(engine.mem(0x0048) + 1));
            engine.set_mem(0x0045, 0x00);
            scene_rebuild_vert(engine, r);
            return;
        }
        if cbool(engine.mem(0x0048) == 0x10) {
            return;
        }
        update_player_terrain_contact(engine, r);
        engine.set_mem(0x85, 0x00);
        engine.set_mem(0x56, u8v(engine.mem(0x56) & 0x07));
        if cbool(engine.mem(0x0044) == 0x00) {
            if cbool(u8v((engine.mem(0x0047) - 1)) & 0x80) {
                return;
            }
            engine.set_mem(0x0047, u8v(engine.mem(0x0047) - 1));
            engine.set_mem(0x57, 0x00);
            routine_0061(engine, r);
            engine.set_mem(0x007C, 0x30);
            engine.set_mem(0x0044, 0x3F);
            engine.set_mem(0x0043, 0x00);
        } else {
            if cbool(engine.mem(0x0044) < 0x3E) {
                return;
            }
            if cbool(u8v(engine.mem(0x0047) + 1) >= 0x04) {
                return;
            }
            engine.set_mem(0x0047, u8v(engine.mem(0x0047) + 1));
            engine.set_mem(0x57, 0x40);
            routine_0061(engine, r);
            engine.set_mem(0x007C, 0x00);
            engine.set_mem(0x0043, 0x00);
            engine.set_mem(0x0044, 0x00);
        }
        reset_room_object_slots(engine, r);
        clear_gameplay_object_sprites(engine, r);
        engine.set_mem(0x007B, 0x00);
        scene_assemble(engine, r);
        routine_0080(engine, r);
        routine_0075(engine, r);
        if cbool(engine.mem(0x0044) != 0x00) {
            engine.set_mem(0x1D, 0x01);
            engine.set_mem(0x1C, 0x00);
            engine.set_mem(0x0213, 0x00);
            engine.set_mem(0x0217, 0x08);
            engine.set_mem(0x0A, 0x0F);
            loop {
                engine.set_mem(0x0B, 0x03);
                loop {
                    if cbool(engine.mem(0x0B) == 0) {
                        engine.set_mem(0x0213, u8v(engine.mem(0x0213) - 1));
                        engine.set_mem(0x0217, u8v(engine.mem(0x0217) - 1));
                        if cbool((engine.mem(0x4E) | engine.mem(0x4F)) == 0) {
                            engine.xor_mem(0x0211, 0x04);
                            engine.xor_mem(0x0215, 0x04);
                        }
                    }
                    engine.set_mem(0x0213, u8v(engine.mem(0x0213) + 0x04));
                    engine.set_mem(0x0217, u8v(engine.mem(0x0213) + 0x08));
                    engine.set_mem(0x1C, u8v(engine.mem(0x1C) - 0x04));
                    r.value = 0xFF;
                    queue_ppu_job_and_wait(engine, r);
                    engine.set_mem(0x0B, u8v(engine.mem(0x0B) - 1));
                    if !cbool((engine.mem(0x0B) & 0x80) == 0) {
                        break;
                    }
                }
                engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
                if !cbool((engine.mem(0x0A) & 0x80) == 0) {
                    break;
                }
            }
            engine.set_mem(0x0016, 0x1E);
            engine.set_mem(0x0017, 0x20);
            engine.set_mem(0x0C, 0x2F);
            farcall_bank_09_r7(engine, r);
            engine.set_mem(0x36, 0);
            r.carry = 1;
            return;
        }
        engine.set_mem(0x1C, 0xFC);
        engine.set_mem(0x1D, 0x01);
        engine.set_mem(0x0213, 0xF0);
        engine.set_mem(0x0217, 0xF8);
        engine.set_mem(0x0A, 0x0F);
        loop {
            engine.set_mem(0x0B, 0x03);
            loop {
                if cbool(engine.mem(0x0B) == 0) {
                    engine.set_mem(0x0213, u8v(engine.mem(0x0213) + 1));
                    engine.set_mem(0x0217, u8v(engine.mem(0x0217) + 1));
                    if cbool((engine.mem(0x4E) | engine.mem(0x4F)) == 0) {
                        engine.xor_mem(0x0211, 0x04);
                        engine.xor_mem(0x0215, 0x04);
                    }
                }
                engine.set_mem(0x0213, u8v(engine.mem(0x0213) - 0x04));
                engine.set_mem(0x0217, u8v(engine.mem(0x0213) + 0x08));
                engine.set_mem(0x1C, u8v(engine.mem(0x1C) + 0x04));
                r.value = 0xFF;
                queue_ppu_job_and_wait(engine, r);
                engine.set_mem(0x0B, u8v(engine.mem(0x0B) - 1));
                if !cbool((engine.mem(0x0B) & 0x80) == 0) {
                    break;
                }
            }
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
            if !cbool((engine.mem(0x0A) & 0x80) == 0) {
                break;
            }
        }
        engine.set_mem(0x0016, 0x00);
        engine.set_mem(0x0017, 0x24);
        engine.set_mem(0x0C, 0x10);
        farcall_bank_09_r7(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }
}

mod project_player_position {
    use super::*;

    /// Copies player position `0x43..0x45` into projection scratch
    /// `0x0E/0x0F/0x0A`, then applies horizontal delta `0x49/0x4A` and vertical
    /// delta `0x4B`.
    pub fn project_player_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0F, engine.mem(0x44));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0x4B) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x4B) + engine.mem(0x0A)));
        }
        let horizontal_subtile_delta: i32 = engine.mem(0x49);
        if cbool(horizontal_subtile_delta != 0) {
            let sum: i32 = u8v(horizontal_subtile_delta + engine.mem(0x0E));
            engine.set_mem(0x0E, u8v(sum & 0x0F));
            let carry: i32 = u8v((sum >> 4) & 1);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + engine.mem(0x4A) + carry));
        }
    }
}

mod update_player_pose_from_motion {
    use super::*;

    /// Updates the player pose byte `0x56` and horizontal flip `0x57` from the
    /// current movement deltas, jump/fall counters, and action lockout.
    pub fn update_player_pose_from_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    x = 0x3D;
                    if cbool(engine.mem(0x46) != 0) {
                        return;
                    }
                    x = 0x09;
                    if cbool(engine.mem(0x50) != 0) {
                        return;
                    }
                    if cbool((engine.mem(0x20) & 0xBF) == 0x80) {
                        return;
                    }
                    a = engine.mem(0x4B);
                    if cbool(a == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a & 0x80) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4E) != 0) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool((engine.mem(0x20) & 0x04) == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    x = 0x0D;
                    engine.set_mem(0x56, x);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0x4F) == 0) {
                        return;
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    x = 0x01;
                    y = 0x00;
                    if cbool(engine.mem(0x4A) & 0x80) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    y = 0x40;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x08, x);
                    engine.set_mem(0x56, (engine.mem(0x56) & 0x07) | engine.mem(0x08));
                    engine.set_mem(0x57, y);
                    return;
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    x = 0x39;
                    y = 0x00;
                    a = engine.mem(0x4A) | engine.mem(0x49);
                    if cbool(a & 0x80) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a != 0) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    x = 0x09;
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    y = 0x40;
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    engine.set_mem(0x08, x);
                    engine.set_mem(0x56, (engine.mem(0x56) & 0x03) | engine.mem(0x08));
                    engine.set_mem(0x57, y);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_player_walk_animation {
    use super::*;

    /// Advances the walking animation every eight movement ticks and folds the
    /// current action/facing button into the pose byte.
    pub fn tick_player_walk_animation(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x46) == 0) {
            if cbool(engine.mem(0x56) < 0x20) {
                if cbool(engine.mem(0x20) & 0x40) {
                    engine.set_mem(0x56, u8v(engine.mem(0x56) | 0x10));
                } else {
                    engine.set_mem(0x56, u8v(engine.mem(0x56) & 0xEF));
                }
            }
        }
        if cbool((engine.mem(0x20) & 0x0F) == 0) {
            return;
        }
        if cbool((engine.mem(0x4F) | engine.mem(0x4E)) != 0) {
            return;
        }
        engine.set_mem(0x4D, u8v(engine.mem(0x4D) + 1));
        if cbool((engine.mem(0x4D) & 0x07) != 0) {
            return;
        }
        if cbool(engine.mem(0x56) & 0x08) {
            engine.set_mem(0x57, u8v(engine.mem(0x57) ^ 0x40));
        } else {
            engine.set_mem(0x56, u8v(engine.mem(0x56) ^ 0x04));
        }
    }
}

mod try_move_player_with_collision {
    use super::*;

    /// Projects a player move, handles room exits/tile actions/object contact,
    /// retries speed-boost nudges, and restores movement deltas before return.
    pub fn try_move_player_with_collision(engine: &mut Engine, r: &mut RoutineContext) {
        let saved_vertical_delta: i32 = engine.mem(0x4B);
        let saved_horizontal_subtile_delta: i32 = engine.mem(0x49);
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut v: i32 = 0;
        let mut state: i32 = 1;
        'dispatch: loop {
            match state {
                1 => {
                    project_player_position(engine, r);
                    check_position_out_of_bounds(engine, r);
                    if cbool(r.carry) {
                        handle_player_room_transition(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 7;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    dispatch_projected_tile_actions(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    routine_0110(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 8;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem(0x08);
                    if cbool(a == 0x09) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a < 0x09) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    x = engine.mem(0x09);
                    r.index = x;
                    v = engine.mem(u16v(0x0401 + x));
                    r.value = v;
                    if cbool(v == 0x01) {
                        unlock_door_with_key(engine, r);
                        {
                            state = 8;
                            continue 'dispatch;
                        }
                    }
                    apply_event_collectible_reward(engine, r);
                    routine_0089(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    x = engine.mem(0x09);
                    r.index = x;
                    v = engine.mem(u16v(0x0401 + x));
                    r.value = v;
                    if cbool(v == 0x01) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(v >= 0x1A) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    collect_room_pickup_object(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    try_trigger_magic_contact_actor(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    r.carry = 0;
                    {
                        state = 8;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    if cbool(engine.mem(0x88) == 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem(0x49);
                    if cbool(a == 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    x = a;
                    if !cbool(a & 0x08) {
                        x = u8v(x - 2);
                    }
                    x = u8v(x + 1);
                    a = u8v(x & 0x0F);
                    engine.set_mem(0x49, a);
                    if cbool(a != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    engine.set_mem(0x49, saved_horizontal_subtile_delta);
                    x = engine.mem(0x4B);
                    if cbool(x == 0) {
                        {
                            state = 7;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(x & 0x80) {
                        x = u8v(x - 2);
                    }
                    x = u8v(x + 1);
                    engine.set_mem(0x4B, x);
                    if cbool(x != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 7;
                    continue 'dispatch;
                }
                7 => {
                    r.carry = 1;
                    state = 8;
                    continue 'dispatch;
                }
                8 => {
                    engine.set_mem(0x49, saved_horizontal_subtile_delta);
                    engine.set_mem(0x4B, saved_vertical_delta);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod try_trigger_magic_contact_actor {
    use super::*;

    /// Marks the contacted actor for its high-bit behavior when the selected
    /// magic-contact timer is active and magic remains.
    pub fn try_trigger_magic_contact_actor(engine: &mut Engine, r: &mut RoutineContext) {
        if (cbool(engine.mem(0x2D) < 0x30)
            && cbool(engine.mem(0x87) != 0)
            && cbool(engine.mem(0x59) != 0))
        {
            let hit_slot: i32 = engine.mem(0x09);
            engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
        }
    }
}

mod apply_event_collectible_reward {
    use super::*;

    /// Applies a collectible reward that came from an event/shop path where no
    /// room object slot needs to be cleared.
    pub fn apply_event_collectible_reward(engine: &mut Engine, r: &mut RoutineContext) {
        let reward_id: i32 = u8v(u8v(r.value - 0x02));
        engine.set_mem(0x04A1, 0x00);
        if cbool(reward_id >= 0x18) {
            engine.set_mem(0x8F, 0x06);
            return;
        }
        if cbool(reward_id < 0x08) {
            const EVENT_REWARD_TEXT: [i32; 8] = [
                0xD16A, 0xD199, 0xDB47, 0xDB52, 0xDB66, 0xDB7B, 0xDBB7, 0xDB9B,
            ];
            engine.set_mem(0x0C, u8v(EVENT_REWARD_TEXT[reward_id as usize] & 0xFF));
            engine.set_mem(0x0D, u8v(EVENT_REWARD_TEXT[reward_id as usize] >> 8));
            r.value = u8v(reward_id << 1);
            r.index = r.value;
            match reward_id {
                0 => {
                    animate_health_refill_to_cap(engine, r);
                }
                1 => {
                    animate_magic_refill_to_cap(engine, r);
                }
                2 => {
                    collect_large_coin_reward(engine, r);
                }
                3 => {
                    trigger_damage_pickup(engine, r);
                }
                4 => {
                    collect_key_bundle_reward(engine, r);
                }
                5 => {
                    grant_long_invulnerability(engine, r);
                }
                6 => {
                    defeat_active_room_actors(engine, r);
                }
                7 => {
                    grant_long_speed_boost(engine, r);
                }
                _ => {}
            }
            return;
        }
        {
            let inventory_item_id: i32 = u8v(reward_id - 0x08);
            if cbool(engine.mem(u16v(0x60 + inventory_item_id)) >= 0x0B) {
                engine.set_mem(0x8F, 0x1D);
                return;
            }
            engine.inc_mem(u16v(0x60 + inventory_item_id));
            engine.set_mem(0x8F, 0x13);
            if cbool(inventory_item_id == 0x0E) {
                routine_0089(engine, r);
                enter_fragment_pickup_room(engine, r);
            }
        }
    }
}

mod collect_room_pickup_object {
    use super::*;

    /// Clears the touched room object slot/OAM entry and applies its reward.
    pub fn collect_room_pickup_object(engine: &mut Engine, r: &mut RoutineContext) {
        let reward_id: i32 = u8v(u8v(r.value - 0x02));
        if cbool(reward_id >= 0x18) {
            return;
        }
        {
            let object_slot_offset: i32 = u8v(r.index);
            engine.set_mem(u16v(0x0401 + object_slot_offset), 0x00);
            engine.set_mem(u16v(0x0406 + object_slot_offset), 0xF0);
        }
        {
            let oam_offset: i32 = u8v((engine.mem(0x08) << 3) | 0x80);
            engine.set_mem(u16v(0x0200 + oam_offset), 0xEF);
            engine.set_mem(u16v(0x0204 + oam_offset), 0xEF);
            r.index = oam_offset;
        }
        if cbool(reward_id < 0x08) {
            const PICKUP_REWARD_TEXT: [i32; 8] = [
                0xDB26, 0xDB31, 0xDB3C, 0xDB52, 0xDB5D, 0xDB71, 0xDBB7, 0xDB85,
            ];
            engine.set_mem(0x0C, u8v(PICKUP_REWARD_TEXT[reward_id as usize] & 0xFF));
            engine.set_mem(0x0D, u8v(PICKUP_REWARD_TEXT[reward_id as usize] >> 8));
            r.value = u8v(reward_id << 1);
            r.index = r.value;
            match reward_id {
                0 => {
                    collect_small_health_reward(engine, r);
                }
                1 => {
                    collect_small_magic_reward(engine, r);
                }
                2 => {
                    collect_small_coin_reward(engine, r);
                }
                3 => {
                    trigger_damage_pickup(engine, r);
                }
                4 => {
                    collect_single_key_reward(engine, r);
                }
                5 => {
                    grant_short_invulnerability(engine, r);
                }
                6 => {
                    defeat_active_room_actors(engine, r);
                }
                7 => {
                    grant_short_speed_boost(engine, r);
                }
                _ => {}
            }
            return;
        }
        {
            let inventory_item_id: i32 = u8v(reward_id - 0x08);
            if cbool(engine.mem(u16v(0x60 + inventory_item_id)) >= 0x0B) {
                engine.set_mem(0x8F, 0x1D);
                return;
            }
            engine.inc_mem(u16v(0x60 + inventory_item_id));
            engine.set_mem(0x8F, 0x13);
            if cbool(inventory_item_id == 0x0E) {
                routine_0089(engine, r);
                enter_fragment_pickup_room(engine, r);
            }
        }
    }
}

mod collect_small_health_reward {
    use super::*;

    /// Adds a small health reward and plays the health pickup sound.
    pub fn collect_small_health_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x1E);
        r.value = 0x05;
        add_health_points(engine, r);
    }
}

mod collect_small_magic_reward {
    use super::*;

    /// Adds a small magic reward.
    pub fn collect_small_magic_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x05;
        add_magic_points(engine, r);
    }
}

mod collect_small_coin_reward {
    use super::*;

    /// Adds the small coin reward.
    pub fn collect_small_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x02;
        add_coins(engine, r);
    }
}

mod collect_large_coin_reward {
    use super::*;

    /// Adds the large coin reward.
    pub fn collect_large_coin_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x32;
        add_coins(engine, r);
    }
}

mod trigger_damage_pickup {
    use super::*;

    /// Applies the harmful pickup/trap effect.
    pub fn trigger_damage_pickup(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x1D);
        r.value = 0x05;
        subtract_health_points(engine, r);
    }
}

mod collect_single_key_reward {
    use super::*;

    /// Adds one key.
    pub fn collect_single_key_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x15);
        add_key(engine, r);
    }
}

mod collect_key_bundle_reward {
    use super::*;

    /// Adds the large key bundle reward.
    pub fn collect_key_bundle_reward(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x15);
        r.value = 0x14;
        add_keys(engine, r);
    }
}

mod grant_short_invulnerability {
    use super::*;

    /// Grants the short invulnerability timer.
    pub fn grant_short_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x13);
        engine.set_mem(0x85, 0x0A);
        r.value = 0x0A;
    }
}

mod grant_long_invulnerability {
    use super::*;

    /// Grants the long invulnerability timer.
    pub fn grant_long_invulnerability(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x13);
        engine.set_mem(0x85, 0x1E);
        r.value = 0x1E;
    }
}

mod grant_short_speed_boost {
    use super::*;

    /// Starts or queues a short speed/action boost timer in `0x88..0x8A`.
    pub fn grant_short_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
        let boost_duration: i32 = 0x1E;
        let mut displaced_timer: i32 = 0;
        engine.set_mem(0x8F, 0x13);
        displaced_timer = engine.mem(0x88);
        if cbool(displaced_timer != 0) {
            displaced_timer = engine.mem(0x89);
            if cbool(displaced_timer != 0) {
                engine.set_mem(0x8A, boost_duration);
            }
            engine.set_mem(0x89, boost_duration);
        }
        engine.set_mem(0x88, boost_duration);
        r.value = displaced_timer;
        r.index = boost_duration;
    }
}

mod grant_long_speed_boost {
    use super::*;

    /// Starts or queues a long speed/action boost timer in `0x88..0x8B`.
    pub fn grant_long_speed_boost(engine: &mut Engine, r: &mut RoutineContext) {
        let boost_duration: i32 = 0x3C;
        let mut displaced_timer: i32 = 0;
        engine.set_mem(0x8F, 0x13);
        displaced_timer = engine.mem(0x88);
        if cbool(displaced_timer != 0) {
            displaced_timer = engine.mem(0x89);
            if cbool(displaced_timer != 0) {
                displaced_timer = engine.mem(0x8A);
                if cbool(displaced_timer != 0) {
                    engine.set_mem(0x8B, boost_duration);
                }
                engine.set_mem(0x8A, boost_duration);
            }
            engine.set_mem(0x89, boost_duration);
        }
        engine.set_mem(0x88, boost_duration);
        r.value = displaced_timer;
        r.index = boost_duration;
    }
}

mod defeat_active_room_actors {
    use super::*;

    /// Marks active room actors as defeated, then runs the palette flash effect.
    pub fn defeat_active_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
        let mut slot_offset: i32 = 0x00;
        for _ in 0..9 {
            if cbool(engine.mem(u16v(0x0401 + slot_offset)) == 0x01) {
                engine.set_mem(u16v(0x0401 + slot_offset), 0x80);
            }
            slot_offset = u8v(slot_offset + 0x10);
        }
        engine.set_mem(0x8F, 0x18);
        engine.set_mem(0x90, 0xFF);
        r.index = 0x02;
        routine_0074(engine, r);
    }
}

mod check_top_boundary_exit_clear {
    use super::*;

    /// Returns carry set when the tile above the top screen edge is empty and
    /// the player can wrap to the room above.
    pub fn check_top_boundary_exit_clear(engine: &mut Engine, r: &mut RoutineContext) {
        if engine.mem(0x86) != 0 || engine.mem(0x4F) != 0 {
            return;
        }
        if engine.mem(0x0E) != 0 {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, 0x00);
        resolve_room_tile_pointer(engine, r);
        let tile_ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let tile = engine.mem(tile_ptr) & 0x3F;
        r.carry = u8v(tile == 0);
    }
}

mod apply_hazard_tile_contact {
    use super::*;

    /// Applies tile `0x30` hazard contact at `tile_ptr + r.offset`, including
    /// the short recoil timer and one-hit invulnerability latch.
    pub fn apply_hazard_tile_contact(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let tile = engine.mem(u16v(tile_ptr + r.offset)) & 0x3F;
        if tile != 0x30 {
            r.carry = 0;
            return;
        }
        if engine.mem(0x4F) == 0 {
            engine.set_mem(0x4F, 0x0A);
        }
        if engine.mem(0x85) == 0 {
            consume_health_point(engine, r);
            engine.set_mem(0x8F, 0x0A);
            engine.set_mem(0x85, 0x01);
        }
        r.carry = 1;
    }
}

mod probe_player_solid_tile {
    use super::*;

    /// Reports whether a player footprint sample collides with terrain.
    /// Empty tiles only count as contact when the player is tile-aligned.
    pub fn probe_player_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let tile = engine.mem(u16v(tile_ptr + r.offset)) & 0x3F;
        if tile == 0 {
            if engine.mem(0x43) == 0 {
                r.carry = 1;
            } else {
                r.carry = 0;
            }
        } else if tile == 0x02 {
            r.carry = 1;
        } else {
            r.carry = u8v(tile >= 0x30);
        }
    }
}

mod dispatch_overhead_tile_action {
    use super::*;

    /// Handles Up-button interactions with the tile directly above the player.
    /// Tile `0x05` and `0x04` jump to their dedicated scripts; tile `0x03`
    /// requires the selected `0x0E` item and all four matching fragments.
    pub fn dispatch_overhead_tile_action(engine: &mut Engine, r: &mut RoutineContext) {
        let player_y = engine.mem(0x45);
        if player_y == 0 {
            return;
        }

        engine.set_mem(0x0D, u8v(player_y - 1));
        engine.set_mem(0x0C, engine.mem(0x44));
        resolve_room_tile_pointer(engine, r);

        let tile_ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        if dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 0x00) {
            return;
        }
        if engine.mem(0x43) != 0 {
            dispatch_overhead_tile_at_offset(engine, r, tile_ptr, 0x0C);
        }
    }

    fn dispatch_overhead_tile_at_offset(
        engine: &mut Engine,
        r: &mut RoutineContext,
        tile_ptr: i32,
        offset: i32,
    ) -> bool {
        r.offset = offset;
        match engine.mem(u16v(tile_ptr + r.offset)) & 0x3F {
            0x05 => {
                routine_0175(engine, r);
                engine.lotw_nonlocal_handoff = 1;
                true
            }
            0x04 => {
                routine_0187(engine, r);
                engine.lotw_nonlocal_handoff = 1;
                true
            }
            0x03 => {
                dispatch_four_fragment_overhead_tile(engine, r);
                true
            }
            _ => false,
        }
    }

    fn dispatch_four_fragment_overhead_tile(engine: &mut Engine, r: &mut RoutineContext) -> bool {
        let selected_slot = engine.mem(0x55);
        if engine.mem(u16v(0x51 + selected_slot)) != 0x0E {
            return false;
        }

        let mut fragment_count = engine.mem(0x6E);
        for slot in 0..=2 {
            if engine.mem(u16v(0x51 + slot)) == 0x0E {
                fragment_count = u8v(fragment_count + 1);
            }
        }
        if fragment_count != 0x04 {
            return false;
        }

        enter_room_link_destination(engine, r);
        engine.lotw_nonlocal_handoff = 1;
        true
    }
}

mod dispatch_projected_tile_actions {
    use super::*;

    /// Checks the projected player footprint for room tile actions. The
    /// original projection scratch is restored before returning so callers can
    /// continue collision resolution with the same candidate position.
    pub fn dispatch_projected_tile_actions(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xE5, 0x90);
        engine.set_mem(0xE6, 0x04);

        let saved_subtile_x = engine.mem(0x0E);
        let saved_tile_x = engine.mem(0x0F);
        let saved_pixel_y = engine.mem(0x0A);

        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, engine.mem(0x0A));
        resolve_room_tile_pointer(engine, r);

        let mut handled = dispatch_projected_tile_action_at_offset(engine, r, 0x00);
        if !handled && engine.mem(0x0E) != 0 {
            handled = dispatch_projected_tile_action_at_offset(engine, r, 0x0C);
        }

        let projected_y = engine.mem(0x0A);
        if !handled && projected_y < 0xB0 && (projected_y & 0x0F) != 0 {
            handled = dispatch_projected_tile_action_at_offset(engine, r, 0x01);
            if !handled && engine.mem(0x0E) != 0 {
                handled = dispatch_projected_tile_action_at_offset(engine, r, 0x0D);
            }
        }

        r.carry = u8v(handled);
        engine.set_mem(0x0A, saved_pixel_y);
        engine.set_mem(0x0F, saved_tile_x);
        engine.set_mem(0x0E, saved_subtile_x);
    }

    fn dispatch_projected_tile_action_at_offset(
        engine: &mut Engine,
        r: &mut RoutineContext,
        offset: i32,
    ) -> bool {
        r.offset = offset;
        dispatch_room_tile_action(engine, r);
        cbool(r.carry)
    }
}

mod seed_object_position_from_tile_offset {
    use super::*;

    /// Converts tile-sample offset `0x0B` plus projected tile coordinates into
    /// object scratch position `0xF9..0xFC`.
    pub fn seed_object_position_from_tile_offset(engine: &mut Engine, r: &mut RoutineContext) {
        let mut tile_offset: i32 = engine.mem(0x0B);
        if cbool(tile_offset >= 0x0C) {
            tile_offset = u8v(tile_offset - 0x0C);
            engine.inc_mem(0x0F);
        }
        if cbool(tile_offset != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) + 0x10));
        }
        engine.set_mem(0xFB, engine.mem(0x0A) & 0xF0);
        engine.set_mem(0xFC, 0x00);
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xF9, 0x00);
        r.value = 0x00;
        r.offset = tile_offset;
    }
}

mod redraw_room_tile_column {
    use super::*;

    /// Rebuilds the background column containing object scratch tile-x `0xFA`.
    pub fn redraw_room_tile_column(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_x: i32 = engine.mem(0xFA);
        engine.set_mem(0x0C, tile_x);
        engine.set_mem(0x16, u8v((tile_x << 1) & 0x1F));
        engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        farcall_bank_09_r7(engine, r);
    }
}

mod read_room_tile_action_value {
    use super::*;

    /// Reads the current room-map tile at `0x10/0x11 + 0x0B`. Tile `0x3E`
    /// resolves to the current room replacement value in `0x74`.
    pub fn read_room_tile_action_value(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_offset: i32 = engine.mem(0x0B);
        let room_ptr: i32 = u16v(engine.mem(0x10) | (engine.mem(0x11) << 8));
        let room_tile: i32 = engine.mem(u16v(room_ptr + tile_offset));
        let tile_id: i32 = room_tile & 0x3F;
        r.index = tile_id;
        r.offset = tile_offset;
        if cbool(tile_id == 0x3E) {
            r.value = engine.mem(0x74);
        } else {
            r.value = room_tile;
        }
    }
}

mod try_nudge_player_to_tile_boundary {
    use super::*;

    /// After a blocked move, attempts a one-pixel/subtile nudge toward the
    /// nearest tile boundary unless the player is pressing away from it.
    pub fn try_nudge_player_to_tile_boundary(engine: &mut Engine, r: &mut RoutineContext) {
        let horizontal_delta: i32 = engine.mem(0x49);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0x49, 0x00);
                    engine.set_mem(0x4A, 0x00);
                    if cbool(horizontal_delta == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut a: i32 = u8v(engine.mem(0x45) & 0x0F);
                        if cbool(a == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a < 0x06) {
                            if cbool(engine.mem(0x20) & 0x04) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x4B, 0xFF);
                            engine.set_mem(0x4C, 0xFF);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a >= 0x0B) {
                            if cbool(engine.mem(0x20) & 0x08) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x4B, 0x01);
                            engine.set_mem(0x4C, 0x00);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    {
                        let mut v4B: i32 = engine.mem(0x4B);
                        engine.set_mem(0x4B, 0x00);
                        engine.set_mem(0x4C, 0x00);
                        if cbool(v4B == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        let mut a: i32 = engine.mem(0x43);
                        if cbool(a == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a < 0x06) {
                            if cbool(engine.mem(0x20) & 0x01) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x49, 0x0F);
                            engine.set_mem(0x4A, 0xFF);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a >= 0x0B) {
                            if cbool(engine.mem(0x20) & 0x02) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x49, 0x01);
                            engine.set_mem(0x4A, 0x00);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    try_move_player_with_collision(engine, r);
                    return;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod close_inventory_item_menu {
    use super::*;

    /// Attempts to close the item menu, restore the pre-menu gameplay snapshot,
    /// and redraw the HUD. Carry from the text/prompt helper aborts the close.
    pub fn close_inventory_item_menu(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, 0x77);
        engine.set_mem(0x0F, 0xB5);
        routine_0052(engine, r);
        if cbool(r.carry) {
            return;
        }
        engine.set_mem(0x8F, 0x10);
        restore_inventory_state_snapshot(engine, r);
        sync_key_hud(engine, r);
        sync_coin_hud(engine, r);
        engine.set_mem(0x7C, 0x20);
        routine_0081(engine, r);
        routine_0060(engine, r);
        routine_0201(engine, r);
    }
}

mod select_inventory_grid_entry {
    use super::*;

    /// Selects the current 7x5 item-grid entry. Values `0x20..0x22` are menu
    /// controls; normal values are copied into the scrolling item-list buffer.
    pub fn select_inventory_grid_entry(engine: &mut Engine, r: &mut RoutineContext) {
        let grid_column: i32 = engine.mem(0xF5);
        let mut grid_value: i32 = u8v(u8v(grid_column << 2) + grid_column);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    grid_value = u8v(grid_value + engine.mem(0xF7));
                    if cbool(grid_value == 0x20) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(grid_value == 0x21) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(grid_value == 0x22) {
                        close_inventory_item_menu(engine, r);
                        return;
                    }
                    r.value = grid_value;
                    set_inventory_list_buffer_index(engine, r);
                    engine.set_mem(u16v(0x0322 + r.index), grid_value);
                    if cbool(r.index == 0x1F) {
                        close_inventory_item_menu(engine, r);
                        return;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.inc_mem(0xF9);
                    update_inventory_list_cursor_sprites(engine, r);
                    return;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.dec_mem(0xF9);
                    update_inventory_list_cursor_sprites(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod move_inventory_cursor_right {
    use super::*;

    /// Moves the inventory grid cursor right across seven columns, wrapping to
    /// column zero.
    pub fn move_inventory_cursor_right(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF5) + 1);
        if cbool(x >= 0x07) {
            x = 0x00;
        }
        engine.set_mem(0xF5, x);
        update_inventory_grid_cursor_sprites(engine, r);
    }
}

mod move_inventory_cursor_left {
    use super::*;

    /// Moves the inventory grid cursor left across seven columns, wrapping to
    /// column six.
    pub fn move_inventory_cursor_left(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF5) - 1);
        if cbool(x & 0x80) {
            x = 0x06;
        }
        engine.set_mem(0xF5, x);
        update_inventory_grid_cursor_sprites(engine, r);
    }
}

mod move_inventory_cursor_up {
    use super::*;

    /// Moves the inventory grid cursor up across five rows, wrapping to row
    /// four.
    pub fn move_inventory_cursor_up(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF7) - 1);
        if cbool(x & 0x80) {
            x = 0x04;
        }
        engine.set_mem(0xF7, x);
        update_inventory_grid_cursor_sprites(engine, r);
    }
}

mod move_inventory_cursor_down {
    use super::*;

    /// Moves the inventory grid cursor down across five rows, wrapping to row
    /// zero.
    pub fn move_inventory_cursor_down(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF7) + 1);
        if cbool(x >= 0x05) {
            x = 0x00;
        }
        engine.set_mem(0xF7, x);
        update_inventory_grid_cursor_sprites(engine, r);
    }
}

mod update_inventory_list_cursor_sprites {
    use super::*;

    /// Positions the two arrow sprites that point at the scrolling selected
    /// item-list slot `0xF9 & 0x1F`.
    pub fn update_inventory_list_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
        let mut list_slot: i32 = engine.mem(0xF9) & 0x1F;
        let mut cursor_tile: i32 = 0x61;
        if cbool(list_slot >= 0x10) {
            list_slot = u8v(list_slot - 0x10);
            cursor_tile = 0x69;
        }
        engine.set_mem(0x0280, cursor_tile);
        engine.set_mem(0x0284, cursor_tile);
        engine.set_mem(0x08, list_slot);

        let scaled_slot: i32 = u8v((list_slot >> 2) + list_slot);
        let carry: i32 = u8v((scaled_slot >> 5) & 1);
        let right_x: i32 = u8v(u8v(scaled_slot << 3) + 0x36 + carry);
        engine.set_mem(0x0287, right_x);
        let left_x: i32 = u8v(right_x - 0x08);
        engine.set_mem(0x0283, left_x);
        r.index = cursor_tile;
        r.value = left_x;
    }
}

mod update_inventory_grid_cursor_sprites {
    use super::*;

    fn scale_grid_coordinate(value: i32) -> (i32, i32) {
        (u8v(value << 3), u8v((value >> 5) & 1))
    }

    /// Positions the 2x2 cursor around the active inventory grid cell.
    pub fn update_inventory_grid_cursor_sprites(engine: &mut Engine, r: &mut RoutineContext) {
        let (column_pixels, column_carry) = scale_grid_coordinate(engine.mem(0xF5));
        let right_x: i32 = u8v(column_pixels + 0x36 + column_carry);
        engine.set_mem(0x0297, right_x);
        let left_x: i32 = u8v(right_x - 0x08);
        engine.set_mem(0x0293, left_x);

        let (row_pixels, row_carry) = scale_grid_coordinate(engine.mem(0xF7));
        let y: i32 = u8v(row_pixels + 0x81 + row_carry);
        engine.set_mem(0x0290, y);
        engine.set_mem(0x0294, y);
        r.value = y;
    }
}

mod set_inventory_list_buffer_index {
    use super::*;

    /// Converts the scrolling item-list cursor into a 32-byte buffer index.
    pub fn set_inventory_list_buffer_index(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = engine.mem(0xF9) & 0x1F;
    }
}

mod routine_0192 {
    use super::*;
    pub fn routine_0192(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0194(engine, r);
        routine_0067(engine, r);
        routine_0200(engine, r);
        r.value = engine.mem(0xFE);
        routine_0123(engine, r);
        routine_0084(engine, r);
        routine_0077(engine, r);
        routine_0061(engine, r);
        routine_0063(engine, r);
        routine_0060(engine, r);
        routine_0070(engine, r);
        update_player_pose_from_motion(engine, r);
        tick_player_walk_animation(engine, r);
    }
}

mod routine_0195 {
    use super::*;
    pub fn routine_0195(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        routine_0067(engine, r);
        engine.set_mem(0x08, a);
        engine.set_mem(0x47, u8v((a & 0x0C) >> 2));
        engine.set_mem(0x7C, u8v((a & 0x03) << 4));
        engine.set_mem(0x44, u8v(engine.mem(0x7C) + 0x07));
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x43, 0x08);
        engine.set_mem(0x45, 0xA0);
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        engine.set_mem(0x7B, 0x00);
        clear_gameplay_object_sprites(engine, r);
        routine_0084(engine, r);
        if cbool(a == 0x04) {
            engine.set_mem(0x7A, u8v(0x1F + 0xA0));
        }
        routine_0078(engine, r);
        update_player_pose_from_motion(engine, r);
        routine_0061(engine, r);
        routine_0060(engine, r);
    }
}

mod routine_0196 {
    use super::*;
    pub fn routine_0196(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        routine_0068(engine, r);
        engine.set_mem(0x08, a);
        engine.set_mem(0x47, u8v((a & 0x0C) >> 2));
        engine.set_mem(0x7C, u8v((a & 0x03) << 4));
        engine.set_mem(0x44, u8v(engine.mem(0x7C) + 0x07));
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x43, 0x08);
        engine.set_mem(0x45, 0xA0);
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        engine.set_mem(0x7B, 0x00);
        clear_gameplay_object_sprites(engine, r);
        routine_0084(engine, r);
        if cbool(a == 0x04) {
            engine.set_mem(0x7A, u8v(0x1F + 0xA0));
        }
        routine_0078(engine, r);
        update_player_pose_from_motion(engine, r);
        routine_0061(engine, r);
        routine_0060(engine, r);
    }
}

mod routine_0197 {
    use super::*;
    pub fn routine_0197(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0x10;
        let mut a: i32 = 0;
        engine.set_mem(0x08, 0x58);
        {
            x = 2;
            while cbool(x >= 0) {
                let mut item: i32 = engine.mem(u16v(0x0051 + x));
                if cbool(item & 0x80) {
                    a = 0xEF;
                } else {
                    let mut t: i32 = u8v((u8v(item << 2)) + 0xA1);
                    engine.set_mem(u16v(0x0241 + y), t);
                    engine.set_mem(u16v(0x0245 + y), u8v(t + 0x02));
                    a = 0xBB;
                }
                engine.set_mem(u16v(0x0240 + y), a);
                engine.set_mem(u16v(0x0244 + y), a);
                engine.set_mem(u16v(0x0243 + y), engine.mem(0x08));
                engine.set_mem(u16v(0x0247 + y), u8v(engine.mem(0x08) + 0x08));
                engine.set_mem(0x08, u8v(u8v(engine.mem(0x08) + 0x08) - 0x28));
                engine.set_mem(u16v(0x0242 + y), 0x01);
                engine.set_mem(u16v(0x0246 + y), 0x01);
                y = u8v(y - 0x08);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.index = 0xFF;
        r.offset = y;
    }
}

mod routine_0198 {
    use super::*;
    pub fn routine_0198(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    a = 0xEF;
                    x = engine.mem(0x80);
                    if cbool(x & 0x80) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x0060 + x)) >= 0x0B) {
                        engine.set_mem(0x80, 0xEF);
                        a = 0xEF;
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    a = u8v(x << 2);
                    a = u8v(a + 0xA1);
                    engine.set_mem(0x0241, a);
                    a = u8v(a + 0x02);
                    engine.set_mem(0x0245, a);
                    engine.set_mem(0x0243, 0x40);
                    engine.set_mem(0x0247, 0x48);
                    a = 0xA4;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x0240, a);
                    engine.set_mem(0x0244, a);
                    engine.set_mem(0x0242, 0x01);
                    engine.set_mem(0x0246, 0x01);
                    a = 0xEF;
                    x = engine.mem(0x82);
                    if cbool(x & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x0060 + x)) >= 0x0B) {
                        engine.set_mem(0x82, 0xEF);
                        a = 0xEF;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    a = u8v(x << 2);
                    a = u8v(a + 0xA1);
                    engine.set_mem(0x0249, a);
                    a = u8v(a + 0x02);
                    engine.set_mem(0x024D, a);
                    engine.set_mem(0x024B, 0xB0);
                    engine.set_mem(0x024F, 0xB8);
                    a = 0xA0;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x0248, a);
                    engine.set_mem(0x024C, a);
                    engine.set_mem(0x024A, 0x01);
                    engine.set_mem(0x024E, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0199 {
    use super::*;
    pub fn routine_0199(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0250, 0x98);
        engine.set_mem(0x0254, 0x98);
        engine.set_mem(0x0251, 0xF1);
        engine.set_mem(0x0255, 0xF3);
        engine.set_mem(0x0252, 0x02);
        engine.set_mem(0x0256, 0x02);
        engine.set_mem(0x0253, 0x78);
        engine.set_mem(0x0257, 0x80);
        r.value = 0x80;
    }
}

mod routine_0200 {
    use super::*;
    pub fn routine_0200(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0240, 0xEF);
        engine.set_mem(0x0244, 0xEF);
        engine.set_mem(0x0248, 0xEF);
        engine.set_mem(0x024C, 0xEF);
        engine.set_mem(0x0250, 0xEF);
        engine.set_mem(0x0254, 0xEF);
        r.value = 0xEF;
    }
}

mod routine_0201 {
    use super::*;
    pub fn routine_0201(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x37;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0280 + x), engine.mem(u16v(0xFF6F + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x2C, 0x34);
        engine.set_mem(0x2D, 0x35);
        engine.set_mem(0x2E, 0x36);
        engine.set_mem(0x2F, 0x37);
        r.index = 0xFF;
        r.value = 0x37;
    }
}

mod consume_health_point {
    use super::*;

    /// Spends one health point, returning carry set when health was already
    /// empty.
    pub fn consume_health_point(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = engine.mem(0x58);
        if cbool(r.value == 0) {
            r.carry = 1;
            return;
        }
        engine.set_mem(0x58, u8v(engine.mem(0x58) - 1));
        sync_health_hud(engine, r);
        r.carry = 0;
    }
}

mod subtract_health_points {
    use super::*;

    /// Subtracts `r.value` health, saturating at zero. Carry is set when the
    /// subtraction did not underflow.
    pub fn subtract_health_points(engine: &mut Engine, r: &mut RoutineContext) {
        let damage: i32 = u8v(r.value);
        engine.set_mem(0x08, damage);
        let health: i32 = engine.mem(0x58);
        let enough_health: i32 = u8v(health >= damage);
        if cbool(enough_health) {
            engine.set_mem(0x58, u8v(health - damage));
        } else {
            engine.set_mem(0x58, 0x00);
        }
        sync_health_hud(engine, r);
        r.carry = enough_health;
    }
}

mod consume_magic_point {
    use super::*;

    /// Spends one magic point and preserves the caller's `r.index`. Carry is
    /// set when no magic was available.
    pub fn consume_magic_point(engine: &mut Engine, r: &mut RoutineContext) {
        let saved_index: i32 = u8v(r.index);
        r.value = engine.mem(0x59);
        r.carry = 1;
        if cbool(engine.mem(0x59) != 0) {
            engine.set_mem(0x59, u8v(engine.mem(0x59) - 1));
            sync_magic_hud(engine, r);
            r.carry = 0;
        }
        r.index = saved_index;
    }
}

mod add_health_points {
    use super::*;

    /// Adds `r.value` health and clamps it to the HUD/resource maximum.
    pub fn add_health_points(engine: &mut Engine, r: &mut RoutineContext) {
        let total: i32 = u8v(u16v(r.value) + engine.mem(0x58));
        let capped_total: i32 = if cbool(total > 0xFF) {
            0x6D
        } else if cbool(u8v(total) >= 0x6E) {
            0x6D
        } else {
            u8v(total)
        };
        engine.set_mem(0x58, capped_total);
        sync_health_hud(engine, r);
    }
}

mod add_magic_points {
    use super::*;

    /// Adds `r.value` magic and clamps it to the HUD/resource maximum.
    pub fn add_magic_points(engine: &mut Engine, r: &mut RoutineContext) {
        let total: i32 = u8v(u16v(r.value) + engine.mem(0x59));
        let capped_total: i32 = if cbool(total > 0xFF) {
            0x6D
        } else if cbool(u8v(total) >= 0x6E) {
            0x6D
        } else {
            u8v(total)
        };
        engine.set_mem(0x59, capped_total);
        sync_magic_hud(engine, r);
    }
}

mod add_coins {
    use super::*;

    /// Adds `r.value` coins and clamps them to the HUD/resource maximum.
    pub fn add_coins(engine: &mut Engine, r: &mut RoutineContext) {
        let total: i32 = u8v(u16v(r.value) + engine.mem(0x5A));
        let capped_total: i32 = if cbool(total > 0xFF) {
            0x6D
        } else if cbool(u8v(total) >= 0x6E) {
            0x6D
        } else {
            u8v(total)
        };
        engine.set_mem(0x5A, capped_total);
        sync_coin_hud(engine, r);
    }
}

mod spend_coins {
    use super::*;

    /// Spends `r.value` coins. Carry is set on success and clear when the
    /// player cannot afford the cost.
    pub fn spend_coins(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x08, r.value);
        let remaining_coins: i32 = u16v(engine.mem(0x5A)) - u16v(engine.mem(0x08));
        r.value = u8v(remaining_coins);
        if cbool(remaining_coins & 0x100) {
            r.carry = 0;
            return;
        }
        engine.set_mem(0x5A, r.value);
        sync_coin_hud(engine, r);
        r.carry = 1;
    }
}

mod add_key {
    use super::*;

    /// Adds one key and refreshes the key HUD digits.
    pub fn add_key(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x5B, u8v(engine.mem(0x5B) + 1));
        sync_key_hud(engine, r);
        r.carry = 0;
    }
}

mod add_keys {
    use super::*;

    /// Adds `r.value` keys and clamps them to the HUD/resource maximum.
    pub fn add_keys(engine: &mut Engine, r: &mut RoutineContext) {
        let total: i32 = u8v(u16v(r.value) + engine.mem(0x5B));
        let capped_total: i32 = if cbool(total > 0xFF) {
            0x6D
        } else if cbool(u8v(total) >= 0x6E) {
            0x6D
        } else {
            u8v(total)
        };
        engine.set_mem(0x5B, capped_total);
        sync_key_hud(engine, r);
    }
}

mod consume_key {
    use super::*;

    /// Spends one key, returning carry set when no key was available.
    pub fn consume_key(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = engine.mem(0x5B);
        if cbool(r.value == 0) {
            r.carry = 1;
            return;
        }
        engine.set_mem(0x5B, u8v(engine.mem(0x5B) - 1));
        sync_key_hud(engine, r);
        r.carry = 0;
    }
}

mod update_room_actors {
    use super::*;
    // Updates live room objects by copying each 16-byte object slot into
    // scratch RAM `0xED..0xFC`, running the correct actor state path, then
    // copying the scratch state back to the slot.
    pub fn update_room_actors(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x48) == 0x10) {
                        return;
                    }
                    if cbool(engine.mem(0x2D) >= 0x30) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut scheduler_phase: i32 = engine.mem(0xE9);
                        let mut first_actor_slot: i32 =
                            u8v((scheduler_phase << 1) + scheduler_phase);
                        engine.set_mem(0xE3, first_actor_slot);
                        engine.set_mem(0xE4, u8v(first_actor_slot + 3));
                        let mut object_slot_lo: i32 = u8v(engine.mem(0xE3) << 4);
                        engine.set_mem(0xE5, object_slot_lo);
                        engine.set_mem(0xE7, u8v(object_slot_lo + 0x20));
                        engine.set_mem(0xE6, 0x04);
                        engine.set_mem(0xE8, engine.mem(0x78));
                    }
                    loop {
                        let mut actor_state: i32 = 0;
                        load_object_slot_scratch(engine, r);
                        actor_state = engine.mem(0xEE);
                        if cbool(actor_state == 0) {
                            tick_inactive_actor_slot(engine, r);
                        } else if cbool(actor_state & 0x80) {
                            routine_0240(engine, r);
                        } else if cbool(actor_state == 0x01) {
                            dispatch_actor_behavior(engine, r);
                        } else if cbool(actor_state >= 0x18) {
                            tick_actor_materialize_delay(engine, r);
                        } else {
                            tick_standard_actor(engine, r);
                        }
                        store_object_slot_scratch(engine, r);
                        engine.inc_mem(0xE3);
                        engine.set_mem(0xE5, u8v(engine.mem(0xE5) + 0x10));
                        engine.set_mem(0xE7, u8v(engine.mem(0xE7) + 0x10));
                        if !cbool(engine.mem(0xE3) < engine.mem(0xE4)) {
                            break;
                        }
                    }
                    {
                        let mut next_scheduler_phase: i32 = u8v(engine.mem(0xE9) + 1);
                        engine.set_mem(
                            0xE9,
                            (if cbool(next_scheduler_phase >= 0x03) {
                                0x00
                            } else {
                                next_scheduler_phase
                            }),
                        );
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xE9) & 0x01) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xE5, 0x00);
                    engine.set_mem(0xE6, 0x04);
                    engine.set_mem(0xE3, 0x00);
                    engine.set_mem(0xE7, 0x20);
                    engine.set_mem(0xE8, engine.mem(0x78));
                    load_object_slot_scratch(engine, r);
                    {
                        let mut actor_state: i32 = engine.mem(0xEE);
                        if cbool(actor_state == 0) {
                            initialize_large_actor_slot(engine, r);
                        } else if cbool(actor_state & 0x80) {
                            update_large_actor_facing_from_velocity(engine, r);
                            animate_large_actor_body_tiles(engine, r);
                        } else {
                            tick_large_chasing_actor(engine, r);
                        }
                    }
                    store_object_slot_scratch(engine, r);
                    compose_large_actor_body_slots(engine, r);
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xE3, 0x04);
                    engine.set_mem(0xE5, 0x40);
                    engine.set_mem(0xE6, 0x04);
                    engine.set_mem(0xE7, 0x60);
                    engine.set_mem(0xE8, engine.mem(0x78));
                    loop {
                        let mut actor_state: i32 = 0;
                        load_object_slot_scratch(engine, r);
                        actor_state = engine.mem(0xEE);
                        if (cbool(actor_state == 0) || cbool(actor_state & 0x80)) {
                            engine.set_mem(0xEE, 0x00);
                            maybe_spawn_pursuer_actor(engine, r);
                        } else {
                            dispatch_actor_behavior(engine, r);
                        }
                        store_object_slot_scratch(engine, r);
                        engine.inc_mem(0xE3);
                        engine.set_mem(0xE5, u8v(engine.mem(0xE5) + 0x10));
                        engine.set_mem(0xE7, u8v(engine.mem(0xE7) + 0x10));
                        if !cbool(engine.mem(0xE3) < 0x09) {
                            break;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.xor_mem(0xE9, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod load_object_slot_scratch {
    use super::*;

    /// Copies the object slot addressed by `0xE5..0xE6` into scratch RAM
    /// `0xED..0xFC`.
    pub fn load_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
        let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
        let mut slot_offset: i32 = 0;
        {
            slot_offset = 0x0F;
            while cbool(slot_offset >= 0) {
                engine.set_mem(
                    u16v(0x00ED + slot_offset),
                    engine.mem(u16v(slot_ptr + slot_offset)),
                );
                {
                    slot_offset -= 1;
                    slot_offset
                };
            }
        }
        r.offset = 0xFF;
    }
}

mod store_object_slot_scratch {
    use super::*;

    /// Writes scratch RAM `0xED..0xFC` back to the object slot addressed by
    /// `0xE5..0xE6`.
    pub fn store_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
        let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
        let mut slot_offset: i32 = 0;
        {
            slot_offset = 0x0F;
            while cbool(slot_offset >= 0) {
                engine.set_mem(
                    u16v(slot_ptr + slot_offset),
                    engine.mem(u16v(0x00ED + slot_offset)),
                );
                {
                    slot_offset -= 1;
                    slot_offset
                };
            }
        }
        r.offset = 0xFF;
    }
}

mod tick_inactive_actor_slot {
    use super::*;
    // Initializes an inactive scratch slot from the room actor record at
    // `0xE7..0xE8`. A nonzero timer leaves the actor materializing; a zero
    // timer promotes it to the normal active state.
    pub fn tick_inactive_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF3, u8v(engine.mem(0xF3) - 1));
        let actor_timer: i32 = engine.mem(0xF3);
        let actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        if cbool((engine.mem(u16v(actor_data_ptr + 2)) | engine.mem(u16v(actor_data_ptr + 3))) == 0)
        {
            r.value = 0x0C;
            rng_update(engine, r);
            engine.set_mem(0x0A, u8v(r.value << 4));
            r.value = 0x40;
            rng_update(engine, r);
            engine.set_mem(0x0F, r.value);
        } else {
            engine.set_mem(0x0A, engine.mem(u16v(actor_data_ptr + 3)));
            engine.set_mem(0x0F, engine.mem(u16v(actor_data_ptr + 2)));
        }
        engine.set_mem(0x0E, 0x00);
        engine.set_mem(0x0B, 0x00);
        check_player_overlap(engine, r);
        if cbool(r.carry) {}
        check_projected_terrain_collision(engine, r);
        if cbool(r.carry) {}
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        engine.set_mem(0xFC, 0x00);
        engine.set_mem(0xF2, engine.mem(u16v(actor_data_ptr + 4)));
        engine.set_mem(0xF8, engine.mem(u16v(actor_data_ptr + 5)));
        {
            let mut current_member_bit: i32 = 0x00;
            let mut carry_bit: i32 = 1;
            let mut member_index: i32 = engine.mem(0x40);
            loop {
                let mut next_carry_bit: i32 = u8v((current_member_bit >> 7) & 1);
                current_member_bit = u8v((current_member_bit << 1) | carry_bit);
                carry_bit = next_carry_bit;
                member_index = u8v(member_index - 1);
                if !cbool((member_index & 0x80) == 0) {
                    break;
                }
            }
            current_member_bit = u8v(current_member_bit & engine.mem(0x41));
            if cbool(current_member_bit == 0) {
                let mut contact_damage: i32 = engine.mem(0xF8);
                let mut damage_overflow: i32 = u8v((contact_damage >> 7) & 1);
                engine.set_mem(0xF8, u8v(contact_damage << 1));
                if cbool(damage_overflow) {
                    engine.set_mem(0xF8, 0xFF);
                }
            }
        }
        engine.set_mem(0xEE, 0x7F);
        engine.set_mem(0xED, 0xF9);
        engine.set_mem(0xEF, 0x01);
        if cbool(actor_timer == 0) {
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0xED, engine.mem(u16v(actor_data_ptr + 0)));
            engine.set_mem(0xEF, engine.mem(u16v(actor_data_ptr + 1)));
        } else {
            if cbool((engine.mem(0xF3) & 0x03) == 0) {
                engine.set_mem(0xEF, u8v(engine.mem(0xEF) ^ 0x40));
            }
        }
    }
}

mod tick_actor_materialize_delay {
    use super::*;
    // Counts down a materializing actor. When the timer reaches zero, the slot
    // becomes behavior-dispatched state `0x01` with sprite bytes from room data.
    pub fn tick_actor_materialize_delay(engine: &mut Engine, r: &mut RoutineContext) {
        let mut actor_timer: i32 = u8v(engine.mem(0xF3) - 1);
        engine.set_mem(0xF3, actor_timer);
        if cbool(actor_timer == 0) {
            let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0xED, engine.mem(actor_data_ptr));
            engine.set_mem(0xEF, engine.mem(u16v(actor_data_ptr + 1)));
        } else if cbool((actor_timer & 0x03) == 0) {
            engine.xor_mem(0xEF, 0x40);
        }
    }
}

mod maybe_spawn_pursuer_actor {
    use super::*;
    // Some late-game rooms periodically seed extra actors from the player slot.
    // The 1-in-30 roll keeps empty secondary slots from respawning every frame.
    pub fn maybe_spawn_pursuer_actor(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x1E;
        rng_update(engine, r);
        if cbool(r.value != 0) {
            r.index = r.value;
            return;
        }
        r.index = 0;
        let mut scratch_offset: i32 = 0x03;
        let mut source_slot_offset: i32 = 0x03;
        if cbool(engine.mem(0x0402) & 0x40) {
            source_slot_offset = 0x13;
        }
        loop {
            engine.set_mem(
                u16v(0x00F9 + scratch_offset),
                engine.mem(u16v(0x040C + source_slot_offset)),
            );
            source_slot_offset = u8v(source_slot_offset - 1);
            if cbool(
                ({
                    let __old = scratch_offset;
                    scratch_offset -= 1;
                    __old
                }) == 0,
            ) {
                break;
            }
        }
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        let actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        engine.set_mem(0xF2, engine.mem(u16v(actor_data_ptr + 4)));
        engine.set_mem(0xF8, engine.mem(u16v(actor_data_ptr + 5)));
        engine.set_mem(0xEE, 0x01);
        engine.set_mem(0xED, 0x81);
        r.value = 0x04;
        rng_update(engine, r);
        engine.set_mem(0xEF, r.value);
        engine.set_mem(0xF1, 0x80);
        r.offset = source_slot_offset;
        r.index = scratch_offset;
    }
}

mod dispatch_actor_behavior {
    use super::*;
    const ACTOR_BEHAVIOR_HANDLERS: [i32; 9] = [
        0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
    ];

    // Dispatches the behavior id stored at room actor data byte 8. The original
    // handler address is mirrored into 0x0E/0x0F for trace-compatible scratch.
    pub fn dispatch_actor_behavior(engine: &mut Engine, r: &mut RoutineContext) {
        let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        let mut behavior_id: i32 = engine.mem(u16v(actor_data_ptr + 8));
        if cbool(behavior_id >= 0x09) {
            behavior_id = 0x00;
        }
        engine.set_mem(
            0x0E,
            u8v(ACTOR_BEHAVIOR_HANDLERS[behavior_id as usize] & 0xFF),
        );
        engine.set_mem(
            0x0F,
            u8v(ACTOR_BEHAVIOR_HANDLERS[behavior_id as usize] >> 8),
        );
        match behavior_id {
            0 => {
                tick_wandering_jump_actor(engine, r);
            }
            1 => {
                tick_random_floating_actor(engine, r);
            }
            2 => {
                tick_ledge_walking_actor(engine, r);
            }
            3 => {
                tick_chasing_jump_actor(engine, r);
            }
            4 => {
                tick_reflecting_chase_actor(engine, r);
            }
            5 => {
                tick_overhead_probe_actor(engine, r);
            }
            6 => {
                tick_contact_trigger_actor(engine, r);
            }
            7 => {
                tick_contact_recoil_actor(engine, r);
            }
            8 => {
                tick_timed_chase_actor(engine, r);
            }
            _ => {}
        }
    }
}

mod tick_standard_actor {
    use super::*;
    // Generic non-boss actor tick: keep existing movement going, try terrain
    // response, expire the actor when its timer reaches zero, then update the
    // terrain probe for the next frame.
    pub fn tick_standard_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut actor_timer: i32 = 0;
        let mut saved_tile_y: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF0) == 0) {
                        if cbool(engine.mem(0xF1) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        try_actor_jump_arc_motion(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        commit_actor_projected_position(engine, r);
                    }
                    try_actor_gravity_motion(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    commit_actor_projected_position(engine, r);
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    actor_timer = u8v(engine.mem(0xF3) - 1);
                    if cbool(actor_timer == 0) {
                        engine.set_mem(0xEE, 0x00);
                        engine.set_mem(0xF3, 0xF0);
                        r.index = actor_timer;
                        return;
                    }
                    engine.set_mem(0xF3, actor_timer);
                    if cbool(actor_timer < 0x3C) {
                        actor_timer = 0xEF;
                        saved_tile_y = engine.mem(0xFB);
                        if cbool(saved_tile_y == 0xEF) {
                            actor_timer = engine.mem(0xFC);
                        }
                        engine.set_mem(0xFB, actor_timer);
                        engine.set_mem(0xFC, saved_tile_y);
                    }
                    update_object_terrain_probe(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_wandering_jump_actor {
    use super::*;
    // Wanders horizontally, occasionally starts a jump arc, then falls under
    // the shared gravity helper until terrain accepts the projected position.
    pub fn tick_wandering_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_tile_dx: i32 = 0;
        let mut keep_existing_motion: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF3) >= 0x20) {
                    } else if cbool(engine.mem(0xF1) != 0) {
                        keep_existing_motion = 1;
                    } else if cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0) {
                        keep_existing_motion = 1;
                    }
                    if !cbool(keep_existing_motion) {
                        engine.set_mem(0xF3, 0x00);
                        choose_random_cardinal_actor_direction(engine, r);
                        r.value = 0x06;
                        rng_update(engine, r);
                        engine.set_mem(0xF6, u8v(r.value + 1));
                        r.value = 0x04;
                        rng_update(engine, r);
                        r.index = r.value;
                        if cbool(r.value == 0) {
                            engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                        }
                    }
                    saved_tile_dx = engine.mem(0xF6);
                    r.offset = engine.mem(0xF6);
                    r.value = engine.mem(0xF4);
                    build_direction_velocity(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        try_actor_gravity_motion(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 4;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    try_actor_jump_arc_motion(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xF1, 0x00);
                    try_move_actor_with_terrain(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    stop_actor_motion(engine, r);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    commit_actor_projected_position(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    update_object_terrain_probe(engine, r);
                    update_actor_animation(engine, r);
                    engine.set_mem(0xF6, saved_tile_dx);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_random_floating_actor {
    use super::*;
    // Chooses a random direction when stationary, then moves without terrain
    // collision. Bounds/player contact can stop the motion.
    pub fn tick_random_floating_actor(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            choose_random_actor_direction(engine, r);
        }
        {
            let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            let mut speed: i32 = engine.mem(u16v(actor_data_ptr + 0x09));
            r.offset = speed;
            r.value = engine.mem(0xF4);
            build_direction_velocity(engine, r);
        }
        try_move_actor_without_terrain(engine, r);
        if cbool(r.carry) {
            stop_actor_motion(engine, r);
        } else {
            commit_actor_projected_position(engine, r);
        }
        update_actor_animation(engine, r);
    }
}

mod tick_ledge_walking_actor {
    use super::*;
    // Walks along terrain ledges: blocked movement stops motion, supported
    // projections commit, and unsupported projections fall through gravity.
    pub fn tick_ledge_walking_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut should_commit_position: i32 = 0;
        let mut should_stop_motion: i32 = 0;
        let mut skip_resolution: i32 = 0;
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            reverse_actor_horizontal_direction(engine, r);
        }
        if cbool(engine.mem(0xF0) != 0) {
            try_actor_gravity_motion(engine, r);
            if cbool(r.carry == 0) {
                should_commit_position = 1;
            } else {
                skip_resolution = 1;
            }
        } else {
            let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(actor_data_ptr + 9));
            r.value = engine.mem(0xF4);
            build_direction_velocity(engine, r);
            try_move_actor_with_terrain(engine, r);
            if cbool(r.carry) {
                should_stop_motion = 1;
            } else {
                r.offset = 0x01;
                probe_object_solid_tile(engine, r);
                if cbool(r.carry == 0) {
                    should_stop_motion = 1;
                } else if cbool(engine.mem(0x0E) == 0) {
                    should_commit_position = 1;
                } else {
                    r.offset = 0x0D;
                    probe_object_solid_tile(engine, r);
                    if cbool(r.carry == 0) {
                        should_stop_motion = 1;
                    } else {
                        should_commit_position = 1;
                    }
                }
            }
        }
        if !cbool(skip_resolution) {
            if cbool(should_stop_motion) {
                stop_actor_motion(engine, r);
            } else if cbool(should_commit_position) {
                commit_actor_projected_position(engine, r);
            }
        }
        update_object_terrain_probe(engine, r);
        update_actor_animation(engine, r);
    }
}

mod tick_chasing_jump_actor {
    use super::*;
    // Re-aims toward the player, marks the direction as jump-capable with
    // `0x80`, then uses the same jump/gravity movement path as wanderers.
    pub fn tick_chasing_jump_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF4, engine.mem(0xF4) & 0x0F);
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0) {
                        if cbool(engine.mem(0xF3) < 0x10) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF9) == 0) {
                        let mut room_tile_ptr: i32 = 0;
                        engine.set_mem(0x0C, engine.mem(0xFA));
                        engine.set_mem(0x0D, engine.mem(0xFB));
                        resolve_room_tile_pointer(engine, r);
                        room_tile_ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                        if cbool((engine.mem(room_tile_ptr) & 0x3F) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool((engine.mem(u16v(room_tile_ptr + 1)) & 0x3F) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    if cbool((engine.mem(0xF4) & 0x03) == 0) {
                        engine.set_mem(0xF4, 0x01);
                    }
                    {
                        let mut turn_timer: i32 = u8v(engine.mem(0xF3) - 1);
                        engine.set_mem(0xF3, 0x00);
                        if cbool(turn_timer == 0) {
                            if cbool((engine.mem(0xF4) & 0x03) == 0) {
                                {
                                    state = 1;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF4, u8v(engine.mem(0xF4) ^ 0x03));
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    aim_actor_toward_player(engine, r);
                    engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xF3, 0x00);
                    aim_actor_toward_player(engine, r);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    {
                        let mut actor_data_ptr: i32 =
                            u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(actor_data_ptr + 0x09));
                    }
                    r.value = engine.mem(0xF4);
                    build_direction_velocity(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        try_actor_gravity_motion(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 6;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    try_actor_jump_arc_motion(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0xF1, 0x00);
                    try_move_actor_with_terrain(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    stop_actor_motion(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    commit_actor_projected_position(engine, r);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    update_object_terrain_probe(engine, r);
                    update_actor_animation(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_reflecting_chase_actor {
    use super::*;
    // Aims from player overlap, moves without terrain, and asks the velocity
    // reflection helper to bounce away when blocked.
    pub fn tick_reflecting_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut keep_current_direction: i32 =
            u8v((cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0)
                && cbool(engine.mem(0xF3) < 0x20)));
        if !cbool(keep_current_direction) {
            aim_actor_from_player_overlap(engine, r);
        }
        {
            let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(actor_data_ptr + 0x09));
            r.value = engine.mem(0xF4);
            build_direction_velocity(engine, r);
        }
        try_move_actor_without_terrain(engine, r);
        if cbool(r.carry) {
            try_reflect_object_velocity(engine, r);
            if cbool(r.carry) {
                stop_actor_motion(engine, r);
                update_actor_animation(engine, r);
                return;
            }
        }
        commit_actor_projected_position(engine, r);
        update_actor_animation(engine, r);
    }
}

mod tick_overhead_probe_actor {
    use super::*;
    // Alternates between overhead probing, falling, and a jump arc. This is the
    // only normal behavior that asks `probe_actor_overhead_step` before moving.
    pub fn tick_overhead_probe_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF0) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x0F, engine.mem(0xFA));
                    engine.set_mem(0x0E, engine.mem(0xF9));
                    engine.set_mem(0x0A, engine.mem(0xFB));
                    probe_actor_overhead_step(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
                    engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
                        reverse_actor_horizontal_direction(engine, r);
                    }
                    check_player_x_overlap(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut actor_data_ptr: i32 =
                            u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(actor_data_ptr + 0x09));
                    }
                    r.value = engine.mem(0xF4);
                    build_direction_velocity(engine, r);
                    try_move_actor_with_terrain(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    probe_actor_overhead_step(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xF5, 0x00);
                    engine.set_mem(0xF6, 0x00);
                    update_object_terrain_probe(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    try_actor_gravity_motion(engine, r);
                    commit_actor_projected_position(engine, r);
                    {
                        let mut saved_fall_counter: i32 = engine.mem(0xF0);
                        update_object_terrain_probe(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 4;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF1, u8v(saved_fall_counter + 0x05 + 1));
                        {
                            state = 7;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    commit_actor_projected_position(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    try_actor_jump_arc_motion(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    commit_actor_projected_position(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    stop_actor_motion(engine, r);
                    state = 7;
                    continue 'dispatch;
                }
                7 => {
                    update_actor_animation(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_contact_trigger_actor {
    use super::*;
    // Sits inert until the player overlaps a one-step projection in any
    // cardinal direction, then switches into the chasing jump behavior.
    pub fn tick_contact_trigger_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        tick_chasing_jump_actor(engine, r);
                        return;
                    }
                    r.value = 0x01;
                    check_actor_direction_contact(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x02;
                    check_actor_direction_contact(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x04;
                    check_actor_direction_contact(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x08;
                    check_actor_direction_contact(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut actor_data_ptr: i32 =
                            u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        let mut actor_type: i32 = engine.mem(u16v(actor_data_ptr + 4));
                        engine.set_mem(0x00F2, actor_type);
                        r.value = 0x00;
                        engine.set_mem(0xFC, 0x00);
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.value = 0x01;
                    engine.set_mem(0xF4, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod check_actor_direction_contact {
    use super::*;
    // Projects the one-step direction in `r.value` and reports player contact.
    pub fn check_actor_direction_contact(engine: &mut Engine, r: &mut RoutineContext) {
        r.offset = 0x01;
        build_direction_velocity(engine, r);
        project_actor_position(engine, r);
        check_player_overlap(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        apply_actor_player_contact_damage(engine, r);
        r.carry = 1;
    }
}

mod tick_contact_recoil_actor {
    use super::*;
    // Random floating behavior that turns into a high-bit/contact recoil state
    // when movement was blocked specifically by player overlap.
    pub fn tick_contact_recoil_actor(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            choose_random_actor_direction(engine, r);
        }
        {
            let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(actor_data_ptr + 0x09));
            r.value = engine.mem(0xF4);
            build_direction_velocity(engine, r);
        }
        try_move_actor_without_terrain(engine, r);
        if cbool(r.carry) {
            if cbool(engine.mem(0xEA) != 0) {
                r.value = 0x80;
                engine.set_mem(0xEE, 0x80);
                return;
            }
            stop_actor_motion(engine, r);
        } else {
            commit_actor_projected_position(engine, r);
        }
        update_actor_animation(engine, r);
    }
}

mod tick_timed_chase_actor {
    use super::*;
    // Chases for `0xF1` ticks. Once it has a direction, abrupt multi-axis
    // changes are rejected unless the timer has settled for several frames.
    pub fn tick_timed_chase_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut chase_timer: i32 = u8v(engine.mem(0xF1) - 1);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF1, chase_timer);
                    if cbool(chase_timer == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF4) == 0) {
                        aim_actor_from_player_overlap(engine, r);
                    } else {
                        if cbool(engine.mem(0xF3) >= 0x08) {
                            let mut direction_diff: i32 = 0;
                            let mut bit_count: i32 = 0;
                            let mut changed_bits: i32 = 0;
                            engine.set_mem(0x08, engine.mem(0xF4));
                            aim_actor_from_player_overlap(engine, r);
                            direction_diff = u8v(engine.mem(0xF4) ^ engine.mem(0x08));
                            changed_bits = 0x00;
                            bit_count = 0x04;
                            loop {
                                let mut bit: i32 = direction_diff & 1;
                                direction_diff >>= 1;
                                if cbool(bit) {
                                    {
                                        let __old = changed_bits;
                                        changed_bits += 1;
                                        __old
                                    };
                                }
                                if !cbool(
                                    {
                                        bit_count -= 1;
                                        bit_count
                                    } != 0,
                                ) {
                                    break;
                                }
                            }
                            {
                                let __old = changed_bits;
                                changed_bits -= 1;
                                __old
                            };
                            if cbool(changed_bits != 0) {
                                engine.set_mem(0xF4, engine.mem(0x08));
                            }
                        }
                    }
                    {
                        let mut actor_data_ptr: i32 =
                            u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(actor_data_ptr + 0x09));
                        r.value = engine.mem(0xF4);
                        build_direction_velocity(engine, r);
                    }
                    try_move_actor_without_terrain(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    commit_actor_projected_position(engine, r);
                    update_actor_animation(engine, r);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.value = 0x00;
                    engine.set_mem(0xEE, 0x00);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod probe_actor_overhead_step {
    use super::*;
    // Probes the projected tile one row above the actor when the projected Y
    // position is tile-aligned. Carry is left from the solid-tile probe.
    pub fn probe_actor_overhead_step(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0x0A) & 0x0F) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, u8v(engine.mem(0x0A) - 0x10));
        resolve_room_tile_pointer(engine, r);
        r.offset = 0x00;
        probe_projected_solid_tile(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        r.offset = 0x0C;
        probe_projected_solid_tile(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
    }
}

mod aim_actor_toward_player {
    use super::*;
    // Sets direction bits in `0xF4` so an actor tends toward the player. Room
    // actor data byte 9 allows occasional upward bias when the actor is below.
    pub fn aim_actor_toward_player(engine: &mut Engine, r: &mut RoutineContext) {
        let mut direction_bits: i32 = 0x00;
        let mut dx: i32 = u16v(u16v(engine.mem(0xFA)) - engine.mem(0x0044));
        if cbool(u8v(dx) != 0) {
            {
                direction_bits += 1;
                direction_bits
            };
            if !cbool(dx & 0x100) {
                {
                    direction_bits += 1;
                    direction_bits
                };
            }
        }
        engine.set_mem(0xF4, direction_bits);
        {
            let mut dy: i32 = u16v(u16v(engine.mem(0xFB)) - engine.mem(0x0045));
            if !cbool(dy & 0x100) {
                let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                let mut vertical_bias_enabled: i32 = engine.mem(u16v(actor_data_ptr + 0x09));
                if cbool(vertical_bias_enabled != 0) {
                    r.value = 0x03;
                    rng_update(engine, r);
                    r.index = r.value;
                    if cbool(r.index == 0) {
                        engine.set_mem(0xF4, u8v(engine.mem(0xF4) | 0x80));
                    }
                }
            } else {
                r.value = 0x03;
                rng_update(engine, r);
                r.index = r.value;
                if cbool(r.index == 0) {
                    engine.set_mem(0xF4, 0x04);
                }
            }
        }
    }
}

mod aim_actor_from_player_overlap {
    use super::*;
    // Builds direction bits by checking whether the actor already overlaps the
    // player on each axis.
    pub fn aim_actor_from_player_overlap(engine: &mut Engine, r: &mut RoutineContext) {
        let mut direction_bits: i32 = 0;
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0A, engine.mem(0xFB));
        check_player_x_overlap(engine, r);
        direction_bits = 0x00;
        if cbool(r.carry == 0) {
            let mut actor_is_right_of_player: i32 =
                u8v((if cbool(engine.mem(0xFA) >= engine.mem(0x44)) {
                    1
                } else {
                    0
                }));
            direction_bits = 0x01;
            if cbool(actor_is_right_of_player) {
                direction_bits = 0x02;
            }
        }
        engine.set_mem(0xF4, direction_bits);
        check_player_y_overlap(engine, r);
        direction_bits = 0x00;
        if cbool(r.carry == 0) {
            let mut actor_is_below_player: i32 =
                u8v((if cbool(engine.mem(0xFB) >= engine.mem(0x45)) {
                    1
                } else {
                    0
                }));
            direction_bits = 0x04;
            if cbool(actor_is_below_player) {
                direction_bits = 0x08;
            }
        }
        engine.set_mem(0xF4, u8v(direction_bits | engine.mem(0xF4)));
        engine.set_mem(0xF3, 0x00);
    }
}

mod reverse_actor_horizontal_direction {
    use super::*;
    pub fn reverse_actor_horizontal_direction(engine: &mut Engine, r: &mut RoutineContext) {
        let mut direction_bits: i32 = engine.mem(0xF4) & 0x03;
        if cbool(direction_bits == 0) {
            direction_bits = 0x01;
        }
        direction_bits ^= 0x03;
        engine.set_mem(0xF4, direction_bits);
        r.value = direction_bits;
    }
}

mod choose_random_actor_direction {
    use super::*;
    // Chooses one of the eight direction-bit patterns in the original table.
    pub fn choose_random_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x08;
        rng_update(engine, r);
        r.index = r.value;
        engine.set_mem(0xF4, engine.mem(u16v(0xEEB3 + r.index)));
    }
}

mod choose_random_cardinal_actor_direction {
    use super::*;
    const DIRECTION_LOOKUP: [i32; 8] = [0x01, 0x05, 0x04, 0x06, 0x02, 0x0A, 0x08, 0x09];

    // Chooses from every other entry in the direction table, giving a smaller
    // cardinal-ish set used by wandering actors.
    pub fn choose_random_cardinal_actor_direction(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x03;
        rng_update(engine, r);
        let direction_index: i32 = u8v(r.value << 1);
        engine.set_mem(0xF4, DIRECTION_LOOKUP[direction_index as usize]);
    }
}

mod try_actor_gravity_motion {
    use super::*;
    // Advances a falling actor. If the projected move is blocked, horizontal
    // velocity is dropped and the move is retried before vertical motion stops.
    pub fn try_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF7, u8v((engine.mem(0xF0) >> 1) + 0x02));
        try_move_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        try_move_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF7, 0x00);
    }
}

mod try_actor_jump_arc_motion {
    use super::*;
    // Uses `0xF1` as a jump-arc countdown and converts it into upward velocity.
    pub fn try_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let mut jump_counter: i32 = engine.mem(0xF1);
        if cbool(jump_counter == 0) {
            jump_counter = 0x0F;
        }
        jump_counter = u8v(jump_counter - 1);
        engine.set_mem(0xF1, jump_counter);
        r.index = jump_counter;
        engine.set_mem(0xF7, u8v(((jump_counter >> 1) ^ 0xFF) + 1));
        try_move_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        try_move_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF1, u8v(engine.mem(0xF1) + 1));
        try_reflect_object_velocity(engine, r);
    }
}

mod commit_actor_projected_position {
    use super::*;
    // Commits projected actor position `0x0E/0x0F/0x0A` back to actor scratch.
    pub fn commit_actor_projected_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        r.value = engine.mem(0x0A);
    }
}

mod stop_actor_motion {
    use super::*;
    // Clears actor velocity and arc/probe counters.
    pub fn stop_actor_motion(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF5, 0);
        engine.set_mem(0xF7, 0);
        engine.set_mem(0xF1, 0);
        engine.set_mem(0xF0, 0);
    }
}

mod project_actor_position {
    use super::*;
    // Projects current actor scratch position `0xF9..0xFB` through velocity
    // `0xF5..0xF7`, leaving the projected position in `0x0E/0x0F/0x0A`.
    pub fn project_actor_position(engine: &mut Engine, r: &mut RoutineContext) {
        let mut subtile_dx: i32 = 0;
        let mut subtile_sum: i32 = 0;
        let mut tile_carry: i32 = 0;
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0A, engine.mem(0xFB));
        if cbool(engine.mem(0xF7) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0xF7) + engine.mem(0x0A)));
        }
        subtile_dx = engine.mem(0xF5);
        if cbool(subtile_dx != 0) {
            subtile_sum = u8v(subtile_dx + engine.mem(0x0E));
            engine.set_mem(0x0E, u8v(subtile_sum & 0x0F));
            tile_carry = u8v((subtile_sum >> 4) & 1);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + engine.mem(0xF6) + tile_carry));
        }
    }
}

mod update_actor_animation {
    use super::*;
    const ANIMATION_HANDLERS: [i32; 4] = [0xF03B, 0xF04B, 0xF071, 0xF0B9];

    // Dispatches the animation mode stored in room actor data byte 7.
    pub fn update_actor_animation(engine: &mut Engine, r: &mut RoutineContext) {
        let mut actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        let mut animation_id: i32 = u8v(engine.mem(u16v(actor_data_ptr + 7)) & 0x03);
        let mut original_handler: i32 = ANIMATION_HANDLERS[animation_id as usize];
        engine.set_mem(0x0E, u8v(original_handler & 0xFF));
        engine.set_mem(0x0F, u8v(original_handler >> 8));
        r.offset = 0x07;
        r.index = u8v(animation_id << 1);
        r.value = u8v(animation_id << 1);
        match animation_id {
            0 => {
                animate_actor_flip_toggle(engine, r);
            }
            1 => {
                animate_actor_walk_toggle(engine, r);
            }
            2 => {
                animate_actor_directional_walk(engine, r);
            }
            3 => {
                animate_actor_cycle_tiles(engine, r);
            }
            _ => {}
        }
    }
}

mod animate_actor_flip_toggle {
    use super::*;
    pub fn animate_actor_flip_toggle(engine: &mut Engine, r: &mut RoutineContext) {
        let mut animation_phase: i32 = 0;
        engine.inc_mem(0xF3);
        animation_phase = engine.mem(0xF3) & 0x03;
        if cbool(animation_phase == 0) {
            animation_phase = engine.mem(0xEF) ^ 0x40;
            engine.set_mem(0xEF, animation_phase);
        }
        r.value = animation_phase;
    }
}

mod animate_actor_walk_toggle {
    use super::*;
    // Faces the actor from horizontal velocity and toggles the sprite tile bit
    // every four animation ticks.
    pub fn animate_actor_walk_toggle(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF5) != 0) {
            let mut facing_bit: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, facing_bit);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | facing_bit));
        }
        engine.inc_mem(0xF3);
        if cbool((engine.mem(0xF3) & 0x03) == 0) {
            engine.xor_mem(0xED, 0x04);
        }
    }
}

mod animate_actor_directional_walk {
    use super::*;
    // Similar to `animate_actor_walk_toggle`, with a separate vertical-motion
    // tile bit so climbing/falling frames differ from horizontal frames.
    pub fn animate_actor_directional_walk(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF5) != 0) {
            let mut facing_bit: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, facing_bit);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | facing_bit));
            engine.set_mem(0xED, u8v(engine.mem(0xED) & 0xF7));
        } else {
            if cbool(engine.mem(0xF7) != 0) {
                engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | 0x08));
            }
        }
        engine.inc_mem(0xF3);
        if cbool((engine.mem(0xF3) & 0x03) == 0) {
            if cbool((engine.mem(0xED) & 0x08) != 0) {
                engine.xor_mem(0xEF, 0x40);
            } else {
                engine.xor_mem(0xED, 0x04);
            }
        }
    }
}

mod animate_actor_cycle_tiles {
    use super::*;
    // Cycles the two sprite-tile animation bits from the frame timer.
    pub fn animate_actor_cycle_tiles(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF5) != 0) {
            let mut facing_bit: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, facing_bit);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | facing_bit));
        }
        engine.inc_mem(0xF3);
        let animation_tile_bits: i32 = u8v((engine.mem(0xF3) & 0x06) << 1);
        engine.set_mem(0x08, animation_tile_bits);
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | animation_tile_bits));
    }
}

mod try_move_actor_with_terrain {
    use super::*;
    // Projects motion, rejects out-of-bounds and solid terrain, applies player
    // contact damage, and restores the original vertical velocity before
    // returning carry set when movement was blocked.
    pub fn try_move_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_vertical_velocity: i32 = engine.mem(0xF7);
        let mut blocked: i32 = 0;
        loop {
            project_actor_position(engine, r);
            check_position_out_of_bounds(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
                engine.set_mem(0xF3, 0xF0);
                blocked = 1;
                break;
            }
            if cbool(u8v(engine.mem(0xEE) - 1) == 0) {
                check_player_overlap(engine, r);
                if cbool(r.carry) {
                    apply_actor_player_contact_damage(engine, r);
                }
            }
            check_projected_terrain_collision(engine, r);
            if cbool(r.carry == 0) {
                blocked = 0;
                break;
            }
            {
                let mut adjusted_vertical_velocity: i32 = engine.mem(0xF7);
                if cbool(adjusted_vertical_velocity == 0) {
                    blocked = 1;
                    break;
                }
                if !cbool(adjusted_vertical_velocity & 0x80) {
                    adjusted_vertical_velocity = u8v(adjusted_vertical_velocity - 2);
                }
                adjusted_vertical_velocity = u8v(adjusted_vertical_velocity + 1);
                engine.set_mem(0xF7, adjusted_vertical_velocity);
                if cbool(adjusted_vertical_velocity == 0) {
                    blocked = 1;
                    break;
                }
            }
        }
        engine.set_mem(0xF7, saved_vertical_velocity);
        r.carry = blocked;
    }
}

mod try_move_actor_without_terrain {
    use super::*;
    // Projects motion for actors that ignore terrain, but still applies player
    // contact and clears the actor if it leaves bounds.
    pub fn try_move_actor_without_terrain(engine: &mut Engine, r: &mut RoutineContext) {
        project_actor_position(engine, r);
        check_player_overlap(engine, r);
        if cbool(r.carry) {
            apply_actor_player_contact_damage(engine, r);
            r.carry = 1;
            return;
        }
        check_position_out_of_bounds(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        engine.set_mem(0xEE, 0x00);
        engine.set_mem(0xF3, 0xF0);
    }
}

mod apply_actor_player_contact_damage {
    use super::*;
    // Applies contact damage unless the player is already invulnerable or a
    // special character/item state suppresses the hit.
    pub fn apply_actor_player_contact_damage(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x85) != 0) {
            return;
        }
        if cbool(u8v(engine.mem(0xEE) - 1) != 0) {
            return;
        }
        if cbool(engine.mem(0x2D) >= 0x30) {
            if cbool(engine.mem(0xE3) != 0) {
                let mut selected_item_slot: i32 = engine.mem(0x55);
                if cbool(engine.mem(u16v(0x0051 + selected_item_slot)) == 0x0A) {
                    engine.set_mem(0x8F, 0x01);
                    return;
                }
            }
        } else {
            if cbool(engine.mem(0x40) == 0x04) {
                return;
            }
        }
        r.value = engine.mem(0xF8);
        subtract_health_points(engine, r);
        engine.set_mem(0x8F, 0x21);
        engine.set_mem(0x90, 0x01);
        engine.set_mem(0x85, 0x01);
        engine.set_mem(0xEF, u8v(engine.mem(0xEF) & 0xDF));
    }
}

mod update_object_terrain_probe {
    use super::*;
    fn mark_probe_clear(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
        r.carry = 0;
    }

    /// Updates the normal one-tile-wide terrain probe for the current object.
    /// When the checked footprint stays clear, the object terrain counter
    /// `0xF0` advances and carry is clear.
    pub fn update_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF1) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0xFA));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        let mut tile_y: i32 = engine.mem(0xFB);
        let active_state: i32 = u8v(engine.mem(0xEE) - 1);
        if cbool(active_state == 0) {
            if cbool(tile_y >= 0xB0) {
                return;
            }
            engine.set_mem(0x0D, tile_y);
            tile_y = u8v(tile_y + 1);
            engine.set_mem(0x0A, tile_y);
            check_player_overlap(engine, r);
            if cbool(r.carry) {
                return;
            }
        } else {
            if cbool(tile_y == 0xEF) {
                tile_y = engine.mem(0xFC);
            }
            engine.set_mem(0x0D, tile_y);
        }
        resolve_room_tile_pointer(engine, r);
        if cbool(engine.mem(0xF9) == 0) {
            let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            if cbool((engine.mem(tile_ptr) & 0x3F) == 0) {
                return;
            }
            if cbool((engine.mem(u16v(tile_ptr + 1)) & 0x3F) == 0) {
                return;
            }
        }
        r.offset = 0x01;
        probe_object_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0xF9) == 0) {
            return;
        }
        r.offset = 0x0D;
        probe_object_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        mark_probe_clear(engine, r);
    }
}

mod update_wide_object_terrain_probe {
    use super::*;

    /// Updates the wider terrain probe used by large objects. It samples the
    /// lower footprint and advances `0xF0` when no solid tile or player overlap
    /// blocks movement.
    pub fn update_wide_object_terrain_probe(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF1) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0xFA));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0D, engine.mem(0xFB));
        engine.set_mem(0x0A, u8v(engine.mem(0xFB) + 1));
        resolve_room_tile_pointer(engine, r);
        if cbool(engine.mem(0xFB) >= 0xA0) {
            engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
            return;
        }
        check_player_overlap_wide(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.offset = 0x02;
        probe_object_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.offset = 0x0E;
        probe_object_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0xF9) != 0) {
            r.offset = 0x1A;
            probe_object_solid_tile(engine, r);
            if cbool(r.carry) {
                return;
            }
        }
        engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
    }
}

mod probe_object_solid_tile {
    use super::*;

    /// Probes the room tile at `current_tile_pointer + r.offset`. Carry is set
    /// when the low six tile bits are in the solid range `>= 0x30`.
    pub fn probe_object_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let tile_id: i32 = u8v(engine.mem(u16v(tile_ptr + r.offset)) & 0x3F);
        r.carry = u8v(u8v(tile_id >= 0x30));
    }
}

mod check_projected_terrain_collision {
    use super::*;

    /// Checks the projected one-tile-wide object footprint in `0x0E..0x0F/0x0A`
    /// against terrain. Carry is clear only when all sampled tiles are clear.
    pub fn check_projected_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, engine.mem(0x0A));
        resolve_room_tile_pointer(engine, r);
        r.offset = 0x00;
        probe_projected_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0x0E) != 0) {
            r.offset = 0x0C;
            probe_projected_solid_tile(engine, r);
            if cbool(r.carry) {
                return;
            }
        }
        if cbool(engine.mem(0x0A) >= 0xB0) {
            return;
        }
        if cbool((engine.mem(0x0A) & 0x0F) == 0) {
            return;
        }
        r.offset = 0x01;
        probe_projected_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        r.offset = 0x0D;
        probe_projected_solid_tile(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.carry = 0;
    }
}

mod check_projected_wide_terrain_collision {
    use super::*;
    fn probe(engine: &mut Engine, r: &mut RoutineContext, tile_offset: i32) -> i32 {
        r.offset = tile_offset;
        probe_projected_solid_tile(engine, r);
        return r.carry;
    }

    /// Checks the projected wide object footprint in `0x0E..0x0F/0x0A` against
    /// terrain. Carry is clear only when every sampled tile is clear.
    pub fn check_projected_wide_terrain_collision(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, engine.mem(0x0A));
        resolve_room_tile_pointer(engine, r);
        if cbool(probe(engine, r, 0x00)) {
            return;
        }
        if cbool(probe(engine, r, 0x01)) {
            return;
        }
        if cbool(probe(engine, r, 0x0C)) {
            return;
        }
        if cbool(probe(engine, r, 0x0D)) {
            return;
        }
        if cbool(engine.mem(0x0E) != 0) {
            if cbool(probe(engine, r, 0x18)) {
                return;
            }
            if cbool(probe(engine, r, 0x19)) {
                return;
            }
        }
        if cbool(engine.mem(0x0A) >= 0xB0) {
            return;
        }
        if cbool((engine.mem(0x0A) & 0x0F) == 0) {
            return;
        }
        if cbool(probe(engine, r, 0x02)) {
            return;
        }
        if cbool(probe(engine, r, 0x0E)) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        if cbool(probe(engine, r, 0x1A)) {
            return;
        }
        r.carry = 0;
    }
}

mod probe_projected_solid_tile {
    use super::*;

    /// Probes a projected footprint tile at `current_tile_pointer + r.offset`.
    /// Carry is set when the low six tile bits are in the solid range.
    pub fn probe_projected_solid_tile(engine: &mut Engine, r: &mut RoutineContext) {
        let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let tile_id: i32 = u8v(engine.mem(u16v(tile_ptr + r.offset)) & 0x3F);
        r.carry = u8v(u8v(tile_id >= 0x30));
    }
}

mod try_reflect_object_velocity {
    use super::*;

    /// Attempts to reflect object velocity away from the nearest subtile edge
    /// and re-run movement validation. Carry remains set if no reflection was
    /// possible.
    pub fn try_reflect_object_velocity(engine: &mut Engine, r: &mut RoutineContext) {
        let mut edge_nibble: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF6, 0x00);
                    if cbool(engine.mem(0xF5) != 0) {
                        engine.set_mem(0xF5, 0x00);
                        edge_nibble = engine.mem(0xFB) & 0x0F;
                        if cbool(edge_nibble == 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(edge_nibble < 0x06) {
                            if cbool(engine.mem(0xF4) & 0x04) {
                                {
                                    state = 2;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF7, 0xFF);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool(edge_nibble >= 0x0B) {
                            if cbool(engine.mem(0xF4) & 0x08) {
                                {
                                    state = 2;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF7, 0x01);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF7) == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF7, 0x00);
                    edge_nibble = engine.mem(0xF9);
                    if cbool(edge_nibble == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(edge_nibble < 0x06) {
                        if cbool(engine.mem(0xF4) & 0x01) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF5, 0x0F);
                        engine.set_mem(0xF6, 0xFF);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(edge_nibble >= 0x0B) {
                        if cbool(engine.mem(0xF4) & 0x02) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF5, 0x01);
                        engine.set_mem(0xF6, 0x00);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    try_move_actor_with_terrain(engine, r);
                    return;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod initialize_large_actor_slot {
    use super::*;

    /// Initializes the special large actor slot from room actor data.
    ///
    /// Large actors use slot `0x0400` as their logical state and slots
    /// `0x0410..0x043F` as linked body pieces. This routine rejects blocked
    /// spawn positions before seeding the logical slot and initial health
    /// state for the body pieces.
    pub fn initialize_large_actor_slot(engine: &mut Engine, r: &mut RoutineContext) {
        let actor_data_ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        engine.set_mem(0x2E, 0x3D);
        engine.set_mem(0x0A, engine.mem(u16v(actor_data_ptr + 3)));
        engine.set_mem(0x0F, engine.mem(u16v(actor_data_ptr + 2)));
        engine.set_mem(0x0E, 0x00);
        engine.set_mem(0x0B, 0x00);
        check_projected_wide_terrain_collision(engine, r);
        if cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        engine.set_mem(0xEE, 0x01);
        engine.set_mem(0xED, 0x81);
        engine.set_mem(0xEF, 0x02);
        engine.set_mem(0xF8, engine.mem(u16v(actor_data_ptr + 5)));
        {
            let actor_health: i32 = engine.mem(u16v(actor_data_ptr + 4));
            engine.set_mem(0xF2, actor_health);
            engine.set_mem(0x0415, actor_health);
            engine.set_mem(0x0425, actor_health);
            engine.set_mem(0x0435, actor_health);
        }
        engine.set_mem(0x0E, 0xE1);
        engine.set_mem(0x0F, 0xA7);
        with_large_actor_asset_banks(engine, r, routine_0017);
        engine.set_mem(0x0E, 0x53);
        engine.set_mem(0x0F, 0xCB);
        with_large_actor_asset_banks(engine, r, build_object_health_meter_alt_tiles);
    }
}

mod tick_large_chasing_actor {
    use super::*;

    /// Updates the active large actor: aim toward the player, run the wide
    /// jump/gravity movement path, then advance facing and animation state.
    pub fn tick_large_chasing_actor(engine: &mut Engine, r: &mut RoutineContext) {
        let mut horizontal_direction: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF4, engine.mem(0xF4) & 0x0F);
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
                        if cbool((engine.mem(0xF4) & 0x03) == 0) {
                            engine.set_mem(0xF4, 0x01);
                        }
                        {
                            let mut turn_timer: i32 = engine.mem(0xF3);
                            engine.set_mem(0xF3, 0x00);
                            turn_timer = u8v(turn_timer - 1);
                            if cbool(turn_timer == 0) {
                                horizontal_direction = engine.mem(0xF4) & 0x03;
                                if cbool(horizontal_direction != 0) {
                                    engine.set_mem(0xF4, u8v(horizontal_direction ^ 0x03));
                                    {
                                        state = 2;
                                        continue 'dispatch;
                                    }
                                }
                                {
                                    state = 1;
                                    continue 'dispatch;
                                }
                            }
                            aim_actor_toward_player(engine, r);
                            engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    } else {
                        if cbool(engine.mem(0xF3) < 0x32) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xF3, 0x00);
                    aim_actor_toward_player(engine, r);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    r.value = engine.mem(0xF4);
                    r.offset = 0x02;
                    build_direction_velocity(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        try_large_actor_gravity_motion(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 6;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    try_large_actor_jump_arc_motion(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0xF1, 0x00);
                    try_move_large_actor_with_terrain(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    stop_actor_motion(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    commit_actor_projected_position(engine, r);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    update_wide_object_terrain_probe(engine, r);
                    update_large_actor_facing_from_velocity(engine, r);
                    animate_large_actor_body_tiles(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod try_large_actor_gravity_motion {
    use super::*;

    /// Applies the large actor's falling motion. If wide movement is blocked,
    /// it retries without horizontal velocity before cancelling vertical speed.
    pub fn try_large_actor_gravity_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let fall_velocity: i32 = u8v((engine.mem(0xF0) >> 2) + 1);
        engine.set_mem(0xF7, fall_velocity);
        r.value = fall_velocity;
        try_move_large_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        r.value = 0x00;
        try_move_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF7, 0x00);
        r.value = 0x00;
    }
}

mod try_large_actor_jump_arc_motion {
    use super::*;

    /// Advances the large actor's jump arc and retries straight-up movement
    /// when horizontal motion collides with terrain.
    pub fn try_large_actor_jump_arc_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let mut jump_counter: i32 = engine.mem(0xF1);
        if cbool(jump_counter == 0) {
            jump_counter = 0x19;
        }
        jump_counter = u8v(jump_counter - 1);
        engine.set_mem(0xF1, jump_counter);
        r.index = jump_counter;
        engine.set_mem(0xF7, u8v(((jump_counter >> 2) ^ 0xFF) + 1));
        try_move_large_actor_with_terrain(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        try_move_large_actor_with_terrain(engine, r);
    }
}

mod try_move_large_actor_with_terrain {
    use super::*;

    /// Projects wide actor motion, applies player contact damage, and rejects
    /// terrain using the three-tile-wide footprint. Carry is set when blocked.
    pub fn try_move_large_actor_with_terrain(engine: &mut Engine, r: &mut RoutineContext) {
        let saved_vertical_velocity: i32 = engine.mem(0xF7);
        let mut blocked: i32 = 0;
        loop {
            project_actor_position(engine, r);
            check_position_out_of_bounds(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
                engine.set_mem(0xF3, 0xF0);
                blocked = 1;
                break;
            }
            check_player_overlap_wide(engine, r);
            if cbool(r.carry) {
                apply_actor_player_contact_damage(engine, r);
            }
            check_projected_wide_terrain_collision(engine, r);
            if cbool(r.carry == 0) {
                blocked = 0;
                break;
            }
            {
                let mut adjusted_vertical_velocity: i32 = engine.mem(0xF7);
                if cbool(adjusted_vertical_velocity == 0) {
                    blocked = 1;
                    break;
                }
                if !cbool(adjusted_vertical_velocity & 0x80) {
                    adjusted_vertical_velocity = u8v(adjusted_vertical_velocity - 2);
                }
                adjusted_vertical_velocity = u8v(adjusted_vertical_velocity + 1);
                engine.set_mem(0xF7, adjusted_vertical_velocity);
                if cbool(adjusted_vertical_velocity == 0) {
                    blocked = 1;
                    break;
                }
            }
        }
        engine.set_mem(0xF7, saved_vertical_velocity);
        r.carry = blocked;
    }
}

mod update_large_actor_facing_from_velocity {
    use super::*;

    /// Updates the large actor's facing bit from horizontal velocity.
    pub fn update_large_actor_facing_from_velocity(engine: &mut Engine, r: &mut RoutineContext) {
        let mut facing_bit: i32 = 0x00;
        if cbool(engine.mem(0xF6) & 0x80) {
        } else if cbool(engine.mem(0xF5) == 0) {
            return;
        } else {
            facing_bit = 0x40;
        }
        engine.set_mem(0x08, facing_bit);
        engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | facing_bit));
    }
}

mod animate_large_actor_body_tiles {
    use super::*;

    /// Advances the large actor's animation timer and stores the base body
    /// tile id for the linked sprite slots.
    pub fn animate_large_actor_body_tiles(engine: &mut Engine, r: &mut RoutineContext) {
        let animation_timer: i32 = engine.inc_mem(0xF3);
        let body_tile_id: i32 = u8v(((animation_timer & 0x0C) << 1) | 0x41);
        engine.set_mem(0xED, body_tile_id);
        r.value = body_tile_id;
    }
}

mod compose_large_actor_body_slots {
    use super::*;
    fn swap_slot_sprite_id(engine: &mut Engine, a: i32, b: i32) {
        let slot_sprite_id: i32 = engine.mem(a);
        engine.set_mem(a, engine.mem(b));
        engine.set_mem(b, slot_sprite_id);
    }

    /// Mirrors the large actor's logical slot into the three linked body slots.
    ///
    /// Slot `0x0400` remains the damage/state anchor. Slots `0x0410`,
    /// `0x0420`, and `0x0430` are arranged as the visible 2x2 body, then their
    /// sprite ids are swapped by facing/flip bits so draw order stays correct.
    pub fn compose_large_actor_body_slots(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x041F, engine.mem(0xFC));
        engine.set_mem(0x042F, engine.mem(0xFC));
        engine.set_mem(0x043F, engine.mem(0xFC));
        {
            let tile_y: i32 = engine.mem(0xFB);
            engine.set_mem(0x041E, tile_y);
            engine.set_mem(0x042E, u8v(tile_y + 0x10));
            engine.set_mem(0x043E, u8v(tile_y + 0x10));
        }
        engine.set_mem(0x041C, engine.mem(0xF9));
        engine.set_mem(0x042C, engine.mem(0xF9));
        engine.set_mem(0x043C, engine.mem(0xF9));
        {
            let tile_x: i32 = engine.mem(0xFA);
            engine.set_mem(0x042D, tile_x);
            engine.set_mem(0x041D, u8v(tile_x + 1));
            engine.set_mem(0x043D, u8v(tile_x + 1));
        }
        {
            let mut actor_state: i32 = engine.mem(0xEE);
            if !cbool(actor_state & 0x80) {
                if cbool((engine.mem(0x0411) | engine.mem(0x0421) | engine.mem(0x0431)) & 0x80) {
                    actor_state = 0x80;
                }
            }
            engine.set_mem(0x0401, actor_state);
            engine.set_mem(0x0411, actor_state);
            engine.set_mem(0x0421, actor_state);
            engine.set_mem(0x0431, actor_state);
        }
        {
            let mut minimum_health: i32 = engine.mem(0xF2);
            if cbool(minimum_health >= engine.mem(0x0415)) {
                minimum_health = engine.mem(0x0415);
            }
            if cbool(minimum_health >= engine.mem(0x0425)) {
                minimum_health = engine.mem(0x0425);
            }
            if cbool(minimum_health >= engine.mem(0x0435)) {
                minimum_health = engine.mem(0x0435);
            }
            engine.set_mem(0x0405, minimum_health);
        }
        {
            let body_tile_id: i32 = engine.mem(0xED);
            let upper_right_tile: i32 = u8v(body_tile_id | 0x04);
            engine.set_mem(0x0410, upper_right_tile);
            let lower_right_tile: i32 = u8v(upper_right_tile | 0x20);
            engine.set_mem(0x0430, lower_right_tile);
            let lower_left_tile: i32 = u8v(lower_right_tile & 0xFB);
            engine.set_mem(0x0420, lower_left_tile);
        }
        {
            let sprite_attrs: i32 = engine.mem(0xEF);
            engine.set_mem(0x0412, sprite_attrs);
            engine.set_mem(0x0422, sprite_attrs);
            engine.set_mem(0x0432, sprite_attrs);
            if cbool(sprite_attrs & 0x40) {
                swap_slot_sprite_id(engine, 0x0400, 0x0410);
                swap_slot_sprite_id(engine, 0x0420, 0x0430);
            }
            if cbool(sprite_attrs & 0x80) {
                swap_slot_sprite_id(engine, 0x0400, 0x0420);
                swap_slot_sprite_id(engine, 0x0410, 0x0430);
            }
        }
        with_large_actor_asset_banks(engine, r, |engine, r| {
            engine.set_mem(0x0E, 0x53);
            engine.set_mem(0x0F, 0xCB);
            build_object_health_meter_alt_tiles(engine, r);
        });
    }
}

mod update_player_projectiles {
    use super::*;

    /// Walks the pooled player projectile slots at `0x04B0` and either updates
    /// an active slot or spawns a new shot on a fire-button edge.
    pub fn update_player_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xE3, 0x0B);
        engine.set_mem(0xE5, 0xB0);
        engine.set_mem(0xE6, 0x04);
        loop {
            let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
            let active_lifetime: i32 = engine.mem(u16v(slot_ptr + 1));
            if cbool(active_lifetime != 0) {
                r.value = active_lifetime;
                r.offset = 0x01;
                update_player_projectile_slot(engine, r);
            } else {
                if cbool(engine.mem(0x20) & 0x40) {
                    if !cbool(engine.mem(0xFD) & 0x40) {
                        r.value = 0x00;
                        r.offset = 0x01;
                        spawn_player_projectile(engine, r);
                    }
                }
            }
            engine.inc_mem(0xE3);
            {
                let next_slot_lo: i32 = u16v(0x10 + engine.mem(0xE5));
                engine.set_mem(0xE5, u8v(next_slot_lo));
                engine.set_mem(0xE6, u8v(engine.mem(0xE6) + (next_slot_lo >> 8)));
            }
            if !cbool(u8v(engine.mem(0xE3) - 0x0B) < engine.mem(0x5E)) {
                break;
            }
        }
    }
}

mod spawn_player_projectile {
    use super::*;

    /// Initializes the current empty projectile slot from the player's facing,
    /// current pose, and resource constraints.
    pub fn spawn_player_projectile(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    load_object_slot_scratch(engine, r);
                    engine.set_mem(0xFD, u8v((engine.mem(0x20) & 0x40) | engine.mem(0xFD)));
                    r.offset = u8v((if cbool(engine.mem(0x88) != 0) {
                        0x04
                    } else {
                        0x02
                    }));
                    r.value = engine.mem(0xFD);
                    build_direction_velocity(engine, r);
                    project_player_projectile_position(engine, r);
                    check_position_out_of_bounds(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    consume_magic_point(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF9, engine.mem(0x0E));
                    engine.set_mem(0xFA, engine.mem(0x0F));
                    engine.set_mem(0xFB, engine.mem(0x0A));
                    load_effective_projectile_lifetime(engine, r);
                    engine.set_mem(0xEE, r.value);
                    if cbool(r.carry == 0) {
                        consume_magic_point(engine, r);
                    }
                    load_effective_projectile_damage(engine, r);
                    engine.set_mem(0xF8, r.value);
                    if cbool(r.carry == 0) {
                        consume_magic_point(engine, r);
                    }
                    engine.set_mem(0xEF, 0x00);
                    engine.set_mem(0xED, 0x21);
                    engine.set_mem(0x8F, u8v(0x22 + engine.mem(0x40)));
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xEE) != 0) {
                        apply_projectile_direction_bits(engine, r);
                    }
                    store_object_slot_scratch(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod update_player_projectile_slot {
    use super::*;

    fn store_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
    }

    fn finish_projectile_slot_update(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xEE) != 0) {
            apply_projectile_direction_bits(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }

    /// Advances one active player projectile, applying terrain collision,
    /// actor hits, damage, and expiry back into the object slot.
    pub fn update_player_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xEE, u8v(engine.mem(0xEE) - 1));
        if cbool(engine.mem(0xEE) == 0) {
            finish_projectile_slot_update(engine, r);
            return;
        }
        project_actor_position(engine, r);
        check_position_out_of_bounds(engine, r);
        if cbool(r.carry) {
            engine.set_mem(0xEE, 0x00);
            finish_projectile_slot_update(engine, r);
            return;
        }
        routine_0109(engine, r);
        if !cbool(r.carry) {
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        if (cbool(engine.mem(0x2D) >= 0x30) && cbool(engine.mem(0x08) >= 0x04)) {
            let hit_slot: i32 = engine.mem(0x09);
            engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0x8F, 0x0C);
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        {
            let mut hit_slot: i32 = engine.mem(0x09);
            if cbool(u8v(engine.mem(u16v(0x0401 + hit_slot)) - 1) != 0) {
                store_projectile_position(engine, r);
                finish_projectile_slot_update(engine, r);
                return;
            }
            hit_slot = engine.mem(0x09);
            {
                let knockback: i32 = (if cbool(engine.mem(0xEE) & 0x01) {
                    0x02
                } else {
                    0xFE
                });
                engine.set_mem(u16v(0x040F + hit_slot), knockback);
            }
            {
                let target_health: i32 = engine.mem(u16v(0x0405 + hit_slot));
                let projectile_damage: i32 = engine.mem(0xF8);
                engine.set_mem(
                    u16v(0x0405 + hit_slot),
                    u8v(target_health - projectile_damage),
                );
                if cbool(target_health >= projectile_damage) {
                    engine.set_mem(0x8F, 0x06);
                } else {
                    engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
                    engine.set_mem(u16v(0x0405 + hit_slot), 0x00);
                }
            }
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
        }
    }
}

mod project_player_projectile_position {
    use super::*;

    /// Projects player position plus projectile velocity into the shared
    /// collision-coordinate scratch registers.
    pub fn project_player_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0F, engine.mem(0x44));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0xF7) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF7) << 2);
            a = u8v(a + engine.mem(0x0A));
            engine.set_mem(0x0A, a);
        }
        if cbool(engine.mem(0xF5) != 0) {
            let projected_subtile: i32 =
                u8v(u8v((engine.mem(0xF5) << 2) & 0x0F) + engine.mem(0x0E));
            engine.set_mem(0x0E, projected_subtile & 0x0F);
            engine.set_mem(
                0x0F,
                u8v(engine.mem(0x0F) + engine.mem(0xF6) + ((projected_subtile >> 4) & 1)),
            );
        }
    }
}

mod apply_projectile_direction_bits {
    use super::*;

    /// Copies the projectile direction bits from its lifetime/state byte into
    /// the sprite/object descriptor used by later render and collision code.
    pub fn apply_projectile_direction_bits(engine: &mut Engine, r: &mut RoutineContext) {
        let direction_bits: i32 = engine.mem(0xEE) & 0x0C;
        engine.set_mem(0x08, direction_bits);
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | direction_bits));
        r.value = engine.mem(0xED);
    }
}

mod update_tile_projectile {
    use super::*;

    /// Updates the singleton tile-removal projectile stored at `0x0490` and
    /// restores its saved background tile when it expires.
    pub fn update_tile_projectile(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0491) == 0) {
            return;
        }
        engine.set_mem(0xE5, 0x90);
        engine.set_mem(0xE6, 0x04);
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xF3, u8v(engine.mem(0xF3) - 1));
        if cbool(engine.mem(0xF3) != 0) {
            update_tile_projectile_motion(engine, r);
            return;
        }
        if cbool((engine.mem(0xED) & 0x01) == 0) {
            if cbool(u8v((engine.mem(0xFB) & 0x0F) | engine.mem(0xF9)) != 0) {
                engine.set_mem(0xF3, u8v(engine.mem(0xF3) + 1));
                update_tile_projectile_motion(engine, r);
                return;
            }
        }
        engine.set_mem(0xEE, 0x00);
        if cbool(engine.mem(0xF0) != 0) {
            engine.set_mem(0x0C, engine.mem(0xFA));
            engine.set_mem(0x0D, engine.mem(0xFB));
            resolve_room_tile_pointer(engine, r);
            let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            engine.set_mem(tile_ptr, engine.mem(0xF0));
            let screen_diff: i32 = u8v(engine.mem(0xFA) - engine.mem(0x7C));
            if (cbool(screen_diff < 0x11) || cbool(screen_diff >= 0xFE)) {
                let tile_x: i32 = engine.mem(0xFA);
                engine.set_mem(0x0C, tile_x);
                engine.set_mem(0x16, u8v((tile_x << 1) & 0x1F));
                engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
                engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
                engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
                farcall_bank_09_r7(engine, r);
            }
        }
        store_object_slot_scratch(engine, r);
    }
}

mod update_tile_projectile_motion {
    use super::*;

    /// Advances the tile-removal projectile, including collision checks,
    /// bouncing, contact damage, and final tile replacement.
    pub fn update_tile_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    {
                        let mut i: i32 = 0;
                        {
                            i = 0x0800;
                            while cbool(i < 0xA000) {
                                engine.set_mem(i, 0);
                                {
                                    i += 1;
                                    i
                                };
                            }
                        }
                    }
                    if cbool(engine.mem(0xED) & 0x01) {
                        if cbool((engine.mem(0xF3) & 0x03) == 0) {
                            engine.xor_mem(0xED, 0x04);
                        }
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xE3, 0x09);
                    project_actor_position(engine, r);
                    check_actor_position_out_of_bounds(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    check_projected_terrain_collision(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    check_player_overlap(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    routine_0109(engine, r);
                    if cbool(r.carry) {
                        let hit_slot: i32 = engine.mem(0x09);
                        engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
                    }
                    engine.set_mem(0xF9, engine.mem(0x0E));
                    engine.set_mem(0xFA, engine.mem(0x0F));
                    engine.set_mem(0xFB, engine.mem(0x0A));
                    engine.set_mem(0xF4, 0x00);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x85) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    consume_health_point(engine, r);
                    engine.set_mem(0x8F, 0x0A);
                    engine.set_mem(0x85, 0x02);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF4, u8v(engine.mem(0xF4) + 1));
                    if cbool(engine.mem(0xF5) != 0) {
                        engine.set_mem(0xF5, u8v((0 - engine.mem(0xF5)) & 0x0F));
                        engine.xor_mem(0xF6, 0xFF);
                    }
                    engine.set_mem(0xF7, u8v(u8v(!engine.mem(0xF7)) + 1));
                    if cbool(engine.mem(0x8F) == 0) {
                        engine.set_mem(0x8F, 0x06);
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    if cbool(u8v((engine.mem(0xFB) & 0x0F) | engine.mem(0xF9)) != 0) {
                        engine.set_mem(0xF3, u8v(engine.mem(0xF3) + 1));
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    {
                        engine.set_mem(0xEE, 0x00);
                        if cbool(engine.mem(0xF0) != 0) {
                            engine.set_mem(0x0C, engine.mem(0xFA));
                            engine.set_mem(0x0D, engine.mem(0xFB));
                            resolve_room_tile_pointer(engine, r);
                            let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                            engine.set_mem(tile_ptr, engine.mem(0xF0));
                            let screen_diff: i32 = u8v(engine.mem(0xFA) - engine.mem(0x7C));
                            if (cbool(screen_diff < 0x11) || cbool(screen_diff >= 0xFE)) {
                                let tile_x: i32 = engine.mem(0xFA);
                                engine.set_mem(0x0C, tile_x);
                                engine.set_mem(0x16, u8v((tile_x << 1) & 0x1F));
                                engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
                                engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
                                engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
                                farcall_bank_09_r7(engine, r);
                            }
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    store_object_slot_scratch(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

// Music channel state is stored in 0x10-byte lanes. The current lane offset
// lives in 0x02: 0x00 pulse 1, 0x10 pulse 2, 0x20 triangle, 0x30 noise, and
// 0x40 for the pulse-2 sound-effect overlay.
mod tick_pulse1_channel {
    use super::*;
    fn silence_pulse1(engine: &mut Engine, _r: &mut RoutineContext) {
        engine.device_write(0x4000, (engine.mem(0x99) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFE));
    }

    pub fn tick_pulse1_channel(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((engine.mem(0x94) & 0x80) == 0) {
                        silence_pulse1(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0x93)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut stream_ptr: i32 = u16v(engine.mem(0x95) | (engine.mem(0x96) << 8));
                        let mut note_byte: i32 = engine.mem(stream_ptr);
                        if cbool(note_byte == 0) {
                            rewind_or_stop_audio_stream(engine, r);
                            silence_pulse1(engine, r);
                            return;
                        }
                        if cbool(note_byte == 0xFF) {
                            dispatch_audio_stream_command(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0x93, u8v(note_byte & 0x7F));
                        if cbool(note_byte & 0x80) {
                            start_rest_envelope(engine, r);
                        } else {
                            load_note_period(engine, r);
                            engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x01));
                            engine.device_write(0x4001, engine.mem(0x9A));
                            engine.device_write(0x4002, engine.mem(0x04));
                            engine.device_write(0x4003, (engine.mem(0x05) & 0x07) | 0x18);
                            start_note_envelope(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x01) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0x9D)) == 0) {
                        next_envelope_volume(engine, r);
                        engine.device_write(0x4000, r.value);
                    }
                    advance_envelope_phase(engine, r);
                    if cbool(r.carry) {
                        silence_pulse1(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_pulse2_channel {
    use super::*;
    fn silence_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
        engine.device_write(0x4004, (engine.mem(0xA9) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFD));
    }

    pub fn tick_pulse2_channel(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_flags: i32 = engine.mem(0xA4);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((channel_flags & 0x80) == 0) {
                        if cbool(channel_flags & 0x40) {
                            return;
                        }
                        silence_pulse2(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xA3)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut stream_ptr: i32 = u16v(engine.mem(0xA5) | (engine.mem(0xA6) << 8));
                        let mut note_byte: i32 = engine.mem(stream_ptr);
                        if cbool(note_byte == 0) {
                            rewind_or_stop_audio_stream(engine, r);
                            silence_pulse2(engine, r);
                            return;
                        }
                        if cbool(note_byte == 0xFF) {
                            dispatch_audio_stream_command(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xA3, u8v(note_byte & 0x7F));
                        if cbool(note_byte & 0x80) {
                            if cbool(engine.mem(0xA4) & 0x40) {
                                return;
                            }
                            start_rest_envelope(engine, r);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool(engine.mem(0xA4) & 0x40) {
                            inc16_95(engine, r);
                            return;
                        }
                        load_note_period(engine, r);
                        engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x02));
                        engine.device_write(0x4004, engine.mem(0xA9));
                        engine.device_write(0x4005, engine.mem(0xAA));
                        engine.device_write(0x4006, engine.mem(0x04));
                        engine.device_write(0x4007, (engine.mem(0x05) & 0x07) | 0x18);
                        start_note_envelope(engine, r);
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xA4) & 0x40) {
                        return;
                    }
                    if cbool((engine.mem(0x27) & 0x02) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xAD)) == 0) {
                        next_envelope_volume(engine, r);
                        engine.device_write(0x4004, r.value);
                    }
                    advance_envelope_phase(engine, r);
                    if cbool(r.carry) {
                        silence_pulse2(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod tick_triangle_channel {
    use super::*;
    fn silence_triangle(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x00;
        engine.device_write((0x4008), 0x00);
        engine.set_mem(0x27, engine.mem(0x27) & 0xFB);
        r.value = engine.mem(0x27);
    }

    pub fn tick_triangle_channel(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xB4) & 0x80) == 0) {
            silence_triangle(engine, r);
            return;
        }
        if cbool(u8v(engine.mem(0xB3) - 1) != 0) {
            engine.set_mem(0xB3, u8v(engine.mem(0xB3) - 1));
            return;
        }
        engine.set_mem(0xB3, u8v(engine.mem(0xB3) - 1));
        loop {
            let mut stream_ptr: i32 = u16v(engine.mem(0xB5) | (engine.mem(0xB6) << 8));
            let mut note_byte: i32 = engine.mem(stream_ptr);
            if cbool(note_byte == 0) {
                rewind_or_stop_audio_stream(engine, r);
                silence_triangle(engine, r);
                return;
            }
            if cbool(note_byte != 0xFF) {
                let mut is_rest: i32 = u8v(note_byte & 0x80);
                r.value = note_byte;
                inc16_95(engine, r);
                r.value = u8v(note_byte & 0x7F);
                engine.set_mem(0xB3, r.value);
                if cbool(is_rest) {
                    silence_triangle(engine, r);
                    return;
                }
                load_note_period(engine, r);
                engine.set_mem(0x27, engine.mem(0x27) | 0x04);
                engine.device_write((0x4008), engine.mem(0xBA));
                engine.device_write((0x400A), engine.mem(0x04));
                r.value = u8v((engine.mem(0x05) & 0x07) | 0xF8);
                engine.device_write((0x400B), r.value);
                return;
            }
            dispatch_audio_stream_command(engine, r);
        }
    }
}

mod tick_noise_channel {
    use super::*;
    fn silence_noise(engine: &mut Engine, _r: &mut RoutineContext) {
        engine.device_write(0x400C, 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xF7));
    }

    pub fn tick_noise_channel(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((engine.mem(0xC4) & 0x80) == 0) {
                        silence_noise(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xC3)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut stream_ptr: i32 = u16v(engine.mem(0xC5) | (engine.mem(0xC6) << 8));
                        let mut note_byte: i32 = engine.mem(stream_ptr);
                        if cbool(note_byte == 0) {
                            rewind_or_stop_audio_stream(engine, r);
                            silence_noise(engine, r);
                            return;
                        }
                        if cbool(note_byte == 0xFF) {
                            dispatch_audio_stream_command(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xC3, u8v(note_byte & 0x7F));
                        if cbool(note_byte & 0x80) {
                            start_rest_envelope(engine, r);
                        } else {
                            engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x08));
                            engine.device_write(0x400E, engine.mem(0xCA));
                            engine.device_write(0x400F, 0x80);
                            start_note_envelope(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x08) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xCD)) == 0) {
                        next_envelope_volume(engine, r);
                        engine.device_write(0x400C, r.value);
                    }
                    advance_envelope_phase(engine, r);
                    if cbool(r.carry) {
                        silence_noise(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod dispatch_audio_stream_command {
    use super::*;
    fn deref_stream(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
        let mut channel_offset: i32 = u8v(r.index);
        let mut lo: i32 = engine.mem((0x95 + channel_offset) & 0xFF);
        let mut hi: i32 = engine.mem((0x96 + channel_offset) & 0xFF);
        return engine.mem(u16v(lo | (hi << 8)));
    }

    // A 0xFF stream byte is followed by command id and value bytes. The command
    // updates per-channel playback state, then leaves the stream pointer at the
    // next note/rest/control byte.
    pub fn dispatch_audio_stream_command(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = engine.mem(0x02);
        inc16_95(engine, r);
        {
            let __v = deref_stream(engine, r);
            engine.set_mem(0x04, __v);
        }
        inc16_95(engine, r);
        {
            let __v = deref_stream(engine, r);
            engine.set_mem(0x05, __v);
        }
        inc16_95(engine, r);
        let mut command_id: i32 = engine.mem(0x04);
        if cbool(command_id >= 0x05) {
            return;
        }
        const ORIGINAL_COMMAND_HANDLERS: [i32; 5] = [0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05];
        let mut original_handler: i32 = ORIGINAL_COMMAND_HANDLERS[command_id as usize];
        engine.set_mem(0x06, u8v(original_handler & 0xFF));
        engine.set_mem(0x07, u8v(original_handler >> 8));
        r.value = engine.mem(0x05);
        r.index = engine.mem(0x02);
        match command_id {
            0 => {
                audio_cmd_set_duty_instrument(engine, r);
            }
            1 => {
                audio_cmd_set_volume_scale(engine, r);
            }
            2 => {
                audio_cmd_set_channel_flags(engine, r);
            }
            3 => {
                audio_cmd_set_pitch_offset(engine, r);
            }
            4 => {
                audio_cmd_set_sweep_value(engine, r);
            }
            _ => {}
        }
    }
}

mod audio_cmd_set_duty_instrument {
    use super::*;
    // Command 0 packs pulse duty in the high nibble and envelope table choice
    // in the low nibble. The low nibble is expanded to a 16-byte table offset.
    pub fn audio_cmd_set_duty_instrument(engine: &mut Engine, r: &mut RoutineContext) {
        let mut command_value: i32 = u8v(r.value);
        let mut channel_offset: i32 = u8v(r.index);
        let mut duty_bits: i32 = u8v(u8v(command_value & 0xF0) << 2);
        engine.set_mem(0x00, duty_bits);
        engine.set_mem(
            (0x99 + channel_offset) & 0xFF,
            u8v((engine.mem((0x99 + channel_offset) & 0xFF) & 0x3F) | duty_bits),
        );
        let mut envelope_offset: i32 = u8v(command_value << 4);
        engine.set_mem((0xA2 + channel_offset) & 0xFF, envelope_offset);
        engine.set_mem(
            (0x9A + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDD2 + envelope_offset)),
        );
        r.value = engine.mem(u16v(0xFDD2 + envelope_offset));
        r.offset = envelope_offset;
        r.index = channel_offset;
    }
}

mod audio_cmd_set_volume_scale {
    use super::*;
    // Command 1 stores the per-channel multiplier used after the envelope's raw
    // 0..15 volume accumulator is updated.
    pub fn audio_cmd_set_volume_scale(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = u8v(r.index);
        if !cbool(engine.mem(0x02) == 0x40) {
            let mut music_volume_override: i32 = engine.mem(0x92);
            if cbool(music_volume_override != 0) {
                r.value = music_volume_override;
                r.index = channel_offset;
                return;
            }
        }
        {
            let mut adjusted_command: i32 = u8v(0x0F + engine.mem(0x05));
            let mut scale: i32 = if cbool(adjusted_command >= 0x08) {
                u8v(adjusted_command - 0x08)
            } else {
                0x00
            };
            scale = u8v(scale << 1);
            scale = u8v(scale + 1);
            engine.set_mem((0xA0 + channel_offset) & 0xFF, scale);
            r.value = scale;
        }
        r.index = channel_offset;
    }
}

mod audio_cmd_set_channel_flags {
    use super::*;
    // Command 2 replaces the channel flag/register shadow byte at 0x99+x.
    pub fn audio_cmd_set_channel_flags(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0x99 + r.index) & 0xFF, r.value);
    }
}

mod audio_cmd_set_pitch_offset {
    use super::*;
    // Command 3 stores a fine pitch offset subtracted from the period table.
    pub fn audio_cmd_set_pitch_offset(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0xA1 + r.index) & 0xFF, r.value);
    }
}

mod audio_cmd_set_sweep_value {
    use super::*;
    // Command 4 replaces the square-channel sweep/noise-period shadow byte.
    pub fn audio_cmd_set_sweep_value(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0x9A + r.index) & 0xFF, r.value);
    }
}

mod load_note_period {
    use super::*;
    // Note bytes use the low nibble as the pitch-table index and the high
    // nibble as the octave shift. The resulting period lands in 0x04/0x05.
    pub fn load_note_period(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut stream_ptr: i32 = u16v(
            engine.mem(u8v(0x95 + channel_offset)) | (engine.mem(u8v(0x96 + channel_offset)) << 8),
        );
        let mut note_byte: i32 = engine.mem(stream_ptr);
        inc16_95(engine, r);
        {
            let mut pitch_index: i32 = u8v((note_byte & 0x0F) << 1);
            let mut lo: i32 = engine.mem(u16v(0xFDB1 + pitch_index));
            let mut hi: i32 = engine.mem(u16v(0xFDB2 + pitch_index));
            channel_offset = engine.mem(0x02);
            {
                let mut sub: i32 = u16v(u16v(lo) - engine.mem(u8v(0xA1 + channel_offset)));
                lo = u8v(sub);
                if cbool(sub & 0x100) {
                    hi = u8v(hi - 1);
                }
            }
            {
                let mut octave_shift_count: i32 = u8v(note_byte >> 4);
                while cbool(octave_shift_count != 0) {
                    let mut carry_from_hi: i32 = u8v(hi & 1);
                    hi = u8v(hi >> 1);
                    lo = u8v((lo >> 1) | (carry_from_hi << 7));
                    {
                        octave_shift_count -= 1;
                        octave_shift_count
                    };
                }
            }
            engine.set_mem(0x04, lo);
            engine.set_mem(0x05, hi);
        }
    }
}

mod scale_envelope_volume {
    use super::*;
    // Multiply the raw envelope accumulator in 0x00 by r.offset+1, then divide
    // by 16 to return the APU's 4-bit volume value.
    pub fn scale_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
        let mut scaled_volume: i32 = 0x00;
        let mut multiplier: i32 = u8v(u8v(r.offset + 1));
        loop {
            scaled_volume = u8v(scaled_volume + engine.mem(0x00));
            multiplier = u8v(multiplier - 1);
            if !cbool(multiplier != 0) {
                break;
            }
        }
        scaled_volume >>= 4;
        engine.set_mem(0x00, scaled_volume);
        r.value = scaled_volume;
        r.offset = 0;
    }
}

mod start_note_envelope {
    use super::*;
    // Load the first active-note envelope phase into the selected channel lane.
    pub fn start_note_envelope(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut envelope_offset: i32 = engine.mem((0xA2 + channel_offset) & 0xFF);
        engine.set_mem((0x9B + channel_offset) & 0xFF, envelope_offset);
        engine.set_mem(
            (0x9C + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCB + envelope_offset)),
        );
        engine.set_mem(
            (0x9D + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCC + envelope_offset)),
        );
        engine.set_mem(
            (0x9E + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCD + envelope_offset)),
        );
        engine.set_mem(
            (0x9F + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCE + envelope_offset)),
        );
        r.index = channel_offset;
        r.offset = envelope_offset;
    }
}

mod start_rest_envelope {
    use super::*;
    // Rest bytes reuse the same envelope table with a +0x0C offset, which
    // gives the ticker a timed silent phase instead of an audible period.
    pub fn start_rest_envelope(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut rest_envelope_offset: i32 = u8v(engine.mem((0xA2 + channel_offset) & 0xFF) + 0x0C);
        engine.set_mem((0x9B + channel_offset) & 0xFF, rest_envelope_offset);
        engine.set_mem(
            (0x9C + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCB + rest_envelope_offset)),
        );
        engine.set_mem(
            (0x9D + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCC + rest_envelope_offset)),
        );
        engine.set_mem(
            (0x9E + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCD + rest_envelope_offset)),
        );
        r.index = channel_offset;
        r.offset = rest_envelope_offset;
        r.value = engine.mem(u16v(0xFDCD + rest_envelope_offset));
    }
}

mod rewind_or_stop_audio_stream {
    use super::*;
    // A zero stream byte jumps to the saved loop pointer when one exists; a
    // missing loop pointer clears the active bit while preserving sfx overlay.
    pub fn rewind_or_stop_audio_stream(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut loop_pointer_hi: i32 = 0;
        engine.set_mem(
            (0x95 + channel_offset) & 0xFF,
            engine.mem((0x97 + channel_offset) & 0xFF),
        );
        loop_pointer_hi = engine.mem((0x98 + channel_offset) & 0xFF);
        engine.set_mem((0x96 + channel_offset) & 0xFF, loop_pointer_hi);
        if cbool(loop_pointer_hi != 0) {
            engine.set_mem((0x93 + channel_offset) & 0xFF, 0x01);
        } else {
            engine.and_mem((0x94 + channel_offset) & 0xFF, 0x40);
        }
        r.index = channel_offset;
    }
}

mod next_envelope_volume {
    use super::*;
    // Update the current envelope accumulator and compose the APU volume
    // register value from channel flags, constant-volume bit, and scaled volume.
    pub fn next_envelope_volume(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut envelope_phase: i32 = engine.mem(u8v(0x9B + channel_offset));
        engine.set_mem(
            u8v(0x9D + channel_offset),
            engine.mem(u16v(0xFDCC + envelope_phase)),
        );
        {
            let mut envelope_delta: i32 = engine.mem(u8v(0x9C + channel_offset));
            let mut accumulator: i32 = u8v(envelope_delta + engine.mem(u8v(0x9F + channel_offset)));
            if cbool(envelope_delta & 0x80) {
                if cbool(accumulator >= 0x10) {
                    accumulator = 0x00;
                }
            } else {
                if cbool(accumulator >= 0x10) {
                    accumulator = 0x0F;
                }
            }
            engine.set_mem(u8v(0x9F + channel_offset), accumulator);
            engine.set_mem(0x00, accumulator);
        }
        r.offset = engine.mem(u8v(0xA0 + channel_offset));
        scale_envelope_volume(engine, r);
        {
            let mut volume_register: i32 =
                u8v((engine.mem(u8v(0x99 + channel_offset)) & 0xC0) | engine.mem(0x00) | 0x30);
            r.value = volume_register;
        }
    }
}

mod advance_envelope_phase {
    use super::*;
    // Tick the phase duration. When it expires, advance four bytes in the
    // envelope table; low nibbles >= 0x0C mark the terminal silent phase.
    pub fn advance_envelope_phase(engine: &mut Engine, r: &mut RoutineContext) {
        let mut channel_offset: i32 = engine.mem(0x02);
        let mut phase_low_nibble: i32 = 0;
        let mut next_phase: i32 = 0;
        if cbool(engine.dec_mem((0x9E + channel_offset) & 0xFF) != 0) {
            r.index = channel_offset;
            r.carry = 0;
            return;
        }
        phase_low_nibble = engine.mem((0x9B + channel_offset) & 0xFF) & 0x0F;
        if cbool(phase_low_nibble >= 0x0C) {
            r.index = channel_offset;
            r.value = phase_low_nibble;
            r.carry = 1;
            return;
        }
        next_phase = u8v(engine.mem((0x9B + channel_offset) & 0xFF) + 0x04);
        engine.set_mem((0x9B + channel_offset) & 0xFF, next_phase);
        engine.set_mem(
            (0x9C + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCB + next_phase)),
        );
        engine.set_mem(
            (0x9D + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCC + next_phase)),
        );
        engine.set_mem(
            (0x9E + channel_offset) & 0xFF,
            engine.mem(u16v(0xFDCD + next_phase)),
        );
        r.index = channel_offset;
        r.offset = next_phase;
        r.carry = 0;
    }
}

mod scene_assemble {
    use super::*;
    pub fn scene_assemble(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0086(engine, r);
        routine_0085(engine, r);
        r.carry = u8v(u8v((if cbool((engine.mem(0x76) + 0x03) > 0xFF) {
            1
        } else {
            0
        })));
        text_attr_build(engine, r);
        routine_0087(engine, r);
    }
}

mod sfx_overlay_voice {
    use super::*;
    fn silence_sfx_pulse2(engine: &mut Engine, _r: &mut RoutineContext) {
        engine.device_write(0x4004, (engine.mem(0xD9) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFD));
    }

    pub fn sfx_overlay_voice(engine: &mut Engine, r: &mut RoutineContext) {
        let mut start: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x8F) != 0) {
                        if cbool((engine.mem(0xD4) & 0x80) == 0) {
                            start = 1;
                        } else if cbool(engine.mem(0x90) >= engine.mem(0x91)) {
                            start = 1;
                        } else {
                            engine.set_mem(0x90, 0x00);
                            engine.set_mem(0x8F, 0x00);
                        }
                    }
                    if !cbool(start) {
                        if cbool((engine.mem(0xD4) & 0x80) == 0) {
                            return;
                        }
                        if cbool(u8v(engine.dec_mem(0xD3)) != 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    } else {
                        let mut sfx_table_index: i32 = 0;
                        engine.set_mem(0x91, engine.mem(0x90));
                        sfx_table_index = u8v(engine.mem(0x8F) << 1);
                        engine.set_mem(0xD5, engine.mem(u16v(0x8014 + sfx_table_index)));
                        engine.set_mem(0xD6, engine.mem(u16v(0x8015 + sfx_table_index)));
                        engine.set_mem(0xD4, 0x80);
                        engine.set_mem(0xA4, u8v(engine.mem(0xA4) | 0x40));
                        engine.set_mem(0x8F, 0x00);
                        engine.set_mem(0x90, 0x00);
                    }
                    loop {
                        let mut stream_ptr: i32 = u16v(engine.mem(0xD5) | (engine.mem(0xD6) << 8));
                        let mut note_byte: i32 = engine.mem(stream_ptr);
                        if cbool(note_byte == 0) {
                            engine.set_mem(0xD4, 0x00);
                            engine.set_mem(0x91, 0x00);
                            engine.set_mem(0xA4, u8v(engine.mem(0xA4) & 0xBF));
                            silence_sfx_pulse2(engine, r);
                            return;
                        }
                        if cbool(note_byte == 0xFF) {
                            dispatch_audio_stream_command(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xD3, u8v(note_byte & 0x7F));
                        if cbool(note_byte & 0x80) {
                            start_rest_envelope(engine, r);
                        } else {
                            load_note_period(engine, r);
                            engine.set_mem(0x27, u8v(0x02 | engine.mem(0x27)));
                            engine.device_write(0x4005, engine.mem(0xDA));
                            engine.device_write(0x4006, engine.mem(0x04));
                            engine.device_write(0x4007, (engine.mem(0x05) & 0x07) | 0xC0);
                            start_note_envelope(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x02) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xDD)) == 0) {
                        next_envelope_volume(engine, r);
                        engine.device_write(0x4004, r.value);
                    }
                    advance_envelope_phase(engine, r);
                    if cbool(r.carry) {
                        silence_sfx_pulse2(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod song_init {
    use super::*;
    pub fn song_init(engine: &mut Engine, r: &mut RoutineContext) {
        let mut song: i32 = engine.mem(0x8E);
        let mut idx: i32 = 0;
        let mut x: i32 = 0;
        let mut blk: i32 = 0;
        x = u8v((if cbool(song < 0x0A) { 0x0A } else { 0x0C }));
        engine.set_mem(0x34, x);
        engine.set_mem(0x35, u8v(x + 1));
        sound_set_song_banks(engine, r);
        engine.set_mem(0x92, 0x00);
        engine.set_mem(0x8F, 0x00);
        idx = u8v((if cbool(song < 0x0A) {
            song
        } else {
            u8v(song - 0x0A)
        }));
        idx = u8v(idx << 1);
        {
            engine.set_mem(0x0E, engine.mem(u16v(0x8000 + idx)));
            engine.set_mem(0x0F, engine.mem(u16v(0x8001 + idx)));
        }
        engine.set_mem(0x0C, 0x93);
        engine.set_mem(0x0D, 0x00);
        {
            blk = 0;
            while cbool(blk < 4) {
                let mut y: i32 = 0;
                let mut s: i32 = u16v(engine.mem(0x0E) | (engine.mem(0x0F) << 8));
                let mut d: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                {
                    y = 7;
                    while cbool(y >= 0) {
                        engine.set_mem(u16v(d + y), engine.mem(u16v(s + y)));
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                    }
                }
                d = u16v(engine.mem(0x0C) + 8);
                engine.set_mem(0x0C, u8v(d));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (d >> 8)));
                d = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                {
                    y = 7;
                    while cbool(y >= 0) {
                        engine.set_mem(u16v(d + y), 0x00);
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                    }
                }
                d = u16v(engine.mem(0x0C) + 8);
                engine.set_mem(0x0C, u8v(d));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (d >> 8)));
                s = u16v(engine.mem(0x0E) + 8);
                engine.set_mem(0x0E, u8v(s));
                engine.set_mem(0x0F, u8v(engine.mem(0x0F) + (s >> 8)));
                {
                    let __old = blk;
                    blk += 1;
                    __old
                };
            }
        }
        ppu_commit_banks(engine, r);
    }
}

mod sound_restore_game_banks {
    use super::*;
    pub fn sound_restore_game_banks(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x06);
        engine.device_write(0x8001, engine.mem(0x30));
        engine.device_write(0x8000, 0x07);
        engine.device_write(0x8001, engine.mem(0x31));
    }
}

mod sound_set_default_banks {
    use super::*;
    pub fn sound_set_default_banks(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x06;
        let mut y: i32 = 0x0A;
        engine.device_write(0x8000, x);
        engine.device_write(0x8001, y);
        x = u8v(x + 1);
        y = u8v(y + 1);
        engine.device_write(0x8000, x);
        engine.device_write(0x8001, y);
        r.index = x;
        r.offset = y;
    }
}

mod sound_set_song_banks {
    use super::*;
    pub fn sound_set_song_banks(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x06);
        engine.device_write(0x8001, engine.mem(0x34));
        engine.device_write(0x8000, 0x07);
        engine.device_write(0x8001, engine.mem(0x35));
    }
}

mod sound_tick {
    use super::*;
    pub fn sound_tick(engine: &mut Engine, r: &mut RoutineContext) {
        sound_set_default_banks(engine, r);
        engine.set_mem(0x02, 0x40);
        r.value = 0x40;
        sfx_overlay_voice(engine, r);
        if cbool(engine.mem(0x8D) != 0) {
            if !cbool(engine.mem(0xD4) & 0x80) {
                engine.device_write((0x4004), (engine.mem(0xA9) & 0xC0) | 0x30);
            }
            engine.device_write((0x4000), (engine.mem(0x99) & 0xC0) | 0x30);
            engine.device_write((0x4008), 0x00);
            engine.device_write((0x400C), 0x30);
            r.value = 0x30;
        } else {
            sound_set_song_banks(engine, r);
            engine.set_mem(0x02, 0x00);
            r.value = 0x00;
            tick_pulse1_channel(engine, r);
            engine.set_mem(0x02, 0x10);
            r.value = 0x10;
            tick_pulse2_channel(engine, r);
            engine.set_mem(0x02, 0x20);
            r.value = 0x20;
            tick_triangle_channel(engine, r);
            engine.set_mem(0x02, 0x30);
            r.value = 0x30;
            tick_noise_channel(engine, r);
        }
        sound_restore_game_banks(engine, r);
    }
}

mod statusbar_split {
    use super::*;
    pub fn statusbar_split(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2001, engine.mem(0x24));
        engine.set_mem(0x23, u8v((engine.mem(0x23) & 0xFE) | engine.mem(0x1D)));
        engine.device_write(0x2000, engine.mem(0x23));
        engine.device_write(0x2005, engine.mem(0x1C));
        engine.device_write(0x2005, engine.mem(0x1E));
        if cbool(engine.mem(0x29) != 0) {
            let _ = engine.mem(0x2002);
            engine.device_write(0x2000, engine.mem(0x23) & 0xFE);
            engine.device_write(0x2005, 0x00);
            engine.device_write(0x2005, 0xC4);
            engine.device_write(0x8000, 0x01);
            engine.device_write(0x8001, 0x16);
            engine.device_write(0x8000, 0x04);
            engine.device_write(0x8001, 0x3E);
            engine.device_write(0x8000, 0x05);
            engine.device_write(0x8001, 0x3F);
        }
        sound_tick(engine, r);
        if cbool(engine.mem(0x29) == 0) {
            return;
        }
        engine.device_write(0x8000, 0x01);
        engine.device_write(0x2000, engine.mem(0x23));
        engine.device_write(0x2005, engine.mem(0x1C));
        engine.device_write(0x2005, engine.mem(0x1E));
        engine.device_write(0x8001, engine.mem(0x2B));
        engine.device_write(0x8000, 0x04);
        engine.device_write(0x8001, engine.mem(0x2E));
        engine.device_write(0x8000, 0x05);
        engine.device_write(0x8001, engine.mem(0x2F));
    }
}

mod text_attr_build {
    use super::*;
    pub fn text_attr_build(engine: &mut Engine, r: &mut RoutineContext) {
        let mut p: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        let mut carry_in: i32 = u8v(r.carry);
        let mut b: i32 = 0;
        b = engine.mem(p);
        engine.set_mem(0x7A, u8v(b + 0xA0 + carry_in));
        engine.set_mem(0x79, 0);
        engine.set_mem(0x2D, engine.mem(u16v(p + 1)));
        engine.set_mem(0x70, engine.mem(u16v(p + 2)));
        engine.set_mem(0x71, engine.mem(u16v(p + 3)));
        engine.set_mem(0x74, engine.mem(u16v(p + 4)));
        engine.set_mem(0x2A, engine.mem(u16v(p + 5)));
        engine.set_mem(0x2B, engine.mem(u16v(p + 6)));
        {
            let mut ms_y: i32 = engine.mem(0x48);
            let mut ms_x: i32 = engine.mem(0x47);
            let mut idx: i32 = u8v(((ms_y << 2) & 0x04) | ms_x);
            let mut a: i32 = engine.mem(u16v(0x0300 + idx));
            let mut cnt: i32 = u8v((ms_y >> 1) + 1);
            let mut c: i32 = 0;
            loop {
                c = u8v((a >> 7) & 1);
                a = u8v(a << 1);
                if !cbool(
                    {
                        cnt -= 1;
                        cnt
                    } != 0,
                ) {
                    break;
                }
            }
            r.value = a;
            r.carry = c;
        }
        {
            let mut y: i32 = 0x07;
            let mut a: i32 = 0;
            if cbool(r.carry) {
                a = engine.mem(u16v(p + y));
            } else {
                a = 0;
            }
            engine.set_mem(0x04A1, a);
            if cbool(a != 0) {
                engine.set_mem(0x04A2, 0x01);
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                engine.set_mem(0x04AD, engine.mem(u16v(p + y)));
                engine.set_mem(0x04AC, 0x00);
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                engine.set_mem(0x04AE, engine.mem(u16v(p + y)));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                b = engine.mem(u16v(p + y));
                if cbool(b == 0x17) {
                    engine.set_mem(0x04A1, 0x19);
                    engine.set_mem(0x04A0, 0xDD);
                } else {
                    engine.set_mem(0x04A0, 0xE9);
                }
            }
        }
        {
            let mut x: i32 = engine.mem(0x8E);
            let mut do_d02e: i32 = 1;
            if cbool(x < 0x05) {
                let mut a: i32 = 0x00;
                let mut c: i32 = 1;
                let mut i: i32 = (x);
                loop {
                    let mut nc: i32 = u8v((a >> 7) & 1);
                    a = u8v((a << 1) | c);
                    c = nc;
                    {
                        i -= 1;
                        i
                    };
                    if !cbool(i >= 0) {
                        break;
                    }
                }
                a = u8v(a & engine.mem(u16v(p + 0x15)));
                if cbool(a != 0) {
                    do_d02e = 0;
                }
            }
            if cbool(do_d02e) {
                r.value = engine.mem(u16v(p + 0x0B));
                routine_0123(engine, r);
            }
        }
        engine.set_mem(0x80, engine.mem(u16v(p + 0x10)));
        engine.set_mem(0x81, engine.mem(u16v(p + 0x11)));
        engine.set_mem(0x82, engine.mem(u16v(p + 0x12)));
        engine.set_mem(0x83, engine.mem(u16v(p + 0x13)));
        engine.set_mem(0x41, engine.mem(u16v(p + 0x14)));
    }
}

mod vblank_commit {
    use super::*;
    pub fn vblank_commit(engine: &mut Engine, r: &mut RoutineContext) {
        let save = *r;
        {
            engine.ppu.set_vblank(cbool(1));
            engine
                .ppu
                .set_sprite0(cbool((if cbool(engine.mem(0x24) & 0x18) { 1 } else { 0 })));
            engine.ppu.eval_sprite_overflow();
        }
        {
            let __v = engine.device_read(0x2002);
            engine.set_mem(0x26, __v);
        }
        engine.device_write(0x2003, 0x00);
        engine.device_write(0x4014, 0x02);
        let mut req: i32 = engine.mem(0x28);
        if cbool(req == 0) {
            vblank_commit_tail(engine, r);
            {
                *r = save;
                return;
            }
        }
        engine.set_mem(0x28, 0x00);
        if cbool(req >= 0x07) {
            vblank_commit_tail(engine, r);
            {
                *r = save;
                return;
            }
        }
        {
            const jt_lo: [i32; 7] = [0x51, 0x52, 0x5F, 0x90, 0xE5, 0x34, 0x44];
            const jt_hi: [i32; 7] = [0xD3, 0xD2, 0xD2, 0xD2, 0xD2, 0xD3, 0xD3];
            engine.set_mem(0x06, jt_lo[req as usize]);
            engine.set_mem(0x07, jt_hi[req as usize]);
        }
        let _ = engine.device_read(0x2002);
        engine.device_write(0x2006, engine.mem(0x17));
        engine.device_write(0x2006, engine.mem(0x16));
        engine.device_write(0x2000, u8v(engine.mem(0x23) & 0x04));
        match req {
            1 => {
                vram_fill_run(engine, r);
            }
            2 => {
                vram_upload_palette(engine, r);
            }
            3 => {
                vram_upload_hud(engine, r);
            }
            4 => {
                vram_blit_stack(engine, r);
            }
            5 => {
                vram_copy_indirect(engine, r);
            }
            6 => {
                vram_poke2(engine, r);
            }
            _ => {}
        }
        *r = save;
    }
}

mod vblank_commit_tail {
    use super::*;
    pub fn vblank_commit_tail(engine: &mut Engine, r: &mut RoutineContext) {
        ppu_commit_banks(engine, r);
        statusbar_split(engine, r);
        if cbool(engine.mem(0x36) != 0) {
            engine.dec_mem(0x36);
        }
        frame_counters(engine, r);
        engine.device_write(0x8000, engine.mem(0x25));
    }
}

mod vram_blit_stack {
    use super::*;
    pub fn vram_blit_stack(engine: &mut Engine, r: &mut RoutineContext) {
        {
            let mut i: i32 = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, engine.mem(u16v(0x0100 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_copy_indirect {
    use super::*;
    pub fn vram_copy_indirect(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x1A);
        let mut src: i32 = u16v(engine.mem(0x18) | (engine.mem(0x19) << 8));
        let mut y: i32 = 0;
        loop {
            engine.device_write(0x2007, engine.mem(u16v(src + y)));
            {
                let __old = y;
                y += 1;
                __old
            };
            if !cbool(
                {
                    x -= 1;
                    x
                } != 0,
            ) {
                break;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_fill_run {
    use super::*;
    pub fn vram_fill_run(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x1A);
        let mut a: i32 = engine.mem(0x18);
        loop {
            engine.device_write(0x2007, a);
            if !cbool(
                {
                    x -= 1;
                    x
                } != 0,
            ) {
                break;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_poke2 {
    use super::*;
    pub fn vram_poke2(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2007, engine.mem(0x19));
        engine.device_write(0x2007, engine.mem(0x18));
        vblank_commit_tail(engine, r);
    }
}

mod vram_upload_hud {
    use super::*;
    pub fn vram_upload_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        engine.device_write(0x2000, u8v(engine.mem(0x23) | 0x04));
        {
            x = 0x17;
            while cbool(x >= 0) {
                engine.device_write(0x2007, engine.mem(u16v(0x0140 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.device_write(0x2006, engine.mem(0x17));
        engine.device_write(0x2006, u8v(engine.mem(0x16) + 1));
        {
            x = 0x17;
            while cbool(x >= 0) {
                engine.device_write(0x2007, engine.mem(u16v(0x0158 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            x = 0x0A;
            while cbool(x >= 0) {
                engine.device_write(0x2006, engine.mem(0x19));
                engine.device_write(0x2006, engine.mem(u16v(0x0170 + x)));
                let _ = engine.device_read(0x2007);
                {
                    let mut v: i32 = u8v((engine.device_read(0x2007) & engine.mem(0x18))
                        | engine.mem(u16v(0x0171 + x)));
                    engine.device_write(0x2006, engine.mem(0x19));
                    engine.device_write(0x2006, engine.mem(u16v(0x0170 + x)));
                    engine.device_write(0x2007, v);
                }
                x -= 2;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_upload_palette {
    use super::*;
    pub fn vram_upload_palette(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        engine.device_write(0x2006, 0x3F);
        engine.device_write(0x2006, 0x00);
        {
            y = 0;
            while cbool(y < 0x20) {
                engine.device_write(0x2007, engine.mem(u16v(0x0180 + y)));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
            }
        }
        engine.device_write(0x2006, 0x3F);
        engine.device_write(0x2006, 0x00);
        engine.device_write(0x2006, 0x00);
        engine.device_write(0x2006, 0x00);
        vblank_commit_tail(engine, r);
    }
}
