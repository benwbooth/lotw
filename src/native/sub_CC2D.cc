#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

extern "C" void sub_CC2D(Regs* r)
{
    lotw::native::GameState game;
    const auto buttons = lotw::native::run_frame_task(
        r,
        lotw::native::wait_any_button_pressed(),
        [r] { return lotw::native::redraw_scene_and_read_buttons(r); });
    r->a = buttons;
    game.set_buttons(buttons);
}
