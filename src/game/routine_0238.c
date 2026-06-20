

#include "game_memory.h"
#include "routine_context.h"

void routine_0238(RoutineContext *r)
{
    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
    r->value = GAME_MEM8(0x0A);
}
