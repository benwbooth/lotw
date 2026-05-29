/* $D07C:  LDA #$EF / LDX #$80 / -: STA $0200,X / INX*4 / BNE - / RTS
 * Writes $EF to every 4th byte of the upper half of OAM ($0280..$02FC),
 * i.e. the Y position of sprites 32..63. X wraps to 0 after $80..$FC+4. */
#include "ram.h"
#include "regs.h"

void sub_D07C(Regs *r)
{
    u8 x = 0x80;
    do {
        RAM8((u16)(0x0200 + x)) = 0xEF;
        x = (u8)(x + 4);            /* four INX */
    } while (x != 0);                /* BNE until X wraps to 0 */
    r->x = x;                        /* X = 0 at RTS */
    r->a = 0xEF;
}
