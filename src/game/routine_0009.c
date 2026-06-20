

#include "game_memory.h"
#include "routine_context.h"

void routine_0009(RoutineContext *r)
{
    GAME_MEM8(0x08) = (u8)(GAME_MEM8(0xEE) & 0x0C);
    GAME_MEM8(0xED) = (u8)((GAME_MEM8(0xED) & 0xF3) | GAME_MEM8(0x08));
    (void)r;
}
