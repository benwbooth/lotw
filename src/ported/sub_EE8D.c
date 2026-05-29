/* $EE8D:  LDA $F4 / AND #$03 / BNE +5 / LDA #$01 / EOR #$03 / STA $F4 / RTS
 * v = $F4 & 3; if v==0 v=1; $F4 = v ^ 3.  (rotates a 2-bit field, skipping 0). */
#include "ram.h"
#include "regs.h"

void sub_EE8D(Regs *r)
{
    u8 v = RAM8(0xF4) & 0x03;
    if (v == 0)              /* BNE skips the LDA #$01 when nonzero */
        v = 0x01;
    v ^= 0x03;
    RAM8(0xF4) = v;
    r->a = v;               /* A holds the stored value at RTS */
}
