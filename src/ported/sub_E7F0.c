/* $E7F0:  TXA / PHA / LDA magic / SEC / BEQ L_E7FD / DEC magic / JSR L_CACC / CLC
 *         L_E7FD: PLA / TAX / RTS
 * If magic != 0: decrement it, refresh display (CACC), return C=0.
 * If magic == 0: return C=1. X is preserved (saved/restored). */
#include "ram.h"
#include "regs.h"

void sub_CACC(Regs *r);

void sub_E7F0(Regs *r)
{
    u8 saved_x = r->x;
    r->a = magic;
    r->c = 1;
    if (magic != 0) {
        magic = (u8)(magic - 1);
        sub_CACC(r);
        r->c = 0;
    }
    r->x = saved_x;
}
