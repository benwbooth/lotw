/* $EE9A:  LDA #$08 / JSR rng_update / TAX / LDA $EEB3,X / STA $F4 / RTS
 * Picks a random sound id (0..7) from the table at $EEB3 and stores it to $F4. */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

void sub_EE9A(Regs *r)
{
    r->a = 0x08;
    rng_update(r);          /* A in [0,8), result also in rng_s2 */
    r->x = r->a;
    RAM8(0xF4) = RAM8((u16)(0xEEB3 + r->x));
}
