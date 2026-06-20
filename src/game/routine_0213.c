

#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0xE5) | (GAME_MEM8(0xE6) << 8));
    int y;
    for (y = 0x0F; y >= 0; --y)
        GAME_MEM8((u16)(0x00ED + y)) = GAME_MEM8((u16)(ptr + y));
    r->offset = 0xFF;
}
