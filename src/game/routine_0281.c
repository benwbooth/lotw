

#include "game_memory.h"
#include "routine_context.h"

void routine_0281(RoutineContext *r)
{
    GAME_MEM8((0xA1 + r->index) & 0xFF) = r->value;
}
