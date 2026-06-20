#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0074(RoutineContext* r);
extern "C" void routine_0115(RoutineContext* r);
extern "C" void routine_0241(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0026_script(RoutineContext* r)
{
    lotw::native::GameState game;

    if ((GAME_MEM8(0xEE) & 0x7F) == 0) {
        game.set_prompt_state(0x18);
        game.set_prompt_argument(0xFF);
        r->index = 0x03;
        routine_0074(r);

        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->index = 0x03;
        routine_0074(r);

        game.set_frame_counter(0x05);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->index = 0x03;
        routine_0074(r);

        GAME_MEM8(0xEE) = (u8)(GAME_MEM8(0xEE) + 1);
        game.set_prompt_state(0x02);
        GAME_MEM8(0xF1) = 0x0F;
        GAME_MEM8(0xF5) = 0x00;
        GAME_MEM8(0xF6) = 0x00;
        GAME_MEM8(0xF0) = 0x00;
        GAME_MEM8(0xFC) = GAME_MEM8(0xFB);
    }

    if (GAME_MEM8(0xF0) == 0) {
        GAME_MEM8(0xF1) = (u8)(GAME_MEM8(0xF1) - 1);
        if (GAME_MEM8(0xF1) == 0) {
            GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) | 0x80);
            GAME_MEM8(0xF0) = 0x01;
            co_return;
        }
        {
            u8 a = (u8)(GAME_MEM8(0xF1) >> 2);
            a = (u8)((a ^ 0xFF) + 1);
            GAME_MEM8(0xF7) = a;
        }
        routine_0241(r);
        routine_0115(r);
        if (r->carry) {
            GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) | 0x80);
            GAME_MEM8(0xF0) = 0x01;
            co_return;
        }
        GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
        co_return;
    }

    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
    GAME_MEM8(0xF7) = (u8)((GAME_MEM8(0xF0) >> 2) + 1);
    routine_0241(r);
    routine_0115(r);
    if (r->carry) {
        GAME_MEM8(0xEE) = 0x00;
        GAME_MEM8(0xF3) = 0xF0;
        GAME_MEM8(0xEB) = 0x01;
        co_return;
    }
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
}

}

extern "C" void routine_0259(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0026_script(r),
        [] { return std::uint8_t{0}; });
}
