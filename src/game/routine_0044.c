




#include "game_memory.h"
#include "routine_context.h"

void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0044(RoutineContext *r)
{
    u8 saved_a = r->value;
    u8 v = (u8)(GAME_MEM8(0x0A) + 0x06);
    if (v >= 0xF0)
        v = (u8)(v + 0x10);
    GAME_MEM8(0x1E) = v;

    r->value = saved_a;
    queue_ppu_job_and_wait(r);
    r->value = 0xFF;
    queue_ppu_job_and_wait(r);
    r->value = 0xFF;
    queue_ppu_job_and_wait(r);
    r->value = 0xFF;
    queue_ppu_job_and_wait(r);
}
