/* $CBFA:
 *   LDA $08 / LDY #$00 / SEC
 * L_CBFF:
 *   INY / SBC #$0A / BCS L_CBFF
 *   ADC #$0B / STA $08 / RTS
 * Divides $08 by 10: Y = quotient, $08 (and A) = remainder.
 * Loop subtracts 10 with carry until a borrow occurs (overshoot by 10), then
 * ADC #$0B with C=0 adds back 11 to recover the remainder. */
#include "ram.h"
#include "regs.h"

void sub_CBFA(Regs *r)
{
    u8 a = RAM8(0x08);
    u8 y = 0;
    u8 carry = 1;          /* SEC */

    do {                   /* INY ; SBC #$0A ; BCS */
        int t;
        y = (u8)(y + 1);
        t = (int)a - 0x0A - (1 - carry);
        a = (u8)t;
        carry = (t >= 0) ? 1 : 0;   /* SBC sets C when no borrow */
    } while (carry);

    /* ADC #$0B with C=0 */
    a = (u8)(a + 0x0B + carry);
    RAM8(0x08) = a;
    r->a = a;
    r->y = y;
}
