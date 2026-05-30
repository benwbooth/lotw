/* $E852:  INC keys / JSR L_CAE2 / CLC / RTS
 * Increment key count, refresh display (CAE2), return C=0. */
#include "ram.h"
#include "regs.h"

void sub_CAE2(Regs *r);

void sub_E852(Regs *r)
{
    keys = (u8)(keys + 1);
    sub_CAE2(r);
    r->c = 0;
}
