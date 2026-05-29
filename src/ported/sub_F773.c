/* $F773:
 *   LDA $EE / AND #$0C / STA $08
 *   LDA $ED / AND #$F3 / ORA $08 / STA $ED
 *   RTS
 * Copies bits 2-3 of $EE into bits 2-3 of $ED (via scratch $08). */
#include "ram.h"
#include "regs.h"

void sub_F773(Regs *r)
{
    u8 bits = RAM8(0xEE) & 0x0C;
    RAM8(0x08) = bits;
    RAM8(0xED) = (u8)((RAM8(0xED) & 0xF3) | bits);
    r->a = RAM8(0xED);
}
