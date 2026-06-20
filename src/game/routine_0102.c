







#include "game_memory.h"
#include "routine_context.h"

void routine_0102(RoutineContext *r)
{
    u8 a = GAME_MEM8(0x08);
    u8 y = 0;
    u8 carry = 1;

    do {
        int t;
        y = (u8)(y + 1);
        t = (int)a - 0x0A - (1 - carry);
        a = (u8)t;
        carry = (t >= 0) ? 1 : 0;
    } while (carry);


    a = (u8)(a + 0x0B + carry);
    GAME_MEM8(0x08) = a;
    r->value = a;
    r->offset = y;
}
