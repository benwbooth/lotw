





#include "game_memory.h"
#include "routine_context.h"

void routine_0166(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    u8 x = GAME_MEM8((u16)(ptr + r->offset)) & 0x3F;

    if (x == 0) {
        if (GAME_MEM8(0x43) == 0)
            r->carry = 1;
        else
            r->carry = 0;
    } else if (x == 0x02) {
        r->carry = 1;
    } else {
        r->carry = (u8)(x >= 0x30);
    }
}
