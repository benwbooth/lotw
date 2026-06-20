





#include "game_memory.h"
#include "routine_context.h"

void routine_0023(RoutineContext *r);
void routine_0031(RoutineContext *r);

void routine_0027(RoutineContext *r)
{
    u8 saved = GAME_MEM8(0x4B);

    for (;;) {
        routine_0023(r);
        routine_0031(r);
        if (!r->carry)
            break;
        {
            u8 x = GAME_MEM8(0x4B);
            if (x == 0) {
                r->carry = 1;
                break;
            }
            if (!(x & 0x80)) {
                x = (u8)(x - 1);
                x = (u8)(x - 1);
            }
            x = (u8)(x + 1);
            GAME_MEM8(0x4B) = x;
            if (x != 0)
                continue;
            r->carry = 1;
            break;
        }
    }

    GAME_MEM8(0x4B) = saved;
}
