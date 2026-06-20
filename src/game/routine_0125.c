





#include "game_memory.h"
#include "routine_context.h"

void routine_0125(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x55);
    u8 item = GAME_MEM8((0x51 + x) & 0xFF);

    if (item == 0x08 && GAME_MEM8(0x59) != 0) {
        r->value = (u8)(GAME_MEM8(0x5D) << 2);
        r->carry = 0;
    } else {
        r->value = GAME_MEM8(0x5D);
        r->carry = 1;
    }
}
