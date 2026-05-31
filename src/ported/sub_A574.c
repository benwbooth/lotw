/* $A574 (bank13):
 *   LDA $FA / BNE L_A59D
 *     ; first call ($FA==0): initialise the column run
 *     LDA #$0E / STA vram_dst_lo ($16)
 *     LDA #$20 / STA vram_dst_hi ($17)
 *     LDA $1D / EOR #$01 / ASL A / ASL A / ORA vram_dst_hi / STA vram_dst_hi
 *     LDA $1D / EOR #$01 / ASL A x4 / CLC / ADC #$07 / ORA scroll_x_tile ($7C) / STA $F9
 *     LDA #$09 / STA $FA
 *   L_A59D:
 *     LDA $F9 / STA $0C
 *     JSR $C833                 ; farcall_bank_09_r7
 *     INC vram_dst_lo / INC vram_dst_lo
 *     INC $F9
 *     DEC $FA / BNE L_A5B4
 *       LDA $1D / EOR #$01 / STA $1D
 *   L_A5B4: RTS
 */
#include "ram.h"
#include "regs.h"

void farcall_bank_09_r7(Regs *r);

void sub_A574(Regs *r)
{
    if (RAM8(0xFA) == 0) {
        RAM8(0x16) = 0x0E;                          /* vram_dst_lo */
        RAM8(0x17) = 0x20;                          /* vram_dst_hi */
        RAM8(0x17) = (u8)(((u8)((RAM8(0x1D) ^ 0x01) << 2)) | RAM8(0x17));
        RAM8(0xF9) = (u8)(((u8)((((RAM8(0x1D) ^ 0x01) << 4) + 0x07)) ) | RAM8(0x7C));
        RAM8(0xFA) = 0x09;
    }

    RAM8(0x0C) = RAM8(0xF9);
    farcall_bank_09_r7(r);

    RAM8(0x16) = (u8)(RAM8(0x16) + 1);
    RAM8(0x16) = (u8)(RAM8(0x16) + 1);
    RAM8(0xF9) = (u8)(RAM8(0xF9) + 1);

    RAM8(0xFA) = (u8)(RAM8(0xFA) - 1);
    if (RAM8(0xFA) == 0)
        RAM8(0x1D) ^= 0x01;
}
