mod common;

use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let replay = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("fixtures/reference/outside_walk.replay");
    let out_dir = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("build/port_capture/replay");
    let frames_csv = args.get(4).map(String::as_str).unwrap_or("1,60,120,180");

    let input = common::parse_replay(replay)?;
    let (capture, max_capture) = common::parse_capture_set(frames_csv)?;
    let max_frame = input.len().saturating_sub(1).max(max_capture);
    let mut engine = common::load_rom(rom, true)?;

    let apu_frame = Arc::new(Mutex::new(0usize));
    let apu_trace = if let Ok(path) = env::var("LOTW_ROUTINE_APU_TRACE") {
        if path.is_empty() {
            None
        } else {
            common::ensure_parent(&path)?;
            let mut f = File::create(path)?;
            writeln!(f, "frame\taddr\tvalue")?;
            Some(Arc::new(Mutex::new(f)))
        }
    } else {
        None
    };
    if let Some(trace) = apu_trace.clone() {
        let apu_frame = Arc::clone(&apu_frame);
        engine.set_apu_trace(move |addr, value| {
            let frame = *apu_frame.lock().expect("apu frame mutex poisoned");
            let mut f = trace.lock().expect("apu trace mutex poisoned");
            let _ = writeln!(f, "{frame}\t{addr:04X}\t{value:02X}");
        });
    }

    let mut runner = common::start_runner(engine)?;
    for frame in 1..=max_frame {
        *apu_frame.lock().unwrap() = frame;
        let buttons = input.get(frame).copied().unwrap_or(0);
        runner.engine_mut().ppu.set_buttons(buttons as i32);
        if !common::step_frame(&mut runner) {
            eprintln!("game loop returned at frame {frame}");
            break;
        }
        let fb = common::render_frame(runner.engine_mut());
        if capture.contains(&frame) {
            let prefix = format!("{out_dir}/frame_{frame:06}");
            common::write_ppm(format!("{prefix}.ppm"), &fb)?;
            common::write_ram(format!("{out_dir}/ram_{frame:06}.bin"), runner.engine())?;
            common::write_ppu_state(format!("{out_dir}/ppu_{frame:06}.bin"), runner.engine())?;
            let e = runner.engine();
            eprintln!(
                "captured f{frame} char={:02X} map={:02X},{:02X} px={:02X} py={:02X} scroll={:02X},{:02X} song={:02X} item={:02X} inv0={:02X} mirror={}",
                e.state.character_index(),
                e.state.map_screen_x(),
                e.state.map_screen_y(),
                e.state.player_x_tile(),
                e.state.player_y(),
                e.state.scroll_pixel_x(),
                e.mem(0x1e),
                e.state.song(),
                e.state.selected_item_slot(),
                e.mem(0x60),
                e.ppu.mirror,
            );
        }
    }
    Ok(())
}
