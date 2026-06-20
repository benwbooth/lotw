#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

namespace {

lotw::native::FrameTask sub_A5B5_script(Regs* r)
{
    lotw::native::GameState game;

    u8 y = 0x04;
    do {
        game.set_frame_counter(0x05);
        for (int x = 0x0C; x >= 0; x--) {
            const u8 lo = (u8)(RAM8((u16)(0x0180 + x)) & 0x0F);
            RAM8(0x08) = lo;
            const u8 hi = (u8)(RAM8((u16)(0x0180 + x)) & 0xF0);
            const u8 out = ((int)hi - 0x10 < 0) ? 0x0F : (u8)((hi - 0x10) | lo);
            RAM8((u16)(0x0180 + x)) = out;
        }
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        y--;
    } while (y != 0);
}

} // namespace

extern "C" void sub_A5B5(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_A5B5_script(r),
        [] { return std::uint8_t{0}; });
}
