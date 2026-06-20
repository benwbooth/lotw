







#include "game_memory.h"
#include "routine_context.h"

void routine_0184(RoutineContext *r)
{
    u8 a = GAME_MEM8(0xF9) & 0x1F;
    u8 x = 0x61;
    u8 base;
    u16 v;
    u8 carry, res;

    if (a >= 0x10) {
        a = (u8)(a - 0x10);
        x = 0x69;
    }
    GAME_MEM8(0x0280) = x;
    GAME_MEM8(0x0284) = x;
    GAME_MEM8(0x08) = a;

    base = (u8)((a >> 2) + a);
    v = (u8)(base << 3);
    carry = (u8)((base >> 5) & 1);
    res = (u8)((u8)v + 0x36 + carry);
    GAME_MEM8(0x0287) = res;
    res = (u8)(res - 0x08);
    GAME_MEM8(0x0283) = res;

    r->index = x;
    r->value = res;
}
