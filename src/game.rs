// Rust game routine module. The functions here are the checked-in native game logic.
//
// Numbered `routine_####` names are retained as stable port labels while the
// original game systems are being identified. Keep semantic discoveries in
// `docs/routine_catalog.md` first, then rename or alias routines only after the
// dataflow is understood well enough to make the name useful.
use crate::engine::RoutineFn;
use crate::frame;
use crate::native::*;
use crate::{Engine, RoutineContext, cbool, not, u8v, u16v};

pub use apply_projectile_direction_bits::apply_projectile_direction_bits;
pub use farcall_bank_0C0D_seed::farcall_bank_0C0D_seed;
pub use farcall_bank_09_r7::farcall_bank_09_r7;
pub use farcall_return_home::farcall_return_home;
pub use frame_counters::frame_counters;
pub use game_update::game_update;
pub use inc16_95::inc16_95;
pub use load_object_slot_scratch::load_object_slot_scratch;
pub use main_init::main_init;
pub use metasprite_build::metasprite_build;
pub use ppu_commit_banks::ppu_commit_banks;
pub use project_player_projectile_position::project_player_projectile_position;
pub use ram_state_init::ram_state_init;
pub use read_controllers::read_controllers;
pub use reset::reset;
pub use rng_update::rng_update;
pub use routine_0003::routine_0003;
pub use routine_0005::routine_0005;
pub use routine_0006::routine_0006;
pub use routine_0007::routine_0007;
pub use routine_0008::routine_0008;
pub use routine_0009::routine_0009;
pub use routine_0010::routine_0010;
pub use routine_0011::routine_0011;
pub use routine_0012::routine_0012;
pub use routine_0013::routine_0013;
pub use routine_0014::routine_0014;
pub use routine_0015::routine_0015;
pub use routine_0016::routine_0016;
pub use routine_0017::routine_0017;
pub use routine_0018::routine_0018;
pub use routine_0019::routine_0019;
pub use routine_0021::routine_0021;
pub use routine_0022::routine_0022;
pub use routine_0023::routine_0023;
pub use routine_0024::routine_0024;
pub use routine_0025::routine_0025;
pub use routine_0026::routine_0026;
pub use routine_0027::routine_0027;
pub use routine_0028::routine_0028;
pub use routine_0030::routine_0030;
pub use routine_0031::routine_0031;
pub use routine_0032::routine_0032;
pub use routine_0035::routine_0035;
pub use routine_0036::routine_0036;
pub use routine_0037::routine_0037;
pub use routine_0038::routine_0038;
pub use routine_0040::routine_0040;
pub use routine_0041::routine_0041;
pub use routine_0042::routine_0042;
pub use routine_0043::routine_0043;
pub use routine_0044::routine_0044;
pub use routine_0046::routine_0046;
pub use routine_0047::routine_0047;
pub use routine_0048::routine_0048;
pub use routine_0051::routine_0051;
pub use routine_0052::routine_0052;
pub use routine_0053::routine_0053;
pub use routine_0054::routine_0054;
pub use routine_0056::routine_0056;
pub use routine_0057::routine_0057;
pub use routine_0059::routine_0059;
pub use routine_0060::routine_0060;
pub use routine_0061::routine_0061;
pub use routine_0062::routine_0062;
pub use routine_0063::routine_0063;
pub use routine_0064::routine_0064;
pub use routine_0065::routine_0065;
pub use routine_0066::routine_0066;
pub use routine_0073::routine_0073;
pub use routine_0075::routine_0075;
pub use routine_0076::routine_0076;
pub use routine_0077::routine_0077;
pub use routine_0078::routine_0078;
pub use routine_0079::routine_0079;
pub use routine_0080::routine_0080;
pub use routine_0081::routine_0081;
pub use routine_0082::routine_0082;
pub use routine_0083::routine_0083;
pub use routine_0084::routine_0084;
pub use routine_0085::routine_0085;
pub use routine_0086::routine_0086;
pub use routine_0087::routine_0087;
pub use routine_0088::routine_0088;
pub use routine_0089::routine_0089;
pub use routine_0090::routine_0090;
pub use routine_0091::routine_0091;
pub use routine_0092::routine_0092;
pub use routine_0093::routine_0093;
pub use routine_0094::routine_0094;
pub use routine_0095::routine_0095;
pub use routine_0096::routine_0096;
pub use routine_0097::routine_0097;
pub use routine_0098::routine_0098;
pub use routine_0099::routine_0099;
pub use routine_0100::routine_0100;
pub use routine_0101::routine_0101;
pub use routine_0102::routine_0102;
pub use routine_0103::routine_0103;
pub use routine_0106::routine_0106;
pub use routine_0107::routine_0107;
pub use routine_0108::routine_0108;
pub use routine_0111::routine_0111;
pub use routine_0112::routine_0112;
pub use routine_0113::routine_0113;
pub use routine_0114::routine_0114;
pub use routine_0115::routine_0115;
pub use routine_0116::routine_0116;
pub use routine_0117::routine_0117;
pub use routine_0118::routine_0118;
pub use routine_0119::routine_0119;
pub use routine_0120::routine_0120;
pub use routine_0121::routine_0121;
pub use routine_0122::routine_0122;
pub use routine_0123::routine_0123;
pub use routine_0124::routine_0124;
pub use routine_0125::routine_0125;
pub use routine_0126::routine_0126;
pub use routine_0127::routine_0127;
pub use routine_0128::routine_0128;
pub use routine_0129::routine_0129;
pub use routine_0130::routine_0130;
pub use routine_0131::routine_0131;
pub use routine_0132::routine_0132;
pub use routine_0135::routine_0135;
pub use routine_0136::routine_0136;
pub use routine_0137::routine_0137;
pub use routine_0138::routine_0138;
pub use routine_0139::routine_0139;
pub use routine_0140::routine_0140;
pub use routine_0141::routine_0141;
pub use routine_0142::routine_0142;
pub use routine_0143::routine_0143;
pub use routine_0144::routine_0144;
pub use routine_0145::routine_0145;
pub use routine_0146::routine_0146;
pub use routine_0147::routine_0147;
pub use routine_0149::routine_0149;
pub use routine_0150::routine_0150;
pub use routine_0151::routine_0151;
pub use routine_0152::routine_0152;
pub use routine_0153::routine_0153;
pub use routine_0154::routine_0154;
pub use routine_0155::routine_0155;
pub use routine_0156::routine_0156;
pub use routine_0157::routine_0157;
pub use routine_0158::routine_0158;
pub use routine_0159::routine_0159;
pub use routine_0160::routine_0160;
pub use routine_0161::routine_0161;
pub use routine_0162::routine_0162;
pub use routine_0164::routine_0164;
pub use routine_0165::routine_0165;
pub use routine_0166::routine_0166;
pub use routine_0167::routine_0167;
pub use routine_0168::routine_0168;
pub use routine_0170::routine_0170;
pub use routine_0171::routine_0171;
pub use routine_0172::routine_0172;
pub use routine_0173::routine_0173;
pub use routine_0178::routine_0178;
pub use routine_0179::routine_0179;
pub use routine_0180::routine_0180;
pub use routine_0181::routine_0181;
pub use routine_0182::routine_0182;
pub use routine_0183::routine_0183;
pub use routine_0184::routine_0184;
pub use routine_0185::routine_0185;
pub use routine_0186::routine_0186;
pub use routine_0192::routine_0192;
pub use routine_0195::routine_0195;
pub use routine_0196::routine_0196;
pub use routine_0197::routine_0197;
pub use routine_0198::routine_0198;
pub use routine_0199::routine_0199;
pub use routine_0200::routine_0200;
pub use routine_0201::routine_0201;
pub use routine_0202::routine_0202;
pub use routine_0203::routine_0203;
pub use routine_0204::routine_0204;
pub use routine_0205::routine_0205;
pub use routine_0206::routine_0206;
pub use routine_0207::routine_0207;
pub use routine_0208::routine_0208;
pub use routine_0209::routine_0209;
pub use routine_0210::routine_0210;
pub use routine_0211::routine_0211;
pub use routine_0212::routine_0212;
pub use routine_0215::routine_0215;
pub use routine_0216::routine_0216;
pub use routine_0217::routine_0217;
pub use routine_0218::routine_0218;
pub use routine_0219::routine_0219;
pub use routine_0220::routine_0220;
pub use routine_0221::routine_0221;
pub use routine_0222::routine_0222;
pub use routine_0223::routine_0223;
pub use routine_0224::routine_0224;
pub use routine_0225::routine_0225;
pub use routine_0226::routine_0226;
pub use routine_0227::routine_0227;
pub use routine_0228::routine_0228;
pub use routine_0229::routine_0229;
pub use routine_0230::routine_0230;
pub use routine_0231::routine_0231;
pub use routine_0232::routine_0232;
pub use routine_0233::routine_0233;
pub use routine_0234::routine_0234;
pub use routine_0235::routine_0235;
pub use routine_0236::routine_0236;
pub use routine_0237::routine_0237;
pub use routine_0238::routine_0238;
pub use routine_0239::routine_0239;
pub use routine_0241::routine_0241;
pub use routine_0242::routine_0242;
pub use routine_0243::routine_0243;
pub use routine_0244::routine_0244;
pub use routine_0245::routine_0245;
pub use routine_0246::routine_0246;
pub use routine_0247::routine_0247;
pub use routine_0248::routine_0248;
pub use routine_0249::routine_0249;
pub use routine_0250::routine_0250;
pub use routine_0251::routine_0251;
pub use routine_0252::routine_0252;
pub use routine_0253::routine_0253;
pub use routine_0254::routine_0254;
pub use routine_0255::routine_0255;
pub use routine_0256::routine_0256;
pub use routine_0257::routine_0257;
pub use routine_0258::routine_0258;
pub use routine_0260::routine_0260;
pub use routine_0261::routine_0261;
pub use routine_0262::routine_0262;
pub use routine_0263::routine_0263;
pub use routine_0264::routine_0264;
pub use routine_0265::routine_0265;
pub use routine_0273::routine_0273;
pub use routine_0274::routine_0274;
pub use routine_0275::routine_0275;
pub use routine_0276::routine_0276;
pub use routine_0277::routine_0277;
pub use routine_0278::routine_0278;
pub use routine_0279::routine_0279;
pub use routine_0280::routine_0280;
pub use routine_0281::routine_0281;
pub use routine_0282::routine_0282;
pub use routine_0283::routine_0283;
pub use routine_0284::routine_0284;
pub use routine_0285::routine_0285;
pub use routine_0286::routine_0286;
pub use routine_0287::routine_0287;
pub use routine_0288::routine_0288;
pub use routine_0289::routine_0289;
pub use scene_assemble::scene_assemble;
pub use sfx_overlay_voice::sfx_overlay_voice;
pub use song_init::song_init;
pub use sound_restore_game_banks::sound_restore_game_banks;
pub use sound_set_default_banks::sound_set_default_banks;
pub use sound_set_song_banks::sound_set_song_banks;
pub use sound_tick::sound_tick;
pub use spawn_player_projectile::spawn_player_projectile;
pub use statusbar_split::statusbar_split;
pub use store_object_slot_scratch::store_object_slot_scratch;
pub use text_attr_build::text_attr_build;
pub use update_player_projectile_slot::update_player_projectile_slot;
pub use update_player_projectiles::update_player_projectiles;
pub use update_tile_projectile::update_tile_projectile;
pub use update_tile_projectile_motion::update_tile_projectile_motion;
pub use vblank_commit::vblank_commit;
pub use vblank_commit_tail::vblank_commit_tail;
pub use vram_blit_stack::vram_blit_stack;
pub use vram_copy_indirect::vram_copy_indirect;
pub use vram_fill_run::vram_fill_run;
pub use vram_poke2::vram_poke2;
pub use vram_upload_hud::vram_upload_hud;
pub use vram_upload_palette::vram_upload_palette;

mod farcall_bank_09_r7 {
    use super::*;
    pub fn farcall_bank_09_r7(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_r7: i32 = engine.mem(0x31);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, 0x09);
        engine.device_write(0x8001, 0x09);
        engine.set_mem(0x0D, 0x00);
        r.value = 0x00;
        routine_0090(engine, r);
        metasprite_build(engine, r);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, saved_r7);
        engine.device_write(0x8001, saved_r7);
        r.value = saved_r7;
    }
}

mod farcall_bank_0C0D_seed {
    use super::*;
    pub fn farcall_bank_0C0D_seed(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x32, engine.mem(0x30));
        engine.set_mem(0x33, engine.mem(0x31));
        engine.set_mem(0x25, 0x06);
        engine.device_write(0x8000, 0x06);
        engine.set_mem(0x30, 0x0C);
        engine.device_write(0x8001, 0x0C);
        engine.set_mem(0x25, 0x07);
        engine.device_write(0x8000, 0x07);
        engine.set_mem(0x31, 0x0D);
        engine.device_write(0x8001, 0x0D);
        r.value = 0x0D;
        r.offset = 0x07;
    }
}

mod farcall_return_home {
    use super::*;
    pub fn farcall_return_home(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x31, engine.mem(0x33));
        engine.set_mem(0x30, engine.mem(0x32));
    }
}

mod frame_counters {
    use super::*;
    pub fn frame_counters(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.dec_mem(0x84) != 0) {
            return;
        }
        {
            let mut x: i32 = 7;
            while cbool(x >= 0) {
                if cbool(engine.mem((0x85 + x) & 0xFF) != 0) {
                    engine.dec_mem((0x85 + x) & 0xFF);
                }
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x84, 0x3C);
        r.index = 0xFF;
    }
}

mod game_update {
    use super::*;
    pub fn game_update(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut y: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xE3, 0xFF);
                    if cbool(engine.mem(0xEB) != 0) {
                        routine_0139(engine, r);
                        return;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0140(engine, r);
                    if cbool(engine.mem(0x20) & 0x10) {
                        routine_0174(engine, r);
                        return;
                    }
                    routine_0136(engine, r);
                    if cbool(engine.mem(0x46) != 0) {
                        engine.dec_mem(0x46);
                        engine.set_mem(0x20, 0x00);
                    }
                    {
                        let mut clear_hi: i32 = 1;
                        if cbool(engine.mem(0x40) == 0x04) {
                            if cbool((engine.mem(0x84) & 0x07) == 0) {
                                clear_hi = 1;
                            } else {
                                clear_hi = (if cbool(engine.mem(0x20) & 0x40) { 0 } else { 1 });
                            }
                        } else {
                            clear_hi = (if cbool(engine.mem(0x20) & 0x40) { 0 } else { 1 });
                        }
                        if cbool(clear_hi) {
                            engine.and_mem(0xFD, 0x0F);
                        }
                    }
                    a = engine.mem(0x20) & 0x0F;
                    if cbool(a != 0) {
                        engine.set_mem(0x08, a);
                        engine.set_mem(0xFD, u8v((engine.mem(0xFD) & 0xF0) | a));
                    }
                    if cbool(engine.mem(0x20) & 0x20) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x20) & 0x08) {
                        routine_0167(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                    }
                    y = 0x01;
                    while cbool(engine.mem(u16v(0x0087 + y)) != 0) {
                        {
                            let __old = y;
                            y += 1;
                            __old
                        };
                        if cbool(y >= 0x05) {
                            y = 0x06;
                            break;
                        }
                    }
                    r.offset = y;
                    routine_0107(engine, r);
                    if cbool(engine.mem(0x4E) != 0) {
                        engine.set_mem(0x4B, u8v((engine.mem(0x4E) >> 2) + 1));
                        routine_0146(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0x49, 0x00);
                        engine.set_mem(0x4A, 0x00);
                        routine_0146(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4F) != 0) {
                        routine_0135(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                        engine.set_mem(0x4F, 0x00);
                    } else if cbool(engine.mem(0x20) & 0x80) {
                        routine_0135(engine, r);
                        if cbool(engine.lotw_nonlocal_handoff) {
                            return;
                        }
                        engine.set_mem(0x4F, 0x00);
                    } else {
                        engine.set_mem(0x22, 0x00);
                        engine.set_mem(0x4F, 0x00);
                    }
                    routine_0146(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    routine_0173(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x43, engine.mem(0x0E));
                    engine.set_mem(0x44, engine.mem(0x0F));
                    a = engine.mem(0x0A);
                    if cbool(a >= 0xEF) {
                        a = 0x00;
                    }
                    engine.set_mem(0x45, a);
                    routine_0163(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0x4F, 0x00);
                    engine.set_mem(0x4E, 0x00);
                    routine_0163(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    engine.set_mem(0x8F, 0x10);
                    loop {
                        routine_0103(engine, r);
                        if cbool(r.value & 0xF0) {
                            break;
                        }
                        if cbool((engine.mem(0x20) & 0x03) == 0) {
                            continue;
                        }
                        engine.shl_mem(0x20, 1);
                        engine.shl_mem(0x20, 1);
                        r.offset = 0x01;
                        routine_0107(engine, r);
                        {
                            let mut t: i32 = u8v(engine.mem(0x4B) + engine.mem(0x55));
                            let mut ni: i32 = 0;
                            if cbool(t & 0x80) {
                                ni = 0x03;
                            } else if cbool(t < 0x04) {
                                ni = t;
                            } else {
                                ni = 0x00;
                            }
                            engine.set_mem(0x55, ni);
                        }
                        engine.set_mem(0x8F, 0x0C);
                    }
                    engine.set_mem(0x8F, 0x10);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    routine_0144(engine, r);
                    routine_0145(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod inc16_95 {
    use super::*;
    pub fn inc16_95(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        if cbool(engine.inc_mem((0x95 + x) & 0xFF) == 0) {
            engine.inc_mem((0x96 + x) & 0xFF);
        }
        r.index = x;
    }
}

mod main_init {
    use super::*;
    fn farcall_0C0D(
        engine: &mut Engine,
        r: &mut RoutineContext,
        mut lo: i32,
        mut hi: i32,
        target: RoutineFn,
    ) {
        let mut old6: i32 = engine.mem(0x30);
        let mut old7: i32 = engine.mem(0x31);
        engine.set_mem(0x32, old6);
        engine.set_mem(0x33, old7);
        engine.set_mem(0x0E, lo);
        engine.set_mem(0x0F, hi);
        engine.set_mem(0x30, 0x0C);
        engine.set_mem(0x31, 0x0D);
        engine.set_mem(0x25, 0x07);
        engine.prg_map_shadow();
        target(engine, r);
        engine.set_mem(0x31, old7);
        engine.set_mem(0x30, old6);
        engine.set_mem(0x25, 0x06);
        engine.prg_map_shadow();
    }

    pub fn main_init(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2000, 0x00);
        engine.device_write(0x2001, 0x00);
        engine.device_write(0x4010, 0x00);
        engine.set_mem(0x0027, 0x1F);
        engine.device_write(0x4015, 0x1F);
        engine.device_write(0x4017, 0xC0);
        engine.device_write(0xA000, 0x00);
        farcall_bank_0C0D_seed(engine, r);
        ram_state_init(engine, r);
        farcall_0C0D(engine, r, 0x64, 0xAE, routine_0033);
        engine.set_mem(0x46, 0x00);
        engine.set_mem(0x7B, 0x00);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7C, 0x30);
        engine.set_mem(0x44, 0x3C);
        engine.set_mem(0x45, 0xA0);
        scene_assemble(engine, r);
        engine.set_mem(0x20, 0x08);
        game_update(engine, r);
        main_loop_dispatch(engine, r);
    }
}

mod metasprite_build {
    use super::*;
    pub fn metasprite_build(engine: &mut Engine, r: &mut RoutineContext) {
        let mut p0C: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut p79: i32 = u16v(engine.mem(0x79) | (engine.mem(0x7A) << 8));
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut dst_lo: i32 = 0;
        let mut mask2: i32 = 0;
        engine.set_mem(0x0B, 0x00);
        {
            x = 0x16;
            while cbool(x >= 0) {
                let mut e: i32 = engine.mem(u16v(p0C + engine.mem(0x0B)));
                let mut ty: i32 = u16v(u8v(e << 2));
                engine.set_mem(u16v(0x0141 + x), engine.mem(u16v(p79 + ((ty + 0) & 0xFF))));
                engine.set_mem(u16v(0x0140 + x), engine.mem(u16v(p79 + ((ty + 1) & 0xFF))));
                engine.set_mem(u16v(0x0159 + x), engine.mem(u16v(p79 + ((ty + 2) & 0xFF))));
                engine.set_mem(u16v(0x0158 + x), engine.mem(u16v(p79 + ((ty + 3) & 0xFF))));
                engine.add_mem(0x0B, 1);
                x -= 2;
            }
        }
        engine.set_mem(0x19, u8v(engine.mem(0x17) + 0x03));
        dst_lo = engine.mem(0x16);
        engine.set_mem(0x0B, u8v((dst_lo >> 2) + 0xC0));
        mask2 = u8v(dst_lo & 0x02);
        engine.set_mem(0x18, (if cbool(mask2) { 0x33 } else { 0xCC }));
        y = 0x00;
        {
            x = 0x0A;
            while cbool(x >= 0) {
                let mut b0: i32 = 0;
                let mut b1: i32 = 0;
                let mut v: i32 = 0;
                engine.set_mem(u16v(0x0170 + x), engine.mem(0x0B));
                engine.set_mem(0x0B, u8v(engine.mem(0x0B) + 0x08));
                b0 = engine.mem(u16v(
                    p0C + ({
                        let __old = y;
                        y += 1;
                        __old
                    }),
                ));
                v = u8v((b0 & 0xC0) >> 4);
                engine.set_mem(u16v(0x0171 + x), v);
                b1 = engine.mem(u16v(
                    p0C + ({
                        let __old = y;
                        y += 1;
                        __old
                    }),
                ));
                v = u8v((b1 & 0xC0) | engine.mem(u16v(0x0171 + x)));
                engine.set_mem(u16v(0x0171 + x), v);
                if cbool(mask2 == 0) {
                    engine.set_mem(u16v(0x0171 + x), u8v(engine.mem(u16v(0x0171 + x)) >> 1));
                    engine.set_mem(u16v(0x0171 + x), u8v(engine.mem(u16v(0x0171 + x)) >> 1));
                }
                x -= 2;
            }
        }
        r.value = 0x03;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod ppu_commit_banks {
    use super::*;
    pub fn ppu_commit_banks(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 7;
            while cbool(x >= 0) {
                engine.device_write(0x8000, u8v(x));
                engine.device_write(0x8001, engine.mem(u16v(0x2A + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod ram_state_init {
    use super::*;
    pub fn ram_state_init(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut i: i32 = 0;
        x = 0;
        loop {
            engine.set_mem(0x0000 + x, engine.mem(0x9B9F + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
        {
            i = 0x3F;
            while cbool(i >= 0) {
                engine.set_mem(0x0100 + u8v(i), engine.mem(0x9C9E + u8v(i)));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        {
            i = 0x1F;
            while cbool(i >= 0) {
                engine.set_mem(0x0180 + u8v(i), 0x0F);
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        x = 0;
        loop {
            engine.set_mem(0x0300 + x, engine.mem(0x9D3E + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
        x = 0;
        loop {
            engine.set_mem(0x0400 + x, engine.mem(0x9DC9 + x));
            if !cbool(
                {
                    x = u8v(x + 1);
                    x
                } != 0,
            ) {
                break;
            }
        }
    }
}

mod read_controllers {
    use super::*;
    pub fn read_controllers(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        if let Some(__buttons) = engine.next_input() {
            engine.ppu.set_buttons(__buttons);
        }
        engine.device_write(0x4016, 0x01);
        engine.device_write(0x4016, 0x00);
        {
            x = 8;
            while cbool(x != 0) {
                a = u8v(engine.device_read(0x4016) | engine.device_read(0x4017));
                c = a & 1;
                a >>= 1;
                engine.set_mem(0x20, u8v((engine.mem(0x20) << 1) | c));
                c = a & 1;
                engine.set_mem(0x21, u8v((engine.mem(0x21) << 1) | c));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x20, engine.mem(0x20) | engine.mem(0x21));
        return;
        engine.set_mem(0x4016, 0x01);
        engine.set_mem(0x4016, 0x00);
        {
            x = 8;
            while cbool(x != 0) {
                a = engine.mem(0x4016) | engine.mem(0x4017);
                c = a & 1;
                a >>= 1;
                engine.set_mem(0x20, u8v((engine.mem(0x20) << 1) | c));
                c = a & 1;
                engine.set_mem(0x21, u8v((engine.mem(0x21) << 1) | c));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x20, engine.mem(0x20) | engine.mem(0x21));
    }
}

mod reset {
    use super::*;
    pub fn reset(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x00);
        engine.device_write(0xA001, 0x00);
        engine.device_write(0xE000, 0x00);
        main_init(engine, r);
    }
}

mod rng_update {
    use super::*;
    pub fn rng_update(engine: &mut Engine, r: &mut RoutineContext) {
        let mut count: i32 = u8v(r.value);
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: i32 = 0;
        engine.set_mem(0x38, count);
        if cbool(count == 0) {
            r.value = engine.mem(0x3B);
            return;
        }
        x = engine.mem(0x3B);
        y = engine.mem(0x3A);
        loop {
            let mut xy: i32 = 0;
            let mut s: i32 = 0;
            let mut carry: i32 = 0;
            engine.set_mem(0x39, y);
            xy = u16v((u16v((x << 8) | y) << 1) + 1);
            x = u8v(xy >> 8);
            y = u8v(xy);
            s = u16v(y + engine.mem(0x3A));
            y = u8v(s);
            carry = u8v(s >> 8);
            a = u8v(x + engine.mem(0x3B) + carry);
            a = u8v(a + engine.mem(0x39));
            a &= 0x7F;
            x = a;
            engine.set_mem(0x3B, a);
            engine.set_mem(0x3A, y);
            if !cbool(a >= count) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0003 {
    use super::*;
    pub fn routine_0003(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xFA) == 0) {
            engine.set_mem(0x16, 0x0E);
            engine.set_mem(0x17, 0x20);
            engine.set_mem(
                0x17,
                u8v((u8v((engine.mem(0x1D) ^ 0x01) << 2)) | engine.mem(0x17)),
            );
            engine.set_mem(
                0xF9,
                u8v((u8v((((engine.mem(0x1D) ^ 0x01) << 4) + 0x07))) | engine.mem(0x7C)),
            );
            engine.set_mem(0xFA, 0x09);
        }
        engine.set_mem(0x0C, engine.mem(0xF9));
        farcall_bank_09_r7(engine, r);
        engine.set_mem(0x16, u8v(engine.mem(0x16) + 1));
        engine.set_mem(0x16, u8v(engine.mem(0x16) + 1));
        engine.set_mem(0xF9, u8v(engine.mem(0xF9) + 1));
        engine.set_mem(0xFA, u8v(engine.mem(0xFA) - 1));
        if cbool(engine.mem(0xFA) == 0) {
            engine.xor_mem(0x1D, 0x01);
        }
    }
}

mod routine_0005 {
    use super::*;
    pub fn routine_0005(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xE3, 0x01);
        engine.set_mem(0xE5, 0x10);
        engine.set_mem(0xE6, 0x04);
        loop {
            let mut p: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
            if cbool(engine.mem(u16v(p + 1)) != 0) {
                routine_0007(engine, r);
            } else if (cbool(engine.mem(0x20) & 0x40) && !cbool(engine.mem(0xFD) & 0x40)) {
                routine_0006(engine, r);
            }
            engine.set_mem(0xE3, u8v(engine.mem(0xE3) + 1));
            {
                let mut np: i32 = u16v(engine.mem(0xE5) + 0x10);
                engine.set_mem(0xE5, u8v(np));
                engine.set_mem(0xE6, u8v(engine.mem(0xE6) + (np >> 8)));
            }
            if !cbool(engine.mem(0xE3) < 0x04) {
                break;
            }
        }
        routine_0012(engine, r);
    }
}

mod routine_0006 {
    use super::*;
    pub fn routine_0006(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xFD, u8v((engine.mem(0x20) & 0x40) | engine.mem(0xFD)));
        r.value = engine.mem(0xFD);
        r.offset = 0x02;
        routine_0015(engine, r);
        routine_0008(engine, r);
        routine_0010(engine, r);
        if !cbool(r.carry) {
            engine.set_mem(0xF9, engine.mem(0x0E));
            engine.set_mem(0xFB, engine.mem(0x0A));
            engine.set_mem(0xEE, 0x18);
            engine.set_mem(0xEF, 0x00);
            engine.set_mem(0xED, 0x21);
            engine.set_mem(0x8F, 0x19);
        }
        if cbool(engine.mem(0xEE) != 0) {
            routine_0009(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }
}

mod routine_0007 {
    use super::*;
    pub fn routine_0007(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xEE, u8v(engine.mem(0xEE) - 1));
        if cbool(engine.mem(0xEE) != 0) {
            routine_0011(engine, r);
            routine_0010(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
            } else {
                engine.set_mem(0xF9, engine.mem(0x0E));
                engine.set_mem(0xFB, engine.mem(0x0A));
            }
        }
        if cbool(engine.mem(0xEE) != 0) {
            routine_0009(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }
}

mod routine_0008 {
    use super::*;
    pub fn routine_0008(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0xF7) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF7) << 2);
            engine.set_mem(0x0A, u8v(a + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0xF5) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF5) << 2);
            engine.set_mem(0x0E, u8v(a + engine.mem(0x0E)));
        }
    }
}

mod routine_0009 {
    use super::*;
    pub fn routine_0009(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x08, u8v(engine.mem(0xEE) & 0x0C));
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | engine.mem(0x08)));
    }
}

mod routine_0010 {
    use super::*;
    pub fn routine_0010(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xA1) {
            return;
        }
        if cbool(engine.mem(0x0E) < 0xF1) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0x00) {
            return;
        }
        r.carry = 1;
    }
}

mod routine_0011 {
    use super::*;
    pub fn routine_0011(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0A, engine.mem(0xFB));
        if cbool(engine.mem(0xF7) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0xF7) + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0xF5) != 0) {
            engine.set_mem(0x0E, u8v(engine.mem(0xF5) + engine.mem(0x0E)));
        }
    }
}

mod routine_0012 {
    use super::*;
    pub fn routine_0012(engine: &mut Engine, r: &mut RoutineContext) {
        let mut count: i32 = 0;
        engine.set_mem(0x0F, 0x88);
        engine.set_mem(0x0E, 0x10);
        count = 0x03;
        loop {
            routine_0013(engine, r);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + 0x08));
            engine.set_mem(0x0E, u8v(engine.mem(0x0E) + 0x10));
            count = u8v(count - 0x01);
            if !cbool(count != 0) {
                break;
            }
        }
    }
}

mod routine_0013 {
    use super::*;
    pub fn routine_0013(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x0F);
        let mut y: i32 = engine.mem(0x0E);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(u16v(0x0401 + y)) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x040E + y)) >= 0xBF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut attr: i32 = engine.mem(u16v(0x0402 + y));
                        engine.set_mem(u16v(0x0202 + x), attr);
                        engine.set_mem(u16v(0x0206 + x), attr);
                        if cbool(attr & 0x40) {
                            let mut t: i32 = engine.mem(u16v(0x0400 + y));
                            engine.set_mem(u16v(0x0205 + x), t);
                            engine.set_mem(u16v(0x0201 + x), u8v(t + 2));
                        } else {
                            let mut t: i32 = engine.mem(u16v(0x0400 + y));
                            engine.set_mem(u16v(0x0201 + x), t);
                            engine.set_mem(u16v(0x0205 + x), u8v(t + 2));
                        }
                    }
                    {
                        let mut px: i32 = engine.mem(u16v(0x040C + y));
                        engine.set_mem(u16v(0x0203 + x), px);
                        engine.set_mem(u16v(0x0207 + x), u8v(px + 8));
                        {
                            let mut py: i32 = u8v(engine.mem(u16v(0x040E + y)) + 0x2B);
                            engine.set_mem(u16v(0x0200 + x), py);
                            engine.set_mem(u16v(0x0204 + x), py);
                        }
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(u16v(0x0200 + x), 0xEF);
                    engine.set_mem(u16v(0x0204 + x), 0xEF);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0014 {
    use super::*;
    pub fn routine_0014(engine: &mut Engine, r: &mut RoutineContext) {
        let mut c: i32 = u8v(engine.mem(0x3E) - 1);
        if cbool(c & 0x80) {
            c = 0x07;
        }
        engine.set_mem(0x3E, c);
        let mut x: i32 = u8v(c << 2);
        let mut base: i32 = (if cbool(c & 0x06) { 0x0280 } else { 0x0210 });
        engine.set_mem(0x0200, engine.mem(u16v(base + 0 + x)));
        engine.set_mem(0x0201, engine.mem(u16v(base + 1 + x)));
        engine.set_mem(0x0202, engine.mem(u16v(base + 2 + x)));
        engine.set_mem(0x0203, engine.mem(u16v(base + 3 + x)));
        engine.set_mem(u16v(base + x), 0xEF);
    }
}

mod routine_0015 {
    use super::*;
    pub fn routine_0015(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        engine.set_mem(0x09, r.offset);
        x = u8v((r.value & 0x0F) << 1);
        a = 0x00;
        y = r.offset;
        loop {
            a = u8v(a + engine.mem(u16v(0xFE8B + x)));
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y != 0) {
                break;
            }
        }
        engine.set_mem(0xF5, a);
        y = engine.mem(0x09);
        a = 0x00;
        loop {
            a = u8v(a + engine.mem(u16v(0xFE8C + x)));
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y != 0) {
                break;
            }
        }
        engine.set_mem(0xF7, a);
    }
}

mod routine_0016 {
    use super::*;
    pub fn routine_0016(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0240 + x), engine.mem(u16v(0xAAFC + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        routine_0099(engine, r);
    }
}

mod routine_0017 {
    use super::*;
    pub fn routine_0017(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0240 + x), engine.mem(u16v(0xAB3C + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        routine_0098(engine, r);
    }
}

mod routine_0018 {
    use super::*;
    pub fn routine_0018(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x3F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x02C0 + x), engine.mem(u16v(0xAB7C + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        routine_0100(engine, r);
    }
}

mod routine_0019 {
    use super::*;
    pub fn routine_0019(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = u8v(engine.mem(0x56) & 0x1F);
        engine.set_mem(0x08, v);
        engine.set_mem(0x0410, u8v((engine.mem(0x0410) & 0xE0) | v));
        engine.set_mem(0x0420, u8v((engine.mem(0x0420) & 0xE0) | v));
        engine.set_mem(0x0430, u8v((engine.mem(0x0430) & 0xE0) | v));
        let mut xf: i32 = engine.mem(0x43);
        engine.set_mem(0x041C, xf);
        engine.set_mem(0x042C, xf);
        engine.set_mem(0x043C, xf);
        let mut x: i32 = engine.mem(0x44);
        {
            let __old = x;
            x += 1;
            __old
        };
        engine.set_mem(0x042D, x);
        x -= 3;
        engine.set_mem(0x043D, x);
        {
            let __old = x;
            x -= 1;
            __old
        };
        engine.set_mem(0x041D, x);
    }
}

mod routine_0021 {
    use super::*;
    fn tail_acbb(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0024(engine, r);
        routine_0025(engine, r);
        routine_0026(engine, r);
    }

    fn tail_aca1(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x43, engine.mem(0x0E));
        engine.set_mem(0x45, engine.mem(0x0A));
        routine_0028(engine, r);
        tail_acbb(engine, r);
    }

    fn tail_acaf(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        routine_0028(engine, r);
        tail_acbb(engine, r);
    }

    pub fn routine_0021(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    r.value = engine.mem(0x20);
                    if cbool(r.value & 0x10) {
                        routine_0029(engine, r);
                        return;
                    }
                    if !cbool(engine.mem(0x20) & 0x40) {
                        engine.set_mem(0xFD, u8v(engine.mem(0xFD) & 0x0F));
                    }
                    r.value = u8v(engine.mem(0x20) & 0x0F);
                    if cbool(r.value != 0) {
                        engine.set_mem(0x08, r.value);
                        engine.set_mem(0xFD, u8v((engine.mem(0xFD) & 0xF0) | engine.mem(0x08)));
                    }
                    if cbool(engine.mem(0x85) == 0) {
                        if cbool((engine.mem(0x26) & 0x40) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.index = u8v(engine.mem(0x3E) + 1);
                        if cbool(((r.index) & 0x06) != 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        {
                            let mut sum: i32 =
                                u8v(u8v(engine.mem(0x1C) + engine.mem(u16v(0x040C + r.index))));
                            r.value = u8v((if cbool(sum < 0xB0) { 0x0A } else { 0x05 }));
                        }
                        routine_0030(engine, r);
                        engine.set_mem(0x4F, 0x0A);
                        engine.set_mem(0x8F, 0x21);
                        engine.set_mem(0x90, 0x02);
                        engine.set_mem(0x85, 0x01);
                        routine_0100(engine, r);
                    }
                    if (cbool(engine.mem(0x4F) == 0) && cbool(engine.mem(0x4E) == 0)) {
                        engine.set_mem(0x85, 0x00);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x20, u8v((engine.mem(0x20) & 0xF0) | 0x02));
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0032(engine, r);
                    if cbool(engine.mem(0x4E) != 0) {
                        r.value = u8v(engine.mem(0x4E) >> 2);
                        r.value = u8v(r.value + 1);
                        engine.set_mem(0x4B, r.value);
                        routine_0027(engine, r);
                        if !cbool(r.carry) {
                            tail_aca1(engine, r);
                            return;
                        }
                        engine.set_mem(0x49, 0x00);
                        routine_0027(engine, r);
                        if !cbool(r.carry) {
                            return;
                        }
                        tail_acaf(engine, r);
                        return;
                    }
                    if cbool(engine.mem(0x4F) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0x20) & 0x80) {
                        engine.set_mem(0x22, 0x00);
                        r.value = 0x00;
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    routine_0022(engine, r);
                    r.value = 0x00;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x4F, r.value);
                    routine_0027(engine, r);
                    if cbool(r.carry) {
                        tail_acaf(engine, r);
                        return;
                    }
                    tail_aca1(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0022 {
    use super::*;
    pub fn routine_0022(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x4F);
        if cbool(x == 0) {
            if cbool(engine.mem(0x22) != 0) {
                return;
            }
            engine.set_mem(0x8F, 0x1B);
            engine.set_mem(0x4F, engine.mem(0x5C));
        }
        engine.set_mem(0x22, 0x01);
        engine.set_mem(0x4F, u8v(engine.mem(0x4F) - 1));
        engine.set_mem(0x4B, u8v((u8v(x >> 2) ^ 0xFF) + 1));
        routine_0027(engine, r);
        if cbool(r.carry) {
            engine.set_mem(0x49, 0x00);
            routine_0027(engine, r);
        }
        if !cbool(r.carry) {
            engine.set_mem(0x43, engine.mem(0x0E));
            engine.set_mem(0x45, engine.mem(0x0A));
            routine_0028(engine, r);
        } else {
            engine.set_mem(0x4F, 0x00);
            engine.set_mem(0x4E, 0x00);
            routine_0028(engine, r);
        }
        routine_0024(engine, r);
        routine_0025(engine, r);
        routine_0026(engine, r);
    }
}

mod routine_0023 {
    use super::*;
    pub fn routine_0023(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0x4B) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x4B) + engine.mem(0x0A)));
        }
        if cbool(engine.mem(0x49) != 0) {
            engine.set_mem(0x0E, u8v(engine.mem(0x49) + engine.mem(0x0E)));
        }
    }
}

mod routine_0024 {
    use super::*;
    pub fn routine_0024(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    r.index = 0x09;
                    if cbool(u8v(engine.mem(0x20) & 0xBF) == 0x80) {
                        engine.set_mem(0x56, r.index);
                        return;
                    }
                    if cbool(engine.mem(0x4B) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4B) & 0x80) {
                        if cbool(engine.mem(0x4F) == 0) {
                            engine.set_mem(0x56, r.index);
                            return;
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4E) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool((engine.mem(0x20) & 0x04) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.index = 0x0D;
                    engine.set_mem(0x56, r.index);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.index = 0x01;
                    r.offset = 0x00;
                    if cbool(engine.mem(0x49) & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    r.offset = 0x40;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x08, r.index);
                    engine.set_mem(0x56, u8v((engine.mem(0x56) & 0x07) | engine.mem(0x08)));
                    engine.set_mem(0x57, r.offset);
                    return;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    r.index = 0x39;
                    r.offset = 0x00;
                    if cbool(engine.mem(0x49) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    r.offset = 0x40;
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0x08, r.index);
                    engine.set_mem(0x56, u8v((engine.mem(0x56) & 0x03) | engine.mem(0x08)));
                    engine.set_mem(0x57, r.offset);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0025 {
    use super::*;
    pub fn routine_0025(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x56) < 0x20) {
            let mut a: i32 = engine.mem(0x56);
            if cbool(engine.mem(0x20) & 0x40) {
                a = u8v(a | 0x10);
            } else {
                a = u8v(a & 0xEF);
            }
            engine.set_mem(0x56, a);
        }
        if cbool((engine.mem(0x20) & 0x0F) == 0) {
            return;
        }
        if cbool((engine.mem(0x4F) | engine.mem(0x4E)) != 0) {
            return;
        }
        engine.set_mem(0x4D, u8v(engine.mem(0x4D) + 1));
        if cbool((engine.mem(0x4D) & 0x07) != 0) {
            return;
        }
        if cbool(engine.mem(0x56) & 0x08) {
            engine.xor_mem(0x57, 0x40);
        } else {
            engine.xor_mem(0x56, 0x04);
        }
    }
}

mod routine_0026 {
    use super::*;
    pub fn routine_0026(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x85) != 0) {
            if cbool((engine.mem(0x84) & 0x01) == 0) {
                engine.set_mem(0x0210, 0xEF);
                engine.set_mem(0x0214, 0xEF);
                return;
            }
        }
        engine.set_mem(0x0210, u8v(engine.mem(0x45) + 0x2B));
        engine.set_mem(0x0214, u8v(engine.mem(0x45) + 0x2B));
        engine.set_mem(0x0213, engine.mem(0x43));
        engine.set_mem(0x0217, u8v(engine.mem(0x43) + 0x08));
        engine.set_mem(0x0212, u8v(engine.mem(0x57) | 0x20));
        engine.set_mem(0x0216, u8v(engine.mem(0x57) | 0x20));
        if cbool(engine.mem(0x57) & 0x40) {
            r.index = engine.mem(0x56);
            engine.set_mem(0x0215, r.index);
            r.index = u8v(r.index + 2);
            engine.set_mem(0x0211, r.index);
        } else {
            r.index = engine.mem(0x56);
            engine.set_mem(0x0211, r.index);
            r.index = u8v(r.index + 2);
            engine.set_mem(0x0215, r.index);
        }
    }
}

mod routine_0027 {
    use super::*;
    pub fn routine_0027(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved: i32 = engine.mem(0x4B);
        loop {
            routine_0023(engine, r);
            routine_0031(engine, r);
            if !cbool(r.carry) {
                break;
            }
            {
                let mut x: i32 = engine.mem(0x4B);
                if cbool(x == 0) {
                    r.carry = 1;
                    break;
                }
                if !cbool(x & 0x80) {
                    x = u8v(x - 1);
                    x = u8v(x - 1);
                }
                x = u8v(x + 1);
                engine.set_mem(0x4B, x);
                if cbool(x != 0) {
                    continue;
                }
                r.carry = 1;
                break;
            }
        }
        engine.set_mem(0x4B, saved);
    }
}

mod routine_0028 {
    use super::*;
    pub fn routine_0028(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x4F) != 0) {
            r.carry = 0;
            return;
        }
        if cbool(engine.mem(0x45) < 0xA0) {
            engine.set_mem(0x4E, u8v(engine.mem(0x4E) + 1));
            return;
        }
        {
            let mut a: i32 = engine.mem(0x4E);
            if cbool(a >= engine.mem(0x5C)) {
                a = u8v(a - 0x07);
                if cbool(a >= engine.mem(0x5C)) {
                    a = engine.mem(0x5C);
                }
                a = u8v(a - 0x01);
                engine.set_mem(0x4F, a);
                engine.set_mem(0x8F, 0x0A);
            }
        }
        engine.set_mem(0x4E, 0x00);
    }
}

mod routine_0030 {
    use super::*;
    pub fn routine_0030(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lhs: i32 = 0;
        let mut res: i32 = 0;
        engine.set_mem(0x08, r.value);
        lhs = engine.mem(0x58);
        {
            let mut t: i32 = u16v(lhs) - u16v(engine.mem(0x08));
            res = u8v(t);
            r.carry = (if cbool(t & 0x100) { 0 } else { 1 });
            r.zero = u8v((if cbool(res == 0) { 1 } else { 0 }));
            r.negative = (res >> 7) & 1;
        }
        engine.set_mem(0x58, res);
        if !cbool(r.carry) {
            engine.set_mem(0x58, 0x00);
        }
    }
}

mod routine_0031 {
    use super::*;
    pub fn routine_0031(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xA1) {
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(0x0E) >= 0xF1) {
            r.carry = 1;
            return;
        }
        r.carry = 0;
    }
}

mod routine_0032 {
    use super::*;
    pub fn routine_0032(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = u8v((engine.mem(0x20) & 0x0F) << 1);
        engine.set_mem(0x49, engine.mem(u16v(0xFE8B + r.index)));
        engine.set_mem(0x4B, engine.mem(u16v(0xFE8C + r.index)));
    }
}

mod routine_0035 {
    use super::*;
    pub fn routine_0035(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x04;
        rng_update(engine, r);
        r.index = r.value;
        engine.set_mem(0x20, engine.mem(u16v(0xB0FE + r.index)));
        r.value = 0x0A;
        rng_update(engine, r);
        r.index = r.value;
        if cbool(r.index == 0) {
            engine.set_mem(0x20, u8v(engine.mem(0x20) | 0x40));
        }
    }
}

mod routine_0036 {
    use super::*;
    pub fn routine_0036(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x7F;
            while cbool(x >= 0) {
                engine.set_mem(0x0240 + x, engine.mem(0xB71C + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0037 {
    use super::*;
    pub fn routine_0037(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x1F;
            while cbool(x >= 0) {
                engine.set_mem(0x0240 + x, engine.mem(0xB6FC + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0038 {
    use super::*;
    pub fn routine_0038(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0xEF;
        if cbool(engine.mem(0x84) & 0x30) {
            x = 0x80;
        }
        engine.set_mem(0x0240, x);
        engine.set_mem(0x0244, x);
        engine.set_mem(0x0248, x);
        engine.set_mem(0x024C, x);
        engine.set_mem(0x0250, x);
        engine.set_mem(0x0254, x);
        engine.set_mem(0x0258, x);
        engine.set_mem(0x025C, x);
        r.index = x;
    }
}

mod routine_0040 {
    use super::*;
    pub fn routine_0040(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = 0;
        let mut y: i32 = 0;
        routine_0048(engine, r);
        ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        {
            let mut i: i32 = 0;
            {
                i = 0;
                y = 0;
                while cbool(i < 256) {
                    let mut b: i32 = engine.mem(u16v(ptr + y));
                    if cbool(b == 0x00) {
                        r.carry = 1;
                        return;
                    }
                    if cbool(b == 0x0D) {
                        routine_0042(engine, r);
                        r.value = 0x05;
                        routine_0044(engine, r);
                        r.carry = 0;
                        return;
                    }
                    engine.set_mem(0x08, b & 0x0F);
                    engine.set_mem(u16v(0x0140 + y), u8v(((b & 0xF0) << 1) | engine.mem(0x08)));
                    {
                        i += 1;
                        i
                    };
                    {
                        y += 1;
                        y
                    };
                }
            }
        }
    }
}

mod routine_0041 {
    use super::*;
    pub fn routine_0041(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = 0;
        let mut y: i32 = 0;
        let mut c: i32 = 0;
        let mut lo: i32 = 0;
        let mut guard: i32 = 0;
        routine_0048(engine, r);
        ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        y = 0x00;
        {
            guard = 0;
            while cbool(guard < 256) {
                c = engine.mem(u16v(ptr + y));
                if cbool(c == 0x00) {
                    r.carry = 1;
                    return;
                }
                if cbool(c == 0x0D) {
                    let mut sum: i32 = 0;
                    {
                        let __old = y;
                        y += 1;
                        __old
                    };
                    sum = u16v(y + engine.mem(0x0C));
                    lo = u8v(sum);
                    engine.set_mem(0x0C, lo);
                    if cbool(sum > 0xFF) {
                        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 1));
                    }
                    r.value = 0x08;
                    routine_0042(engine, r);
                    r.value = 0x05;
                    routine_0044(engine, r);
                    r.carry = 0;
                    return;
                }
                {
                    let mut lonib: i32 = c & 0x0F;
                    let mut hi: i32 = 0;
                    let mut v: i32 = 0;
                    engine.set_mem(0x08, lonib);
                    hi = u8v((c & 0xF0) << 1);
                    v = u8v(hi | engine.mem(0x08));
                    v = u8v(v + 0x10);
                    engine.set_mem(u16v(0x0140 + y), v);
                }
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                {
                    let __old = guard;
                    guard += 1;
                    __old
                };
            }
        }
    }
}

mod routine_0042 {
    use super::*;
    pub fn routine_0042(engine: &mut Engine, r: &mut RoutineContext) {
        let mut hi: i32 = 0x08;
        let mut a: i32 = engine.mem(0x0A);
        let mut carry: i32 = 0;
        carry = a >> 7;
        a = u8v(a << 1);
        hi = u8v((hi << 1) | carry);
        carry = a >> 7;
        a = u8v(a << 1);
        hi = u8v((hi << 1) | carry);
        engine.set_mem(0x17, hi);
        engine.set_mem(0x16, a);
        r.value = a;
    }
}

mod routine_0043 {
    use super::*;
    pub fn routine_0043(engine: &mut Engine, r: &mut RoutineContext) {
        loop {
            engine.inc_mem(0x0A);
            if cbool((engine.mem(0x0A) & 0x07) == 0) {
                break;
            }
            r.value = 0xFF;
            routine_0044(engine, r);
        }
        if cbool(engine.mem(0x0A) == 0xF0) {
            engine.set_mem(0x0A, 0x00);
        }
    }
}

mod routine_0044 {
    use super::*;
    pub fn routine_0044(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_a: i32 = u8v(r.value);
        let mut v: i32 = u8v(engine.mem(0x0A) + 0x06);
        if cbool(v >= 0xF0) {
            v = u8v(v + 0x10);
        }
        engine.set_mem(0x1E, v);
        r.value = saved_a;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
        r.value = 0xFF;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0046 {
    use super::*;
    pub fn routine_0046(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        engine.set_mem(0x0180, 0x0F);
        engine.set_mem(0x0181, 0x0C);
        engine.set_mem(0x0182, 0x10);
        engine.set_mem(0x0183, 0x30);
        {
            x = 0x1B;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0184 + x), 0x0F);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.value = 0x0F;
        routine_0075(engine, r);
    }
}

mod routine_0047 {
    use super::*;
    pub fn routine_0047(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x00;
        loop {
            engine.set_mem(u16v(0x0200 + x), 0xEF);
            x = u8v(x + 4);
            if !cbool(x != 0) {
                break;
            }
        }
        r.index = x;
        r.value = 0xEF;
    }
}

mod routine_0048 {
    use super::*;
    pub fn routine_0048(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        {
            y = 0x1F;
            while cbool(y >= 0) {
                engine.set_mem(0x0140 + y, 0xC0);
                {
                    y -= 1;
                    y
                };
            }
        }
        r.value = 0xC0;
        r.offset = 0xFF;
    }
}

mod routine_0051 {
    use super::*;
    pub fn routine_0051(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        x = 0x0F;
        y = 0x07;
        loop {
            let mut b: i32 = engine.mem(u16v(0x0308 + y));
            engine.set_mem(u16v(0x0322 + x), u8v(b >> 4));
            {
                let __old = x;
                x -= 1;
                __old
            };
            engine.set_mem(u16v(0x0322 + x), u8v(b & 0x0F));
            {
                let __old = x;
                x -= 1;
                __old
            };
            {
                let __old = y;
                y -= 1;
                __old
            };
            if !cbool(y >= 0) {
                break;
            }
        }
        {
            x = 0x0F;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0332 + x), u8v(engine.mem(u16v(0x0310 + x)) & 0x0F));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            let mut a: i32 = engine.mem(0x0320);
            {
                x = 0x0F;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0322 + x));
                    engine.set_mem(u16v(0x0322 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = engine.mem(0x0321);
            {
                x = 0x0F;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0332 + x));
                    engine.set_mem(u16v(0x0332 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = 0x00;
            {
                x = 0x1F;
                while cbool(x >= 0) {
                    a = u8v(a + engine.mem(u16v(0x0322 + x)));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            engine.set_mem(0x0389, a);
        }
        {
            let mut a: i32 = 0x0A;
            {
                x = 0x1F;
                while cbool(x >= 0) {
                    a = u8v(a ^ engine.mem(u16v(0x0322 + x)));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            engine.set_mem(0x038A, a);
        }
        {
            let mut a: i32 = engine.mem(0x0389);
            {
                x = 0x0E;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0322 + x));
                    engine.set_mem(u16v(0x0322 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        {
            let mut a: i32 = engine.mem(0x038A);
            {
                x = 0x0E;
                while cbool(x >= 0) {
                    let mut cin: i32 = u8v(a & 1);
                    a >>= 1;
                    let mut c: i32 = engine.mem(u16v(0x0332 + x));
                    engine.set_mem(u16v(0x0332 + x), u8v((c << 1) | cin));
                    x -= 2;
                }
            }
        }
        engine.set_mem(0x3A, engine.mem(0x0331));
        engine.set_mem(0x3B, engine.mem(0x0341));
        {
            x = 0x0E;
            while cbool(x >= 0) {
                engine.set_mem(0x08, u8v(x));
                r.value = 0x20;
                rng_update(engine, r);
                x = engine.mem(0x08);
                engine.set_mem(
                    u16v(0x0322 + x),
                    u8v(r.value ^ engine.mem(u16v(0x0322 + x))),
                );
                r.value = 0x20;
                rng_update(engine, r);
                x = engine.mem(0x08);
                engine.set_mem(
                    u16v(0x0332 + x),
                    u8v(r.value ^ engine.mem(u16v(0x0332 + x))),
                );
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
    }
}

mod routine_0052 {
    use super::*;
    pub fn routine_0052(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    {
                        x = 0x1F;
                        while cbool(x >= 0) {
                            engine.set_mem(u16v(0x0342 + x), engine.mem(u16v(0x0322 + x)));
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    engine.set_mem(0x3A, engine.mem(0x0351));
                    engine.set_mem(0x3B, engine.mem(0x0361));
                    {
                        x = 0x0E;
                        while cbool(x >= 0) {
                            engine.set_mem(0x08, u8v(x));
                            r.value = 0x20;
                            rng_update(engine, r);
                            x = engine.mem(0x08);
                            engine.xor_mem(u16v(0x0342 + x), r.value);
                            r.value = 0x20;
                            rng_update(engine, r);
                            x = engine.mem(0x08);
                            engine.xor_mem(u16v(0x0352 + x), r.value);
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0E;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0352 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0352 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x038A, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0E;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0342 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0342 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0389, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x1F;
                            while cbool(x >= 0) {
                                a = u8v(a + engine.mem(u16v(0x0342 + x)));
                                {
                                    let __old = x;
                                    x -= 1;
                                    __old
                                };
                            }
                        }
                        if cbool(a != engine.mem(0x0389)) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    {
                        let mut a: i32 = 0x0A;
                        {
                            x = 0x1F;
                            while cbool(x >= 0) {
                                a ^= engine.mem(u16v(0x0342 + x));
                                {
                                    let __old = x;
                                    x -= 1;
                                    __old
                                };
                            }
                        }
                        if cbool(a != engine.mem(0x038A)) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0F;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0342 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0342 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0320, a);
                    }
                    {
                        let mut a: i32 = 0;
                        {
                            x = 0x0F;
                            while cbool(x >= 0) {
                                let mut c: i32 = engine.mem(u16v(0x0352 + x));
                                a = u8v((a >> 1) | ((c & 1) << 7));
                                engine.set_mem(u16v(0x0352 + x), u8v(c >> 1));
                                x -= 2;
                            }
                        }
                        engine.set_mem(0x0321, a);
                    }
                    x = 0x0F;
                    y = 0x07;
                    loop {
                        let mut hi: i32 = engine.mem(u16v(0x0342 + x));
                        {
                            let __old = x;
                            x -= 1;
                            __old
                        };
                        let mut lo: i32 = engine.mem(u16v(0x0342 + x));
                        {
                            let __old = x;
                            x -= 1;
                            __old
                        };
                        engine.set_mem(u16v(0x0308 + y), u8v((hi << 4) | lo));
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                        if !cbool(y >= 0) {
                            break;
                        }
                    }
                    {
                        x = 0x0F;
                        while cbool(x >= 0) {
                            engine.set_mem(u16v(0x0310 + x), engine.mem(u16v(0x0352 + x)));
                            {
                                let __old = x;
                                x -= 1;
                                __old
                            };
                        }
                    }
                    r.carry = 0;
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x8F, 0x1C);
                    engine.set_mem(0x90, 0x1C);
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0053 {
    use super::*;
    pub fn routine_0053(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x40;
        loop {
            engine.set_mem(u16v(0x00 + x), engine.mem(u16v(0x9B9F + x)));
            {
                let __old = x;
                x += 1;
                __old
            };
            if !cbool(x != 0x8C) {
                break;
            }
        }
        {
            x = 0x1F;
            while cbool((x & 0x80) == 0) {
                engine.set_mem(u16v(0x0180 + x), 0x0F);
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.value = 0x0F;
        r.index = 0xFF;
    }
}

mod routine_0054 {
    use super::*;
    pub fn routine_0054(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl: i32 = engine.mem(0x23);
        let mut mask: i32 = engine.mem(0x24);
        let mut i: i32 = 0;
        engine.device_write(0x2000, ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, mask & 0xE7);
        engine.device_write(0x2006, 0x20);
        engine.device_write(0x2006, 0x00);
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0x9EC9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0x9FC9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0xA0C9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x100) {
                engine.device_write(0x2007, engine.mem(u16v(0xA1C9 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x2A, engine.mem(0xA2E9));
        engine.set_mem(0x2B, engine.mem(0xA2EA));
        engine.set_mem(0x24, mask);
        engine.set_mem(0x23, ctrl);
        engine.device_write(0x2000, ctrl);
        r.value = ctrl;
        r.index = 0;
    }
}

mod routine_0056 {
    use super::*;
    pub fn routine_0056(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        loop {
            let mut lo: i32 = engine.mem(u16v(0x0180 + x)) & 0x0F;
            engine.set_mem(0x08, lo);
            let mut hi: i32 = engine.mem(u16v(0x0180 + x)) & 0xF0;
            let mut sub: i32 = engine.mem(0x09);
            let mut res: i32 = 0;
            if cbool(hi >= sub) {
                res = u8v(u8v(hi - sub) | lo);
            } else {
                res = 0x0F;
            }
            engine.set_mem(u16v(0x0180 + x), res);
            {
                x += 1;
                x
            };
            {
                y -= 1;
                y
            };
            if !cbool(y != 0) {
                break;
            }
        }
        r.index = x;
        r.offset = y;
    }
}

mod routine_0057 {
    use super::*;
    pub fn routine_0057(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x1F;
            while cbool(x >= 0) {
                engine.set_mem(0x0180 + x, engine.mem(0xA2C9 + x));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0059 {
    use super::*;
    pub fn routine_0059(engine: &mut Engine, r: &mut RoutineContext) {
        let mut scrollpos: i32 = u8v((engine.mem(0x7C) << 4) | engine.mem(0x7B));
        let mut playerpos: i32 = u8v((engine.mem(0x44) << 4) | engine.mem(0x43));
        let mut delta: i32 = u8v(playerpos - scrollpos);
        let mut out_carry: i32 = 0;
        engine.set_mem(0x08, scrollpos);
        if cbool(delta < 0x60) {
            if cbool((engine.mem(0x7C) | engine.mem(0x7B)) == 0) {
                out_carry = 1;
            } else {
                let mut t: i32 = u8v(engine.mem(0x44) - 0x06);
                if cbool(engine.mem(0x44) < 0x06) {
                    engine.set_mem(0x7B, 0x00);
                    engine.set_mem(0x7C, 0x00);
                    out_carry = 0;
                } else {
                    engine.set_mem(0x7C, t);
                    engine.set_mem(0x7B, engine.mem(0x43));
                    engine.set_mem(0x7F, 0xFF);
                    out_carry = 0;
                }
            }
        } else if cbool(delta < 0x91) {
            out_carry = 1;
        } else {
            if cbool(engine.mem(0x7C) >= 0x30) {
                engine.set_mem(0x7C, 0x30);
                engine.set_mem(0x7B, 0x00);
                out_carry = 1;
            } else {
                engine.set_mem(0x7C, u8v(engine.mem(0x44) - 0x09));
                engine.set_mem(0x7B, engine.mem(0x43));
                engine.set_mem(0x7F, 0x01);
                out_carry = 0;
            }
        }
        routine_0060(engine, r);
        r.carry = u8v(out_carry);
    }
}

mod routine_0060 {
    use super::*;
    pub fn routine_0060(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x7C);
        let mut carry: i32 = 0;
        let mut i: i32 = 0;
        {
            i = 0;
            while cbool(i < 4) {
                carry = u8v(a >> 7);
                a = u8v(a << 1);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        a |= engine.mem(0x7B);
        r.index = a;
        a = u8v(0x00 << 1) | carry;
        engine.set_mem(0x1C, r.index);
        engine.set_mem(0x1D, a);
        r.value = a;
    }
}

mod routine_0061 {
    use super::*;
    pub fn routine_0061(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut world_x: i32 = 0;
        if (cbool(engine.mem(0x85) != 0) && cbool((engine.mem(0x84) & 0x01) == 0)) {
            engine.set_mem(0x0210, 0xEF);
            engine.set_mem(0x0214, 0xEF);
            return;
        }
        a = u8v(engine.mem(0x45) + 0x2B);
        engine.set_mem(0x0210, a);
        engine.set_mem(0x0214, a);
        world_x = u8v((engine.mem(0x7C) << 4) | engine.mem(0x7B));
        engine.set_mem(0x08, world_x);
        a = u8v((engine.mem(0x44) << 4) | engine.mem(0x43));
        a = u8v(a - world_x);
        engine.set_mem(0x0213, a);
        engine.set_mem(0x0217, u8v(a + 0x08));
        engine.set_mem(0x0212, engine.mem(0x57));
        engine.set_mem(0x0216, engine.mem(0x57));
        x = engine.mem(0x56);
        if cbool(engine.mem(0x57) & 0x40) {
            engine.set_mem(0x0215, x);
            engine.set_mem(0x0211, u8v(x + 2));
        } else {
            engine.set_mem(0x0211, x);
            engine.set_mem(0x0215, u8v(x + 2));
        }
    }
}

mod routine_0062 {
    use super::*;
    pub fn routine_0062(engine: &mut Engine, r: &mut RoutineContext) {
        let mut value: i32 = 0;
        let mut slot: i32 = 0;
        let mut offset: i32 = 0;
        value = engine.mem(0x0055);
        slot = 0x13;
        if cbool(value >= 0x03) {
            slot = 0xEF;
            engine.set_mem(0x0238, slot);
            engine.set_mem(0x023C, slot);
        } else {
            engine.set_mem(0x0238, slot);
            engine.set_mem(0x023C, slot);
            value = u8v(value << 4);
            value = u8v(value + 0xC8);
            engine.set_mem(0x023B, value);
            value = u8v(value + 0x08);
            engine.set_mem(0x023F, value);
            engine.set_mem(0x0239, 0xFF);
            engine.set_mem(0x023D, 0xFF);
            engine.set_mem(0x023A, 0x01);
            engine.set_mem(0x023E, 0x41);
        }
        slot = 0x02;
        offset = 0x10;
        loop {
            value = engine.mem(u16v((0x0051) + slot));
            if cbool(value & 0x80) {
                value = 0xEF;
            } else {
                value = u8v(value << 2);
                value = u8v(value + 0xA1);
                engine.set_mem(u16v(0x0221 + offset), value);
                value = u8v(value + 0x02);
                engine.set_mem(u16v(0x0225 + offset), value);
                value = u8v(offset << 1);
                value = u8v(value + 0xC8);
                engine.set_mem(u16v(0x0223 + offset), value);
                value = u8v(value + 0x08);
                engine.set_mem(u16v(0x0227 + offset), value);
                engine.set_mem(u16v(0x0222 + offset), 0x01);
                engine.set_mem(u16v(0x0226 + offset), 0x01);
                value = 0x13;
            }
            engine.set_mem(u16v(0x0220 + offset), value);
            engine.set_mem(u16v(0x0224 + offset), value);
            offset = u8v(offset - 0x08);
            if cbool(
                {
                    let __old = slot;
                    slot -= 1;
                    __old
                } == 0,
            ) {
                break;
            }
        }
    }
}

mod routine_0063 {
    use super::*;
    pub fn routine_0063(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        engine.set_mem(0x0A, 0x10);
        x = engine.mem(0x3F);
        y = engine.mem(0x3E);
        loop {
            r.index = x;
            r.offset = y;
            routine_0064(engine, r);
            x = u8v((u8v(x + 0x08)) | 0x80);
            y = u8v(y + 0x30);
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
            if !cbool(engine.mem(0x0A) != 0) {
                break;
            }
        }
        engine.set_mem(0x3F, u8v((u8v(x + 0x38)) | 0x80));
        engine.set_mem(0x3E, u8v(y + 0x10));
    }
}

mod routine_0064 {
    use super::*;
    pub fn routine_0064(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        let mut a: i32 = 0;
        let mut t: i32 = 0;
        let mut carry: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem((0x0400) + 1 + y) == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem((0x0400) + 0x0E + y) >= 0xBF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem((0x0400) + 2 + y);
                    engine.set_mem(0x0202 + x, a);
                    engine.set_mem(0x0206 + x, a);
                    if cbool(a & 0x40) {
                        a = engine.mem((0x0400) + y);
                        engine.set_mem(0x0205 + x, a);
                        a = u8v(a + 0x02);
                        engine.set_mem(0x0201 + x, a);
                    } else {
                        a = engine.mem((0x0400) + y);
                        engine.set_mem(0x0201 + x, a);
                        a = u8v(a + 0x02);
                        engine.set_mem(0x0205 + x, a);
                    }
                    {
                        let mut d: i32 =
                            u16v(engine.mem((0x0400) + 0x0C + y)) + 0x100 - engine.mem(0x007B);
                        a = u8v(d) & 0x0F;
                        engine.set_mem(0x08, a);
                        carry = u8v(d >> 8);
                        d = u16v(engine.mem((0x0400) + 0x0D + y)) + carry - engine.mem(0x007C) - 1;
                        a = u8v(d);
                        if cbool(a >= 0x10) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        a = u8v((a << 4) | engine.mem(0x08));
                        engine.set_mem(0x08, a);
                    }
                    if cbool(engine.mem((0x0400) + 1 + y) == 0x01) {
                        if cbool(engine.mem((0x0400) + 0x0F + y) != 0) {
                            engine.set_mem(
                                0x08,
                                u8v(engine.mem(0x08) + engine.mem((0x0400) + 0x0F + y)),
                            );
                            engine.set_mem((0x0400) + 0x0F + y, 0x00);
                        }
                    }
                    a = engine.mem(0x08);
                    if cbool(a >= 0xEF) {
                        engine.set_mem(0x0203 + x, a);
                        t = u8v(engine.mem((0x0400) + 0x0E + y) + 0x2B);
                        engine.set_mem(0x0200 + x, t);
                        engine.set_mem(0x0204 + x, 0xEF);
                        return;
                    }
                    engine.set_mem(0x0203 + x, a);
                    a = u8v(a + 0x08);
                    engine.set_mem(0x0207 + x, a);
                    t = u8v(engine.mem((0x0400) + 0x0E + y) + 0x2B);
                    engine.set_mem(0x0200 + x, t);
                    engine.set_mem(0x0204 + x, t);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x0200 + x, 0xEF);
                    engine.set_mem(0x0204 + x, 0xEF);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0065 {
    use super::*;
    pub fn routine_0065(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 3;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0200 + x), engine.mem(u16v(0xFF6B + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            x = 4;
            while cbool(x <= 0xFF) {
                engine.set_mem(u16v(0x0200 + x), 0xF8);
                {
                    let __old = x;
                    x += 1;
                    __old
                };
            }
        }
        r.index = 0x00;
    }
}

mod routine_0066 {
    use super::*;
    pub fn routine_0066(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl: i32 = engine.mem(0x23);
        let mut mask: i32 = engine.mem(0x24);
        let mut i: i32 = 0;
        engine.device_write(0x2000, ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, mask & 0xE7);
        engine.device_write(0x2006, 0x20);
        engine.device_write(0x2006, 0x00);
        {
            i = 0;
            while cbool(i < 5 * 0xC0) {
                engine.device_write(0x2007, 0xC0);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, 0x00);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 5 * 0xC0) {
                engine.device_write(0x2007, 0xC0);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            i = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, 0x00);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x24, mask);
        engine.set_mem(0x23, ctrl);
        engine.device_write(0x2000, ctrl);
        r.value = ctrl;
        r.index = 0;
        r.offset = 0;
    }
}

mod routine_0073 {
    use super::*;
    pub fn routine_0073(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut y: i32 = u8v(r.offset);
        loop {
            let mut lo: i32 = engine.mem(u16v(0x0180 + x)) & 0x0F;
            engine.set_mem(0x08, lo);
            let mut hi: i32 = engine.mem(u16v(0x0180 + x)) & 0xF0;
            let mut sub: i32 = engine.mem(0x09);
            let mut res: i32 = 0;
            if cbool(hi >= sub) {
                res = u8v(u8v(hi - sub) | lo);
            } else {
                res = 0x0F;
            }
            engine.set_mem(u16v(0x0180 + x), res);
            {
                x += 1;
                x
            };
            {
                y -= 1;
                y
            };
            if !cbool(y != 0) {
                break;
            }
        }
        r.index = x;
        r.offset = y;
    }
}

mod routine_0075 {
    use super::*;
    pub fn routine_0075(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0106(engine, r);
        engine.set_mem(0x16, 0x00);
        engine.set_mem(0x17, 0x3F);
        r.value = 0x02;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0076 {
    use super::*;
    pub fn routine_0076(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_ctrl: i32 = 0;
        let mut saved_mask: i32 = 0;
        let mut i: i32 = 0;
        routine_0106(engine, r);
        saved_ctrl = engine.mem(0x23);
        engine.device_write(0x2000, saved_ctrl & 0x7B);
        engine.set_mem(0x29, 0x00);
        saved_mask = engine.mem(0x24);
        engine.device_write(0x2001, saved_mask & 0xE7);
        engine.device_write(0x2006, 0x23);
        engine.device_write(0x2006, 0x20);
        {
            i = 0;
            while cbool(i < 0xA0) {
                engine.device_write(0x2007, engine.mem(u16v(0xFECB + i)));
                {
                    i += 1;
                    i
                };
            }
        }
        engine.device_write(0x2006, 0x23);
        engine.device_write(0x2006, 0xF0);
        {
            i = 0;
            while cbool(i < 0x10) {
                engine.device_write(0x2007, 0x00);
                {
                    i += 1;
                    i
                };
            }
        }
        engine.add_mem(0x29, 1);
        engine.set_mem(0x24, saved_mask);
        engine.set_mem(0x23, saved_ctrl);
        engine.device_write(0x2000, saved_ctrl);
        r.value = saved_ctrl;
        r.offset = 0x00;
    }
}

mod routine_0077 {
    use super::*;
    pub fn routine_0077(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x7C) & 0xFE);
        engine.set_mem(0x0D, 0x00);
        routine_0090(engine, r);
        routine_0079(engine, r);
    }
}

mod routine_0078 {
    use super::*;
    pub fn routine_0078(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x7C) & 0xFE);
        engine.set_mem(0x0D, 0x00);
        routine_0090(engine, r);
        engine.set_mem(0x0D, u8v((engine.mem(0x0D) - 0x05) + engine.mem(0x76)));
        routine_0079(engine, r);
    }
}

mod routine_0079 {
    use super::*;
    pub fn routine_0079(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ctrl_save: i32 = engine.mem(0x23);
        let mut v29_save: i32 = engine.mem(0x29);
        let mut v24_save: i32 = engine.mem(0x24);
        let mut c0c_save: i32 = engine.mem(0x0C);
        let mut c0d_save: i32 = engine.mem(0x0D);
        let mut p0C: i32 = 0;
        let mut p79: i32 = 0;
        let mut outer: i32 = 0;
        engine.device_write(0x2000, (ctrl_save & 0x7F) | 0x04);
        engine.set_mem(0x29, 0x00);
        engine.device_write(0x2001, v24_save & 0xE7);
        p79 = u16v(engine.mem(0x79) | (engine.mem(0x7A) << 8));
        {
            let mut sx: i32 = engine.mem(0x7C);
            let mut lo: i32 = u8v((sx << 1) & 0x1C);
            let mut hi: i32 = u8v((sx & 0x10) >> 2);
            let mut t: i32 = u16v(0x00 + lo);
            engine.set_mem(0x16, u8v(t));
            engine.set_mem(0x17, u8v(0x20 + hi + (t >> 8)));
        }
        engine.set_mem(0x0A, 0x12);
        p0C = u16v(c0c_save | (c0d_save << 8));
        {
            outer = 0;
            while cbool(outer < 0x12) {
                let mut inner: i32 = 0;
                engine.set_mem(0x0B, 0x0C);
                engine.device_write(0x2006, engine.mem(0x17));
                engine.device_write(0x2006, engine.mem(0x16));
                engine.set_mem(0x08, 0x00);
                loop {
                    let mut idx: i32 = engine.mem(u16v(p0C + engine.mem(0x08)));
                    let mut y: i32 = u8v(idx << 2);
                    engine.device_write(0x2007, engine.mem(u16v(p79 + y)));
                    engine.device_write(0x2007, engine.mem(u16v(p79 + u8v(y + 1))));
                    engine.inc_mem(0x08);
                    engine.dec_mem(0x0B);
                    if !cbool(engine.mem(0x0B) != 0) {
                        break;
                    }
                }
                engine.set_mem(0x0B, 0x0C);
                engine.device_write(0x2006, engine.mem(0x17));
                inner = u8v(engine.mem(0x16) + 1);
                engine.device_write(0x2006, inner);
                engine.set_mem(0x08, 0x00);
                loop {
                    let mut idx: i32 = engine.mem(u16v(p0C + engine.mem(0x08)));
                    let mut y: i32 = u8v((idx << 2) + 2);
                    engine.device_write(0x2007, engine.mem(u16v(p79 + y)));
                    engine.device_write(0x2007, engine.mem(u16v(p79 + u8v(y + 1))));
                    engine.inc_mem(0x08);
                    engine.dec_mem(0x0B);
                    if !cbool(engine.mem(0x0B) != 0) {
                        break;
                    }
                }
                engine.add_mem(0x16, 2);
                if cbool(engine.mem(0x16) & 0x20) {
                    engine.set_mem(0x16, 0x00);
                    engine.xor_mem(0x17, 0x04);
                }
                {
                    let mut t: i32 = u16v(0x0C + engine.mem(0x0C));
                    engine.set_mem(0x0C, u8v(t));
                    engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
                    p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                }
                engine.dec_mem(0x0A);
                {
                    let __old = outer;
                    outer += 1;
                    __old
                };
            }
        }
        engine.set_mem(0x0D, c0d_save);
        engine.set_mem(0x0C, c0c_save);
        p0C = u16v(c0c_save | (c0d_save << 8));
        {
            let mut sx: i32 = engine.mem(0x7C);
            let mut lo: i32 = u8v((sx >> 1) & 0x07);
            let mut hi: i32 = u8v((sx & 0x10) >> 2);
            let mut t: i32 = u16v(0xC0 + lo);
            engine.set_mem(0x16, u8v(t));
            engine.set_mem(0x17, u8v(0x23 + hi + (t >> 8)));
        }
        engine.set_mem(0x0A, 0x09);
        loop {
            let mut x: i32 = 0;
            {
                x = 6;
                while cbool(x > 0) {
                    let mut a: i32 = 0;
                    a = engine.mem(u16v(p0C + 0x0D));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x01));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x0C));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    a = engine.mem(u16v(p0C + 0x00));
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    {
                        let mut c1: i32 = (a >> 7) & 1;
                        a = u8v(a << 1);
                        engine.set_mem(0x08, u8v((engine.mem(0x08) << 1) | c1));
                    }
                    engine.device_write(0x2006, engine.mem(0x17));
                    engine.device_write(0x2006, engine.mem(0x16));
                    engine.device_write(0x2007, engine.mem(0x08));
                    {
                        let mut t: i32 = u16v(0x02 + engine.mem(0x0C));
                        engine.set_mem(0x0C, u8v(t));
                        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
                    }
                    {
                        let mut t: i32 = u16v(0x08 + engine.mem(0x16));
                        engine.set_mem(0x16, u8v(t));
                        engine.set_mem(0x17, u8v(engine.mem(0x17) + (t >> 8)));
                    }
                    p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                    {
                        let __old = x;
                        x -= 1;
                        __old
                    };
                }
            }
            {
                let mut t: i32 = u16v(0x0C + engine.mem(0x0C));
                engine.set_mem(0x0C, u8v(t));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (t >> 8)));
            }
            {
                let mut t: i32 = u16v(0xD1 + engine.mem(0x16));
                engine.set_mem(0x16, u8v(t));
                engine.set_mem(0x17, u8v(engine.mem(0x17) + 0xFF + (t >> 8)));
            }
            p0C = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            if cbool(engine.mem(0x16) & 0x08) {
                engine.set_mem(0x16, 0xC0);
                engine.xor_mem(0x17, 0x04);
            }
            engine.dec_mem(0x0A);
            if cbool(engine.mem(0x0A) == 0) {
                break;
            }
        }
        engine.set_mem(0x24, v24_save);
        engine.set_mem(0x29, v29_save);
        engine.set_mem(0x23, ctrl_save);
        engine.device_write(0x2000, ctrl_save);
        r.value = ctrl_save;
        r.index = 0;
    }
}

mod routine_0080 {
    use super::*;
    pub fn routine_0080(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sx: i32 = 0;
        routine_0106(engine, r);
        sx = engine.mem(0x7C);
        engine.set_mem(0x16, u8v((sx << 1) & 0x1F));
        engine.set_mem(0x17, u8v((sx & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        engine.set_mem(0x08, sx);
        engine.set_mem(0x09, 0x10);
        loop {
            engine.set_mem(0x0C, engine.mem(0x08));
            farcall_bank_09_r7(engine, r);
            engine.set_mem(0x16, u8v(engine.mem(0x16) + 2));
            if cbool(engine.mem(0x16) & 0x20) {
                engine.set_mem(0x16, 0x00);
                engine.xor_mem(0x17, 0x04);
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            engine.set_mem(0x09, u8v(engine.mem(0x09) - 1));
            if !cbool(engine.mem(0x09) != 0) {
                break;
            }
        }
    }
}

mod routine_0081 {
    use super::*;
    pub fn routine_0081(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sx: i32 = 0;
        routine_0106(engine, r);
        sx = engine.mem(0x7C);
        engine.set_mem(0x16, u8v((sx << 1) & 0x1F));
        engine.set_mem(0x17, u8v((sx & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        engine.set_mem(0x08, sx);
        engine.set_mem(0x09, 0x10);
        loop {
            engine.set_mem(0x0C, engine.mem(0x08));
            routine_0083(engine, r);
            engine.set_mem(0x16, u8v(engine.mem(0x16) + 2));
            if cbool(engine.mem(0x16) & 0x20) {
                engine.set_mem(0x16, 0x00);
                engine.xor_mem(0x17, 0x04);
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            engine.set_mem(0x09, u8v(engine.mem(0x09) - 1));
            if !cbool(engine.mem(0x09) != 0) {
                break;
            }
        }
    }
}

mod routine_0082 {
    use super::*;
    pub fn routine_0082(engine: &mut Engine, r: &mut RoutineContext) {
        let mut col: i32 = 0;
        routine_0106(engine, r);
        if cbool(engine.mem(0x7F) & 0x80) {
            col = engine.mem(0x7C);
        } else {
            col = u8v(engine.mem(0x7C) + 0x10);
        }
        engine.set_mem(0x0C, col);
        engine.set_mem(0x16, u8v((col << 1) & 0x1F));
        engine.set_mem(0x17, u8v((col & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        farcall_bank_09_r7(engine, r);
    }
}

mod routine_0083 {
    use super::*;
    pub fn routine_0083(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0D, 0x00);
        routine_0090(engine, r);
        engine.set_mem(0x0D, u8v(u8v(engine.mem(0x0D) - 0x05) + engine.mem(0x76)));
        metasprite_build(engine, r);
    }
}

mod routine_0084 {
    use super::*;
    pub fn routine_0084(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0086(engine, r);
        text_attr_build(engine, r);
        routine_0087(engine, r);
    }
}

mod routine_0085 {
    use super::*;
    pub fn routine_0085(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut ptr: i32 = 0;
        let mut i: i32 = 0;
        engine.set_mem(0x77, engine.mem(0x75));
        engine.set_mem(0x78, engine.mem(0x76));
        lo = engine.mem(0x77);
        hi = engine.mem(0x78);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0500 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0600 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        ptr = u16v(lo | (hi << 8));
        {
            i = 0;
            while cbool(i < 256) {
                engine.set_mem(u16v(0x0700 + i), engine.mem(u16v(ptr + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        {
            let __old = hi;
            hi += 1;
            __old
        };
        engine.set_mem(0x78, hi);
        r.offset = 0;
    }
}

mod routine_0086 {
    use super::*;
    pub fn routine_0086(engine: &mut Engine, r: &mut RoutineContext) {
        let mut bank: i32 = u8v(engine.mem(0x48) >> 1);
        let mut t: i32 = 0;
        let mut lo: i32 = 0;
        if cbool(bank != engine.mem(0x30)) {
            engine.set_mem(0x30, bank);
            r.value = 0xFF;
            queue_ppu_job_and_wait(engine, r);
        }
        t = u8v(((engine.mem(0x48) & 0x01) << 2));
        t = u8v((t | engine.mem(0x47)) << 2);
        lo = u8v(t + 0x80);
        engine.set_mem(0x76, lo);
        engine.set_mem(0x78, u8v(lo + 0x03));
        engine.set_mem(0x77, 0x00);
        engine.set_mem(0x75, 0x00);
        r.carry = u8v((if cbool((lo + 0x03) > 0xFF) { 1 } else { 0 }));
    }
}

mod routine_0087 {
    use super::*;
    pub fn routine_0087(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        {
            y = 0xE0;
            while cbool(y <= 0xFF) {
                engine.set_mem(u16v(0x00A0 + u8v(y)), engine.mem(u16v(ptr + u8v(y))));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
            }
        }
        a = engine.mem(0x40);
        if cbool(a >= 0x06) {
            r.value = a;
            r.carry = 1;
            return;
        }
        a = u8v((a << 2) + 0x03);
        x = a;
        {
            y = 0x03;
            while cbool(y >= 0) {
                engine.set_mem(u16v(0x0190 + y), engine.mem(u16v(0xFFC5 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
                {
                    let __old = y;
                    y -= 1;
                    __old
                };
            }
        }
        r.value = a;
        r.index = x;
        r.offset = u8v(0xFF);
        r.carry = 0;
    }
}

mod routine_0088 {
    use super::*;
    pub fn routine_0088(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ms_y: i32 = engine.mem(0x48);
        let mut ms_x: i32 = engine.mem(0x47);
        let mut idx: i32 = u8v(((ms_y << 2) & 0x04) | ms_x);
        let mut a: i32 = engine.mem(u16v(0x0300 + idx));
        let mut cnt: i32 = u8v((ms_y >> 1) + 1);
        loop {
            a = u8v(a << 1);
            if !cbool(
                {
                    cnt -= 1;
                    cnt
                } != 0,
            ) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0089 {
    use super::*;
    pub fn routine_0089(engine: &mut Engine, r: &mut RoutineContext) {
        let mut msy: i32 = engine.mem(0x48);
        let mut x: i32 = u8v((msy >> 1) + 1);
        let mut a: i32 = 0xFF;
        let mut carry: i32 = 0;
        let mut idx: i32 = 0;
        loop {
            let mut newcarry: i32 = a & 1;
            a = u8v((carry << 7) | (a >> 1));
            carry = newcarry;
            {
                let __old = x;
                x -= 1;
                __old
            };
            if !cbool(x != 0) {
                break;
            }
        }
        idx = u8v(((u8v(msy << 2)) & 0x04) | engine.mem(0x47));
        engine.and_mem(u16v(0x0300 + idx), a);
        r.value = engine.mem(u16v(0x0300 + idx));
        r.index = idx;
    }
}

mod routine_0090 {
    use super::*;
    pub fn routine_0090(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_0d: i32 = engine.mem(0x0D);
        routine_0091(engine, r);
        engine.set_mem(0x11, engine.mem(0x0D));
        {
            let mut a: i32 = u8v(saved_0d >> 4);
            let mut s: i32 = u16v(a + engine.mem(0x0C));
            engine.set_mem(0x0C, u8v(s));
            engine.set_mem(0x10, u8v(s));
            if cbool(s & 0x100) {
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 1));
                engine.set_mem(0x11, u8v(engine.mem(0x11) + 1));
            }
        }
        engine.set_mem(0x0D, u8v(engine.mem(0x0D) + 0x05));
        {
            let mut lo: i32 = u16v(engine.mem(0x10) + engine.mem(0x75));
            let mut carry: i32 = u8v(lo >> 8);
            engine.set_mem(0x10, u8v(lo));
            engine.set_mem(0x11, u8v(engine.mem(0x11) + engine.mem(0x76) + carry));
        }
    }
}

mod routine_0091 {
    use super::*;
    pub fn routine_0091(engine: &mut Engine, r: &mut RoutineContext) {
        let mut four: i32 = u16v(engine.mem(0x0C) << 2);
        let mut eight: i32 = u16v(engine.mem(0x0C) << 3);
        let mut result: i32 = u16v(four + eight);
        let mut x: i32 = u8v(four >> 8);
        let mut y: i32 = u8v(four);
        engine.set_mem(0x0C, u8v(result));
        engine.set_mem(0x0D, u8v(result >> 8));
        r.index = x;
        r.offset = y;
        r.value = u8v(result >> 8);
    }
}

mod routine_0092 {
    use super::*;
    pub fn routine_0092(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0106(engine, r);
        engine.set_mem(0x16, 0x60);
        engine.set_mem(0x17, 0x23);
        r.value = 0x04;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0093 {
    use super::*;
    pub fn routine_0093(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = engine.mem(0x58);
        if cbool(v >= 0x6D) {
            v = 0x6D;
        }
        engine.set_mem(0x58, v);
        engine.set_mem(0x08, v);
        r.value = v;
        r.index = 0x00;
        routine_0097(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod routine_0094 {
    use super::*;
    pub fn routine_0094(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = engine.mem(0x59);
        if cbool(v >= 0x6D) {
            v = 0x6D;
        }
        engine.set_mem(0x59, v);
        engine.set_mem(0x08, v);
        r.value = v;
        r.index = 0x06;
        routine_0097(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod routine_0095 {
    use super::*;
    pub fn routine_0095(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = engine.mem(0x5B);
        if cbool(v >= 0x6D) {
            v = 0x6D;
        }
        engine.set_mem(0x5B, v);
        engine.set_mem(0x08, v);
        r.value = v;
        r.index = 0x0C;
        routine_0097(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod routine_0096 {
    use super::*;
    pub fn routine_0096(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = engine.mem(0x5A);
        if cbool(v >= 0x6D) {
            v = 0x6D;
        }
        engine.set_mem(0x5A, v);
        engine.set_mem(0x08, v);
        r.value = v;
        r.index = 0x12;
        routine_0097(engine, r);
        r.value = 0x01;
        engine.set_mem(0x3C, 0x01);
    }
}

mod routine_0097 {
    use super::*;
    pub fn routine_0097(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: i32 = 0;
        let mut i: i32 = 0;
        a = r.index;
        engine.set_mem((0x01FB), a);
        x = a;
        {
            i = 0;
            while cbool(i < 5) {
                engine.set_mem(
                    u16v(
                        0x0101 + {
                            let __old = x;
                            x += 1;
                            __old
                        },
                    ),
                    0xDC,
                );
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        a = engine.mem((0x01FB));
        engine.set_mem((0x01FB), a);
        x = a;
        {
            i = 0;
            while cbool(i < 5) {
                engine.set_mem(
                    u16v(
                        0x0121 + {
                            let __old = x;
                            x += 1;
                            __old
                        },
                    ),
                    0xDF,
                );
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        a = engine.mem((0x01FB));
        x = a;
        r.index = x;
        routine_0102(engine, r);
        y = r.offset;
        a = x;
        x = a;
        loop {
            y = u8v(y - 1);
            if cbool(y == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0101 + x));
            y = u8v(y - 1);
            if cbool(y == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0101 + x));
            x = u8v(x + 1);
        }
        x = a;
        y = engine.mem(0x08);
        loop {
            y = u8v(y - 1);
            if cbool(y == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0121 + x));
            y = u8v(y - 1);
            if cbool(y == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0121 + x));
            x = u8v(x + 1);
        }
        r.offset = y;
        r.index = x;
        r.value = a;
    }
}

mod routine_0098 {
    use super::*;
    pub fn routine_0098(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x0405);
        if cbool(a >= 0x6D) {
            a = 0x6D;
        }
        engine.set_mem(0x08, a);
        engine.set_mem(0x09, 0x00);
        r.index = 0xA5;
        r.offset = 0xAB;
        routine_0101(engine, r);
    }
}

mod routine_0099 {
    use super::*;
    pub fn routine_0099(engine: &mut Engine, r: &mut RoutineContext) {
        let mut value: i32 = 0;
        let mut slot: i32 = 0;
        let mut count: i32 = 0;
        value = engine.mem(0x0405);
        if cbool(value >= 0x6D) {
            value = 0x6D;
        }
        engine.set_mem(0x08, value);
        engine.set_mem(0x09, 0x00);
        slot = 0x65;
        count = 0x6B;
        value = slot;
        slot = engine.mem(0x09);
        engine.set_mem(u16v(0x0259 + slot), value);
        engine.set_mem(u16v(0x025D + slot), value);
        engine.set_mem(u16v(0x0261 + slot), value);
        engine.set_mem(u16v(0x0265 + slot), value);
        engine.set_mem(u16v(0x0269 + slot), value);
        value = count;
        engine.set_mem(u16v(0x026D + slot), value);
        engine.set_mem(u16v(0x0271 + slot), value);
        engine.set_mem(u16v(0x0275 + slot), value);
        engine.set_mem(u16v(0x0279 + slot), value);
        engine.set_mem(u16v(0x027D + slot), value);
        routine_0102(engine, r);
        count = r.offset;
        slot = u8v(engine.mem(0x09) + 0x18);
        loop {
            count = u8v(count - 1);
            if cbool(count == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + slot));
            engine.dec_mem(u16v(0x0241 + slot));
            count = u8v(count - 1);
            if cbool(count == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + slot));
            engine.dec_mem(u16v(0x0241 + slot));
            slot = u8v(slot + 4);
        }
        slot = u8v(engine.mem(0x09) + 0x2C);
        count = engine.mem(0x08);
        loop {
            count = u8v(count - 1);
            if cbool(count == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + slot));
            engine.dec_mem(u16v(0x0241 + slot));
            count = u8v(count - 1);
            if cbool(count == 0) {
                break;
            }
            engine.dec_mem(u16v(0x0241 + slot));
            engine.dec_mem(u16v(0x0241 + slot));
            slot = u8v(slot + 4);
        }
        r.value = value;
        r.index = slot;
        r.offset = count;
    }
}

mod routine_0100 {
    use super::*;
    pub fn routine_0100(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x58);
        if cbool(a >= 0x6D) {
            a = 0x6D;
        }
        engine.set_mem(0x08, a);
        engine.set_mem(0x09, 0x80);
        r.index = 0x65;
        r.offset = 0x6B;
        routine_0101(engine, r);
    }
}

mod routine_0101 {
    use super::*;
    pub fn routine_0101(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x09);
        let mut full: i32 = u8v(r.index);
        engine.set_mem(u16v(0x0259 + x), full);
        engine.set_mem(u16v(0x025D + x), full);
        engine.set_mem(u16v(0x0261 + x), full);
        engine.set_mem(u16v(0x0265 + x), full);
        engine.set_mem(u16v(0x0269 + x), full);
        {
            let mut empty: i32 = u8v(r.offset);
            engine.set_mem(u16v(0x026D + x), empty);
            engine.set_mem(u16v(0x0271 + x), empty);
            engine.set_mem(u16v(0x0275 + x), empty);
            engine.set_mem(u16v(0x0279 + x), empty);
            engine.set_mem(u16v(0x027D + x), empty);
        }
        routine_0102(engine, r);
        {
            let mut y: i32 = u8v(r.offset);
            let mut xx: i32 = u8v(engine.mem(0x09) + 0x18);
            loop {
                if cbool(
                    {
                        y -= 1;
                        y
                    } == 0,
                ) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + xx), 2);
                if cbool(
                    {
                        y -= 1;
                        y
                    } == 0,
                ) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + xx), 2);
                xx = u8v(xx + 4);
            }
        }
        {
            let mut xx: i32 = u8v(engine.mem(0x09) + 0x2C);
            let mut y: i32 = engine.mem(0x08);
            loop {
                if cbool(
                    {
                        y -= 1;
                        y
                    } == 0,
                ) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + xx), 2);
                if cbool(
                    {
                        y -= 1;
                        y
                    } == 0,
                ) {
                    break;
                }
                engine.sub_mem(u16v(0x0241 + xx), 2);
                xx = u8v(xx + 4);
            }
        }
    }
}

mod routine_0102 {
    use super::*;
    pub fn routine_0102(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x08);
        let mut y: i32 = 0;
        let mut carry: i32 = 1;
        loop {
            let mut t: i32 = 0;
            y = u8v(y + 1);
            t = (a) - 0x0A - (1 - carry);
            a = u8v(t);
            carry = u8v((if cbool(t >= 0) { 1 } else { 0 }));
            if !cbool(carry) {
                break;
            }
        }
        a = u8v(a + 0x0B + carry);
        engine.set_mem(0x08, a);
        r.value = a;
        r.offset = y;
    }
}

mod routine_0103 {
    use super::*;
    pub fn routine_0103(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0104(engine, r);
        routine_0105(engine, r);
        {
            let mut btn: i32 = u8v(r.value);
            routine_0104(engine, r);
            r.value = btn;
            engine.set_mem(0x20, btn);
        }
    }
}

mod routine_0106 {
    use super::*;
    pub fn routine_0106(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x28, 0);
    }
}

mod routine_0107 {
    use super::*;
    pub fn routine_0107(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = u8v(r.offset);
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        let mut sign_fill: i32 = 0;
        engine.set_mem(0x09, y);
        if cbool(y == 0) {
            engine.set_mem(0x49, 0);
            engine.set_mem(0x4A, 0);
            engine.set_mem(0x4B, 0);
            return;
        }
        x = u8v((engine.mem(0x20) & 0x0F) << 1);
        a = 0;
        {
            c = y;
            while cbool(c != 0) {
                a = u8v(a + engine.mem(0xFE8B + x));
                {
                    let __old = c;
                    c -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x49, a & 0x0F);
        sign_fill = (if cbool(a & 0x80) { 0xF0 } else { 0x00 });
        engine.set_mem(0x08, sign_fill);
        engine.set_mem(0x4A, u8v(((a & 0xF0) >> 4) | sign_fill));
        a = 0;
        {
            c = y;
            while cbool(c != 0) {
                a = u8v(a + engine.mem(0xFE8C + x));
                {
                    let __old = c;
                    c -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x4B, a);
    }
}

mod routine_0108 {
    use super::*;
    pub fn routine_0108(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = u8v(r.offset);
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut c: i32 = 0;
        let mut sign_fill: i32 = 0;
        engine.set_mem(0x09, y);
        if cbool(y == 0) {
            engine.set_mem(0xF5, 0);
            engine.set_mem(0xF6, 0);
            engine.set_mem(0xF7, 0);
            return;
        }
        x = u8v((r.value & 0x0F) << 1);
        a = 0;
        {
            c = y;
            while cbool(c != 0) {
                a = u8v(a + engine.mem(0xFE8B + x));
                {
                    let __old = c;
                    c -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0xF5, a & 0x0F);
        sign_fill = (if cbool(a & 0x80) { 0xF0 } else { 0x00 });
        engine.set_mem(0x08, sign_fill);
        engine.set_mem(0xF6, u8v(((a & 0xF0) >> 4) | sign_fill));
        a = 0;
        {
            c = y;
            while cbool(c != 0) {
                a = u8v(a + engine.mem(0xFE8C + x));
                {
                    let __old = c;
                    c -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0xF7, a);
    }
}

mod routine_0111 {
    use super::*;
    pub fn routine_0111(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xEA, 0x00);
        routine_0113(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        routine_0112(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        engine.set_mem(0xEA, 0x01);
        r.carry = 1;
    }
}

mod routine_0112 {
    use super::*;
    pub fn routine_0112(engine: &mut Engine, r: &mut RoutineContext) {
        let mut d: i32 = u8v(engine.mem(0x0F) - engine.mem(0x44));
        if cbool(d == 0) {
            return;
        }
        if cbool(d < 0x02) {
            let mut f: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
            r.carry = (if cbool(f & 0x80) { 1 } else { 0 });
            return;
        }
        if cbool(d < 0xFF) {
            return;
        }
        {
            let mut f: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
            if cbool(f == 0) {
                return;
            }
            if cbool(f & 0x80) {
                return;
            }
            r.carry = 1;
        }
    }
}

mod routine_0113 {
    use super::*;
    pub fn routine_0113(engine: &mut Engine, r: &mut RoutineContext) {
        let mut diff: i32 = u8v(engine.mem(0x0A) - engine.mem(0x45));
        if cbool(diff < 0x10) {
            r.carry = 1;
        } else if cbool(diff < 0xF1) {
            r.carry = 0;
        } else {
            r.carry = 1;
        }
    }
}

mod routine_0114 {
    use super::*;
    pub fn routine_0114(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dy: i32 = 0;
        let mut dx: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xEA, 0x00);
                    dy = u8v(engine.mem(0x0A) - engine.mem(0x45));
                    if (cbool(dy >= 0x10) && cbool(dy < 0xE1)) {
                        r.carry = 0;
                        return;
                    }
                    dx = u8v(engine.mem(0x0F) - engine.mem(0x44));
                    if cbool(dx == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(dx == 0xFF) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(dx < 0x02) {
                        let mut f: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
                        if cbool(f & 0x80) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.carry = 0;
                        return;
                    }
                    if cbool(dx < 0xFE) {
                        return;
                    }
                    {
                        let mut f: i32 = u8v(engine.mem(0x0E) - engine.mem(0x43));
                        if cbool(f == 0) {
                            return;
                        }
                        if cbool(f & 0x80) {
                            return;
                        }
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xEA, 0x01);
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0115 {
    use super::*;
    pub fn routine_0115(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xC0) {
            r.carry = 1;
        } else if cbool(engine.mem(0x0F) < 0x3F) {
            r.carry = 0;
        } else if cbool(engine.mem(0x0E) == 0) {
            r.carry = 0;
        } else {
            r.carry = 1;
        }
    }
}

mod routine_0116 {
    use super::*;
    pub fn routine_0116(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0A) >= 0xB0) {
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(0x0F) < 0x3F) {
            r.carry = 0;
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            r.carry = 0;
            return;
        }
        r.carry = 1;
    }
}

mod routine_0117 {
    use super::*;
    pub fn routine_0117(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x0F;
            while cbool(x >= 0) {
                r.index = u8v(x);
                r.offset = engine.mem(u16v(0x0060 + x));
                routine_0118(engine, r);
                r.index = u8v(x);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.index = 0xFF;
    }
}

mod routine_0118 {
    use super::*;
    pub fn routine_0118(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut s: i32 = 0;
        lo = u8v((x & 0x07) << 2);
        lo = u8v(((x & 0x08) << 4) | lo);
        hi = 0x00;
        s = u16v(0xC2 + lo);
        engine.set_mem(0x16, u8v(s));
        engine.set_mem(0x17, u8v(0x20 + hi + (s >> 8)));
        r.value = r.offset;
        routine_0121(engine, r);
        {
            let mut in_: i32 = x;
            let mut dx: i32 = u8v(engine.mem(0x40) << 1);
            let mut yy: i32 = 0;
            let mut carry: i32 = 0;
            let mut v: i32 = 0;
            if cbool(in_ >= 0x08) {
                {
                    let __old = dx;
                    dx += 1;
                    __old
                };
            }
            yy = u8v((in_ & 0x07) + 1);
            v = engine.mem(u16v(0xFFBB + dx));
            carry = 0;
            loop {
                carry = u8v(v >> 7);
                v = u8v(v << 1);
                if !cbool(
                    {
                        yy -= 1;
                        yy
                    } != 0,
                ) {
                    break;
                }
            }
            r.carry = carry;
        }
        r.value = x;
        routine_0122(engine, r);
        if !cbool(r.carry) {
            engine.set_mem(0x18, u8v(engine.mem(0x18) - 0x40));
            engine.set_mem(0x19, u8v(engine.mem(0x19) - 0x40));
        }
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0119 {
    use super::*;
    pub fn routine_0119(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x16, 0xDE);
        engine.set_mem(0x17, 0x21);
        routine_0125(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x1E);
        engine.set_mem(0x17, 0x22);
        routine_0124(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x5E);
        engine.set_mem(0x17, 0x22);
        routine_0126(engine, r);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0120 {
    use super::*;
    pub fn routine_0120(engine: &mut Engine, r: &mut RoutineContext) {
        let mut lo: i32 = 0;
        let mut hi: i32 = 0;
        let mut c: i32 = 0;
        engine.set_mem(0x16, 0x47);
        engine.set_mem(0x17, 0x22);
        if cbool(engine.mem(0x7C) & 0x10) {
            let mut s: i32 = u16v(0x00 + engine.mem(0x16));
            engine.set_mem(0x16, u8v(s));
            engine.set_mem(0x17, u8v(0x04 + engine.mem(0x17) + (s >> 8)));
        }
        r.value = engine.mem(0x81);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
        lo = engine.mem(0x16);
        c = u8v((0x0E + lo) >> 8);
        engine.set_mem(0x16, u8v(0x0E + lo));
        hi = engine.mem(0x17);
        engine.set_mem(0x17, u8v(0x00 + hi + c));
        r.value = engine.mem(0x83);
        routine_0121(engine, r);
        r.value = 0x06;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0121 {
    use super::*;
    pub fn routine_0121(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        let mut hi: i32 = 0xD0;
        while cbool(a >= 0x0A) {
            a = u8v(a - 0x0A);
            {
                hi += 1;
                hi
            };
        }
        a = u8v(a + 0xD0);
        engine.set_mem(0x18, a);
        if cbool(hi == 0xD0) {
            hi = 0xC0;
        }
        engine.set_mem(0x19, hi);
    }
}

mod routine_0122 {
    use super::*;
    pub fn routine_0122(engine: &mut Engine, r: &mut RoutineContext) {
        let mut in_: i32 = u8v(r.value);
        let mut x: i32 = u8v(engine.mem(0x40) << 1);
        if cbool(in_ >= 0x08) {
            {
                let __old = x;
                x += 1;
                __old
            };
        }
        let mut y: i32 = u8v((in_ & 0x07) + 1);
        let mut a: i32 = engine.mem(u16v(0xFFBB + x));
        loop {
            a = u8v(a << 1);
            if !cbool(
                {
                    y -= 1;
                    y
                } != 0,
            ) {
                break;
            }
        }
        r.value = a;
    }
}

mod routine_0123 {
    use super::*;
    pub fn routine_0123(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(r.value == engine.mem(0x8E)) {
            return;
        }
        engine.set_mem(0x8E, r.value);
        song_init(engine, r);
    }
}

mod routine_0124 {
    use super::*;
    pub fn routine_0124(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x55);
        let mut item: i32 = engine.mem((0x51 + x) & 0xFF);
        r.index = x;
        if (cbool(item == 0x06) && cbool(engine.mem(0x59) != 0)) {
            let mut jump: i32 = engine.mem(0x5C);
            r.value = u8v((jump >> 2) + jump);
            r.carry = 0;
        } else {
            r.value = engine.mem(0x5C);
            r.carry = 1;
        }
    }
}

mod routine_0125 {
    use super::*;
    pub fn routine_0125(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x55);
        let mut item: i32 = engine.mem((0x51 + x) & 0xFF);
        if (cbool(item == 0x08) && cbool(engine.mem(0x59) != 0)) {
            r.value = u8v(engine.mem(0x5D) << 2);
            r.carry = 0;
        } else {
            r.value = engine.mem(0x5D);
            r.carry = 1;
        }
    }
}

mod routine_0126 {
    use super::*;
    pub fn routine_0126(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x55);
        r.index = x;
        if (cbool(engine.mem((0x51 + x) & 0xFF) == 0x09) && cbool(engine.mem(0x59) != 0)) {
            r.value = u8v(engine.mem(0x5F) << 1);
            r.carry = 0;
            return;
        }
        r.value = engine.mem(0x5F);
        r.carry = 1;
    }
}

mod routine_0127 {
    use super::*;
    pub fn routine_0127(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x80;
        loop {
            engine.set_mem(u16v(0x0200 + x), 0xEF);
            x = u8v(x + 4);
            if !cbool(x != 0) {
                break;
            }
        }
        r.index = x;
        r.value = 0xEF;
    }
}

mod routine_0128 {
    use super::*;
    pub fn routine_0128(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x00;
        let mut y: i32 = 0x10;
        loop {
            engine.set_mem(u16v(0x0401 + x), 0x00);
            engine.set_mem(u16v(0x0406 + x), 0x02);
            x = u8v(x + 0x10);
            if !cbool(
                {
                    y -= 1;
                    y
                } != 0,
            ) {
                break;
            }
        }
        engine.set_mem(0xE9, 0x00);
        r.value = 0x00;
        r.index = x;
        r.offset = 0x00;
    }
}

mod routine_0129 {
    use super::*;
    pub fn routine_0129(engine: &mut Engine, r: &mut RoutineContext) {
        let mut i: i32 = 0;
        {
            i = 7;
            while cbool(i >= 0) {
                engine.set_mem(0x0308 + i, engine.mem(0x0300 + i));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        {
            i = 15;
            while cbool(i >= 0) {
                engine.set_mem(0x0310 + i, engine.mem(0x0060 + i));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x0321, engine.mem(0x5A));
        engine.set_mem(0x0320, engine.mem(0x5B));
        r.index = 0xFF;
    }
}

mod routine_0130 {
    use super::*;
    pub fn routine_0130(engine: &mut Engine, r: &mut RoutineContext) {
        let mut i: i32 = 0;
        {
            i = 7;
            while cbool(i >= 0) {
                engine.set_mem(0x0300 + i, engine.mem(0x0308 + i));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        {
            i = 15;
            while cbool(i >= 0) {
                engine.set_mem(0x0060 + i, engine.mem(0x0310 + i));
                {
                    let __old = i;
                    i -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x5A, engine.mem(0x0321));
        engine.set_mem(0x5B, engine.mem(0x0320));
        r.index = 0xFF;
    }
}

mod routine_0131 {
    use super::*;
    pub fn routine_0131(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0x1F;
        let mut x: i32 = 0x26;
        let mut i: i32 = 0;
        loop {
            {
                i = 0;
                while cbool(i < 4) {
                    let mut out: i32 = u8v(engine.mem(u16v(0x0322 + y)) | 0x80);
                    if cbool(out >= 0xA0) {
                        out = 0x7F;
                    }
                    engine.set_mem(u16v(0x0362 + (x & 0xFF)), out);
                    x = (x - 1) & 0xFF;
                    y = (y - 1) & 0xFF;
                    {
                        i += 1;
                        i
                    };
                }
            }
            x = (x - 1) & 0xFF;
            if !cbool((x & 0x80) == 0) {
                break;
            }
        }
        engine.set_mem(0x1A, 0x13);
        engine.set_mem(0x1B, 0x00);
        engine.set_mem(0x16, 0xE6);
        engine.set_mem(0x17, 0x24);
        engine.set_mem(0x18, 0x62);
        engine.set_mem(0x19, 0x03);
        r.value = 0x05;
        queue_ppu_job_and_wait(engine, r);
        engine.set_mem(0x16, 0x06);
        engine.set_mem(0x17, 0x25);
        engine.set_mem(0x18, 0x76);
        engine.set_mem(0x19, 0x03);
        r.value = 0x05;
        queue_ppu_job_and_wait(engine, r);
    }
}

mod routine_0132 {
    use super::*;
    pub fn routine_0132(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x1F;
            while cbool(x >= 0) {
                engine.set_mem(0x0322 + x, 0x7F);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.value = 0x7F;
        r.index = 0xFF;
    }
}

mod routine_0135 {
    use super::*;
    pub fn routine_0135(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x4F) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x22) != 0) {
                        return;
                    }
                    engine.set_mem(0x8F, 0x1B);
                    engine.set_mem(0x4F, engine.mem(0x5C));
                    {
                        let mut x: i32 = engine.mem(0x55);
                        if cbool(engine.mem(u16v(0x51 + x)) == 0x06) {
                            routine_0204(engine, r);
                            if !cbool(r.carry) {
                                let mut f: i32 = engine.mem(0x4F);
                                engine.set_mem(0x4F, u8v((f >> 2) + f));
                            }
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.lotw_nonlocal_handoff = 1;
                    engine.set_mem(0x22, 0x01);
                    {
                        let mut old4f: i32 = engine.mem(0x4F);
                        engine.set_mem(0x4F, u8v(old4f - 1));
                        let mut t: i32 = u8v(old4f >> 2);
                        engine.set_mem(0x4B, u8v((t ^ 0xFF) + 1));
                    }
                    routine_0146(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x49, 0x00);
                    engine.set_mem(0x4A, 0x00);
                    routine_0146(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.inc_mem(0x4F);
                    routine_0173(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x43, engine.mem(0x0E));
                    engine.set_mem(0x44, engine.mem(0x0F));
                    {
                        let mut y: i32 = engine.mem(0x0A);
                        if cbool(y >= 0xEF) {
                            y = 0x00;
                        }
                        engine.set_mem(0x45, y);
                    }
                    routine_0163(engine, r);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x4F, 0x00);
                    engine.set_mem(0x4E, 0x00);
                    routine_0163(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    routine_0144(engine, r);
                    routine_0145(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0136 {
    use super::*;
    pub fn routine_0136(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = engine.mem(0x0055);
        let mut x: i32 = engine.mem(u16v((0x0051) + y));
        if cbool(x >= 0x02) {
            if cbool(x == 0x0B) {
                if cbool(engine.mem(0x59) != 0) {
                    return;
                }
                x = engine.mem(0x0055);
                engine.set_mem(u16v((0x0051) + x), 0xFF);
                routine_0062(engine, r);
                routine_0134(engine, r);
                return;
            }
            if cbool(x != 0x0D) {
                return;
            }
            if cbool(engine.mem(0x0048) >= 0x11) {
                engine.set_mem(0x0055, 0x03);
                return;
            }
            x = engine.mem(0x0055);
            engine.set_mem(u16v((0x0051) + x), 0xFF);
            routine_0062(engine, r);
            engine.set_mem(0x8F, 0x12);
            engine.set_mem(0x0048, 0x10);
            engine.set_mem(0x0047, 0x03);
            engine.set_mem(0x007C, 0x12);
            engine.set_mem(0x0045, 0xB0);
            engine.set_mem(0x0044, 0x1A);
            engine.set_mem(0x0043, 0x00);
            engine.set_mem(0x007B, 0x00);
            routine_0067(engine, r);
            routine_0128(engine, r);
            scene_assemble(engine, r);
            routine_0077(engine, r);
            routine_0127(engine, r);
            routine_0060(engine, r);
            routine_0061(engine, r);
            routine_0070(engine, r);
            r.carry = 1;
            return;
        }
        if cbool(engine.mem(u16v(0x86 + x)) != 0) {
            return;
        }
        r.index = x;
        routine_0204(engine, r);
        if cbool(r.carry == 0) {
            engine.set_mem(u16v(0x86 + x), 0x02);
            return;
        }
        {
            let mut t: i32 = engine.mem(0x37);
            if (cbool(t == 0) || cbool(t & 0x80)) {
                return;
            }
            engine.set_mem(0x37, 0xFD);
            engine.set_mem(0x8F, 0x1A);
        }
    }
}

mod routine_0137 {
    use super::*;
    pub fn routine_0137(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        let mut a: i32 = 0;
        r.offset = 0x0C;
        engine.set_mem(0x47, engine.mem(u16v(ptr + r.offset)));
        {
            let __old = r.offset;
            r.offset += 1;
            __old
        };
        engine.set_mem(0x48, engine.mem(u16v(ptr + r.offset)));
        {
            let __old = r.offset;
            r.offset += 1;
            __old
        };
        a = engine.mem(u16v(ptr + r.offset));
        engine.set_mem(0x44, a);
        if cbool(a >= 0x08) {
            a = u8v(a - 0x08);
        } else {
            a = 0x00;
        }
        if cbool(a >= 0x31) {
            a = 0x30;
        }
        engine.set_mem(0x7C, a);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);
        {
            let __old = r.offset;
            r.offset += 1;
            __old
        };
        r.value = engine.mem(u16v(ptr + r.offset));
        engine.set_mem(0x45, r.value);
        routine_0067(engine, r);
        routine_0128(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0127(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod routine_0138 {
    use super::*;
    pub fn routine_0138(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0141(engine, r);
        engine.set_mem(0x48, 0x11);
        r.index = u8v(engine.mem(0x6E) - 1);
        engine.set_mem(0x47, r.index);
        engine.set_mem(0x7C, 0x12);
        engine.set_mem(0x45, 0x10);
        engine.set_mem(0x44, 0x1A);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);
        r.value = 0x00;
        routine_0067(engine, r);
        routine_0128(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0127(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod routine_0139 {
    use super::*;
    pub fn routine_0139(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xEB, 0x00);
        routine_0141(engine, r);
        engine.set_mem(0x2E, 0x3E);
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x47, 0x03);
        engine.set_mem(0x7C, 0x12);
        engine.set_mem(0x45, 0xB0);
        engine.set_mem(0x44, 0x1A);
        engine.set_mem(0x43, 0x00);
        engine.set_mem(0x7B, 0x00);
        r.value = 0x00;
        routine_0067(engine, r);
        routine_0128(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0127(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        r.carry = 1;
    }
}

mod routine_0140 {
    use super::*;
    pub fn routine_0140(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x55);
        if (cbool(engine.mem(u16v(0x51 + x)) == 0x0F)
            && cbool(engine.mem(0x47) == 0x01)
            && cbool(engine.mem(0x48) == 0x05)
            && cbool(engine.mem(0x7C) == 0x10)
            && cbool(engine.mem(0x7B) == 0x00)
            && cbool(engine.mem(0x45) == 0xA0))
        {
            engine.set_mem(0xEC, 0x01);
        }
    }
}

mod routine_0141 {
    use super::*;
    pub fn routine_0141(engine: &mut Engine, r: &mut RoutineContext) {
        let mut outer: i32 = 0;
        routine_0065(engine, r);
        engine.set_mem(0x85, 0x00);
        routine_0061(engine, r);
        routine_0062(engine, r);
        if cbool(engine.mem(0x7C) >= 0x21) {
            engine.set_mem(0x7C, 0x20);
        }
        routine_0080(engine, r);
        engine.set_mem(0x7C, u8v(engine.mem(0x7C) + 0x10));
        routine_0080(engine, r);
        engine.set_mem(0x08, 0x01);
        loop {
            let mut x: i32 = 0x0C;
            loop {
                let mut sum: i32 = u16v(engine.mem(0x1C) + engine.mem(0x08));
                engine.set_mem(0x1C, u8v(sum));
                if cbool(sum & 0x100) {
                    engine.set_mem(0x1D, u8v(engine.mem(0x1D) ^ 0x01));
                }
                r.value = 0xFF;
                queue_ppu_job_and_wait(engine, r);
                if !cbool(
                    {
                        x -= 1;
                        x
                    } != 0,
                ) {
                    break;
                }
            }
            engine.set_mem(0x08, u8v(engine.mem(0x08) + 1));
            outer = engine.mem(0x08);
            if !cbool(outer < 0x20) {
                break;
            }
        }
        engine.set_mem(0x8F, 0x18);
        engine.set_mem(0x90, 0xFF);
        r.index = 0x08;
        routine_0074(engine, r);
    }
}

mod routine_0142 {
    use super::*;
    fn scene_rebuild_full(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0067(engine, r);
        routine_0128(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0127(engine, r);
        routine_0060(engine, r);
        routine_0061(engine, r);
        routine_0070(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }

    fn scene_rebuild_vert(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0128(engine, r);
        routine_0127(engine, r);
        scene_assemble(engine, r);
        routine_0077(engine, r);
        routine_0075(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }

    pub fn routine_0142(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x0045);
        if cbool(a < 0x10) {
            routine_0164(engine, r);
            if cbool(r.carry == 0) {
                return;
            }
            if cbool(engine.mem(0x0048) == 0x00) {
                engine.set_mem(0x0048, 0x10);
                engine.set_mem(0x0047, 0x03);
                engine.set_mem(0x007C, 0x12);
                engine.set_mem(0x0045, 0xB0);
                engine.set_mem(0x0044, 0x1A);
                engine.set_mem(0x0043, 0x00);
                engine.set_mem(0x007B, 0x00);
                scene_rebuild_full(engine, r);
                return;
            }
            if cbool(engine.mem(0x0048) == 0x10) {
                return;
            }
            engine.set_mem(0x0048, u8v(engine.mem(0x0048) - 1));
            engine.set_mem(0x0045, 0xB0);
            scene_rebuild_vert(engine, r);
            return;
        }
        if cbool(a >= 0xA1) {
            if cbool(engine.mem(0x0048) == 0x10) {
                engine.set_mem(0x0048, 0x00);
                engine.set_mem(0x0047, 0x00);
                engine.set_mem(0x007C, 0x00);
                engine.set_mem(0x0045, 0x00);
                engine.set_mem(0x0043, 0x00);
                engine.set_mem(0x007B, 0x00);
                engine.set_mem(0x0044, 0x01);
                scene_rebuild_full(engine, r);
                return;
            }
            if cbool(u8v(engine.mem(0x0048) + 1) >= 0x10) {
                return;
            }
            engine.set_mem(0x0048, u8v(engine.mem(0x0048) + 1));
            engine.set_mem(0x0045, 0x00);
            scene_rebuild_vert(engine, r);
            return;
        }
        if cbool(engine.mem(0x0048) == 0x10) {
            return;
        }
        routine_0163(engine, r);
        engine.set_mem(0x85, 0x00);
        engine.set_mem(0x56, u8v(engine.mem(0x56) & 0x07));
        if cbool(engine.mem(0x0044) == 0x00) {
            if cbool(u8v((engine.mem(0x0047) - 1)) & 0x80) {
                return;
            }
            engine.set_mem(0x0047, u8v(engine.mem(0x0047) - 1));
            engine.set_mem(0x57, 0x00);
            routine_0061(engine, r);
            engine.set_mem(0x007C, 0x30);
            engine.set_mem(0x0044, 0x3F);
            engine.set_mem(0x0043, 0x00);
        } else {
            if cbool(engine.mem(0x0044) < 0x3E) {
                return;
            }
            if cbool(u8v(engine.mem(0x0047) + 1) >= 0x04) {
                return;
            }
            engine.set_mem(0x0047, u8v(engine.mem(0x0047) + 1));
            engine.set_mem(0x57, 0x40);
            routine_0061(engine, r);
            engine.set_mem(0x007C, 0x00);
            engine.set_mem(0x0043, 0x00);
            engine.set_mem(0x0044, 0x00);
        }
        routine_0128(engine, r);
        routine_0127(engine, r);
        engine.set_mem(0x007B, 0x00);
        scene_assemble(engine, r);
        routine_0080(engine, r);
        routine_0075(engine, r);
        if cbool(engine.mem(0x0044) != 0x00) {
            engine.set_mem(0x1D, 0x01);
            engine.set_mem(0x1C, 0x00);
            engine.set_mem(0x0213, 0x00);
            engine.set_mem(0x0217, 0x08);
            engine.set_mem(0x0A, 0x0F);
            loop {
                engine.set_mem(0x0B, 0x03);
                loop {
                    if cbool(engine.mem(0x0B) == 0) {
                        engine.set_mem(0x0213, u8v(engine.mem(0x0213) - 1));
                        engine.set_mem(0x0217, u8v(engine.mem(0x0217) - 1));
                        if cbool((engine.mem(0x4E) | engine.mem(0x4F)) == 0) {
                            engine.xor_mem(0x0211, 0x04);
                            engine.xor_mem(0x0215, 0x04);
                        }
                    }
                    engine.set_mem(0x0213, u8v(engine.mem(0x0213) + 0x04));
                    engine.set_mem(0x0217, u8v(engine.mem(0x0213) + 0x08));
                    engine.set_mem(0x1C, u8v(engine.mem(0x1C) - 0x04));
                    r.value = 0xFF;
                    queue_ppu_job_and_wait(engine, r);
                    engine.set_mem(0x0B, u8v(engine.mem(0x0B) - 1));
                    if !cbool((engine.mem(0x0B) & 0x80) == 0) {
                        break;
                    }
                }
                engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
                if !cbool((engine.mem(0x0A) & 0x80) == 0) {
                    break;
                }
            }
            engine.set_mem(0x0016, 0x1E);
            engine.set_mem(0x0017, 0x20);
            engine.set_mem(0x0C, 0x2F);
            farcall_bank_09_r7(engine, r);
            engine.set_mem(0x36, 0);
            r.carry = 1;
            return;
        }
        engine.set_mem(0x1C, 0xFC);
        engine.set_mem(0x1D, 0x01);
        engine.set_mem(0x0213, 0xF0);
        engine.set_mem(0x0217, 0xF8);
        engine.set_mem(0x0A, 0x0F);
        loop {
            engine.set_mem(0x0B, 0x03);
            loop {
                if cbool(engine.mem(0x0B) == 0) {
                    engine.set_mem(0x0213, u8v(engine.mem(0x0213) + 1));
                    engine.set_mem(0x0217, u8v(engine.mem(0x0217) + 1));
                    if cbool((engine.mem(0x4E) | engine.mem(0x4F)) == 0) {
                        engine.xor_mem(0x0211, 0x04);
                        engine.xor_mem(0x0215, 0x04);
                    }
                }
                engine.set_mem(0x0213, u8v(engine.mem(0x0213) - 0x04));
                engine.set_mem(0x0217, u8v(engine.mem(0x0213) + 0x08));
                engine.set_mem(0x1C, u8v(engine.mem(0x1C) + 0x04));
                r.value = 0xFF;
                queue_ppu_job_and_wait(engine, r);
                engine.set_mem(0x0B, u8v(engine.mem(0x0B) - 1));
                if !cbool((engine.mem(0x0B) & 0x80) == 0) {
                    break;
                }
            }
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) - 1));
            if !cbool((engine.mem(0x0A) & 0x80) == 0) {
                break;
            }
        }
        engine.set_mem(0x0016, 0x00);
        engine.set_mem(0x0017, 0x24);
        engine.set_mem(0x0C, 0x10);
        farcall_bank_09_r7(engine, r);
        engine.set_mem(0x36, 0);
        r.carry = 1;
    }
}

mod routine_0143 {
    use super::*;
    pub fn routine_0143(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dx: i32 = 0;
        let mut sum: i32 = 0;
        let mut carry: i32 = 0;
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0F, engine.mem(0x44));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0x4B) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x4B) + engine.mem(0x0A)));
        }
        dx = engine.mem(0x49);
        if cbool(dx != 0) {
            sum = u8v(dx + engine.mem(0x0E));
            engine.set_mem(0x0E, u8v(sum & 0x0F));
            carry = u8v((sum >> 4) & 1);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + engine.mem(0x4A) + carry));
        }
    }
}

mod routine_0144 {
    use super::*;
    pub fn routine_0144(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    x = 0x3D;
                    if cbool(engine.mem(0x46) != 0) {
                        return;
                    }
                    x = 0x09;
                    if cbool(engine.mem(0x50) != 0) {
                        return;
                    }
                    if cbool((engine.mem(0x20) & 0xBF) == 0x80) {
                        return;
                    }
                    a = engine.mem(0x4B);
                    if cbool(a == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a & 0x80) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x4E) != 0) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    if cbool((engine.mem(0x20) & 0x04) == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    x = 0x0D;
                    engine.set_mem(0x56, x);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0x4F) == 0) {
                        return;
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    x = 0x01;
                    y = 0x00;
                    if cbool(engine.mem(0x4A) & 0x80) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x49) == 0) {
                        return;
                    }
                    y = 0x40;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.set_mem(0x08, x);
                    engine.set_mem(0x56, (engine.mem(0x56) & 0x07) | engine.mem(0x08));
                    engine.set_mem(0x57, y);
                    return;
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    x = 0x39;
                    y = 0x00;
                    a = engine.mem(0x4A) | engine.mem(0x49);
                    if cbool(a & 0x80) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a != 0) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    x = 0x09;
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    y = 0x40;
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    engine.set_mem(0x08, x);
                    engine.set_mem(0x56, (engine.mem(0x56) & 0x03) | engine.mem(0x08));
                    engine.set_mem(0x57, y);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0145 {
    use super::*;
    pub fn routine_0145(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x46) == 0) {
            if cbool(engine.mem(0x56) < 0x20) {
                if cbool(engine.mem(0x20) & 0x40) {
                    engine.set_mem(0x56, u8v(engine.mem(0x56) | 0x10));
                } else {
                    engine.set_mem(0x56, u8v(engine.mem(0x56) & 0xEF));
                }
            }
        }
        if cbool((engine.mem(0x20) & 0x0F) == 0) {
            return;
        }
        if cbool((engine.mem(0x4F) | engine.mem(0x4E)) != 0) {
            return;
        }
        engine.set_mem(0x4D, u8v(engine.mem(0x4D) + 1));
        if cbool((engine.mem(0x4D) & 0x07) != 0) {
            return;
        }
        if cbool(engine.mem(0x56) & 0x08) {
            engine.set_mem(0x57, u8v(engine.mem(0x57) ^ 0x40));
        } else {
            engine.set_mem(0x56, u8v(engine.mem(0x56) ^ 0x04));
        }
    }
}

mod routine_0146 {
    use super::*;
    pub fn routine_0146(engine: &mut Engine, r: &mut RoutineContext) {
        let mut save4B: i32 = engine.mem(0x4B);
        let mut save49: i32 = engine.mem(0x49);
        let mut a: i32 = 0;
        let mut x: i32 = 0;
        let mut v: i32 = 0;
        let mut state: i32 = 1;
        'dispatch: loop {
            match state {
                1 => {
                    routine_0143(engine, r);
                    routine_0115(engine, r);
                    if cbool(r.carry) {
                        routine_0142(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 7;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    routine_0168(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    routine_0110(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 8;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem(0x08);
                    if cbool(a == 0x09) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a < 0x09) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    x = engine.mem(0x09);
                    r.index = x;
                    v = engine.mem(u16v(0x0401 + x));
                    r.value = v;
                    if cbool(v == 0x01) {
                        routine_0148(engine, r);
                        {
                            state = 8;
                            continue 'dispatch;
                        }
                    }
                    routine_0149(engine, r);
                    routine_0089(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    x = engine.mem(0x09);
                    r.index = x;
                    v = engine.mem(u16v(0x0401 + x));
                    r.value = v;
                    if cbool(v == 0x01) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(v >= 0x1A) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    routine_0150(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    routine_0147(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    r.carry = 0;
                    {
                        state = 8;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    if cbool(engine.mem(0x88) == 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    a = engine.mem(0x49);
                    if cbool(a == 0) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    x = a;
                    if !cbool(a & 0x08) {
                        x = u8v(x - 2);
                    }
                    x = u8v(x + 1);
                    a = u8v(x & 0x0F);
                    engine.set_mem(0x49, a);
                    if cbool(a != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    engine.set_mem(0x49, save49);
                    x = engine.mem(0x4B);
                    if cbool(x == 0) {
                        {
                            state = 7;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(x & 0x80) {
                        x = u8v(x - 2);
                    }
                    x = u8v(x + 1);
                    engine.set_mem(0x4B, x);
                    if cbool(x != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    state = 7;
                    continue 'dispatch;
                }
                7 => {
                    r.carry = 1;
                    state = 8;
                    continue 'dispatch;
                }
                8 => {
                    engine.set_mem(0x49, save49);
                    engine.set_mem(0x4B, save4B);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0147 {
    use super::*;
    pub fn routine_0147(engine: &mut Engine, r: &mut RoutineContext) {
        if (cbool(engine.mem(0x2D) < 0x30)
            && cbool(engine.mem(0x87) != 0)
            && cbool(engine.mem(0x59) != 0))
        {
            let hit_slot: i32 = engine.mem(0x09);
            engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
        }
    }
}

mod routine_0149 {
    use super::*;
    pub fn routine_0149(engine: &mut Engine, r: &mut RoutineContext) {
        let mut n: i32 = u8v(u8v(r.value - 0x02));
        engine.set_mem(0x04A1, 0x00);
        if cbool(n >= 0x18) {
            engine.set_mem(0x8F, 0x06);
            return;
        }
        if cbool(n < 0x08) {
            const tbl: [i32; 8] = [
                0xD16A, 0xD199, 0xDB47, 0xDB52, 0xDB66, 0xDB7B, 0xDBB7, 0xDB9B,
            ];
            engine.set_mem(0x0C, u8v(tbl[n as usize as usize] & 0xFF));
            engine.set_mem(0x0D, u8v(tbl[n as usize as usize] >> 8));
            r.value = u8v(n << 1);
            r.index = r.value;
            match n {
                0 => {
                    routine_0133(engine, r);
                }
                1 => {
                    routine_0134(engine, r);
                }
                2 => {
                    routine_0154(engine, r);
                }
                3 => {
                    routine_0155(engine, r);
                }
                4 => {
                    routine_0157(engine, r);
                }
                5 => {
                    routine_0159(engine, r);
                }
                6 => {
                    routine_0162(engine, r);
                }
                7 => {
                    routine_0161(engine, r);
                }
                _ => {}
            }
            return;
        }
        {
            let mut x: i32 = u8v(n - 0x08);
            if cbool(engine.mem(u16v(0x60 + x)) >= 0x0B) {
                engine.set_mem(0x8F, 0x1D);
                return;
            }
            engine.inc_mem(u16v(0x60 + x));
            engine.set_mem(0x8F, 0x13);
            if cbool(x == 0x0E) {
                routine_0089(engine, r);
                routine_0138(engine, r);
            }
        }
    }
}

mod routine_0150 {
    use super::*;
    pub fn routine_0150(engine: &mut Engine, r: &mut RoutineContext) {
        let mut n: i32 = u8v(u8v(r.value - 0x02));
        if cbool(n >= 0x18) {
            return;
        }
        {
            let mut slot: i32 = u8v(r.index);
            engine.set_mem(u16v(0x0401 + slot), 0x00);
            engine.set_mem(u16v(0x0406 + slot), 0xF0);
        }
        {
            let mut oam: i32 = u8v((engine.mem(0x08) << 3) | 0x80);
            engine.set_mem(u16v(0x0200 + oam), 0xEF);
            engine.set_mem(u16v(0x0204 + oam), 0xEF);
            r.index = oam;
        }
        if cbool(n < 0x08) {
            const tbl: [i32; 8] = [
                0xDB26, 0xDB31, 0xDB3C, 0xDB52, 0xDB5D, 0xDB71, 0xDBB7, 0xDB85,
            ];
            engine.set_mem(0x0C, u8v(tbl[n as usize as usize] & 0xFF));
            engine.set_mem(0x0D, u8v(tbl[n as usize as usize] >> 8));
            r.value = u8v(n << 1);
            r.index = r.value;
            match n {
                0 => {
                    routine_0151(engine, r);
                }
                1 => {
                    routine_0152(engine, r);
                }
                2 => {
                    routine_0153(engine, r);
                }
                3 => {
                    routine_0155(engine, r);
                }
                4 => {
                    routine_0156(engine, r);
                }
                5 => {
                    routine_0158(engine, r);
                }
                6 => {
                    routine_0162(engine, r);
                }
                7 => {
                    routine_0160(engine, r);
                }
                _ => {}
            }
            return;
        }
        {
            let mut x: i32 = u8v(n - 0x08);
            if cbool(engine.mem(u16v(0x60 + x)) >= 0x0B) {
                engine.set_mem(0x8F, 0x1D);
                return;
            }
            engine.inc_mem(u16v(0x60 + x));
            engine.set_mem(0x8F, 0x13);
            if cbool(x == 0x0E) {
                routine_0089(engine, r);
                routine_0138(engine, r);
            }
        }
    }
}

mod routine_0151 {
    use super::*;
    pub fn routine_0151(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x1E);
        r.value = 0x05;
        routine_0205(engine, r);
    }
}

mod routine_0152 {
    use super::*;
    pub fn routine_0152(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x05;
        routine_0206(engine, r);
    }
}

mod routine_0153 {
    use super::*;
    pub fn routine_0153(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x02;
        routine_0207(engine, r);
    }
}

mod routine_0154 {
    use super::*;
    pub fn routine_0154(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x11);
        r.value = 0x32;
        routine_0207(engine, r);
    }
}

mod routine_0155 {
    use super::*;
    pub fn routine_0155(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x1D);
        r.value = 0x05;
        routine_0203(engine, r);
    }
}

mod routine_0156 {
    use super::*;
    pub fn routine_0156(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x15);
        routine_0209(engine, r);
    }
}

mod routine_0157 {
    use super::*;
    pub fn routine_0157(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x15);
        r.value = 0x14;
        routine_0210(engine, r);
    }
}

mod routine_0158 {
    use super::*;
    pub fn routine_0158(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x13);
        engine.set_mem(0x85, 0x0A);
        r.value = 0x0A;
    }
}

mod routine_0159 {
    use super::*;
    pub fn routine_0159(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x8F, 0x13);
        engine.set_mem(0x85, 0x1E);
        r.value = 0x1E;
    }
}

mod routine_0160 {
    use super::*;
    pub fn routine_0160(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x1E;
        let mut a: i32 = 0;
        engine.set_mem(0x8F, 0x13);
        a = engine.mem(0x88);
        if cbool(a != 0) {
            a = engine.mem(0x89);
            if cbool(a != 0) {
                engine.set_mem(0x8A, x);
            }
            engine.set_mem(0x89, x);
        }
        engine.set_mem(0x88, x);
        r.value = a;
        r.index = x;
    }
}

mod routine_0161 {
    use super::*;
    pub fn routine_0161(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x3C;
        let mut a: i32 = 0;
        engine.set_mem(0x8F, 0x13);
        a = engine.mem(0x88);
        if cbool(a != 0) {
            a = engine.mem(0x89);
            if cbool(a != 0) {
                a = engine.mem(0x8A);
                if cbool(a != 0) {
                    engine.set_mem(0x8B, x);
                }
                engine.set_mem(0x8A, x);
            }
            engine.set_mem(0x89, x);
        }
        engine.set_mem(0x88, x);
        r.value = a;
        r.index = x;
    }
}

mod routine_0162 {
    use super::*;
    pub fn routine_0162(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x09;
        let mut y: i32 = 0x00;
        loop {
            if cbool(engine.mem(u16v(0x0401 + y)) == 0x01) {
                engine.set_mem(u16v(0x0401 + y), 0x80);
            }
            y = u8v(y + 0x10);
            if !cbool(
                {
                    x -= 1;
                    x
                } != 0,
            ) {
                break;
            }
        }
        engine.set_mem(0x8F, 0x18);
        engine.set_mem(0x90, 0xFF);
        r.index = 0x02;
        routine_0074(engine, r);
    }
}

mod routine_0164 {
    use super::*;
    pub fn routine_0164(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0x86) | engine.mem(0x4F)) != 0) {
            return;
        }
        if cbool(engine.mem(0x0E) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, 0x00);
        routine_0090(engine, r);
        {
            let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            let mut v: i32 = engine.mem(ptr) & 0x3F;
            r.carry = u8v((if cbool(v == 0) { 1 } else { 0 }));
        }
    }
}

mod routine_0165 {
    use super::*;
    pub fn routine_0165(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut v: i32 = u8v(engine.mem(u16v(ptr + r.offset)) & 0x3F);
        if cbool(v != 0x30) {
            r.carry = 0;
            return;
        }
        if cbool(engine.mem(0x4F) == 0) {
            engine.set_mem(0x4F, 0x0A);
        }
        if cbool(engine.mem(0x85) == 0) {
            routine_0202(engine, r);
            engine.set_mem(0x8F, 0x0A);
            engine.set_mem(0x85, 0x01);
        }
        r.carry = 1;
    }
}

mod routine_0166 {
    use super::*;
    pub fn routine_0166(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut x: i32 = u8v(engine.mem(u16v(ptr + r.offset)) & 0x3F);
        if cbool(x == 0) {
            if cbool(engine.mem(0x43) == 0) {
                r.carry = 1;
            } else {
                r.carry = 0;
            }
        } else if cbool(x == 0x02) {
            r.carry = 1;
        } else {
            r.carry = u8v(u8v(x >= 0x30));
        }
    }
}

mod routine_0167 {
    use super::*;
    pub fn routine_0167(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut ptr: i32 = 0;
        let mut x: i32 = engine.mem(0x45);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(x == 0) {
                        return;
                    }
                    x = u8v(x - 1);
                    engine.set_mem(0x0D, x);
                    x = engine.mem(0x44);
                    engine.set_mem(0x0C, x);
                    routine_0090(engine, r);
                    ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                    r.offset = 0x00;
                    a = engine.mem(u16v(ptr + r.offset)) & 0x3F;
                    if cbool(a == 0x05) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x04) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x03) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x43) == 0) {
                        return;
                    }
                    r.offset = 0x0C;
                    a = engine.mem(u16v(ptr + r.offset)) & 0x3F;
                    if cbool(a == 0x05) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x04) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x03) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0175(engine, r);
                    engine.lotw_nonlocal_handoff = 1;
                    return;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    routine_0187(engine, r);
                    engine.lotw_nonlocal_handoff = 1;
                    return;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    {
                        let mut ei: i32 = engine.mem(0x55);
                        if cbool(engine.mem(u16v(0x51 + ei)) != 0x0E) {
                            return;
                        }
                        {
                            let mut cnt: i32 = engine.mem(0x6E);
                            let mut idx: i32 = 0;
                            {
                                idx = 2;
                                while cbool(idx >= 0) {
                                    if cbool(engine.mem(u16v(0x51 + idx)) == 0x0E) {
                                        cnt = u8v(cnt + 1);
                                    }
                                    {
                                        let __old = idx;
                                        idx -= 1;
                                        __old
                                    };
                                }
                            }
                            if cbool(cnt != 0x04) {
                                return;
                            }
                        }
                        routine_0137(engine, r);
                        engine.lotw_nonlocal_handoff = 1;
                        return;
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0168 {
    use super::*;
    pub fn routine_0168(engine: &mut Engine, r: &mut RoutineContext) {
        let mut s_0E: i32 = 0;
        let mut s_0F: i32 = 0;
        let mut s_0A: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xE5, 0x90);
                    engine.set_mem(0xE6, 0x04);
                    s_0E = engine.mem(0x0E);
                    s_0F = engine.mem(0x0F);
                    s_0A = engine.mem(0x0A);
                    engine.set_mem(0x0C, engine.mem(0x0F));
                    engine.set_mem(0x0D, engine.mem(0x0A));
                    routine_0090(engine, r);
                    r.offset = 0x00;
                    routine_0169(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x0E) != 0) {
                        r.offset = 0x0C;
                        routine_0169(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    {
                        let mut a: i32 = engine.mem(0x0A);
                        if cbool(a >= 0xB0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool((a & 0x0F) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.offset = 0x01;
                        routine_0169(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(engine.mem(0x0E) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        r.offset = 0x0D;
                        routine_0169(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.carry = 0;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x0A, s_0A);
                    engine.set_mem(0x0F, s_0F);
                    engine.set_mem(0x0E, s_0E);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0170 {
    use super::*;
    pub fn routine_0170(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0x0B);
        let mut y: i32 = 0;
        if cbool(a >= 0x0C) {
            a = u8v(a - 0x0C);
            engine.inc_mem(0x0F);
        }
        y = a;
        if cbool(y != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0x0A) + 0x10));
        }
        engine.set_mem(0xFB, engine.mem(0x0A) & 0xF0);
        engine.set_mem(0xFC, 0x00);
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xF9, 0x00);
        r.value = 0x00;
        r.offset = y;
    }
}

mod routine_0171 {
    use super::*;
    pub fn routine_0171(engine: &mut Engine, r: &mut RoutineContext) {
        let mut fa: i32 = engine.mem(0xFA);
        engine.set_mem(0x0C, fa);
        engine.set_mem(0x16, u8v((fa << 1) & 0x1F));
        engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
        engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
        engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
        farcall_bank_09_r7(engine, r);
    }
}

mod routine_0172 {
    use super::*;
    pub fn routine_0172(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = engine.mem(0x0B);
        let mut ptr: i32 = u16v(engine.mem(0x10) | (engine.mem(0x11) << 8));
        let mut b: i32 = engine.mem(u16v(ptr + y));
        let mut x: i32 = b & 0x3F;
        r.index = x;
        r.offset = y;
        if cbool(x == 0x3E) {
            r.value = engine.mem(0x74);
        } else {
            r.value = b;
        }
    }
}

mod routine_0173 {
    use super::*;
    pub fn routine_0173(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v49: i32 = engine.mem(0x49);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0x49, 0x00);
                    engine.set_mem(0x4A, 0x00);
                    if cbool(v49 == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut a: i32 = u8v(engine.mem(0x45) & 0x0F);
                        if cbool(a == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a < 0x06) {
                            if cbool(engine.mem(0x20) & 0x04) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x4B, 0xFF);
                            engine.set_mem(0x4C, 0xFF);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a >= 0x0B) {
                            if cbool(engine.mem(0x20) & 0x08) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x4B, 0x01);
                            engine.set_mem(0x4C, 0x00);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    {
                        let mut v4B: i32 = engine.mem(0x4B);
                        engine.set_mem(0x4B, 0x00);
                        engine.set_mem(0x4C, 0x00);
                        if cbool(v4B == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        let mut a: i32 = engine.mem(0x43);
                        if cbool(a == 0) {
                            {
                                state = 3;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a < 0x06) {
                            if cbool(engine.mem(0x20) & 0x01) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x49, 0x0F);
                            engine.set_mem(0x4A, 0xFF);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(a >= 0x0B) {
                            if cbool(engine.mem(0x20) & 0x02) {
                                {
                                    state = 3;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0x49, 0x01);
                            engine.set_mem(0x4A, 0x00);
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    routine_0146(engine, r);
                    return;
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0178 {
    use super::*;
    pub fn routine_0178(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, 0x77);
        engine.set_mem(0x0F, 0xB5);
        routine_0052(engine, r);
        if cbool(r.carry) {
            return;
        }
        engine.set_mem(0x8F, 0x10);
        routine_0130(engine, r);
        routine_0095(engine, r);
        routine_0096(engine, r);
        engine.set_mem(0x7C, 0x20);
        routine_0081(engine, r);
        routine_0060(engine, r);
        routine_0201(engine, r);
    }
}

mod routine_0179 {
    use super::*;
    pub fn routine_0179(engine: &mut Engine, r: &mut RoutineContext) {
        let mut f5: i32 = engine.mem(0xF5);
        let mut a: i32 = u8v(u8v(f5 << 2) + f5);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    a = u8v(a + engine.mem(0xF7));
                    if cbool(a == 0x20) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x21) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(a == 0x22) {
                        routine_0178(engine, r);
                        return;
                    }
                    r.value = a;
                    routine_0186(engine, r);
                    engine.set_mem(u16v(0x0322 + r.index), a);
                    if cbool(r.index == 0x1F) {
                        routine_0178(engine, r);
                        return;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.inc_mem(0xF9);
                    routine_0184(engine, r);
                    return;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.dec_mem(0xF9);
                    routine_0184(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0180 {
    use super::*;
    pub fn routine_0180(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF5) + 1);
        if cbool(x >= 0x07) {
            x = 0x00;
        }
        engine.set_mem(0xF5, x);
        routine_0185(engine, r);
    }
}

mod routine_0181 {
    use super::*;
    pub fn routine_0181(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF5) - 1);
        if cbool(x & 0x80) {
            x = 0x06;
        }
        engine.set_mem(0xF5, x);
        routine_0185(engine, r);
    }
}

mod routine_0182 {
    use super::*;
    pub fn routine_0182(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF7) - 1);
        if cbool(x & 0x80) {
            x = 0x04;
        }
        engine.set_mem(0xF7, x);
        routine_0185(engine, r);
    }
}

mod routine_0183 {
    use super::*;
    pub fn routine_0183(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(engine.mem(0xF7) + 1);
        if cbool(x >= 0x05) {
            x = 0x00;
        }
        engine.set_mem(0xF7, x);
        routine_0185(engine, r);
    }
}

mod routine_0184 {
    use super::*;
    pub fn routine_0184(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = engine.mem(0xF9) & 0x1F;
        let mut x: i32 = 0x61;
        let mut base: i32 = 0;
        let mut v: i32 = 0;
        let mut carry: i32 = 0;
        let mut res: i32 = 0;
        if cbool(a >= 0x10) {
            a = u8v(a - 0x10);
            x = 0x69;
        }
        engine.set_mem(0x0280, x);
        engine.set_mem(0x0284, x);
        engine.set_mem(0x08, a);
        base = u8v((a >> 2) + a);
        v = u8v(base << 3);
        carry = u8v((base >> 5) & 1);
        res = u8v(u8v(v) + 0x36 + carry);
        engine.set_mem(0x0287, res);
        res = u8v(res - 0x08);
        engine.set_mem(0x0283, res);
        r.index = x;
        r.value = res;
    }
}

mod routine_0185 {
    use super::*;
    fn asl3(engine: &mut Engine, mut v: i32, carry_out: &mut i32) -> i32 {
        let mut c: i32 = 0;
        let mut i: i32 = 0;
        {
            i = 0;
            while cbool(i < 3) {
                c = (v >> 7) & 1;
                v = u8v(v << 1);
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        *carry_out = c;
        return v;
    }

    pub fn routine_0185(engine: &mut Engine, r: &mut RoutineContext) {
        let mut c: i32 = 0;
        let mut a: i32 = 0;
        let mut t: i32 = 0;
        t = asl3(engine, engine.mem(0xF5), &mut c);
        a = u8v(t + 0x36 + c);
        engine.set_mem(0x0297, a);
        a = u8v(a - 0x08);
        engine.set_mem(0x0293, a);
        t = asl3(engine, engine.mem(0xF7), &mut c);
        a = u8v(t + 0x81 + c);
        engine.set_mem(0x0290, a);
        engine.set_mem(0x0294, a);
        r.value = a;
    }
}

mod routine_0186 {
    use super::*;
    pub fn routine_0186(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = engine.mem(0xF9) & 0x1F;
    }
}

mod routine_0192 {
    use super::*;
    pub fn routine_0192(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0194(engine, r);
        routine_0067(engine, r);
        routine_0200(engine, r);
        r.value = engine.mem(0xFE);
        routine_0123(engine, r);
        routine_0084(engine, r);
        routine_0077(engine, r);
        routine_0061(engine, r);
        routine_0063(engine, r);
        routine_0060(engine, r);
        routine_0070(engine, r);
        routine_0144(engine, r);
        routine_0145(engine, r);
    }
}

mod routine_0195 {
    use super::*;
    pub fn routine_0195(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        routine_0067(engine, r);
        engine.set_mem(0x08, a);
        engine.set_mem(0x47, u8v((a & 0x0C) >> 2));
        engine.set_mem(0x7C, u8v((a & 0x03) << 4));
        engine.set_mem(0x44, u8v(engine.mem(0x7C) + 0x07));
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x43, 0x08);
        engine.set_mem(0x45, 0xA0);
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        engine.set_mem(0x7B, 0x00);
        routine_0127(engine, r);
        routine_0084(engine, r);
        if cbool(a == 0x04) {
            engine.set_mem(0x7A, u8v(0x1F + 0xA0));
        }
        routine_0078(engine, r);
        routine_0144(engine, r);
        routine_0061(engine, r);
        routine_0060(engine, r);
    }
}

mod routine_0196 {
    use super::*;
    pub fn routine_0196(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        routine_0068(engine, r);
        engine.set_mem(0x08, a);
        engine.set_mem(0x47, u8v((a & 0x0C) >> 2));
        engine.set_mem(0x7C, u8v((a & 0x03) << 4));
        engine.set_mem(0x44, u8v(engine.mem(0x7C) + 0x07));
        engine.set_mem(0x48, 0x10);
        engine.set_mem(0x43, 0x08);
        engine.set_mem(0x45, 0xA0);
        engine.set_mem(0x4F, 0x00);
        engine.set_mem(0x4E, 0x00);
        engine.set_mem(0x7B, 0x00);
        routine_0127(engine, r);
        routine_0084(engine, r);
        if cbool(a == 0x04) {
            engine.set_mem(0x7A, u8v(0x1F + 0xA0));
        }
        routine_0078(engine, r);
        routine_0144(engine, r);
        routine_0061(engine, r);
        routine_0060(engine, r);
    }
}

mod routine_0197 {
    use super::*;
    pub fn routine_0197(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0x10;
        let mut a: i32 = 0;
        engine.set_mem(0x08, 0x58);
        {
            x = 2;
            while cbool(x >= 0) {
                let mut item: i32 = engine.mem(u16v(0x0051 + x));
                if cbool(item & 0x80) {
                    a = 0xEF;
                } else {
                    let mut t: i32 = u8v((u8v(item << 2)) + 0xA1);
                    engine.set_mem(u16v(0x0241 + y), t);
                    engine.set_mem(u16v(0x0245 + y), u8v(t + 0x02));
                    a = 0xBB;
                }
                engine.set_mem(u16v(0x0240 + y), a);
                engine.set_mem(u16v(0x0244 + y), a);
                engine.set_mem(u16v(0x0243 + y), engine.mem(0x08));
                engine.set_mem(u16v(0x0247 + y), u8v(engine.mem(0x08) + 0x08));
                engine.set_mem(0x08, u8v(u8v(engine.mem(0x08) + 0x08) - 0x28));
                engine.set_mem(u16v(0x0242 + y), 0x01);
                engine.set_mem(u16v(0x0246 + y), 0x01);
                y = u8v(y - 0x08);
                {
                    x -= 1;
                    x
                };
            }
        }
        r.index = 0xFF;
        r.offset = y;
    }
}

mod routine_0198 {
    use super::*;
    pub fn routine_0198(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    a = 0xEF;
                    x = engine.mem(0x80);
                    if cbool(x & 0x80) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x0060 + x)) >= 0x0B) {
                        engine.set_mem(0x80, 0xEF);
                        a = 0xEF;
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    a = u8v(x << 2);
                    a = u8v(a + 0xA1);
                    engine.set_mem(0x0241, a);
                    a = u8v(a + 0x02);
                    engine.set_mem(0x0245, a);
                    engine.set_mem(0x0243, 0x40);
                    engine.set_mem(0x0247, 0x48);
                    a = 0xA4;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0x0240, a);
                    engine.set_mem(0x0244, a);
                    engine.set_mem(0x0242, 0x01);
                    engine.set_mem(0x0246, 0x01);
                    a = 0xEF;
                    x = engine.mem(0x82);
                    if cbool(x & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(u16v(0x0060 + x)) >= 0x0B) {
                        engine.set_mem(0x82, 0xEF);
                        a = 0xEF;
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    a = u8v(x << 2);
                    a = u8v(a + 0xA1);
                    engine.set_mem(0x0249, a);
                    a = u8v(a + 0x02);
                    engine.set_mem(0x024D, a);
                    engine.set_mem(0x024B, 0xB0);
                    engine.set_mem(0x024F, 0xB8);
                    a = 0xA0;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0x0248, a);
                    engine.set_mem(0x024C, a);
                    engine.set_mem(0x024A, 0x01);
                    engine.set_mem(0x024E, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0199 {
    use super::*;
    pub fn routine_0199(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0250, 0x98);
        engine.set_mem(0x0254, 0x98);
        engine.set_mem(0x0251, 0xF1);
        engine.set_mem(0x0255, 0xF3);
        engine.set_mem(0x0252, 0x02);
        engine.set_mem(0x0256, 0x02);
        engine.set_mem(0x0253, 0x78);
        engine.set_mem(0x0257, 0x80);
        r.value = 0x80;
    }
}

mod routine_0200 {
    use super::*;
    pub fn routine_0200(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0240, 0xEF);
        engine.set_mem(0x0244, 0xEF);
        engine.set_mem(0x0248, 0xEF);
        engine.set_mem(0x024C, 0xEF);
        engine.set_mem(0x0250, 0xEF);
        engine.set_mem(0x0254, 0xEF);
        r.value = 0xEF;
    }
}

mod routine_0201 {
    use super::*;
    pub fn routine_0201(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        {
            x = 0x37;
            while cbool(x >= 0) {
                engine.set_mem(u16v(0x0280 + x), engine.mem(u16v(0xFF6F + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.set_mem(0x2C, 0x34);
        engine.set_mem(0x2D, 0x35);
        engine.set_mem(0x2E, 0x36);
        engine.set_mem(0x2F, 0x37);
        r.index = 0xFF;
        r.value = 0x37;
    }
}

mod routine_0202 {
    use super::*;
    pub fn routine_0202(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = engine.mem(0x58);
        if cbool(r.value == 0) {
            r.carry = 1;
            return;
        }
        engine.set_mem(0x58, u8v(engine.mem(0x58) - 1));
        routine_0093(engine, r);
        r.carry = 0;
    }
}

mod routine_0203 {
    use super::*;
    pub fn routine_0203(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dmg: i32 = u8v(r.value);
        let mut res: i32 = 0;
        let mut carry: i32 = 0;
        engine.set_mem(0x08, dmg);
        res = u16v(engine.mem(0x58)) - dmg;
        engine.set_mem(0x58, u8v(res));
        carry = u8v(res < 0x100);
        if cbool(carry == 0) {
            engine.set_mem(0x58, 0x00);
        }
        routine_0093(engine, r);
        r.carry = carry;
    }
}

mod routine_0204 {
    use super::*;
    pub fn routine_0204(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_x: i32 = u8v(r.index);
        r.value = engine.mem(0x59);
        r.carry = 1;
        if cbool(engine.mem(0x59) != 0) {
            engine.set_mem(0x59, u8v(engine.mem(0x59) - 1));
            routine_0094(engine, r);
            r.carry = 0;
        }
        r.index = saved_x;
    }
}

mod routine_0205 {
    use super::*;
    pub fn routine_0205(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sum: i32 = u8v(u16v(r.value) + engine.mem(0x58));
        let mut v: i32 = 0;
        if cbool(sum > 0xFF) {
            v = 0x6D;
        } else if cbool(u8v(sum) >= 0x6E) {
            v = 0x6D;
        } else {
            v = u8v(sum);
        }
        engine.set_mem(0x58, v);
        routine_0093(engine, r);
    }
}

mod routine_0206 {
    use super::*;
    pub fn routine_0206(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sum: i32 = u8v(u16v(r.value) + engine.mem(0x59));
        let mut v: i32 = 0;
        if cbool(sum > 0xFF) {
            v = 0x6D;
        } else if cbool(u8v(sum) >= 0x6E) {
            v = 0x6D;
        } else {
            v = u8v(sum);
        }
        engine.set_mem(0x59, v);
        routine_0094(engine, r);
    }
}

mod routine_0207 {
    use super::*;
    pub fn routine_0207(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sum: i32 = u8v(u16v(r.value) + engine.mem(0x5A));
        let mut v: i32 = 0;
        if cbool(sum > 0xFF) {
            v = 0x6D;
        } else if cbool(u8v(sum) >= 0x6E) {
            v = 0x6D;
        } else {
            v = u8v(sum);
        }
        engine.set_mem(0x5A, v);
        routine_0096(engine, r);
    }
}

mod routine_0208 {
    use super::*;
    pub fn routine_0208(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x08, r.value);
        let mut res: i32 = u16v(engine.mem(0x5A)) - u16v(engine.mem(0x08));
        r.value = u8v(res);
        if cbool(res & 0x100) {
            r.carry = 0;
            return;
        }
        engine.set_mem(0x5A, r.value);
        routine_0096(engine, r);
        r.carry = 1;
    }
}

mod routine_0209 {
    use super::*;
    pub fn routine_0209(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x5B, u8v(engine.mem(0x5B) + 1));
        routine_0095(engine, r);
        r.carry = 0;
    }
}

mod routine_0210 {
    use super::*;
    pub fn routine_0210(engine: &mut Engine, r: &mut RoutineContext) {
        let mut sum: i32 = u8v(u16v(r.value) + engine.mem(0x5B));
        let mut v: i32 = 0;
        if cbool(sum > 0xFF) {
            v = 0x6D;
        } else if cbool(u8v(sum) >= 0x6E) {
            v = 0x6D;
        } else {
            v = u8v(sum);
        }
        engine.set_mem(0x5B, v);
        routine_0095(engine, r);
    }
}

mod routine_0211 {
    use super::*;
    pub fn routine_0211(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = engine.mem(0x5B);
        if cbool(r.value == 0) {
            r.carry = 1;
            return;
        }
        engine.set_mem(0x5B, u8v(engine.mem(0x5B) - 1));
        routine_0095(engine, r);
        r.carry = 0;
    }
}

mod routine_0212 {
    use super::*;
    pub fn routine_0212(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x48) == 0x10) {
                        return;
                    }
                    if cbool(engine.mem(0x2D) >= 0x30) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut e9: i32 = engine.mem(0xE9);
                        let mut v: i32 = u8v((e9 << 1) + e9);
                        engine.set_mem(0xE3, v);
                        engine.set_mem(0xE4, u8v(v + 3));
                        let mut e5: i32 = u8v(engine.mem(0xE3) << 4);
                        engine.set_mem(0xE5, e5);
                        engine.set_mem(0xE7, u8v(e5 + 0x20));
                        engine.set_mem(0xE6, 0x04);
                        engine.set_mem(0xE8, engine.mem(0x78));
                    }
                    loop {
                        let mut ee: i32 = 0;
                        load_object_slot_scratch(engine, r);
                        ee = engine.mem(0xEE);
                        if cbool(ee == 0) {
                            routine_0215(engine, r);
                        } else if cbool(ee & 0x80) {
                            routine_0240(engine, r);
                        } else if cbool(ee == 0x01) {
                            routine_0218(engine, r);
                        } else if cbool(ee >= 0x18) {
                            routine_0216(engine, r);
                        } else {
                            routine_0219(engine, r);
                        }
                        store_object_slot_scratch(engine, r);
                        engine.inc_mem(0xE3);
                        engine.set_mem(0xE5, u8v(engine.mem(0xE5) + 0x10));
                        engine.set_mem(0xE7, u8v(engine.mem(0xE7) + 0x10));
                        if !cbool(engine.mem(0xE3) < engine.mem(0xE4)) {
                            break;
                        }
                    }
                    {
                        let mut e9: i32 = u8v(engine.mem(0xE9) + 1);
                        engine.set_mem(0xE9, (if cbool(e9 >= 0x03) { 0x00 } else { e9 }));
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xE9) & 0x01) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xE5, 0x00);
                    engine.set_mem(0xE6, 0x04);
                    engine.set_mem(0xE3, 0x00);
                    engine.set_mem(0xE7, 0x20);
                    engine.set_mem(0xE8, engine.mem(0x78));
                    load_object_slot_scratch(engine, r);
                    {
                        let mut ee: i32 = engine.mem(0xEE);
                        if cbool(ee == 0) {
                            routine_0257(engine, r);
                        } else if cbool(ee & 0x80) {
                            routine_0263(engine, r);
                            routine_0264(engine, r);
                        } else {
                            routine_0258(engine, r);
                        }
                    }
                    store_object_slot_scratch(engine, r);
                    routine_0265(engine, r);
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xE3, 0x04);
                    engine.set_mem(0xE5, 0x40);
                    engine.set_mem(0xE6, 0x04);
                    engine.set_mem(0xE7, 0x60);
                    engine.set_mem(0xE8, engine.mem(0x78));
                    loop {
                        let mut ee: i32 = 0;
                        load_object_slot_scratch(engine, r);
                        ee = engine.mem(0xEE);
                        if (cbool(ee == 0) || cbool(ee & 0x80)) {
                            engine.set_mem(0xEE, 0x00);
                            routine_0217(engine, r);
                        } else {
                            routine_0218(engine, r);
                        }
                        store_object_slot_scratch(engine, r);
                        engine.inc_mem(0xE3);
                        engine.set_mem(0xE5, u8v(engine.mem(0xE5) + 0x10));
                        engine.set_mem(0xE7, u8v(engine.mem(0xE7) + 0x10));
                        if !cbool(engine.mem(0xE3) < 0x09) {
                            break;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    engine.xor_mem(0xE9, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod load_object_slot_scratch {
    use super::*;

    /// Copies the object slot addressed by `0xE5..0xE6` into scratch RAM
    /// `0xED..0xFC`.
    pub fn load_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
        let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
        let mut slot_offset: i32 = 0;
        {
            slot_offset = 0x0F;
            while cbool(slot_offset >= 0) {
                engine.set_mem(
                    u16v(0x00ED + slot_offset),
                    engine.mem(u16v(slot_ptr + slot_offset)),
                );
                {
                    slot_offset -= 1;
                    slot_offset
                };
            }
        }
        r.offset = 0xFF;
    }
}

mod store_object_slot_scratch {
    use super::*;

    /// Writes scratch RAM `0xED..0xFC` back to the object slot addressed by
    /// `0xE5..0xE6`.
    pub fn store_object_slot_scratch(engine: &mut Engine, r: &mut RoutineContext) {
        let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
        let mut slot_offset: i32 = 0;
        {
            slot_offset = 0x0F;
            while cbool(slot_offset >= 0) {
                engine.set_mem(
                    u16v(slot_ptr + slot_offset),
                    engine.mem(u16v(0x00ED + slot_offset)),
                );
                {
                    slot_offset -= 1;
                    slot_offset
                };
            }
        }
        r.offset = 0xFF;
    }
}

mod routine_0215 {
    use super::*;
    pub fn routine_0215(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut t: i32 = 0;
        let mut e7: i32 = 0;
        engine.set_mem(0xF3, u8v(engine.mem(0xF3) - 1));
        x = engine.mem(0xF3);
        if cbool(x >= 0x3C) {}
        e7 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        if cbool((engine.mem(u16v(e7 + 2)) | engine.mem(u16v(e7 + 3))) == 0) {
            r.value = 0x0C;
            rng_update(engine, r);
            engine.set_mem(0x0A, u8v(r.value << 4));
            r.value = 0x40;
            rng_update(engine, r);
            engine.set_mem(0x0F, r.value);
        } else {
            engine.set_mem(0x0A, engine.mem(u16v(e7 + 3)));
            engine.set_mem(0x0F, engine.mem(u16v(e7 + 2)));
        }
        engine.set_mem(0x0E, 0x00);
        engine.set_mem(0x0B, 0x00);
        routine_0111(engine, r);
        if cbool(r.carry) {}
        routine_0253(engine, r);
        if cbool(r.carry) {}
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        engine.set_mem(0xFC, 0x00);
        engine.set_mem(0xF2, engine.mem(u16v(e7 + 4)));
        engine.set_mem(0xF8, engine.mem(u16v(e7 + 5)));
        {
            let mut a: i32 = 0x00;
            let mut c: i32 = 1;
            let mut xi: i32 = engine.mem(0x40);
            loop {
                let mut nc: i32 = u8v((a >> 7) & 1);
                a = u8v((a << 1) | c);
                c = nc;
                xi = u8v(xi - 1);
                if !cbool((xi & 0x80) == 0) {
                    break;
                }
            }
            a = u8v(a & engine.mem(0x41));
            if cbool(a == 0) {
                let mut f8: i32 = engine.mem(0xF8);
                let mut carry: i32 = u8v((f8 >> 7) & 1);
                engine.set_mem(0xF8, u8v(f8 << 1));
                if cbool(carry) {
                    engine.set_mem(0xF8, 0xFF);
                }
            }
        }
        engine.set_mem(0xEE, 0x7F);
        engine.set_mem(0xED, 0xF9);
        engine.set_mem(0xEF, 0x01);
        t = engine.mem(0xF3);
        if cbool(t == 0) {
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0xED, engine.mem(u16v(e7 + 0)));
            engine.set_mem(0xEF, engine.mem(u16v(e7 + 1)));
        } else {
            if cbool((engine.mem(0xF3) & 0x03) == 0) {
                engine.set_mem(0xEF, u8v(engine.mem(0xEF) ^ 0x40));
            }
        }
    }
}

mod routine_0216 {
    use super::*;
    pub fn routine_0216(engine: &mut Engine, r: &mut RoutineContext) {
        let mut t: i32 = u8v(engine.mem(0xF3) - 1);
        engine.set_mem(0xF3, t);
        if cbool(t == 0) {
            let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0xED, engine.mem(ptr));
            engine.set_mem(0xEF, engine.mem(u16v(ptr + 1)));
        } else if cbool((t & 0x03) == 0) {
            engine.xor_mem(0xEF, 0x40);
        }
    }
}

mod routine_0217 {
    use super::*;
    pub fn routine_0217(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut e7: i32 = 0;
        r.value = 0x1E;
        rng_update(engine, r);
        if cbool(r.value != 0) {
            r.index = r.value;
            return;
        }
        r.index = 0;
        x = 0x03;
        y = 0x03;
        if cbool(engine.mem(0x0402) & 0x40) {
            y = 0x13;
        }
        loop {
            engine.set_mem(u16v(0x00F9 + x), engine.mem(u16v(0x040C + y)));
            y = u8v(y - 1);
            if cbool(
                ({
                    let __old = x;
                    x -= 1;
                    __old
                }) == 0,
            ) {
                break;
            }
        }
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        e7 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        engine.set_mem(0xF2, engine.mem(u16v(e7 + 4)));
        engine.set_mem(0xF8, engine.mem(u16v(e7 + 5)));
        engine.set_mem(0xEE, 0x01);
        engine.set_mem(0xED, 0x81);
        r.value = 0x04;
        rng_update(engine, r);
        engine.set_mem(0xEF, r.value);
        engine.set_mem(0xF1, 0x80);
        r.offset = y;
        r.index = x;
    }
}

mod routine_0218 {
    use super::*;
    const boss_state_table: [i32; 9] = [
        0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
    ];

    pub fn routine_0218(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        let mut idx: i32 = engine.mem(u16v(ptr + 8));
        if cbool(idx >= 0x09) {
            idx = 0x00;
        }
        engine.set_mem(0x0E, u8v(boss_state_table[idx as usize as usize] & 0xFF));
        engine.set_mem(0x0F, u8v(boss_state_table[idx as usize as usize] >> 8));
        match idx {
            0 => {
                routine_0220(engine, r);
            }
            1 => {
                routine_0221(engine, r);
            }
            2 => {
                routine_0222(engine, r);
            }
            3 => {
                routine_0223(engine, r);
            }
            4 => {
                routine_0224(engine, r);
            }
            5 => {
                routine_0225(engine, r);
            }
            6 => {
                routine_0226(engine, r);
            }
            7 => {
                routine_0228(engine, r);
            }
            8 => {
                routine_0229(engine, r);
            }
            _ => {}
        }
    }
}

mod routine_0219 {
    use super::*;
    pub fn routine_0219(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF0) == 0) {
                        if cbool(engine.mem(0xF1) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        routine_0237(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        routine_0238(engine, r);
                    }
                    routine_0236(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    routine_0238(engine, r);
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    x = u8v(engine.mem(0xF3) - 1);
                    if cbool(x == 0) {
                        engine.set_mem(0xEE, 0x00);
                        engine.set_mem(0xF3, 0xF0);
                        r.index = x;
                        return;
                    }
                    engine.set_mem(0xF3, x);
                    if cbool(x < 0x3C) {
                        x = 0xEF;
                        a = engine.mem(0xFB);
                        if cbool(a == 0xEF) {
                            x = engine.mem(0xFC);
                        }
                        engine.set_mem(0xFB, x);
                        engine.set_mem(0xFC, a);
                    }
                    routine_0250(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0220 {
    use super::*;
    pub fn routine_0220(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_f6: i32 = 0;
        let mut do_place: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF3) >= 0x20) {
                    } else if cbool(engine.mem(0xF1) != 0) {
                        do_place = 1;
                    } else if cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0) {
                        do_place = 1;
                    }
                    if !cbool(do_place) {
                        engine.set_mem(0xF3, 0x00);
                        routine_0235(engine, r);
                        r.value = 0x06;
                        rng_update(engine, r);
                        engine.set_mem(0xF6, u8v(r.value + 1));
                        r.value = 0x04;
                        rng_update(engine, r);
                        r.index = r.value;
                        if cbool(r.value == 0) {
                            engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                        }
                    }
                    saved_f6 = engine.mem(0xF6);
                    r.offset = engine.mem(0xF6);
                    r.value = engine.mem(0xF4);
                    routine_0108(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        routine_0236(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 4;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0237(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xF1, 0x00);
                    routine_0247(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    routine_0239(engine, r);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    routine_0238(engine, r);
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    routine_0250(engine, r);
                    routine_0242(engine, r);
                    engine.set_mem(0xF6, saved_f6);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0221 {
    use super::*;
    pub fn routine_0221(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            routine_0234(engine, r);
        }
        {
            let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            let mut v: i32 = engine.mem(u16v(ptr + 0x09));
            r.offset = v;
            r.value = engine.mem(0xF4);
            routine_0108(engine, r);
        }
        routine_0248(engine, r);
        if cbool(r.carry) {
            routine_0239(engine, r);
        } else {
            routine_0238(engine, r);
        }
        routine_0242(engine, r);
    }
}

mod routine_0222 {
    use super::*;
    pub fn routine_0222(engine: &mut Engine, r: &mut RoutineContext) {
        let mut reached_EBC6: i32 = 0;
        let mut reached_EBCC: i32 = 0;
        let mut done: i32 = 0;
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            routine_0233(engine, r);
        }
        if cbool(engine.mem(0xF0) != 0) {
            routine_0236(engine, r);
            if cbool(r.carry == 0) {
                reached_EBC6 = 1;
            } else {
                done = 1;
            }
        } else {
            let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(ptr + 9));
            r.value = engine.mem(0xF4);
            routine_0108(engine, r);
            routine_0247(engine, r);
            if cbool(r.carry) {
                reached_EBCC = 1;
            } else {
                r.offset = 0x01;
                routine_0252(engine, r);
                if cbool(r.carry == 0) {
                    reached_EBCC = 1;
                } else if cbool(engine.mem(0x0E) == 0) {
                    reached_EBC6 = 1;
                } else {
                    r.offset = 0x0D;
                    routine_0252(engine, r);
                    if cbool(r.carry == 0) {
                        reached_EBCC = 1;
                    } else {
                        reached_EBC6 = 1;
                    }
                }
            }
        }
        if !cbool(done) {
            if cbool(reached_EBCC) {
                routine_0239(engine, r);
            } else if cbool(reached_EBC6) {
                routine_0238(engine, r);
            }
        }
        routine_0250(engine, r);
        routine_0242(engine, r);
    }
}

mod routine_0223 {
    use super::*;
    pub fn routine_0223(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF4, engine.mem(0xF4) & 0x0F);
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0) {
                        if cbool(engine.mem(0xF3) < 0x10) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF9) == 0) {
                        let mut ptr: i32 = 0;
                        engine.set_mem(0x0C, engine.mem(0xFA));
                        engine.set_mem(0x0D, engine.mem(0xFB));
                        routine_0090(engine, r);
                        ptr = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                        if cbool((engine.mem(ptr) & 0x3F) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool((engine.mem(u16v(ptr + 1)) & 0x3F) == 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    }
                    if cbool((engine.mem(0xF4) & 0x03) == 0) {
                        engine.set_mem(0xF4, 0x01);
                    }
                    {
                        let mut x: i32 = u8v(engine.mem(0xF3) - 1);
                        engine.set_mem(0xF3, 0x00);
                        if cbool(x == 0) {
                            if cbool((engine.mem(0xF4) & 0x03) == 0) {
                                {
                                    state = 1;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF4, u8v(engine.mem(0xF4) ^ 0x03));
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    routine_0231(engine, r);
                    engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xF3, 0x00);
                    routine_0231(engine, r);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    {
                        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(ptr + 0x09));
                    }
                    r.value = engine.mem(0xF4);
                    routine_0108(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        routine_0236(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 6;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    routine_0237(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0xF1, 0x00);
                    routine_0247(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    routine_0239(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    routine_0238(engine, r);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    routine_0250(engine, r);
                    routine_0242(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0224 {
    use super::*;
    pub fn routine_0224(engine: &mut Engine, r: &mut RoutineContext) {
        let mut skip: i32 =
            u8v((cbool((engine.mem(0xF5) | engine.mem(0xF7)) != 0)
                && cbool(engine.mem(0xF3) < 0x20)));
        if !cbool(skip) {
            routine_0232(engine, r);
        }
        {
            let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(ptr + 0x09));
            r.value = engine.mem(0xF4);
            routine_0108(engine, r);
        }
        routine_0248(engine, r);
        if cbool(r.carry) {
            routine_0256(engine, r);
            if cbool(r.carry) {
                routine_0239(engine, r);
                routine_0242(engine, r);
                return;
            }
        }
        routine_0238(engine, r);
        routine_0242(engine, r);
    }
}

mod routine_0225 {
    use super::*;
    pub fn routine_0225(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF0) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0x0F, engine.mem(0xFA));
                    engine.set_mem(0x0E, engine.mem(0xF9));
                    engine.set_mem(0x0A, engine.mem(0xFB));
                    routine_0230(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
                    engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
                    {
                        state = 3;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
                        routine_0233(engine, r);
                    }
                    routine_0112(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(ptr + 0x09));
                    }
                    r.value = engine.mem(0xF4);
                    routine_0108(engine, r);
                    routine_0247(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    routine_0230(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    engine.set_mem(0xF5, 0x00);
                    engine.set_mem(0xF6, 0x00);
                    routine_0250(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    routine_0236(engine, r);
                    routine_0238(engine, r);
                    {
                        let mut saved_f0: i32 = engine.mem(0xF0);
                        routine_0250(engine, r);
                        if !cbool(r.carry) {
                            {
                                state = 4;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF1, u8v(saved_f0 + 0x05 + 1));
                        {
                            state = 7;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    routine_0238(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    routine_0237(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 6;
                            continue 'dispatch;
                        }
                    }
                    routine_0238(engine, r);
                    {
                        state = 7;
                        continue 'dispatch;
                    }
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    routine_0239(engine, r);
                    state = 7;
                    continue 'dispatch;
                }
                7 => {
                    routine_0242(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0226 {
    use super::*;
    pub fn routine_0226(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        routine_0223(engine, r);
                        return;
                    }
                    r.value = 0x01;
                    routine_0227(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x02;
                    routine_0227(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x04;
                    routine_0227(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    r.value = 0x08;
                    routine_0227(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        let mut v: i32 = engine.mem(u16v(ptr + 4));
                        engine.set_mem(0x00F2, v);
                        r.value = 0x00;
                        engine.set_mem(0xFC, 0x00);
                    }
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.value = 0x01;
                    engine.set_mem(0xF4, 0x01);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0227 {
    use super::*;
    pub fn routine_0227(engine: &mut Engine, r: &mut RoutineContext) {
        r.offset = 0x01;
        routine_0108(engine, r);
        routine_0241(engine, r);
        routine_0111(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        routine_0249(engine, r);
        r.carry = 1;
    }
}

mod routine_0228 {
    use super::*;
    pub fn routine_0228(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
            routine_0234(engine, r);
        }
        {
            let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
            r.offset = engine.mem(u16v(ptr + 0x09));
            r.value = engine.mem(0xF4);
            routine_0108(engine, r);
        }
        routine_0248(engine, r);
        if cbool(r.carry) {
            if cbool(engine.mem(0xEA) != 0) {
                r.value = 0x80;
                engine.set_mem(0xEE, 0x80);
                return;
            }
            routine_0239(engine, r);
        } else {
            routine_0238(engine, r);
        }
        routine_0242(engine, r);
    }
}

mod routine_0229 {
    use super::*;
    pub fn routine_0229(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dec: i32 = u8v(engine.mem(0xF1) - 1);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF1, dec);
                    if cbool(dec == 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF4) == 0) {
                        routine_0232(engine, r);
                    } else {
                        if cbool(engine.mem(0xF3) >= 0x08) {
                            let mut diff: i32 = 0;
                            let mut bit_count: i32 = 0;
                            let mut changed_bits: i32 = 0;
                            engine.set_mem(0x08, engine.mem(0xF4));
                            routine_0232(engine, r);
                            diff = u8v(engine.mem(0xF4) ^ engine.mem(0x08));
                            changed_bits = 0x00;
                            bit_count = 0x04;
                            loop {
                                let mut bit: i32 = diff & 1;
                                diff >>= 1;
                                if cbool(bit) {
                                    {
                                        let __old = changed_bits;
                                        changed_bits += 1;
                                        __old
                                    };
                                }
                                if !cbool(
                                    {
                                        bit_count -= 1;
                                        bit_count
                                    } != 0,
                                ) {
                                    break;
                                }
                            }
                            {
                                let __old = changed_bits;
                                changed_bits -= 1;
                                __old
                            };
                            if cbool(changed_bits != 0) {
                                engine.set_mem(0xF4, engine.mem(0x08));
                            }
                        }
                    }
                    {
                        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                        r.offset = engine.mem(u16v(ptr + 0x09));
                        r.value = engine.mem(0xF4);
                        routine_0108(engine, r);
                    }
                    routine_0248(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    routine_0238(engine, r);
                    routine_0242(engine, r);
                    return;
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    r.value = 0x00;
                    engine.set_mem(0xEE, 0x00);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0230 {
    use super::*;
    pub fn routine_0230(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0x0A) & 0x0F) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, u8v(engine.mem(0x0A) - 0x10));
        routine_0090(engine, r);
        r.offset = 0x00;
        routine_0255(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        r.offset = 0x0C;
        routine_0255(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
    }
}

mod routine_0231 {
    use super::*;
    pub fn routine_0231(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x00;
        let mut dx: i32 = u16v(u16v(engine.mem(0xFA)) - engine.mem(0x0044));
        if cbool(u8v(dx) != 0) {
            {
                x += 1;
                x
            };
            if !cbool(dx & 0x100) {
                {
                    x += 1;
                    x
                };
            }
        }
        engine.set_mem(0xF4, x);
        {
            let mut dy: i32 = u16v(u16v(engine.mem(0xFB)) - engine.mem(0x0045));
            if !cbool(dy & 0x100) {
                let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
                let mut flag: i32 = engine.mem(u16v(ptr + 0x09));
                if cbool(flag != 0) {
                    r.value = 0x03;
                    rng_update(engine, r);
                    r.index = r.value;
                    if cbool(r.index == 0) {
                        engine.set_mem(0xF4, u8v(engine.mem(0xF4) | 0x80));
                    }
                }
            } else {
                r.value = 0x03;
                rng_update(engine, r);
                r.index = r.value;
                if cbool(r.index == 0) {
                    engine.set_mem(0xF4, 0x04);
                }
            }
        }
    }
}

mod routine_0232 {
    use super::*;
    pub fn routine_0232(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0A, engine.mem(0xFB));
        routine_0112(engine, r);
        x = 0x00;
        if cbool(r.carry == 0) {
            let mut d: i32 = u8v(engine.mem(0xFA) - engine.mem(0x44));
            let mut carry: i32 = u8v((if cbool(engine.mem(0xFA) >= engine.mem(0x44)) {
                1
            } else {
                0
            }));
            x = 0x01;
            if cbool(carry) {
                x = 0x02;
            }
        }
        engine.set_mem(0xF4, x);
        routine_0113(engine, r);
        x = 0x00;
        if cbool(r.carry == 0) {
            let mut carry: i32 = u8v((if cbool(engine.mem(0xFB) >= engine.mem(0x45)) {
                1
            } else {
                0
            }));
            x = 0x04;
            if cbool(carry) {
                x = 0x08;
            }
        }
        engine.set_mem(0xF4, u8v(x | engine.mem(0xF4)));
        engine.set_mem(0xF3, 0x00);
    }
}

mod routine_0233 {
    use super::*;
    pub fn routine_0233(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = engine.mem(0xF4) & 0x03;
        if cbool(v == 0) {
            v = 0x01;
        }
        v ^= 0x03;
        engine.set_mem(0xF4, v);
        r.value = v;
    }
}

mod routine_0234 {
    use super::*;
    pub fn routine_0234(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x08;
        rng_update(engine, r);
        r.index = r.value;
        engine.set_mem(0xF4, engine.mem(u16v(0xEEB3 + r.index)));
    }
}

mod routine_0235 {
    use super::*;
    const sound_lookup_eeb3: [i32; 8] = [0x01, 0x05, 0x04, 0x06, 0x02, 0x0A, 0x08, 0x09];

    pub fn routine_0235(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        r.value = 0x03;
        rng_update(engine, r);
        x = u8v(r.value << 1);
        engine.set_mem(0xF4, sound_lookup_eeb3[x as usize]);
    }
}

mod routine_0236 {
    use super::*;
    pub fn routine_0236(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF7, u8v((engine.mem(0xF0) >> 1) + 0x02));
        routine_0247(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        routine_0247(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF7, 0x00);
    }
}

mod routine_0237 {
    use super::*;
    pub fn routine_0237(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0xF1);
        if cbool(x == 0) {
            x = 0x0F;
        }
        x = u8v(x - 1);
        engine.set_mem(0xF1, x);
        r.index = x;
        engine.set_mem(0xF7, u8v(((x >> 1) ^ 0xFF) + 1));
        routine_0247(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        routine_0247(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF1, u8v(engine.mem(0xF1) + 1));
        routine_0256(engine, r);
    }
}

mod routine_0238 {
    use super::*;
    pub fn routine_0238(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        r.value = engine.mem(0x0A);
    }
}

mod routine_0239 {
    use super::*;
    pub fn routine_0239(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF5, 0);
        engine.set_mem(0xF7, 0);
        engine.set_mem(0xF1, 0);
        engine.set_mem(0xF0, 0);
    }
}

mod routine_0241 {
    use super::*;
    pub fn routine_0241(engine: &mut Engine, r: &mut RoutineContext) {
        let mut dx: i32 = 0;
        let mut sum: i32 = 0;
        let mut carry: i32 = 0;
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0A, engine.mem(0xFB));
        if cbool(engine.mem(0xF7) != 0) {
            engine.set_mem(0x0A, u8v(engine.mem(0xF7) + engine.mem(0x0A)));
        }
        dx = engine.mem(0xF5);
        if cbool(dx != 0) {
            sum = u8v(dx + engine.mem(0x0E));
            engine.set_mem(0x0E, u8v(sum & 0x0F));
            carry = u8v((sum >> 4) & 1);
            engine.set_mem(0x0F, u8v(engine.mem(0x0F) + engine.mem(0xF6) + carry));
        }
    }
}

mod routine_0242 {
    use super::*;
    pub fn routine_0242(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        const table: [i32; 4] = [0xF03B, 0xF04B, 0xF071, 0xF0B9];
        let mut idx: i32 = u8v(engine.mem(u16v(ptr + 7)) & 0x03);
        let mut handler: i32 = table[idx as usize];
        engine.set_mem(0x0E, u8v(handler & 0xFF));
        engine.set_mem(0x0F, u8v(handler >> 8));
        r.offset = 0x07;
        r.index = u8v(idx << 1);
        r.value = u8v(idx << 1);
        match idx {
            0 => {
                routine_0243(engine, r);
            }
            1 => {
                routine_0244(engine, r);
            }
            2 => {
                routine_0245(engine, r);
            }
            3 => {
                routine_0246(engine, r);
            }
            _ => {}
        }
    }
}

mod routine_0243 {
    use super::*;
    pub fn routine_0243(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        engine.inc_mem(0xF3);
        a = engine.mem(0xF3) & 0x03;
        if cbool(a == 0) {
            a = engine.mem(0xEF) ^ 0x40;
            engine.set_mem(0xEF, a);
        }
        r.value = a;
    }
}

mod routine_0244 {
    use super::*;
    pub fn routine_0244(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF5) != 0) {
            let mut y: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, y);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | y));
        }
        engine.inc_mem(0xF3);
        if cbool((engine.mem(0xF3) & 0x03) == 0) {
            engine.xor_mem(0xED, 0x04);
        }
    }
}

mod routine_0245 {
    use super::*;
    pub fn routine_0245(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF5) != 0) {
            let mut y: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, y);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | y));
            engine.set_mem(0xED, u8v(engine.mem(0xED) & 0xF7));
        } else {
            if cbool(engine.mem(0xF7) != 0) {
                engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | 0x08));
            }
        }
        engine.inc_mem(0xF3);
        if cbool((engine.mem(0xF3) & 0x03) == 0) {
            if cbool((engine.mem(0xED) & 0x08) != 0) {
                engine.xor_mem(0xEF, 0x40);
            } else {
                engine.xor_mem(0xED, 0x04);
            }
        }
    }
}

mod routine_0246 {
    use super::*;
    pub fn routine_0246(engine: &mut Engine, r: &mut RoutineContext) {
        let mut t: i32 = 0;
        if cbool(engine.mem(0xF5) != 0) {
            let mut y: i32 = (if cbool(engine.mem(0xF6) & 0x80) {
                0x00
            } else {
                0x40
            });
            engine.set_mem(0x08, y);
            engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | y));
        }
        engine.inc_mem(0xF3);
        t = u8v((engine.mem(0xF3) & 0x06) << 1);
        engine.set_mem(0x08, t);
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | t));
    }
}

mod routine_0247 {
    use super::*;
    pub fn routine_0247(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_f7: i32 = engine.mem(0xF7);
        let mut cflag: i32 = 0;
        loop {
            routine_0241(engine, r);
            routine_0115(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
                engine.set_mem(0xF3, 0xF0);
                cflag = 1;
                break;
            }
            if cbool(u8v(engine.mem(0xEE) - 1) == 0) {
                routine_0111(engine, r);
                if cbool(r.carry) {
                    routine_0249(engine, r);
                }
            }
            routine_0253(engine, r);
            if cbool(r.carry == 0) {
                cflag = 0;
                break;
            }
            {
                let mut x: i32 = engine.mem(0xF7);
                if cbool(x == 0) {
                    cflag = 1;
                    break;
                }
                if !cbool(x & 0x80) {
                    x = u8v(x - 2);
                }
                x = u8v(x + 1);
                engine.set_mem(0xF7, x);
                if cbool(x == 0) {
                    cflag = 1;
                    break;
                }
            }
        }
        engine.set_mem(0xF7, saved_f7);
        r.carry = cflag;
    }
}

mod routine_0248 {
    use super::*;
    pub fn routine_0248(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0241(engine, r);
        routine_0111(engine, r);
        if cbool(r.carry) {
            routine_0249(engine, r);
            r.carry = 1;
            return;
        }
        routine_0115(engine, r);
        if cbool(r.carry == 0) {
            return;
        }
        engine.set_mem(0xEE, 0x00);
        engine.set_mem(0xF3, 0xF0);
    }
}

mod routine_0249 {
    use super::*;
    pub fn routine_0249(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x85) != 0) {
            return;
        }
        if cbool(u8v(engine.mem(0xEE) - 1) != 0) {
            return;
        }
        if cbool(engine.mem(0x2D) >= 0x30) {
            if cbool(engine.mem(0xE3) != 0) {
                let mut x: i32 = engine.mem(0x55);
                if cbool(engine.mem(u16v(0x0051 + x)) == 0x0A) {
                    engine.set_mem(0x8F, 0x01);
                    return;
                }
            }
        } else {
            if cbool(engine.mem(0x40) == 0x04) {
                return;
            }
        }
        r.value = engine.mem(0xF8);
        routine_0203(engine, r);
        engine.set_mem(0x8F, 0x21);
        engine.set_mem(0x90, 0x01);
        engine.set_mem(0x85, 0x01);
        engine.set_mem(0xEF, u8v(engine.mem(0xEF) & 0xDF));
    }
}

mod routine_0250 {
    use super::*;
    fn f179_fail(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF0) >= 0x0C) {
            engine.set_mem(0xF1, u8v(engine.mem(0xF0) - 0x04));
        }
        engine.set_mem(0xF0, 0x00);
        r.carry = 1;
    }

    fn f179_ok(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
        r.carry = 0;
    }

    pub fn routine_0250(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        if cbool(engine.mem(0xF1) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0xFA));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        x = engine.mem(0xFB);
        y = u8v(engine.mem(0xEE) - 1);
        if cbool(y == 0) {
            if cbool(x >= 0xB0) {
                return;
            }
            engine.set_mem(0x0D, x);
            x = u8v(x + 1);
            engine.set_mem(0x0A, x);
            routine_0111(engine, r);
            if cbool(r.carry) {
                return;
            }
        } else {
            if cbool(x != 0xEF) {
            } else {
                x = engine.mem(0xFC);
            }
            engine.set_mem(0x0D, x);
        }
        routine_0090(engine, r);
        if cbool(engine.mem(0xF9) == 0) {
            let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            if cbool((engine.mem(ptr) & 0x3F) == 0) {
                return;
            }
            if cbool((engine.mem(u16v(ptr + 1)) & 0x3F) == 0) {
                return;
            }
        }
        r.offset = 0x01;
        routine_0252(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0xF9) == 0) {
            return;
        }
        r.offset = 0x0D;
        routine_0252(engine, r);
        if cbool(r.carry) {
            return;
        }
        f179_ok(engine, r);
    }
}

mod routine_0251 {
    use super::*;
    fn bail(engine: &mut Engine, r: &mut RoutineContext) {
        let mut f0: i32 = engine.mem(0xF0);
        if cbool(f0 >= 0x0C) {
            engine.set_mem(0xF1, u8v(f0 - 0x04));
        }
        engine.set_mem(0xF0, 0x00);
    }

    pub fn routine_0251(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xF1) != 0) {
            return;
        }
        engine.set_mem(0x0C, engine.mem(0xFA));
        engine.set_mem(0x0F, engine.mem(0xFA));
        engine.set_mem(0x0E, engine.mem(0xF9));
        engine.set_mem(0x0D, engine.mem(0xFB));
        engine.set_mem(0x0A, u8v(engine.mem(0xFB) + 1));
        routine_0090(engine, r);
        if cbool(engine.mem(0xFB) >= 0xA0) {
            engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
            return;
        }
        routine_0114(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.offset = 0x02;
        routine_0252(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.offset = 0x0E;
        routine_0252(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0xF9) != 0) {
            r.offset = 0x1A;
            routine_0252(engine, r);
            if cbool(r.carry) {
                return;
            }
        }
        engine.set_mem(0xF0, u8v(engine.mem(0xF0) + 1));
    }
}

mod routine_0252 {
    use super::*;
    pub fn routine_0252(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut v: i32 = u8v(engine.mem(u16v(ptr + r.offset)) & 0x3F);
        r.carry = u8v(u8v(v >= 0x30));
    }
}

mod routine_0253 {
    use super::*;
    pub fn routine_0253(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, engine.mem(0x0A));
        routine_0090(engine, r);
        r.offset = 0x00;
        routine_0255(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0x0E) != 0) {
            r.offset = 0x0C;
            routine_0255(engine, r);
            if cbool(r.carry) {
                return;
            }
        }
        if cbool(engine.mem(0x0A) >= 0xB0) {
            return;
        }
        if cbool((engine.mem(0x0A) & 0x0F) == 0) {
            return;
        }
        r.offset = 0x01;
        routine_0255(engine, r);
        if cbool(r.carry) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        r.offset = 0x0D;
        routine_0255(engine, r);
        if cbool(r.carry) {
            return;
        }
        r.carry = 0;
    }
}

mod routine_0254 {
    use super::*;
    fn probe(engine: &mut Engine, r: &mut RoutineContext, mut y: i32) -> i32 {
        r.offset = y;
        routine_0255(engine, r);
        return r.carry;
    }

    pub fn routine_0254(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0C, engine.mem(0x0F));
        engine.set_mem(0x0D, engine.mem(0x0A));
        routine_0090(engine, r);
        if cbool(probe(engine, r, 0x00)) {
            return;
        }
        if cbool(probe(engine, r, 0x01)) {
            return;
        }
        if cbool(probe(engine, r, 0x0C)) {
            return;
        }
        if cbool(probe(engine, r, 0x0D)) {
            return;
        }
        if cbool(engine.mem(0x0E) != 0) {
            if cbool(probe(engine, r, 0x18)) {
                return;
            }
            if cbool(probe(engine, r, 0x19)) {
                return;
            }
        }
        if cbool(engine.mem(0x0A) >= 0xB0) {
            return;
        }
        if cbool((engine.mem(0x0A) & 0x0F) == 0) {
            return;
        }
        if cbool(probe(engine, r, 0x02)) {
            return;
        }
        if cbool(probe(engine, r, 0x0E)) {
            return;
        }
        if cbool(engine.mem(0x0E) == 0) {
            return;
        }
        if cbool(probe(engine, r, 0x1A)) {
            return;
        }
        r.carry = 0;
    }
}

mod routine_0255 {
    use super::*;
    pub fn routine_0255(engine: &mut Engine, r: &mut RoutineContext) {
        let mut ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
        let mut v: i32 = u8v(engine.mem(u16v(ptr + r.offset)) & 0x3F);
        r.carry = u8v(u8v(v >= 0x30));
    }
}

mod routine_0256 {
    use super::*;
    pub fn routine_0256(engine: &mut Engine, r: &mut RoutineContext) {
        let mut v: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF6, 0x00);
                    if cbool(engine.mem(0xF5) != 0) {
                        engine.set_mem(0xF5, 0x00);
                        v = engine.mem(0xFB) & 0x0F;
                        if cbool(v == 0) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        if cbool(v < 0x06) {
                            if cbool(engine.mem(0xF4) & 0x04) {
                                {
                                    state = 2;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF7, 0xFF);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool(v >= 0x0B) {
                            if cbool(engine.mem(0xF4) & 0x08) {
                                {
                                    state = 2;
                                    continue 'dispatch;
                                }
                            }
                            engine.set_mem(0xF7, 0x01);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF7) == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF7, 0x00);
                    v = engine.mem(0xF9);
                    if cbool(v == 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    if cbool(v < 0x06) {
                        if cbool(engine.mem(0xF4) & 0x01) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF5, 0x0F);
                        engine.set_mem(0xF6, 0xFF);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    if cbool(v >= 0x0B) {
                        if cbool(engine.mem(0xF4) & 0x02) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                        engine.set_mem(0xF5, 0x01);
                        engine.set_mem(0xF6, 0x00);
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    {
                        state = 2;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    routine_0247(engine, r);
                    return;
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    r.carry = 1;
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0257 {
    use super::*;
    fn farcall_0C0D(engine: &mut Engine, r: &mut RoutineContext, target: RoutineFn) {
        let mut old6: i32 = engine.mem(0x30);
        let mut old7: i32 = engine.mem(0x31);
        engine.set_mem(0x32, old6);
        engine.set_mem(0x33, old7);
        engine.set_mem(0x30, 0x0C);
        engine.set_mem(0x31, 0x0D);
        engine.set_mem(0x25, 0x07);
        engine.prg_map_shadow();
        target(engine, r);
        engine.set_mem(0x31, old7);
        engine.set_mem(0x30, old6);
        engine.set_mem(0x25, 0x06);
        engine.prg_map_shadow();
    }

    pub fn routine_0257(engine: &mut Engine, r: &mut RoutineContext) {
        let mut e7: i32 = u16v(engine.mem(0xE7) | (engine.mem(0xE8) << 8));
        engine.set_mem(0x2E, 0x3D);
        engine.set_mem(0x0A, engine.mem(u16v(e7 + 3)));
        engine.set_mem(0x0F, engine.mem(u16v(e7 + 2)));
        engine.set_mem(0x0E, 0x00);
        engine.set_mem(0x0B, 0x00);
        routine_0254(engine, r);
        if cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
        engine.set_mem(0xF1, 0x00);
        engine.set_mem(0xF0, 0x00);
        engine.set_mem(0xF4, 0x00);
        engine.set_mem(0xEE, 0x01);
        engine.set_mem(0xED, 0x81);
        engine.set_mem(0xEF, 0x02);
        engine.set_mem(0xF8, engine.mem(u16v(e7 + 5)));
        {
            let mut bl: i32 = engine.mem(u16v(e7 + 4));
            engine.set_mem(0xF2, bl);
            engine.set_mem(0x0415, bl);
            engine.set_mem(0x0425, bl);
            engine.set_mem(0x0435, bl);
        }
        engine.set_mem(0x0E, 0xE1);
        engine.set_mem(0x0F, 0xA7);
        farcall_0C0D(engine, r, routine_0017);
        engine.set_mem(0x0E, 0x53);
        engine.set_mem(0x0F, 0xCB);
        farcall_0C0D(engine, r, routine_0098);
    }
}

mod routine_0258 {
    use super::*;
    pub fn routine_0258(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    engine.set_mem(0xF4, engine.mem(0xF4) & 0x0F);
                    if cbool((engine.mem(0xF5) | engine.mem(0xF7)) == 0) {
                        if cbool((engine.mem(0xF4) & 0x03) == 0) {
                            engine.set_mem(0xF4, 0x01);
                        }
                        {
                            let mut x: i32 = engine.mem(0xF3);
                            engine.set_mem(0xF3, 0x00);
                            x = u8v(x - 1);
                            if cbool(x == 0) {
                                a = engine.mem(0xF4) & 0x03;
                                if cbool(a != 0) {
                                    engine.set_mem(0xF4, u8v(a ^ 0x03));
                                    {
                                        state = 2;
                                        continue 'dispatch;
                                    }
                                }
                                {
                                    state = 1;
                                    continue 'dispatch;
                                }
                            }
                            routine_0231(engine, r);
                            engine.set_mem(0xF4, u8v(0x80 | engine.mem(0xF4)));
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    } else {
                        if cbool(engine.mem(0xF3) < 0x32) {
                            {
                                state = 2;
                                continue 'dispatch;
                            }
                        }
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    engine.set_mem(0xF3, 0x00);
                    routine_0231(engine, r);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    r.value = engine.mem(0xF4);
                    r.offset = 0x02;
                    routine_0108(engine, r);
                    if cbool(engine.mem(0xF0) != 0) {
                        routine_0260(engine, r);
                        if cbool(r.carry) {
                            {
                                state = 6;
                                continue 'dispatch;
                            }
                        }
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0xF1) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if !cbool(engine.mem(0xF4) & 0x80) {
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    routine_0261(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    engine.set_mem(0xF1, 0x00);
                    routine_0262(engine, r);
                    if !cbool(r.carry) {
                        {
                            state = 5;
                            continue 'dispatch;
                        }
                    }
                    routine_0239(engine, r);
                    {
                        state = 6;
                        continue 'dispatch;
                    }
                    state = 5;
                    continue 'dispatch;
                }
                5 => {
                    routine_0238(engine, r);
                    state = 6;
                    continue 'dispatch;
                }
                6 => {
                    routine_0251(engine, r);
                    routine_0263(engine, r);
                    routine_0264(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0260 {
    use super::*;
    pub fn routine_0260(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(engine.mem(0xF0) >> 2);
        a = u8v(a + 1);
        engine.set_mem(0xF7, a);
        r.value = a;
        routine_0262(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        r.value = 0x00;
        routine_0247(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF7, 0x00);
        r.value = 0x00;
    }
}

mod routine_0261 {
    use super::*;
    pub fn routine_0261(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0xF1);
        if cbool(x == 0) {
            x = 0x19;
        }
        x = u8v(x - 1);
        engine.set_mem(0xF1, x);
        r.index = x;
        engine.set_mem(0xF7, u8v(((x >> 2) ^ 0xFF) + 1));
        routine_0262(engine, r);
        if !cbool(r.carry) {
            return;
        }
        engine.set_mem(0xF5, 0x00);
        engine.set_mem(0xF6, 0x00);
        routine_0262(engine, r);
    }
}

mod routine_0262 {
    use super::*;
    pub fn routine_0262(engine: &mut Engine, r: &mut RoutineContext) {
        let mut saved_f7: i32 = engine.mem(0xF7);
        let mut cflag: i32 = 0;
        loop {
            routine_0241(engine, r);
            routine_0115(engine, r);
            if cbool(r.carry) {
                engine.set_mem(0xEE, 0x00);
                engine.set_mem(0xF3, 0xF0);
                cflag = 1;
                break;
            }
            routine_0114(engine, r);
            if cbool(r.carry) {
                routine_0249(engine, r);
            }
            routine_0254(engine, r);
            if cbool(r.carry == 0) {
                cflag = 0;
                break;
            }
            {
                let mut x: i32 = engine.mem(0xF7);
                if cbool(x == 0) {
                    cflag = 1;
                    break;
                }
                if !cbool(x & 0x80) {
                    x = u8v(x - 2);
                }
                x = u8v(x + 1);
                engine.set_mem(0xF7, x);
                if cbool(x == 0) {
                    cflag = 1;
                    break;
                }
            }
        }
        engine.set_mem(0xF7, saved_f7);
        r.carry = cflag;
    }
}

mod routine_0263 {
    use super::*;
    pub fn routine_0263(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0x00;
        if cbool(engine.mem(0xF6) & 0x80) {
        } else if cbool(engine.mem(0xF5) == 0) {
            return;
        } else {
            y = 0x40;
        }
        engine.set_mem(0x08, y);
        engine.set_mem(0xEF, u8v((engine.mem(0xEF) & 0x3F) | y));
    }
}

mod routine_0264 {
    use super::*;
    pub fn routine_0264(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0;
        a = engine.inc_mem(0xF3);
        a = u8v(((a & 0x0C) << 1) | 0x41);
        engine.set_mem(0xED, a);
        r.value = a;
    }
}

mod routine_0265 {
    use super::*;
    fn swap(engine: &mut Engine, mut a: i32, mut b: i32) {
        let mut t: i32 = engine.mem(a);
        engine.set_mem(a, engine.mem(b));
        engine.set_mem(b, t);
    }

    pub fn routine_0265(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x041F, engine.mem(0xFC));
        engine.set_mem(0x042F, engine.mem(0xFC));
        engine.set_mem(0x043F, engine.mem(0xFC));
        {
            let mut fb: i32 = engine.mem(0xFB);
            engine.set_mem(0x041E, fb);
            engine.set_mem(0x042E, u8v(fb + 0x10));
            engine.set_mem(0x043E, u8v(fb + 0x10));
        }
        engine.set_mem(0x041C, engine.mem(0xF9));
        engine.set_mem(0x042C, engine.mem(0xF9));
        engine.set_mem(0x043C, engine.mem(0xF9));
        {
            let mut fa: i32 = engine.mem(0xFA);
            engine.set_mem(0x042D, fa);
            engine.set_mem(0x041D, u8v(fa + 1));
            engine.set_mem(0x043D, u8v(fa + 1));
        }
        {
            let mut xv: i32 = engine.mem(0xEE);
            if !cbool(xv & 0x80) {
                if cbool((engine.mem(0x0411) | engine.mem(0x0421) | engine.mem(0x0431)) & 0x80) {
                    xv = 0x80;
                }
            }
            engine.set_mem(0x0401, xv);
            engine.set_mem(0x0411, xv);
            engine.set_mem(0x0421, xv);
            engine.set_mem(0x0431, xv);
        }
        {
            let mut a: i32 = engine.mem(0xF2);
            if cbool(a >= engine.mem(0x0415)) {
                a = engine.mem(0x0415);
            }
            if cbool(a >= engine.mem(0x0425)) {
                a = engine.mem(0x0425);
            }
            if cbool(a >= engine.mem(0x0435)) {
                a = engine.mem(0x0435);
            }
            engine.set_mem(0x0405, a);
        }
        {
            let mut ed: i32 = engine.mem(0xED);
            let mut a: i32 = u8v(ed | 0x04);
            engine.set_mem(0x0410, a);
            a = u8v(a | 0x20);
            engine.set_mem(0x0430, a);
            a = u8v(a & 0xFB);
            engine.set_mem(0x0420, a);
        }
        {
            let mut ef: i32 = engine.mem(0xEF);
            engine.set_mem(0x0412, ef);
            engine.set_mem(0x0422, ef);
            engine.set_mem(0x0432, ef);
            if cbool(ef & 0x40) {
                swap(engine, 0x0400, 0x0410);
                swap(engine, 0x0420, 0x0430);
            }
            if cbool(ef & 0x80) {
                swap(engine, 0x0400, 0x0420);
                swap(engine, 0x0410, 0x0430);
            }
        }
        {
            let mut old6: i32 = engine.mem(0x30);
            let mut old7: i32 = engine.mem(0x31);
            engine.set_mem(0x32, old6);
            engine.set_mem(0x33, old7);
            engine.set_mem(0x30, 0x0C);
            engine.set_mem(0x31, 0x0D);
            engine.set_mem(0x25, 0x07);
            engine.prg_map_shadow();
            engine.set_mem(0x0E, 0x53);
            engine.set_mem(0x0F, 0xCB);
            routine_0098(engine, r);
            engine.set_mem(0x31, old7);
            engine.set_mem(0x30, old6);
            engine.set_mem(0x25, 0x06);
            engine.prg_map_shadow();
        }
    }
}

mod update_player_projectiles {
    use super::*;

    /// Walks the pooled player projectile slots at `0x04B0` and either updates
    /// an active slot or spawns a new shot on a fire-button edge.
    pub fn update_player_projectiles(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xE3, 0x0B);
        engine.set_mem(0xE5, 0xB0);
        engine.set_mem(0xE6, 0x04);
        loop {
            let slot_ptr: i32 = u16v(engine.mem(0xE5) | (engine.mem(0xE6) << 8));
            let active_lifetime: i32 = engine.mem(u16v(slot_ptr + 1));
            if cbool(active_lifetime != 0) {
                r.value = active_lifetime;
                r.offset = 0x01;
                update_player_projectile_slot(engine, r);
            } else {
                if cbool(engine.mem(0x20) & 0x40) {
                    if !cbool(engine.mem(0xFD) & 0x40) {
                        r.value = 0x00;
                        r.offset = 0x01;
                        spawn_player_projectile(engine, r);
                    }
                }
            }
            engine.inc_mem(0xE3);
            {
                let next_slot_lo: i32 = u16v(0x10 + engine.mem(0xE5));
                engine.set_mem(0xE5, u8v(next_slot_lo));
                engine.set_mem(0xE6, u8v(engine.mem(0xE6) + (next_slot_lo >> 8)));
            }
            if !cbool(u8v(engine.mem(0xE3) - 0x0B) < engine.mem(0x5E)) {
                break;
            }
        }
    }
}

mod spawn_player_projectile {
    use super::*;

    /// Initializes the current empty projectile slot from the player's facing,
    /// current pose, and resource constraints.
    pub fn spawn_player_projectile(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    load_object_slot_scratch(engine, r);
                    engine.set_mem(0xFD, u8v((engine.mem(0x20) & 0x40) | engine.mem(0xFD)));
                    r.offset = u8v((if cbool(engine.mem(0x88) != 0) {
                        0x04
                    } else {
                        0x02
                    }));
                    r.value = engine.mem(0xFD);
                    routine_0108(engine, r);
                    project_player_projectile_position(engine, r);
                    routine_0115(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    routine_0204(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF9, engine.mem(0x0E));
                    engine.set_mem(0xFA, engine.mem(0x0F));
                    engine.set_mem(0xFB, engine.mem(0x0A));
                    routine_0126(engine, r);
                    engine.set_mem(0xEE, r.value);
                    if cbool(r.carry == 0) {
                        routine_0204(engine, r);
                    }
                    routine_0125(engine, r);
                    engine.set_mem(0xF8, r.value);
                    if cbool(r.carry == 0) {
                        routine_0204(engine, r);
                    }
                    engine.set_mem(0xEF, 0x00);
                    engine.set_mem(0xED, 0x21);
                    engine.set_mem(0x8F, u8v(0x22 + engine.mem(0x40)));
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xEE) != 0) {
                        apply_projectile_direction_bits(engine, r);
                    }
                    store_object_slot_scratch(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod update_player_projectile_slot {
    use super::*;

    fn store_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0xF9, engine.mem(0x0E));
        engine.set_mem(0xFA, engine.mem(0x0F));
        engine.set_mem(0xFB, engine.mem(0x0A));
    }

    fn finish_projectile_slot_update(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0xEE) != 0) {
            apply_projectile_direction_bits(engine, r);
        }
        store_object_slot_scratch(engine, r);
    }

    /// Advances one active player projectile, applying terrain collision,
    /// actor hits, damage, and expiry back into the object slot.
    pub fn update_player_projectile_slot(engine: &mut Engine, r: &mut RoutineContext) {
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xEE, u8v(engine.mem(0xEE) - 1));
        if cbool(engine.mem(0xEE) == 0) {
            finish_projectile_slot_update(engine, r);
            return;
        }
        routine_0241(engine, r);
        routine_0115(engine, r);
        if cbool(r.carry) {
            engine.set_mem(0xEE, 0x00);
            finish_projectile_slot_update(engine, r);
            return;
        }
        routine_0109(engine, r);
        if !cbool(r.carry) {
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        if (cbool(engine.mem(0x2D) >= 0x30) && cbool(engine.mem(0x08) >= 0x04)) {
            let hit_slot: i32 = engine.mem(0x09);
            engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
            engine.set_mem(0xEE, 0x01);
            engine.set_mem(0x8F, 0x0C);
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
            return;
        }
        {
            let mut hit_slot: i32 = engine.mem(0x09);
            if cbool(u8v(engine.mem(u16v(0x0401 + hit_slot)) - 1) != 0) {
                store_projectile_position(engine, r);
                finish_projectile_slot_update(engine, r);
                return;
            }
            hit_slot = engine.mem(0x09);
            {
                let knockback: i32 = (if cbool(engine.mem(0xEE) & 0x01) {
                    0x02
                } else {
                    0xFE
                });
                engine.set_mem(u16v(0x040F + hit_slot), knockback);
            }
            {
                let target_health: i32 = engine.mem(u16v(0x0405 + hit_slot));
                let projectile_damage: i32 = engine.mem(0xF8);
                engine.set_mem(
                    u16v(0x0405 + hit_slot),
                    u8v(target_health - projectile_damage),
                );
                if cbool(target_health >= projectile_damage) {
                    engine.set_mem(0x8F, 0x06);
                } else {
                    engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
                    engine.set_mem(u16v(0x0405 + hit_slot), 0x00);
                }
            }
            store_projectile_position(engine, r);
            finish_projectile_slot_update(engine, r);
        }
    }
}

mod project_player_projectile_position {
    use super::*;

    /// Projects player position plus projectile velocity into the shared
    /// collision-coordinate scratch registers.
    pub fn project_player_projectile_position(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem(0x0E, engine.mem(0x43));
        engine.set_mem(0x0F, engine.mem(0x44));
        engine.set_mem(0x0A, engine.mem(0x45));
        if cbool(engine.mem(0xF7) != 0) {
            let mut a: i32 = u8v(engine.mem(0xF7) << 2);
            a = u8v(a + engine.mem(0x0A));
            engine.set_mem(0x0A, a);
        }
        if cbool(engine.mem(0xF5) != 0) {
            let projected_subtile: i32 =
                u8v(u8v((engine.mem(0xF5) << 2) & 0x0F) + engine.mem(0x0E));
            engine.set_mem(0x0E, projected_subtile & 0x0F);
            engine.set_mem(
                0x0F,
                u8v(engine.mem(0x0F) + engine.mem(0xF6) + ((projected_subtile >> 4) & 1)),
            );
        }
    }
}

mod apply_projectile_direction_bits {
    use super::*;

    /// Copies the projectile direction bits from its lifetime/state byte into
    /// the sprite/object descriptor used by later render and collision code.
    pub fn apply_projectile_direction_bits(engine: &mut Engine, r: &mut RoutineContext) {
        let direction_bits: i32 = engine.mem(0xEE) & 0x0C;
        engine.set_mem(0x08, direction_bits);
        engine.set_mem(0xED, u8v((engine.mem(0xED) & 0xF3) | direction_bits));
        r.value = engine.mem(0xED);
    }
}

mod update_tile_projectile {
    use super::*;

    /// Updates the singleton tile-removal projectile stored at `0x0490` and
    /// restores its saved background tile when it expires.
    pub fn update_tile_projectile(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool(engine.mem(0x0491) == 0) {
            return;
        }
        engine.set_mem(0xE5, 0x90);
        engine.set_mem(0xE6, 0x04);
        load_object_slot_scratch(engine, r);
        engine.set_mem(0xF3, u8v(engine.mem(0xF3) - 1));
        if cbool(engine.mem(0xF3) != 0) {
            update_tile_projectile_motion(engine, r);
            return;
        }
        if cbool((engine.mem(0xED) & 0x01) == 0) {
            if cbool(u8v((engine.mem(0xFB) & 0x0F) | engine.mem(0xF9)) != 0) {
                engine.set_mem(0xF3, u8v(engine.mem(0xF3) + 1));
                update_tile_projectile_motion(engine, r);
                return;
            }
        }
        engine.set_mem(0xEE, 0x00);
        if cbool(engine.mem(0xF0) != 0) {
            engine.set_mem(0x0C, engine.mem(0xFA));
            engine.set_mem(0x0D, engine.mem(0xFB));
            routine_0090(engine, r);
            let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
            engine.set_mem(tile_ptr, engine.mem(0xF0));
            let screen_diff: i32 = u8v(engine.mem(0xFA) - engine.mem(0x7C));
            if (cbool(screen_diff < 0x11) || cbool(screen_diff >= 0xFE)) {
                let tile_x: i32 = engine.mem(0xFA);
                engine.set_mem(0x0C, tile_x);
                engine.set_mem(0x16, u8v((tile_x << 1) & 0x1F));
                engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
                engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
                engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
                farcall_bank_09_r7(engine, r);
            }
        }
        store_object_slot_scratch(engine, r);
    }
}

mod update_tile_projectile_motion {
    use super::*;

    /// Advances the tile-removal projectile, including collision checks,
    /// bouncing, contact damage, and final tile replacement.
    pub fn update_tile_projectile_motion(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    {
                        let mut i: i32 = 0;
                        {
                            i = 0x0800;
                            while cbool(i < 0xA000) {
                                engine.set_mem(i, 0);
                                {
                                    i += 1;
                                    i
                                };
                            }
                        }
                    }
                    if cbool(engine.mem(0xED) & 0x01) {
                        if cbool((engine.mem(0xF3) & 0x03) == 0) {
                            engine.xor_mem(0xED, 0x04);
                        }
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xE3, 0x09);
                    routine_0241(engine, r);
                    routine_0116(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    routine_0253(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    routine_0111(engine, r);
                    if cbool(r.carry) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    routine_0109(engine, r);
                    if cbool(r.carry) {
                        let hit_slot: i32 = engine.mem(0x09);
                        engine.set_mem(u16v(0x0401 + hit_slot), 0x80);
                    }
                    engine.set_mem(0xF9, engine.mem(0x0E));
                    engine.set_mem(0xFA, engine.mem(0x0F));
                    engine.set_mem(0xFB, engine.mem(0x0A));
                    engine.set_mem(0xF4, 0x00);
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    if cbool(engine.mem(0x85) != 0) {
                        {
                            state = 2;
                            continue 'dispatch;
                        }
                    }
                    routine_0202(engine, r);
                    engine.set_mem(0x8F, 0x0A);
                    engine.set_mem(0x85, 0x02);
                    state = 2;
                    continue 'dispatch;
                }
                2 => {
                    if cbool(engine.mem(0xF4) != 0) {
                        {
                            state = 3;
                            continue 'dispatch;
                        }
                    }
                    engine.set_mem(0xF4, u8v(engine.mem(0xF4) + 1));
                    if cbool(engine.mem(0xF5) != 0) {
                        engine.set_mem(0xF5, u8v((0 - engine.mem(0xF5)) & 0x0F));
                        engine.xor_mem(0xF6, 0xFF);
                    }
                    engine.set_mem(0xF7, u8v(u8v(!engine.mem(0xF7)) + 1));
                    if cbool(engine.mem(0x8F) == 0) {
                        engine.set_mem(0x8F, 0x06);
                    }
                    {
                        state = 4;
                        continue 'dispatch;
                    }
                    state = 3;
                    continue 'dispatch;
                }
                3 => {
                    if cbool(u8v((engine.mem(0xFB) & 0x0F) | engine.mem(0xF9)) != 0) {
                        engine.set_mem(0xF3, u8v(engine.mem(0xF3) + 1));
                        {
                            state = 4;
                            continue 'dispatch;
                        }
                    }
                    {
                        engine.set_mem(0xEE, 0x00);
                        if cbool(engine.mem(0xF0) != 0) {
                            engine.set_mem(0x0C, engine.mem(0xFA));
                            engine.set_mem(0x0D, engine.mem(0xFB));
                            routine_0090(engine, r);
                            let tile_ptr: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                            engine.set_mem(tile_ptr, engine.mem(0xF0));
                            let screen_diff: i32 = u8v(engine.mem(0xFA) - engine.mem(0x7C));
                            if (cbool(screen_diff < 0x11) || cbool(screen_diff >= 0xFE)) {
                                let tile_x: i32 = engine.mem(0xFA);
                                engine.set_mem(0x0C, tile_x);
                                engine.set_mem(0x16, u8v((tile_x << 1) & 0x1F));
                                engine.set_mem(0x17, u8v((engine.mem(0xFA) & 0x10) >> 2));
                                engine.set_mem(0x16, u8v(0x00 + engine.mem(0x16)));
                                engine.set_mem(0x17, u8v(0x20 + engine.mem(0x17)));
                                farcall_bank_09_r7(engine, r);
                            }
                        }
                    }
                    state = 4;
                    continue 'dispatch;
                }
                4 => {
                    store_object_slot_scratch(engine, r);
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0273 {
    use super::*;
    fn silence_F95E(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x4000, (engine.mem(0x99) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFE));
    }

    pub fn routine_0273(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((engine.mem(0x94) & 0x80) == 0) {
                        silence_F95E(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0x93)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut ptr: i32 = u16v(engine.mem(0x95) | (engine.mem(0x96) << 8));
                        let mut note: i32 = engine.mem(ptr);
                        if cbool(note == 0) {
                            routine_0287(engine, r);
                            silence_F95E(engine, r);
                            return;
                        }
                        if cbool(note == 0xFF) {
                            routine_0277(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0x93, u8v(note & 0x7F));
                        if cbool(note & 0x80) {
                            routine_0286(engine, r);
                        } else {
                            routine_0283(engine, r);
                            engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x01));
                            engine.device_write(0x4001, engine.mem(0x9A));
                            engine.device_write(0x4002, engine.mem(0x04));
                            engine.device_write(0x4003, (engine.mem(0x05) & 0x07) | 0x18);
                            routine_0285(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x01) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0x9D)) == 0) {
                        routine_0288(engine, r);
                        engine.device_write(0x4000, r.value);
                    }
                    routine_0289(engine, r);
                    if cbool(r.carry) {
                        silence_F95E(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0274 {
    use super::*;
    fn silence_F9F9(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x4004, (engine.mem(0xA9) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFD));
    }

    pub fn routine_0274(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a4: i32 = engine.mem(0xA4);
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((a4 & 0x80) == 0) {
                        if cbool(a4 & 0x40) {
                            return;
                        }
                        silence_F9F9(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xA3)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut ptr: i32 = u16v(engine.mem(0xA5) | (engine.mem(0xA6) << 8));
                        let mut note: i32 = engine.mem(ptr);
                        if cbool(note == 0) {
                            routine_0287(engine, r);
                            silence_F9F9(engine, r);
                            return;
                        }
                        if cbool(note == 0xFF) {
                            routine_0277(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xA3, u8v(note & 0x7F));
                        if cbool(note & 0x80) {
                            if cbool(engine.mem(0xA4) & 0x40) {
                                return;
                            }
                            routine_0286(engine, r);
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                        if cbool(engine.mem(0xA4) & 0x40) {
                            inc16_95(engine, r);
                            return;
                        }
                        routine_0283(engine, r);
                        engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x02));
                        engine.device_write(0x4004, engine.mem(0xA9));
                        engine.device_write(0x4005, engine.mem(0xAA));
                        engine.device_write(0x4006, engine.mem(0x04));
                        engine.device_write(0x4007, (engine.mem(0x05) & 0x07) | 0x18);
                        routine_0285(engine, r);
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool(engine.mem(0xA4) & 0x40) {
                        return;
                    }
                    if cbool((engine.mem(0x27) & 0x02) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xAD)) == 0) {
                        routine_0288(engine, r);
                        engine.device_write(0x4004, r.value);
                    }
                    routine_0289(engine, r);
                    if cbool(r.carry) {
                        silence_F9F9(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0275 {
    use super::*;
    fn fa54(engine: &mut Engine, r: &mut RoutineContext) {
        r.value = 0x00;
        engine.device_write((0x4008), 0x00);
        engine.set_mem(0x27, engine.mem(0x27) & 0xFB);
        r.value = engine.mem(0x27);
    }

    pub fn routine_0275(engine: &mut Engine, r: &mut RoutineContext) {
        if cbool((engine.mem(0xB4) & 0x80) == 0) {
            fa54(engine, r);
            return;
        }
        if cbool(u8v(engine.mem(0xB3) - 1) != 0) {
            engine.set_mem(0xB3, u8v(engine.mem(0xB3) - 1));
            return;
        }
        engine.set_mem(0xB3, u8v(engine.mem(0xB3) - 1));
        loop {
            let mut ptr: i32 = u16v(engine.mem(0xB5) | (engine.mem(0xB6) << 8));
            let mut cmd: i32 = engine.mem(ptr);
            if cbool(cmd == 0) {
                routine_0287(engine, r);
                fa54(engine, r);
                return;
            }
            if cbool(cmd != 0xFF) {
                let mut saved_n: i32 = u8v(cmd & 0x80);
                r.value = cmd;
                inc16_95(engine, r);
                r.value = u8v(cmd & 0x7F);
                engine.set_mem(0xB3, r.value);
                if cbool(saved_n) {
                    fa54(engine, r);
                    return;
                }
                routine_0283(engine, r);
                engine.set_mem(0x27, engine.mem(0x27) | 0x04);
                engine.device_write((0x4008), engine.mem(0xBA));
                engine.device_write((0x400A), engine.mem(0x04));
                r.value = u8v((engine.mem(0x05) & 0x07) | 0xF8);
                engine.device_write((0x400B), r.value);
                return;
            }
            routine_0277(engine, r);
        }
    }
}

mod routine_0276 {
    use super::*;
    fn silence_FB82(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x400C, 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xF7));
    }

    pub fn routine_0276(engine: &mut Engine, r: &mut RoutineContext) {
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool((engine.mem(0xC4) & 0x80) == 0) {
                        silence_FB82(engine, r);
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xC3)) != 0) {
                        {
                            state = 1;
                            continue 'dispatch;
                        }
                    }
                    loop {
                        let mut ptr: i32 = u16v(engine.mem(0xC5) | (engine.mem(0xC6) << 8));
                        let mut note: i32 = engine.mem(ptr);
                        if cbool(note == 0) {
                            routine_0287(engine, r);
                            silence_FB82(engine, r);
                            return;
                        }
                        if cbool(note == 0xFF) {
                            routine_0277(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xC3, u8v(note & 0x7F));
                        if cbool(note & 0x80) {
                            routine_0286(engine, r);
                        } else {
                            engine.set_mem(0x27, u8v(engine.mem(0x27) | 0x08));
                            engine.device_write(0x400E, engine.mem(0xCA));
                            engine.device_write(0x400F, 0x80);
                            routine_0285(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x08) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xCD)) == 0) {
                        routine_0288(engine, r);
                        engine.device_write(0x400C, r.value);
                    }
                    routine_0289(engine, r);
                    if cbool(r.carry) {
                        silence_FB82(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod routine_0277 {
    use super::*;
    fn deref_stream(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
        let mut x: i32 = u8v(r.index);
        let mut lo: i32 = engine.mem((0x95 + x) & 0xFF);
        let mut hi: i32 = engine.mem((0x96 + x) & 0xFF);
        return engine.mem(u16v(lo | (hi << 8)));
    }

    pub fn routine_0277(engine: &mut Engine, r: &mut RoutineContext) {
        r.index = engine.mem(0x02);
        inc16_95(engine, r);
        {
            let __v = deref_stream(engine, r);
            engine.set_mem(0x04, __v);
        }
        inc16_95(engine, r);
        {
            let __v = deref_stream(engine, r);
            engine.set_mem(0x05, __v);
        }
        inc16_95(engine, r);
        let mut idx: i32 = engine.mem(0x04);
        if cbool(idx >= 0x05) {
            return;
        }
        const tbl: [i32; 5] = [0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05];
        let mut p: i32 = tbl[idx as usize];
        engine.set_mem(0x06, u8v(p & 0xFF));
        engine.set_mem(0x07, u8v(p >> 8));
        r.value = engine.mem(0x05);
        r.index = engine.mem(0x02);
        match idx {
            0 => {
                routine_0278(engine, r);
            }
            1 => {
                routine_0279(engine, r);
            }
            2 => {
                routine_0280(engine, r);
            }
            3 => {
                routine_0281(engine, r);
            }
            4 => {
                routine_0282(engine, r);
            }
            _ => {}
        }
    }
}

mod routine_0278 {
    use super::*;
    pub fn routine_0278(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = u8v(r.value);
        let mut x: i32 = u8v(r.index);
        let mut hi: i32 = u8v(u8v(a & 0xF0) << 2);
        engine.set_mem(0x00, hi);
        engine.set_mem(
            (0x99 + x) & 0xFF,
            u8v((engine.mem((0x99 + x) & 0xFF) & 0x3F) | hi),
        );
        a = u8v(a << 4);
        engine.set_mem((0xA2 + x) & 0xFF, a);
        engine.set_mem((0x9A + x) & 0xFF, engine.mem(u16v(0xFDD2 + a)));
        r.value = engine.mem(u16v(0xFDD2 + a));
        r.offset = a;
        r.index = x;
    }
}

mod routine_0279 {
    use super::*;
    pub fn routine_0279(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = u8v(r.index);
        let mut a: i32 = 0;
        let mut take_fbec: i32 = 0;
        a = engine.mem(0x02);
        if cbool(a == 0x40) {
            take_fbec = 1;
        } else {
            a = engine.mem(0x92);
            if cbool(a != 0) {
                r.value = a;
                r.index = x;
                return;
            }
            take_fbec = 1;
        }
        {
            let mut sum: i32 = u16v(0x0F + engine.mem(0x05));
            let mut carry_in: i32 = 1;
            let mut diff: i32 = u16v((sum & 0xFF) - 0x08 + (carry_in - 1));
            let mut bcs: i32 = u8v((sum & 0xFF) >= 0x08);
            a = u8v(diff);
            if !cbool(bcs) {
                a = 0x00;
            }
            a = u8v(a << 1);
            a = u8v(a + 1);
            engine.set_mem((0xA0 + x) & 0xFF, a);
        }
        r.value = a;
        r.index = x;
    }
}

mod routine_0280 {
    use super::*;
    pub fn routine_0280(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0x99 + r.index) & 0xFF, r.value);
    }
}

mod routine_0281 {
    use super::*;
    pub fn routine_0281(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0xA1 + r.index) & 0xFF, r.value);
    }
}

mod routine_0282 {
    use super::*;
    pub fn routine_0282(engine: &mut Engine, r: &mut RoutineContext) {
        engine.set_mem((0x9A + r.index) & 0xFF, r.value);
    }
}

mod routine_0283 {
    use super::*;
    pub fn routine_0283(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut ptr: i32 = u16v(engine.mem(u8v(0x95 + x)) | (engine.mem(u8v(0x96 + x)) << 8));
        let mut note: i32 = engine.mem(ptr);
        inc16_95(engine, r);
        {
            let mut y: i32 = note;
            let mut idx: i32 = u8v((note & 0x0F) << 1);
            let mut lo: i32 = engine.mem(u16v(0xFDB1 + idx));
            let mut hi: i32 = engine.mem(u16v(0xFDB2 + idx));
            x = engine.mem(0x02);
            {
                let mut sub: i32 = u16v(u16v(lo) - engine.mem(u8v(0xA1 + x)));
                lo = u8v(sub);
                if cbool(sub & 0x100) {
                    hi = u8v(hi - 1);
                }
            }
            {
                let mut cnt: i32 = u8v(y >> 4);
                while cbool(cnt != 0) {
                    let mut newcarry: i32 = u8v(hi & 1);
                    hi = u8v(hi >> 1);
                    lo = u8v((lo >> 1) | (newcarry << 7));
                    {
                        cnt -= 1;
                        cnt
                    };
                }
            }
            engine.set_mem(0x04, lo);
            engine.set_mem(0x05, hi);
        }
    }
}

mod routine_0284 {
    use super::*;
    pub fn routine_0284(engine: &mut Engine, r: &mut RoutineContext) {
        let mut a: i32 = 0x00;
        let mut y: i32 = u8v(u8v(r.offset + 1));
        loop {
            a = u8v(a + engine.mem(0x00));
            y = u8v(y - 1);
            if !cbool(y != 0) {
                break;
            }
        }
        a >>= 4;
        engine.set_mem(0x00, a);
        r.value = a;
        r.offset = 0;
    }
}

mod routine_0285 {
    use super::*;
    pub fn routine_0285(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut y: i32 = engine.mem((0xA2 + x) & 0xFF);
        engine.set_mem((0x9B + x) & 0xFF, y);
        engine.set_mem((0x9C + x) & 0xFF, engine.mem(u16v(0xFDCB + y)));
        engine.set_mem((0x9D + x) & 0xFF, engine.mem(u16v(0xFDCC + y)));
        engine.set_mem((0x9E + x) & 0xFF, engine.mem(u16v(0xFDCD + y)));
        engine.set_mem((0x9F + x) & 0xFF, engine.mem(u16v(0xFDCE + y)));
        r.index = x;
        r.offset = y;
    }
}

mod routine_0286 {
    use super::*;
    pub fn routine_0286(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut y: i32 = u8v(engine.mem((0xA2 + x) & 0xFF) + 0x0C);
        engine.set_mem((0x9B + x) & 0xFF, y);
        engine.set_mem((0x9C + x) & 0xFF, engine.mem(u16v(0xFDCB + y)));
        engine.set_mem((0x9D + x) & 0xFF, engine.mem(u16v(0xFDCC + y)));
        engine.set_mem((0x9E + x) & 0xFF, engine.mem(u16v(0xFDCD + y)));
        r.index = x;
        r.offset = y;
        r.value = engine.mem(u16v(0xFDCD + y));
    }
}

mod routine_0287 {
    use super::*;
    pub fn routine_0287(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut hi: i32 = 0;
        engine.set_mem((0x95 + x) & 0xFF, engine.mem((0x97 + x) & 0xFF));
        hi = engine.mem((0x98 + x) & 0xFF);
        engine.set_mem((0x96 + x) & 0xFF, hi);
        if cbool(hi != 0) {
            engine.set_mem((0x93 + x) & 0xFF, 0x01);
        } else {
            engine.and_mem((0x94 + x) & 0xFF, 0x40);
        }
        r.index = x;
    }
}

mod routine_0288 {
    use super::*;
    pub fn routine_0288(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut idx: i32 = engine.mem(u8v(0x9B + x));
        engine.set_mem(u8v(0x9D + x), engine.mem(u16v(0xFDCC + idx)));
        {
            let mut v: i32 = engine.mem(u8v(0x9C + x));
            let mut a: i32 = u8v(v + engine.mem(u8v(0x9F + x)));
            if cbool(v & 0x80) {
                if cbool(a >= 0x10) {
                    a = 0x00;
                }
            } else {
                if cbool(a >= 0x10) {
                    a = 0x0F;
                }
            }
            engine.set_mem(u8v(0x9F + x), a);
            engine.set_mem(0x00, a);
        }
        r.offset = engine.mem(u8v(0xA0 + x));
        routine_0284(engine, r);
        {
            let mut result: i32 = u8v((engine.mem(u8v(0x99 + x)) & 0xC0) | engine.mem(0x00) | 0x30);
            r.value = result;
        }
    }
}

mod routine_0289 {
    use super::*;
    pub fn routine_0289(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x02);
        let mut a: i32 = 0;
        let mut y: i32 = 0;
        if cbool(engine.dec_mem((0x9E + x) & 0xFF) != 0) {
            r.index = x;
            r.carry = 0;
            return;
        }
        a = engine.mem((0x9B + x) & 0xFF) & 0x0F;
        if cbool(a >= 0x0C) {
            r.index = x;
            r.value = a;
            r.carry = 1;
            return;
        }
        y = u8v(engine.mem((0x9B + x) & 0xFF) + 0x04);
        engine.set_mem((0x9B + x) & 0xFF, y);
        engine.set_mem((0x9C + x) & 0xFF, engine.mem(u16v(0xFDCB + y)));
        engine.set_mem((0x9D + x) & 0xFF, engine.mem(u16v(0xFDCC + y)));
        engine.set_mem((0x9E + x) & 0xFF, engine.mem(u16v(0xFDCD + y)));
        r.index = x;
        r.offset = y;
        r.carry = 0;
    }
}

mod scene_assemble {
    use super::*;
    pub fn scene_assemble(engine: &mut Engine, r: &mut RoutineContext) {
        routine_0086(engine, r);
        routine_0085(engine, r);
        r.carry = u8v(u8v((if cbool((engine.mem(0x76) + 0x03) > 0xFF) {
            1
        } else {
            0
        })));
        text_attr_build(engine, r);
        routine_0087(engine, r);
    }
}

mod sfx_overlay_voice {
    use super::*;
    fn silence_FB0F(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x4004, (engine.mem(0xD9) & 0xC0) | 0x30);
        engine.set_mem(0x27, u8v(engine.mem(0x27) & 0xFD));
    }

    pub fn sfx_overlay_voice(engine: &mut Engine, r: &mut RoutineContext) {
        let mut start: i32 = 0;
        let mut state: i32 = 0;
        'dispatch: loop {
            match state {
                0 => {
                    if cbool(engine.mem(0x8F) != 0) {
                        if cbool((engine.mem(0xD4) & 0x80) == 0) {
                            start = 1;
                        } else if cbool(engine.mem(0x90) >= engine.mem(0x91)) {
                            start = 1;
                        } else {
                            engine.set_mem(0x90, 0x00);
                            engine.set_mem(0x8F, 0x00);
                        }
                    }
                    if !cbool(start) {
                        if cbool((engine.mem(0xD4) & 0x80) == 0) {
                            return;
                        }
                        if cbool(u8v(engine.dec_mem(0xD3)) != 0) {
                            {
                                state = 1;
                                continue 'dispatch;
                            }
                        }
                    } else {
                        let mut x: i32 = 0;
                        engine.set_mem(0x91, engine.mem(0x90));
                        x = u8v(engine.mem(0x8F) << 1);
                        engine.set_mem(0xD5, engine.mem(u16v(0x8014 + x)));
                        engine.set_mem(0xD6, engine.mem(u16v(0x8015 + x)));
                        engine.set_mem(0xD4, 0x80);
                        engine.set_mem(0xA4, u8v(engine.mem(0xA4) | 0x40));
                        engine.set_mem(0x8F, 0x00);
                        engine.set_mem(0x90, 0x00);
                    }
                    loop {
                        let mut ptr: i32 = u16v(engine.mem(0xD5) | (engine.mem(0xD6) << 8));
                        let mut note: i32 = engine.mem(ptr);
                        if cbool(note == 0) {
                            engine.set_mem(0xD4, 0x00);
                            engine.set_mem(0x91, 0x00);
                            engine.set_mem(0xA4, u8v(engine.mem(0xA4) & 0xBF));
                            silence_FB0F(engine, r);
                            return;
                        }
                        if cbool(note == 0xFF) {
                            routine_0277(engine, r);
                            continue;
                        }
                        inc16_95(engine, r);
                        engine.set_mem(0xD3, u8v(note & 0x7F));
                        if cbool(note & 0x80) {
                            routine_0286(engine, r);
                        } else {
                            routine_0283(engine, r);
                            engine.set_mem(0x27, u8v(0x02 | engine.mem(0x27)));
                            engine.device_write(0x4005, engine.mem(0xDA));
                            engine.device_write(0x4006, engine.mem(0x04));
                            engine.device_write(0x4007, (engine.mem(0x05) & 0x07) | 0xC0);
                            routine_0285(engine, r);
                        }
                        break;
                    }
                    state = 1;
                    continue 'dispatch;
                }
                1 => {
                    if cbool((engine.mem(0x27) & 0x02) == 0) {
                        return;
                    }
                    if cbool(u8v(engine.dec_mem(0xDD)) == 0) {
                        routine_0288(engine, r);
                        engine.device_write(0x4004, r.value);
                    }
                    routine_0289(engine, r);
                    if cbool(r.carry) {
                        silence_FB0F(engine, r);
                    }
                    break 'dispatch;
                }
                _ => break 'dispatch,
            }
        }
    }
}

mod song_init {
    use super::*;
    pub fn song_init(engine: &mut Engine, r: &mut RoutineContext) {
        let mut song: i32 = engine.mem(0x8E);
        let mut idx: i32 = 0;
        let mut x: i32 = 0;
        let mut blk: i32 = 0;
        x = u8v((if cbool(song < 0x0A) { 0x0A } else { 0x0C }));
        engine.set_mem(0x34, x);
        engine.set_mem(0x35, u8v(x + 1));
        sound_set_song_banks(engine, r);
        engine.set_mem(0x92, 0x00);
        engine.set_mem(0x8F, 0x00);
        idx = u8v((if cbool(song < 0x0A) {
            song
        } else {
            u8v(song - 0x0A)
        }));
        idx = u8v(idx << 1);
        {
            engine.set_mem(0x0E, engine.mem(u16v(0x8000 + idx)));
            engine.set_mem(0x0F, engine.mem(u16v(0x8001 + idx)));
        }
        engine.set_mem(0x0C, 0x93);
        engine.set_mem(0x0D, 0x00);
        {
            blk = 0;
            while cbool(blk < 4) {
                let mut y: i32 = 0;
                let mut s: i32 = u16v(engine.mem(0x0E) | (engine.mem(0x0F) << 8));
                let mut d: i32 = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                {
                    y = 7;
                    while cbool(y >= 0) {
                        engine.set_mem(u16v(d + y), engine.mem(u16v(s + y)));
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                    }
                }
                d = u16v(engine.mem(0x0C) + 8);
                engine.set_mem(0x0C, u8v(d));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (d >> 8)));
                d = u16v(engine.mem(0x0C) | (engine.mem(0x0D) << 8));
                {
                    y = 7;
                    while cbool(y >= 0) {
                        engine.set_mem(u16v(d + y), 0x00);
                        {
                            let __old = y;
                            y -= 1;
                            __old
                        };
                    }
                }
                d = u16v(engine.mem(0x0C) + 8);
                engine.set_mem(0x0C, u8v(d));
                engine.set_mem(0x0D, u8v(engine.mem(0x0D) + (d >> 8)));
                s = u16v(engine.mem(0x0E) + 8);
                engine.set_mem(0x0E, u8v(s));
                engine.set_mem(0x0F, u8v(engine.mem(0x0F) + (s >> 8)));
                {
                    let __old = blk;
                    blk += 1;
                    __old
                };
            }
        }
        ppu_commit_banks(engine, r);
    }
}

mod sound_restore_game_banks {
    use super::*;
    pub fn sound_restore_game_banks(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x06);
        engine.device_write(0x8001, engine.mem(0x30));
        engine.device_write(0x8000, 0x07);
        engine.device_write(0x8001, engine.mem(0x31));
    }
}

mod sound_set_default_banks {
    use super::*;
    pub fn sound_set_default_banks(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0x06;
        let mut y: i32 = 0x0A;
        engine.device_write(0x8000, x);
        engine.device_write(0x8001, y);
        x = u8v(x + 1);
        y = u8v(y + 1);
        engine.device_write(0x8000, x);
        engine.device_write(0x8001, y);
        r.index = x;
        r.offset = y;
    }
}

mod sound_set_song_banks {
    use super::*;
    pub fn sound_set_song_banks(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x8000, 0x06);
        engine.device_write(0x8001, engine.mem(0x34));
        engine.device_write(0x8000, 0x07);
        engine.device_write(0x8001, engine.mem(0x35));
    }
}

mod sound_tick {
    use super::*;
    pub fn sound_tick(engine: &mut Engine, r: &mut RoutineContext) {
        sound_set_default_banks(engine, r);
        engine.set_mem(0x02, 0x40);
        r.value = 0x40;
        sfx_overlay_voice(engine, r);
        if cbool(engine.mem(0x8D) != 0) {
            if !cbool(engine.mem(0xD4) & 0x80) {
                engine.device_write((0x4004), (engine.mem(0xA9) & 0xC0) | 0x30);
            }
            engine.device_write((0x4000), (engine.mem(0x99) & 0xC0) | 0x30);
            engine.device_write((0x4008), 0x00);
            engine.device_write((0x400C), 0x30);
            r.value = 0x30;
        } else {
            sound_set_song_banks(engine, r);
            engine.set_mem(0x02, 0x00);
            r.value = 0x00;
            routine_0273(engine, r);
            engine.set_mem(0x02, 0x10);
            r.value = 0x10;
            routine_0274(engine, r);
            engine.set_mem(0x02, 0x20);
            r.value = 0x20;
            routine_0275(engine, r);
            engine.set_mem(0x02, 0x30);
            r.value = 0x30;
            routine_0276(engine, r);
        }
        sound_restore_game_banks(engine, r);
    }
}

mod statusbar_split {
    use super::*;
    pub fn statusbar_split(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2001, engine.mem(0x24));
        engine.set_mem(0x23, u8v((engine.mem(0x23) & 0xFE) | engine.mem(0x1D)));
        engine.device_write(0x2000, engine.mem(0x23));
        engine.device_write(0x2005, engine.mem(0x1C));
        engine.device_write(0x2005, engine.mem(0x1E));
        if cbool(engine.mem(0x29) != 0) {
            let _ = engine.mem(0x2002);
            engine.device_write(0x2000, engine.mem(0x23) & 0xFE);
            engine.device_write(0x2005, 0x00);
            engine.device_write(0x2005, 0xC4);
            engine.device_write(0x8000, 0x01);
            engine.device_write(0x8001, 0x16);
            engine.device_write(0x8000, 0x04);
            engine.device_write(0x8001, 0x3E);
            engine.device_write(0x8000, 0x05);
            engine.device_write(0x8001, 0x3F);
        }
        sound_tick(engine, r);
        if cbool(engine.mem(0x29) == 0) {
            return;
        }
        engine.device_write(0x8000, 0x01);
        engine.device_write(0x2000, engine.mem(0x23));
        engine.device_write(0x2005, engine.mem(0x1C));
        engine.device_write(0x2005, engine.mem(0x1E));
        engine.device_write(0x8001, engine.mem(0x2B));
        engine.device_write(0x8000, 0x04);
        engine.device_write(0x8001, engine.mem(0x2E));
        engine.device_write(0x8000, 0x05);
        engine.device_write(0x8001, engine.mem(0x2F));
    }
}

mod text_attr_build {
    use super::*;
    pub fn text_attr_build(engine: &mut Engine, r: &mut RoutineContext) {
        let mut p: i32 = u16v(engine.mem(0x77) | (engine.mem(0x78) << 8));
        let mut carry_in: i32 = u8v(r.carry);
        let mut b: i32 = 0;
        b = engine.mem(p);
        engine.set_mem(0x7A, u8v(b + 0xA0 + carry_in));
        engine.set_mem(0x79, 0);
        engine.set_mem(0x2D, engine.mem(u16v(p + 1)));
        engine.set_mem(0x70, engine.mem(u16v(p + 2)));
        engine.set_mem(0x71, engine.mem(u16v(p + 3)));
        engine.set_mem(0x74, engine.mem(u16v(p + 4)));
        engine.set_mem(0x2A, engine.mem(u16v(p + 5)));
        engine.set_mem(0x2B, engine.mem(u16v(p + 6)));
        {
            let mut ms_y: i32 = engine.mem(0x48);
            let mut ms_x: i32 = engine.mem(0x47);
            let mut idx: i32 = u8v(((ms_y << 2) & 0x04) | ms_x);
            let mut a: i32 = engine.mem(u16v(0x0300 + idx));
            let mut cnt: i32 = u8v((ms_y >> 1) + 1);
            let mut c: i32 = 0;
            loop {
                c = u8v((a >> 7) & 1);
                a = u8v(a << 1);
                if !cbool(
                    {
                        cnt -= 1;
                        cnt
                    } != 0,
                ) {
                    break;
                }
            }
            r.value = a;
            r.carry = c;
        }
        {
            let mut y: i32 = 0x07;
            let mut a: i32 = 0;
            if cbool(r.carry) {
                a = engine.mem(u16v(p + y));
            } else {
                a = 0;
            }
            engine.set_mem(0x04A1, a);
            if cbool(a != 0) {
                engine.set_mem(0x04A2, 0x01);
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                engine.set_mem(0x04AD, engine.mem(u16v(p + y)));
                engine.set_mem(0x04AC, 0x00);
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                engine.set_mem(0x04AE, engine.mem(u16v(p + y)));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
                b = engine.mem(u16v(p + y));
                if cbool(b == 0x17) {
                    engine.set_mem(0x04A1, 0x19);
                    engine.set_mem(0x04A0, 0xDD);
                } else {
                    engine.set_mem(0x04A0, 0xE9);
                }
            }
        }
        {
            let mut x: i32 = engine.mem(0x8E);
            let mut do_d02e: i32 = 1;
            if cbool(x < 0x05) {
                let mut a: i32 = 0x00;
                let mut c: i32 = 1;
                let mut i: i32 = (x);
                loop {
                    let mut nc: i32 = u8v((a >> 7) & 1);
                    a = u8v((a << 1) | c);
                    c = nc;
                    {
                        i -= 1;
                        i
                    };
                    if !cbool(i >= 0) {
                        break;
                    }
                }
                a = u8v(a & engine.mem(u16v(p + 0x15)));
                if cbool(a != 0) {
                    do_d02e = 0;
                }
            }
            if cbool(do_d02e) {
                r.value = engine.mem(u16v(p + 0x0B));
                routine_0123(engine, r);
            }
        }
        engine.set_mem(0x80, engine.mem(u16v(p + 0x10)));
        engine.set_mem(0x81, engine.mem(u16v(p + 0x11)));
        engine.set_mem(0x82, engine.mem(u16v(p + 0x12)));
        engine.set_mem(0x83, engine.mem(u16v(p + 0x13)));
        engine.set_mem(0x41, engine.mem(u16v(p + 0x14)));
    }
}

mod vblank_commit {
    use super::*;
    pub fn vblank_commit(engine: &mut Engine, r: &mut RoutineContext) {
        let save = *r;
        {
            engine.ppu.set_vblank(cbool(1));
            engine
                .ppu
                .set_sprite0(cbool((if cbool(engine.mem(0x24) & 0x18) { 1 } else { 0 })));
            engine.ppu.eval_sprite_overflow();
        }
        {
            let __v = engine.device_read(0x2002);
            engine.set_mem(0x26, __v);
        }
        engine.device_write(0x2003, 0x00);
        engine.device_write(0x4014, 0x02);
        let mut req: i32 = engine.mem(0x28);
        if cbool(req == 0) {
            vblank_commit_tail(engine, r);
            {
                *r = save;
                return;
            }
        }
        engine.set_mem(0x28, 0x00);
        if cbool(req >= 0x07) {
            vblank_commit_tail(engine, r);
            {
                *r = save;
                return;
            }
        }
        {
            const jt_lo: [i32; 7] = [0x51, 0x52, 0x5F, 0x90, 0xE5, 0x34, 0x44];
            const jt_hi: [i32; 7] = [0xD3, 0xD2, 0xD2, 0xD2, 0xD2, 0xD3, 0xD3];
            engine.set_mem(0x06, jt_lo[req as usize]);
            engine.set_mem(0x07, jt_hi[req as usize]);
        }
        let _ = engine.device_read(0x2002);
        engine.device_write(0x2006, engine.mem(0x17));
        engine.device_write(0x2006, engine.mem(0x16));
        engine.device_write(0x2000, u8v(engine.mem(0x23) & 0x04));
        match req {
            1 => {
                vram_fill_run(engine, r);
            }
            2 => {
                vram_upload_palette(engine, r);
            }
            3 => {
                vram_upload_hud(engine, r);
            }
            4 => {
                vram_blit_stack(engine, r);
            }
            5 => {
                vram_copy_indirect(engine, r);
            }
            6 => {
                vram_poke2(engine, r);
            }
            _ => {}
        }
        *r = save;
    }
}

mod vblank_commit_tail {
    use super::*;
    pub fn vblank_commit_tail(engine: &mut Engine, r: &mut RoutineContext) {
        ppu_commit_banks(engine, r);
        statusbar_split(engine, r);
        if cbool(engine.mem(0x36) != 0) {
            engine.dec_mem(0x36);
        }
        frame_counters(engine, r);
        engine.device_write(0x8000, engine.mem(0x25));
    }
}

mod vram_blit_stack {
    use super::*;
    pub fn vram_blit_stack(engine: &mut Engine, r: &mut RoutineContext) {
        {
            let mut i: i32 = 0;
            while cbool(i < 0x40) {
                engine.device_write(0x2007, engine.mem(u16v(0x0100 + i)));
                {
                    let __old = i;
                    i += 1;
                    __old
                };
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_copy_indirect {
    use super::*;
    pub fn vram_copy_indirect(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x1A);
        let mut src: i32 = u16v(engine.mem(0x18) | (engine.mem(0x19) << 8));
        let mut y: i32 = 0;
        loop {
            engine.device_write(0x2007, engine.mem(u16v(src + y)));
            {
                let __old = y;
                y += 1;
                __old
            };
            if !cbool(
                {
                    x -= 1;
                    x
                } != 0,
            ) {
                break;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_fill_run {
    use super::*;
    pub fn vram_fill_run(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = engine.mem(0x1A);
        let mut a: i32 = engine.mem(0x18);
        loop {
            engine.device_write(0x2007, a);
            if !cbool(
                {
                    x -= 1;
                    x
                } != 0,
            ) {
                break;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_poke2 {
    use super::*;
    pub fn vram_poke2(engine: &mut Engine, r: &mut RoutineContext) {
        engine.device_write(0x2007, engine.mem(0x19));
        engine.device_write(0x2007, engine.mem(0x18));
        vblank_commit_tail(engine, r);
    }
}

mod vram_upload_hud {
    use super::*;
    pub fn vram_upload_hud(engine: &mut Engine, r: &mut RoutineContext) {
        let mut x: i32 = 0;
        engine.device_write(0x2000, u8v(engine.mem(0x23) | 0x04));
        {
            x = 0x17;
            while cbool(x >= 0) {
                engine.device_write(0x2007, engine.mem(u16v(0x0140 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        engine.device_write(0x2006, engine.mem(0x17));
        engine.device_write(0x2006, u8v(engine.mem(0x16) + 1));
        {
            x = 0x17;
            while cbool(x >= 0) {
                engine.device_write(0x2007, engine.mem(u16v(0x0158 + x)));
                {
                    let __old = x;
                    x -= 1;
                    __old
                };
            }
        }
        {
            x = 0x0A;
            while cbool(x >= 0) {
                engine.device_write(0x2006, engine.mem(0x19));
                engine.device_write(0x2006, engine.mem(u16v(0x0170 + x)));
                let _ = engine.device_read(0x2007);
                {
                    let mut v: i32 = u8v((engine.device_read(0x2007) & engine.mem(0x18))
                        | engine.mem(u16v(0x0171 + x)));
                    engine.device_write(0x2006, engine.mem(0x19));
                    engine.device_write(0x2006, engine.mem(u16v(0x0170 + x)));
                    engine.device_write(0x2007, v);
                }
                x -= 2;
            }
        }
        vblank_commit_tail(engine, r);
    }
}

mod vram_upload_palette {
    use super::*;
    pub fn vram_upload_palette(engine: &mut Engine, r: &mut RoutineContext) {
        let mut y: i32 = 0;
        engine.device_write(0x2006, 0x3F);
        engine.device_write(0x2006, 0x00);
        {
            y = 0;
            while cbool(y < 0x20) {
                engine.device_write(0x2007, engine.mem(u16v(0x0180 + y)));
                {
                    let __old = y;
                    y += 1;
                    __old
                };
            }
        }
        engine.device_write(0x2006, 0x3F);
        engine.device_write(0x2006, 0x00);
        engine.device_write(0x2006, 0x00);
        engine.device_write(0x2006, 0x00);
        vblank_commit_tail(engine, r);
    }
}
