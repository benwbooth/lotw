#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C7B5(Regs* r);
extern "C" void sub_D15F(Regs* r);
extern "C" void sub_D0E5(Regs* r);
extern "C" void sub_C1C7(Regs* r);
extern "C" void sub_E3D6(Regs* r);
extern "C" void sub_E400(Regs* r);
extern "C" void sub_E347(Regs* r);
extern "C" void sub_E3C7(Regs* r);
extern "C" void sub_E39E(Regs* r);
extern "C" void sub_E3AD(Regs* r);
extern "C" void sub_E3BA(Regs* r);
extern "C" void sub_E372(Regs* r);
extern "C" void sub_E7B2(Regs* r);

namespace {

lotw::native::FrameTask sub_E2AA_script(Regs* r)
{
    lotw::native::GameState game;

    for (;;) {
        game.set_frame_counter(0x01);
        const u8 b = lotw::native::read_buttons(r);
        r->a = b;

        if (b & 0x80) {
            sub_E372(r);
            sub_D0E5(r);
        } else if (b & 0x40) {
            /* no handler */
        } else if (b & 0x01) {
            sub_E39E(r);
        } else if (b & 0x02) {
            sub_E3AD(r);
        } else if (b & 0x04) {
            sub_E3C7(r);
        } else if (b & 0x08) {
            sub_E3BA(r);
            sub_D0E5(r);
        } else if (b & 0x10) {
            sub_E347(r);
        } else if (b & 0x20) {
            RAM8(0x7C) = 0x20;
            sub_C7B5(r);
            sub_C1C7(r);
            sub_E7B2(r);
            co_return;
        }

        if (game.buttons() & 0xCF) {
            game.set_prompt_state(0x0C);
            game.set_frame_counter(0x0A);
        }
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

} // namespace

extern "C" void sub_E2AA(Regs* r)
{
    RAM8(0x7C) = 0x30;
    sub_C7B5(r);
    sub_D15F(r);
    sub_D0E5(r);
    sub_C1C7(r);

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::read_buttons(r); });

    RAM8(0xF9) = 0;
    RAM8(0xF5) = 0;
    RAM8(0xF7) = 0;
    RAM8(0x0281) = 0xF5;
    RAM8(0x0291) = 0xF5;
    RAM8(0x0285) = 0xF7;
    RAM8(0x0295) = 0xF7;
    RAM8(0x0282) = 0x00;
    RAM8(0x0286) = 0x00;
    RAM8(0x0292) = 0x00;
    RAM8(0x0296) = 0x00;
    sub_E3D6(r);
    sub_E400(r);

    lotw::native::run_frame_task(
        r,
        sub_E2AA_script(r),
        [] { return std::uint8_t{0}; });
}
