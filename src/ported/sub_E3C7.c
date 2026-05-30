/* $E3C7:  LDX $F7 / INX / CPX #$05 / BCC L_E3D0 / LDX #$00
 *   L_E3D0: STX $F7 / JSR L_E400 / RTS
 * Increment tile-Y cursor $F7 (wrap 4->0), then recompute sprite coords. */
#include "ram.h"
#include "regs.h"

void sub_E400(Regs *r);

void sub_E3C7(Regs *r)
{
    u8 x = (u8)(RAM8(0xF7) + 1);
    if (x >= 0x05)                 /* CPX #$05; BCC taken when x<5 */
        x = 0x00;
    RAM8(0xF7) = x;
    sub_E400(r);
}
