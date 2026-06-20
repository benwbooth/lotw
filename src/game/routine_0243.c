



#include "game_memory.h"
#include "routine_context.h"

void routine_0243(RoutineContext *r)
{
    u8 a;
    GAME_MEM8(0xF3)++;
    a = GAME_MEM8(0xF3) & 0x03;
    if (a == 0) {
        a = GAME_MEM8(0xEF) ^ 0x40;
        GAME_MEM8(0xEF) = a;
    }
    r->value = a;
}
