





#include "game_memory.h"
#include "routine_context.h"

void routine_0200(RoutineContext *r)
{
    GAME_MEM8(0x0240) = 0xEF;
    GAME_MEM8(0x0244) = 0xEF;
    GAME_MEM8(0x0248) = 0xEF;
    GAME_MEM8(0x024C) = 0xEF;
    GAME_MEM8(0x0250) = 0xEF;
    GAME_MEM8(0x0254) = 0xEF;
    r->value = 0xEF;
    (void)r;
}
