/* $C5CB:
 *   LDA scroll_x_tile($7C) / AND #$FE / STA $0C
 *   LDA #$00 / STA $0D
 *   JSR L_CA54        ; sub_CA54
 *   JSR L_C5F7        ; sub_C5F7 (PPU strip render)
 *   RTS
 * Sets up pointer $0C/$0D = (scroll_x_tile & $FE), runs CA54 then C5F7. */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_C5F7(Regs *r);

void sub_C5CB(Regs *r)
{
    RAM8(0x0C) = RAM8(0x7C) & 0xFE;
    RAM8(0x0D) = 0x00;
    sub_CA54(r);
    sub_C5F7(r);
}
