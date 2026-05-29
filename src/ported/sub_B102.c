/* $B102:  LDX #$7F / loop: LDA $B71C,X / STA $0240,X / DEX / BPL loop / RTS
 * Copies 128 bytes of OAM data from ROM $B71C into $0240. X ends $FF. */
#include "ram.h"
#include "regs.h"

void sub_B102(Regs *r)
{
    int x;
    for (x = 0x7F; x >= 0; x--)
        RAM8(0x0240 + x) = RAM8(0xB71C + x);
    r->x = 0xFF;   /* DEX past 0 -> $FF (BPL falls through) */
}
