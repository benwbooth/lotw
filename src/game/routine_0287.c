









#include "game_memory.h"
#include "routine_context.h"

void routine_0287(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    u8 hi;
    GAME_MEM8((0x95 + x) & 0xFF) = GAME_MEM8((0x97 + x) & 0xFF);
    hi = GAME_MEM8((0x98 + x) & 0xFF);
    GAME_MEM8((0x96 + x) & 0xFF) = hi;
    if (hi != 0) {
        GAME_MEM8((0x93 + x) & 0xFF) = 0x01;
    } else {
        GAME_MEM8((0x94 + x) & 0xFF) &= 0x40;
    }
    r->index = x;
}
