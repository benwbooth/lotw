mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let btn = args
        .get(2)
        .and_then(|s| i32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(128);
    let frames: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(48);

    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    common::init_game_scene(&mut engine, &mut r);
    engine.state.set_scroll_tile_x(16);
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    engine.state.set_scroll_tile_x(32);
    game::upload_staged_room_columns(&mut engine, &mut r);
    game::refresh_scroll_register_shadows(&mut engine, &mut r);
    game::upload_status_panel_template(&mut engine, &mut r);
    game::upload_inventory_item_list(&mut engine, &mut r);
    engine.state.set_song(0);
    engine.state.set_sound_paused(0);
    game::song_init(&mut engine, &mut r);

    for fr in 0..frames {
        engine.ppu.set_buttons(btn);
        engine.state.set_frame_counter(1);
        game::read_controllers(&mut engine, &mut r);
        game::game_update(&mut engine, &mut r);
        game::update_player_projectiles(&mut engine, &mut r);
        game::update_room_actors(&mut engine, &mut r);
        game::update_tile_projectile(&mut engine, &mut r);
        game::update_camera_scroll_from_player(&mut engine, &mut r);
        game::draw_player_sprites(&mut engine, &mut r);
        game::draw_room_object_sprites(&mut engine, &mut r);
        lotw::native::commit_foreground_frame_and_wait(&mut engine, &mut r);
        game::sound_tick(&mut engine, &mut r);
        if (engine.ppu.mask & 24) == 0 {
            engine.ppu.mask = 30;
        }
        engine.ppu.ctrl |= 8;
        let fb = common::render_frame(&mut engine);
        common::write_ppm(format!("build/anim/f{fr:03}.ppm"), &fb)?;
        eprintln!(
            "frame {fr}: player x_tile={:02X} y={:02X} input20={:02X}",
            engine.state.player_x_tile(),
            engine.state.player_y(),
            engine.state.buttons()
        );
    }
    Ok(())
}
