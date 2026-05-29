/* $B10E:  LDX #$1F / loop: LDA $B6FC,X / STA $0240,X / DEX / BPL loop / RTS
 * Copies 32 bytes of OAM data from ROM $B6FC into $0240. X ends $FF. */
#include "ram.h"
#include "regs.h"

void sub_B10E(Regs *r)
{
    int x;
    for (x = 0x1F; x >= 0; x--)
        RAM8(0x0240 + x) = RAM8(0xB6FC + x);
    r->x = 0xFF;
}
