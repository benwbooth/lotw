






#include "game_memory.h"
#include "routine_context.h"

void routine_0263(RoutineContext *r)
{
    u8 y = 0x00;
    if (GAME_MEM8(0xF6) & 0x80) {

    } else if (GAME_MEM8(0xF5) == 0) {
        return;
    } else {
        y = 0x40;
    }
    GAME_MEM8(0x08) = y;
    GAME_MEM8(0xEF) = (u8)((GAME_MEM8(0xEF) & 0x3F) | y);
}
