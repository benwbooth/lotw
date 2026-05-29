/* $EFF1: compute a target tile/pixel position from base ($F9/$FA/$FB) plus
 * deltas $F5/$F6 (X) and $F7 (Y).  Same shape as $D8B6 with different sources.
 *   LDA $F9 / STA $0E
 *   LDA $FA / STA $0F
 *   LDA $FB / STA $0A
 *   LDA $F7 / BEQ +
 *     CLC / ADC $0A / STA $0A     ; $0A += $F7
 * + LDA $F5 / BEQ ++
 *     CLC / ADC $0E / PHA         ; sum = $F5 + $0E
 *     AND #$0F / STA $0E          ; $0E = sum & $0F
 *     PLA / ASL Ax4               ; A discarded; carry = bit4 of sum
 *     LDA $0F / ADC $F6 / STA $0F ; $0F = $0F + $F6 + carry
 * ++ RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EFF1(Regs *r)
{
    u8 dx, sum, carry;

    RAM8(0x0E) = RAM8(0xF9);
    RAM8(0x0F) = RAM8(0xFA);
    RAM8(0x0A) = RAM8(0xFB);

    if (RAM8(0xF7) != 0)
        RAM8(0x0A) = (u8)(RAM8(0xF7) + RAM8(0x0A));

    dx = RAM8(0xF5);
    if (dx != 0) {
        sum = (u8)(dx + RAM8(0x0E));      /* CLC / ADC $0E -> sum */
        RAM8(0x0E) = (u8)(sum & 0x0F);    /* AND #$0F on sum */
        carry = (u8)((sum >> 4) & 1);     /* carry after ASL A x4 */
        RAM8(0x0F) = (u8)(RAM8(0x0F) + RAM8(0xF6) + carry);
    }
}
