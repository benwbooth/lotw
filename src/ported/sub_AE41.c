/* $AE41 — validate the candidate position ($0A=Y, $0E=X from ACC5). Returns
 * carry SET (blocked/out-of-bounds) if Y >= $A1, or if X >= $F1; otherwise
 * carry CLEAR (in bounds). Only output is the carry flag. RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_AE41(Regs *r)
{
    if (RAM8(0x0A) >= 0xA1) {                       /* CMP #$A1 / BCS L_AE4D */
        r->c = 1;                                   /* SEC */
        return;
    }
    if (RAM8(0x0E) >= 0xF1) {                       /* CMP #$F1 / BCC L_AE4F else fall to L_AE4D */
        r->c = 1;                                   /* SEC */
        return;
    }
    r->c = 0;                                       /* L_AE4F: CLC */
}
