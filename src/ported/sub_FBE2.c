/* $FBE2 (sound command handler, X = channel index):
 *   LDA $02 / CMP #$40 / BEQ $FBEC
 *   LDA $92 / BNE $FBFE
 * $FBEC: LDA #$0F / CLC / ADC $05 / SEC / SBC #$08 / BCS $FBF8 / LDA #$00
 * $FBF8: ASL A / CLC / ADC #$01 / STA $A0,X
 * $FBFE: RTS
 * If $02==$40 (or $02!=$40 && $92==0): compute (clamp(($0F+$05)-$08))*2+1 into
 * $A0,X. Otherwise ($02!=$40 && $92!=0) do nothing. X is the channel index. */
#include "ram.h"
#include "regs.h"

void sub_FBE2(Regs *r)
{
    u8 x = r->x;
    u8 a;
    u8 take_fbec;

    a = RAM8(0x02);                       /* LDA $02 ; CMP #$40 */
    if (a == 0x40) {
        take_fbec = 1;
    } else {
        a = RAM8(0x92);                   /* LDA $92 ; BNE $FBFE */
        if (a != 0) {                     /* skip -> RTS */
            r->a = a;
            r->x = x;
            return;
        }
        take_fbec = 1;
    }
    (void)take_fbec;

    /* $FBEC: A = $0F + $05, then SEC/SBC #$08 with borrow check */
    {
        u16 sum = (u16)(0x0F + RAM8(0x05));   /* CLC/ADC $05 */
        int carry_in = 1;                      /* SEC */
        u16 diff = (u16)((sum & 0xFF) - 0x08 + (carry_in - 1));
        u8 bcs = ((sum & 0xFF) >= 0x08);       /* carry set after subtract */
        a = (u8)diff;
        if (!bcs)                              /* BCS skips this LDA #$00 */
            a = 0x00;
        a = (u8)(a << 1);                      /* ASL A */
        a = (u8)(a + 1);                       /* CLC/ADC #$01 */
        RAM8((0xA0 + x) & 0xFF) = a;
    }
    r->a = a;
    r->x = x;
}
