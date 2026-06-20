#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0087(RoutineContext* r);
extern "C" void routine_0073(RoutineContext* r);
extern "C" void routine_0075(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0014_script(RoutineContext* r)
{
    lotw::native::GameState game;

    u8 v = 0x40;
    GAME_MEM8(0x09) = v;
    do {
        game.set_frame_counter(0x05);
        routine_0087(r);
        r->index = 0x04;
        r->offset = 0x1C;
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

extern "C" void routine_0070(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0014_script(r),
        [] { return std::uint8_t{0}; });
}
