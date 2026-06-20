














#include "game_memory.h"
#include "routine_context.h"

void routine_0121(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0120(RoutineContext *r)
{
    u8 lo, hi, c;

    GAME_MEM8(0x16) = 0x47;
    GAME_MEM8(0x17) = 0x22;

    if (GAME_MEM8(0x7C) & 0x10) {

        u16 s = (u16)(0x00 + GAME_MEM8(0x16));
        GAME_MEM8(0x16) = (u8)s;
        GAME_MEM8(0x17) = (u8)(0x04 + GAME_MEM8(0x17) + (s >> 8));
    }

    r->value = GAME_MEM8(0x81);
    routine_0121(r);
    r->value = 0x06;
    queue_ppu_job_and_wait(r);


    lo = GAME_MEM8(0x16);
    c = (u8)((0x0E + lo) >> 8);
    GAME_MEM8(0x16) = (u8)(0x0E + lo);
    hi = GAME_MEM8(0x17);
    GAME_MEM8(0x17) = (u8)(0x00 + hi + c);

    r->value = GAME_MEM8(0x83);
    routine_0121(r);
    r->value = 0x06;
    queue_ppu_job_and_wait(r);
}
