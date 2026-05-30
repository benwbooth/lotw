/* $EBD8 — boss-state handler ($EAAD jump-table target [3]). Computes/adjusts the
 * direction code in $F4, optionally re-deriving it via EE19, then runs the CD70
 * delta build and the placement/collision pipeline (shared tail with $EAFD).
 *
 *   LDA $F4 / AND #$0F / STA $F4
 *   LDA $F5 / ORA $F7 / BNE L_EC2E
 *   LDA $F9 / BNE L_EC02
 *     LDA $FA / STA $0C / LDA $FB / STA $0D / JSR CA54
 *     LDY 0 / ($0C),Y &$3F / BEQ L_EC34 ; INY / ($0C),Y &$3F / BEQ L_EC34
 *   L_EC02: LDA $F4 / AND #$03 / BNE L_EC0C / LDA #1 / STA $F4
 *   L_EC0C: LDX $F3 / LDA #0 / STA $F3 / DEX / BNE L_EC22
 *     LDA $F4 / AND #$03 / BEQ L_EC34 / EOR #$03 / STA $F4 / JMP L_EC3B
 *   L_EC22: JSR EE19 / LDA #$80 / ORA $F4 / STA $F4 / JMP L_EC3B
 *   L_EC2E: LDA $F3 / CMP #$10 / BCC L_EC3B
 *   L_EC34: LDA #0 / STA $F3 / JSR EE19
 *   L_EC3B: LDY 9 / ($E7),Y / TAY / LDA $F4 / JSR CD70
 *     LDA $F0 / BNE L_EC65
 *     LDA $F1 / BNE L_EC51
 *     LDA $F4 / BPL L_EC56
 *   L_EC51: JSR EEDA / BCC L_EC6A
 *   L_EC56: LDA #0 / STA $F1 / JSR F0E1 / BCC L_EC6A / JSR EF11 / JMP L_EC6D
 *   L_EC65: JSR EEBB / BCS L_EC6D
 *   L_EC6A: JSR EF04
 *   L_EC6D: JSR F179 / JSR F01E / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_CD70(Regs *r);
void sub_EE19(Regs *r);
void sub_EEBB(Regs *r);
void sub_EEDA(Regs *r);
void sub_EF04(Regs *r);
void sub_EF11(Regs *r);
void sub_F01E(Regs *r);
void sub_F0E1(Regs *r);
void sub_F179(Regs *r);

void sub_EBD8(Regs *r)
{
    RAM8(0xF4) = RAM8(0xF4) & 0x0F;

    if ((RAM8(0xF5) | RAM8(0xF7)) != 0) {
        /* L_EC2E */
        if (RAM8(0xF3) < 0x10)
            goto L_EC3B;
        goto L_EC34;
    }

    if (RAM8(0xF9) == 0) {                 /* BNE L_EC02 not taken */
        u16 ptr;
        RAM8(0x0C) = RAM8(0xFA);
        RAM8(0x0D) = RAM8(0xFB);
        sub_CA54(r);                        /* rebuilds $0C/$0D */
        ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
        if ((RAM8(ptr) & 0x3F) == 0)
            goto L_EC34;
        if ((RAM8((u16)(ptr + 1)) & 0x3F) == 0)
            goto L_EC34;
    }

    /* L_EC02 */
    if ((RAM8(0xF4) & 0x03) == 0)
        RAM8(0xF4) = 0x01;

    /* L_EC0C */
    {
        u8 x = (u8)(RAM8(0xF3) - 1);        /* LDX $F3 / DEX */
        RAM8(0xF3) = 0x00;
        if (x == 0) {                       /* BNE L_EC22 not taken */
            if ((RAM8(0xF4) & 0x03) == 0)   /* BEQ L_EC34 */
                goto L_EC34;
            RAM8(0xF4) = (u8)(RAM8(0xF4) ^ 0x03);
            goto L_EC3B;
        }
    }

    /* L_EC22 */
    sub_EE19(r);
    RAM8(0xF4) = (u8)(0x80 | RAM8(0xF4));
    goto L_EC3B;

L_EC34:
    RAM8(0xF3) = 0x00;
    sub_EE19(r);

L_EC3B:
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 0x09));     /* LDA ($E7),Y / TAY */
    }
    r->a = RAM8(0xF4);
    sub_CD70(r);

    if (RAM8(0xF0) != 0) {
        /* L_EC65 */
        sub_EEBB(r);
        if (r->c) goto L_EC6D;              /* BCS L_EC6D */
        goto L_EC6A;
    }

    if (RAM8(0xF1) != 0)
        goto L_EC51;
    if (!(RAM8(0xF4) & 0x80))               /* LDA $F4 / BPL L_EC56 */
        goto L_EC56;

L_EC51:
    sub_EEDA(r);
    if (!r->c) goto L_EC6A;                 /* BCC L_EC6A */

L_EC56:
    RAM8(0xF1) = 0x00;
    sub_F0E1(r);
    if (!r->c) goto L_EC6A;                 /* BCC L_EC6A */
    sub_EF11(r);
    goto L_EC6D;

L_EC6A:
    sub_EF04(r);

L_EC6D:
    sub_F179(r);
    sub_F01E(r);
}
