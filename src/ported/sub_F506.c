/* $F506:
 *   LDA $F7 / PHA
 * L_F509 (loop):
 *   JSR L_EFF1
 *   JSR L_CF08 / BCS L_F52E
 *   JSR L_CEC7 / BCC L_F519
 *     JSR L_F136
 * L_F519:
 *   JSR L_F275 / BCC L_F537       ; no collision -> done (C clear)
 *   LDX $F7 / BEQ L_F536
 *   BMI L_F526 / DEX / DEX
 * L_F526:
 *   INX / STX $F7 / BNE L_F509
 *   JMP L_F536
 * L_F52E:
 *   LDA #$00 / STA $EE / LDA #$F0 / STA $F3
 * L_F536: SEC
 * L_F537: PLA / STA $F7 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EFF1(Regs *r);
void sub_CF08(Regs *r);
void sub_CEC7(Regs *r);
void sub_F136(Regs *r);
void sub_F275(Regs *r);

void sub_F506(Regs *r)
{
    u8 saved_f7 = RAM8(0xF7);    /* PHA */
    u8 cflag;

    for (;;) {
        sub_EFF1(r);

        sub_CF08(r);
        if (r->c) {              /* BCS L_F52E */
            RAM8(0xEE) = 0x00;
            RAM8(0xF3) = 0xF0;
            cflag = 1;           /* L_F536: SEC */
            break;
        }

        sub_CEC7(r);
        if (r->c)                /* BCC L_F519 taken when C=0 */
            sub_F136(r);

        /* L_F519 */
        sub_F275(r);
        if (r->c == 0) {         /* BCC L_F537 -> done, C clear */
            cflag = 0;
            break;
        }

        {
            u8 x = RAM8(0xF7);   /* LDX $F7 */
            if (x == 0) {        /* BEQ L_F536 */
                cflag = 1;
                break;
            }
            if (!(x & 0x80))     /* BMI L_F526 not taken */
                x = (u8)(x - 2); /* DEX / DEX */
            x = (u8)(x + 1);     /* L_F526: INX */
            RAM8(0xF7) = x;      /* STX $F7 */
            if (x == 0) {        /* BNE L_F509: fall to JMP L_F536 */
                cflag = 1;
                break;
            }
            /* loop back to L_F509 */
        }
    }

    /* L_F537 */
    RAM8(0xF7) = saved_f7;       /* PLA / STA $F7 */
    r->c = cflag;
}
