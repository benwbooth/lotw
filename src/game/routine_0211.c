



#include "game_memory.h"
#include "routine_context.h"

void routine_0095(RoutineContext *r);

void routine_0211(RoutineContext *r)
{
    r->value = keys;
    if (r->value == 0) {
        r->carry = 1;
        return;
    }
    keys = (u8)(keys - 1);
    routine_0095(r);
    r->carry = 0;
}
