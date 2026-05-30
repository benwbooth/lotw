/* $DB52:  LDA #$1D / STA $008F / LDA #$05 / JSR L_E7DB / RTS
 * Sets action code $8F=$1D, then subtracts $05 from health via sub_E7DB. */
#include "ram.h"
#include "regs.h"

void sub_E7DB(Regs *r);

void sub_DB52(Regs *r)
{
    RAM8(0x8F) = 0x1D;
    r->a = 0x05;
    sub_E7DB(r);
}
