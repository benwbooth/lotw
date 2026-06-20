








#include "game_memory.h"
#include "routine_context.h"

void routine_0241(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0249(RoutineContext *r);
void routine_0115(RoutineContext *r);

void routine_0248(RoutineContext *r)
{
    routine_0241(r);

    routine_0111(r);
    if (r->carry) {
        routine_0249(r);
        r->carry = 1;
        return;
    }

    routine_0115(r);
    if (r->carry == 0)
        return;

    GAME_MEM8(0xEE) = 0x00;
    GAME_MEM8(0xF3) = 0xF0;
}
