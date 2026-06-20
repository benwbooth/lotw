










#include "game_memory.h"
#include "routine_context.h"

void routine_0126(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x55);
    r->index = x;
    if (GAME_MEM8((0x51 + x) & 0xFF) == 0x09 &&
        GAME_MEM8(0x59) != 0) {
        r->value = (u8)(GAME_MEM8(0x5F) << 1);
        r->carry = 0;
        return;
    }
    r->value = GAME_MEM8(0x5F);
    r->carry = 1;
}
