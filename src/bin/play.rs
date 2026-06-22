mod common;

use std::{
    env,
    error::Error,
    time::{Duration, Instant},
};

use lotw::{PPU_H, PPU_W, game};
use sdl3::{
    audio::{AudioFormat, AudioSpec},
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::PixelFormat,
    render::ScaleMode,
    sys::render::SDL_LOGICAL_PRESENTATION_LETTERBOX,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let max_frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let autostart = args.get(3).is_some_and(|s| s == "auto");

    let engine = common::load_rom(rom, true)?;
    let mut runner = common::start_runner(engine)?;

    let sdl = sdl3::init()?;
    let video = sdl.video()?;
    let window = video
        .window(
            "Legacy of the Wizard",
            (PPU_W * 3) as u32,
            (PPU_H * 3) as u32,
        )
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas();
    canvas.set_logical_size(
        PPU_W as u32,
        PPU_H as u32,
        SDL_LOGICAL_PRESENTATION_LETTERBOX,
    )?;
    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.create_texture_streaming(PixelFormat::RGB24, PPU_W as u32, PPU_H as u32)?;
    texture.set_scale_mode(ScaleMode::Nearest);

    let audio = sdl.audio()?;
    let desired = AudioSpec {
        freq: Some(lotw::engine::APU_SR as i32),
        channels: Some(1),
        format: Some(AudioFormat::s16_sys()),
    };
    let audio_stream = audio
        .default_playback_device()
        .open_device_stream(Some(&desired))
        .ok();
    if let Some(stream) = &audio_stream {
        let _ = stream.resume();
    }

    let mut event_pump = sdl.event_pump()?;
    let mut fb = vec![0; PPU_W * PPU_H * 3];
    let mut audio_buf = vec![0i16; common::SPF];
    let mut running = true;
    let mut frames = 0usize;
    let mut next_frame = Instant::now();

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }

        let keyboard = event_pump.keyboard_state();
        let mut buttons = 0;
        if keyboard.is_scancode_pressed(Scancode::Right) {
            buttons |= 0x80;
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            buttons |= 0x40;
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            buttons |= 0x20;
        }
        if keyboard.is_scancode_pressed(Scancode::Up) {
            buttons |= 0x10;
        }
        if keyboard.is_scancode_pressed(Scancode::Return) {
            buttons |= 0x08;
        }
        if keyboard.is_scancode_pressed(Scancode::RShift) {
            buttons |= 0x04;
        }
        if keyboard.is_scancode_pressed(Scancode::X) {
            buttons |= 0x02;
        }
        if keyboard.is_scancode_pressed(Scancode::Z) {
            buttons |= 0x01;
        }
        if autostart {
            if (150..168).contains(&frames) {
                buttons |= 0x08;
            } else if frames >= 200 {
                let seg = (frames - 200) / 45;
                match seg % 6 {
                    0 => buttons |= 0x40,
                    1 => buttons |= 0x80,
                    2 => buttons |= 0x10,
                    3 => buttons |= 0x20,
                    4 if (frames - 200) % 45 < 6 => buttons |= 0x01,
                    5 if (frames - 200) % 45 < 6 => buttons |= 0x08,
                    _ => {}
                }
            }
        }
        runner.engine_mut().ppu.set_buttons(buttons);

        if !common::step_frame(&mut runner) {
            eprintln!("game loop returned at frame {frames}");
            break;
        }

        runner.with_engine_regs(game::sound_tick);
        runner.engine_mut().apu.frame();
        runner.engine_mut().apu.generate(&mut audio_buf);
        if let Some(stream) = &audio_stream {
            if stream.queued_bytes().unwrap_or(0)
                < (audio_buf.len() * std::mem::size_of::<i16>() * 4) as i32
            {
                let _ = stream.put_data_i16(&audio_buf);
            }
        }

        let memory = runner.engine().memory;
        runner.engine_mut().ppu.render(&memory, &mut fb);
        texture.update(None, &fb, PPU_W * 3)?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        frames += 1;
        if max_frames != 0 && frames >= max_frames {
            common::write_ppm("build/boot_last.ppm", &fb)?;
            running = false;
        }

        next_frame += Duration::from_micros(1_000_000 / 60);
        let now = Instant::now();
        if next_frame > now {
            std::thread::sleep(next_frame - now);
        } else {
            next_frame = now;
        }
    }

    eprintln!("ran {frames} frames");
    Ok(())
}
