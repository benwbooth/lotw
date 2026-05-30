/* $E859:  CLC / ADC keys / BCC L_E863 / LDA #$6D / JMP store
 *         L_E863: CMP #$6E / BCC store / LDA #$6D / store: STA keys / JSR L_CAE2 / RTS
 * Adds A to keys ($5B), clamping to $6D on overflow or when result >= $6E.
 * Then updates keys display via sub_CAE2. */
#include "ram.h"
#include "regs.h"

void sub_CAE2(Regs *r);

void sub_E859(Regs *r)
{
    u16 sum = (u16)r->a + keys;       /* CLC then ADC */
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    keys = v;
    sub_CAE2(r);                      /* reads keys from RAM */
}
