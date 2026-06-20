









#include "game_memory.h"
#include "routine_context.h"

void routine_0108(RoutineContext *r);
void routine_0241(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0249(RoutineContext *r);

void routine_0227(RoutineContext *r)
{
    r->offset = 0x01;
    routine_0108(r);

    routine_0241(r);

    routine_0111(r);
    if (r->carry == 0)
        return;

    routine_0249(r);
    r->carry = 1;
}
