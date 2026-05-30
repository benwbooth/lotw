/* $FA60 sfx_overlay_voice — sound-effect overlay that borrows the SQ2 channel.
 *   LDA $8F / BEQ L_FA74                  ; $8F = pending SFX id
 *   LDA $D4 / BPL L_FA79                  ; overlay not yet active -> start it
 *   LDA $90 / CMP $91 / BCS L_FA79        ; new SFX priority >= current -> restart
 *   LDA #0 / STA $90 / STA $8F            ; otherwise clear pending request
 * L_FA74: LDA $D4 / BMI L_FA9E / RTS      ; overlay inactive -> done
 * L_FA79: LDA $90 / STA $91               ; latch priority
 *   LDA $8F / ASL / TAX
 *   LDA $8014,X / STA $D5 / LDA $8015,X / STA $D6   ; stream ptr from ROM table
 *   LDA #$80 / STA $D4                    ; mark overlay active
 *   LDA $A4 / ORA #$40 / STA $A4          ; tell SQ2 music handler overlay owns it
 *   LDA #0 / STA $8F / STA $90 / JMP L_FAA5
 * L_FA9E: DEC $D3 / BEQ L_FAA5 / JMP L_FAF8
 * L_FAA5: LDY #0 / LDA ($D5),Y / BEQ L_FAB7
 *   PHP / CMP #$FF / BNE L_FAC6 / PLP / JSR FB8E / JMP L_FAA5
 * L_FAB7: LDA #0 / STA $D4 / STA $91      ; end of SFX -> release channel
 *   LDA $A4 / AND #$BF / STA $A4 / JMP L_FB0F
 * L_FAC6: JSR FD6B / AND #$7F / STA $D3 / PLP / BMI L_FAF2
 *   JSR FC81 / LDA #$02 / ORA $27 / STA $27
 *   LDA $DA / STA SQ2_SWEEP / LDA $04 / STA SQ2_LO
 *   LDA $05 / AND #$07 / ORA #$C0 / STA SQ2_HI / JSR FCC4 / JMP L_FAF8
 * L_FAF2: JSR FCDF / JMP L_FAF8
 * L_FAF8: LDA $27 / AND #$02 / BNE L_FAFF / RTS
 * L_FAFF: DEC $DD / BNE L_FB09 / JSR FD11 / STA SQ2_VOL
 * L_FB09: JSR FD45 / BCS L_FB0F / RTS
 * L_FB0F: LDA $D9 / AND #$C0 / ORA #$30 / STA SQ2_VOL
 *         LDA $27 / AND #$FD / STA $27 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_FB8E(Regs *r);
void sub_FC81(Regs *r);
void sub_FCC4(Regs *r);
void sub_FCDF(Regs *r);
void sub_FD11(Regs *r);
void sub_FD45(Regs *r);
void inc16_95(Regs *r);

static void silence_FB0F(Regs *r)
{
    REG_W(0x4004, (RAM8(0xD9) & 0xC0) | 0x30);   /* SQ2_VOL */
    RAM8(0x27) = (u8)(RAM8(0x27) & 0xFD);
}

void sfx_overlay_voice(Regs *r)
{
    int start = 0;                       /* take L_FA79 (begin/restart SFX) */

    if (RAM8(0x8F) != 0) {               /* LDA $8F / BEQ L_FA74 */
        if ((RAM8(0xD4) & 0x80) == 0) {  /* LDA $D4 / BPL L_FA79 */
            start = 1;
        } else if (RAM8(0x90) >= RAM8(0x91)) { /* LDA $90 / CMP $91 / BCS L_FA79 */
            start = 1;
        } else {
            RAM8(0x90) = 0x00;
            RAM8(0x8F) = 0x00;
        }
    }

    if (!start) {
        /* L_FA74 */
        if ((RAM8(0xD4) & 0x80) == 0)    /* LDA $D4 / BMI L_FA9E; else RTS */
            return;
        /* L_FA9E */
        if ((u8)(--RAM8(0xD3)) != 0)     /* DEC $D3 / BEQ L_FAA5; else L_FAF8 */
            goto L_FAF8;
    } else {
        /* L_FA79 */
        u8 x;
        RAM8(0x91) = RAM8(0x90);         /* LDA $90 / STA $91 */
        x = (u8)(RAM8(0x8F) << 1);       /* LDA $8F / ASL / TAX */
        RAM8(0xD5) = RAM8((u16)(0x8014 + x));   /* ROM table */
        RAM8(0xD6) = RAM8((u16)(0x8015 + x));
        RAM8(0xD4) = 0x80;
        RAM8(0xA4) = (u8)(RAM8(0xA4) | 0x40);
        RAM8(0x8F) = 0x00;
        RAM8(0x90) = 0x00;
        /* JMP L_FAA5 */
    }

    /* L_FAA5 loop */
    for (;;) {
        u16 ptr = (u16)(RAM8(0xD5) | (RAM8(0xD6) << 8));
        u8 note = RAM8(ptr);             /* LDA ($D5),Y  Y=0 */
        if (note == 0) {                 /* BEQ L_FAB7 */
            RAM8(0xD4) = 0x00;
            RAM8(0x91) = 0x00;
            RAM8(0xA4) = (u8)(RAM8(0xA4) & 0xBF);
            silence_FB0F(r);             /* JMP L_FB0F */
            return;
        }
        if (note == 0xFF) {              /* CMP #$FF / BNE L_FAC6 not taken */
            sub_FB8E(r);
            continue;                    /* JMP L_FAA5 */
        }
        /* L_FAC6 */
        inc16_95(r);                     /* JSR FD6B (A unchanged = note) */
        RAM8(0xD3) = (u8)(note & 0x7F);
        if (note & 0x80) {               /* PLP / BMI L_FAF2 */
            sub_FCDF(r);
        } else {
            sub_FC81(r);
            RAM8(0x27) = (u8)(0x02 | RAM8(0x27));
            REG_W(0x4005, RAM8(0xDA));               /* SQ2_SWEEP */
            REG_W(0x4006, RAM8(0x04));               /* SQ2_LO */
            REG_W(0x4007, (RAM8(0x05) & 0x07) | 0xC0); /* SQ2_HI */
            sub_FCC4(r);
        }
        break;                           /* both paths JMP L_FAF8 */
    }

L_FAF8:
    if ((RAM8(0x27) & 0x02) == 0)        /* LDA $27 / AND #$02 / BNE L_FAFF; else RTS */
        return;
    /* L_FAFF */
    if ((u8)(--RAM8(0xDD)) == 0) {       /* DEC $DD / BNE L_FB09 */
        sub_FD11(r);
        REG_W(0x4004, r->a);             /* STA SQ2_VOL */
    }
    /* L_FB09 */
    sub_FD45(r);
    if (r->c)                            /* BCS L_FB0F; else RTS */
        silence_FB0F(r);
}
