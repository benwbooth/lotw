

#include "game_memory.h"
#include "routine_context.h"

void routine_0233(RoutineContext *r)
{
    u8 v = GAME_MEM8(0xF4) & 0x03;
    if (v == 0)
        v = 0x01;
    v ^= 0x03;
    GAME_MEM8(0xF4) = v;
    r->value = v;
}
