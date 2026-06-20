























#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" {

void routine_0053(RoutineContext *r); void routine_0128(RoutineContext *r); void routine_0065(RoutineContext *r);
void routine_0036(RoutineContext *r); void routine_0054(RoutineContext *r); void routine_0055(RoutineContext *r);
void routine_0069(RoutineContext *r); void routine_0037(RoutineContext *r); void routine_0090(RoutineContext *r);
void routine_0076(RoutineContext *r); void routine_0093(RoutineContext *r); void routine_0094(RoutineContext *r);
void routine_0096(RoutineContext *r); void routine_0095(RoutineContext *r); void routine_0060(RoutineContext *r);
void routine_0127(RoutineContext *r); void routine_0061(RoutineContext *r); void routine_0062(RoutineContext *r);
void routine_0063(RoutineContext *r); void routine_0038(RoutineContext *r); void routine_0035(RoutineContext *r);
void routine_0075(RoutineContext *r); void routine_0066(RoutineContext *r);
void routine_0077(RoutineContext *r); void routine_0070(RoutineContext *r); void routine_0266(RoutineContext *r);
void routine_0212(RoutineContext *r); void routine_0271(RoutineContext *r); void routine_0059(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void game_update(RoutineContext *r);
void rng_update(RoutineContext *r);
void song_init(RoutineContext *r);


void routine_0034(RoutineContext *r);
void routine_0039(RoutineContext *r);
}



static void farcall_cce4(RoutineContext *r, u8 lo, u8 hi, RoutineFn target)
{
    GAME_MEM8(0x0E) = lo; GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = GAME_MEM8(0x32); GAME_MEM8(0x31) = GAME_MEM8(0x33); GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
}

static lotw::native::FrameTask frame_task_0004_script(RoutineContext *r)
{
    lotw::native::GameState game;

restart:
    routine_0053(r);
    GAME_MEM8(0x2C) = 0x37;
    GAME_MEM8(0x29) = 0x00;
    GAME_MEM8(0x23) = 0xA0;
    REG_W(0x2000, 0xA0);
    GAME_MEM8(0x24) = 0x00;
    REG_W(0x2001, 0x00);
    GAME_MEM8(0x1C) = 0x00;
    GAME_MEM8(0x1D) = 0x00;
    GAME_MEM8(0x1E) = 0xE8;
    {
        int x;
        for (x = 0x1F; x >= 0; x--) GAME_MEM8((u16)(0x0180 + x)) = 0x0F;
    }
    farcall_cce4(r, 0x69, 0xC5, routine_0075);
    routine_0128(r);
    routine_0065(r);
    routine_0036(r);
    GAME_MEM8(0x2C) = 0x15;
    GAME_MEM8(0x8E) = 0x09;
    song_init(r);
    routine_0054(r);
    GAME_MEM8(0x24) = 0x1E;
    REG_W(0x2001, 0x1E);
    game.set_frame_counter(0x78);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
    routine_0055(r);
    game.set_countdown_timer(0x14);


    for (;;) {
        u8 pad;
        game.set_frame_counter(0x01);
        pad = lotw::native::read_buttons(r);
        if (pad == 0xFF) {
            game.set_prompt_state(0x1A);
            GAME_MEM8(0x37) = 0x1A;
        }
        (void)pad;
        if (game.buttons() & 0x10) {
            routine_0034(r);
            co_return;
        }
        if (game.button_chord() == 0x83) {
            routine_0039(r);
            co_return;
        }
        if ((GAME_MEM8(0x84) & 0x07) == 0) {
            u8 lo = GAME_MEM8(0x0182) & 0x0F;
            u8 hi = GAME_MEM8(0x0182) & 0xF0;
            GAME_MEM8(0x08) = lo;
            if ((u8)(hi - 0x10) > hi) hi = 0x30;
            else hi = (u8)(hi - 0x10);
            GAME_MEM8(0x0193) = hi;
            GAME_MEM8(0x0182) = (u8)(hi | GAME_MEM8(0x08));
        }
        GAME_MEM8(0x0E) = 0x35; GAME_MEM8(0x0F) = 0xC1;
        GAME_MEM8(0x30) = GAME_MEM8(0x32); GAME_MEM8(0x31) = GAME_MEM8(0x33); GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
        if (game.countdown_timer_active()) continue;
        break;
    }


    routine_0069(r);
    routine_0065(r);
    routine_0128(r);
    routine_0037(r);
    r->value = 0x04; rng_update(r); GAME_MEM8(0x47) = r->value;
    r->value = 0x10; rng_update(r); GAME_MEM8(0x48) = r->value;
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);


    for (;;) {
        u8 t;
        r->value = 0x40; rng_update(r);
        GAME_MEM8(0x44) = r->value;
        GAME_MEM8(0x0C) = r->value;
        GAME_MEM8(0x43) = 0x00;
        r->value = 0x0B; rng_update(r);
        r->value = (u8)(r->value << 4);
        GAME_MEM8(0x45) = r->value;
        GAME_MEM8(0x0D) = r->value;
        routine_0090(r);
        {
            u16 p = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
            t = GAME_MEM8(p) & 0x3F;
            if (t >= 0x30) continue;
            if (t == 0x02) continue;
            if (t == GAME_MEM8(0x70)) continue;
            t = GAME_MEM8((u16)(p + 1)) & 0x3F;
            if (t < 0x30) continue;
            if (t == 0x30) continue;
        }
        break;
    }
    {
        u8 x = GAME_MEM8(0x44);
        if ((u8)(x - 0x08) > x) x = 0x00;
        else x = (u8)(x - 0x08);
        if (x >= 0x30) x = 0x30;
        GAME_MEM8(0x7C) = x;
        GAME_MEM8(0x7B) = 0x00;
    }


    {
        u8  chr;
        for (;;) {
            u8 mask, a, c; int y;
            r->value = 0x05; rng_update(r);
            chr = r->value;
            a = 0x00; c = 1;
            for (y = chr; y >= 0; y--) {
                u8 nc = (a >> 7) & 1;
                a = (u8)((a << 1) | c);
                c = nc;
            }
            mask = a;
            if ((mask & GAME_MEM8(0x41)) != 0) break;
        }
        GAME_MEM8(0x51) = GAME_MEM8((u16)(0xB0AC + chr));
        GAME_MEM8(0x55) = 0x00;
        GAME_MEM8(0x40) = chr;
        {
            int i;
            u16 y = (u16)(0xFFA7 + ((chr << 2) + 0x03));
            for (i = 3; i >= 0; i--) {
                GAME_MEM8((u16)(0x5C + i)) = GAME_MEM8(y);
                y--;
            }
        }
        GAME_MEM8(0x2C) = (u8)(GAME_MEM8(0x40) + 0x38);
    }
    GAME_MEM8(0x2E) = 0x3E;
    GAME_MEM8(0x2F) = 0x20;
    GAME_MEM8(0x56) = 0x0D;
    GAME_MEM8(0x57) = 0x00;
    GAME_MEM8(0x42) = 0x01;
    game.set_player_health(0x64);
    game.set_player_magic(0x64);
    farcall_cce4(r, 0x8B, 0xC3, routine_0066);
    routine_0076(r);
    farcall_cce4(r, 0xCB, 0xC5, routine_0077);
    routine_0093(r);
    routine_0094(r);
    routine_0096(r);
    routine_0095(r);
    routine_0060(r);
    routine_0127(r);
    routine_0061(r);
    routine_0062(r);
    farcall_cce4(r, 0x92, 0xC4, routine_0070);
    game.set_countdown_timer(0x0A);


    for (;;) {
        game.set_frame_counter(0x01);
        GAME_MEM8(0x7E) = GAME_MEM8(0x7C);
        routine_0038(r);
        lotw::native::read_buttons(r);
        if (game.buttons() & 0x10) {
            routine_0034(r);
            co_return;
        }

        game.set_buttons(GAME_MEM8(0xFE));
        {
            int do_b044 = 1;
            if ((GAME_MEM8(0x49) | GAME_MEM8(0x4B)) != 0) {
                GAME_MEM8(0x42) = (u8)(GAME_MEM8(0x42) - 1);
                if (GAME_MEM8(0x42) != 0) do_b044 = 0;
            }
            if (do_b044) {
                GAME_MEM8(0x42) = 0x80;
                routine_0035(r);
                GAME_MEM8(0xFE) = game.buttons();
            }
        }

        farcall_cce4(r, 0x2B, 0xD4, game_update);
        farcall_cce4(r, 0x28, 0xF6, routine_0266);
        farcall_cce4(r, 0x7C, 0xE8, routine_0212);
        farcall_cce4(r, 0x82, 0xF7, routine_0271);
        farcall_cce4(r, 0x5D, 0xC1, routine_0059);
        routine_0061(r);
        routine_0063(r);
        if (GAME_MEM8(0x7E) != GAME_MEM8(0x7C)) GAME_MEM8(0x3D)++;
        GAME_MEM8(0x0E) = 0x35; GAME_MEM8(0x0F) = 0xC1;
        GAME_MEM8(0x30) = GAME_MEM8(0x32); GAME_MEM8(0x31) = GAME_MEM8(0x33); GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
        if (game.countdown_timer_active()) continue;
        break;
    }

    routine_0069(r);
    goto restart;
}

extern "C" void routine_0033(RoutineContext *r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0004_script(r),
        [] { return std::uint8_t{0}; });
}
