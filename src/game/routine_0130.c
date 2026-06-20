





#include "game_memory.h"
#include "routine_context.h"

void routine_0130(RoutineContext *r)
{
    int i;
    for (i = 7; i >= 0; i--)
        GAME_MEM8(0x0300 + i) = GAME_MEM8(0x0308 + i);
    for (i = 15; i >= 0; i--)
        GAME_MEM8(0x0060 + i) = GAME_MEM8(0x0310 + i);
    GAME_MEM8(0x5A) = GAME_MEM8(0x0321);
    GAME_MEM8(0x5B) = GAME_MEM8(0x0320);
    r->index = 0xFF;
}
