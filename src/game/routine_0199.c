
#include "game_memory.h"
#include "routine_context.h"

void routine_0199(RoutineContext *r)
{
    GAME_MEM8(0x0250) = 0x98;
    GAME_MEM8(0x0254) = 0x98;
    GAME_MEM8(0x0251) = 0xF1;
    GAME_MEM8(0x0255) = 0xF3;
    GAME_MEM8(0x0252) = 0x02;
    GAME_MEM8(0x0256) = 0x02;
    GAME_MEM8(0x0253) = 0x78;
    GAME_MEM8(0x0257) = 0x80;
    r->value = 0x80;
}
