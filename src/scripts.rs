//! Scripted-input / replay helpers.
//!
//! Drives the game from a pre-recorded sequence of input conditions instead of
//! a live controller. A [`FrameTask`] is an ordered list of [`Wait`]s; each
//! frame the control loop calls [`FrameTask::step`] with the current button
//! state, advancing through the waits until they are all satisfied. This backs
//! attract-mode / title-prompt sequences and deterministic replays.

/// The kind of condition a single scripted [`Wait`] is gating on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitKind {
    /// Wait until none of the masked buttons are held (a full release edge).
    ButtonsReleased,
    /// Wait until at least one of the masked buttons is pressed.
    ButtonPressed,
    /// Wait until the next frame boundary, regardless of input.
    NextFrame,
}

/// A single scripted step: a [`WaitKind`] together with the button mask it
/// applies to. The mask is unused for [`WaitKind::NextFrame`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wait {
    kind: WaitKind,
    mask: i32,
}

impl Wait {
    /// Wait for all buttons in `mask` to be released.
    pub const fn buttons_released(mask: i32) -> Self {
        Self {
            kind: WaitKind::ButtonsReleased,
            mask,
        }
    }

    /// Wait for any button in `mask` to be pressed.
    pub const fn button_pressed(mask: i32) -> Self {
        Self {
            kind: WaitKind::ButtonPressed,
            mask,
        }
    }

    /// Wait for the next frame; the mask is irrelevant so it is left at 0.
    pub const fn next_frame() -> Self {
        Self {
            kind: WaitKind::NextFrame,
            mask: 0,
        }
    }

    /// Whether this wait's condition is satisfied given the current `buttons`
    /// bitmask and whether a `frame_elapsed` this tick.
    fn ready(self, buttons: i32, frame_elapsed: bool) -> bool {
        match self.kind {
            // Released: no masked bit is currently held.
            WaitKind::ButtonsReleased => (buttons & self.mask) == 0,
            // Pressed: at least one masked bit is currently held.
            WaitKind::ButtonPressed => (buttons & self.mask) != 0,
            // NextFrame: simply waits for the frame boundary.
            WaitKind::NextFrame => frame_elapsed,
        }
    }
}

/// An ordered, stateful sequence of scripted [`Wait`]s. `cursor` tracks how far
/// through the sequence we have progressed across frames.
#[derive(Clone, Debug)]
pub struct FrameTask {
    waits: Vec<Wait>,
    cursor: usize,
}

impl FrameTask {
    /// Create a task from a list of waits, positioned at the first wait.
    pub fn new(waits: Vec<Wait>) -> Self {
        Self { waits, cursor: 0 }
    }

    /// Advance the task using this frame's `buttons` and `frame_elapsed` flag.
    ///
    /// Consumes as many *non-frame* waits as are already satisfied this tick,
    /// then stops at the first unsatisfied wait. Returns `true` when the whole
    /// sequence is complete, `false` while it is still pending.
    pub fn step(&mut self, buttons: i32, frame_elapsed: bool) -> bool {
        loop {
            // No wait left at the cursor means the sequence is finished.
            let Some(wait) = self.waits.get(self.cursor).copied() else {
                return true;
            };
            // Stop here if the current wait's condition is not yet met.
            if !wait.ready(buttons, frame_elapsed) {
                return false;
            }
            // Condition met: advance past this wait.
            self.cursor += 1;
            // A frame wait consumes the rest of this tick: report completion
            // only if it was the final wait, otherwise yield until next frame
            // (we do not keep consuming further waits within the same frame).
            if wait.kind == WaitKind::NextFrame {
                return self.cursor == self.waits.len();
            }
        }
    }
}

/// Bitmask for the NES Start button (bit 3) within the controller byte.
pub const BUTTON_START: i32 = 0x08;

/// Mask matching every controller button (all 8 bits) — used when a scripted
/// step cares about "any" / "all" buttons rather than a specific one.
const ALL_BUTTONS_MASK: i32 = 255;

/// Task that completes once all buttons are released.
pub fn wait_buttons_released() -> FrameTask {
    FrameTask::new(vec![Wait::buttons_released(ALL_BUTTONS_MASK)])
}

/// Task that completes once any button is pressed.
pub fn wait_any_button_pressed() -> FrameTask {
    FrameTask::new(vec![Wait::button_pressed(ALL_BUTTONS_MASK)])
}

/// Task that waits for a full release, then any subsequent press — the standard
/// "press any key to continue" debounce.
pub fn wait_release_then_any_press() -> FrameTask {
    FrameTask::new(vec![
        Wait::buttons_released(ALL_BUTTONS_MASK),
        Wait::button_pressed(ALL_BUTTONS_MASK),
    ])
}

/// Task that waits for a full release, then a press of the specific `mask`
/// button(s), then another full release — a complete debounced button "click".
pub fn wait_release_then_button_then_release(mask: i32) -> FrameTask {
    FrameTask::new(vec![
        Wait::buttons_released(ALL_BUTTONS_MASK),
        Wait::button_pressed(mask),
        Wait::buttons_released(ALL_BUTTONS_MASK),
    ])
}

/// Task that simply waits `count` frames before completing.
pub fn wait_frames(count: usize) -> FrameTask {
    FrameTask::new((0..count).map(|_| Wait::next_frame()).collect())
}

/// Begin the title-screen "press Start" gate (routine $AE11).
///
/// Puts the prompt into its waiting state and bumps the sound-paused counter,
/// then returns a debounced Start-button task: release, press Start, release.
/// Pair with [`finish_ae11_press_start_gate`] once the task completes.
pub fn ae11_press_start_gate(engine: &mut crate::Engine) -> FrameTask {
    // prompt_state 3 = "awaiting Start press".
    engine.state.prompt_state = 3;
    // Increment the (saturating, byte-wide) sound-pause nesting counter so audio
    // stays paused while the prompt is up; masked to a single byte.
    engine.state.sound_paused = (engine.state.sound_paused + 1) & ((crate::bits::BYTE_MASK) as u8);
    // Debounced Start: full release, Start press, full release.
    FrameTask::new(vec![
        Wait::buttons_released(ALL_BUTTONS_MASK),
        Wait::button_pressed(BUTTON_START),
        Wait::buttons_released(ALL_BUTTONS_MASK),
    ])
}

/// Complete the title-screen "press Start" gate started by
/// [`ae11_press_start_gate`]: advance the prompt and unbalance the matching
/// sound-pause increment.
pub fn finish_ae11_press_start_gate(engine: &mut crate::Engine) {
    // prompt_state 4 = "Start acknowledged, proceeding".
    engine.state.prompt_state = 4;
    // Decrement the sound-pause counter to undo the increment from the start
    // gate; masked to a single byte to mirror the 8-bit hardware wrap.
    engine.state.sound_paused = (engine.state.sound_paused - 1) & ((crate::bits::BYTE_MASK) as u8);
}
