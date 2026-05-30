/* $F8F0 — Square-1 sound channel update.
 *   BIT $94 / BMI L_F8F7 / JMP L_F95E       ; bit7 of $94: active?
 * L_F8F7: DEC $93 / BEQ L_F8FE / JMP L_F948 ; note duration countdown
 * L_F8FE: LDY #0 / LDA ($95),Y / BEQ L_F910 ; fetch next stream byte
 *   PHP / CMP #$FF / BNE L_F916
 *     PLP / JSR FB8E / JMP L_F8FE           ; $FF = command escape, loop
 * L_F910: JSR FCF9 / JMP L_F95E             ; 0 = end -> silence
 * L_F916: JSR FD6B / AND #$7F / STA $93     ; duration = next byte & $7F
 *   PLP / BMI L_F942                        ; bit7 of note byte: tie?
 *     JSR FC81 / LDA $27 / ORA #$01 / STA $27
 *     LDA $9A / STA SQ1_SWEEP
 *     LDA $04 / STA SQ1_LO
 *     LDA $05 / AND #$07 / ORA #$18 / STA SQ1_HI
 *     JSR FCC4 / JMP L_F948
 * L_F942: JSR FCDF / JMP L_F948
 * L_F948: LDA $27 / LSR / BCS L_F94E / RTS
 * L_F94E: DEC $9D / BNE L_F958 / JSR FD11 / STA SQ1_VOL
 * L_F958: JSR FD45 / BCS L_F95E / RTS
 * L_F95E: LDA $99 / AND #$C0 / ORA #$30 / STA SQ1_VOL
 *         LDA $27 / AND #$FE / STA $27 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_FB8E(Regs *r);
void sub_FC81(Regs *r);
void sub_FCC4(Regs *r);
void sub_FCDF(Regs *r);
void sub_FCF9(Regs *r);
void sub_FD11(Regs *r);
void sub_FD45(Regs *r);
void inc16_95(Regs *r);

static void silence_F95E(Regs *r)
{
    REG_W(0x4000, (RAM8(0x99) & 0xC0) | 0x30);   /* SQ1_VOL */
    RAM8(0x27) = (u8)(RAM8(0x27) & 0xFE);
}

void sub_F8F0(Regs *r)
{
    if ((RAM8(0x94) & 0x80) == 0) {      /* BIT $94 / BMI; else JMP L_F95E */
        silence_F95E(r);
        return;
    }

    /* L_F8F7 */
    if ((u8)(--RAM8(0x93)) != 0)         /* DEC $93 / BEQ L_F8FE; else L_F948 */
        goto L_F948;

    /* L_F8FE loop */
    for (;;) {
        u16 ptr = (u16)(RAM8(0x95) | (RAM8(0x96) << 8));
        u8 note = RAM8(ptr);             /* LDA ($95),Y  Y=0 */
        if (note == 0) {                 /* BEQ L_F910 */
            sub_FCF9(r);
            silence_F95E(r);
            return;
        }
        if (note == 0xFF) {              /* CMP #$FF / BNE L_F916 not taken */
            sub_FB8E(r);
            continue;                    /* JMP L_F8FE */
        }
        /* L_F916 */
        inc16_95(r);                     /* JSR FD6B (does not touch A) */
        RAM8(0x93) = (u8)(note & 0x7F);  /* AND #$7F / STA $93 (A still = note) */
        if (note & 0x80) {               /* PLP / BMI L_F942 (N from original note) */
            sub_FCDF(r);
        } else {
            sub_FC81(r);
            RAM8(0x27) = (u8)(RAM8(0x27) | 0x01);
            REG_W(0x4001, RAM8(0x9A));               /* SQ1_SWEEP */
            REG_W(0x4002, RAM8(0x04));               /* SQ1_LO */
            REG_W(0x4003, (RAM8(0x05) & 0x07) | 0x18); /* SQ1_HI */
            sub_FCC4(r);
        }
        break;                           /* both paths JMP L_F948 */
    }

L_F948:
    if ((RAM8(0x27) & 0x01) == 0)        /* LDA $27 / LSR / BCS L_F94E; else RTS */
        return;
    /* L_F94E */
    if ((u8)(--RAM8(0x9D)) == 0) {       /* DEC $9D / BNE L_F958 */
        sub_FD11(r);
        REG_W(0x4000, r->a);             /* STA SQ1_VOL */
    }
    /* L_F958 */
    sub_FD45(r);
    if (r->c)                            /* BCS L_F95E; else RTS */
        silence_F95E(r);
}
