

#include "game_memory.h"
#include "routine_context.h"

void routine_0057(RoutineContext *r)
{
    int x;
    for (x = 0x1F; x >= 0; x--)
        GAME_MEM8(0x0180 + x) = GAME_MEM8(0xA2C9 + x);
    r->index = 0xFF;
}
