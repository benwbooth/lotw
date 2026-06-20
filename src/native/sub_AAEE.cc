#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C2B1(Regs* r);

namespace {

lotw::native::FrameTask sub_AAEE_script(Regs* r)
{
    lotw::native::GameState game;

    sub_C1D8(r);
    sub_C2B1(r);
    game.set_frame_counter(0x01);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
}

} // namespace

extern "C" void sub_AAEE(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_AAEE_script(r),
        [] { return std::uint8_t{0}; });
}
