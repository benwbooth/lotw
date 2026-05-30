/* $ED6F:
 *   LDA $F5 / ORA $F7 / BNE L_ED78 / JSR L_EE9A
 *   L_ED78: LDY #$09 / LDA ($E7),Y / TAY / LDA $F4 / JSR L_CD70 / JSR L_F11B
 *           BCC L_ED91 / LDA $EA / BNE L_ED9A / JSR L_EF11 / JMP L_ED94
 *   L_ED91: JSR L_EF04
 *   L_ED94: JSR L_F01E / JMP L_EFF0 (RTS)
 *   L_ED9A: LDA #$80 / STA $EE / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EE9A(Regs *);
void sub_CD70(Regs *);
void sub_F11B(Regs *);
void sub_EF11(Regs *);
void sub_EF04(Regs *);
void sub_F01E(Regs *);

void sub_ED6F(Regs *r)
{
    if ((RAM8(0xF5) | RAM8(0xF7)) == 0)
        sub_EE9A(r);

    /* L_ED78 */
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 0x09));
        r->a = RAM8(0xF4);
        sub_CD70(r);
    }

    sub_F11B(r);
    if (r->c) {
        if (RAM8(0xEA) != 0) {           /* L_ED9A */
            r->a = 0x80;
            RAM8(0xEE) = 0x80;
            return;
        }
        sub_EF11(r);
    } else {
        sub_EF04(r);                     /* L_ED91 */
    }

    /* L_ED94 */
    sub_F01E(r);
    /* JMP L_EFF0 -> RTS */
}
