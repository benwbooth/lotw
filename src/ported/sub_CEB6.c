/* $CEB6:  LDA $0A / SEC / SBC player_y ($45) / CMP #$10 / BCC set
 *         CMP #$F1 / BCC clr / set: SEC RTS / clr: CLC RTS
 * diff = $0A - player_y. Returns C=1 if diff<$10 or diff>=$F1, else C=0.
 * (On-screen vertical proximity check.) Output: carry. */
#include "ram.h"
#include "regs.h"

void sub_CEB6(Regs *r)
{
    u8 diff = (u8)(RAM8(0x0A) - RAM8(0x45));
    if (diff < 0x10)
        r->c = 1;
    else if (diff < 0xF1)
        r->c = 0;
    else
        r->c = 1;
}
