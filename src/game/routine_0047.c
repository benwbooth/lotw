


#include "game_memory.h"
#include "routine_context.h"

void routine_0047(RoutineContext *r)
{
    u8 x = 0x00;
    do {
        GAME_MEM8((u16)(0x0200 + x)) = 0xEF;
        x = (u8)(x + 4);
    } while (x != 0);
    r->index = x;
    r->value = 0xEF;
}
