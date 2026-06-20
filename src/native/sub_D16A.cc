#include "native/frame_wait_helpers.hpp"

#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_CAB6(Regs* r);

namespace {

lotw::native::FrameTask sub_D16A_script(Regs* r)
{
    lotw::native::GameState game;
    const std::uint8_t saved_blink = game.sprite_blink_timer();

    game.set_sprite_blink_timer(0x00);
    sub_C1D8(r);
    do {
        game.set_player_health((std::uint8_t)(game.player_health() + 1));
        sub_CAB6(r);
        game.set_prompt_state(0x16);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        r->x = game.player_health();
    } while (game.player_health() < 0x63);

    game.set_prompt_state(0x17);
    game.set_frame_counter(0x00);
    lotw::native::commit_frame_work(r);
    game.set_sprite_blink_timer(saved_blink);
}

} // namespace

extern "C" void sub_D16A(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_D16A_script(r),
        [] { return std::uint8_t{0}; });
}
