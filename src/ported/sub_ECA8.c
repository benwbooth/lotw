/* $ECA8 — boss-state handler ($EAAD jump-table target [5]). Multi-phase placement
 * logic with several entry sub-states keyed on $F0/$F1, ending in the shared
 * F01E phase-dispatch tail.
 *
 *   LDA $F0 / BNE L_ECFA
 *   LDA $F1 / BNE L_ED16
 *   LDA $FA / STA $0F / LDA $F9 / STA $0E / LDA $FB / STA $0A
 *   JSR EDF0 / BCS L_ECC8 / INC $F0 / INC $F0 / JMP L_ECFA
 * L_ECC8: LDA $F5 / ORA $F7 / BNE L_ECD1 / JSR EE8D
 * L_ECD1: JSR CE90 / BCS L_ECED
 *   LDY 9 / ($E7),Y / TAY / LDA $F4 / JSR CD70
 *   JSR F0E1 / BCS L_ED21 / JSR EDF0 / BCC L_ED21 / JMP L_ED10
 * L_ECED: LDA #0 / STA $F5 / STA $F6 / JSR F179 / LDA $F0 / BCS L_ED21
 * L_ECFA: JSR EEBB / JSR EF04 / LDA $F0 / PHA / JSR F179 / PLA / BCC L_ED10
 *   ADC #$05 / STA $F1 / JMP L_ED24       ; carry=1 here -> $F1 = $F0 + 6
 * L_ED10: JSR EF04 / JMP L_ED24
 * L_ED16: JSR EEDA / BCS L_ED21 / JSR EF04 / JMP L_ED24
 * L_ED21: JSR EF11
 * L_ED24: JSR F01E / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CD70(Regs *r);
void sub_CE90(Regs *r);
void sub_EDF0(Regs *r);
void sub_EE8D(Regs *r);
void sub_EEBB(Regs *r);
void sub_EEDA(Regs *r);
void sub_EF04(Regs *r);
void sub_EF11(Regs *r);
void sub_F01E(Regs *r);
void sub_F0E1(Regs *r);
void sub_F179(Regs *r);

void sub_ECA8(Regs *r)
{
    if (RAM8(0xF0) != 0)
        goto L_ECFA;
    if (RAM8(0xF1) != 0)
        goto L_ED16;

    RAM8(0x0F) = RAM8(0xFA);
    RAM8(0x0E) = RAM8(0xF9);
    RAM8(0x0A) = RAM8(0xFB);

    sub_EDF0(r);
    if (r->c)                       /* BCS L_ECC8 */
        goto L_ECC8;
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
    goto L_ECFA;

L_ECC8:
    if ((RAM8(0xF5) | RAM8(0xF7)) == 0)   /* BNE L_ECD1 not taken */
        sub_EE8D(r);

    /* L_ECD1 */
    sub_CE90(r);
    if (r->c)                        /* BCS L_ECED */
        goto L_ECED;
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 0x09));   /* LDA ($E7),Y / TAY */
    }
    r->a = RAM8(0xF4);
    sub_CD70(r);
    sub_F0E1(r);
    if (r->c)                        /* BCS L_ED21 */
        goto L_ED21;
    sub_EDF0(r);
    if (!r->c)                       /* BCC L_ED21 */
        goto L_ED21;
    goto L_ED10;

L_ECED:
    RAM8(0xF5) = 0x00;
    RAM8(0xF6) = 0x00;
    sub_F179(r);
    /* LDA $F0 (no flag effect) / BCS L_ED21 : carry from F179 */
    if (r->c)
        goto L_ED21;

L_ECFA:
    sub_EEBB(r);
    sub_EF04(r);
    {
        u8 saved_f0 = RAM8(0xF0);    /* LDA $F0 / PHA */
        sub_F179(r);
        /* PLA (A=saved_f0, carry unchanged) / BCC L_ED10 */
        if (!r->c)
            goto L_ED10;
        /* ADC #$05 with carry=1 -> saved_f0 + 6 */
        RAM8(0xF1) = (u8)(saved_f0 + 0x05 + 1);
        goto L_ED24;
    }

L_ED10:
    sub_EF04(r);
    goto L_ED24;

L_ED16:
    sub_EEDA(r);
    if (r->c)                        /* BCS L_ED21 */
        goto L_ED21;
    sub_EF04(r);
    goto L_ED24;

L_ED21:
    sub_EF11(r);

L_ED24:
    sub_F01E(r);
}
