


#include "game_memory.h"
#include "routine_context.h"

void routine_0101(RoutineContext *r);

void routine_0098(RoutineContext *r)
{
    u8 a = GAME_MEM8(0x0405);
    if (a >= 0x6D) a = 0x6D;
    GAME_MEM8(0x08) = a;
    GAME_MEM8(0x09) = 0x00;
    r->index = 0xA5;
    r->offset = 0xAB;
    routine_0101(r);
}
