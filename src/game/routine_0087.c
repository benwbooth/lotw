








#include "game_memory.h"
#include "routine_context.h"

void routine_0087(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x77) | (GAME_MEM8(0x78) << 8));
    u8 a, x;
    int y;


    for (y = 0xE0; y <= 0xFF; y++)
        GAME_MEM8((u16)(0x00A0 + (u8)y)) = GAME_MEM8((u16)(ptr + (u8)y));

    a = cur_character;
    if (a >= 0x06) {
        r->value = a;
        r->carry = 1;
        return;
    }
    a = (u8)((a << 2) + 0x03);
    x = a;
    for (y = 0x03; y >= 0; y--) {
        GAME_MEM8((u16)(0x0190 + y)) = GAME_MEM8((u16)(0xFFC5 + x));
        x--;
    }
    r->value = a;
    r->index = x;
    r->offset = (u8)0xFF;
    r->carry = 0;
}
