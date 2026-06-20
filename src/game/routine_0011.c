


#include "game_memory.h"
#include "routine_context.h"

void routine_0011(RoutineContext *r)
{
    GAME_MEM8(0x0E) = GAME_MEM8(0xF9);
    GAME_MEM8(0x0A) = GAME_MEM8(0xFB);
    if (GAME_MEM8(0xF7) != 0)
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0xF7) + GAME_MEM8(0x0A));
    if (GAME_MEM8(0xF5) != 0)
        GAME_MEM8(0x0E) = (u8)(GAME_MEM8(0xF5) + GAME_MEM8(0x0E));
    (void)r;
}
