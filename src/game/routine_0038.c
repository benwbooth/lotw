


#include "game_memory.h"
#include "routine_context.h"

void routine_0038(RoutineContext *r)
{
    u8 x = 0xEF;
    if (GAME_MEM8(0x84) & 0x30)
        x = 0x80;
    GAME_MEM8(0x0240) = x;
    GAME_MEM8(0x0244) = x;
    GAME_MEM8(0x0248) = x;
    GAME_MEM8(0x024C) = x;
    GAME_MEM8(0x0250) = x;
    GAME_MEM8(0x0254) = x;
    GAME_MEM8(0x0258) = x;
    GAME_MEM8(0x025C) = x;
    r->index = x;
}
