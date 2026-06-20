


#include "game_memory.h"
#include "routine_context.h"

void routine_0185(RoutineContext *r);

void routine_0183(RoutineContext *r)
{
    u8 x = (u8)(GAME_MEM8(0xF7) + 1);
    if (x >= 0x05)
        x = 0x00;
    GAME_MEM8(0xF7) = x;
    routine_0185(r);
}
