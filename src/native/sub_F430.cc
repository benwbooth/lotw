#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C540(Regs* r);
extern "C" void sub_CF08(Regs* r);
extern "C" void sub_EFF1(Regs* r);

namespace {

lotw::native::FrameTask sub_F430_script(Regs* r)
{
    lotw::native::GameState game;

    if ((RAM8(0xEE) & 0x7F) == 0) {
        game.set_prompt_state(0x18);
        game.set_prompt_argument(0xFF);
        r->x = 0x03;
        sub_C540(r);

        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->x = 0x03;
        sub_C540(r);

        game.set_frame_counter(0x05);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->x = 0x03;
        sub_C540(r);

        RAM8(0xEE) = (u8)(RAM8(0xEE) + 1);
        game.set_prompt_state(0x02);
        RAM8(0xF1) = 0x0F;
        RAM8(0xF5) = 0x00;
        RAM8(0xF6) = 0x00;
        RAM8(0xF0) = 0x00;
        RAM8(0xFC) = RAM8(0xFB);
    }

    if (RAM8(0xF0) == 0) {
        RAM8(0xF1) = (u8)(RAM8(0xF1) - 1);
        if (RAM8(0xF1) == 0) {
            RAM8(0xEF) = (u8)(RAM8(0xEF) | 0x80);
            RAM8(0xF0) = 0x01;
            co_return;
        }
        {
            u8 a = (u8)(RAM8(0xF1) >> 2);
            a = (u8)((a ^ 0xFF) + 1);
            RAM8(0xF7) = a;
        }
        sub_EFF1(r);
        sub_CF08(r);
        if (r->c) {
            RAM8(0xEF) = (u8)(RAM8(0xEF) | 0x80);
            RAM8(0xF0) = 0x01;
            co_return;
        }
        RAM8(0xFB) = RAM8(0x0A);
        co_return;
    }

    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
    RAM8(0xF7) = (u8)((RAM8(0xF0) >> 2) + 1);
    sub_EFF1(r);
    sub_CF08(r);
    if (r->c) {
        RAM8(0xEE) = 0x00;
        RAM8(0xF3) = 0xF0;
        RAM8(0xEB) = 0x01;
        co_return;
    }
    RAM8(0xFB) = RAM8(0x0A);
}

} // namespace

extern "C" void sub_F430(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_F430_script(r),
        [] { return std::uint8_t{0}; });
}
