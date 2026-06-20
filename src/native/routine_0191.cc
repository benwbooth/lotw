#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0107(RoutineContext* r);
extern "C" void routine_0143(RoutineContext* r);
extern "C" void routine_0061(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0025_script(RoutineContext* r)
{
    lotw::native::GameState game;

    for (;;) {
        game.set_frame_counter(0x01);
        const u8 buttons = lotw::native::read_buttons(r);
        if (buttons & 0x80) {
            r->value = 0x80;
            co_return;
        }

        r->value = buttons & 0x0F;
        r->offset = 0x01;
        routine_0107(r);
        routine_0143(r);

        const u8 ty = GAME_MEM8(0x0A);
        if (ty >= 0x30 && ty < 0xA1) {
            const u8 lo = GAME_MEM8(0x0F) & 0x0F;
            if (lo >= 0x02) {
                const bool store = (lo < 0x0D) || (GAME_MEM8(0x0E) == 0);
                if (store) {
                    GAME_MEM8(0x43) = GAME_MEM8(0x0E);
                    GAME_MEM8(0x44) = GAME_MEM8(0x0F);
                    GAME_MEM8(0x45) = GAME_MEM8(0x0A);
                }
            }
        }

        routine_0061(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

}

extern "C" void routine_0191(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0025_script(r),
        [] { return std::uint8_t{0}; });
}
