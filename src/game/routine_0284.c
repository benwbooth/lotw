




#include "game_memory.h"
#include "routine_context.h"

void routine_0284(RoutineContext *r)
{
    u8 a = 0x00;
    u8 y = (u8)(r->offset + 1);
    do {
        a = (u8)(a + GAME_MEM8(0x00));
        y = (u8)(y - 1);
    } while (y != 0);
    a >>= 4;
    GAME_MEM8(0x00) = a;
    r->value = a;
    r->offset = 0;
}
