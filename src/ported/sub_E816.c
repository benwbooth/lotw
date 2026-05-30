/* $E816:  CLC / ADC magic / BCC L_E820 / LDA #$6D / JMP store
 *         L_E820: CMP #$6E / BCC store / LDA #$6D / store: STA magic / JSR L_CACC / RTS
 * Adds A to magic ($59), clamping to $6D on overflow or when result >= $6E.
 * Then updates magic bar via sub_CACC. */
#include "ram.h"
#include "regs.h"

void sub_CACC(Regs *r);

void sub_E816(Regs *r)
{
    u16 sum = (u16)r->a + magic;      /* CLC then ADC */
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    magic = v;
    sub_CACC(r);                      /* reads magic from RAM */
}
