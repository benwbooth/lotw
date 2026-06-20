










#include "game_memory.h"
#include "routine_context.h"

void routine_0201(RoutineContext *r)
{
    int x;
    for (x = 0x37; x >= 0; x--)
        GAME_MEM8((u16)(0x0280 + x)) = GAME_MEM8((u16)(0xFF6F + x));

    GAME_MEM8(0x2C) = 0x34;
    GAME_MEM8(0x2D) = 0x35;
    GAME_MEM8(0x2E) = 0x36;
    GAME_MEM8(0x2F) = 0x37;

    r->index = 0xFF;
    r->value = 0x37;
}
