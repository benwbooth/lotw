mod common;

use std::{env, error::Error};

use lotw::{RoutineContext, game};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let btn = args
        .get(2)
        .and_then(|s| i32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0x80);
    let frames: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(48);

    let mut engine = common::load_rom(rom, false)?;
    let mut r = RoutineContext::default();
    common::init_game_scene(&mut engine, &mut r);
    engine.set_mem(0x7c, 0x10);
    game::routine_0081(&mut engine, &mut r);
    game::routine_0060(&mut engine, &mut r);
    engine.set_mem(0x7c, 0x20);
    game::routine_0081(&mut engine, &mut r);
    game::routine_0060(&mut engine, &mut r);
    game::routine_0076(&mut engine, &mut r);
    game::upload_inventory_item_list(&mut engine, &mut r);
    engine.set_mem(0x8e, 0);
    engine.set_mem(0x8d, 0);
    game::song_init(&mut engine, &mut r);

    for fr in 0..frames {
        engine.ppu.set_buttons(btn);
        engine.set_mem(0x36, 1);
        game::read_controllers(&mut engine, &mut r);
        game::game_update(&mut engine, &mut r);
        game::update_player_projectiles(&mut engine, &mut r);
        game::update_room_actors(&mut engine, &mut r);
        game::update_tile_projectile(&mut engine, &mut r);
        game::routine_0059(&mut engine, &mut r);
        game::routine_0061(&mut engine, &mut r);
        game::routine_0063(&mut engine, &mut r);
        lotw::native::commit_foreground_frame_and_wait(&mut engine, &mut r);
        game::sound_tick(&mut engine, &mut r);
        if (engine.ppu.mask & 0x18) == 0 {
            engine.ppu.mask = 0x1e;
        }
        engine.ppu.ctrl |= 0x08;
        let fb = common::render_frame(&mut engine);
        common::write_ppm(format!("build/anim/f{fr:03}.ppm"), &fb)?;
        eprintln!(
            "frame {fr}: player x_tile={:02X} y={:02X} input20={:02X}",
            engine.mem(0x44),
            engine.mem(0x45),
            engine.mem(0x20)
        );
    }
    Ok(())
}
