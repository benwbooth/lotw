#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C7FE(Regs* r);
extern "C" void sub_CAA5(Regs* r);
extern "C" void sub_C569(Regs* r);

namespace lotw::native {

void commit_frame_work(Regs* r)
{
    GameState game;

    if (RAM8(0x3D) != 0) {
        RAM8(0x3D) = 0;
        sub_C7FE(r);
    } else if (RAM8(0x3C) != 0) {
        RAM8(0x3C) = 0;
        sub_CAA5(r);
    } else if (game.frame_counter_active()) {
        sub_C569(r);
    }
}

} // namespace lotw::native

extern "C" void sub_C135(Regs* r)
{
    lotw::native::commit_frame_work(r);
    lotw::native::wait_for_frame_counter(r);
}
