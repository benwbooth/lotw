/* $E3AD:  LDX $F5 / DEX / BPL L_E3B4 / LDX #$06
 *   L_E3B4: STX $F5 / JSR L_E400 / RTS
 * Decrement tile-X cursor $F5 (wrap 0->6 when result negative), recompute coords. */
#include "ram.h"
#include "regs.h"

void sub_E400(Regs *r);

void sub_E3AD(Regs *r)
{
    u8 x = (u8)(RAM8(0xF5) - 1);
    if (x & 0x80)                  /* DEX; BPL taken when result >= 0 */
        x = 0x06;
    RAM8(0xF5) = x;
    sub_E400(r);
}
