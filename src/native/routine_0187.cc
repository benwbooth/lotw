#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0192(RoutineContext* r);
extern "C" void routine_0193(RoutineContext* r);
extern "C" void routine_0195(RoutineContext* r);
extern "C" void routine_0198(RoutineContext* r);
extern "C" void routine_0120(RoutineContext* r);
extern "C" void routine_0199(RoutineContext* r);
extern "C" void routine_0070(RoutineContext* r);
extern "C" void routine_0189(RoutineContext* r);
extern "C" void routine_0208(RoutineContext* r);

static void wait_release(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::read_buttons(r); });
}

extern "C" void routine_0187(RoutineContext* r)
{
    lotw::native::GameState game;

    routine_0193(r);

    {
        const u8 s80 = GAME_MEM8(0x80);
        const u8 s81 = GAME_MEM8(0x81);
        const u8 s82 = GAME_MEM8(0x82);
        const u8 s83 = GAME_MEM8(0x83);
        r->value = GAME_MEM8(0x47);
        routine_0195(r);
        GAME_MEM8(0x83) = s83;
        GAME_MEM8(0x82) = s82;
        GAME_MEM8(0x81) = s81;
        GAME_MEM8(0x80) = s80;
    }

    routine_0198(r);
    routine_0120(r);
    routine_0199(r);
    routine_0070(r);

    for (;;) {
        routine_0189(r);
        if (r->carry) {
            routine_0192(r);
            return;
        }

        const u8 nib = GAME_MEM8(0x44) & 0x0F;
        u8 x;
        if (nib < 0x03) {
            continue;
        }
        if (nib < 0x05) {
            x = 0x00;
        } else {
            x = 0x02;
            if (nib < 0x0A || nib >= 0x0C)
                continue;
        }

        const u8 item = GAME_MEM8((u16)(0x80 + x));
        if (item & 0x80) {
            game.set_prompt_state(0x06);
        } else {
            const u8 price = GAME_MEM8((u16)(0x81 + x));
            r->value = price;
            routine_0208(r);
            if (r->carry) {
                GAME_MEM8((u16)(0x80 + x)) = 0xFF;
                routine_0198(r);
                GAME_MEM8((u16)(0x60 + item))++;
                game.set_prompt_state(0x10);
            } else {
                if (item == 0x0D && GAME_MEM8(0x37) != 0)
                    GAME_MEM8(0x61) = 0x01;
                game.set_prompt_state(0x06);
            }
        }

        wait_release(r);
    }
}
