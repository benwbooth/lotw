#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "game_memory.h"

extern "C" void routine_0029(RoutineContext* r)
{
    lotw::native::GameState game;
    lotw::native::run_frame_task(
        r,
        lotw::native::ae11_press_start_gate(game),
        [r] { return lotw::native::read_buttons(r); });
}
