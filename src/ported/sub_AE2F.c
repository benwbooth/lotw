/* $AE2F — apply damage A to health, clamping at 0. health -= A; if it
 * underflowed (borrow), force health = 0. PHP/PLP preserve the SBC flags so the
 * routine returns with the SBC's carry/zero/negative (carry clear iff damage
 * exceeded health). A is the damage input. RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_AE2F(Regs *r)
{
    u8 lhs, res;
    RAM8(0x08) = r->a;                              /* STA $08 (damage) */
    lhs = RAM8(0x58);                               /* LDA health */
    /* SEC / SBC $08 */
    {
        u16 t = (u16)lhs - (u16)RAM8(0x08);
        res = (u8)t;
        /* PHP captures flags here: carry = no borrow */
        r->c = (t & 0x100) ? 0 : 1;
        r->z = (res == 0) ? 1 : 0;
        r->n = (res >> 7) & 1;
    }
    RAM8(0x58) = res;                               /* STA health */
    if (!r->c) {                                    /* BCS L_AE3F else clamp */
        RAM8(0x58) = 0x00;                          /* LDA #$00 / STA health */
    }
    /* PLP restores the captured (SBC) flags as the return flags (already in r) */
}
