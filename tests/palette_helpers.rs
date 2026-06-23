use lotw::{Engine, RoutineContext, game};

#[test]
fn dim_palette_range_by_step_saturates_to_black() {
    let mut engine = Engine::new();
    let mut r = RoutineContext {
        index: 0x00,
        offset: 0x03,
        ..RoutineContext::default()
    };

    engine.state.set_scratch1(0x20);
    engine.state.set_palette_buffer(0x00, 0x3A);
    engine.state.set_palette_buffer(0x01, 0x1B);
    engine.state.set_palette_buffer(0x02, 0x0C);

    game::dim_palette_range_by_step(&mut engine, &mut r);

    assert_eq!(engine.state.palette_buffer(0x00), 0x1A);
    assert_eq!(engine.state.palette_buffer(0x01), 0x0F);
    assert_eq!(engine.state.palette_buffer(0x02), 0x0F);
    assert_eq!(r.index, 0x03);
    assert_eq!(r.offset, 0x00);
}

#[test]
fn reset_menu_state_copies_partial_ram_defaults_and_blacks_palette() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for addr in 0x40..0x8C {
        engine.state.set_byte(addr, 0xAA);
        engine.state.set_byte(
            (lotw::game::ZP_INIT_TABLE + addr) as u16 as i32,
            addr ^ 0x55,
        );
    }
    for offset in 0..=0x1F {
        engine
            .state
            .set_palette_buffer(offset, 0x30 + (offset & 0x0F));
    }

    game::reset_menu_state_and_palette(&mut engine, &mut r);

    assert_eq!(engine.state.character_index(), 0x15);
    assert_eq!(engine.state.long_boost_timer(), 0xDE);
    assert!((0..=0x1F).all(|offset| engine.state.palette_buffer(offset) == 0x0F));
    assert_eq!(r.value, 0x0F);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn load_title_palette_buffer_copies_rom_palette() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    for offset in 0..=0x1F {
        engine.state.set_byte(
            (lotw::game::TITLE_PALETTE_TABLE + offset) as u16 as i32,
            0x40 + offset,
        );
    }

    game::load_title_palette_buffer(&mut engine, &mut r);

    assert_eq!(engine.state.palette_buffer(0x00), 0x40);
    assert_eq!(engine.state.palette_buffer(0x1F), 0x5F);
    assert_eq!(r.index, 0xFF);
}

#[test]
fn room_palette_buffer_copies_room_bytes_and_family_override() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.state.set_palette_src_ptr_lo(0x00);
    engine.state.set_palette_src_ptr_hi(0xB0);
    engine.state.set_character_index(0x02);
    for offset in 0xE0..=0xFF {
        engine.state.set_byte(
            (lotw::game::PALETTE_DATA_TABLE + offset) as u16 as i32,
            0x30 + (offset & 0x1F),
        );
    }
    for offset in 0x08..=0x0B {
        engine.state.set_byte(
            (lotw::game::FAMILY_PALETTE_TABLE + offset) as u16 as i32,
            0x60 + offset,
        );
    }

    game::build_room_palette_buffer(&mut engine, &mut r);

    assert_eq!(engine.state.palette_buffer(0x00), 0x30);
    assert_eq!(engine.state.palette_buffer(0x0F), 0x3F);
    assert_eq!(engine.state.palette_buffer(0x10), 0x68);
    assert_eq!(engine.state.palette_buffer(0x13), 0x6B);
    assert_eq!(engine.state.palette_buffer(0x1F), 0x4F);
    assert_eq!(r.value, 0x0B);
    assert_eq!(r.index, 0x07);
    assert_eq!(r.offset, 0xFF);
    assert_eq!(r.carry, 0);
}

#[test]
fn upload_title_screen_nametables_copies_rom_pages_and_chr_banks() {
    let mut engine = Engine::new();
    let mut r = RoutineContext::default();

    engine.ppu.mirror = 1;
    engine.ppu.vram.fill(0x77);
    engine.state.set_ppu_ctrl_shadow(0xAB);
    engine.state.set_ppu_mask_shadow(0x1E);
    engine
        .state
        .set_byte(lotw::game::TITLE_CHR_BANK_TABLE, 0x12);
    engine
        .state
        .set_byte(lotw::game::TITLE_CHR_BANK_TABLE + 1, 0x34);

    for (page_index, source_page) in [0x9EC9, 0x9FC9, 0xA0C9, 0xA1C9].into_iter().enumerate() {
        for offset in 0..0x100 {
            engine.state.set_byte(
                source_page + offset,
                (page_index as i32 * 0x40) + (offset & 0x3F),
            );
        }
    }

    game::upload_title_screen_nametables(&mut engine, &mut r);

    assert_eq!(engine.ppu.vram[0x000], 0x00);
    assert_eq!(engine.ppu.vram[0x100], 0x40);
    assert_eq!(engine.ppu.vram[0x200], 0x80);
    assert_eq!(engine.ppu.vram[0x300], 0xC0);
    assert_eq!(engine.ppu.vram[0x3FF], 0xFF);
    assert_eq!(engine.ppu.vram[0x400], 0x77);
    assert_eq!(engine.state.chr_bank(0), 0x12);
    assert_eq!(engine.state.chr_bank(1), 0x34);
    assert_eq!(engine.state.ppu_ctrl_shadow(), 0xAB);
    assert_eq!(engine.state.ppu_mask_shadow(), 0x1E);
    assert_eq!(engine.state.statusbar_split_flag(), 0x00);
    assert_eq!(engine.ppu.ctrl, 0xAB);
    assert_eq!(engine.ppu.mask, 0x06);
    assert_eq!(r.value, 0xAB);
    assert_eq!(r.index, 0x00);
}
