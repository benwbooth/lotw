#include "native/frame_wait_helpers.hpp"

#include "routine_context.h"

extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0094(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0019_script(RoutineContext* r)
{
    lotw::native::GameState game;
    const std::uint8_t saved_blink = game.sprite_blink_timer();

    game.set_sprite_blink_timer(0x00);
    routine_0061(r);
    do {
        game.set_player_magic((std::uint8_t)(game.player_magic() + 1));
        routine_0094(r);
        game.set_prompt_state(0x16);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        r->index = game.player_magic();
    } while (game.player_magic() < 0x63);

    game.set_prompt_state(0x17);
    game.set_frame_counter(0x00);
    lotw::native::commit_frame_work(r);
    game.set_sprite_blink_timer(saved_blink);
}

}

extern "C" void routine_0134(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0019_script(r),
        [] { return std::uint8_t{0}; });
}
