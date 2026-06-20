#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C1D8(Regs* r);

namespace {

lotw::native::FrameTask sub_B4C5_script(Regs* r)
{
    lotw::native::GameState game;

    RAM8(0x56) = r->x;
    RAM8(0x57) = r->y;
    game.set_frame_counter(0x08);
    sub_C1D8(r);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
}

} // namespace

extern "C" void sub_B4C5(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B4C5_script(r),
        [] { return std::uint8_t{0}; });
}
