










#include "game_memory.h"
#include "routine_context.h"

void routine_0128(RoutineContext *r)
{
    u8 x = 0x00;
    u8 y = 0x10;
    do {
        GAME_MEM8((u16)(0x0401 + x)) = 0x00;
        GAME_MEM8((u16)(0x0406 + x)) = 0x02;
        x = (u8)(x + 0x10);
    } while (--y != 0);
    GAME_MEM8(0xE9) = 0x00;
    r->value = 0x00;
    r->index = x;
    r->offset = 0x00;
}
