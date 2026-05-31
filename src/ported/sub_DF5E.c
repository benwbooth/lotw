/* $DF5E:
 *   LDA $FA / STA $0C
 *   ASL A / AND #$1F / STA vram_dst_lo ($16)
 *   LDA $FA / AND #$10 / LSR A / LSR A / STA vram_dst_hi ($17)
 *   CLC / LDA #$00 / ADC vram_dst_lo / STA vram_dst_lo
 *   LDA #$20 / ADC vram_dst_hi / STA vram_dst_hi      ; dst = $2000 + offset
 *   JSR farcall_bank_09_r7
 * Computes a nametable VRAM destination from $FA, sets pointer $0C=$FA, then
 * runs the bank-9 metasprite-column far-call.
 */
#include "ram.h"
#include "regs.h"

void farcall_bank_09_r7(Regs *r);

void sub_DF5E(Regs *r)
{
    u8 fa = RAM8(0xFA);

    RAM8(0x0C) = fa;
    RAM8(0x16) = (u8)((fa << 1) & 0x1F);   /* vram_dst_lo */
    RAM8(0x17) = (u8)((RAM8(0xFA) & 0x10) >> 2);  /* vram_dst_hi */

    RAM8(0x16) = (u8)(0x00 + RAM8(0x16));
    RAM8(0x17) = (u8)(0x20 + RAM8(0x17));

    farcall_bank_09_r7(r);
}
