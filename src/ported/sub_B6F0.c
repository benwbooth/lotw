/* $B6F0:  LDX #$1F / loop: LDA $A2C9,X / STA $0180,X / DEX / BPL loop / RTS
 * Copies 32 bytes from ROM $A2C9 into $0180. X ends $FF. */
#include "ram.h"
#include "regs.h"

void sub_B6F0(Regs *r)
{
    int x;
    for (x = 0x1F; x >= 0; x--)
        RAM8(0x0180 + x) = RAM8(0xA2C9 + x);
    r->x = 0xFF;
}
