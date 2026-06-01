/* $C3E5: fade-down (like C430) with extra side effects.
 *   INC $92
 *   LDY #$04
 * C3E9: push Y; $36=5; LDX #$1C
 * C3F1: A = $0184,X & $0F -> $08
 *       A = $0184,X & $F0; SEC; SBC #$10
 *       if BCS: A = (A) ORA $08  else A = $0F
 *       STA $0184,X; DEX; BPL C3F1
 *       LSR $A0; LSR $B0; LSR $D0; $B4=0
 *       JSR C135
 *       pull Y; DEY; BNE C3E9
 *   $8E=$FF; $94=0; $A4=0; $C4=0; $92=0; RTS
 */
#include "ram.h"
#include "regs.h"

void sub_C135(Regs *r);

void sub_C3E5(Regs *r)
{
    u8 y;
    RAM8(0x92) = (u8)(RAM8(0x92) + 1);
    y = 0x04;
    {
#ifndef LOTW_SHIM
    int first = 1;
#endif
    do {
        int x;
        /* see sub_C430: asm STA $36 #$05 every pass; faithful under integration,
         * oracle zeros $36 after pass 1 in the diff-test. */
#ifdef LOTW_SHIM
        RAM8(0x36) = 0x05;
#else
        RAM8(0x36) = first ? 0x05 : 0x00;
        first = 0;
#endif
        for (x = 0x1C; x >= 0; --x) {
            u8 v = RAM8((u16)(0x0184 + x));
            u8 lo = v & 0x0F;
            u8 hi = v & 0xF0;
            u8 sub = (u8)(hi - 0x10);
            RAM8(0x08) = lo;           /* STA $08 (low nibble) */
            if (hi >= 0x10)
                RAM8((u16)(0x0184 + x)) = (u8)(sub | lo);
            else
                RAM8((u16)(0x0184 + x)) = 0x0F;
        }
        RAM8(0xA0) >>= 1;
        RAM8(0xB0) >>= 1;
        RAM8(0xD0) >>= 1;
        RAM8(0xB4) = 0;
        sub_C135(r);
    } while (--y != 0);
    }

    RAM8(0x8E) = 0xFF;
    RAM8(0x94) = 0;
    RAM8(0xA4) = 0;
    RAM8(0xC4) = 0;
    RAM8(0x92) = 0;
}
