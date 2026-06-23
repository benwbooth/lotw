mod common;

use std::{
    env,
    error::Error,
    time::{Duration, Instant},
};

use lotw::{PPU_H, PPU_W};
use sdl3::{
    GamepadSubsystem, JoystickSubsystem,
    audio::{AudioFormat, AudioSpec},
    event::Event,
    gamepad::{Axis, Button, Gamepad},
    joystick::{HatState, Joystick},
    keyboard::{Keycode, Scancode},
    pixels::PixelFormat,
    render::ScaleMode,
    sys::render::SDL_LOGICAL_PRESENTATION_LETTERBOX,
};

const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / 60);
const STICK_DEAD_ZONE: i16 = 12_000;

struct InputDevices {
    gamepad: Option<GamepadSubsystem>,
    joystick: Option<JoystickSubsystem>,
    gamepads: Vec<Gamepad>,
    joysticks: Vec<Joystick>,
}

impl InputDevices {
    fn new(sdl: &sdl3::Sdl) -> Self {
        let gamepad = sdl.gamepad().ok();
        let joystick = sdl.joystick().ok();
        let mut devices = Self {
            gamepad,
            joystick,
            gamepads: Vec::new(),
            joysticks: Vec::new(),
        };
        devices.refresh();
        devices
    }

    fn refresh(&mut self) {
        self.gamepads = self
            .gamepad
            .as_ref()
            .and_then(|gamepad| gamepad.gamepads().ok())
            .into_iter()
            .flatten()
            .filter_map(|id| self.gamepad.as_ref()?.open(id).ok())
            .collect();

        self.joysticks = self
            .joystick
            .as_ref()
            .and_then(|joystick| joystick.joysticks().ok())
            .into_iter()
            .flatten()
            .filter_map(|id| self.joystick.as_ref()?.open(id).ok())
            .collect();

        if env::var_os("LOTW_INPUT_DEBUG").is_some() {
            eprintln!(
                "input: opened {} gamepad(s), {} joystick(s)",
                self.gamepads.len(),
                self.joysticks.len()
            );
            for gamepad in &self.gamepads {
                eprintln!(
                    "input: gamepad {}",
                    gamepad.name().unwrap_or_else(|| "(unnamed)".to_string())
                );
            }
            for joystick in &self.joysticks {
                eprintln!("input: joystick {}", joystick.name());
            }
        }
    }

    fn update(&self) {
        if let Some(gamepad) = &self.gamepad {
            gamepad.update();
        }
        if let Some(joystick) = &self.joystick {
            joystick.update();
        }
    }

    fn buttons(&self) -> i32 {
        gamepad_buttons(&self.gamepads) | joystick_buttons(&self.joysticks)
    }
}

fn gamepad_buttons(gamepads: &[Gamepad]) -> i32 {
    let mut buttons = 0;
    for gamepad in gamepads.iter().filter(|gamepad| gamepad.connected()) {
        if gamepad.button(Button::DPadRight) || gamepad.axis(Axis::LeftX) > STICK_DEAD_ZONE {
            buttons |= 128;
        }
        if gamepad.button(Button::DPadLeft) || gamepad.axis(Axis::LeftX) < -STICK_DEAD_ZONE {
            buttons |= 64;
        }
        if gamepad.button(Button::DPadDown) || gamepad.axis(Axis::LeftY) > STICK_DEAD_ZONE {
            buttons |= 32;
        }
        if gamepad.button(Button::DPadUp) || gamepad.axis(Axis::LeftY) < -STICK_DEAD_ZONE {
            buttons |= 16;
        }
        if gamepad.button(Button::Start) {
            buttons |= 8;
        }
        if gamepad.button(Button::Back) {
            buttons |= 4;
        }
        if gamepad.button(Button::East) {
            buttons |= 2;
        }
        if gamepad.button(Button::South) {
            buttons |= 1;
        }
    }
    buttons
}

fn joystick_button(joystick: &Joystick, button: u32) -> bool {
    button < joystick.num_buttons() && joystick.button(button).unwrap_or(false)
}

fn joystick_axis(joystick: &Joystick, axis: u32) -> i16 {
    if axis < joystick.num_axes() {
        joystick.axis(axis).unwrap_or(0)
    } else {
        0
    }
}

fn joystick_hat_pressed(joystick: &Joystick, wanted: u8) -> bool {
    (0..joystick.num_hats()).any(|hat| {
        joystick
            .hat(hat)
            .is_ok_and(|state| (state as u8 & wanted) != 0)
    })
}

fn joystick_buttons(joysticks: &[Joystick]) -> i32 {
    let mut buttons = 0;
    for joystick in joysticks.iter().filter(|joystick| joystick.connected()) {
        if joystick_hat_pressed(joystick, HatState::Right as u8)
            || joystick_axis(joystick, 0) > STICK_DEAD_ZONE
            || joystick_axis(joystick, 6) > STICK_DEAD_ZONE
        {
            buttons |= 128;
        }
        if joystick_hat_pressed(joystick, HatState::Left as u8)
            || joystick_axis(joystick, 0) < -STICK_DEAD_ZONE
            || joystick_axis(joystick, 6) < -STICK_DEAD_ZONE
        {
            buttons |= 64;
        }
        if joystick_hat_pressed(joystick, HatState::Down as u8)
            || joystick_axis(joystick, 1) > STICK_DEAD_ZONE
            || joystick_axis(joystick, 7) > STICK_DEAD_ZONE
        {
            buttons |= 32;
        }
        if joystick_hat_pressed(joystick, HatState::Up as u8)
            || joystick_axis(joystick, 1) < -STICK_DEAD_ZONE
            || joystick_axis(joystick, 7) < -STICK_DEAD_ZONE
        {
            buttons |= 16;
        }
        if joystick_button(joystick, 7) || joystick_button(joystick, 9) {
            buttons |= 8;
        }
        if joystick_button(joystick, 6) || joystick_button(joystick, 8) {
            buttons |= 4;
        }
        if joystick_button(joystick, 1) || joystick_button(joystick, 2) {
            buttons |= 2;
        }
        if joystick_button(joystick, 0) {
            buttons |= 1;
        }
    }
    buttons
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let max_frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let autostart = args.get(3).is_some_and(|s| s == "auto");

    let engine = common::load_rom(rom, true)?;
    let mut runner = common::start_runner(engine)?;

    sdl3::hint::set("SDL_JOYSTICK_THREAD", "1");
    sdl3::hint::set("SDL_JOYSTICK_HIDAPI", "1");
    sdl3::hint::set("SDL_JOYSTICK_HIDAPI_STEAM", "1");

    let sdl = sdl3::init()?;
    let input_debug = env::var_os("LOTW_INPUT_DEBUG").is_some();
    if input_debug {
        eprintln!("input: SDL {}", sdl3::version::version());
    }
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
    let mut input_devices = InputDevices::new(&sdl);
    let mut fb = vec![0; PPU_W * PPU_H * 3];
    let mut audio_buf = vec![0i16; common::SPF];
    let mut running = true;
    let mut frames = 0usize;
    let mut next_frame = Instant::now();
    let mut last_buttons = -1;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::ControllerDeviceAdded { .. }
                | Event::ControllerDeviceRemoved { .. }
                | Event::JoyDeviceAdded { .. }
                | Event::JoyDeviceRemoved { .. } => input_devices.refresh(),
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    keymod,
                    ..
                } if keymod
                    .intersects(sdl3::keyboard::Mod::LCTRLMOD | sdl3::keyboard::Mod::RCTRLMOD) =>
                {
                    running = false;
                }
                _ => {}
            }
        }

        let keyboard = event_pump.keyboard_state();
        let mut buttons = 0;
        if keyboard.is_scancode_pressed(Scancode::Right) {
            buttons |= 128;
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            buttons |= 64;
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            buttons |= 32;
        }
        if keyboard.is_scancode_pressed(Scancode::Up) {
            buttons |= 16;
        }
        if keyboard.is_scancode_pressed(Scancode::Return) {
            buttons |= 8;
        }
        if keyboard.is_scancode_pressed(Scancode::RShift) {
            buttons |= 4;
        }
        if keyboard.is_scancode_pressed(Scancode::X) {
            buttons |= 2;
        }
        if keyboard.is_scancode_pressed(Scancode::Z) {
            buttons |= 1;
        }
        input_devices.update();
        buttons |= input_devices.buttons();
        if input_debug && buttons != last_buttons {
            eprintln!("input: nes buttons {buttons:02X}");
            last_buttons = buttons;
        }
        if autostart {
            if (150..168).contains(&frames) {
                buttons |= 8;
            } else if frames >= 200 {
                let seg = (frames - 200) / 45;
                match seg % 6 {
                    0 => buttons |= 64,
                    1 => buttons |= 128,
                    2 => buttons |= 16,
                    3 => buttons |= 32,
                    4 if (frames - 200) % 45 < 6 => buttons |= 1,
                    5 if (frames - 200) % 45 < 6 => buttons |= 8,
                    _ => {}
                }
            }
        }
        runner.engine_mut().ppu.buttons = buttons as u8;

        if !common::step_frame(&mut runner) {
            eprintln!("game loop returned at frame {frames}");
            break;
        }

        runner.engine_mut().apu.frame();
        runner.engine_mut().apu.generate(&mut audio_buf);
        if let Some(stream) = &audio_stream {
            if stream.queued_bytes().unwrap_or(0)
                < (audio_buf.len() * std::mem::size_of::<i16>() * 4) as i32
            {
                let _ = stream.put_data_i16(&audio_buf);
            }
        }

        let memory = runner.engine().state.ram_bytes().to_vec();
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

        next_frame += FRAME_TIME;
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
