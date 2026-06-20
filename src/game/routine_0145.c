









#include "game_memory.h"
#include "routine_context.h"

void routine_0145(RoutineContext *r)
{
    if (GAME_MEM8(0x46) == 0) {
        if (GAME_MEM8(0x56) < 0x20) {

            if (GAME_MEM8(0x20) & 0x40)
                GAME_MEM8(0x56) = (u8)(GAME_MEM8(0x56) | 0x10);
            else
                GAME_MEM8(0x56) = (u8)(GAME_MEM8(0x56) & 0xEF);
        }
    }

    if ((GAME_MEM8(0x20) & 0x0F) == 0)
        return;
    if ((GAME_MEM8(0x4F) | GAME_MEM8(0x4E)) != 0)
        return;
    GAME_MEM8(0x4D) = (u8)(GAME_MEM8(0x4D) + 1);
    if ((GAME_MEM8(0x4D) & 0x07) != 0)
        return;
    if (GAME_MEM8(0x56) & 0x08)
        GAME_MEM8(0x57) = (u8)(GAME_MEM8(0x57) ^ 0x40);
    else
        GAME_MEM8(0x56) = (u8)(GAME_MEM8(0x56) ^ 0x04);
}
