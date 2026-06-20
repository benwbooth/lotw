



#include "game_memory.h"
#include "routine_context.h"

void routine_0093(RoutineContext *r);

void routine_0202(RoutineContext *r)
{
    r->value = health;
    if (r->value == 0) {
        r->carry = 1;
        return;
    }
    health = (u8)(health - 1);
    routine_0093(r);
    r->carry = 0;
}
