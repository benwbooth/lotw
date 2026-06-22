# Routine Catalog

This catalog is the working map for the numbered `routine_####` functions that
remain in `src/game.rs` and `src/native.rs`.

The numbers are stable port labels, not good semantic names. A routine should
only be renamed or given a public semantic alias when its dataflow and call
sites are understood well enough that the name is more useful than the numeric
label. Until then, this file is the source of truth for the current hypothesis.

## Status Legend

- `named`: the Rust code already has a semantic name.
- `inferred`: the role follows from call sites, memory reads/writes, and tests.
- `cluster`: the routine belongs to a known subsystem, but its exact local role
  still needs trace evidence or a narrow regression test.
- `unknown`: do not rename yet.

## Runtime Skeleton

The native executable runs these major phases:

1. `main_init` initializes RAM, banks, PPU/APU state, builds the starting room,
   and enters `native::main_loop_dispatch`.
2. `native::main_loop_dispatch` is the foreground game loop.
3. `frame::FrameRunner` parks the foreground loop at explicit waits.
4. `vblank_commit` is the NMI-style interrupt boundary. It saves the current
   `RoutineContext`, commits OAM/VRAM/audio/frame timers, then restores the
   interrupted context.
5. `game_update` processes live input, movement, item actions, character swaps,
   room collisions, and follow-up state changes for one foreground tick.
6. `update_player_projectiles`, `update_room_actors`, and `update_tile_projectile`
   update player shots, room actors, and the special tile-removal projectile
   before the render pass.

## Memory Map

The port still uses the original RAM layout. These names are inferred from the
Rust dataflow and should be preferred in comments and future renames.

| Address | Meaning | Evidence |
|---|---|---|
| `0x20` | current buttons | `read_controllers`, `game_update`, tests |
| `0x23` | PPU control shadow | `statusbar_split`, VRAM upload routines |
| `0x24` | PPU mask shadow | renderer and sprite-zero handling |
| `0x25` | selected PRG-bank register | bank switching helpers |
| `0x26` | PPU status latch | `vblank_commit` |
| `0x28` | queued VRAM job id | `clear_pending_vram_job`, `queue_ppu_job_and_wait`, `vblank_commit` |
| `0x2A..0x2F` | PPU bank shadows | `text_attr_build`, `ppu_commit_banks` |
| `0x30..0x33` | PRG bank shadows and saved banks | farcall helpers |
| `0x36` | frame wait countdown | `frame` module, `vblank_commit_tail` |
| `0x40` | current family member index | character select and actor logic |
| `0x41` | room actor availability mask | scene assembly and actor spawn |
| `0x43..0x45` | player sub-tile, tile-x, pixel-y | movement/collision/projectiles |
| `0x46` | action lockout timer | `game_update`, item pickup feedback |
| `0x47..0x48` | map-space room coordinates | room assembly |
| `0x49..0x4F` | movement/action scratch | movement and collision checks |
| `0x51..0x55` | carried items and selected slot | inventory and action routines |
| `0x58` | health counter | `add_health_points`, `consume_health_point`, `subtract_health_points` |
| `0x59` | magic/action counter | `add_magic_points`, `consume_magic_point`, projectile/action use |
| `0x5A` | coin counter | `add_coins`, `spend_coins`, shop item purchases |
| `0x5B` | key counter | `add_key`, `add_keys`, `consume_key`, door and tile-action costs |
| `0x5C..0x5F` | jump duration, projectile damage, projectile slot count, projectile lifetime | `load_effective_jump_duration`, `load_effective_projectile_damage`, `update_player_projectiles`, `load_effective_projectile_lifetime` |
| `0x6E` | fragment/event progress count | `enter_fragment_pickup_room`, `dispatch_overhead_tile_action` |
| `0x70..0x74` | current room tile/action ids | scene assembly and item actions |
| `0x75..0x78` | room map pointer and saved pointer | tile addressing and scene assembly |
| `0x79..0x7A` | current metasprite/table pointer | sprite build and room assets |
| `0x7B..0x7C` | scroll/world position | sprite projection and room render |
| `0x84..0x8C` | frame/effect timers | `frame_counters`, invulnerability and speed-boost rewards |
| `0x8D..0x8F` | audio/sfx mode and pending sfx id | sound tick and sfx overlay |
| `0x90..0x92` | sfx/music scratch | sound init and overlay voice |
| `0x93..0xC6` | music channel state | `tick_*_channel`, `dispatch_audio_stream_command`, envelope helpers |
| `0xD3..0xDD` | sfx overlay channel state | `sfx_overlay_voice` |
| `0xE3..0xE9` | object-loop slot index and pointers | actor update loops |
| `0xEB..0xEC` | pending special-exit and final-exit flags | `enter_pending_special_exit_room`, `check_final_exit_trigger` |
| `0xED..0xFC` | current object scratch slot | copied by `load_object_slot_scratch/store_object_slot_scratch` |
| `0xFD` | held/directional action latch | input edge/action gating |
| `0x0200..0x02FF` | OAM staging | render and vblank commit |
| `0x0300..0x03FF` | persistent room/event bits plus inventory/status staging buffers | map progress, room flags, `snapshot_inventory_state`, `upload_inventory_item_list` |
| `0x0400..0x04AF` | 16-byte object slots | actors, items, projectiles, door slot |
| `0x04B0..` | pooled player projectile slots | `update_player_projectiles`, `update_player_projectile_slot` |

## Complete Numbered Routine Coverage

The following ranges cover every numbered routine currently present. A range is
used when the routines form one tightly-coupled subsystem and individual names
would currently be weaker than the cluster name.

| Module | Routines | Status | Current role |
|---|---:|---|---|
| `game` | `0003`, `0005..0019` | cluster | opening/title scripted helpers, timed waits, cutscene sprite setup, and startup scene setup |
| `game` | `0021..0028`, `0030..0032` | cluster | password/title/menu support and first-room transition helpers |
| `native` | `0001`, `0002`, `0004`, `0020`, `0029` | inferred | high-level start flow, intro/menu flow, and blocking input gates rewritten around frame tasks |
| `native` | `0033`, `0034`, `0039`, `0045`, `0049`, `0050`, `0055` | inferred | title screen, family select, password entry, and start-game screen orchestration |
| `game` | `0035..0038`, `0040..0044`, `0046..0048` | cluster | family/password/menu visual and state helpers |
| `game` | `0051..0054`, `0056`, `0057` | cluster | transition, palette, and display setup helpers used by menu/start flows |
| `native` | `0058` | inferred | post-render PPU/status handling in the foreground loop |
| `game` | `0059..0066` | inferred | frame render pass, OAM clearing, background/object sprite projection, and palette/display setup |
| `native` | `0067..0072`, `0074` | inferred | room transition and item/inventory screen orchestration |
| `game` | `0073..0089` | inferred | VRAM/PPU setup, room render upload, palette updates, and room assembly helpers |
| `native` | `0109`, `0110` | inferred | object/player overlap search across live object slots |
| `game` | `0117..0123` | cluster | persistent room flag and room tile mutation helpers |

## Named Non-Numbered Routines

These already have semantic names and should be kept as the preferred call
surface when touching nearby code:

| Routine | Role |
|---|---|
| `add_coins` | add to the coin counter and refresh its HUD digits |
| `add_health_points` | add to health and refresh its HUD digits |
| `add_key` | add one key and refresh the key HUD digits |
| `add_keys` | add to the key counter and refresh its HUD digits |
| `add_magic_points` | add to magic and refresh its HUD digits |
| `aim_actor_from_player_overlap` | set actor direction bits by comparing actor/player overlap on each axis |
| `aim_actor_toward_player` | set actor direction bits toward the player, with optional room-data vertical bias |
| `animate_actor_cycle_tiles` | cycle actor sprite tile bits from the animation timer |
| `animate_actor_directional_walk` | update actor facing and horizontal/vertical walk animation bits |
| `animate_actor_flip_toggle` | periodically toggle the actor sprite flip bit |
| `animate_actor_walk_toggle` | update actor facing and toggle a walking sprite tile bit |
| `animate_large_actor_body_tiles` | advance the large actor animation timer and derive linked body-slot tile ids |
| `animate_health_refill_to_cap` | count health up to the reward cap while updating HUD and prompt animation |
| `animate_magic_refill_to_cap` | count magic up to the reward cap while updating HUD and prompt animation |
| `advance_envelope_phase` | tick the selected audio channel's envelope duration and advance or terminate its phase |
| `apply_actor_player_contact_damage` | apply actor contact damage and hit feedback unless invulnerability or special state suppresses it |
| `apply_event_collectible_reward` | apply a reward from an event/shop path where no room object slot is cleared |
| `apply_hazard_tile_contact` | apply floor hazard contact damage, recoil, and invulnerability latch for tile `0x30` |
| `audio_cmd_set_channel_flags` | audio bytecode command 2: replace the selected channel flag/register shadow byte |
| `audio_cmd_set_duty_instrument` | audio bytecode command 0: set pulse duty bits and choose the envelope/instrument table offset |
| `audio_cmd_set_pitch_offset` | audio bytecode command 3: set the selected channel's fine pitch offset |
| `audio_cmd_set_sweep_value` | audio bytecode command 4: set the selected channel's sweep/noise-period shadow byte |
| `audio_cmd_set_volume_scale` | audio bytecode command 1: set the per-channel envelope volume scale |
| `build_direction_velocity` | convert direction bits and speed into object velocity scratch `0xF5..0xF7` |
| `build_health_meter_sprites` | build a two-row OAM health meter from full/empty tile ids |
| `build_input_movement_delta` | convert current input and speed into player movement scratch `0x49..0x4B` |
| `build_object_health_meter_alt_tiles` | build object health with the alternate `0xA5/0xAB` sprite tile pair |
| `build_object_health_meter_standard_tiles` | build object health with the standard `0x65/0x6B` sprite tile pair |
| `build_player_health_meter_sprites` | build the player health sprite meter in the second OAM meter slot |
| `build_status_resource_meter_tiles` | build the two-row status resource meter in VRAM staging buffers |
| `check_actor_position_out_of_bounds` | test projected actor position against the tighter actor bounds |
| `check_actor_direction_contact` | project one actor direction and report whether it contacts the player |
| `check_final_exit_trigger` | set the final-exit flag when selected item `0x0F` is used at the exact required room position |
| `check_projected_terrain_collision` | test the projected one-tile-wide object footprint against terrain |
| `check_projected_wide_terrain_collision` | test the projected wide object footprint against terrain |
| `check_player_overlap` | test projected object position against the player hitbox |
| `check_player_overlap_wide` | wider player hitbox test used by large/falling movement probes |
| `check_player_x_overlap` | horizontal half of the player hitbox test |
| `check_player_y_overlap` | vertical half of the player hitbox test |
| `check_position_out_of_bounds` | test projected position against the general playfield bounds |
| `check_top_boundary_exit_clear` | report whether the top-edge room transition is clear for the player to wrap upward |
| `choose_random_actor_direction` | choose one actor direction-bit pattern from the full movement table |
| `choose_random_cardinal_actor_direction` | choose one actor direction-bit pattern from the smaller wandering set |
| `clear_gameplay_object_sprites` | hide the gameplay-object half of OAM while leaving HUD sprites untouched |
| `clear_inventory_item_list_buffer` | fill the inventory item-list source buffer with blank tile ids |
| `clear_pending_vram_job` | clear the deferred VRAM job selector at `0x28` |
| `clear_temporary_room_sprites` | hide the temporary room item and coin/cost sprites in OAM |
| `close_inventory_item_menu` | close the item menu, restore the gameplay snapshot, and redraw HUD state |
| `collect_key_bundle_reward` | add the large key bundle reward and play its pickup sound |
| `collect_large_coin_reward` | add the large coin reward and play its pickup sound |
| `collect_room_pickup_object` | clear the touched room object/OAM entry and apply its collectible reward |
| `collect_single_key_reward` | add one key and play its pickup sound |
| `collect_small_coin_reward` | add the small coin reward and play its pickup sound |
| `collect_small_health_reward` | add the small health reward and play its pickup sound |
| `collect_small_magic_reward` | add the small magic reward and play its pickup sound |
| `commit_actor_projected_position` | copy projected actor position from `0x0E/0x0F/0x0A` back to actor scratch `0xF9..0xFB` |
| `compose_large_actor_body_slots` | mirror the large actor logical slot into the three linked 2x2 body sprite slots and refresh its health meter |
| `consume_health_point` | spend one health point and report empty health through carry |
| `consume_key` | spend one key and report missing keys through carry |
| `consume_magic_point` | spend one magic point and report missing magic through carry |
| `dispatch_actor_behavior` | route an active room actor to the behavior handler selected by room actor data byte 8 |
| `dispatch_audio_stream_command` | consume a `0xFF`-prefixed audio stream command and route it to the selected channel helper |
| `dispatch_overhead_tile_action` | handle Up-button interactions with the tile above the player |
| `dispatch_projected_tile_actions` | probe the projected player footprint for room tile-action triggers |
| `dispatch_room_tile_action` | dispatch the sampled projected room tile, including costs, object spawns, and tile projectiles |
| `draw_carried_item_sprites` | draw the three carried-item slots for temporary room item selection |
| `draw_coin_cost_sprites` | draw the two-sprite coin/cost marker shared by shop and refill rooms |
| `draw_shop_item_sprites` | draw the two shop item slots and hide unavailable or overstocked items |
| `defeat_active_room_actors` | mark active room actors as defeated and run the palette flash reward effect |
| `enter_temporary_room_page` | enter a temporary room page with the full fade and audio-channel reset |
| `enter_fragment_pickup_room` | run the warp effect and enter the fragment-progress room selected by `0x6E` |
| `enter_pending_special_exit_room` | consume the pending special-exit flag and enter its fixed destination room |
| `enter_room_link_destination` | load a destination room and player position from the active room link record |
| `farcall_bank_09_r7` | temporarily map bank 9 into PRG slot 7 and build a metasprite |
| `farcall_bank_0C0D_seed` | seed PRG banks 0x0C/0x0D into the bank shadows |
| `farcall_return_home` | restore saved PRG bank shadows after a farcall-style section |
| `frame_counters` | tick coarse frame timers once per second |
| `game_update` | foreground input/player/item update |
| `grant_long_invulnerability` | start the long invulnerability reward timer |
| `grant_long_speed_boost` | start or queue a long speed/action boost reward timer |
| `grant_short_invulnerability` | start the short invulnerability reward timer |
| `grant_short_speed_boost` | start or queue a short speed/action boost reward timer |
| `handle_player_room_transition` | handle player transitions across room edges, including scroll/rebuild effects |
| `inc16_95` | increment the music stream pointer for the selected channel |
| `initialize_large_actor_slot` | spawn the special large actor slot from room actor data after checking its wide footprint |
| `load_effective_jump_duration` | load jump duration, applying the selected jump-item boost when magic is available |
| `load_effective_projectile_damage` | load projectile damage, applying the selected power-item boost when magic is available |
| `load_effective_projectile_lifetime` | load projectile lifetime/state, applying the selected range-item boost when magic is available |
| `load_object_slot_scratch` | copy a 16-byte object slot into scratch RAM `0xED..0xFC` |
| `load_note_period` | convert an audio note byte into low/high APU period bytes in `0x04/0x05` |
| `main_init` | hardware/RAM/bootstrap sequence and handoff to main loop |
| `maybe_spawn_pursuer_actor` | one-in-30 secondary actor spawn path that seeds scratch position from the player slot |
| `metasprite_build` | build HUD/metasprite staging data for a queued VRAM upload |
| `move_inventory_cursor_down` | move the inventory grid cursor down, wrapping across five rows |
| `move_inventory_cursor_left` | move the inventory grid cursor left, wrapping across seven columns |
| `move_inventory_cursor_right` | move the inventory grid cursor right, wrapping across seven columns |
| `move_inventory_cursor_up` | move the inventory grid cursor up, wrapping across five rows |
| `next_envelope_volume` | update the selected audio channel's envelope accumulator and compose the APU volume byte |
| `ppu_commit_banks` | write all PPU bank shadows to the mapper |
| `pop_room_checkpoint` | restore the most recently saved gameplay room position, scroll, and room identity fields |
| `push_room_checkpoint` | save gameplay room position, scroll, room identity, and current song before temporary room flows |
| `project_player_projectile_position` | project a player projectile from player pose and slot velocity |
| `probe_object_solid_tile` | test a tile in the current object terrain-probe footprint for solidity |
| `probe_actor_overhead_step` | probe the projected tile row above an actor when it is tile-aligned |
| `probe_projected_solid_tile` | test a tile in the projected movement footprint for solidity |
| `probe_player_solid_tile` | test a player-footprint tile sample, including the tile-aligned empty-floor case |
| `project_actor_position` | project actor scratch position through actor velocity into movement scratch |
| `project_player_position` | project player position through horizontal and vertical movement deltas into scratch |
| `ram_state_init` | initialize zero-page, palette, and RAM defaults from ROM tables |
| `read_controllers` | read replay/live input into the current button byte |
| `read_debounced_buttons` | wait for release, press, and release, returning the pressed buttons |
| `read_room_tile_action_value` | read a room-map tile sample and resolve replacement tile `0x3E` through `0x74` |
| `redraw_room_tile_column` | rebuild the background column containing object scratch tile-x `0xFA` |
| `refresh_temporary_room_page` | rebuild a temporary room page with the shorter fade that preserves audio state |
| `reset` | top-level reset entry |
| `reset_room_object_slots` | clear all room object slots to inactive and reset the actor scheduler phase |
| `resolve_room_tile_pointer` | convert room tile coordinates in scratch into a room tile pointer |
| `restore_room_from_checkpoint` | pop a temporary-room checkpoint and rebuild the gameplay room, saved song, sprites, and player pose |
| `restore_status_sprite_template` | restore the fixed status/menu sprite template and its PPU bank shadows |
| `reverse_actor_horizontal_direction` | flip the low horizontal actor direction bits |
| `rng_update` | update random source bounded by `r.value` |
| `run_carried_item_loadout_flow` | refill the active family member's carried-item queue from inventory |
| `run_character_select_overlay` | open the in-game character-select overlay, wait for select-button release, and restore the room |
| `run_character_select_room_flow` | run the special room flow for resource refill, carried-item return, family selection, and item pages |
| `run_inventory_item_grid_menu` | run the interactive inventory item-grid editor from the character-selection room |
| `run_shop_room_flow` | enter the overhead-tile shop room, handle purchases, and restore the previous gameplay room on exit |
| `run_warp_transition_effect` | shared scroll/audio transition used before scripted room warps |
| `rewind_or_stop_audio_stream` | handle a zero audio stream byte by rewinding to the loop pointer or stopping the channel |
| `restore_inventory_state_snapshot` | restore progress, inventory counts, coins, and keys saved before inventory/status flows |
| `scale_envelope_volume` | apply the selected channel volume scale to the raw 4-bit envelope accumulator |
| `scale_room_tile_column` | multiply a room tile column by the room-data stride of 12 |
| `scene_assemble` | rebuild room state from current map coordinates |
| `seed_object_position_from_tile_offset` | convert a tile sample offset and projected coordinates into object scratch position |
| `select_inventory_grid_entry` | copy the active inventory grid entry into the scrolling item-list buffer or handle menu controls |
| `set_inventory_list_buffer_index` | convert the scrolling item-list cursor into a 32-byte buffer index |
| `show_inventory_item_list_screen` | show the read-only inventory item-list page until the player presses a button |
| `spawn_player_projectile` | allocate/spawn a player projectile from current input and facing |
| `split_meter_value` | split a resource value into full 10-point blocks and a partial block |
| `sfx_overlay_voice` | play pending sound effects over music channel state |
| `song_init` | initialize all music channels for the selected song id |
| `sound_tick` | per-frame music and sfx tick |
| `spend_coins` | subtract a coin cost and report affordability through carry |
| `snapshot_inventory_state` | save progress, inventory counts, coins, and keys before temporary inventory/status flows |
| `stop_actor_motion` | clear actor velocity and arc/probe motion counters |
| `start_note_envelope` | load the selected channel's active-note envelope phase state |
| `start_rest_envelope` | load the selected channel's timed silent envelope phase state |
| `statusbar_split` | status-bar scroll/bank update plus audio tick |
| `store_object_slot_scratch` | copy scratch RAM `0xED..0xFC` back into the current 16-byte object slot |
| `subtract_health_points` | subtract damage from health and saturate underflow at zero |
| `sync_coin_hud` | clamp coins and queue their HUD digits for redraw |
| `sync_health_hud` | clamp health and queue its HUD digits for redraw |
| `sync_key_hud` | clamp keys and queue their HUD digits for redraw |
| `sync_magic_hud` | clamp magic and queue its HUD digits for redraw |
| `text_attr_build` | derive room actor/tile/CHR metadata from the current room record |
| `tick_actor_materialize_delay` | count down a materializing actor and promote it to behavior-dispatched state when ready |
| `tick_chasing_jump_actor` | actor behavior that re-aims toward the player and uses jump/gravity terrain movement |
| `tick_contact_recoil_actor` | actor behavior that switches to a high-bit recoil state when player contact blocks movement |
| `tick_contact_trigger_actor` | actor behavior that wakes into chasing movement after one-step player contact |
| `tick_defeated_actor_reward_drop` | run the high-bit defeated-actor rise/fall sequence and turn it into a pickup drop |
| `tick_inactive_actor_slot` | initialize an inactive actor scratch slot from room actor data and spawn timing |
| `tick_large_chasing_actor` | large actor behavior that aims toward the player and uses the wide jump/gravity movement path |
| `tick_ledge_walking_actor` | actor behavior that walks along supported ledges and falls when unsupported |
| `tick_noise_channel` | per-frame music tick for the noise channel lane at `0xC3..0xC6` |
| `tick_overhead_probe_actor` | actor behavior that alternates overhead probes, falling, and jump arcs |
| `tick_player_jump_action` | start or continue the player jump/action arc and apply the selected jump-item boost |
| `tick_player_walk_animation` | advance the player walk animation and facing/action overlay bits |
| `tick_pulse1_channel` | per-frame music tick for the first square/pulse channel lane at `0x93..0x96` |
| `tick_pulse2_channel` | per-frame music tick for the second square/pulse channel lane at `0xA3..0xA6`, including sfx overlay suppression |
| `tick_random_floating_actor` | actor behavior that chooses random directions and moves without terrain collision |
| `tick_reflecting_chase_actor` | actor behavior that aims from player overlap and reflects velocity when blocked |
| `tick_selected_item_effect` | apply the currently selected passive or consumable item effect |
| `tick_special_exit_actor_sequence` | run the special-exit actor rise/fall sequence and raise pending special-exit flag `0xEB` |
| `tick_standard_actor` | generic non-boss actor tick for motion continuation, collision response, expiry, and terrain probing |
| `tick_timed_chase_actor` | actor behavior that chases for a finite timer and rejects abrupt multi-axis turns |
| `tick_triangle_channel` | per-frame music tick for the triangle channel lane at `0xB3..0xB6` |
| `tick_wandering_jump_actor` | actor behavior that wanders, occasionally jumps, then falls through terrain-aware movement |
| `try_reflect_object_velocity` | try to reflect object velocity away from a blocked subtile edge |
| `try_actor_gravity_motion` | try a falling actor move, dropping horizontal velocity if the first projection is blocked |
| `try_actor_jump_arc_motion` | convert the actor jump countdown into upward velocity and try the projected move |
| `try_large_actor_gravity_motion` | apply large actor falling motion with the wide movement probe before the horizontal fallback |
| `try_large_actor_jump_arc_motion` | advance the large actor jump arc and retry straight-up movement on terrain collision |
| `try_move_actor_with_terrain` | project an actor move, check bounds/player/terrain, and report whether movement was blocked |
| `try_move_actor_without_terrain` | project an actor move that ignores terrain but still checks player contact and bounds |
| `try_move_large_actor_with_terrain` | project large actor motion, apply wide contact damage, and reject the three-tile-wide footprint |
| `try_move_player_with_collision` | project a player move, handle room exits/tile actions/object contact, and restore deltas |
| `try_nudge_player_to_tile_boundary` | retry a blocked player move with a small nudge toward the nearest tile boundary |
| `trigger_damage_pickup` | apply the harmful pickup/trap reward effect |
| `try_trigger_magic_contact_actor` | mark a contacted actor for high-bit behavior when the magic-contact timer is active |
| `unlock_door_with_key` | spend a key and run the door-unlock prompt/music sequence |
| `update_actor_animation` | dispatch the actor animation mode from room actor data byte 7 |
| `update_inventory_grid_cursor_sprites` | position the 2x2 cursor around the active inventory grid cell |
| `update_inventory_list_cursor_sprites` | position the arrow sprites for the scrolling selected item-list slot |
| `update_object_terrain_probe` | advance the normal object terrain probe when its footprint stays clear |
| `update_player_pose_from_motion` | update player pose and horizontal flip from movement, jump/fall, and lockout state |
| `update_room_actors` | room actor scheduler that copies object slots to scratch, runs the state path, and stores them back |
| `update_large_actor_facing_from_velocity` | update the large actor facing bit from horizontal velocity |
| `upload_resource_hud` | queue the resource HUD VRAM upload after counter changes |
| `update_player_terrain_contact` | update player floor, fall, hazard, and post-move contact state |
| `update_player_projectile_slot` | update one player projectile slot and clear it on expiry/collision |
| `update_player_projectiles` | pooled player projectile slot scheduler |
| `update_tile_projectile` | special tile-removal projectile scheduler |
| `update_tile_projectile_motion` | special projectile movement, collision, bounce, and tile replacement |
| `upload_inventory_item_list` | stage and upload the two visible rows of the inventory item-list buffer |
| `update_wide_object_terrain_probe` | advance the wide object terrain probe when its footprint stays clear |
| `vblank_commit` | NMI-style interrupt body for OAM, VRAM jobs, and tail work |
| `vblank_commit_tail` | common NMI tail: banks, status bar, sound, frame timers |
| `vram_*` | deferred VRAM job implementations |
| `wait_for_button_press` | frame-advance until any button is pressed |
| `wait_for_buttons_released` | frame-advance until all buttons are released |
| `walk_character_select_room_until_action` | move within the character-select room and track the selected tile until action is pressed |
| `walk_loadout_room_until_action_or_exit` | move within the carried-item loadout room until action or the exit tile |
| `walk_purchase_room_until_action_or_exit` | move within purchase/refill rooms until action or the exit tile |

## Next Renaming Targets

The safest remaining concrete rename/alias batches are:

1. Object/player overlap search helpers: `routine_0109`, `routine_0110`.

Each batch should come with a narrow regression test or an existing replay smoke
before replacing numeric call sites.
