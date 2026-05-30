/* $E3BA:  LDX $F7 / DEX / BPL L_E3C1 / LDX #$04
 *   L_E3C1: STX $F7 / JSR L_E400 / RTS
 * Decrement tile-Y cursor $F7 (wrap 0->4 when result negative), recompute coords. */
#include "ram.h"
#include "regs.h"

void sub_E400(Regs *r);

void sub_E3BA(Regs *r)
{
    u8 x = (u8)(RAM8(0xF7) - 1);
    if (x & 0x80)                  /* DEX; BPL taken when result >= 0 */
        x = 0x04;
    RAM8(0xF7) = x;
    sub_E400(r);
}
