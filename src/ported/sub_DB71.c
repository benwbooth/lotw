/* $DB71:  LDA #$13 / STA $008F / LDA #$0A / STA $85 / RTS
 * Item-action handler: set state byte $8F=$13, scratch $85=$0A. A=$0A on exit. */
#include "ram.h"
#include "regs.h"

void sub_DB71(Regs *r)
{
    RAM8(0x8F) = 0x13;
    RAM8(0x85) = 0x0A;
    r->a = 0x0A;
}
