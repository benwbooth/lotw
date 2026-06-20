





#include "game_memory.h"
#include "routine_context.h"

void routine_0147(RoutineContext *r)
{
    if (GAME_MEM8(0x2D) < 0x30 && GAME_MEM8(0x87) != 0 && GAME_MEM8(0x59) != 0) {
        u8 x = GAME_MEM8(0x09);
        GAME_MEM8((u16)(0x0401 + x)) = 0x80;
    }
    (void)r;
}
