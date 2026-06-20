











#include "game_memory.h"
#include "routine_context.h"

void routine_0089(RoutineContext *r)
{
    u8 msy = GAME_MEM8(0x48);
    u8 x   = (u8)((msy >> 1) + 1);
    u8 a   = 0xFF;
    u8 carry = 0;
    u8 idx;

    do {
        u8 newcarry = a & 1;
        a = (u8)((carry << 7) | (a >> 1));
        carry = newcarry;
        x--;
    } while (x != 0);

    idx = (u8)((((u8)(msy << 2)) & 0x04) | GAME_MEM8(0x47));
    GAME_MEM8((u16)(0x0300 + idx)) &= a;

    r->value = GAME_MEM8((u16)(0x0300 + idx));
    r->index = idx;
}
