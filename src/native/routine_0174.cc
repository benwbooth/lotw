#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0193(RoutineContext* r);
extern "C" void routine_0195(RoutineContext* r);
extern "C" void routine_0197(RoutineContext* r);
extern "C" void routine_0117(RoutineContext* r);
extern "C" void routine_0119(RoutineContext* r);
extern "C" void routine_0060(RoutineContext* r);
extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0070(RoutineContext* r);
extern "C" void routine_0194(RoutineContext* r);
extern "C" void routine_0067(RoutineContext* r);
extern "C" void routine_0200(RoutineContext* r);
extern "C" void routine_0123(RoutineContext* r);
extern "C" void routine_0084(RoutineContext* r);
extern "C" void routine_0077(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);

extern "C" void routine_0174(RoutineContext* r)
{
    lotw::native::GameState game;
    game.set_prompt_state(0x03);
    game.push_dialog_depth();

    if (GAME_MEM8(0x2D) < 0x30) {
        routine_0193(r);
        r->value = 0x08;
        routine_0195(r);
        routine_0197(r);
        routine_0117(r);
        routine_0119(r);
        GAME_MEM8(0x7B) = 0x08;
        routine_0060(r);
        routine_0061(r);
        routine_0070(r);
    }

    lotw::native::run_frame_task(
        r,
        lotw::native::wait_release_then_button_then_release(0x10),
        [r] { return lotw::native::read_buttons(r); });

    game.set_prompt_state(0x04);

    if (GAME_MEM8(0x2D) < 0x30) {
        routine_0194(r);
        routine_0067(r);
        routine_0200(r);
        r->value = GAME_MEM8(0xFE);
        routine_0123(r);
        routine_0084(r);
        routine_0077(r);
        routine_0061(r);
        routine_0063(r);
        routine_0060(r);
        routine_0070(r);
    }

    game.pop_dialog_depth();
}
