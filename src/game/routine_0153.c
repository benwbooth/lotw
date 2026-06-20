

#include "game_memory.h"
#include "routine_context.h"

void routine_0207(RoutineContext *r);

void routine_0153(RoutineContext *r)
{
    GAME_MEM8(0x8F) = 0x11;
    r->value = 0x02;
    routine_0207(r);
}
