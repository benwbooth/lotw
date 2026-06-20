#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0057(RoutineContext* r);
extern "C" void routine_0056(RoutineContext* r);
extern "C" void routine_0075(RoutineContext* r);

namespace {

void enter_return_home_frame_commit()
{
    GAME_MEM8(0x0E) = 0x35;
    GAME_MEM8(0x0F) = 0xC1;
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

lotw::native::FrameTask frame_task_0010_script(RoutineContext* r)
{
    lotw::native::GameState game;

    GAME_MEM8(0x09) = 0x40;
    do {
        game.set_frame_counter(0x05);
        routine_0057(r);
        r->index = 0x00;
        r->offset = 0x20;
        routine_0056(r);

        enter_return_home_frame_commit();
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        leave_return_home();

        GAME_MEM8(0x09) = (u8)(GAME_MEM8(0x09) - 0x10);
    } while ((GAME_MEM8(0x09) & 0x80) == 0);
    routine_0075(r);
}

}

extern "C" void routine_0055(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0010_script(r),
        [] { return std::uint8_t{0}; });
}
