/* $B631 (bank13 @ $A000):
 *   LDX #$40 / -: LDA $9B9F,X / STA $00,X / INX / CPX #$8C / BNE -
 *   LDA #$0F / LDX #$1F / -: STA $0180,X / DEX / BPL - / RTS
 * Copies $9B9F+X -> $00+X for X=$40..$8B, then fills $0180..$019F with $0F.
 */
#include "ram.h"
#include "regs.h"

void sub_B631(Regs *r)
{
    u8 x = 0x40;
    do {                                 /* copy table $9B9F,X -> zp $00,X */
        RAM8((u16)(0x00 + x)) = RAM8((u16)(0x9B9F + x));
        x++;
    } while (x != 0x8C);                  /* CPX #$8C / BNE */

    for (x = 0x1F; (x & 0x80) == 0; x--)  /* DEX / BPL: X = $1F..$00 */
        RAM8((u16)(0x0180 + x)) = 0x0F;

    r->a = 0x0F;
    r->x = 0xFF;                          /* X after DEX past 0 -> $FF (BPL falls through) */
}
