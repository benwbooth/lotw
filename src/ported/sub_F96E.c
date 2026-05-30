/* $F96E — Square-2 sound channel update (shares SQ2 with the SFX overlay; the
 * bit6 of $A4 marks "overlay owns the channel" and gates most output).
 *   BIT $A4 / BMI L_F978 / BVS L_F977(RTS) / JMP L_F9F9
 * L_F978: DEC $A3 / BEQ L_F97F / JMP L_F9DD
 * L_F97F: LDY #0 / LDA ($A5),Y / BEQ L_F991
 *   PHP / CMP #$FF / BNE L_F997 / PLP / JSR FB8E / JMP L_F97F
 * L_F991: JSR FCF9 / JMP L_F9F9
 * L_F997: JSR FD6B / AND #$7F / STA $A3 / PLP / BMI L_F9D2
 *   BIT $A4 / BVC L_F9AB / JSR FD6B / JMP L_F9D6
 * L_F9AB: JSR FC81 / LDA $27 / ORA #$02 / STA $27
 *   LDA $A9 / STA SQ2_VOL / LDA $AA / STA SQ2_SWEEP
 *   LDA $04 / STA SQ2_LO / LDA $05 / AND #$07 / ORA #$18 / STA SQ2_HI
 *   JSR FCC4 / JMP L_F9DD
 * L_F9D2: BIT $A4 / BVC L_F9D7
 * L_F9D6: RTS
 * L_F9D7: JSR FCDF / JMP L_F9DD
 * L_F9DD: BIT $A4 / BVC L_F9E2 / RTS
 * L_F9E2: LDA $27 / AND #$02 / BNE L_F9E9 / RTS
 * L_F9E9: DEC $AD / BNE L_F9F3 / JSR FD11 / STA SQ2_VOL
 * L_F9F3: JSR FD45 / BCS L_F9F9 / RTS
 * L_F9F9: LDA $A9 / AND #$C0 / ORA #$30 / STA SQ2_VOL
 *         LDA $27 / AND #$FD / STA $27 / RTS
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

static void silence_F9F9(Regs *r)
{
    REG_W(0x4004, (RAM8(0xA9) & 0xC0) | 0x30);   /* SQ2_VOL */
    RAM8(0x27) = (u8)(RAM8(0x27) & 0xFD);
}

void sub_F96E(Regs *r)
{
    u8 a4 = RAM8(0xA4);
    if ((a4 & 0x80) == 0) {          /* BMI L_F978 not taken */
        if (a4 & 0x40)               /* BVS L_F977 -> RTS */
            return;
        silence_F9F9(r);             /* JMP L_F9F9 */
        return;
    }

    /* L_F978 */
    if ((u8)(--RAM8(0xA3)) != 0)     /* DEC $A3 / BEQ L_F97F; else L_F9DD */
        goto L_F9DD;

    /* L_F97F loop */
    for (;;) {
        u16 ptr = (u16)(RAM8(0xA5) | (RAM8(0xA6) << 8));
        u8 note = RAM8(ptr);         /* LDA ($A5),Y  Y=0 */
        if (note == 0) {             /* BEQ L_F991 */
            sub_FCF9(r);
            silence_F9F9(r);
            return;
        }
        if (note == 0xFF) {          /* CMP #$FF / BNE L_F997 not taken */
            sub_FB8E(r);
            continue;                /* JMP L_F97F */
        }
        /* L_F997 */
        inc16_95(r);                 /* JSR FD6B (A unchanged = note) */
        RAM8(0xA3) = (u8)(note & 0x7F);
        if (note & 0x80) {           /* PLP / BMI L_F9D2 */
            /* L_F9D2: BIT $A4 / BVC L_F9D7 */
            if (RAM8(0xA4) & 0x40)   /* BVC not taken (V set) -> L_F9D6 RTS */
                return;
            sub_FCDF(r);             /* L_F9D7 */
            goto L_F9DD;
        }
        /* note bit7 clear */
        if (RAM8(0xA4) & 0x40) {     /* BIT $A4 / BVC L_F9AB; V set -> fall through */
            inc16_95(r);             /* JSR FD6B / JMP L_F9D6 (RTS) */
            return;
        }
        /* L_F9AB */
        sub_FC81(r);
        RAM8(0x27) = (u8)(RAM8(0x27) | 0x02);
        REG_W(0x4004, RAM8(0xA9));               /* SQ2_VOL */
        REG_W(0x4005, RAM8(0xAA));               /* SQ2_SWEEP */
        REG_W(0x4006, RAM8(0x04));               /* SQ2_LO */
        REG_W(0x4007, (RAM8(0x05) & 0x07) | 0x18); /* SQ2_HI */
        sub_FCC4(r);
        break;                       /* JMP L_F9DD */
    }

L_F9DD:
    if (RAM8(0xA4) & 0x40)           /* BIT $A4 / BVC L_F9E2; V set -> RTS */
        return;
    if ((RAM8(0x27) & 0x02) == 0)    /* LDA $27 / AND #$02 / BNE L_F9E9; else RTS */
        return;
    /* L_F9E9 */
    if ((u8)(--RAM8(0xAD)) == 0) {   /* DEC $AD / BNE L_F9F3 */
        sub_FD11(r);
        REG_W(0x4004, r->a);         /* STA SQ2_VOL */
    }
    /* L_F9F3 */
    sub_FD45(r);
    if (r->c)                        /* BCS L_F9F9; else RTS */
        silence_F9F9(r);
}
