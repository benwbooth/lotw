use lotw::{Engine, RoutineContext, game};

#[test]
fn dispatch_audio_stream_command_sets_pitch_offset_and_advances_stream() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x00);
    engine.set_mem(0x95, 0x00);
    engine.set_mem(0x96, 0x20);
    engine.set_mem(0x2000, 0xFF);
    engine.set_mem(0x2001, 0x03);
    engine.set_mem(0x2002, 0x07);

    game::dispatch_audio_stream_command(&mut engine, &mut r);

    assert_eq!(engine.mem(0xA1), 0x07);
    assert_eq!(engine.mem(0x95), 0x03);
    assert_eq!(engine.mem(0x96), 0x20);
}

#[test]
fn rewind_or_stop_audio_stream_restarts_loop_or_clears_active_flag() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x10);
    engine.set_mem(0xA7, 0x34);
    engine.set_mem(0xA8, 0x12);

    game::rewind_or_stop_audio_stream(&mut engine, &mut r);

    assert_eq!(engine.mem(0xA5), 0x34);
    assert_eq!(engine.mem(0xA6), 0x12);
    assert_eq!(engine.mem(0xA3), 0x01);
    assert_eq!(r.index, 0x10);

    engine.set_mem(0x02, 0x00);
    engine.set_mem(0x97, 0x00);
    engine.set_mem(0x98, 0x00);
    engine.set_mem(0x94, 0xC0);

    game::rewind_or_stop_audio_stream(&mut engine, &mut r);

    assert_eq!(engine.mem(0x94), 0x40);
}

#[test]
fn load_note_period_applies_pitch_offset_and_octave_shift() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x00);
    engine.set_mem(0x95, 0x00);
    engine.set_mem(0x96, 0x20);
    engine.set_mem(0x2000, 0x11);
    engine.set_mem(0xA1, 0x10);
    engine.set_mem(0xFDB3, 0x40);
    engine.set_mem(0xFDB4, 0x02);

    game::load_note_period(&mut engine, &mut r);

    assert_eq!(engine.mem(0x04), 0x18);
    assert_eq!(engine.mem(0x05), 0x01);
    assert_eq!(engine.mem(0x95), 0x01);
    assert_eq!(engine.mem(0x96), 0x20);
}

#[test]
fn start_note_envelope_loads_phase_state_for_selected_channel() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x10);
    engine.set_mem(0xB2, 0x04);
    engine.set_mem(0xFDCF, 0x81);
    engine.set_mem(0xFDD0, 0x02);
    engine.set_mem(0xFDD1, 0x03);
    engine.set_mem(0xFDD2, 0x04);

    game::start_note_envelope(&mut engine, &mut r);

    assert_eq!(engine.mem(0xAB), 0x04);
    assert_eq!(engine.mem(0xAC), 0x81);
    assert_eq!(engine.mem(0xAD), 0x02);
    assert_eq!(engine.mem(0xAE), 0x03);
    assert_eq!(engine.mem(0xAF), 0x04);
    assert_eq!(r.index, 0x10);
    assert_eq!(r.offset, 0x04);
}

#[test]
fn next_envelope_volume_scales_accumulator_into_apu_volume_byte() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x00);
    engine.set_mem(0x9B, 0x04);
    engine.set_mem(0x9C, 0x01);
    engine.set_mem(0x9F, 0x05);
    engine.set_mem(0x99, 0x80);
    engine.set_mem(0xA0, 0x03);
    engine.set_mem(0xFDD0, 0x02);

    game::next_envelope_volume(&mut engine, &mut r);

    assert_eq!(engine.mem(0x9D), 0x02);
    assert_eq!(engine.mem(0x9F), 0x06);
    assert_eq!(engine.mem(0x00), 0x01);
    assert_eq!(r.value, 0xB1);
}

#[test]
fn advance_envelope_phase_moves_to_next_phase_or_reports_terminal_phase() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x02, 0x00);
    engine.set_mem(0x9B, 0x04);
    engine.set_mem(0x9E, 0x01);
    engine.set_mem(0xFDD3, 0x82);
    engine.set_mem(0xFDD4, 0x03);
    engine.set_mem(0xFDD5, 0x04);

    game::advance_envelope_phase(&mut engine, &mut r);

    assert_eq!(engine.mem(0x9B), 0x08);
    assert_eq!(engine.mem(0x9C), 0x82);
    assert_eq!(engine.mem(0x9D), 0x03);
    assert_eq!(engine.mem(0x9E), 0x04);
    assert_eq!(r.carry, 0);

    engine.set_mem(0x9B, 0x0C);
    engine.set_mem(0x9E, 0x01);

    game::advance_envelope_phase(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(r.value, 0x0C);
}
