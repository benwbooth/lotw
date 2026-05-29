/* $CF08:  LDA $0A / CMP #$C0 / BCS set / LDA $0F / CMP #$3F / BCC clr
 *         LDA $0E / BEQ clr / set: SEC RTS / clr: CLC RTS
 * C=1 if $0A>=$C0; else C=0 if $0F<$3F or $0E==0; else C=1. Output: carry. */
#include "ram.h"
#include "regs.h"

void sub_CF08(Regs *r)
{
    if (RAM8(0x0A) >= 0xC0)
        r->c = 1;
    else if (RAM8(0x0F) < 0x3F)
        r->c = 0;
    else if (RAM8(0x0E) == 0)
        r->c = 0;
    else
        r->c = 1;
}
