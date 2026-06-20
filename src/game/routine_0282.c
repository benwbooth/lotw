

#include "game_memory.h"
#include "routine_context.h"

void routine_0282(RoutineContext *r)
{
    GAME_MEM8((0x9A + r->index) & 0xFF) = r->value;
}
