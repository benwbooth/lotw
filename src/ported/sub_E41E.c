/* $E41E:  LDA $F9 / AND #$1F / TAX / RTS  ->  X = $F9 & $1F */
#include "ram.h"
#include "regs.h"

void sub_E41E(Regs *r)
{
    r->x = RAM8(0xF9) & 0x1F;
}
