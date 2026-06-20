



#include "game_memory.h"
#include "routine_context.h"

void routine_0060(RoutineContext *r)
{
    u8 a = GAME_MEM8(0x7C);
    u8 carry = 0;
    int i;
    for (i = 0; i < 4; i++) {
        carry = (u8)(a >> 7);
        a = (u8)(a << 1);
    }
    a |= GAME_MEM8(0x7B);
    r->index = a;
    a = (u8)(0x00 << 1) | carry;
    GAME_MEM8(0x1C) = r->index;
    GAME_MEM8(0x1D) = a;
    r->value = a;
}
