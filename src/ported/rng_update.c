/* $CC64 rng_update — 16-bit shift/add LCG over state $39-$3B, iterated until the
 * result < the requested modulus (A on entry). Result -> rng_s2 ($3B), also A.
 * Relies on the caller's LDA setting Z for the count==0 early-out. */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r)
{
    u8 count = r->a, x, y, a;

    rng_count = count;
    if (count == 0) {            /* BEQ: Z from caller's LDA count */
        r->a = rng_s2;
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
    r->a = a;
}
