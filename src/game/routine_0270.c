




#include "game_memory.h"
#include "routine_context.h"

void routine_0270(RoutineContext *r)
{
    u8 bits = GAME_MEM8(0xEE) & 0x0C;
    GAME_MEM8(0x08) = bits;
    GAME_MEM8(0xED) = (u8)((GAME_MEM8(0xED) & 0xF3) | bits);
    r->value = GAME_MEM8(0xED);
}
