/* $DB26 item-action (dispatch table @ $DB06): potion-of-health style item.
 *   LDA #$1E / STA a:$008F / LDA #$05 / JSR L_E800 / RTS
 * Sets sound/effect id $8F = $1E, then adds 5 to health via sub_E800. */
#include "ram.h"
#include "regs.h"

void sub_E800(Regs *r);

void sub_DB26(Regs *r)
{
    RAM8(0x8F) = 0x1E;
    r->a = 0x05;
    sub_E800(r);
}
