#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C540(Regs* r);
extern "C" void sub_D08A(Regs* r);
extern "C" void sub_C2B1(Regs* r);
extern "C" void sub_A5B5(Regs* r);
extern "C" void scene_assemble(Regs* r);
extern "C" void sub_C38B(Regs* r);
extern "C" void sub_C5CB(Regs* r);
extern "C" void sub_C76C(Regs* r);
extern "C" void sub_C1C7(Regs* r);
extern "C" void sub_AD7A(Regs* r);
extern "C" void sub_A7D2(Regs* r);
extern "C" void sub_A7F0(Regs* r);
extern "C" void queue_ppu_job_and_wait(Regs* r);

namespace {

using PortFn = void (*)(Regs*);

void farcall_cce4(Regs* r, u8 lo, u8 hi, PortFn target)
{
    RAM8(0x0E) = lo;
    RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32);
    RAM8(0x31) = RAM8(0x33);
    RAM8(0x25) = 0x06;
    NES_PRG_SYNC();
    target(r);
    RAM8(0x30) = 0x0C;
    RAM8(0x31) = 0x0D;
    RAM8(0x25) = 0x07;
    NES_PRG_SYNC();
}

lotw::native::FrameTask sub_A2EB_script(Regs* r)
{
    lotw::native::GameState game;

    game.set_prompt_state(0x18);
    game.set_sprite_blink_timer(0x00);
    sub_C1D8(r);

    r->x = 0x02;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);
    sub_D08A(r);
    sub_C2B1(r);
    r->x = 0x03;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);
    sub_A5B5(r);

    game.set_prompt_state(0x20);
    game.set_frame_counter(0x3C);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    RAM8(0x48) = 0x13;
    RAM8(0x47) = 0x02;
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);
    sub_C38B(r);

    RAM8(0x0200) = 0xEF;
    RAM8(0x1E) = 0x22;
    RAM8(0x7B) = 0x00;
    RAM8(0x43) = 0x00;
    RAM8(0x7C) = 0x10;
    farcall_cce4(r, 0xCB, 0xC5, sub_C5CB);
    r->x = 0x04;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);
    RAM8(0x7C) = 0x00;
    farcall_cce4(r, 0x6C, 0xC7, sub_C76C);
    RAM8(0x2D) = 0x3D;

    for (;;) {
        u8 x = RAM8(0x1E);
        if (x == 0)
            x = 0xF0;
        if (x == 0xC2)
            break;
        x = (u8)(x - 1);
        RAM8(0x1E) = x;
        RAM8(0x1D) = (u8)((x & 0x08) >> 3);
        r->a = 0xFF;
        queue_ppu_job_and_wait(r);
    }

    r->x = 0x02;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);
    farcall_cce4(r, 0xC7, 0xC1, sub_C1C7);

    RAM8(0x040C) = 0x00;
    RAM8(0x040D) = 0x00;
    RAM8(0x0406) = 0x00;
    RAM8(0xE9) = 0x00;
    RAM8(0x7B) = 0x00;
    RAM8(0x7C) = 0x00;
    RAM8(0x0405) = 0x64;
    RAM8(0x3E) = 0x08;
    RAM8(0x43) = (u8)(((u8)(RAM8(0x44) << 4)) | RAM8(0x43));
    sub_AD7A(r);
    RAM8(0x0210) = 0xEF;
    RAM8(0x0214) = 0xEF;
    sub_A7D2(r);
    sub_A7F0(r);
}

} // namespace

extern "C" void sub_A2EB(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_A2EB_script(r),
        [] { return std::uint8_t{0}; });
}
