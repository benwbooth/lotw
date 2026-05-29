/* $CF1C:
 *   LDA $0A / CMP #$B0 / BCS L_CF2C   ; $0A >= $B0 -> SEC
 *   LDA $0F / CMP #$3F / BCC L_CF2E   ; $0F <  $3F -> CLC
 *   LDA $0E / BEQ L_CF2E              ; $0E == 0   -> CLC
 * L_CF2C: SEC / RTS
 * L_CF2E: CLC / RTS
 * Output: carry (callers test it).
 */
#include "ram.h"
#include "regs.h"

void sub_CF1C(Regs *r)
{
    if (RAM8(0x0A) >= 0xB0) {          /* BCS L_CF2C */
        r->c = 1;
        return;
    }
    if (RAM8(0x0F) < 0x3F) {           /* BCC L_CF2E */
        r->c = 0;
        return;
    }
    if (RAM8(0x0E) == 0) {             /* BEQ L_CF2E */
        r->c = 0;
        return;
    }
    r->c = 1;                          /* SEC / RTS */
}
