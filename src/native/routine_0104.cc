#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

extern "C" void routine_0104(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::redraw_scene_and_read_buttons(r); });
}
