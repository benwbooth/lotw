












#include "game_memory.h"
#include "routine_context.h"

void routine_0107(RoutineContext *r)
{
    u8 y = r->offset;
    u8 x, a, c, sign_fill;

    GAME_MEM8(0x09) = y;
    if (y == 0) {
        GAME_MEM8(0x49) = 0;
        GAME_MEM8(0x4A) = 0;
        GAME_MEM8(0x4B) = 0;
        return;
    }

    x = (u8)((GAME_MEM8(0x20) & 0x0F) << 1);

    a = 0;
    for (c = y; c != 0; c--)
        a = (u8)(a + GAME_MEM8(0xFE8B + x));

    GAME_MEM8(0x49) = a & 0x0F;
    sign_fill = (a & 0x80) ? 0xF0 : 0x00;
    GAME_MEM8(0x08) = sign_fill;
    GAME_MEM8(0x4A) = (u8)(((a & 0xF0) >> 4) | sign_fill);

    a = 0;
    for (c = y; c != 0; c--)
        a = (u8)(a + GAME_MEM8(0xFE8C + x));
    GAME_MEM8(0x4B) = a;
}
