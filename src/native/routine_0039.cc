#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0045(RoutineContext* r);
extern "C" void routine_0069(RoutineContext* r);
extern "C" void routine_0066(RoutineContext* r);
extern "C" void routine_0047(RoutineContext* r);
extern "C" void routine_0046(RoutineContext* r);
extern "C" void routine_0043(RoutineContext* r);
extern "C" void routine_0040(RoutineContext* r);
extern "C" void routine_0041(RoutineContext* r);
extern "C" void routine_0075(RoutineContext* r);
extern "C" void queue_ppu_job_and_wait(RoutineContext* r);
extern "C" void song_init(RoutineContext* r);

namespace {

static bool d4_clear()
{
    return GAME_MEM8(0xD4) == 0;
}

static bool d4_set()
{
    return GAME_MEM8(0xD4) != 0;
}

lotw::native::FrameTask frame_task_0006_script(RoutineContext* r)
{
    lotw::native::GameState game;

    GAME_MEM8(0x92) = (u8)(GAME_MEM8(0x92) + 1);
    routine_0045(r);
    routine_0069(r);
    routine_0066(r);
    routine_0047(r);
    GAME_MEM8(0x2A) = 0x20;
    GAME_MEM8(0x2B) = 0x22;
    GAME_MEM8(0x24) = (u8)(GAME_MEM8(0x24) | 0x18);

    r->value = 0xFF;
    queue_ppu_job_and_wait(r);

    GAME_MEM8(0x8E) = 0x0A;
    song_init(r);

    GAME_MEM8(0x1C) = 0x00;
    GAME_MEM8(0x1D) = 0x00;
    GAME_MEM8(0x0A) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x7C) = 0x00;
    routine_0046(r);

    GAME_MEM8(0x18) = 0x40;
    GAME_MEM8(0x19) = 0x01;
    GAME_MEM8(0x1A) = 0x20;
    GAME_MEM8(0x0C) = 0x9C;
    GAME_MEM8(0x0D) = 0xB7;

    do {
        routine_0043(r);
        routine_0040(r);
        if (r->carry)
            break;
        routine_0043(r);
        routine_0041(r);
    } while (!r->carry);

    game.set_prompt_state(0x20);
    while (d4_clear())
        co_yield lotw::native::Wait::next_frame();
    while (d4_set())
        co_yield lotw::native::Wait::next_frame();

    game.set_frame_counter(0x3C);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    GAME_MEM8(0x94) = 0x00;
    GAME_MEM8(0xA4) = 0x00;
    GAME_MEM8(0xB4) = 0x00;
    GAME_MEM8(0xC4) = 0x00;
    game.set_prompt_state(0x18);

    u8 cnt = 0x0A;
    do {
        for (int x = 0x1F; x >= 0; x--)
            GAME_MEM8((u16)(0x0180 + x)) = 0x30;
        routine_0075(r);
        game.set_frame_counter(0x01);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        routine_0046(r);
        routine_0075(r);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        cnt = (u8)(cnt - 1);
    } while (cnt != 0);
}

}

extern "C" void routine_0039(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0006_script(r),
        [] { return std::uint8_t{0}; });
}
