







#include "game_memory.h"
#include "routine_context.h"

void routine_0106(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0092(RoutineContext *r)
{
    routine_0106(r);
    GAME_MEM8(0x16) = 0x60;
    GAME_MEM8(0x17) = 0x23;
    r->value = 0x04;
    queue_ppu_job_and_wait(r);
}
