/* $DF37:  convert a position into VRAM-related bytes.
 *   LDA $0B / CMP #$0C / BCC + / SBC #$0C / INC $0F
 * + TAY / BEQ ++ / LDA $0A / CLC / ADC #$10 / STA $0A
 * ++ LDA $0A / AND #$F0 / STA $FB
 *    LDA #$00 / STA $FC
 *    LDA $0F / STA $FA
 *    LDA #$00 / STA $F9 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_DF37(Regs *r)
{
    u8 a = RAM8(0x0B);
    u8 y;

    if (a >= 0x0C) {            /* CMP #$0C; BCC + -> not taken means a>=0C, carry set */
        a = (u8)(a - 0x0C);     /* SBC #$0C, carry set -> plain subtract */
        RAM8(0x0F)++;           /* INC $0F */
    }
    y = a;                      /* TAY */
    if (y != 0) {               /* BEQ ++ */
        RAM8(0x0A) = (u8)(RAM8(0x0A) + 0x10);   /* CLC; ADC #$10 */
    }
    RAM8(0xFB) = RAM8(0x0A) & 0xF0;
    RAM8(0xFC) = 0x00;
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xF9) = 0x00;
    r->a = 0x00;
    r->y = y;
}
