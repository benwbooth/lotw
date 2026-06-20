#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0063(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0007_script(RoutineContext* r)
{
    lotw::native::GameState game;

    GAME_MEM8(0xB4) = 0;
    GAME_MEM8(0x0D) = 0x10;
    do {
        if (GAME_MEM8(0xA0) != 0) GAME_MEM8(0xA0) = (u8)(GAME_MEM8(0xA0) - 1);
        if (GAME_MEM8(0xB0) != 0) GAME_MEM8(0xB0) = (u8)(GAME_MEM8(0xB0) - 1);
        if (GAME_MEM8(0xD0) != 0) GAME_MEM8(0xD0) = (u8)(GAME_MEM8(0xD0) - 1);
        GAME_MEM8(0x0C) = 0x14;
        do {
            routine_0063(r);
            game.set_frame_counter(0x01);
            lotw::native::commit_frame_work(r);
            while (game.frame_counter_active())
                co_yield lotw::native::Wait::next_frame();
            GAME_MEM8(0x0C) = (u8)(GAME_MEM8(0x0C) - 1);
        } while (GAME_MEM8(0x0C) != 0);
        GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) - 1);
    } while (GAME_MEM8(0x0D) != 0);
}

}

extern "C" void routine_0045(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0007_script(r),
        [] { return std::uint8_t{0}; });
}
