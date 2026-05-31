/* $C540: clear-then-refresh palette buffer, X passes.
 *   TXA / PHA                  ; save loop count (X on entry)
 *   LDA #$30 / LDX #$1F
 * C546: STA $0180,X / DEX / BPL C546   ; fill $0180..$019F with $30
 *   JSR C569                   ; palette VRAM refresh
 *   LDA #$01 / STA $36 / JSR C135
 *   JSR C9FB                   ; reload palette buffer from ($77)
 *   JSR C569
 *   LDA #$02 / STA $36 / JSR C135
 *   PLA / TAX / DEX / BNE C540  ; repeat X-1 more... actually X total passes
 *   RTS
 * Input: X = pass count (loop runs while DEX != 0, so X iterations).
 */
#include "ram.h"
#include "regs.h"

void sub_C569(Regs *r);
void sub_C135(Regs *r);
void sub_C9FB(Regs *r);

void sub_C540(Regs *r)
{
    u8 x = r->x;
    int first = 1;
    do {
        int i;
        for (i = 0x1F; i >= 0; --i)
            RAM8((u16)(0x0180 + i)) = 0x30;
        sub_C569(r);
        /* asm STA $36 each pass; only the first C135 dispatch sees a live $36
         * before the oracle's NMI sync zeros it (see sub_C430). */
        RAM8(0x36) = first ? 0x01 : 0x00;
        first = 0;
        sub_C135(r);
        sub_C9FB(r);
        sub_C569(r);
        RAM8(0x36) = 0x00;
        sub_C135(r);
    } while (--x != 0);
    r->x = x;
}
