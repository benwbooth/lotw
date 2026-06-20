

#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

void routine_0234(RoutineContext *r)
{
    r->value = 0x08;
    rng_update(r);
    r->index = r->value;
    GAME_MEM8(0xF4) = GAME_MEM8((u16)(0xEEB3 + r->index));
}
