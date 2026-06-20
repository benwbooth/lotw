
#include "game_memory.h"
#include "routine_context.h"

void routine_0186(RoutineContext *r)
{
    r->index = GAME_MEM8(0xF9) & 0x1F;
}
