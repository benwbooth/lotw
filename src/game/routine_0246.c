






#include "game_memory.h"
#include "routine_context.h"

void routine_0246(RoutineContext *r)
{
    u8 t;
    if (GAME_MEM8(0xF5) != 0) {
        u8 y = (GAME_MEM8(0xF6) & 0x80) ? 0x00 : 0x40;
        GAME_MEM8(0x08) = y;
        GAME_MEM8(0xEF) = (u8)((GAME_MEM8(0xEF) & 0x3F) | y);
    }
    GAME_MEM8(0xF3)++;
    t = (u8)((GAME_MEM8(0xF3) & 0x06) << 1);
    GAME_MEM8(0x08) = t;
    GAME_MEM8(0xED) = (u8)((GAME_MEM8(0xED) & 0xF3) | t);
}
