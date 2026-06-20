#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0081(RoutineContext* r);
extern "C" void routine_0132(RoutineContext* r);
extern "C" void routine_0131(RoutineContext* r);
extern "C" void routine_0060(RoutineContext* r);
extern "C" void routine_0184(RoutineContext* r);
extern "C" void routine_0185(RoutineContext* r);
extern "C" void routine_0178(RoutineContext* r);
extern "C" void routine_0183(RoutineContext* r);
extern "C" void routine_0180(RoutineContext* r);
extern "C" void routine_0181(RoutineContext* r);
extern "C" void routine_0182(RoutineContext* r);
extern "C" void routine_0179(RoutineContext* r);
extern "C" void routine_0201(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0022_script(RoutineContext* r)
{
    lotw::native::GameState game;

    for (;;) {
        game.set_frame_counter(0x01);
        const u8 b = lotw::native::read_buttons(r);
        r->value = b;

        if (b & 0x80) {
            routine_0179(r);
            routine_0131(r);
        } else if (b & 0x40) {

        } else if (b & 0x01) {
            routine_0180(r);
        } else if (b & 0x02) {
            routine_0181(r);
        } else if (b & 0x04) {
            routine_0183(r);
        } else if (b & 0x08) {
            routine_0182(r);
            routine_0131(r);
        } else if (b & 0x10) {
            routine_0178(r);
        } else if (b & 0x20) {
            GAME_MEM8(0x7C) = 0x20;
            routine_0081(r);
            routine_0060(r);
            routine_0201(r);
            co_return;
        }

        if (game.buttons() & 0xCF) {
            game.set_prompt_state(0x0C);
            game.set_frame_counter(0x0A);
        }
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

}

extern "C" void routine_0177(RoutineContext* r)
{
    GAME_MEM8(0x7C) = 0x30;
    routine_0081(r);
    routine_0132(r);
    routine_0131(r);
    routine_0060(r);

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::read_buttons(r); });

    GAME_MEM8(0xF9) = 0;
    GAME_MEM8(0xF5) = 0;
    GAME_MEM8(0xF7) = 0;
    GAME_MEM8(0x0281) = 0xF5;
    GAME_MEM8(0x0291) = 0xF5;
    GAME_MEM8(0x0285) = 0xF7;
    GAME_MEM8(0x0295) = 0xF7;
    GAME_MEM8(0x0282) = 0x00;
    GAME_MEM8(0x0286) = 0x00;
    GAME_MEM8(0x0292) = 0x00;
    GAME_MEM8(0x0296) = 0x00;
    routine_0184(r);
    routine_0185(r);

    lotw::native::run_frame_task(
        r,
        frame_task_0022_script(r),
        [] { return std::uint8_t{0}; });
}
