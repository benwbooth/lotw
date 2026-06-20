#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

extern "C" void sub_CC17(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::redraw_scene_and_read_buttons(r); });
}
