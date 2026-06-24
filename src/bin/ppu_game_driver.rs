//! Headless driver that renders one in-game frame using the real game logic.
//!
//! Unlike `ppu_shim_test` (which hand-fills VRAM), this driver assembles an
//! actual playable scene via [`common::init_game_scene`], stages the room
//! columns / status panel / inventory the way the game does, runs a few
//! `game_update` cycles to let actors and scrolling settle, then renders the
//! final frame to `build/game_frame.ppm`. It exercises the full game-update +
//! PPU pipeline without the SDL front-end.
//!
//! Usage: `ppu_game_driver [rom]` (ROM defaults to `rom/lotw.nes`).

mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

/// Number of warm-up game-update frames to run before rendering, so the scene
/// (actors, scrolling, sprites) reaches a steady state.
const WARMUP_FRAMES: usize = 4;

/// Build and warm up an in-game scene, then render a single frame to
/// `build/game_frame.ppm`. Returns an error on I/O failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Load ROM and build a deterministic playable scene.
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    common::init_game_scene(&mut engine, &mut r);
    // Pre-stage room tile columns at two scroll positions so the nametable is
    // populated to the right of the start, then prime the scroll shadows.
    engine.state.scroll_tile_x = 16;
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    engine.state.scroll_tile_x = 32;
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    // Draw the HUD: status panel template and the inventory item list.
    game::upload_status_panel_template(&mut engine, &mut r);
    game::upload_inventory_item_list(&mut engine, &mut r);
    // Run a few full game-update cycles so the scene settles before rendering.
    for fr in 0..WARMUP_FRAMES {
        eprintln!("frame {fr}: game_update...");
        engine.state.frame_counter = 1;
        game::game_update(&mut engine, &mut r);
        game::update_player_projectiles(&mut engine, &mut r);
        game::update_room_actors(&mut engine, &mut r);
        game::update_tile_projectile(&mut engine, &mut r);
        game::update_camera_scroll_from_player(&mut engine, &mut r);
        game::draw_player_sprites(&mut engine, &mut r);
        game::draw_room_object_sprites(&mut engine, &mut r);
        lotw::game::commit_foreground_frame_and_wait(&mut engine, &mut r);
    }
    // Force rendering on if the game left it disabled: mask bit 3 (8) = show
    // background, bit 4 (16) = show sprites; 30 = both plus the leftmost columns.
    if (engine.ppu.mask & 24) == 0 {
        engine.ppu.mask = 30;
    }
    // Ensure the NMI-enable / 8x16 sprite control bit (8) is set for rendering.
    engine.ppu.ctrl |= 8;
    // Render the final frame and report a lit-pixel sanity count.
    let fb = common::render_frame(&mut engine);
    common::write_ppm("build/game_frame.ppm", &fb)?;
    let lit = fb
        .chunks_exact(3)
        .filter(|px| px.iter().any(|c| *c != 0))
        .count();
    eprintln!("rendered build/game_frame.ppm ({lit} lit pixels)");
    Ok(())
}
