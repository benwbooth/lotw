

#include "game_memory.h"
#include "routine_context.h"

void routine_0036(RoutineContext *r)
{
    int x;
    for (x = 0x7F; x >= 0; x--)
        GAME_MEM8(0x0240 + x) = GAME_MEM8(0xB71C + x);
    r->index = 0xFF;
}
