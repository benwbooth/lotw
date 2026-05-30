/* $FB8E (sound command dispatcher):
 *   LDX $02 / JSR inc16_95 / LDA ($95,X) / STA $04   ; cmd byte from stream
 *   JSR inc16_95 / LDA ($95,X) / STA $05             ; arg byte from stream
 *   JSR inc16_95
 *   LDA $04 / CMP #$05 / BCC L_FBA8 / RTS            ; only cmds 0..4 dispatch
 * L_FBA8:
 *   ASL A / TAX / LDA $FBBB,X / STA $06 / LDA $FBBC,X / STA $07  ; table lookup
 *   LDA $05 / LDX $02 / JMP ($0006)                  ; A=arg, X=channel
 *
 * Table @ $FBBB: FBC5, FBE2, FBFF, FC02, FC05.
 * At the indirect jump the handler sees A = $05 (arg), X = $02 (channel).
 *
 * NOTE: the stream pointer is ($95,X) indexed-indirect with X = $02. The zp,X
 * address wraps within page 0, and inc16_95 (LDX $02 / INC $95,X) can modify
 * $02 itself when $95+$02 wraps onto $02/$03. The register X used by each
 * LDA ($95,X) is exactly the value the preceding inc16_95 left (= mem[$02] as
 * read at the start of that call), so we read the deref index from r->x, which
 * inc16_95 sets, NOT by re-reading RAM8($02). */
#include "ram.h"
#include "regs.h"

void sub_FBE2(Regs *r);
void sub_FBC5(Regs *r);
void sub_FBFF(Regs *r);
void sub_FC02(Regs *r);
void sub_FC05(Regs *r);
void inc16_95(Regs *r);

static u8 deref_stream(Regs *r)
{
    /* LDA ($95,X) with X = r->x (left by the preceding inc16_95) */
    u8 x = r->x;
    u8 lo = RAM8((0x95 + x) & 0xFF);
    u8 hi = RAM8((0x96 + x) & 0xFF);
    return RAM8((u16)(lo | (hi << 8)));
}

void sub_FB8E(Regs *r)
{
    r->x = RAM8(0x02);          /* LDX $02 */

    inc16_95(r);                /* advances stream; leaves X = mem[$02] read inside */
    RAM8(0x04) = deref_stream(r);

    inc16_95(r);
    RAM8(0x05) = deref_stream(r);

    inc16_95(r);

    u8 idx = RAM8(0x04);
    if (idx >= 0x05)
        return;

    /* table pointer mirrored into $06/$07 */
    static const u16 tbl[5] = { 0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05 };
    u16 p = tbl[idx];
    RAM8(0x06) = (u8)(p & 0xFF);
    RAM8(0x07) = (u8)(p >> 8);

    r->a = RAM8(0x05);          /* LDA $05 */
    r->x = RAM8(0x02);          /* LDX $02 */

    switch (idx) {
    case 0: sub_FBC5(r); break;
    case 1: sub_FBE2(r); break;
    case 2: sub_FBFF(r); break;
    case 3: sub_FC02(r); break;
    case 4: sub_FC05(r); break;
    }
}
