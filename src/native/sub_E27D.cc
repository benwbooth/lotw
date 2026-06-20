#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C7B5(Regs* r);
extern "C" void sub_C1C7(Regs* r);
extern "C" void sub_B4D4(Regs* r);
extern "C" void sub_D0E5(Regs* r);

extern "C" void sub_E27D(Regs* r)
{
    RAM8(0x7C) = 0x10;
    sub_C7B5(r);
    sub_C1C7(r);

    RAM8(0x0E) = 0xD4;
    RAM8(0x0F) = 0xB4;
    sub_B4D4(r);
    sub_D0E5(r);

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_release_then_any_press(),
        [r] { return lotw::native::read_buttons(r); });

    RAM8(0x7C) = 0x20;
    sub_C7B5(r);
    sub_C1C7(r);
}
