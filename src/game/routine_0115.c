


#include "game_memory.h"
#include "routine_context.h"

void routine_0115(RoutineContext *r)
{
    if (GAME_MEM8(0x0A) >= 0xC0)
        r->carry = 1;
    else if (GAME_MEM8(0x0F) < 0x3F)
        r->carry = 0;
    else if (GAME_MEM8(0x0E) == 0)
        r->carry = 0;
    else
        r->carry = 1;
}
