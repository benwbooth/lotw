/* $C76C:
 *   JSR L_CC97                 ; wait for vblank_vram_req ($28) to be idle
 *   compute VRAM dest from scroll_x_tile ($7C):
 *     vram_dst_lo ($16) = (scroll_x_tile<<1) & $1F
 *     vram_dst_hi ($17) = (scroll_x_tile & $10) >> 2
 *     CLC; vram_dst_lo = $00 + vram_dst_lo
 *          vram_dst_hi = $20 + vram_dst_hi
 *   $08 = scroll_x_tile; $09 = $10 (16 iterations)
 *   loop: $0C = $08; JSR farcall_bank_09_r7
 *         vram_dst_lo += 2; if (vram_dst_lo & $20) { vram_dst_lo=0; vram_dst_hi ^= $04 }
 *         $08++; $09--; until $09==0
 */
#include "ram.h"
#include "regs.h"

void sub_CC97(Regs *r);
void farcall_bank_09_r7(Regs *r);

void sub_C76C(Regs *r)
{
    u8 sx;

    sub_CC97(r);

    sx = RAM8(0x7C);
    RAM8(0x16) = (u8)((sx << 1) & 0x1F);
    RAM8(0x17) = (u8)((sx & 0x10) >> 2);
    RAM8(0x16) = (u8)(0x00 + RAM8(0x16));
    RAM8(0x17) = (u8)(0x20 + RAM8(0x17));

    RAM8(0x08) = sx;
    RAM8(0x09) = 0x10;

    do {
        RAM8(0x0C) = RAM8(0x08);
        farcall_bank_09_r7(r);
        RAM8(0x16) = (u8)(RAM8(0x16) + 2);
        if (RAM8(0x16) & 0x20) {
            RAM8(0x16) = 0x00;
            RAM8(0x17) ^= 0x04;
        }
        RAM8(0x08) = (u8)(RAM8(0x08) + 1);
        RAM8(0x09) = (u8)(RAM8(0x09) - 1);
    } while (RAM8(0x09) != 0);
}
