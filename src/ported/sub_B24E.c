/* $B24E:
 *   LDA #$08 / STA vram_dst_hi      ; $17 = $08
 *   LDA $0A
 *   ASL A / ROL vram_dst_hi
 *   ASL A / ROL vram_dst_hi
 *   STA vram_dst_lo                 ; $16
 *   RTS
 * Computes a 16-bit VRAM dst = $0800 + ($0A << 2), split across hi=$17 lo=$16.
 * A returns the low byte ($0A << 2). */
#include "ram.h"
#include "regs.h"

void sub_B24E(Regs *r)
{
    u8 hi = 0x08;
    u8 a = RAM8(0x0A);
    u8 carry;

    carry = a >> 7;  a = (u8)(a << 1);  hi = (u8)((hi << 1) | carry);
    carry = a >> 7;  a = (u8)(a << 1);  hi = (u8)((hi << 1) | carry);

    RAM8(0x17) = hi;
    RAM8(0x16) = a;
    r->a = a;
}
