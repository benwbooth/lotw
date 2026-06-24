//! Replay-driven capture tool for verifying the port against reference frames.
//!
//! Replays a recorded button sequence (a `.replay` file) through the booted
//! game and, at a chosen set of frame numbers, dumps the rendered image (PPM),
//! the work RAM, and the full PPU state so they can be compared against captured
//! reference data. Optionally, setting the `LOTW_ROUTINE_APU_TRACE` environment
//! variable to a file path records every APU register write (with its frame) to
//! that file as a TSV trace.
//!
//! Usage: `replay_capture_port [rom] [replay] [out_dir] [frames_csv]`
//! - `rom`        ROM path (default `rom/lotw.nes`)
//! - `replay`     replay file (default `fixtures/reference/outside_walk.replay`)
//! - `out_dir`    output directory (default `build/port_capture/replay`)
//! - `frames_csv` comma-separated 1-based frames to capture (default `1,60,120,180`)

mod common;

use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

/// Replay the input sequence, capturing image/RAM/PPU dumps at the requested
/// frames (and optionally an APU trace). Returns an error on I/O failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments with defaults.
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

    // Load the replay input and the set of frames to capture. Run far enough to
    // cover both the whole replay and the last requested capture frame.
    let input = common::parse_replay(replay)?;
    let (capture, max_capture) = common::parse_capture_set(frames_csv)?;
    let max_frame = input.len().saturating_sub(1).max(max_capture);
    let mut engine = common::load_rom(rom, true)?;

    // Shared "current frame" counter so the APU trace callback can tag each write.
    let apu_frame = Arc::new(Mutex::new(0usize));
    // If LOTW_ROUTINE_APU_TRACE is set to a non-empty path, open it as a TSV file.
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
    // Install the APU-write trace callback (logs frame, register address, value).
    if let Some(trace) = apu_trace.clone() {
        let apu_frame = Arc::clone(&apu_frame);
        engine.set_apu_trace(move |addr, value| {
            let frame = *apu_frame.lock().expect("apu frame mutex poisoned");
            let mut f = trace.lock().expect("apu trace mutex poisoned");
            let _ = writeln!(f, "{frame}\t{addr:04X}\t{value:02X}");
        });
    }

    // Boot the game, then step through each frame feeding the recorded input.
    let mut runner = common::start_runner(engine)?;
    for frame in 1..=max_frame {
        // Publish the current frame for the APU trace, then latch this frame's input.
        *apu_frame.lock().unwrap() = frame;
        let buttons = input.get(frame).copied().unwrap_or(0);
        runner.engine_mut().ppu.buttons = (buttons as u8);
        if !common::step_frame(&mut runner) {
            eprintln!("game loop returned at frame {frame}");
            break;
        }
        // Render the frame; dump artifacts only on the requested capture frames.
        let fb = common::render_frame(runner.engine_mut());
        if capture.contains(&frame) {
            // Write the rendered image, work RAM, and full PPU state side by side.
            let prefix = format!("{out_dir}/frame_{frame:06}");
            common::write_ppm(format!("{prefix}.ppm"), &fb)?;
            common::write_ram(format!("{out_dir}/ram_{frame:06}.bin"), runner.engine())?;
            common::write_ppu_state(format!("{out_dir}/ppu_{frame:06}.bin"), runner.engine())?;
            // Log a one-line summary of key game-state fields for this capture.
            let e = runner.engine();
            eprintln!(
                "captured f{frame} char={:02X} map={:02X},{:02X} px={:02X} py={:02X} scroll={:02X},{:02X} song={:02X} item={:02X} inv0={:02X} mirror={}",
                e.state.character_index,
                e.state.map_screen_x,
                e.state.map_screen_y,
                e.state.player_x_tile,
                e.state.player_y,
                e.state.scroll_pixel_x,
                e.state.scroll_y,
                e.state.song,
                e.state.selected_item_slot,
                e.state.inventory_item(0),
                e.ppu.mirror,
            );
        }
    }
    Ok(())
}
