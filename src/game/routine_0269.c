










#include "game_memory.h"
#include "routine_context.h"

void routine_0269(RoutineContext *r)
{
    GAME_MEM8(0x0E) = GAME_MEM8(0x43);
    GAME_MEM8(0x0F) = GAME_MEM8(0x44);
    GAME_MEM8(0x0A) = GAME_MEM8(0x45);

    if (GAME_MEM8(0xF7) != 0) {
        u8 a = (u8)(GAME_MEM8(0xF7) << 2);
        a = (u8)(a + GAME_MEM8(0x0A));
        GAME_MEM8(0x0A) = a;
    }

    if (GAME_MEM8(0xF5) != 0) {
        u8 pulled = (u8)((u8)((GAME_MEM8(0xF5) << 2) & 0x0F) + GAME_MEM8(0x0E));
        GAME_MEM8(0x0E) = pulled & 0x0F;

        GAME_MEM8(0x0F) = (u8)(GAME_MEM8(0x0F) + GAME_MEM8(0xF6) + ((pulled >> 4) & 1));
    }
}
