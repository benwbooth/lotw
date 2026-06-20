#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C461(Regs* r);
extern "C" void sub_C38B(Regs* r);
extern "C" void sub_C57A(Regs* r);
extern "C" void sub_C375(Regs* r);
extern "C" void sub_B631(Regs* r);
extern "C" void sub_CAB6(Regs* r);
extern "C" void sub_CAF8(Regs* r);
extern "C" void sub_CAE2(Regs* r);

namespace {

using PortFn = void (*)(Regs*);

void enter_return_home(std::uint8_t lo, std::uint8_t hi)
{
    RAM8(0x0E) = lo;
    RAM8(0x0F) = hi;
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

void farcall_cce4(Regs* r, std::uint8_t lo, std::uint8_t hi, PortFn target)
{
    enter_return_home(lo, hi);
    target(r);
    leave_return_home();
}

lotw::native::FrameTask sub_B0B1_script(Regs* r)
{
    lotw::native::GameState game;

    sub_C461(r);
    farcall_cce4(r, 0x8B, 0xC3, sub_C38B);
    sub_C57A(r);
    sub_C375(r);
    sub_B631(r);
    sub_CAB6(r);
    sub_CAF8(r);
    sub_CAE2(r);
    sub_CAF8(r);

    game.set_frame_counter(0x01);
    enter_return_home(0x35, 0xC1);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
    leave_return_home();
}

} // namespace

extern "C" void sub_B0B1(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B0B1_script(r),
        [] { return std::uint8_t{0}; });
}
