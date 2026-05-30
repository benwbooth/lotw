/* $F0B9 animation/phase step (dispatch table entry @ $F033[3]):
 *   LDA $F5 / BEQ L_F0CF / LDY #$00 / LDA $F6 / BMI L_F0C5 / LDY #$40
 *   L_F0C5: STY $08 / LDA $EF / AND #$3F / ORA $08 / STA $EF
 *   L_F0CF: INC $F3 / LDA $F3 / AND #$06 / ASL A / STA $08
 *           LDA $ED / AND #$F3 / ORA $08 / STA $ED / RTS
 * Updates attr bits ($EF) from $F5/$F6, advances $F3 and sets bits 2-3 of $ED
 * from (($F3 & 6) << 1). RAM-only. */
#include "ram.h"
#include "regs.h"

void sub_F0B9(Regs *r)
{
    u8 t;
    if (RAM8(0xF5) != 0) {                  /* BEQ L_F0CF skips this block */
        u8 y = (RAM8(0xF6) & 0x80) ? 0x00 : 0x40;
        RAM8(0x08) = y;
        RAM8(0xEF) = (u8)((RAM8(0xEF) & 0x3F) | y);
    }
    RAM8(0xF3)++;
    t = (u8)((RAM8(0xF3) & 0x06) << 1);     /* AND #$06 / ASL A */
    RAM8(0x08) = t;
    RAM8(0xED) = (u8)((RAM8(0xED) & 0xF3) | t);
}
