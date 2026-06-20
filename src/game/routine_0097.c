






















#include "game_memory.h"
#include "routine_context.h"

#define STK_SLOT 0x01FB

void routine_0102(RoutineContext *r);

void routine_0097(RoutineContext *r)
{
    u8 x, y, a;
    int i;


    a = r->index;
    GAME_MEM8(STK_SLOT) = a;


    x = a;
    for (i = 0; i < 5; i++)
        GAME_MEM8((u16)(0x0101 + x++)) = 0xDC;


    a = GAME_MEM8(STK_SLOT);
    GAME_MEM8(STK_SLOT) = a;
    x = a;


    for (i = 0; i < 5; i++)
        GAME_MEM8((u16)(0x0121 + x++)) = 0xDF;


    a = GAME_MEM8(STK_SLOT);
    x = a;

    r->index = x;
    routine_0102(r);
    y = r->offset;

    a = x;


    x = a;
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        GAME_MEM8((u16)(0x0101 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        GAME_MEM8((u16)(0x0101 + x))--;
        x = (u8)(x + 1);
    }


    x = a;
    y = GAME_MEM8(0x08);
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        GAME_MEM8((u16)(0x0121 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        GAME_MEM8((u16)(0x0121 + x))--;
        x = (u8)(x + 1);
    }

    r->offset = y;
    r->index = x;
    r->value = a;
}
