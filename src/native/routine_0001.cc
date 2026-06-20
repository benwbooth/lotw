#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0074(RoutineContext* r);
extern "C" void routine_0128(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);
extern "C" void routine_0004(RoutineContext* r);
extern "C" void scene_assemble(RoutineContext* r);
extern "C" void routine_0066(RoutineContext* r);
extern "C" void routine_0077(RoutineContext* r);
extern "C" void routine_0080(RoutineContext* r);
extern "C" void routine_0060(RoutineContext* r);
extern "C" void routine_0026(RoutineContext* r);
extern "C" void routine_0016(RoutineContext* r);
extern "C" void routine_0018(RoutineContext* r);
extern "C" void queue_ppu_job_and_wait(RoutineContext* r);

namespace {

using RoutineFn = void (*)(RoutineContext*);

void farcall_cce4(RoutineContext* r, u8 lo, u8 hi, RoutineFn target)
{
    GAME_MEM8(0x0E) = lo;
    GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = GAME_MEM8(0x32);
    GAME_MEM8(0x31) = GAME_MEM8(0x33);
    GAME_MEM8(0x25) = 0x06;
    LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x30) = 0x0C;
    GAME_MEM8(0x31) = 0x0D;
    GAME_MEM8(0x25) = 0x07;
    LOTW_BANK_SYNC();
}

lotw::native::FrameTask frame_task_0001_script(RoutineContext* r)
{
    lotw::native::GameState game;

    game.set_prompt_state(0x18);
    game.set_sprite_blink_timer(0x00);
    routine_0061(r);

    r->index = 0x02;
    farcall_cce4(r, 0x40, 0xC5, routine_0074);
    routine_0128(r);
    routine_0063(r);
    r->index = 0x03;
    farcall_cce4(r, 0x40, 0xC5, routine_0074);
    routine_0004(r);

    game.set_prompt_state(0x20);
    game.set_frame_counter(0x3C);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    GAME_MEM8(0x48) = 0x13;
    GAME_MEM8(0x47) = 0x02;
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);
    routine_0066(r);

    GAME_MEM8(0x0200) = 0xEF;
    GAME_MEM8(0x1E) = 0x22;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7C) = 0x10;
    farcall_cce4(r, 0xCB, 0xC5, routine_0077);
    r->index = 0x04;
    farcall_cce4(r, 0x40, 0xC5, routine_0074);
    GAME_MEM8(0x7C) = 0x00;
    farcall_cce4(r, 0x6C, 0xC7, routine_0080);
    GAME_MEM8(0x2D) = 0x3D;

    for (;;) {
        u8 x = GAME_MEM8(0x1E);
        if (x == 0)
            x = 0xF0;
        if (x == 0xC2)
            break;
        x = (u8)(x - 1);
        GAME_MEM8(0x1E) = x;
        GAME_MEM8(0x1D) = (u8)((x & 0x08) >> 3);
        r->value = 0xFF;
        queue_ppu_job_and_wait(r);
    }

    r->index = 0x02;
    farcall_cce4(r, 0x40, 0xC5, routine_0074);
    farcall_cce4(r, 0xC7, 0xC1, routine_0060);

    GAME_MEM8(0x040C) = 0x00;
    GAME_MEM8(0x040D) = 0x00;
    GAME_MEM8(0x0406) = 0x00;
    GAME_MEM8(0xE9) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x7C) = 0x00;
    GAME_MEM8(0x0405) = 0x64;
    GAME_MEM8(0x3E) = 0x08;
    GAME_MEM8(0x43) = (u8)(((u8)(GAME_MEM8(0x44) << 4)) | GAME_MEM8(0x43));
    routine_0026(r);
    GAME_MEM8(0x0210) = 0xEF;
    GAME_MEM8(0x0214) = 0xEF;
    routine_0016(r);
    routine_0018(r);
}

}

extern "C" void routine_0001(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0001_script(r),
        [] { return std::uint8_t{0}; });
}
