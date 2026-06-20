








#include "game_memory.h"
#include "routine_context.h"

void routine_0172(RoutineContext *r)
{
    u8 y = GAME_MEM8(0x0B);
    u16 ptr = (u16)(GAME_MEM8(0x10) | (GAME_MEM8(0x11) << 8));
    u8 b = GAME_MEM8((u16)(ptr + y));
    u8 x = b & 0x3F;

    r->index = x;
    r->offset = y;
    if (x == 0x3E)
        r->value = GAME_MEM8(0x74);
    else
        r->value = b;
}
