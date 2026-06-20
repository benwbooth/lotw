#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C15D(Regs* r);
extern "C" void sub_C2B1(Regs* r);
extern "C" void sub_F628(Regs* r);
extern "C" void sub_E87C(Regs* r);
extern "C" void sub_F782(Regs* r);
extern "C" void sub_B307(Regs* r);
extern "C" void sub_A2EB(Regs* r);
extern "C" void sub_ABBC(Regs* r);
extern "C" void sub_A5E6(Regs* r);
extern "C" void sub_A75D(Regs* r);
extern "C" void sub_A3E3(Regs* r);
extern "C" void game_update(Regs* r);
extern "C" void main_init(Regs* r);

namespace {

using PortFn = void (*)(Regs*);

void farcall_0C0D(Regs* r, u8 lo, u8 hi, PortFn target)
{
    const u8 old6 = RAM8(0x30);
    const u8 old7 = RAM8(0x31);
    RAM8(0x32) = old6;
    RAM8(0x33) = old7;
    RAM8(0x0E) = lo;
    RAM8(0x0F) = hi;
    RAM8(0x30) = 0x0C;
    RAM8(0x31) = 0x0D;
    RAM8(0x25) = 0x07;
    NES_PRG_SYNC();
    target(r);
    RAM8(0x31) = old7;
    RAM8(0x30) = old6;
    RAM8(0x25) = 0x06;
    NES_PRG_SYNC();
}

lotw::native::FrameTask main_loop_dispatch_script(Regs* r)
{
    lotw::native::GameState game;

    for (;;) {
        if (game.player_health() == 0) {
            game.set_sprite_blink_timer(0x00);
            sub_C1D8(r);
            farcall_0C0D(r, 0x07, 0xB3, sub_B307);
            if (r->x == 0)
                continue;
            r->x = (u8)(r->x - 1);
            main_init(r);
            co_return;
        }

        game.set_frame_counter(0x01);
        RAM8(0x7E) = RAM8(0x7C);
        lotw::native::read_buttons(r);
        game_update(r);

        if (RAM8(0xEC) != 0) {
            farcall_0C0D(r, 0xEB, 0xA2, sub_A2EB);
            do {
                lotw::native::read_buttons(r);
                farcall_0C0D(r, 0xBC, 0xAB, sub_ABBC);
                farcall_0C0D(r, 0xE6, 0xA5, sub_A5E6);
                farcall_0C0D(r, 0x5D, 0xA7, sub_A75D);
                farcall_0C0D(r, 0xE3, 0xA3, sub_A3E3);
            } while (game.player_health() == 0);

            RAM8(0x44) = (u8)(RAM8(0x43) >> 4);
            RAM8(0x43) = (u8)(RAM8(0x43) & 0x0F);
            RAM8(0x0200) = 0xEF;
            game.set_sprite_blink_timer(0x00);
            sub_C1D8(r);
            farcall_0C0D(r, 0x07, 0xB3, sub_B307);
            r->x = (u8)(r->x - 1);
            main_init(r);
            co_return;
        }

        sub_F628(r);
        sub_E87C(r);
        sub_F782(r);
        sub_C15D(r);
        {
            const u8 saved_c = r->c;
            sub_C1D8(r);
            sub_C2B1(r);
            r->c = saved_c;
        }
        if (!r->c && RAM8(0x7E) != RAM8(0x7C))
            RAM8(0x3D)++;

        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

} // namespace

extern "C" void main_loop_dispatch(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        main_loop_dispatch_script(r),
        [] { return std::uint8_t{0}; });
}
