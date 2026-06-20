

#include "game_memory.h"
#include "routine_context.h"

void routine_0095(RoutineContext *r);

void routine_0209(RoutineContext *r)
{
    keys = (u8)(keys + 1);
    routine_0095(r);
    r->carry = 0;
}
