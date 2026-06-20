#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0061(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0009_script(RoutineContext* r)
{
    lotw::native::GameState game;

    GAME_MEM8(0x56) = r->index;
    GAME_MEM8(0x57) = r->offset;
    game.set_frame_counter(0x08);
    routine_0061(r);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
}

}

extern "C" void routine_0050(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0009_script(r),
        [] { return std::uint8_t{0}; });
}
