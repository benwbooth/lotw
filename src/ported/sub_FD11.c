/* $FD11: sound envelope step for channel X=$02. Looks up a base value via $9B,X
 * index into $FDCC, advances $9F,X toward a target (clamped to $0F or $00 depending
 * on the sign of $9C,X), runs L_FCB4 over $A0,X, and merges the result into the
 * channel-$99,X control byte (keep top 2 bits, OR step, OR #$30).
 *   LDX $02 / LDY $9B,X / LDA $FDCC,Y / STA $9D,X
 *   LDA $9C,X / BMI neg
 *     CLC / ADC $9F,X / CMP #$10 / BCC store / LDA #$0F / JMP store
 *   neg: CLC / ADC $9F,X / CMP #$10 / BCC store / LDA #$00
 *   store: STA $9F,X / STA $00 / LDY $A0,X / JSR L_FCB4
 *   LDA $99,X / AND #$C0 / ORA $00 / ORA #$30 / RTS   ; A = result
 */
#include "ram.h"
#include "regs.h"

void sub_FCB4(Regs *r);

void sub_FD11(Regs *r)
{
    u8 x = RAM8(0x02);
    u8 idx = RAM8((u8)(0x9B + x));
    RAM8((u8)(0x9D + x)) = RAM8((u16)(0xFDCC + idx));

    {
        u8 v = RAM8((u8)(0x9C + x));
        u8 a = (u8)(v + RAM8((u8)(0x9F + x)));   /* CLC / ADC $9F,X */
        if (v & 0x80) {                          /* BMI neg */
            if (a >= 0x10)                       /* CMP #$10 / BCC store */
                a = 0x00;
        } else {
            if (a >= 0x10)
                a = 0x0F;
        }
        RAM8((u8)(0x9F + x)) = a;
        RAM8(0x00) = a;
    }

    r->y = RAM8((u8)(0xA0 + x));
    sub_FCB4(r);                                 /* uses Y, writes $00, A */

    {
        u8 result = (u8)((RAM8((u8)(0x99 + x)) & 0xC0) | RAM8(0x00) | 0x30);
        r->a = result;
    }
}
