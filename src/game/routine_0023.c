





#include "game_memory.h"
#include "routine_context.h"

void routine_0023(RoutineContext *r)
{
    GAME_MEM8(0x0E) = GAME_MEM8(0x43);
    GAME_MEM8(0x0A) = GAME_MEM8(0x45);

    if (GAME_MEM8(0x4B) != 0) {
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x4B) + GAME_MEM8(0x0A));
    }
    if (GAME_MEM8(0x49) != 0) {
        GAME_MEM8(0x0E) = (u8)(GAME_MEM8(0x49) + GAME_MEM8(0x0E));
    }
}
