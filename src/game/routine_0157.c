

#include "game_memory.h"
#include "routine_context.h"

void routine_0210(RoutineContext *r);

void routine_0157(RoutineContext *r)
{
    GAME_MEM8(0x8F) = 0x15;
    r->value = 0x14;
    routine_0210(r);
}
