/* $DB3C:  LDA #$11 / STA $008F / LDA #$02 / JSR L_E82C / RTS
 * Sets action code $8F=$11, then adds $02 to gold via sub_E82C. */
#include "ram.h"
#include "regs.h"

void sub_E82C(Regs *r);

void sub_DB3C(Regs *r)
{
    RAM8(0x8F) = 0x11;
    r->a = 0x02;
    sub_E82C(r);
}
