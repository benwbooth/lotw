#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_E620(Regs* r);
extern "C" void sub_E660(Regs* r);
extern "C" void sub_E6B7(Regs* r);
extern "C" void sub_CF30(Regs* r);
extern "C" void sub_CF82(Regs* r);
extern "C" void sub_C1C7(Regs* r);
extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C492(Regs* r);
extern "C" void sub_E642(Regs* r);
extern "C" void sub_C3E5(Regs* r);
extern "C" void sub_E79D(Regs* r);
extern "C" void sub_D02E(Regs* r);
extern "C" void sub_C8FF(Regs* r);
extern "C" void sub_C5CB(Regs* r);
extern "C" void sub_C2B1(Regs* r);

extern "C" void sub_E00F(Regs* r)
{
    lotw::native::GameState game;
    game.set_prompt_state(0x03);
    game.push_dialog_depth();

    if (RAM8(0x2D) < 0x30) {
        sub_E620(r);
        r->a = 0x08;
        sub_E660(r);
        sub_E6B7(r);
        sub_CF30(r);
        sub_CF82(r);
        RAM8(0x7B) = 0x08;
        sub_C1C7(r);
        sub_C1D8(r);
        sub_C492(r);
    }

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_release_then_button_then_release(0x10),
        [r] { return lotw::native::read_buttons(r); });

    game.set_prompt_state(0x04);

    if (RAM8(0x2D) < 0x30) {
        sub_E642(r);
        sub_C3E5(r);
        sub_E79D(r);
        r->a = RAM8(0xFE);
        sub_D02E(r);
        sub_C8FF(r);
        sub_C5CB(r);
        sub_C1D8(r);
        sub_C2B1(r);
        sub_C1C7(r);
        sub_C492(r);
    }

    game.pop_dialog_depth();
}
