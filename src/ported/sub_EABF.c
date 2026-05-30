/* $EABF:
 *   LDA $F0 / BNE L_EACF
 *   LDA $F1 / BEQ L_EAD7
 *   JSR L_EEDA / BCS L_EAD7
 *   JSR L_EF04
 * L_EACF:
 *   JSR L_EEBB / BCS L_EAD7
 *   JSR L_EF04
 * L_EAD7:
 *   LDX $F3 / DEX / BNE L_EAE5
 *     LDA #$00 / STA $EE / LDA #$F0 / STA $F3 / RTS
 * L_EAE5:
 *   STX $F3 / CPX #$3C / BCS L_EAF9
 *     LDX #$EF / LDA $FB / CMP #$EF / BNE L_EAF5 / LDX $FC
 *   L_EAF5: STX $FB / STA $FC
 * L_EAF9:
 *   JSR L_F179 / RTS
 *
 * Per-frame entity placement tick: optionally runs candidate-validation passes
 * (EEDA/EEBB + EF04), counts down a timer $F3 (reset to $F0 at expiry), and on
 * non-expiry shrinks the $FB/$FC pointer window then runs F179.
 */
#include "ram.h"
#include "regs.h"

void sub_EEDA(Regs *r);
void sub_EEBB(Regs *r);
void sub_EF04(Regs *r);
void sub_F179(Regs *r);

void sub_EABF(Regs *r)
{
    u8 x, a;

    if (RAM8(0xF0) == 0) {           /* BNE L_EACF not taken */
        if (RAM8(0xF1) == 0)         /* BEQ L_EAD7 */
            goto L_EAD7;
        sub_EEDA(r);
        if (r->c)                    /* BCS L_EAD7 */
            goto L_EAD7;
        sub_EF04(r);
    }

    /* L_EACF */
    sub_EEBB(r);
    if (r->c)                        /* BCS L_EAD7 */
        goto L_EAD7;
    sub_EF04(r);

L_EAD7:
    x = (u8)(RAM8(0xF3) - 1);        /* LDX $F3 / DEX */
    if (x == 0) {                    /* BNE L_EAE5 not taken */
        RAM8(0xEE) = 0x00;
        RAM8(0xF3) = 0xF0;
        r->x = x;                    /* X = 0 at RTS */
        return;
    }

    /* L_EAE5 */
    RAM8(0xF3) = x;
    if (x < 0x3C) {                  /* CPX #$3C / BCS L_EAF9 not taken */
        x = 0xEF;                    /* LDX #$EF */
        a = RAM8(0xFB);              /* LDA $FB */
        if (a == 0xEF)               /* CMP #$EF / BNE L_EAF5 not taken */
            x = RAM8(0xFC);          /* LDX $FC */
        /* L_EAF5 */
        RAM8(0xFB) = x;              /* STX $FB */
        RAM8(0xFC) = a;              /* STA $FC */
    }

    /* L_EAF9 */
    sub_F179(r);
}
