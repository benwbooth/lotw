/* $DB7B:  LDA #$13 / STA $008F / LDA #$1E / STA $85 / RTS
 * Item-action handler: set state byte $8F=$13, scratch $85=$1E. A=$1E on exit. */
#include "ram.h"
#include "regs.h"

void sub_DB7B(Regs *r)
{
    RAM8(0x8F) = 0x13;
    RAM8(0x85) = 0x1E;
    r->a = 0x1E;
}
