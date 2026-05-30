/* $F136:
 *   LDA $85 / BNE ret
 *   LDX $EE / DEX / BNE ret
 *   LDA $2D (mmc3_r3_shadow) / CMP #$30 / BCC L_F154
 *     LDA $E3 / BEQ L_F15A
 *     LDX $55 (equipped_item) / LDA $0051,X (carried_item0) / CMP #$0A / BEQ L_F173
 *     JMP L_F15A
 *   L_F154: LDA $40 (cur_character) / CMP #$04 / BEQ ret
 *   L_F15A: LDA $F8 / JSR L_E7DB / $8F=$21 / $90=$01 / $85=$01
 *           LDA $EF / AND #$DF / STA $EF / RTS
 *   L_F173: LDA #$01 / STA $8F / ret
 */
#include "ram.h"
#include "regs.h"

void sub_E7DB(Regs *r);

void sub_F136(Regs *r)
{
    if (RAM8(0x85) != 0)
        return;
    if ((u8)(RAM8(0xEE) - 1) != 0)   /* LDX $EE / DEX / BNE */
        return;

    if (RAM8(0x2D) >= 0x30) {        /* CMP #$30 / BCC L_F154 */
        if (RAM8(0xE3) != 0) {       /* LDA $E3 / BEQ L_F15A */
            u8 x = RAM8(0x55);
            if (RAM8((u16)(0x0051 + x)) == 0x0A) {   /* BEQ L_F173 */
                RAM8(0x8F) = 0x01;
                return;
            }
            /* JMP L_F15A: fall through to damage */
        }
        /* L_F15A */
    } else {
        /* L_F154 */
        if (RAM8(0x40) == 0x04)      /* cur_character == 4 -> ret */
            return;
        /* L_F15A */
    }

    /* L_F15A: take damage from $F8 */
    r->a = RAM8(0xF8);
    sub_E7DB(r);
    RAM8(0x8F) = 0x21;
    RAM8(0x90) = 0x01;
    RAM8(0x85) = 0x01;
    RAM8(0xEF) = (u8)(RAM8(0xEF) & 0xDF);
}
