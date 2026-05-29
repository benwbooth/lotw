/* $DCCC:  LDA ($0C),Y / AND #$3F / TAX / BEQ + / CPX #$02 / BEQ + / CPX #$30 / RTS
 * Classifies the byte at *(($0C/$0D)+Y) & $3F. Returns carry:
 *   x==0  -> carry = (player_x_fine==0)   (CLC if nonzero, SEC if zero)
 *   x==2  -> carry = 1 (SEC)
 *   else  -> carry = (x >= $30)           (from CMP #$30)
 */
#include "ram.h"
#include "regs.h"

void sub_DCCC(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    u8 x = RAM8((u16)(ptr + r->y)) & 0x3F;

    if (x == 0) {                       /* BEQ L_DCDA */
        if (RAM8(0x43) == 0)            /* player_x_fine; BEQ L_DCE0 -> SEC */
            r->c = 1;
        else                            /* CLC */
            r->c = 0;
    } else if (x == 0x02) {             /* CPX #$02 / BEQ L_DCE0 -> SEC */
        r->c = 1;
    } else {                            /* CPX #$30 -> carry = x >= $30 */
        r->c = (u8)(x >= 0x30);
    }
}
