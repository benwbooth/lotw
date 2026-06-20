

#include "game_memory.h"
#include "routine_context.h"

void routine_0037(RoutineContext *r)
{
    int x;
    for (x = 0x1F; x >= 0; x--)
        GAME_MEM8(0x0240 + x) = GAME_MEM8(0xB6FC + x);
    r->index = 0xFF;
}
