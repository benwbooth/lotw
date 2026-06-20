








#include "game_memory.h"
#include "routine_context.h"

void frame_counters(RoutineContext *r)
{
    if (--GAME_MEM8(0x84) != 0)
        return;
    for (int x = 7; x >= 0; x--) {
        if (GAME_MEM8((0x85 + x) & 0xFF) != 0)
            --GAME_MEM8((0x85 + x) & 0xFF);
    }
    GAME_MEM8(0x84) = 0x3C;
    r->index = 0xFF;
}
