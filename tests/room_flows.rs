use lotw::{Engine, RoutineContext, game};

const ROOM_STATE_ADDRS: [i32; 7] = [0x43, 0x44, 0x45, 0x7B, 0x7C, 0x47, 0x48];

fn set_room_state(engine: &mut Engine, state: [i32; 7]) {
    for (addr, value) in ROOM_STATE_ADDRS.into_iter().zip(state) {
        engine.state.set_byte(addr, value);
    }
}

fn assert_room_state(engine: &Engine, state: [i32; 7]) {
    for (addr, value) in ROOM_STATE_ADDRS.into_iter().zip(state) {
        assert_eq!(
            engine.state.byte(addr),
            value,
            "room state address {addr:#04x}"
        );
    }
}

#[test]
fn room_tile_page_copy_copies_three_source_pages() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.room_metadef_lo = 0x00;
    engine.state.room_metadef_hi = 0x90;
    for page in 0..=2 {
        for offset in 0..0x100 {
            engine.state.set_byte(
                0x9000 + (page << 8) + offset,
                (page * 0x40) + (offset & 0x3F),
            );
        }
    }

    game::copy_room_tile_pages(&mut engine, &mut r);

    assert_eq!(engine.state.room_buffer(0x00), 0x00);
    assert_eq!(engine.state.room_buffer(0x3F), 0x3F);
    assert_eq!(engine.state.room_buffer(0x100), 0x40);
    assert_eq!(engine.state.room_buffer(0x200), 0x80);
    assert_eq!(engine.state.room_buffer(0x2FF), 0xBF);
    assert_eq!(engine.state.palette_src_ptr_lo, 0x00);
    assert_eq!(engine.state.palette_src_ptr_hi, 0x93);
    assert_eq!(r.offset, 0x00);
}

#[test]
fn room_checkpoint_stack_round_trips_room_state_lifo() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    let first = [0x01, 0x23, 0x45, 0x67, 0x89, 0x0A, 0x0B];
    let second = [0x10, 0x32, 0x54, 0x76, 0x98, 0x0C, 0x0D];

    set_room_state(&mut engine, first);
    engine.state.song = 0x11;
    game::push_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 1);
    assert_eq!(engine.state.room_restore_scratch, 0x11);

    set_room_state(&mut engine, second);
    engine.state.song = 0x22;
    game::push_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 2);
    assert_eq!(engine.state.room_restore_scratch, 0x22);

    set_room_state(&mut engine, [0; 7]);

    game::pop_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 1);
    assert_room_state(&engine, second);

    game::pop_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 0);
    assert_room_state(&engine, first);
}
