/* $B2EE:  LDX #$00 / LDA #$EF / -: STA $0200,X / INX*4 / BNE -  / RTS
 * Writes $EF to every 4th byte of OAM ($0200..$02FC), i.e. the Y position of
 * all 64 sprites, hiding them offscreen. X wraps to 0 after 256 to end. */
#include "ram.h"
#include "regs.h"

void sub_B2EE(Regs *r)
{
    u8 x = 0x00;
    do {
        RAM8((u16)(0x0200 + x)) = 0xEF;
        x = (u8)(x + 4);            /* four INX */
    } while (x != 0);                /* BNE until X wraps to 0 */
    r->x = x;                        /* X = 0 at RTS */
    r->a = 0xEF;
}
