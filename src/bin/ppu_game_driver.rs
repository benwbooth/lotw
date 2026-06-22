mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    common::init_game_scene(&mut engine, &mut r);
    engine.set_mem(0x7c, 0x10);
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    engine.set_mem(0x7c, 0x20);
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    game::upload_status_panel_template(&mut engine, &mut r);
    game::upload_inventory_item_list(&mut engine, &mut r);
    for fr in 0..4 {
        eprintln!("frame {fr}: game_update...");
        engine.set_mem(0x36, 0x01);
        game::game_update(&mut engine, &mut r);
        game::update_player_projectiles(&mut engine, &mut r);
        game::update_room_actors(&mut engine, &mut r);
        game::update_tile_projectile(&mut engine, &mut r);
        game::update_camera_scroll_from_player(&mut engine, &mut r);
        game::draw_player_sprites(&mut engine, &mut r);
        game::draw_room_object_sprites(&mut engine, &mut r);
        lotw::native::commit_foreground_frame_and_wait(&mut engine, &mut r);
    }
    if (engine.ppu.mask & 0x18) == 0 {
        engine.ppu.mask = 0x1e;
    }
    engine.ppu.ctrl |= 0x08;
    let fb = common::render_frame(&mut engine);
    common::write_ppm("build/game_frame.ppm", &fb)?;
    let lit = fb
        .chunks_exact(3)
        .filter(|px| px.iter().any(|c| *c != 0))
        .count();
    eprintln!("rendered build/game_frame.ppm ({lit} lit pixels)");
    Ok(())
}
