#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0075(RoutineContext* r);
extern "C" void routine_0087(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0017_script(RoutineContext* r)
{
    lotw::native::GameState game;
    u8 x = r->index;

    do {
        for (int i = 0x1F; i >= 0; --i)
            GAME_MEM8((u16)(0x0180 + i)) = 0x30;
        routine_0075(r);
        game.set_frame_counter(0x01);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        routine_0087(r);
        routine_0075(r);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    } while (--x != 0);
    r->index = x;
}

}

extern "C" void routine_0074(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0017_script(r),
        [] { return std::uint8_t{0}; });
}
