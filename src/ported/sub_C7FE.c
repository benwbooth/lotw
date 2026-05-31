/* $C7FE:
 *   JSR L_CC97                 ; spin-wait on nmi_vram_req ($28)
 *   LDA $7F / BMI L_C80F       ; if $7F negative, use scroll_x_tile as-is
 *     LDA scroll_x_tile ($7C) / CLC / ADC #$10 / STA $0C   ; else col = scroll+$10
 *     JMP L_C813
 *   L_C80F: LDA scroll_x_tile / STA $0C
 *   L_C813:
 *     vram_dst_lo ($16) = ($0C<<1) & $1F
 *     vram_dst_hi ($17) = ($0C & $10) >> 2
 *     CLC; vram_dst_lo = $00 + vram_dst_lo
 *          vram_dst_hi = $20 + vram_dst_hi
 *   JSR farcall_bank_09_r7
 */
#include "ram.h"
#include "regs.h"

void sub_CC97(Regs *r);
void farcall_bank_09_r7(Regs *r);

void sub_C7FE(Regs *r)
{
    u8 col;

    sub_CC97(r);

    if (RAM8(0x7F) & 0x80)
        col = RAM8(0x7C);                       /* scroll_x_tile */
    else
        col = (u8)(RAM8(0x7C) + 0x10);
    RAM8(0x0C) = col;

    RAM8(0x16) = (u8)((col << 1) & 0x1F);       /* vram_dst_lo */
    RAM8(0x17) = (u8)((col & 0x10) >> 2);       /* vram_dst_hi */
    RAM8(0x16) = (u8)(0x00 + RAM8(0x16));
    RAM8(0x17) = (u8)(0x20 + RAM8(0x17));

    farcall_bank_09_r7(r);
}
