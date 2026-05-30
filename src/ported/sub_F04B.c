/* $F04B animation/phase step (dispatch table entry @ $F033[1]):
 *   LDA $F5 / BEQ L_F061 / LDY #$00 / LDA $F6 / BMI L_F057 / LDY #$40
 *   L_F057: STY $08 / LDA $EF / AND #$3F / ORA $08 / STA $EF
 *   L_F061: INC $F3 / LDA $F3 / AND #$03 / BEQ L_F06A / RTS
 *   L_F06A: LDA $ED / EOR #$04 / STA $ED / RTS
 * Updates sprite-attr bits ($EF) from $F5/$F6, advances phase counter $F3, and
 * every 4th step toggles bit2 of $ED. RAM-only. */
#include "ram.h"
#include "regs.h"

void sub_F04B(Regs *r)
{
    if (RAM8(0xF5) != 0) {                 /* BEQ L_F061 skips this block */
        u8 y = (RAM8(0xF6) & 0x80) ? 0x00 : 0x40;
        RAM8(0x08) = y;
        RAM8(0xEF) = (u8)((RAM8(0xEF) & 0x3F) | y);
    }
    RAM8(0xF3)++;
    if ((RAM8(0xF3) & 0x03) == 0)           /* BEQ L_F06A: do the toggle */
        RAM8(0xED) ^= 0x04;
}
