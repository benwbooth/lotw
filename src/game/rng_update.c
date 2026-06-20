


#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r)
{
    u8 count = r->value, x, y, a;

    rng_count = count;
    if (count == 0) {
        r->value = rng_s2;
        return;
    }
    x = rng_s2;
    y = rng_s1;
    do {
        u16 xy, s;
        u8 carry;
        rng_s0 = y;
        xy = (u16)(((u16)((x << 8) | y) << 1) + 1);
        x = (u8)(xy >> 8);
        y = (u8)xy;
        s = (u16)(y + rng_s1);
        y = (u8)s;
        carry = (u8)(s >> 8);
        a = (u8)(x + rng_s2 + carry);
        a = (u8)(a + rng_s0);
        a &= 0x7F;
        x = a;
        rng_s2 = a;
        rng_s1 = y;
    } while (a >= count);
    r->value = a;
}
