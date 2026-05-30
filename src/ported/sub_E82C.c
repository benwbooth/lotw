/* $E82C:  CLC / ADC gold / BCC L_E836 / LDA #$6D / JMP store
 *         L_E836: CMP #$6E / BCC store / LDA #$6D / store: STA gold / JSR L_CAF8 / RTS
 * Adds A to gold ($5A), clamping to $6D on overflow or when result >= $6E.
 * Then updates gold display via sub_CAF8. */
#include "ram.h"
#include "regs.h"

void sub_CAF8(Regs *r);

void sub_E82C(Regs *r)
{
    u16 sum = (u16)r->a + gold;       /* CLC then ADC */
    u8 v;

    if (sum > 0xFF)
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    gold = v;
    sub_CAF8(r);                      /* reads gold from RAM */
}
