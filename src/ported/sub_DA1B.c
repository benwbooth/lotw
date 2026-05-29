/* $DA1B:
 *   LDA mmc3_r3_shadow ($2D) / CMP #$30 / BCS .ret   ; ret if $2D >= $30
 *   LDA $87 / BEQ .ret                                ; ret if $87 == 0
 *   LDA magic ($59) / BEQ .ret                        ; ret if magic == 0
 *   LDX $09 / LDA #$80 / STA $0401,X                  ; else write $80 to $0401+X
 * .ret (DA30): RTS.  Conditionally stores $80 into RAM $0401,X. */
#include "ram.h"
#include "regs.h"

void sub_DA1B(Regs *r)
{
    if (RAM8(0x2D) < 0x30 && RAM8(0x87) != 0 && RAM8(0x59) != 0) {
        u8 x = RAM8(0x09);
        RAM8((u16)(0x0401 + x)) = 0x80;
    }
    (void)r;
}
