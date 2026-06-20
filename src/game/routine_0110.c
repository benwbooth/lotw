





#include "game_memory.h"
#include "routine_context.h"

void routine_0110(RoutineContext *r)
{
    int y = 0x0A;
    u8 x = 0xA0;
    u8 d;

    for (;;) {
        if ((u8)y == GAME_MEM8(0xE3))
            goto skip;
        if (GAME_MEM8((u16)(0x0401 + x)) == 0)
            goto skip;
        if (GAME_MEM8((u16)(0x0401 + x)) & 0x80)
            goto skip;
        if ((GAME_MEM8((u16)(0x0400 + x)) & 0xF9) == 0xE1)
            goto skip;
        if (GAME_MEM8((u16)(0x0402 + x)) & 0x20)
            goto skip;

        d = (u8)(GAME_MEM8(0x0A) - GAME_MEM8((u16)(0x040E + x)));
        if (!(d < 0x10)) {
            if (d < 0xF1)
                goto skip;
        }
        d = (u8)(GAME_MEM8(0x0F) - GAME_MEM8((u16)(0x040D + x)));
        if (d == 0)
            goto hit;
        if (d < 0x02) {
            d = (u8)(GAME_MEM8(0x0E) - GAME_MEM8((u16)(0x040C + x)));
            if (d & 0x80)
                goto hit;
            goto skip;
        }
        if (d < 0xFF)
            goto skip;
        d = (u8)(GAME_MEM8(0x0E) - GAME_MEM8((u16)(0x040C + x)));
        if (d == 0)
            goto skip;
        if (d & 0x80)
            goto skip;
        goto hit;

    skip:
        x = (u8)(x - 0x10);
        --y;
        if (y < 0)
            break;
    }
    r->carry = 0;
    return;

hit:
    GAME_MEM8(0x08) = (u8)y;
    GAME_MEM8(0x09) = x;
    r->carry = 1;
}
