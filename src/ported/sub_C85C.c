/* $C85C:  LDA #$00/STA $0D / JSR L_CA54 / LDA $0D/SEC/SBC #$05/CLC/ADC $76/STA $0D
 *         / JSR metasprite_build / RTS
 * Zeroes $0D, runs sub_CA54, then $0D = ($0D - 5) + $76, then metasprite_build. */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void metasprite_build(Regs *r);

void sub_C85C(Regs *r)
{
    RAM8(0x0D) = 0x00;
    sub_CA54(r);
    RAM8(0x0D) = (u8)((u8)(RAM8(0x0D) - 0x05) + RAM8(0x76));
    metasprite_build(r);
}
