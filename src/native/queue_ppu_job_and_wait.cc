#include "native/frame_wait_helpers.hpp"

#include "regs.h"

extern "C" void queue_ppu_job_and_wait(Regs* r)
{
    lotw::native::GameState game;
    lotw::native::wait_for_ppu_job_idle(r);
    game.request_ppu_job(r->a);
    lotw::native::wait_for_ppu_job_idle(r);
}
