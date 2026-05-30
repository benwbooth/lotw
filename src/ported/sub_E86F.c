/* $E86F:  LDA keys / BEQ L_E87A / DEC keys / JSR L_CAE2 / CLC / RTS
 *         L_E87A: SEC / RTS
 * If keys != 0: decrement it, refresh display (CAE2), return C=0.
 * If keys == 0: return C=1 (cannot decrement). */
#include "ram.h"
#include "regs.h"

void sub_CAE2(Regs *r);

void sub_E86F(Regs *r)
{
    r->a = keys;
    if (r->a == 0) {
        r->c = 1;
        return;
    }
    keys = (u8)(keys - 1);
    sub_CAE2(r);
    r->c = 0;
}
