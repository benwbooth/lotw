






#include "game_memory.h"
#include "routine_context.h"


static u8 asl3(u8 v, u8 *carry_out)
{
    u8 c = 0;
    int i;
    for (i = 0; i < 3; i++) { c = (v >> 7) & 1; v = (u8)(v << 1); }
    *carry_out = c;
    return v;
}

void routine_0185(RoutineContext *r)
{
    u8 c, a, t;

    t = asl3(GAME_MEM8(0xF5), &c);
    a = (u8)(t + 0x36 + c);
    GAME_MEM8(0x0297) = a;
    a = (u8)(a - 0x08);
    GAME_MEM8(0x0293) = a;

    t = asl3(GAME_MEM8(0xF7), &c);
    a = (u8)(t + 0x81 + c);
    GAME_MEM8(0x0290) = a;
    GAME_MEM8(0x0294) = a;

    r->value = a;
}
