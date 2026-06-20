













#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

void routine_0217(RoutineContext *r)
{
    u8 x, y;
    u16 e7;

    r->value = 0x1E;
    rng_update(r);
    if (r->value != 0) {
        r->index = r->value;
        return;
    }
    r->index = 0;

    x = 0x03;
    y = 0x03;
    if (GAME_MEM8(0x0402) & 0x40)
        y = 0x13;


    for (;;) {
        GAME_MEM8((u16)(0x00F9 + x)) = GAME_MEM8((u16)(0x040C + y));
        y = (u8)(y - 1);
        if ((x--) == 0)
            break;
    }

    GAME_MEM8(0xF1) = 0x00;
    GAME_MEM8(0xF0) = 0x00;
    GAME_MEM8(0xF4) = 0x00;

    e7 = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
    GAME_MEM8(0xF2) = GAME_MEM8((u16)(e7 + 4));
    GAME_MEM8(0xF8) = GAME_MEM8((u16)(e7 + 5));

    GAME_MEM8(0xEE) = 0x01;
    GAME_MEM8(0xED) = 0x81;

    r->value = 0x04;
    rng_update(r);
    GAME_MEM8(0xEF) = r->value;

    GAME_MEM8(0xF1) = 0x80;

    r->offset = y;
    r->index = x;
}
