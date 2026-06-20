







#include "game_memory.h"
#include "routine_context.h"

void routine_0113(RoutineContext *r);
void routine_0112(RoutineContext *r);

void routine_0111(RoutineContext *r)
{
    GAME_MEM8(0xEA) = 0x00;

    routine_0113(r);
    if (r->carry == 0) return;

    routine_0112(r);
    if (r->carry == 0) return;

    GAME_MEM8(0xEA) = 0x01;
    r->carry = 1;
}
