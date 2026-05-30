/* $C5DC:
 *   LDA scroll_x_tile($7C) / AND #$FE / STA $0C
 *   LDA #$00 / STA $0D
 *   JSR L_CA54        ; sub_CA54
 *   LDA $0D / SEC / SBC #$05 / CLC / ADC $76 / STA $0D
 *   JSR L_C5F7        ; sub_C5F7 (PPU strip render)
 *   RTS
 * Like C5CB but adjusts $0D by (-5 + $76) after CA54, before C5F7. */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_C5F7(Regs *r);

void sub_C5DC(Regs *r)
{
    RAM8(0x0C) = RAM8(0x7C) & 0xFE;
    RAM8(0x0D) = 0x00;
    sub_CA54(r);
    /* LDA $0D / SEC / SBC #$05 / CLC / ADC $76 / STA $0D */
    RAM8(0x0D) = (u8)((RAM8(0x0D) - 0x05) + RAM8(0x76));
    sub_C5F7(r);
}
