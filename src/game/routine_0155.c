

#include "game_memory.h"
#include "routine_context.h"

void routine_0203(RoutineContext *r);

void routine_0155(RoutineContext *r)
{
    GAME_MEM8(0x8F) = 0x1D;
    r->value = 0x05;
    routine_0203(r);
}
