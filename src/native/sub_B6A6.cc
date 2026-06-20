#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_B6F0(Regs* r);
extern "C" void sub_B6D0(Regs* r);
extern "C" void sub_C569(Regs* r);

namespace {

void enter_return_home_frame_commit()
{
    RAM8(0x0E) = 0x35;
    RAM8(0x0F) = 0xC1;
    RAM8(0x30) = RAM8(0x32);
    RAM8(0x31) = RAM8(0x33);
    RAM8(0x25) = 0x06;
    NES_PRG_SYNC();
}

void leave_return_home()
{
    RAM8(0x30) = 0x0C;
    RAM8(0x31) = 0x0D;
    RAM8(0x25) = 0x07;
    NES_PRG_SYNC();
}

lotw::native::FrameTask sub_B6A6_script(Regs* r)
{
    lotw::native::GameState game;

    RAM8(0x09) = 0x40;
    do {
        game.set_frame_counter(0x05);
        sub_B6F0(r);
        r->x = 0x00;
        r->y = 0x20;
        sub_B6D0(r);

        enter_return_home_frame_commit();
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        leave_return_home();

        RAM8(0x09) = (u8)(RAM8(0x09) - 0x10);
    } while ((RAM8(0x09) & 0x80) == 0);
    sub_C569(r);
}

} // namespace

extern "C" void sub_B6A6(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B6A6_script(r),
        [] { return std::uint8_t{0}; });
}
