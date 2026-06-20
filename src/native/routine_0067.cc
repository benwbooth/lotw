#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

namespace {

lotw::native::FrameTask frame_task_0011_script(RoutineContext* r)
{
    lotw::native::GameState game;

    GAME_MEM8(0x92) = (u8)(GAME_MEM8(0x92) + 1);
    u8 y = 0x04;
    do {
        game.set_frame_counter(0x05);
        for (int x = 0x1C; x >= 0; --x) {
            const u8 v = GAME_MEM8((u16)(0x0184 + x));
            const u8 lo = (u8)(v & 0x0F);
            const u8 hi = (u8)(v & 0xF0);
            GAME_MEM8(0x08) = lo;
            GAME_MEM8((u16)(0x0184 + x)) = (hi >= 0x10) ? (u8)((hi - 0x10) | lo) : 0x0F;
        }
        GAME_MEM8(0xA0) >>= 1;
        GAME_MEM8(0xB0) >>= 1;
        GAME_MEM8(0xD0) >>= 1;
        GAME_MEM8(0xB4) = 0;
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    } while (--y != 0);

    GAME_MEM8(0x8E) = 0xFF;
    GAME_MEM8(0x94) = 0;
    GAME_MEM8(0xA4) = 0;
    GAME_MEM8(0xC4) = 0;
    GAME_MEM8(0x92) = 0;
}

}

extern "C" void routine_0067(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0011_script(r),
        [] { return std::uint8_t{0}; });
}
