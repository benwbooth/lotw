/* $FC05:  STA $9A,X / RTS
 * Sound-command setter: store A into ($9A+X) (zp,X wraps). */
#include "ram.h"
#include "regs.h"

void sub_FC05(Regs *r)
{
    RAM8((0x9A + r->x) & 0xFF) = r->a;
}
