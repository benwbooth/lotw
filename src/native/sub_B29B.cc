#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C2B1(Regs* r);

namespace {

lotw::native::FrameTask sub_B29B_script(Regs* r)
{
    lotw::native::GameState game;

    RAM8(0xB4) = 0;
    RAM8(0x0D) = 0x10;
    do {
        if (RAM8(0xA0) != 0) RAM8(0xA0) = (u8)(RAM8(0xA0) - 1);
        if (RAM8(0xB0) != 0) RAM8(0xB0) = (u8)(RAM8(0xB0) - 1);
        if (RAM8(0xD0) != 0) RAM8(0xD0) = (u8)(RAM8(0xD0) - 1);
        RAM8(0x0C) = 0x14;
        do {
            sub_C2B1(r);
            game.set_frame_counter(0x01);
            lotw::native::commit_frame_work(r);
            while (game.frame_counter_active())
                co_yield lotw::native::Wait::next_frame();
            RAM8(0x0C) = (u8)(RAM8(0x0C) - 1);
        } while (RAM8(0x0C) != 0);
        RAM8(0x0D) = (u8)(RAM8(0x0D) - 1);
    } while (RAM8(0x0D) != 0);
}

} // namespace

extern "C" void sub_B29B(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B29B_script(r),
        [] { return std::uint8_t{0}; });
}
