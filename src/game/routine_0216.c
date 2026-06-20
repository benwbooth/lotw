












#include "game_memory.h"
#include "routine_context.h"

void routine_0216(RoutineContext *r)
{
    u8 t = (u8)(GAME_MEM8(0xF3) - 1);
    GAME_MEM8(0xF3) = t;

    if (t == 0) {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        GAME_MEM8(0xEE) = 0x01;
        GAME_MEM8(0xED) = GAME_MEM8(ptr);
        GAME_MEM8(0xEF) = GAME_MEM8((u16)(ptr + 1));
    } else if ((t & 0x03) == 0) {
        GAME_MEM8(0xEF) ^= 0x40;
    }
}
