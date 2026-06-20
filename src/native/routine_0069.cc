#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

namespace {

lotw::native::FrameTask frame_task_0013_script(RoutineContext* r)
{
    lotw::native::GameState game;

    u8 y = 0x04;
    do {
        game.set_frame_counter(0x05);
        for (int x = 0x20; x >= 0; --x) {
            const u8 v = GAME_MEM8((u16)(0x0180 + x));
            const u8 lo = (u8)(v & 0x0F);
            const u8 hi = (u8)(v & 0xF0);
            GAME_MEM8(0x08) = lo;
            GAME_MEM8((u16)(0x0180 + x)) = (hi >= 0x10) ? (u8)((hi - 0x10) | lo) : 0x0F;
        }
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    } while (--y != 0);
}

}

extern "C" void routine_0069(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0013_script(r),
        [] { return std::uint8_t{0}; });
}
