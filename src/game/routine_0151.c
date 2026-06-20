


#include "game_memory.h"
#include "routine_context.h"

void routine_0205(RoutineContext *r);

void routine_0151(RoutineContext *r)
{
    GAME_MEM8(0x8F) = 0x1E;
    r->value = 0x05;
    routine_0205(r);
}
