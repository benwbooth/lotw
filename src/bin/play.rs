//! SDL3 playable front-end for the Legacy of the Wizard port.
//!
//! This is the interactive binary: it boots the ROM, opens an SDL window plus an
//! audio device, and runs the game in real time. Each iteration of the frame
//! loop polls input (keyboard plus any connected gamepads/joysticks), maps it to
//! the NES controller bitmask, advances the game by one frame, streams the
//! frame's audio to the device, and renders/presents the PPU framebuffer to the
//! window, pacing itself to ~60 fps.
//!
//! The game itself runs as a stackful coroutine driven by [`common::step_frame`]
//! (the runner uses `black_box` barriers to defeat a release-build aliasing
//! miscompile; see `src/frame.rs`). This file only deals with the SDL render /
//! present / input loop around that runner.
//!
//! Usage: `play [rom] [max_frames] [auto]`
//! - `rom`        ROM path (default `rom/lotw.nes`)
//! - `max_frames` if non-zero, quit after this many frames and dump a final PPM
//! - `auto`       the literal string `auto` to run a scripted demo input

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

/// Target wall-clock duration of one frame: one second (1e9 ns) divided by 60 fps.
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / 60);
/// Analog stick threshold (out of i16 range, ~37%) past which a direction counts
/// as pressed, so small resting offsets don't register as movement.
const STICK_DEAD_ZONE: i16 = 12_000;

/// Holds the SDL input subsystems and the currently-open input devices.
///
/// `gamepad`/`joystick` are the subsystem handles (if SDL could initialize them);
/// `gamepads`/`joysticks` are the devices opened from those subsystems and are
/// refreshed whenever a device is added or removed.
struct InputDevices {
    gamepad: Option<GamepadSubsystem>,
    joystick: Option<JoystickSubsystem>,
    gamepads: Vec<Gamepad>,
    joysticks: Vec<Joystick>,
}

impl InputDevices {
    /// Initialize the gamepad and joystick subsystems and open all currently
    /// connected devices. Subsystems that fail to init are simply left absent.
    fn new(sdl: &sdl3::Sdl) -> Self {
        // Try to acquire both subsystems; either may be unavailable.
        let gamepad = sdl.gamepad().ok();
        let joystick = sdl.joystick().ok();
        let mut devices = Self {
            gamepad,
            joystick,
            gamepads: Vec::new(),
            joysticks: Vec::new(),
        };
        // Populate the open-device lists from whatever is currently plugged in.
        devices.refresh();
        devices
    }

    /// Re-enumerate and re-open all connected gamepads and joysticks.
    ///
    /// Called at startup and whenever a device-added/removed event arrives, so
    /// the open-device lists track hot-plugged controllers. With
    /// `LOTW_INPUT_DEBUG` set, prints the discovered devices to stderr.
    fn refresh(&mut self) {
        // Re-open every gamepad the subsystem reports as connected.
        self.gamepads = self
            .gamepad
            .as_ref()
            .and_then(|gamepad| gamepad.gamepads().ok())
            .into_iter()
            .flatten()
            .filter_map(|id| self.gamepad.as_ref()?.open(id).ok())
            .collect();

        // Re-open every joystick the subsystem reports as connected.
        self.joysticks = self
            .joystick
            .as_ref()
            .and_then(|joystick| joystick.joysticks().ok())
            .into_iter()
            .flatten()
            .filter_map(|id| self.joystick.as_ref()?.open(id).ok())
            .collect();

        // Optional verbose logging of the discovered devices.
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

    /// Poll SDL for fresh device state on both subsystems before reading buttons.
    fn update(&self) {
        if let Some(gamepad) = &self.gamepad {
            gamepad.update();
        }
        if let Some(joystick) = &self.joystick {
            joystick.update();
        }
    }

    /// Combine all gamepad and joystick input into a single NES button bitmask.
    fn buttons(&self) -> i32 {
        gamepad_buttons(&self.gamepads) | joystick_buttons(&self.joysticks)
    }
}

/// Map every connected gamepad's buttons and left stick to the NES bitmask.
///
/// The result ORs together all gamepads. D-pad presses or a left-stick deflection
/// past [`STICK_DEAD_ZONE`] set the direction bits; the face/start/back buttons
/// set A/B/Start/Select. Bit layout matches the NES latch order (see
/// [`common`]'s `button_bit`): Right=128, Left=64, Down=32, Up=16, Start=8,
/// Select=4, B=2, A=1.
fn gamepad_buttons(gamepads: &[Gamepad]) -> i32 {
    let mut buttons = 0;
    for gamepad in gamepads.iter().filter(|gamepad| gamepad.connected()) {
        // D-pad or left-stick X/Y, then the four action buttons.
        if gamepad.button(Button::DPadRight) || gamepad.axis(Axis::LeftX) > STICK_DEAD_ZONE {
            buttons |= 128; // Right
        }
        if gamepad.button(Button::DPadLeft) || gamepad.axis(Axis::LeftX) < -STICK_DEAD_ZONE {
            buttons |= 64; // Left
        }
        if gamepad.button(Button::DPadDown) || gamepad.axis(Axis::LeftY) > STICK_DEAD_ZONE {
            buttons |= 32; // Down
        }
        if gamepad.button(Button::DPadUp) || gamepad.axis(Axis::LeftY) < -STICK_DEAD_ZONE {
            buttons |= 16; // Up
        }
        if gamepad.button(Button::Start) {
            buttons |= 8; // Start
        }
        if gamepad.button(Button::Back) {
            buttons |= 4; // Select
        }
        if gamepad.button(Button::East) {
            buttons |= 2; // B (right face button)
        }
        if gamepad.button(Button::South) {
            buttons |= 1; // A (bottom face button)
        }
    }
    buttons
}

/// Read button index `button` on `joystick`, guarding against out-of-range indices.
///
/// Returns `false` if the joystick has fewer buttons than `button` or the read
/// fails.
fn joystick_button(joystick: &Joystick, button: u32) -> bool {
    button < joystick.num_buttons() && joystick.button(button).unwrap_or(false)
}

/// Read axis index `axis` on `joystick`, returning 0 for out-of-range axes.
fn joystick_axis(joystick: &Joystick, axis: u32) -> i16 {
    if axis < joystick.num_axes() {
        joystick.axis(axis).unwrap_or(0)
    } else {
        0
    }
}

/// Test whether any hat (POV) switch on `joystick` is pressed in the `wanted`
/// direction(s).
///
/// `wanted` is an SDL [`HatState`] bitmask; returns true if any hat's state
/// shares a bit with it.
fn joystick_hat_pressed(joystick: &Joystick, wanted: u8) -> bool {
    (0..joystick.num_hats()).any(|hat| {
        joystick
            .hat(hat)
            .is_ok_and(|state| (state as u8 & wanted) != 0)
    })
}

/// Map every connected raw joystick (no SDL gamepad mapping) to the NES bitmask.
///
/// Generic joysticks expose only numbered buttons/axes/hats, so this uses fixed
/// indices: hat or axis 0/1 (and 6/7 for a second stick) drive the directions,
/// and numbered buttons drive the action buttons. Results from all joysticks are
/// ORed together. Bit layout matches [`gamepad_buttons`].
fn joystick_buttons(joysticks: &[Joystick]) -> i32 {
    let mut buttons = 0;
    for joystick in joysticks.iter().filter(|joystick| joystick.connected()) {
        // Directions: hat switch, or axis 0/1 (primary stick) / 6/7 (secondary).
        if joystick_hat_pressed(joystick, HatState::Right as u8)
            || joystick_axis(joystick, 0) > STICK_DEAD_ZONE
            || joystick_axis(joystick, 6) > STICK_DEAD_ZONE
        {
            buttons |= 128; // Right
        }
        if joystick_hat_pressed(joystick, HatState::Left as u8)
            || joystick_axis(joystick, 0) < -STICK_DEAD_ZONE
            || joystick_axis(joystick, 6) < -STICK_DEAD_ZONE
        {
            buttons |= 64; // Left
        }
        if joystick_hat_pressed(joystick, HatState::Down as u8)
            || joystick_axis(joystick, 1) > STICK_DEAD_ZONE
            || joystick_axis(joystick, 7) > STICK_DEAD_ZONE
        {
            buttons |= 32; // Down
        }
        if joystick_hat_pressed(joystick, HatState::Up as u8)
            || joystick_axis(joystick, 1) < -STICK_DEAD_ZONE
            || joystick_axis(joystick, 7) < -STICK_DEAD_ZONE
        {
            buttons |= 16; // Up
        }
        // Action buttons: accept a couple of common numbered-button layouts.
        if joystick_button(joystick, 7) || joystick_button(joystick, 9) {
            buttons |= 8; // Start
        }
        if joystick_button(joystick, 6) || joystick_button(joystick, 8) {
            buttons |= 4; // Select
        }
        if joystick_button(joystick, 1) || joystick_button(joystick, 2) {
            buttons |= 2; // B
        }
        if joystick_button(joystick, 0) {
            buttons |= 1; // A
        }
    }
    buttons
}

/// Boot the ROM and run the interactive SDL game loop until quit (or until the
/// optional frame cap is hit). Returns an error on SDL/IO setup failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments: ROM path, optional frame cap, optional scripted-demo flag.
    let args: Vec<String> = env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let max_frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let autostart = args.get(3).is_some_and(|s| s == "auto");

    // Load the ROM (with power-on RAM pattern) and boot the game coroutine.
    let engine = common::load_rom(rom, true)?;
    let mut runner = common::start_runner(engine)?;

    // Joystick hints: run polling on its own thread and enable the HIDAPI
    // backend (including Steam controllers) before SDL is initialized.
    sdl3::hint::set("SDL_JOYSTICK_THREAD", "1");
    sdl3::hint::set("SDL_JOYSTICK_HIDAPI", "1");
    sdl3::hint::set("SDL_JOYSTICK_HIDAPI_STEAM", "1");

    // Initialize SDL and optionally log its version for input debugging.
    let sdl = sdl3::init()?;
    let input_debug = env::var_os("LOTW_INPUT_DEBUG").is_some();
    if input_debug {
        eprintln!("input: SDL {}", sdl3::version::version());
    }
    // Create a resizable window at 3x the NES resolution.
    let video = sdl.video()?;
    let window = video
        .window(
            "Legacy of the Wizard",
            (PPU_W * 3) as u32, // 3x horizontal scale
            (PPU_H * 3) as u32, // 3x vertical scale
        )
        .resizable()
        .build()?;
    // Render through a canvas whose logical size is the native NES resolution,
    // letterboxed into the (resizable) window so the aspect ratio is preserved.
    let mut canvas = window.into_canvas();
    canvas.set_logical_size(
        PPU_W as u32,
        PPU_H as u32,
        SDL_LOGICAL_PRESENTATION_LETTERBOX,
    )?;
    // Streaming RGB24 texture at native resolution; nearest-neighbour scaling
    // keeps pixels crisp when upscaled.
    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.create_texture_streaming(PixelFormat::RGB24, PPU_W as u32, PPU_H as u32)?;
    texture.set_scale_mode(ScaleMode::Nearest);

    // Open a mono, 16-bit playback stream at the APU's sample rate and start it.
    let audio = sdl.audio()?;
    let desired = AudioSpec {
        freq: Some(lotw::engine::APU_SR as i32),
        channels: Some(1),                    // mono
        format: Some(AudioFormat::s16_sys()), // signed 16-bit, native endianness
    };
    let audio_stream = audio
        .default_playback_device()
        .open_device_stream(Some(&desired))
        .ok();
    if let Some(stream) = &audio_stream {
        let _ = stream.resume();
    }

    // Per-run state: SDL event pump, input devices, reusable framebuffer/audio
    // buffers, frame counter, frame-pacing clock, and last-logged button value.
    let mut event_pump = sdl.event_pump()?;
    let mut input_devices = InputDevices::new(&sdl);
    let mut fb = vec![0; PPU_W * PPU_H * 3];
    let mut audio_buf = vec![0i16; common::SPF];
    let mut running = true;
    let mut frames = 0usize;
    let mut next_frame = Instant::now();
    let mut last_buttons = -1; // sentinel so the first input always logs
    // Rolling buffer of recent framebuffers. Press F12 to dump the last frames
    // to build/jank_NN.ppm for diagnosing transient visual glitches (the glitch
    // is usually a frame or two before the keypress, hence the history).
    let mut frame_ring: std::collections::VecDeque<Vec<u8>> = std::collections::VecDeque::new();
    let mut capture_request = false;
    // Per-frame log of the NES button byte, dumped on F12 so the exact session
    // can be replayed deterministically in another build to diff state.
    let mut input_log: Vec<u8> = Vec::new();

    // Main frame loop: input -> step game -> audio -> render/present -> pace.
    while running {
        // Drain pending SDL events: quit on window close or Ctrl+Q, and
        // re-enumerate input devices when one is plugged in or removed.
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
                // F12: request a dump of the recent-frame ring buffer.
                Event::KeyDown {
                    keycode: Some(Keycode::F12),
                    ..
                } => {
                    capture_request = true;
                }
                _ => {}
            }
        }

        // Build this frame's NES button mask from the keyboard. Bit layout
        // matches the controllers: Right=128, Left=64, Down=32, Up=16, Start=8,
        // Select=4, B=2, A=1. Keys: arrows = d-pad, Enter = Start, RShift =
        // Select, X = B, Z = A.
        let keyboard = event_pump.keyboard_state();
        let mut buttons = 0;
        if keyboard.is_scancode_pressed(Scancode::Right) {
            buttons |= 128; // Right
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            buttons |= 64; // Left
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            buttons |= 32; // Down
        }
        if keyboard.is_scancode_pressed(Scancode::Up) {
            buttons |= 16; // Up
        }
        if keyboard.is_scancode_pressed(Scancode::Return) {
            buttons |= 8; // Start
        }
        if keyboard.is_scancode_pressed(Scancode::RShift) {
            buttons |= 4; // Select
        }
        if keyboard.is_scancode_pressed(Scancode::X) {
            buttons |= 2; // B
        }
        if keyboard.is_scancode_pressed(Scancode::Z) {
            buttons |= 1; // A
        }
        // Fold in gamepad/joystick input on top of the keyboard.
        input_devices.update();
        buttons |= input_devices.buttons();
        // In debug mode, log the combined button mask only when it changes.
        if input_debug && buttons != last_buttons {
            eprintln!("input: nes buttons {buttons:02X}");
            last_buttons = buttons;
        }
        // Scripted demo input (the `auto` flag): tap Start to begin, then cycle
        // through directions and occasional A/Start presses to drive the player.
        if autostart {
            if (150..168).contains(&frames) {
                buttons |= 8; // hold Start over frames 150-167 to start the game
            } else if frames >= 200 {
                // From frame 200 on, advance through a 6-step cycle every 45 frames.
                let seg = (frames - 200) / 45;
                match seg % 6 {
                    0 => buttons |= 64,                           // Left
                    1 => buttons |= 128,                          // Right
                    2 => buttons |= 16,                           // Up
                    3 => buttons |= 32,                           // Down
                    4 if (frames - 200) % 45 < 6 => buttons |= 1, // brief A tap
                    5 if (frames - 200) % 45 < 6 => buttons |= 8, // brief Start tap
                    _ => {}
                }
            }
        }
        // Latch the assembled input into the engine for this frame, and record
        // it for deterministic replay.
        runner.engine_mut().ppu.buttons = buttons as u8;
        input_log.push(buttons as u8);

        // Advance the game by one frame; stop if the game loop returned.
        if !common::step_frame(&mut runner) {
            eprintln!("game loop returned at frame {frames}");
            break;
        }

        // Generate this frame's audio and queue it, but only if the device's
        // backlog is small (under 4 frames' worth) so we don't run far ahead.
        runner.engine_mut().apu.frame();
        runner.engine_mut().apu.generate(&mut audio_buf);
        if let Some(stream) = &audio_stream {
            if stream.queued_bytes().unwrap_or(0)
                < (audio_buf.len() * std::mem::size_of::<i16>() * 4) as i32
            // 4 frames of headroom
            {
                let _ = stream.put_data_i16(&audio_buf);
            }
        }

        // Render the PPU into the framebuffer, upload it to the texture, and
        // present it (pitch is PPU_W * 3 bytes per row for RGB24).
        let memory = runner.engine().state.ram_bytes().to_vec();
        runner.engine_mut().ppu.render(&memory, &mut fb);
        texture.update(None, &fb, PPU_W * 3)?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        // Keep the last 16 rendered frames; on an F12 request, dump them so a
        // transient glitch (which is usually a frame or two before the keypress)
        // is captured in build/jank_NN.ppm for offline analysis.
        frame_ring.push_back(fb.clone());
        if frame_ring.len() > 16 {
            frame_ring.pop_front();
        }
        if capture_request {
            capture_request = false;
            for (i, frame) in frame_ring.iter().enumerate() {
                common::write_ppm(&format!("build/jank_{i:02}.ppm"), frame)?;
            }
            // Also dump the current RAM image ($0000-$07FF) so the exact engine
            // state behind the glitch (OAM at $0200, CHR-bank shadows $2A-$2F)
            // can be inspected offline.
            std::fs::write(
                "build/jank_ram.bin",
                &runner.engine().state.ram_bytes()[..0x800],
            )?;
            // Dump the full input history so this exact session can be replayed
            // deterministically in another build to diff state.
            std::fs::write("build/input.bin", &input_log)?;
            eprintln!(
                "capture: wrote {} frames + RAM + {}-frame input log",
                frame_ring.len(),
                input_log.len()
            );
        }

        // Advance the frame counter; honour the optional frame cap, dumping a
        // final image before quitting.
        frames += 1;
        if max_frames != 0 && frames >= max_frames {
            common::write_ppm("build/boot_last.ppm", &fb)?;
            running = false;
        }

        // Frame pacing: sleep until the next ~60 fps deadline, but if we've
        // fallen behind, reset the deadline to now rather than busy-catching-up.
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
