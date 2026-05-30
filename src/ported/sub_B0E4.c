/* $B0E4:  LDA #$04 / JSR rng_update / TAX / LDA $B0FE,X / STA $20
 *         LDA #$0A / JSR rng_update / TAX / BNE + / LDA $20 / ORA #$40 / STA $20 / RTS
 * Picks a random table entry $B0FE+rand(4) into $20; 1-in-10 chance ORs in $40. */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

void sub_B0E4(Regs *r)
{
    r->a = 0x04;
    rng_update(r);
    r->x = r->a;
    RAM8(0x20) = RAM8((u16)(0xB0FE + r->x));

    r->a = 0x0A;
    rng_update(r);
    r->x = r->a;
    if (r->x == 0)
        RAM8(0x20) = (u8)(RAM8(0x20) | 0x40);
}
