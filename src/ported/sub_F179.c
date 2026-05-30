/* $F179 — entity placement / collision pre-check. Builds pointer $0C/$0D (and
 * scratch $0E,$0F,$0A) from $F9/$FA/$FB, then validates a candidate map cell via
 * CE7C (proximity), CA54 (pointer build), and F233 (map-tile probes). On success
 * INC $F0 / CLC; on failure resets $F0 (and maybe bumps $F1) / SEC.
 *
 *   LDA $F1 / BNE fail
 *   LDA $FA / STA $0C / STA $0F ; LDA $F9 / STA $0E
 *   LDX $FB / LDY $EE / DEY / BEQ alt
 *     CPX #$EF / BNE + / LDX $FC
 *   +: STX $0D / JMP go
 *   alt: CPX #$B0 / BCS ok / STX $0D / INX / STX $0A / JSR CE7C / BCS fail
 *   go: JSR CA54
 *       LDA $F9 / BNE skip
 *         LDY 0 / ($0C),Y &$3F / BEQ fail ; LDY 1 / ($0C),Y &$3F / BEQ fail
 *       skip: LDY 1 / JSR F233 / BCS fail
 *             LDA $F9 / BEQ ok ; LDY $0D / JSR F233 / BCS fail
 *   ok: INC $F0 / CLC / RTS
 *   fail: LDA $F0 / CMP #$0C / BCC + / SEC SBC #$04 / STA $F1
 *   +: LDA #0 / STA $F0 / SEC / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_CE7C(Regs *r);
void sub_F233(Regs *r);

static void f179_fail(Regs *r)
{
    /* L_F1D3 */
    if (RAM8(0xF0) >= 0x0C)
        RAM8(0xF1) = (u8)(RAM8(0xF0) - 0x04);
    /* L_F1DE */
    RAM8(0xF0) = 0x00;
    r->c = 1;                 /* SEC */
}

static void f179_ok(Regs *r)
{
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);   /* INC $F0 */
    r->c = 0;                            /* CLC */
}

void sub_F179(Regs *r)
{
    u8 x, y;

    if (RAM8(0xF1) != 0) { f179_fail(r); return; }   /* BNE L_F1D3 */

    RAM8(0x0C) = RAM8(0xFA);
    RAM8(0x0F) = RAM8(0xFA);
    RAM8(0x0E) = RAM8(0xF9);

    x = RAM8(0xFB);
    y = (u8)(RAM8(0xEE) - 1);         /* LDY $EE / DEY */
    if (y == 0) {
        /* L_F199 */
        if (x >= 0xB0) { f179_ok(r); return; }   /* BCS L_F1CF */
        RAM8(0x0D) = x;
        x = (u8)(x + 1);                          /* INX */
        RAM8(0x0A) = x;
        sub_CE7C(r);
        if (r->c) { f179_fail(r); return; }       /* BCS L_F1D3 */
    } else {
        if (x != 0xEF) {
            /* fallthrough keeps X */
        } else {
            x = RAM8(0xFC);          /* LDX $FC */
        }
        RAM8(0x0D) = x;
    }

    /* L_F1A7 */
    sub_CA54(r);

    if (RAM8(0xF9) == 0) {           /* LDA $F9 / BNE L_F1BD */
        u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
        if ((RAM8(ptr) & 0x3F) == 0) { f179_fail(r); return; }
        if ((RAM8((u16)(ptr + 1)) & 0x3F) == 0) { f179_fail(r); return; }
    }

    /* L_F1BD */
    r->y = 0x01;
    sub_F233(r);
    if (r->c) { f179_fail(r); return; }

    if (RAM8(0xF9) == 0) { f179_ok(r); return; }   /* BEQ L_F1CF */

    r->y = 0x0D;
    sub_F233(r);
    if (r->c) { f179_fail(r); return; }

    f179_ok(r);
}
