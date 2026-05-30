/* $F1E4: per-frame proximity/collision check that advances or resets the $F0
 * counter (and rewinds $F1 on bail). Pure RAM effects.
 *
 *   LDA $F1 / BNE bail
 *   LDA $FA / STA $0C / STA $0F
 *   LDA $F9 / STA $0E
 *   LDX $FB / STX $0D / INX / STX $0A
 *   JSR L_CA54
 *   LDA $FB / CMP #$A0 / BCS done            ; $FB>=$A0 -> just INC $F0
 *   JSR L_CEC7 / BCS bail                     ; near player -> bail
 *   LDY #$02 / JSR L_F233 / BCS bail
 *   LDY #$0E / JSR L_F233 / BCS bail
 *   LDA $F9 / BEQ done
 *   LDY #$1A / JSR L_F233 / BCS bail
 * done(L_F220): INC $F0 / RTS
 * bail(L_F223): LDA $F0 / CMP #$0C / BCC +    ; if $F0>=$0C: $F1 = $F0-4
 *               SEC / SBC #$04 / STA $F1
 *           +:  LDA #$00 / STA $F0 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_CEC7(Regs *r);
void sub_F233(Regs *r);

static void bail(void)            /* L_F223 */
{
    u8 f0 = RAM8(0xF0);
    if (f0 >= 0x0C)
        RAM8(0xF1) = (u8)(f0 - 0x04);
    RAM8(0xF0) = 0x00;
}

void sub_F1E4(Regs *r)
{
    if (RAM8(0xF1) != 0) { bail(); return; }

    RAM8(0x0C) = RAM8(0xFA);
    RAM8(0x0F) = RAM8(0xFA);
    RAM8(0x0E) = RAM8(0xF9);
    RAM8(0x0D) = RAM8(0xFB);
    RAM8(0x0A) = (u8)(RAM8(0xFB) + 1);   /* LDX $FB / STX $0D / INX / STX $0A */

    sub_CA54(r);

    if (RAM8(0xFB) >= 0xA0) {             /* L_F220 path */
        RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
        return;
    }

    sub_CEC7(r);
    if (r->c) { bail(); return; }

    r->y = 0x02; sub_F233(r);
    if (r->c) { bail(); return; }

    r->y = 0x0E; sub_F233(r);
    if (r->c) { bail(); return; }

    if (RAM8(0xF9) != 0) {
        r->y = 0x1A; sub_F233(r);
        if (r->c) { bail(); return; }
    }

    /* L_F220 */
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
}
