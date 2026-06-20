









#include "game_memory.h"
#include "routine_context.h"

void routine_0124(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x55);
    u8 item = GAME_MEM8((0x51 + x) & 0xFF);
    r->index = x;
    if (item == 0x06 && magic != 0) {
        u8 jump = GAME_MEM8(0x5C);
        r->value = (u8)((jump >> 2) + jump);
        r->carry = 0;
    } else {
        r->value = GAME_MEM8(0x5C);
        r->carry = 1;
    }
}
