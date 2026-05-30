/* $F03B phase-0 handler:
 *   INC $F3 / LDA $F3 / AND #$03 / BEQ $F044 / RTS
 *   $F044: LDA $EF / EOR #$40 / STA $EF / RTS
 * Increments frame phase $F3; every 4th frame toggles bit6 of $EF. */
#include "ram.h"
#include "regs.h"

void sub_F03B(Regs *r)
{
    u8 a;
    RAM8(0xF3)++;
    a = RAM8(0xF3) & 0x03;
    if (a == 0) {                       /* BEQ -> falls into $F044 */
        a = RAM8(0xEF) ^ 0x40;
        RAM8(0xEF) = a;
    }
    r->a = a;
}
