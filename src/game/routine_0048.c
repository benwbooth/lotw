

#include "game_memory.h"
#include "routine_context.h"

void routine_0048(RoutineContext *r)
{
    int y;
    for (y = 0x1F; y >= 0; --y)
        GAME_MEM8(0x0140 + y) = 0xC0;
    r->value = 0xC0;
    r->offset = 0xFF;
}
