#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0073(RoutineContext* r);
extern "C" void routine_0075(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0015_script(RoutineContext* r)
{
    lotw::native::GameState game;
    const u16 ptr = (u16)(GAME_MEM8(0x77) | (GAME_MEM8(0x78) << 8));

    u8 v = 0x40;
    GAME_MEM8(0x09) = v;
    do {
        game.set_frame_counter(0x05);
        for (u8 y = 0xE0; y < 0xE4; ++y)
            GAME_MEM8((u16)(0x00A0 + y)) = GAME_MEM8((u16)(ptr + y));
        r->index = 0x00;
        r->offset = 0x04;
        routine_0073(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        v = (u8)(GAME_MEM8(0x09) - 0x10);
        GAME_MEM8(0x09) = v;
    } while ((v & 0x80) == 0);
    routine_0075(r);
}

}

extern "C" void routine_0071(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0015_script(r),
        [] { return std::uint8_t{0}; });
}
