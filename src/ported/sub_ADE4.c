/* $ADE4 — jump/fall update. If already falling ($4F != 0) return with carry
 * clear. While player_y < $A0 keep rising: increment the rise counter $4E and
 * return. Once at/above $A0, convert the accumulated rise $4E into a fall speed:
 * clamp to stat_jump, subtract 7 and re-clamp, store ($value-1) into $4F, set
 * the bump-sound flag $8F=$0A; then clear $4E. Pure RAM math, RTS. No callees.
 * Returns carry clear on the early $4F path (BCC consumers); other paths RTS
 * with whatever carry the last op left (not consumed by callers). */
#include "ram.h"
#include "regs.h"

void sub_ADE4(Regs *r)
{
    if (RAM8(0x4F) != 0) {                          /* LDA $4F / BEQ L_ADEA */
        r->c = 0;                                   /* CLC / RTS */
        return;
    }
    /* L_ADEA */
    if (RAM8(0x45) < 0xA0) {                         /* player_y / CMP #$A0 / BCS L_ADF3 */
        RAM8(0x4E) = (u8)(RAM8(0x4E) + 1);          /* INC $4E / RTS */
        return;
    }
    /* L_ADF3 */
    {
        u8 a = RAM8(0x4E);                          /* LDA $4E */
        if (a >= RAM8(0x5C)) {                       /* CMP stat_jump / BCC L_AE0C */
            a = (u8)(a - 0x07);                     /* SEC / SBC #$07 */
            if (a >= RAM8(0x5C))                     /* CMP stat_jump / BCC L_AE02 */
                a = RAM8(0x5C);                     /* LDA stat_jump */
            /* L_AE02 */
            a = (u8)(a - 0x01);                     /* SEC / SBC #$01 */
            RAM8(0x4F) = a;                         /* STA $4F */
            RAM8(0x8F) = 0x0A;                      /* LDA #$0A / STA $008F */
        }
    }
    /* L_AE0C */
    RAM8(0x4E) = 0x00;                              /* LDA #$00 / STA $4E */
}
