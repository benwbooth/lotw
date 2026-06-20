





#include "game_memory.h"
#include "routine_context.h"

void routine_0025(RoutineContext *r)
{
    if (GAME_MEM8(0x56) < 0x20) {
        u8 a = GAME_MEM8(0x56);
        if (GAME_MEM8(0x20) & 0x40)
            a = (u8)(a | 0x10);
        else
            a = (u8)(a & 0xEF);
        GAME_MEM8(0x56) = a;
    }

    if ((GAME_MEM8(0x20) & 0x0F) == 0) return;
    if ((GAME_MEM8(0x4F) | GAME_MEM8(0x4E)) != 0) return;
    GAME_MEM8(0x4D) = (u8)(GAME_MEM8(0x4D) + 1);
    if ((GAME_MEM8(0x4D) & 0x07) != 0) return;
    if (GAME_MEM8(0x56) & 0x08) {
        GAME_MEM8(0x57) ^= 0x40;
    } else {
        GAME_MEM8(0x56) ^= 0x04;
    }
}
