














#include "game_memory.h"
#include "routine_context.h"

void routine_0108(RoutineContext *r)
{
    u8 y = r->offset;
    u8 x, a, c, sign_fill;

    GAME_MEM8(0x09) = y;
    if (y == 0) {
        GAME_MEM8(0xF5) = 0;
        GAME_MEM8(0xF6) = 0;
        GAME_MEM8(0xF7) = 0;
        return;
    }

    x = (u8)((r->value & 0x0F) << 1);

    a = 0;
    for (c = y; c != 0; c--)
        a = (u8)(a + GAME_MEM8(0xFE8B + x));

    GAME_MEM8(0xF5) = a & 0x0F;
    sign_fill = (a & 0x80) ? 0xF0 : 0x00;
    GAME_MEM8(0x08) = sign_fill;
    GAME_MEM8(0xF6) = (u8)(((a & 0xF0) >> 4) | sign_fill);

    a = 0;
    for (c = y; c != 0; c--)
        a = (u8)(a + GAME_MEM8(0xFE8C + x));
    GAME_MEM8(0xF7) = a;
}
