/* Random number generator — port of rng_update ($CC64).
 *
 * The original is a 16-bit shift/add LCG over the 24-bit state $39-$3B, iterated
 * until the result is < the requested modulus ($38). Each round:
 *   (s2:s1_working) <- ((s2:s1_working) << 1) + 1
 *   then folded with the previous s1/s2 and the saved working byte, masked &$7F.
 * Result lands in rng_s2 ($3B). Behaviourally identical to the asm (verified by
 * differential test vs the m6502 oracle).
 */
#include "ram.h"
#include "rng.h"

u8 rng_update(u8 count)
{
    u8 x, y, a;

    rng_count = count;
    if (count == 0)
        return rng_s2;            /* BEQ done: state untouched */

    x = rng_s2;                   /* working high */
    y = rng_s1;                   /* working low */
    do {
        u16 xy;
        u16 s;
        u8 carry;

        rng_s0 = y;               /* save working low */

        xy = (u16)((x << 8) | y); /* (x:y) <- ((x:y) << 1) + 1 */
        xy = (u16)((xy << 1) + 1);
        x = (u8)(xy >> 8);
        y = (u8)xy;

        s = (u16)(y + rng_s1);    /* y += s1 (carry out) */
        y = (u8)s;
        carry = (u8)(s >> 8);

        a = (u8)(x + rng_s2 + carry);  /* a = x + s2 + carry */
        a = (u8)(a + rng_s0);          /* a += saved low (CLC, no carry in) */
        a &= 0x7F;

        x = a;
        rng_s2 = a;
        rng_s1 = y;
    } while (a >= count);         /* CMP/BCS: repeat while a >= count */

    return a;
}
