#include "native/frame_wait_helpers.hpp"

#include "routine_context.h"

extern "C" void queue_ppu_job_and_wait(RoutineContext* r)
{
    lotw::native::GameState game;
    lotw::native::wait_for_ppu_job_idle(r);
    game.request_ppu_job(r->value);
    lotw::native::wait_for_ppu_job_idle(r);
}
