




#include "game_memory.h"
#include "routine_context.h"

void routine_0073(RoutineContext *r)
{
    u8 x = r->index, y = r->offset;
    do {
        u8 lo = GAME_MEM8((u16)(0x0180 + x)) & 0x0F;
        GAME_MEM8(0x08) = lo;
        u8 hi = GAME_MEM8((u16)(0x0180 + x)) & 0xF0;
        u8 sub = GAME_MEM8(0x09);
        u8 res;
        if (hi >= sub)
            res = (u8)((u8)(hi - sub) | lo);
        else
            res = 0x0F;
        GAME_MEM8((u16)(0x0180 + x)) = res;
        ++x;
        --y;
    } while (y != 0);
    r->index = x;
    r->offset = y;
}
