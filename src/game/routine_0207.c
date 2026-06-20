



#include "game_memory.h"
#include "routine_context.h"

void routine_0096(RoutineContext *r);

void routine_0207(RoutineContext *r)
{
    u16 sum = (u16)r->value + gold;
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    gold = v;
    routine_0096(r);
}
