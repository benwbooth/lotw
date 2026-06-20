


#include "game_memory.h"
#include "routine_context.h"

void routine_0097(RoutineContext *r);

void routine_0093(RoutineContext *r)
{
    u8 v = health;
    if (v >= 0x6D)
        v = 0x6D;
    health = v;
    GAME_MEM8(0x08) = v;
    r->value = v;
    r->index = 0x00;
    routine_0097(r);
    r->value = 0x01;
    GAME_MEM8(0x3C) = 0x01;
}
