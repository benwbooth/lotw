#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0082(RoutineContext* r);
extern "C" void routine_0092(RoutineContext* r);
extern "C" void routine_0075(RoutineContext* r);

namespace lotw::native {

void commit_frame_work(RoutineContext* r)
{
    GameState game;

    if (GAME_MEM8(0x3D) != 0) {
        GAME_MEM8(0x3D) = 0;
        routine_0082(r);
    } else if (GAME_MEM8(0x3C) != 0) {
        GAME_MEM8(0x3C) = 0;
        routine_0092(r);
    } else if (game.frame_counter_active()) {
        routine_0075(r);
    }
}

}

extern "C" void routine_0058(RoutineContext* r)
{
    lotw::native::commit_frame_work(r);
    lotw::native::wait_for_frame_counter(r);
}
