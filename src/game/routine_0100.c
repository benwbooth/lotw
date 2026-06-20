



#include "game_memory.h"
#include "routine_context.h"

void routine_0101(RoutineContext *r);

void routine_0100(RoutineContext *r)
{
    u8 a = GAME_MEM8(0x58);
    if (a >= 0x6D) a = 0x6D;
    GAME_MEM8(0x08) = a;
    GAME_MEM8(0x09) = 0x80;
    r->index = 0x65;
    r->offset = 0x6B;
    routine_0101(r);
}
