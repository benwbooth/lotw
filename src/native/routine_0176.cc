#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0081(RoutineContext* r);
extern "C" void routine_0060(RoutineContext* r);
extern "C" void routine_0051(RoutineContext* r);
extern "C" void routine_0131(RoutineContext* r);

extern "C" void routine_0176(RoutineContext* r)
{
    GAME_MEM8(0x7C) = 0x10;
    routine_0081(r);
    routine_0060(r);

    GAME_MEM8(0x0E) = 0xD4;
    GAME_MEM8(0x0F) = 0xB4;
    routine_0051(r);
    routine_0131(r);

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_release_then_any_press(),
        [r] { return lotw::native::read_buttons(r); });

    GAME_MEM8(0x7C) = 0x20;
    routine_0081(r);
    routine_0060(r);
}
