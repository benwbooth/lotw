#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C520(Regs* r);
extern "C" void sub_C569(Regs* r);

namespace {

lotw::native::FrameTask sub_C4E0_script(Regs* r)
{
    lotw::native::GameState game;
    const u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));

    u8 v = 0x40;
    RAM8(0x09) = v;
    do {
        game.set_frame_counter(0x05);
        for (u8 y = 0xE0; y < 0xE4; ++y)
            RAM8((u16)(0x00A0 + y)) = RAM8((u16)(ptr + y));
        for (u8 y = 0xF0; y < 0xF4; ++y)
            RAM8((u16)(0x00A0 + y)) = RAM8((u16)(ptr + y));
        r->x = 0x00;
        r->y = 0x04;
        sub_C520(r);
        r->x = 0x10;
        r->y = 0x04;
        sub_C520(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        v = (u8)(RAM8(0x09) - 0x10);
        RAM8(0x09) = v;
    } while ((v & 0x80) == 0);
    sub_C569(r);
}

} // namespace

extern "C" void sub_C4E0(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_C4E0_script(r),
        [] { return std::uint8_t{0}; });
}
