



#include "game_memory.h"
#include "routine_context.h"

void routine_0121(RoutineContext *r)
{
    u8 a = r->value;
    u8 hi = 0xD0;

    while (a >= 0x0A) {
        a = (u8)(a - 0x0A);
        ++hi;
    }
    a = (u8)(a + 0xD0);
    GAME_MEM8(0x18) = a;
    if (hi == 0xD0)
        hi = 0xC0;
    GAME_MEM8(0x19) = hi;
}
