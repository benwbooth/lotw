




#include "game_memory.h"
#include "routine_context.h"

void routine_0065(RoutineContext *r)
{
    int x;
    for (x = 3; x >= 0; x--)
        GAME_MEM8((u16)(0x0200 + x)) = GAME_MEM8((u16)(0xFF6B + x));
    for (x = 4; x <= 0xFF; x++)
        GAME_MEM8((u16)(0x0200 + x)) = 0xF8;
    r->index = 0x00;
}
