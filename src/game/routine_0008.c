


#include "game_memory.h"
#include "routine_context.h"

void routine_0008(RoutineContext *r)
{
    GAME_MEM8(0x0E) = GAME_MEM8(0x43);
    GAME_MEM8(0x0A) = GAME_MEM8(0x45);
    if (GAME_MEM8(0xF7) != 0) {
        u8 a = (u8)(GAME_MEM8(0xF7) << 2);
        GAME_MEM8(0x0A) = (u8)(a + GAME_MEM8(0x0A));
    }
    if (GAME_MEM8(0xF5) != 0) {
        u8 a = (u8)(GAME_MEM8(0xF5) << 2);
        GAME_MEM8(0x0E) = (u8)(a + GAME_MEM8(0x0E));
    }
    (void)r;
}
