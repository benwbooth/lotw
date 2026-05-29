/* $DF80:
 *   LDY $0B
 *   LDA ($10),Y / AND #$3F / TAX
 *   LDA $74
 *   CPX #$3E / BEQ L_DF8F
 *   LDA ($10),Y
 * L_DF8F: RTS
 * Reads byte b through pointer $10/$11 indexed by $0B. X = b & $3F.
 * If (b & $3F) == $3E return A = $74, else return A = b (full byte). */
#include "ram.h"
#include "regs.h"

void sub_DF80(Regs *r)
{
    u8 y = RAM8(0x0B);
    u16 ptr = (u16)(RAM8(0x10) | (RAM8(0x11) << 8));
    u8 b = RAM8((u16)(ptr + y));
    u8 x = b & 0x3F;

    r->x = x;
    r->y = y;
    if (x == 0x3E)
        r->a = RAM8(0x74);
    else
        r->a = b;
}
