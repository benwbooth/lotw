/* $B2FC:  LDY #$1F / LDA #$C0 / loop: STA $0140,Y / DEY / BPL loop / RTS
 * Fills $0140..$015F (32 bytes) with $C0. */
#include "ram.h"
#include "regs.h"

void sub_B2FC(Regs *r)
{
    int y;
    for (y = 0x1F; y >= 0; --y)
        RAM8(0x0140 + y) = 0xC0;
    r->a = 0xC0;
    r->y = 0xFF; /* DEY past 0 -> $FF, BPL falls through */
}
