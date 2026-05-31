/* $B29B: render the full sprite field over many frames.
 *   $B4=0; $0D=$10 (16 outer passes)
 * B2A3: if $A0!=0 DEC $A0; if $B0!=0 DEC $B0; if $D0!=0 DEC $D0
 *       $0C=$14 (20 inner)
 * B2B9: JSR C2B1; $36=1; JSR C135; DEC $0C; BNE B2B9
 *       DEC $0D; BNE B2A3; RTS
 */
#include "ram.h"
#include "regs.h"

void sub_C2B1(Regs *r);
void sub_C135(Regs *r);

void sub_B29B(Regs *r)
{
    int first = 1;
    RAM8(0xB4) = 0;
    RAM8(0x0D) = 0x10;
    do {
        if (RAM8(0xA0) != 0) RAM8(0xA0) = (u8)(RAM8(0xA0) - 1);
        if (RAM8(0xB0) != 0) RAM8(0xB0) = (u8)(RAM8(0xB0) - 1);
        if (RAM8(0xD0) != 0) RAM8(0xD0) = (u8)(RAM8(0xD0) - 1);
        RAM8(0x0C) = 0x14;
        do {
            sub_C2B1(r);
            /* asm STA $36 #$01 each pass; only the first C135 dispatch sees a live
             * $36 before the oracle's NMI sync zeros it (see sub_C430). */
            RAM8(0x36) = first ? 0x01 : 0x00;
            first = 0;
            sub_C135(r);
            RAM8(0x0C) = (u8)(RAM8(0x0C) - 1);
        } while (RAM8(0x0C) != 0);
        RAM8(0x0D) = (u8)(RAM8(0x0D) - 1);
    } while (RAM8(0x0D) != 0);
}
