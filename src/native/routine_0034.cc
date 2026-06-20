#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0069(RoutineContext* r);
extern "C" void routine_0066(RoutineContext* r);
extern "C" void routine_0076(RoutineContext* r);
extern "C" void routine_0065(RoutineContext* r);
extern "C" void routine_0053(RoutineContext* r);
extern "C" void routine_0093(RoutineContext* r);
extern "C" void routine_0096(RoutineContext* r);
extern "C" void routine_0095(RoutineContext* r);

namespace {

using RoutineFn = void (*)(RoutineContext*);

void enter_return_home(std::uint8_t lo, std::uint8_t hi)
{
    GAME_MEM8(0x0E) = lo;
    GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = GAME_MEM8(0x32);
    GAME_MEM8(0x31) = GAME_MEM8(0x33);
    GAME_MEM8(0x25) = 0x06;
    LOTW_BANK_SYNC();
}

void leave_return_home()
{
    GAME_MEM8(0x30) = 0x0C;
    GAME_MEM8(0x31) = 0x0D;
    GAME_MEM8(0x25) = 0x07;
    LOTW_BANK_SYNC();
}

void farcall_cce4(RoutineContext* r, std::uint8_t lo, std::uint8_t hi, RoutineFn target)
{
    enter_return_home(lo, hi);
    target(r);
    leave_return_home();
}

lotw::native::FrameTask frame_task_0005_script(RoutineContext* r)
{
    lotw::native::GameState game;

    routine_0069(r);
    farcall_cce4(r, 0x8B, 0xC3, routine_0066);
    routine_0076(r);
    routine_0065(r);
    routine_0053(r);
    routine_0093(r);
    routine_0096(r);
    routine_0095(r);
    routine_0096(r);

    game.set_frame_counter(0x01);
    enter_return_home(0x35, 0xC1);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();
    leave_return_home();
}

}

extern "C" void routine_0034(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0005_script(r),
        [] { return std::uint8_t{0}; });
}
