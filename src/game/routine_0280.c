

#include "game_memory.h"
#include "routine_context.h"

void routine_0280(RoutineContext *r)
{
    GAME_MEM8((0x99 + r->index) & 0xFF) = r->value;
}
