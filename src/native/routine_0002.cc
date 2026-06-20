









































#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" {

void routine_0213(RoutineContext *r);  void routine_0214(RoutineContext *r);  void routine_0099(RoutineContext *r);
void routine_0100(RoutineContext *r);  void routine_0003(RoutineContext *r);  void routine_0012(RoutineContext *r);
void routine_0014(RoutineContext *r);  void routine_0026(RoutineContext *r);  void routine_0024(RoutineContext *r);
void routine_0030(RoutineContext *r);  void routine_0019(RoutineContext *r);  void routine_0020(RoutineContext *r);
void routine_0045(RoutineContext *r);  void routine_0069(RoutineContext *r);  void routine_0066(RoutineContext *r);
void routine_0065(RoutineContext *r);  void routine_0076(RoutineContext *r);  void routine_0093(RoutineContext *r);
void routine_0094(RoutineContext *r);  void routine_0096(RoutineContext *r);  void routine_0095(RoutineContext *r);
void routine_0060(RoutineContext *r);  void routine_0127(RoutineContext *r);  void routine_0061(RoutineContext *r);
void routine_0062(RoutineContext *r);  void routine_0063(RoutineContext *r);  void routine_0128(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);


void scene_assemble(RoutineContext *r);
void game_update(RoutineContext *r);
void routine_0077(RoutineContext *r);
void routine_0070(RoutineContext *r);
void routine_0059(RoutineContext *r);


void routine_0039(RoutineContext *r);

}



static void farcall_cce4(RoutineContext *r, u8 lo, u8 hi, RoutineFn target)
{
    GAME_MEM8(0x0E) = lo; GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = GAME_MEM8(0x32); GAME_MEM8(0x31) = GAME_MEM8(0x33); GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
}



static lotw::native::FrameTask cutscene_a7ff(RoutineContext *r)
{
    lotw::native::GameState game;

    routine_0099(r);
    GAME_MEM8(0x0411) = 0x00;
    GAME_MEM8(0x0421) = 0x00;
    GAME_MEM8(0x0431) = 0x00;
    GAME_MEM8(0x00F2) = 0x00;
    game.set_sprite_blink_timer(0x00);
    GAME_MEM8(0x88) = 0x00;
    routine_0026(r);
    routine_0012(r);
    GAME_MEM8(0x0200) = 0xEF;

    for (;;) {
        if (GAME_MEM8(0x45) >= 0xA0) break;
        GAME_MEM8(0x45)++;
        routine_0026(r);
        game.set_frame_counter(0x01);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

    }


    GAME_MEM8(0x4E) = 0x00;
    GAME_MEM8(0x4F) = 0x00;
    routine_0024(r);
    routine_0026(r);
    GAME_MEM8(0x7C) = 0x20;
    GAME_MEM8(0x1D) = 0x01;
    game.set_prompt_state(0x20);
    game.set_prompt_argument(0x80);
    GAME_MEM8(0x7A) = 0xB6;

    do { routine_0003(r); } while (GAME_MEM8(0xFA) != 0);
    do { routine_0003(r); } while (GAME_MEM8(0xFA) != 0);

    game.set_prompt_state(0x20);
    game.set_prompt_argument(0x80);
    GAME_MEM8(0x7A) = 0xB7;
    do { routine_0003(r); } while (GAME_MEM8(0xFA) != 0);
    do { routine_0003(r); } while (GAME_MEM8(0xFA) != 0);

    GAME_MEM8(0x10) = 0x00;
    do {
        if ((GAME_MEM8(0x84) & 0x07) == 0) {
            GAME_MEM8(0x1D) ^= 0x01;
            game.set_prompt_state(0x20);
            game.set_prompt_argument(0x80);
        }

        r->value = 0xFF; queue_ppu_job_and_wait(r);
        if (game.frame_status_bit6_set()) {
            r->value = 0x05; routine_0030(r);
            routine_0100(r);
        }

        if (GAME_MEM8(0x3E) == 0) GAME_MEM8(0x3E) = 0x02;

        routine_0026(r);
        routine_0014(r);
        GAME_MEM8(0x10)--;
    } while (GAME_MEM8(0x10) != 0);

    GAME_MEM8(0x1D) = 0x01;
    r->value = 0xFF; queue_ppu_job_and_wait(r);
    if (game.player_health() == 0) co_return;


    GAME_MEM8(0x0200) = 0xEF;
    game.set_prompt_state(0x18);
    game.set_prompt_argument(0xFF);
    GAME_MEM8(0x08) = 0x01;
    for (;;) {
        u8 prev = GAME_MEM8(0x45);
        u8 ny = (u8)(prev - GAME_MEM8(0x08));
        GAME_MEM8(0x45) = ny;

        {
            int c = (prev >= GAME_MEM8(0x08)) ? 1 : 0;
            u8 t = (u8)(ny + 0x2B + c);
            if (t >= 0xEF) break;
        }
        routine_0026(r);
        GAME_MEM8(0x08)++;
        r->value = 0xFF; queue_ppu_job_and_wait(r);

    }


    GAME_MEM8(0x0210) = 0xEF;
    GAME_MEM8(0x0214) = 0xEF;
    GAME_MEM8(0x3E) = 0x00;
    GAME_MEM8(0x3F) = 0x80;
    routine_0128(r);
    routine_0045(r);
    routine_0069(r);
    routine_0066(r);
    routine_0065(r);
    GAME_MEM8(0x48) = 0x10;
    GAME_MEM8(0x47) = 0x03;
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);
    GAME_MEM8(0x7C) = 0x12;
    GAME_MEM8(0x45) = 0xC0;
    GAME_MEM8(0x44) = 0x1A;
    GAME_MEM8(0x43) = 0x01;
    GAME_MEM8(0x7B) = 0x01;
    GAME_MEM8(0x56) = 0x09;
    GAME_MEM8(0x2C) = 0x35;
    GAME_MEM8(0x2D) = 0x34;
    GAME_MEM8(0x2E) = 0x36;
    GAME_MEM8(0x2F) = 0x37;
    GAME_MEM8(0x0411) = 0x01;
    GAME_MEM8(0x0421) = 0x01;
    GAME_MEM8(0x0431) = 0x01;
    GAME_MEM8(0x0441) = 0x01;
    GAME_MEM8(0x041E) = 0xA0;
    GAME_MEM8(0x042E) = 0xA0;
    GAME_MEM8(0x043E) = 0xA0;
    GAME_MEM8(0x044E) = 0x70;
    GAME_MEM8(0x044D) = 0x33;
    routine_0019(r);
    {
        u8 v = 0x2D;
        GAME_MEM8(0x0410) = v;
        v = (u8)(v + 0x20); GAME_MEM8(0x0420) = v;
        v = (u8)(v + 0x20); GAME_MEM8(0x0430) = v;
    }
    GAME_MEM8(0x0440) = 0x81;
    GAME_MEM8(0x0412) = 0x40;
    GAME_MEM8(0x0422) = 0x40;
    GAME_MEM8(0x0432) = 0x40;
    GAME_MEM8(0x0442) = 0x40;
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
    routine_0063(r);
    GAME_MEM8(0x40) = 0x07;
    farcall_cce4(r, 0x92, 0xC4, routine_0070);
    game.set_countdown_timer(0x05);
    do { routine_0020(r); } while (game.countdown_timer_active());

    for (;;) {
        if (GAME_MEM8(0x45) == 0xA0) break;
        GAME_MEM8(0x45)--;
        routine_0020(r);
        routine_0020(r);
        if (GAME_MEM8(0x45) == 0xA0) break;
        GAME_MEM8(0x45)--;
        GAME_MEM8(0x57) ^= 0x40;
        routine_0061(r);
        routine_0020(r);
        routine_0020(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

    }


    GAME_MEM8(0x56) = 0x0D;
    routine_0061(r);
    game.set_countdown_timer(0x03);
    do { routine_0020(r); } while (game.countdown_timer_active());

    for (;;) {
        game.set_frame_counter(0x01);
        GAME_MEM8(0x7E) = GAME_MEM8(0x7C);
        game.set_buttons(0x01);
        farcall_cce4(r, 0x2B, 0xD4, game_update);
        farcall_cce4(r, 0x5D, 0xC1, routine_0059);
        routine_0019(r);
        routine_0061(r);
        routine_0063(r);
        if (GAME_MEM8(0x7E) != GAME_MEM8(0x7C)) GAME_MEM8(0x3D)++;

        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        if (GAME_MEM8(0x44) != 0x37) continue;
        break;
    }


    GAME_MEM8(0x56) = 0x19;
    GAME_MEM8(0x0410) = 0x39;
    GAME_MEM8(0x0420) = 0x59;
    GAME_MEM8(0x0430) = 0x79;
    GAME_MEM8(0x0440) = 0x91;
    game.set_countdown_timer(0x14);
    do {
        GAME_MEM8(0x56)   ^= 0x04;
        GAME_MEM8(0x0410) ^= 0x04;
        GAME_MEM8(0x0420) ^= 0x04;
        GAME_MEM8(0x0430) ^= 0x04;
        GAME_MEM8(0x0440) ^= 0x04;
        routine_0020(r); routine_0020(r); routine_0020(r); routine_0020(r);
        routine_0020(r); routine_0020(r); routine_0020(r); routine_0020(r);
    } while (game.countdown_timer_active());

    routine_0039(r);
}

extern "C" void routine_0002(RoutineContext *r)
{
    lotw::native::GameState game;


    GAME_MEM8(0xE5) = 0x00;
    GAME_MEM8(0xE6) = 0x04;
    routine_0213(r);
    if (GAME_MEM8(0x00F2) == 0) {
        lotw::native::run_frame_task(
            r,
            cutscene_a7ff(r),
            [] { return std::uint8_t{0}; });
        return;
    }


    if (game.frame_status_bit6_set()) {
        u8 t = (u8)(GAME_MEM8(0x3E) + 2);
        t &= 0x06;
        if (t != 0) {
            u8 x = (u8)(t << 3);
            if (GAME_MEM8((u16)(0x0401 + x)) != 0) {
                u8 sum;
                GAME_MEM8((u16)(0x0401 + x)) = 0x00;
                sum = (u8)(GAME_MEM8(0x1C) + GAME_MEM8((u16)(0x040C + x)));
                if (sum >= 0xB0 && sum < 0xD0) {

                    u8 bl = GAME_MEM8(0x00F2);
                    if ((u8)(bl - 0x02) > bl)
                        bl = 0x00;
                    else
                        bl = (u8)(bl - 0x02);
                    GAME_MEM8(0x00F2) = bl;
                    routine_0099(r);
                    game.set_prompt_state(0x20);
                    game.set_prompt_argument(0x01);
                } else {
                    game.set_prompt_state(0x01);
                }
            }
        }
    }


    if (GAME_MEM8(0xFA) != 0) goto draw;

    switch (GAME_MEM8(0xF3)) {
    case 4:  goto phase4;
    case 3:  goto phase_open;
    case 2:  goto phase_close;
    case 1:  goto phase_grow;
    default: break;
    }


    {
        u8 sum = (u8)(GAME_MEM8(0x1C) + GAME_MEM8(0x43));
        int carry = (sum < GAME_MEM8(0x1C));
        if (carry || sum >= 0xC0) goto trig_close;
        if (GAME_MEM8(0x1C) >= 0x40) goto trig_close;
        if (sum >= 0xA0) goto trig_l47b;
        if (sum >= 0x80) goto trig_grow;

    }
trig_l47b:
    if (GAME_MEM8(0x1E) >= 0xC3) goto trig_close;
    GAME_MEM8(0xF3) = 0x01;
    GAME_MEM8(0xE9) = 0x04;
    goto phase_grow;
trig_close:
    GAME_MEM8(0xF3) = 0x03;
    GAME_MEM8(0xE9) = 0x02;
    goto phase_open;
trig_grow:
    GAME_MEM8(0xF3) = 0x02;
    GAME_MEM8(0xE9) = 0x08;
    GAME_MEM8(0x7A) = 0xB3;
    goto draw;

phase_grow:
    GAME_MEM8(0xE9)--;
    if (GAME_MEM8(0xE9) == 0) goto grow_done;
    {
        u8 a = GAME_MEM8(0xE9);
        a = (u8)(a << 1) & 0x01;
        a = (u8)(a + 0xA0 + 0x10);
        GAME_MEM8(0x7A) = a;
    }
    GAME_MEM8(0x1C) = (u8)(GAME_MEM8(0x1C) + 0x04);
    if (GAME_MEM8(0x1C) >= 0x40) goto grow_done;
    GAME_MEM8(0x1E) = 0xC2;
    goto draw;
grow_done:
    GAME_MEM8(0xF3) = 0x00;
    goto draw;

phase_close:
    GAME_MEM8(0xE9)--;
    if (GAME_MEM8(0xE9) == 0) goto close_done;
    GAME_MEM8(0x7A) = 0xB4;
    if (GAME_MEM8(0x1E) >= 0xC3) {
        GAME_MEM8(0x1E) = (u8)(GAME_MEM8(0x1E) - 0x04);
    }
    goto draw;
close_done:
    GAME_MEM8(0x7A) = 0xB3;
    GAME_MEM8(0xF3) = 0x00;
    goto draw;

phase_open:
    GAME_MEM8(0xE9)--;
    if (GAME_MEM8(0xE9) == 0) goto open_done;
    GAME_MEM8(0x7A) = 0xB2;
    if (GAME_MEM8(0x1C) != 0) {
        u8 v = GAME_MEM8(0x1C);
        if ((u8)(v - 0x04) > v) v = 0x00;
        else v = (u8)(v - 0x04);
        GAME_MEM8(0x1C) = v;
        if (v >= 0x11) goto open_shrink;
    }

    if (GAME_MEM8(0x1E) < 0xC3) goto draw;
    GAME_MEM8(0x1E) = (u8)(GAME_MEM8(0x1E) - 0x04);
    goto draw;
open_shrink:
    if (GAME_MEM8(0x1E) < 0xD2) {
        GAME_MEM8(0x1E) = (u8)(GAME_MEM8(0x1E) + 0x04);
        goto draw;
    }
    if (GAME_MEM8(0x1C) == 0) goto draw;
    GAME_MEM8(0x1C) = (u8)(GAME_MEM8(0x1C) - 0x04);
    goto draw;
open_done:
    if (GAME_MEM8(0x1C) != 0) {
        GAME_MEM8(0xF3) = 0x00;
        goto draw;
    }

    GAME_MEM8(0x7A) = 0xB0;
    GAME_MEM8(0xF3)++;
    GAME_MEM8(0xE9) = 0x04;
    goto draw;

phase4:
    GAME_MEM8(0xE9)--;
    if (GAME_MEM8(0xE9) == 0) goto phase4_done;
    if (GAME_MEM8(0xE9) == 0x04) game.set_prompt_state(0x20);

    GAME_MEM8(0x7A) = 0xB5;
    GAME_MEM8(0x1E) = 0xC2;
    goto draw;
phase4_done:
    GAME_MEM8(0x7A) = 0xB3;
    GAME_MEM8(0xF3) = 0x00;


draw:
    routine_0003(r);
    routine_0214(r);
    return;
}
