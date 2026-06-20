#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

namespace {

lotw::native::FrameTask sub_C461_script(Regs* r)
{
    lotw::native::GameState game;

    u8 y = 0x04;
    do {
        game.set_frame_counter(0x05);
        for (int x = 0x20; x >= 0; --x) {
            const u8 v = RAM8((u16)(0x0180 + x));
            const u8 lo = (u8)(v & 0x0F);
            const u8 hi = (u8)(v & 0xF0);
            RAM8(0x08) = lo;
            RAM8((u16)(0x0180 + x)) = (hi >= 0x10) ? (u8)((hi - 0x10) | lo) : 0x0F;
        }
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    } while (--y != 0);
}

} // namespace

extern "C" void sub_C461(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_C461_script(r),
        [] { return std::uint8_t{0}; });
}
