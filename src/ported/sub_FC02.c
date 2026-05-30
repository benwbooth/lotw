/* $FC02:  STA $A1,X / RTS
 * Sound-command setter: store A into ($A1+X) (zp,X wraps). */
#include "ram.h"
#include "regs.h"

void sub_FC02(Regs *r)
{
    RAM8((0xA1 + r->x) & 0xFF) = r->a;
}
