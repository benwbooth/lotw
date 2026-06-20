


#include "game_memory.h"
#include "routine_context.h"

void routine_0185(RoutineContext *r);

void routine_0181(RoutineContext *r)
{
    u8 x = (u8)(GAME_MEM8(0xF5) - 1);
    if (x & 0x80)
        x = 0x06;
    GAME_MEM8(0xF5) = x;
    routine_0185(r);
}
