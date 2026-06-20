#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_E86F(Regs* r);
extern "C" void sub_C2B1(Regs* r);
extern "C" void sub_C1D8(Regs* r);
extern "C" void song_init(Regs* r);

namespace {

lotw::native::FrameTask sub_DA31_script(Regs* r)
{
    lotw::native::GameState game;

    sub_E86F(r);
    if (r->c) {
        game.set_prompt_state(0x06);
        r->c = 0;
        co_return;
    }

    const u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    const u8 door = RAM8((u16)(ptr + 0x0A));
    if (door < 0x08)
        RAM8(0x04A2) = 0;

    RAM8(0x04A1) = (u8)(door + 0x02);
    RAM8(0x04A0) = (u8)(((door << 2) & 0xFF) + 0x81);
    game.set_prompt_state(0x1F);

    sub_C2B1(r);

    const std::uint8_t saved_blink = game.sprite_blink_timer();
    game.set_sprite_blink_timer(0);
    sub_C1D8(r);

    const u8 saved_song = RAM8(0x8E);
    RAM8(0x8E) = 0x0E;
    r->a = 0x0E;
    song_init(r);

    game.set_frame_counter(0x78);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    RAM8(0x8E) = saved_song;
    r->a = saved_song;
    song_init(r);

    game.set_sprite_blink_timer(saved_blink);
    r->c = 1;
}

} // namespace

extern "C" void sub_DA31(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_DA31_script(r),
        [] { return std::uint8_t{0}; });
}
