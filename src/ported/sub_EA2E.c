/* $EA2E: decrement timer $F3 and act on it.
 *   DEC $F3
 *   BNE L_EA42
 *     ; $F3 hit 0:
 *     LDA #$01 / STA $EE
 *     LDY #$00 / LDA ($E7),Y / STA $ED      ; $ED = *($E7:E8)
 *     INY      / LDA ($E7),Y / STA $EF      ; $EF = *($E7:E8 + 1)
 *     RTS
 *   L_EA42:
 *     LDA $F3 / AND #$03 / BNE L_EA4E       ; every 4th tick:
 *     LDA $EF / EOR #$40 / STA $EF          ; toggle bit6 of $EF
 *     L_EA4E: RTS
 * Inputs: $F3, pointer $E7/$E8, $EF. */
#include "ram.h"
#include "regs.h"

void sub_EA2E(Regs *r)
{
    u8 t = (u8)(RAM8(0xF3) - 1);
    RAM8(0xF3) = t;

    if (t == 0) {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        RAM8(0xEE) = 0x01;
        RAM8(0xED) = RAM8(ptr);
        RAM8(0xEF) = RAM8((u16)(ptr + 1));
    } else if ((t & 0x03) == 0) {
        RAM8(0xEF) ^= 0x40;
    }
}
