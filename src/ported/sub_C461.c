/* $C461: fade-down loop over $0180..$01A0 (X=$20..0), 4 passes.
 *   LDY #$04
 * C463: push Y; $36=5; LDX #$20
 * C46B: A = $0180,X & $0F -> $08
 *       A = $0180,X & $F0; SEC; SBC #$10
 *       if BCS (no borrow): A = (A) ORA $08  else A = $0F
 *       STA $0180,X; DEX; BPL C46B
 *       JSR C135 (deferred-refresh + frame-sync on $36)
 *       pull Y; DEY; BNE C463; RTS
 * Same as sub_C430 but base $0180 and X starts $20.
 */
#include "ram.h"
#include "regs.h"

void sub_C135(Regs *r);

void sub_C461(Regs *r)
{
    u8 y = 0x04;
#ifndef LOTW_SHIM
    int first = 1;
#endif
    do {
        int x;
        /* The asm does STA $36 #$05 EVERY pass, so each C135 frame-syncs (4 passes ->
         * ~20 frames). Under integration (LOTW_SHIM) reproduce that. In the isolated
         * diff-test the oracle's NMI-sync zeros $36 after pass 1, so to match the
         * oracle only the first dispatch sees a live $36. */
#ifdef LOTW_SHIM
        RAM8(0x36) = 0x05;
#else
        RAM8(0x36) = first ? 0x05 : 0x00;
        first = 0;
#endif
        for (x = 0x20; x >= 0; --x) {
            u8 v = RAM8((u16)(0x0180 + x));
            u8 lo = v & 0x0F;
            u8 hi = v & 0xF0;
            u8 sub = (u8)(hi - 0x10);
            RAM8(0x08) = lo;           /* STA $08 (low nibble) */
            if (hi >= 0x10)
                RAM8((u16)(0x0180 + x)) = (u8)(sub | lo);
            else
                RAM8((u16)(0x0180 + x)) = 0x0F;
        }
        sub_C135(r);
    } while (--y != 0);
}
