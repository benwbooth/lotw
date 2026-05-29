/* $FCC4:
 *   LDX $02
 *   LDY $A2,X       ; Y = entity slot index
 *   STY $9B,X
 *   LDA $FDCB,Y / STA $9C,X   ; copy 4-byte ROM record ($FDCB..$FDCE)+Y
 *   LDA $FDCC,Y / STA $9D,X
 *   LDA $FDCD,Y / STA $9E,X
 *   LDA $FDCE,Y / STA $9F,X
 *   RTS
 * Loads a 4-byte attribute record from ROM table $FDCB indexed by ($A2+X),
 * stashing the index at $9B+X and the 4 bytes at $9C..$9F+X (zp,X wraps). */
#include "ram.h"
#include "regs.h"

void sub_FCC4(Regs *r)
{
    u8 x = RAM8(0x02);
    u8 y = RAM8((0xA2 + x) & 0xFF);

    RAM8((0x9B + x) & 0xFF) = y;
    RAM8((0x9C + x) & 0xFF) = RAM8((u16)(0xFDCB + y));
    RAM8((0x9D + x) & 0xFF) = RAM8((u16)(0xFDCC + y));
    RAM8((0x9E + x) & 0xFF) = RAM8((u16)(0xFDCD + y));
    RAM8((0x9F + x) & 0xFF) = RAM8((u16)(0xFDCE + y));

    r->x = x;
    r->y = y;
}
