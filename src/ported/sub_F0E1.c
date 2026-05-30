/* $F0E1:
 *   LDA $F7 / PHA
 * L_F0E4 (loop):
 *   JSR L_EFF1
 *   JSR L_CF08 / BCS L_F10E
 *   LDX $EE / DEX / BNE L_F0F9
 *     JSR L_CE7C / BCC L_F0F9
 *     JSR L_F136
 * L_F0F9:
 *   JSR L_F23A / BCC L_F117       ; no collision -> done (C clear)
 *   LDX $F7 / BEQ L_F116          ; $F7==0 -> SEC, restore, ret
 *   BMI L_F106 / DEX / DEX
 * L_F106:
 *   INX / STX $F7 / BNE L_F0E4    ; loop while $F7 != 0
 *   JMP L_F116
 * L_F10E:
 *   LDA #$00 / STA $EE / LDA #$F0 / STA $F3
 * L_F116: SEC
 * L_F117: PLA / STA $F7 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EFF1(Regs *r);
void sub_CF08(Regs *r);
void sub_CE7C(Regs *r);
void sub_F136(Regs *r);
void sub_F23A(Regs *r);

void sub_F0E1(Regs *r)
{
    u8 saved_f7 = RAM8(0xF7);    /* PHA */
    u8 cflag;

    for (;;) {
        sub_EFF1(r);

        sub_CF08(r);
        if (r->c) {              /* BCS L_F10E */
            RAM8(0xEE) = 0x00;
            RAM8(0xF3) = 0xF0;
            cflag = 1;           /* L_F116: SEC */
            break;
        }

        if ((u8)(RAM8(0xEE) - 1) == 0) {   /* LDX $EE / DEX / BNE L_F0F9 */
            sub_CE7C(r);
            if (r->c)                       /* BCC L_F0F9 taken when C=0 */
                sub_F136(r);
        }

        /* L_F0F9 */
        sub_F23A(r);
        if (r->c == 0) {         /* BCC L_F117 -> done, C clear */
            cflag = 0;
            break;
        }

        {
            u8 x = RAM8(0xF7);   /* LDX $F7 */
            if (x == 0) {        /* BEQ L_F116 */
                cflag = 1;
                break;
            }
            if (!(x & 0x80)) {   /* BMI L_F106 not taken */
                x = (u8)(x - 2); /* DEX / DEX */
            }
            x = (u8)(x + 1);     /* L_F106: INX */
            RAM8(0xF7) = x;      /* STX $F7 */
            if (x == 0) {        /* BNE L_F0E4: fall to JMP L_F116 */
                cflag = 1;
                break;
            }
            /* loop back to L_F0E4 */
        }
    }

    /* L_F117 */
    RAM8(0xF7) = saved_f7;       /* PLA / STA $F7 */
    r->c = cflag;
}
