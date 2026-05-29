/* $B11A:  LDX #$EF / LDA $84 / AND #$30 / BEQ + / LDX #$80
 *         + STX $0240/$0244/$0248/$024C/$0250/$0254/$0258/$025C / RTS
 * Picks sprite Y ($EF or $80) based on flags in $84, writes it to 8 OAM Y slots. */
#include "ram.h"
#include "regs.h"

void sub_B11A(Regs *r)
{
    u8 x = 0xEF;
    if (RAM8(0x84) & 0x30)        /* BEQ skips the LDX #$80 when result==0 */
        x = 0x80;
    RAM8(0x0240) = x;
    RAM8(0x0244) = x;
    RAM8(0x0248) = x;
    RAM8(0x024C) = x;
    RAM8(0x0250) = x;
    RAM8(0x0254) = x;
    RAM8(0x0258) = x;
    RAM8(0x025C) = x;
    r->x = x;
}
