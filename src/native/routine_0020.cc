#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0003_script(RoutineContext* r)
{
    lotw::native::GameState game;

    routine_0061(r);
    routine_0063(r);
    game.set_frame_counter(0x01);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
}

}

extern "C" void routine_0020(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0003_script(r),
        [] { return std::uint8_t{0}; });
}
