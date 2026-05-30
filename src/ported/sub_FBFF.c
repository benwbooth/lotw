/* $FBFF:  STA $99,X / RTS
 * Sound-command setter: store A into ($99+X) (zp,X wraps). */
#include "ram.h"
#include "regs.h"

void sub_FBFF(Regs *r)
{
    RAM8((0x99 + r->x) & 0xFF) = r->a;
}
