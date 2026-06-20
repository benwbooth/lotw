#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_B29B(Regs* r);
extern "C" void sub_C461(Regs* r);
extern "C" void sub_C38B(Regs* r);
extern "C" void sub_B2EE(Regs* r);
extern "C" void sub_B2CC(Regs* r);
extern "C" void sub_B25D(Regs* r);
extern "C" void sub_B1EA(Regs* r);
extern "C" void sub_B215(Regs* r);
extern "C" void sub_C569(Regs* r);
extern "C" void queue_ppu_job_and_wait(Regs* r);
extern "C" void song_init(Regs* r);

namespace {

static bool d4_clear()
{
    return RAM8(0xD4) == 0;
}

static bool d4_set()
{
    return RAM8(0xD4) != 0;
}

lotw::native::FrameTask sub_B13D_script(Regs* r)
{
    lotw::native::GameState game;

    RAM8(0x92) = (u8)(RAM8(0x92) + 1);
    sub_B29B(r);
    sub_C461(r);
    sub_C38B(r);
    sub_B2EE(r);
    RAM8(0x2A) = 0x20;
    RAM8(0x2B) = 0x22;
    RAM8(0x24) = (u8)(RAM8(0x24) | 0x18);

    r->a = 0xFF;
    queue_ppu_job_and_wait(r);

    RAM8(0x8E) = 0x0A;
    song_init(r);

    RAM8(0x1C) = 0x00;
    RAM8(0x1D) = 0x00;
    RAM8(0x0A) = 0x00;
    RAM8(0x7B) = 0x00;
    RAM8(0x7C) = 0x00;
    sub_B2CC(r);

    RAM8(0x18) = 0x40;
    RAM8(0x19) = 0x01;
    RAM8(0x1A) = 0x20;
    RAM8(0x0C) = 0x9C;
    RAM8(0x0D) = 0xB7;

    do {
        sub_B25D(r);
        sub_B1EA(r);
        if (r->c)
            break;
        sub_B25D(r);
        sub_B215(r);
    } while (!r->c);

    game.set_prompt_state(0x20);
    while (d4_clear())
        co_yield lotw::native::Wait::next_frame();
    while (d4_set())
        co_yield lotw::native::Wait::next_frame();

    game.set_frame_counter(0x3C);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    RAM8(0x94) = 0x00;
    RAM8(0xA4) = 0x00;
    RAM8(0xB4) = 0x00;
    RAM8(0xC4) = 0x00;
    game.set_prompt_state(0x18);

    u8 cnt = 0x0A;
    do {
        for (int x = 0x1F; x >= 0; x--)
            RAM8((u16)(0x0180 + x)) = 0x30;
        sub_C569(r);
        game.set_frame_counter(0x01);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        sub_B2CC(r);
        sub_C569(r);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        cnt = (u8)(cnt - 1);
    } while (cnt != 0);
}

} // namespace

extern "C" void sub_B13D(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B13D_script(r),
        [] { return std::uint8_t{0}; });
}
