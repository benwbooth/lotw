


#include "game_memory.h"
#include "routine_context.h"

void routine_0185(RoutineContext *r);

void routine_0182(RoutineContext *r)
{
    u8 x = (u8)(GAME_MEM8(0xF7) - 1);
    if (x & 0x80)
        x = 0x04;
    GAME_MEM8(0xF7) = x;
    routine_0185(r);
}
