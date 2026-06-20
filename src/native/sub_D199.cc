#include "native/frame_wait_helpers.hpp"

#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_CACC(Regs* r);

namespace {

lotw::native::FrameTask sub_D199_script(Regs* r)
{
    lotw::native::GameState game;
    const std::uint8_t saved_blink = game.sprite_blink_timer();

    game.set_sprite_blink_timer(0x00);
    sub_C1D8(r);
    do {
        game.set_player_magic((std::uint8_t)(game.player_magic() + 1));
        sub_CACC(r);
        game.set_prompt_state(0x16);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        r->x = game.player_magic();
    } while (game.player_magic() < 0x63);

    game.set_prompt_state(0x17);
    game.set_frame_counter(0x00);
    lotw::native::commit_frame_work(r);
    game.set_sprite_blink_timer(saved_blink);
}

} // namespace

extern "C" void sub_D199(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_D199_script(r),
        [] { return std::uint8_t{0}; });
}
