/* $F233:  LDA ($0C),Y / AND #$3F / CMP #$30 / RTS
 * Reads a byte through pointer $0C/$0D indexed by Y; carry = (byte&$3F) >= $30. */
#include "ram.h"
#include "regs.h"

void sub_F233(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    u8 v = RAM8((u16)(ptr + r->y)) & 0x3F;
    r->c = (u8)(v >= 0x30);
}
