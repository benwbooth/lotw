



#include "game_memory.h"
#include "routine_context.h"

void routine_0094(RoutineContext *r);

void routine_0206(RoutineContext *r)
{
    u16 sum = (u16)r->value + magic;
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    magic = v;
    routine_0094(r);
}
