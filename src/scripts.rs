#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitKind {
    ButtonsReleased,
    ButtonPressed,
    NextFrame,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wait {
    kind: WaitKind,
    mask: i32,
}

impl Wait {
    pub const fn buttons_released(mask: i32) -> Self {
        Self {
            kind: WaitKind::ButtonsReleased,
            mask,
        }
    }

    pub const fn button_pressed(mask: i32) -> Self {
        Self {
            kind: WaitKind::ButtonPressed,
            mask,
        }
    }

    pub const fn next_frame() -> Self {
        Self {
            kind: WaitKind::NextFrame,
            mask: 0,
        }
    }

    fn ready(self, buttons: i32, frame_elapsed: bool) -> bool {
        match self.kind {
            WaitKind::ButtonsReleased => (buttons & self.mask) == 0,
            WaitKind::ButtonPressed => (buttons & self.mask) != 0,
            WaitKind::NextFrame => frame_elapsed,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FrameTask {
    waits: Vec<Wait>,
    cursor: usize,
}

impl FrameTask {
    pub fn new(waits: Vec<Wait>) -> Self {
        Self { waits, cursor: 0 }
    }

    pub fn step(&mut self, buttons: i32, frame_elapsed: bool) -> bool {
        loop {
            let Some(wait) = self.waits.get(self.cursor).copied() else {
                return true;
            };
            if !wait.ready(buttons, frame_elapsed) {
                return false;
            }
            self.cursor += 1;
            if wait.kind == WaitKind::NextFrame {
                return self.cursor == self.waits.len();
            }
        }
    }
}

pub const BUTTON_START: i32 = 0x08;

pub fn wait_buttons_released() -> FrameTask {
    FrameTask::new(vec![Wait::buttons_released(0xff)])
}

pub fn wait_any_button_pressed() -> FrameTask {
    FrameTask::new(vec![Wait::button_pressed(0xff)])
}

pub fn wait_release_then_any_press() -> FrameTask {
    FrameTask::new(vec![
        Wait::buttons_released(0xff),
        Wait::button_pressed(0xff),
    ])
}

pub fn wait_release_then_button_then_release(mask: i32) -> FrameTask {
    FrameTask::new(vec![
        Wait::buttons_released(0xff),
        Wait::button_pressed(mask),
        Wait::buttons_released(0xff),
    ])
}

pub fn wait_frames(count: usize) -> FrameTask {
    FrameTask::new((0..count).map(|_| Wait::next_frame()).collect())
}

pub fn ae11_press_start_gate(engine: &mut crate::Engine) -> FrameTask {
    engine.state.set_prompt_state(0x03);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() + 1) & 0xFF);
    FrameTask::new(vec![
        Wait::buttons_released(0xff),
        Wait::button_pressed(BUTTON_START),
        Wait::buttons_released(0xff),
    ])
}

pub fn finish_ae11_press_start_gate(engine: &mut crate::Engine) {
    engine.state.set_prompt_state(0x04);
    engine
        .state
        .set_sound_paused((engine.state.sound_paused() - 1) & 0xFF);
}
