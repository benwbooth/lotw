




#include "game_memory.h"
#include "routine_context.h"

void routine_0044(RoutineContext *r);

void routine_0043(RoutineContext *r)
{
    for (;;) {
        GAME_MEM8(0x0A)++;
        if ((GAME_MEM8(0x0A) & 0x07) == 0)
            break;
        r->value = 0xFF;
        routine_0044(r);
    }

    if (GAME_MEM8(0x0A) == 0xF0)
        GAME_MEM8(0x0A) = 0x00;
}
