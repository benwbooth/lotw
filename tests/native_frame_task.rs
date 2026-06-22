use lotw::{Engine, scripts};

#[test]
fn start_gate_requires_release_press_release() {
    let mut engine = Engine::new();
    let mut gate = scripts::ae11_press_start_gate(&mut engine);

    assert!(!gate.step(scripts::BUTTON_START, false));
    assert_eq!(engine.mem(0x8f), 0x03);
    assert_eq!(engine.mem(0x8d), 0x01);

    assert!(!gate.step(0x00, false));
    assert!(!gate.step(scripts::BUTTON_START, false));
    assert!(!gate.step(scripts::BUTTON_START, false));
    assert!(gate.step(0x00, false));
    scripts::finish_ae11_press_start_gate(&mut engine);
    assert_eq!(engine.mem(0x8f), 0x04);
    assert_eq!(engine.mem(0x8d), 0x00);
}

#[test]
fn common_wait_tasks_advance_only_on_matching_input() {
    let mut release = scripts::wait_buttons_released();
    assert!(!release.step(0x01, false));
    assert!(release.step(0x00, false));

    let mut press = scripts::wait_any_button_pressed();
    assert!(!press.step(0x00, false));
    assert!(press.step(0x40, false));

    let mut release_then_press = scripts::wait_release_then_any_press();
    assert!(!release_then_press.step(0x04, false));
    assert!(!release_then_press.step(0x00, false));
    assert!(release_then_press.step(0x20, false));

    let mut transition = scripts::wait_release_then_button_then_release(0x10);
    assert!(!transition.step(0x10, false));
    assert!(!transition.step(0x00, false));
    assert!(!transition.step(0x08, false));
    assert!(!transition.step(0x10, false));
    assert!(transition.step(0x00, false));

    let mut two_frames = scripts::wait_frames(2);
    assert!(!two_frames.step(0x00, false));
    assert!(!two_frames.step(0x00, false));
    assert!(!two_frames.step(0x00, true));
    assert!(two_frames.step(0x00, true));
}
