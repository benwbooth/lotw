/* $EAFD — boss-state handler ($EAAD jump-table target). Optionally re-rolls a
 * random movement target (when $F3>=$20, or $F1==0 and $F5|$F7==0), then runs the
 * per-step delta build (CD70) and the placement/collision pipeline.
 *
 *   LDA $F3 / CMP #$20 / BCS reroll
 *   LDA $F1 / BNE place
 *   LDA $F5 / ORA $F7 / BNE place
 * reroll (L_EB0D):
 *   LDA #0 / STA $F3 / JSR EEA6
 *   LDA #6 / JSR rng_update / CLC / ADC #1 / STA $F6
 *   LDA #4 / JSR rng_update / TAX / BNE place
 *   LDA #$80 / ORA $F4 / STA $F4
 * place (L_EB2C):
 *   LDA $F6 / PHA / TAY / LDA $F4 / JSR CD70
 *   LDA $F0 / BNE L_EB55
 *   LDA $F1 / BNE L_EB41
 *   LDA $F4 / BPL L_EB46
 * L_EB41: JSR EEDA / BCC L_EB5A
 * L_EB46: LDA #0 / STA $F1 / JSR F0E1 / BCC L_EB5A / JSR EF11 / JMP L_EB5D
 * L_EB55: JSR EEBB / BCS L_EB5D
 * L_EB5A: JSR EF04
 * L_EB5D: JSR F179 / JSR F01E / PLA / STA $F6 / RTS
 */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);
void sub_CD70(Regs *r);
void sub_EEA6(Regs *r);
void sub_EEBB(Regs *r);
void sub_EEDA(Regs *r);
void sub_EF04(Regs *r);
void sub_EF11(Regs *r);
void sub_F01E(Regs *r);
void sub_F0E1(Regs *r);
void sub_F179(Regs *r);

void sub_EAFD(Regs *r)
{
    u8 saved_f6;
    int do_place = 0;

    if (RAM8(0xF3) >= 0x20) {
        /* fall to reroll */
    } else if (RAM8(0xF1) != 0) {
        do_place = 1;
    } else if ((RAM8(0xF5) | RAM8(0xF7)) != 0) {
        do_place = 1;
    }

    if (!do_place) {
        /* reroll (L_EB0D) */
        RAM8(0xF3) = 0x00;
        sub_EEA6(r);
        r->a = 0x06;
        rng_update(r);
        RAM8(0xF6) = (u8)(r->a + 1);   /* CLC / ADC #1 */
        r->a = 0x04;
        rng_update(r);
        r->x = r->a;                    /* TAX */
        if (r->a == 0) {                /* BNE place not taken */
            RAM8(0xF4) = (u8)(0x80 | RAM8(0xF4));
        }
    }

    /* place (L_EB2C) */
    saved_f6 = RAM8(0xF6);              /* PHA */
    r->y = RAM8(0xF6);                  /* TAY */
    r->a = RAM8(0xF4);
    sub_CD70(r);

    if (RAM8(0xF0) != 0) {
        /* L_EB55 */
        sub_EEBB(r);
        if (r->c) goto L_EB5D;          /* BCS L_EB5D */
        goto L_EB5A;
    }

    if (RAM8(0xF1) != 0)
        goto L_EB41;
    if (!(RAM8(0xF4) & 0x80))           /* LDA $F4 / BPL L_EB46 */
        goto L_EB46;

L_EB41:
    sub_EEDA(r);
    if (!r->c) goto L_EB5A;             /* BCC L_EB5A */
    /* fall through to L_EB46 */

L_EB46:
    RAM8(0xF1) = 0x00;
    sub_F0E1(r);
    if (!r->c) goto L_EB5A;             /* BCC L_EB5A */
    sub_EF11(r);
    goto L_EB5D;

L_EB5A:
    sub_EF04(r);

L_EB5D:
    sub_F179(r);
    sub_F01E(r);
    RAM8(0xF6) = saved_f6;              /* PLA / STA $F6 */
}
