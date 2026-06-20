



#include "game_memory.h"
#include "routine_context.h"

void sound_set_default_banks(RoutineContext *r)
{
    u8 x = 0x06, y = 0x0A;
    REG_W(0x8000, x);
    REG_W(0x8001, y);
    x = (u8)(x + 1);
    y = (u8)(y + 1);
    REG_W(0x8000, x);
    REG_W(0x8001, y);
    r->index = x;
    r->offset = y;
}
