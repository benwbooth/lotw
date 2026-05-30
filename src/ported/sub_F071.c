/* $F071 animation/phase step (dispatch table entry @ $F033[2]):
 *   LDA $F5 / BEQ L_F090 / LDY #$00 / LDA $F6 / BMI L_F07D / LDY #$40
 *   L_F07D: STY $08 / LDA $EF / AND #$3F / ORA $08 / STA $EF
 *           LDA $ED / AND #$F7 / STA $ED / JMP L_F09C
 *   L_F090: LDA $F7 / BEQ L_F09C / LDA $ED / AND #$F3 / ORA #$08 / STA $ED
 *   L_F09C: INC $F3 / LDA $F3 / AND #$03 / BEQ L_F0A5 / RTS
 *   L_F0A5: LDA $ED / AND #$08 / BNE L_F0B2 / LDA $ED / EOR #$04 / STA $ED / RTS
 *   L_F0B2: LDA $EF / EOR #$40 / STA $EF / RTS
 * RAM-only. */
#include "ram.h"
#include "regs.h"

void sub_F071(Regs *r)
{
    if (RAM8(0xF5) != 0) {                   /* BEQ L_F090 path: F5 != 0 */
        u8 y = (RAM8(0xF6) & 0x80) ? 0x00 : 0x40;
        RAM8(0x08) = y;
        RAM8(0xEF) = (u8)((RAM8(0xEF) & 0x3F) | y);
        RAM8(0xED) = (u8)(RAM8(0xED) & 0xF7);
    } else {                                 /* L_F090 */
        if (RAM8(0xF7) != 0)                 /* BEQ L_F09C skips */
            RAM8(0xED) = (u8)((RAM8(0xED) & 0xF3) | 0x08);
    }
    /* L_F09C */
    RAM8(0xF3)++;
    if ((RAM8(0xF3) & 0x03) == 0) {          /* BEQ L_F0A5 */
        if ((RAM8(0xED) & 0x08) != 0)        /* BNE L_F0B2 */
            RAM8(0xEF) ^= 0x40;
        else
            RAM8(0xED) ^= 0x04;
    }
}
