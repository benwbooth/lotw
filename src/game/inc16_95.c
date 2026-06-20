

#include "game_memory.h"
#include "routine_context.h"

void inc16_95(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    if (++GAME_MEM8((0x95 + x) & 0xFF) == 0)
        ++GAME_MEM8((0x96 + x) & 0xFF);
    r->index = x;
}
