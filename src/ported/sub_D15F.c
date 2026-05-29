/* $D15F:  LDX #$1F / LDA #$7F / loop: STA $0322,X / DEX / BPL loop / RTS
 * Fills $0322..$0341 (32 bytes) with $7F. */
#include "ram.h"
#include "regs.h"

void sub_D15F(Regs *r)
{
    int x;
    for (x = 0x1F; x >= 0; --x)
        RAM8(0x0322 + x) = 0x7F;
    r->a = 0x7F;
    r->x = 0xFF; /* DEX past 0 -> $FF, BPL falls through */
}
