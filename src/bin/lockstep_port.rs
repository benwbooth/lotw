mod common;

use std::{env, error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2000);
    let outp = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("/tmp/port_trace.bin");
    let input_path = args.get(4).map(String::as_str);

    let mut engine = common::load_rom(rom, true)?;
    let consumed = if let Some(input_path) = input_path {
        let input = common::read_binary(input_path)?;
        let (next, consumed) = common::input_closure(input);
        engine.set_next_input(next);
        Some(consumed)
    } else {
        None
    };

    let mut runner = common::start_runner(engine)?;
    common::ensure_parent(outp)?;
    let mut out = File::create(outp)?;
    for frame in 0..frames {
        if !common::step_frame(&mut runner) {
            eprintln!("lockstep_port: game loop returned at frame {frame}");
            break;
        }
        out.write_all(&runner.engine().memory[..0x800])?;
    }
    if let Some(consumed) = consumed {
        eprintln!(
            "lockstep_port: consumed {} input reads",
            *consumed.lock().unwrap()
        );
    }
    eprintln!("lockstep_port: wrote {frames} frames x 0x800 to {outp}");
    Ok(())
}
