/* $DB5D item-action (dispatch table @ $DB06): key pickup.
 *   LDA #$15 / STA a:$008F / JSR L_E852 / RTS
 * Sets sound/effect id $8F = $15, then increments key count via sub_E852
 * (which also clears carry). */
#include "ram.h"
#include "regs.h"

void sub_E852(Regs *r);

void sub_DB5D(Regs *r)
{
    RAM8(0x8F) = 0x15;
    sub_E852(r);
}
