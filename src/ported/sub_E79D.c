/* $E79D:
 *   LDA #$EF
 *   STA $0240 / STA $0244 / STA $0248 / STA $024C / STA $0250 / STA $0254
 *   RTS
 * Hides 6 sprites by setting their OAM-shadow Y to $EF (offscreen). Output: RAM.
 */
#include "ram.h"
#include "regs.h"

void sub_E79D(Regs *r)
{
    RAM8(0x0240) = 0xEF;
    RAM8(0x0244) = 0xEF;
    RAM8(0x0248) = 0xEF;
    RAM8(0x024C) = 0xEF;
    RAM8(0x0250) = 0xEF;
    RAM8(0x0254) = 0xEF;
    r->a = 0xEF;
    (void)r;
}
