/* $F53B:
 *   LDY #$00 / LDA $F6 / BMI + / LDA $F5 / BEQ ++ / LDY #$40
 *   +:  STY $08 / LDA $EF / AND #$3F / ORA $08 / STA $EF
 *   ++: RTS
 * Sets bits 6-7 of $EF from sign/zero of $F6/$F5:
 *   $F6 negative -> Y=$00 ; else $F5==0 -> return (no write) ; else Y=$40.
 */
#include "ram.h"
#include "regs.h"

void sub_F53B(Regs *r)
{
    u8 y = 0x00;
    if (RAM8(0xF6) & 0x80) {             /* BMI L_F547 */
        /* Y stays 0 */
    } else if (RAM8(0xF5) == 0) {        /* BEQ L_F551: return, no write */
        return;
    } else {
        y = 0x40;
    }
    RAM8(0x08) = y;                       /* STY $08 */
    RAM8(0xEF) = (u8)((RAM8(0xEF) & 0x3F) | y); /* AND #$3F / ORA $08 */
}
