



#include "game_memory.h"
#include "routine_context.h"

void routine_0030(RoutineContext *r)
{
    u8 lhs, res;
    GAME_MEM8(0x08) = r->value;
    lhs = GAME_MEM8(0x58);

    {
        u16 t = (u16)lhs - (u16)GAME_MEM8(0x08);
        res = (u8)t;

        r->carry = (t & 0x100) ? 0 : 1;
        r->zero = (res == 0) ? 1 : 0;
        r->negative = (res >> 7) & 1;
    }
    GAME_MEM8(0x58) = res;
    if (!r->carry) {
        GAME_MEM8(0x58) = 0x00;
    }

}
