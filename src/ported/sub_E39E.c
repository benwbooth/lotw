/* $E39E:  LDX $F5 / INX / CPX #$07 / BCC L_E3A7 / LDX #$00
 *   L_E3A7: STX $F5 / JSR L_E400 / RTS
 * Increment tile-X cursor $F5 (wrap 6->0), then recompute sprite coords. */
#include "ram.h"
#include "regs.h"

void sub_E400(Regs *r);

void sub_E39E(Regs *r)
{
    u8 x = (u8)(RAM8(0xF5) + 1);
    if (x >= 0x07)                 /* CPX #$07; BCC taken when x<7 */
        x = 0x00;
    RAM8(0xF5) = x;
    sub_E400(r);
}
