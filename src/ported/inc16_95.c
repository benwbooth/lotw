/* $FD6B:  LDX $02 / INC $95,X / BNE +2 / INC $96,X / RTS
 * X = $02; 16-bit little-endian increment of ($95+X):($96+X) (zp,X wraps). */
#include "ram.h"
#include "regs.h"

void inc16_95(Regs *r)
{
    u8 x = RAM8(0x02);
    if (++RAM8((0x95 + x) & 0xFF) == 0)
        ++RAM8((0x96 + x) & 0xFF);
    r->x = x;
}
