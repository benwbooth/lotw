










#include "game_memory.h"
#include "routine_context.h"

void routine_0247(RoutineContext *r);

void routine_0236(RoutineContext *r)
{
    GAME_MEM8(0xF7) = (u8)((GAME_MEM8(0xF0) >> 1) + 0x02);
    routine_0247(r);
    if (!r->carry)
        return;

    GAME_MEM8(0xF5) = 0x00;
    GAME_MEM8(0xF6) = 0x00;
    routine_0247(r);
    if (!r->carry)
        return;

    GAME_MEM8(0xF7) = 0x00;
}
