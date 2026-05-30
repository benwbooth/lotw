/* $FB1F — Noise sound channel update.
 *   BIT $C4 / BMI L_FB26 / JMP L_FB82       ; bit7 of $C4: active?
 * L_FB26: DEC $C3 / BEQ L_FB2D / JMP L_FB6B ; note duration countdown
 * L_FB2D: LDY #0 / LDA ($C5),Y / BEQ L_FB3F ; fetch next stream byte
 *   PHP / CMP #$FF / BNE L_FB45 / PLP / JSR FB8E / JMP L_FB2D
 * L_FB3F: JSR FCF9 / JMP L_FB82
 * L_FB45: JSR FD6B / AND #$7F / STA $C3 / PLP / BMI L_FB65
 *   LDA #$08 / ORA $27 / STA $27
 *   LDA $CA / STA NOISE_LO / LDA #$80 / STA NOISE_HI
 *   JSR FCC4 / JMP L_FB6B
 * L_FB65: JSR FCDF / JMP L_FB6B
 * L_FB6B: LDA $27 / AND #$08 / BNE L_FB72 / RTS
 * L_FB72: DEC $CD / BNE L_FB7C / JSR FD11 / STA NOISE_VOL
 * L_FB7C: JSR FD45 / BCS L_FB82 / RTS
 * L_FB82: LDA #$30 / STA NOISE_VOL / LDA $27 / AND #$F7 / STA $27 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_FB8E(Regs *r);
void sub_FCC4(Regs *r);
void sub_FCDF(Regs *r);
void sub_FCF9(Regs *r);
void sub_FD11(Regs *r);
void sub_FD45(Regs *r);
void inc16_95(Regs *r);

static void silence_FB82(Regs *r)
{
    REG_W(0x400C, 0x30);                          /* NOISE_VOL */
    RAM8(0x27) = (u8)(RAM8(0x27) & 0xF7);
}

void sub_FB1F(Regs *r)
{
    if ((RAM8(0xC4) & 0x80) == 0) {      /* BIT $C4 / BMI; else JMP L_FB82 */
        silence_FB82(r);
        return;
    }

    /* L_FB26 */
    if ((u8)(--RAM8(0xC3)) != 0)         /* DEC $C3 / BEQ L_FB2D; else L_FB6B */
        goto L_FB6B;

    /* L_FB2D loop */
    for (;;) {
        u16 ptr = (u16)(RAM8(0xC5) | (RAM8(0xC6) << 8));
        u8 note = RAM8(ptr);             /* LDA ($C5),Y  Y=0 */
        if (note == 0) {                 /* BEQ L_FB3F */
            sub_FCF9(r);
            silence_FB82(r);
            return;
        }
        if (note == 0xFF) {              /* CMP #$FF / BNE L_FB45 not taken */
            sub_FB8E(r);
            continue;                    /* JMP L_FB2D */
        }
        /* L_FB45 */
        inc16_95(r);                     /* JSR FD6B (A unchanged = note) */
        RAM8(0xC3) = (u8)(note & 0x7F);
        if (note & 0x80) {               /* PLP / BMI L_FB65 */
            sub_FCDF(r);
        } else {
            RAM8(0x27) = (u8)(RAM8(0x27) | 0x08);
            REG_W(0x400E, RAM8(0xCA));   /* NOISE_LO */
            REG_W(0x400F, 0x80);         /* NOISE_HI */
            sub_FCC4(r);
        }
        break;                           /* both paths JMP L_FB6B */
    }

L_FB6B:
    if ((RAM8(0x27) & 0x08) == 0)        /* LDA $27 / AND #$08 / BNE L_FB72; else RTS */
        return;
    /* L_FB72 */
    if ((u8)(--RAM8(0xCD)) == 0) {       /* DEC $CD / BNE L_FB7C */
        sub_FD11(r);
        REG_W(0x400C, r->a);             /* STA NOISE_VOL */
    }
    /* L_FB7C */
    sub_FD45(r);
    if (r->c)                            /* BCS L_FB82; else RTS */
        silence_FB82(r);
}
