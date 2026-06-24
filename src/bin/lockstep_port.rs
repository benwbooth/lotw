//! Per-frame RAM trace dumper for byte-exact verification of the Rust port.
//!
//! Boots the game and runs it for a fixed number of frames, appending the full
//! 2 KiB of work RAM after every frame to an output file. The resulting trace
//! can be diffed against a reference trace (e.g. from the original NES / a
//! reference emulator) to confirm the port is byte-for-byte identical. An
//! optional recorded-input file can be supplied to drive controller input
//! deterministically.
//!
//! Usage: `lockstep_port [rom] [frames] [out] [input]`
//! - `rom`    ROM path (default `rom/lotw.nes`)
//! - `frames` number of frames to run (default 2000)
//! - `out`    output trace path (default `/tmp/port_trace.bin`)
//! - `input`  optional binary file of per-frame controller bytes

mod common;

use std::{env, error::Error, fs::File, io::Write};

/// Boot the ROM, run `frames` frames, and append each frame's 2 KiB of RAM to
/// the output file. Returns an error on I/O or boot failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments, falling back to defaults.
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2000);
    let outp = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("/tmp/port_trace.bin");
    let input_path = args.get(4).map(String::as_str);

    // Load the ROM with the hardware power-on RAM pattern (matches reference).
    let mut engine = common::load_rom(rom, true)?;
    // If an input file was given, install it as the deterministic input source
    // and keep the consumed-count handle so we can report it at the end.
    let consumed = if let Some(input_path) = input_path {
        let input = common::read_binary(input_path)?;
        let (next, consumed) = common::input_closure(input);
        engine.set_next_input(next);
        Some(consumed)
    } else {
        None
    };

    // Boot the game and open the output trace file.
    let mut runner = common::start_runner(engine)?;
    common::ensure_parent(outp)?;
    let mut out = File::create(outp)?;
    // Step one frame at a time, appending the 2 KiB of internal RAM each frame.
    for frame in 0..frames {
        if !common::step_frame(&mut runner) {
            eprintln!("lockstep_port: game loop returned at frame {frame}");
            break;
        }
        // Only the first 2048 bytes are the NES internal work RAM.
        out.write_all(runner.engine().state.ram_bytes()[..2048].as_ref())?;
    }
    // Report how many recorded inputs were consumed, if a file was supplied.
    if let Some(consumed) = consumed {
        eprintln!(
            "lockstep_port: consumed {} input reads",
            *consumed.lock().unwrap()
        );
    }
    eprintln!("lockstep_port: wrote {frames} frames x 2048 to {outp}");
    Ok(())
}
