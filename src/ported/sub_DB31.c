/* $DB31:  LDA #$11 / STA $008F / LDA #$05 / JSR L_E816 / RTS
 * Sets action code $8F=$11, then adds $05 to magic via sub_E816. */
#include "ram.h"
#include "regs.h"

void sub_E816(Regs *r);

void sub_DB31(Regs *r)
{
    RAM8(0x8F) = 0x11;
    r->a = 0x05;
    sub_E816(r);
}
