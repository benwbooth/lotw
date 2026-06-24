//! Headless animation driver: renders a sequence of in-game frames to PPM files.
//!
//! Like `ppu_game_driver`, this assembles a real playable scene, but instead of
//! a single frame it holds a fixed controller input and runs the full per-frame
//! game update + render loop for many frames, writing each one to
//! `build/anim/fNNN.ppm`. Useful for eyeballing animation/scrolling behaviour
//! (e.g. walking the player in one direction) as a flipbook of images.
//!
//! Usage: `ppu_anim [rom] [buttons_hex] [frames]`
//! - `rom`         ROM path (default `rom/lotw.nes`)
//! - `buttons_hex` controller bitmask held every frame, hex (default `128` = Right)
//! - `frames`      number of frames to render (default 48)

mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

/// Default held controller input when none is given: 128 = Right on the d-pad.
const DEFAULT_BUTTONS: i32 = 128;
/// Default number of animation frames to render.
const DEFAULT_FRAMES: usize = 48;

/// Assemble a scene, hold `buttons` pressed, and render `frames` animation frames
/// to `build/anim/fNNN.ppm`. Returns an error on I/O failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments. The button mask is hex (with an optional "0x" prefix).
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let btn = args
        .get(2)
        .and_then(|s| i32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(DEFAULT_BUTTONS);
    let frames: usize = args
        .get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_FRAMES);

    // Load ROM and build the deterministic playable scene.
    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    common::init_game_scene(&mut engine, &mut r);
    // Pre-stage room columns at two scroll positions and prime scroll shadows.
    engine.state.scroll_tile_x = 16;
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    engine.state.scroll_tile_x = 32;
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    // Draw the HUD (status panel + inventory) and start background music.
    game::upload_status_panel_template(&mut engine, &mut r);
    game::upload_inventory_item_list(&mut engine, &mut r);
    engine.state.song = 0;
    engine.state.sound_paused = 0;
    game::song_init(&mut engine, &mut r);

    // Per-frame loop: feed the held input, run a full game update, render, save.
    for fr in 0..frames {
        // Hold the same controller buttons every frame and latch them.
        engine.ppu.buttons = (btn as u8);
        engine.state.frame_counter = 1;
        game::read_controllers(&mut engine, &mut r);
        // Full game-update pipeline (player, projectiles, actors, scroll, sprites).
        game::game_update(&mut engine, &mut r);
        game::update_player_projectiles(&mut engine, &mut r);
        game::update_room_actors(&mut engine, &mut r);
        game::update_tile_projectile(&mut engine, &mut r);
        game::update_camera_scroll_from_player(&mut engine, &mut r);
        game::draw_player_sprites(&mut engine, &mut r);
        game::draw_room_object_sprites(&mut engine, &mut r);
        lotw::game::commit_foreground_frame_and_wait(&mut engine, &mut r);
        game::sound_tick(&mut engine, &mut r);
        // Force rendering on if disabled: mask bits 3/4 (8/16) show bg/sprites;
        // 30 enables both plus the leftmost 8-pixel columns.
        if (engine.ppu.mask & 24) == 0 {
            engine.ppu.mask = 30;
        }
        // Ensure the NMI-enable / control bit (8) is set for rendering.
        engine.ppu.ctrl |= 8;
        // Render and write this frame, then log the player's position/input.
        let fb = common::render_frame(&mut engine);
        common::write_ppm(format!("build/anim/f{fr:03}.ppm"), &fb)?;
        eprintln!(
            "frame {fr}: player x_tile={:02X} y={:02X} input20={:02X}",
            engine.state.player_x_tile, engine.state.player_y, engine.state.buttons
        );
    }
    Ok(())
}
