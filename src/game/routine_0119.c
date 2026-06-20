















#include "game_memory.h"
#include "routine_context.h"

void routine_0121(RoutineContext *r);
void routine_0124(RoutineContext *r);
void routine_0125(RoutineContext *r);
void routine_0126(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0119(RoutineContext *r)
{
    GAME_MEM8(0x16) = 0xDE;
    GAME_MEM8(0x17) = 0x21;
    routine_0125(r);
    routine_0121(r);
    r->value = 0x06;
    queue_ppu_job_and_wait(r);

    GAME_MEM8(0x16) = 0x1E;
    GAME_MEM8(0x17) = 0x22;
    routine_0124(r);
    routine_0121(r);
    r->value = 0x06;
    queue_ppu_job_and_wait(r);

    GAME_MEM8(0x16) = 0x5E;
    GAME_MEM8(0x17) = 0x22;
    routine_0126(r);
    routine_0121(r);
    r->value = 0x06;
    queue_ppu_job_and_wait(r);
}
