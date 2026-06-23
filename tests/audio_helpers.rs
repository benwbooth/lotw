use lotw::{Engine, RoutineContext, game};

#[test]
fn dispatch_audio_stream_command_sets_pitch_offset_and_advances_stream() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x00);
    engine.state.set_sound_channel_byte(2, 0x00, 0x00);
    engine.state.set_sound_channel_byte(3, 0x00, 0x20);
    engine.state.set_byte(0x2000, 0xFF);
    engine.state.set_byte(0x2001, 0x03);
    engine.state.set_byte(0x2002, 0x07);

    game::dispatch_audio_stream_command(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(14, 0x00), 0x07);
    assert_eq!(engine.state.sound_channel_byte(2, 0x00), 0x03);
    assert_eq!(engine.state.sound_channel_byte(3, 0x00), 0x20);
}

#[test]
fn rewind_or_stop_audio_stream_restarts_loop_or_clears_active_flag() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x10);
    engine.state.set_sound_channel_byte(4, 0x10, 0x34);
    engine.state.set_sound_channel_byte(5, 0x10, 0x12);

    game::rewind_or_stop_audio_stream(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(2, 0x10), 0x34);
    assert_eq!(engine.state.sound_channel_byte(3, 0x10), 0x12);
    assert_eq!(engine.state.sound_channel_byte(0, 0x10), 0x01);
    assert_eq!(r.index, 0x10);

    engine.state.set_sound_channel_offset(0x00);
    engine.state.set_sound_channel_byte(4, 0x00, 0x00);
    engine.state.set_sound_channel_byte(5, 0x00, 0x00);
    engine.state.set_sound_channel_byte(1, 0x00, 0xC0);

    game::rewind_or_stop_audio_stream(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(1, 0x00), 0x40);
}

#[test]
fn load_note_period_applies_pitch_offset_and_octave_shift() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x00);
    engine.state.set_sound_channel_byte(2, 0x00, 0x00);
    engine.state.set_sound_channel_byte(3, 0x00, 0x20);
    engine.state.set_byte(0x2000, 0x11);
    engine.state.set_sound_channel_byte(14, 0x00, 0x10);
    engine
        .state
        .set_byte(lotw::game::NOTE_PERIOD_TABLE + 2, 0x40);
    engine
        .state
        .set_byte(lotw::game::NOTE_PERIOD_TABLE + 3, 0x02);

    game::load_note_period(&mut engine, &mut r);

    assert_eq!(engine.state.sound_command(), 0x18);
    assert_eq!(engine.state.sound_length(), 0x01);
    assert_eq!(engine.state.sound_channel_byte(2, 0x00), 0x01);
    assert_eq!(engine.state.sound_channel_byte(3, 0x00), 0x20);
}

#[test]
fn start_note_envelope_loads_phase_state_for_selected_channel() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x10);
    engine.state.set_sound_channel_byte(15, 0x10, 0x04);
    engine.state.set_byte(lotw::game::ENVELOPE_TABLE + 4, 0x81);
    engine.state.set_byte(lotw::game::ENVELOPE_TABLE + 5, 0x02);
    engine.state.set_byte(lotw::game::ENVELOPE_TABLE + 6, 0x03);
    engine.state.set_byte(lotw::game::SUSTAIN_TABLE, 0x04);

    game::start_note_envelope(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(8, 0x10), 0x04);
    assert_eq!(engine.state.sound_channel_byte(9, 0x10), 0x81);
    assert_eq!(engine.state.sound_channel_byte(10, 0x10), 0x02);
    assert_eq!(engine.state.sound_channel_byte(11, 0x10), 0x03);
    assert_eq!(engine.state.sound_channel_byte(12, 0x10), 0x04);
    assert_eq!(r.index, 0x10);
    assert_eq!(r.offset, 0x04);
}

#[test]
fn next_envelope_volume_scales_accumulator_into_apu_volume_byte() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x00);
    engine.state.set_sound_channel_byte(8, 0x00, 0x04);
    engine.state.set_sound_channel_byte(9, 0x00, 0x01);
    engine.state.set_sound_channel_byte(12, 0x00, 0x05);
    engine.state.set_sound_channel_byte(6, 0x00, 0x80);
    engine.state.set_sound_channel_byte(13, 0x00, 0x03);
    engine.state.set_byte(lotw::game::ENVELOPE_TABLE + 5, 0x02);

    game::next_envelope_volume(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(10, 0x00), 0x02);
    assert_eq!(engine.state.sound_channel_byte(12, 0x00), 0x06);
    assert_eq!(engine.state.audio_duty_work(), 0x01);
    assert_eq!(r.value, 0xB1);
}

#[test]
fn advance_envelope_phase_moves_to_next_phase_or_reports_terminal_phase() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_sound_channel_offset(0x00);
    engine.state.set_sound_channel_byte(8, 0x00, 0x04);
    engine.state.set_sound_channel_byte(11, 0x00, 0x01);
    engine.state.set_byte(lotw::game::SUSTAIN_TABLE + 1, 0x82);
    engine.state.set_byte(lotw::game::SUSTAIN_TABLE + 2, 0x03);
    engine.state.set_byte(lotw::game::SUSTAIN_TABLE + 3, 0x04);

    game::advance_envelope_phase(&mut engine, &mut r);

    assert_eq!(engine.state.sound_channel_byte(8, 0x00), 0x08);
    assert_eq!(engine.state.sound_channel_byte(9, 0x00), 0x82);
    assert_eq!(engine.state.sound_channel_byte(10, 0x00), 0x03);
    assert_eq!(engine.state.sound_channel_byte(11, 0x00), 0x04);
    assert_eq!(r.carry, 0);

    engine.state.set_sound_channel_byte(8, 0x00, 0x0C);
    engine.state.set_sound_channel_byte(11, 0x00, 0x01);

    game::advance_envelope_phase(&mut engine, &mut r);

    assert_eq!(r.carry, 1);
    assert_eq!(r.value, 0x0C);
}
