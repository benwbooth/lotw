#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0211(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);
extern "C" void routine_0061(RoutineContext* r);
extern "C" void song_init(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0020_script(RoutineContext* r)
{
    lotw::native::GameState game;

    routine_0211(r);
    if (r->carry) {
        game.set_prompt_state(0x06);
        r->carry = 0;
        co_return;
    }

    const u16 ptr = (u16)(GAME_MEM8(0x77) | (GAME_MEM8(0x78) << 8));
    const u8 door = GAME_MEM8((u16)(ptr + 0x0A));
    if (door < 0x08)
        GAME_MEM8(0x04A2) = 0;

    GAME_MEM8(0x04A1) = (u8)(door + 0x02);
    GAME_MEM8(0x04A0) = (u8)(((door << 2) & 0xFF) + 0x81);
    game.set_prompt_state(0x1F);

    routine_0063(r);

    const std::uint8_t saved_blink = game.sprite_blink_timer();
    game.set_sprite_blink_timer(0);
    routine_0061(r);

    const u8 saved_song = GAME_MEM8(0x8E);
    GAME_MEM8(0x8E) = 0x0E;
    r->value = 0x0E;
    song_init(r);

    game.set_frame_counter(0x78);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    GAME_MEM8(0x8E) = saved_song;
    r->value = saved_song;
    song_init(r);

    game.set_sprite_blink_timer(saved_blink);
    r->carry = 1;
}

}

extern "C" void routine_0148(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0020_script(r),
        [] { return std::uint8_t{0}; });
}
