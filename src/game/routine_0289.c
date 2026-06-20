









#include "game_memory.h"
#include "routine_context.h"

void routine_0289(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    u8 a, y;

    if (--GAME_MEM8((0x9E + x) & 0xFF) != 0) {
        r->index = x;
        r->carry = 0;
        return;
    }
    a = GAME_MEM8((0x9B + x) & 0xFF) & 0x0F;
    if (a >= 0x0C) {
        r->index = x;
        r->value = a;
        r->carry = 1;
        return;
    }

    y = (u8)(GAME_MEM8((0x9B + x) & 0xFF) + 0x04);
    GAME_MEM8((0x9B + x) & 0xFF) = y;
    GAME_MEM8((0x9C + x) & 0xFF) = GAME_MEM8((u16)(0xFDCB + y));
    GAME_MEM8((0x9D + x) & 0xFF) = GAME_MEM8((u16)(0xFDCC + y));
    GAME_MEM8((0x9E + x) & 0xFF) = GAME_MEM8((u16)(0xFDCD + y));
    r->index = x;
    r->offset = y;
    r->carry = 0;
}
