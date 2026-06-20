






#include "game_memory.h"
#include "routine_context.h"

void routine_0028(RoutineContext *r)
{
    if (GAME_MEM8(0x4F) != 0) {
        r->carry = 0;
        return;
    }

    if (GAME_MEM8(0x45) < 0xA0) {
        GAME_MEM8(0x4E) = (u8)(GAME_MEM8(0x4E) + 1);
        return;
    }

    {
        u8 a = GAME_MEM8(0x4E);
        if (a >= GAME_MEM8(0x5C)) {
            a = (u8)(a - 0x07);
            if (a >= GAME_MEM8(0x5C))
                a = GAME_MEM8(0x5C);

            a = (u8)(a - 0x01);
            GAME_MEM8(0x4F) = a;
            GAME_MEM8(0x8F) = 0x0A;
        }
    }

    GAME_MEM8(0x4E) = 0x00;
}
