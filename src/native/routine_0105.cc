#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

extern "C" void routine_0105(RoutineContext* r)
{
    lotw::native::GameState game;
    const auto buttons = lotw::native::run_frame_task(
        r,
        lotw::native::wait_any_button_pressed(),
        [r] { return lotw::native::redraw_scene_and_read_buttons(r); });
    r->value = buttons;
    game.set_buttons(buttons);
}
