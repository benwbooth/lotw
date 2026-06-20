



#include "game_memory.h"
#include "routine_context.h"

void routine_0094(RoutineContext *r);

void routine_0204(RoutineContext *r)
{
    u8 saved_x = r->index;
    r->value = magic;
    r->carry = 1;
    if (magic != 0) {
        magic = (u8)(magic - 1);
        routine_0094(r);
        r->carry = 0;
    }
    r->index = saved_x;
}
