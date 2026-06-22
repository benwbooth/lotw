use lotw::{Engine, RoutineContext, game, native};

const ROOM_STATE_ADDRS: [i32; 7] = [0x43, 0x44, 0x45, 0x7B, 0x7C, 0x47, 0x48];

fn set_room_state(engine: &mut Engine, state: [i32; 7]) {
    for (addr, value) in ROOM_STATE_ADDRS.into_iter().zip(state) {
        engine.set_mem(addr, value);
    }
}

fn assert_room_state(engine: &Engine, state: [i32; 7]) {
    for (addr, value) in ROOM_STATE_ADDRS.into_iter().zip(state) {
        assert_eq!(engine.mem(addr), value, "room state address {addr:#04x}");
    }
}

#[test]
fn room_tile_page_copy_copies_three_source_pages() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.set_mem(0x75, 0x00);
    engine.set_mem(0x76, 0x90);
    for page in 0..=2 {
        for offset in 0..0x100 {
            engine.set_mem(
                0x9000 + (page << 8) + offset,
                (page * 0x40) + (offset & 0x3F),
            );
        }
    }

    game::copy_room_tile_pages(&mut engine, &mut r);

    assert_eq!(engine.mem(0x0500), 0x00);
    assert_eq!(engine.mem(0x053F), 0x3F);
    assert_eq!(engine.mem(0x0600), 0x40);
    assert_eq!(engine.mem(0x0700), 0x80);
    assert_eq!(engine.mem(0x07FF), 0xBF);
    assert_eq!(engine.mem(0x77), 0x00);
    assert_eq!(engine.mem(0x78), 0x93);
    assert_eq!(r.offset, 0x00);
}

#[test]
fn room_checkpoint_stack_round_trips_room_state_lifo() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    let first = [0x01, 0x23, 0x45, 0x67, 0x89, 0x0A, 0x0B];
    let second = [0x10, 0x32, 0x54, 0x76, 0x98, 0x0C, 0x0D];

    set_room_state(&mut engine, first);
    engine.set_mem(0x8E, 0x11);
    native::push_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 1);
    assert_eq!(engine.mem(0xFE), 0x11);

    set_room_state(&mut engine, second);
    engine.set_mem(0x8E, 0x22);
    native::push_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 2);
    assert_eq!(engine.mem(0xFE), 0x22);

    set_room_state(&mut engine, [0; 7]);

    native::pop_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 1);
    assert_room_state(&engine, second);

    native::pop_room_checkpoint(&mut engine, &mut r);
    assert_eq!(engine.room_ckpt_sp, 0);
    assert_room_state(&engine, first);
}
