


#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

void routine_0035(RoutineContext *r)
{
    r->value = 0x04;
    rng_update(r);
    r->index = r->value;
    GAME_MEM8(0x20) = GAME_MEM8((u16)(0xB0FE + r->index));

    r->value = 0x0A;
    rng_update(r);
    r->index = r->value;
    if (r->index == 0)
        GAME_MEM8(0x20) = (u8)(GAME_MEM8(0x20) | 0x40);
}
