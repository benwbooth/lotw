


#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);


static const u8 sound_lookup_eeb3[8] = {
    0x01, 0x05, 0x04, 0x06, 0x02, 0x0A, 0x08, 0x09
};

void routine_0235(RoutineContext *r)
{
    u8 x;
    r->value = 0x03;
    rng_update(r);
    x = (u8)(r->value << 1);
    GAME_MEM8(0xF4) = sound_lookup_eeb3[x];
}
