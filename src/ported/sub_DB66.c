/* $DB66:  LDA #$15 / STA $008F / LDA #$14 / JSR L_E859 / RTS
 * Sets action code $8F=$15, then adds $14 to keys via sub_E859. */
#include "ram.h"
#include "regs.h"

void sub_E859(Regs *r);

void sub_DB66(Regs *r)
{
    RAM8(0x8F) = 0x15;
    r->a = 0x14;
    sub_E859(r);
}
