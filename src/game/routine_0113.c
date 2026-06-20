



#include "game_memory.h"
#include "routine_context.h"

void routine_0113(RoutineContext *r)
{
    u8 diff = (u8)(GAME_MEM8(0x0A) - GAME_MEM8(0x45));
    if (diff < 0x10)
        r->carry = 1;
    else if (diff < 0xF1)
        r->carry = 0;
    else
        r->carry = 1;
}
