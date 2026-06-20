



#include "game_memory.h"
#include "routine_context.h"

void routine_0093(RoutineContext *r);

void routine_0205(RoutineContext *r)
{
    u16 sum = (u16)r->value + health;
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    health = v;
    routine_0093(r);
}
