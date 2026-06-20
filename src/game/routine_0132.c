

#include "game_memory.h"
#include "routine_context.h"

void routine_0132(RoutineContext *r)
{
    int x;
    for (x = 0x1F; x >= 0; --x)
        GAME_MEM8(0x0322 + x) = 0x7F;
    r->value = 0x7F;
    r->index = 0xFF;
}
