#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "ram.h"

extern "C" void sub_AE11(Regs* r)
{
    lotw::native::GameState game;
    lotw::native::run_frame_task(
        r,
        lotw::native::ae11_press_start_gate(game),
        [r] { return lotw::native::read_buttons(r); });
}
