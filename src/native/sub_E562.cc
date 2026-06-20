#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_CD2C(Regs* r);
extern "C" void sub_D8B6(Regs* r);
extern "C" void sub_D8E3(Regs* r);
extern "C" void sub_D94E(Regs* r);
extern "C" void sub_C1D8(Regs* r);

namespace {

lotw::native::FrameTask sub_E562_script(Regs* r)
{
    lotw::native::GameState game;

    for (;;) {
        game.set_frame_counter(0x01);
        const u8 buttons = lotw::native::read_buttons(r);
        if (buttons & 0x80) {
            r->a = 0x80;
            r->c = 0;
            co_return;
        }

        r->a = buttons & 0x0F;
        r->y = 0x01;
        sub_CD2C(r);
        sub_D8B6(r);

        const u8 ty = RAM8(0x0A);
        if (ty >= 0xA1) {
            r->a = ty;
            r->c = 1;
            co_return;
        }
        if (ty >= 0x20) {
            const u8 lo = RAM8(0x0F) & 0x0F;
            bool store = false;
            if (lo >= 0x01) {
                if (lo < 0x0F)
                    store = true;
                else if (RAM8(0x0E) == 0)
                    store = true;
            }
            if (store) {
                RAM8(0x43) = RAM8(0x0E);
                RAM8(0x44) = RAM8(0x0F);
                RAM8(0x45) = RAM8(0x0A);
            }
        }

        sub_D8E3(r);
        sub_D94E(r);
        sub_C1D8(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

} // namespace

extern "C" void sub_E562(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_E562_script(r),
        [] { return std::uint8_t{0}; });
}
