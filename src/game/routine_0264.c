

#include "game_memory.h"
#include "routine_context.h"

void routine_0264(RoutineContext *r)
{
    u8 a;
    a = ++GAME_MEM8(0xF3);
    a = (u8)(((a & 0x0C) << 1) | 0x41);
    GAME_MEM8(0xED) = a;
    r->value = a;
}
