/* $C520:  identical code to $B6D0 (different address).
 * loop: LDA $0180,X / AND #$0F / STA $08 / LDA $0180,X / AND #$F0 / SEC SBC $09
 *       BCS + / LDA #$0F / JMP ++ / +: ORA $08 / ++: STA $0180,X / INX / DEY / BNE loop / RTS
 * For Y bytes starting at $0180+X: subtract $09 from the high nibble; on borrow store
 * $0F, else OR the saved low nibble back in. Loops while Y!=0. Inputs X,Y. */
#include "ram.h"
#include "regs.h"

void sub_C520(Regs *r)
{
    u8 x = r->x, y = r->y;
    do {
        u8 lo = RAM8((u16)(0x0180 + x)) & 0x0F;
        RAM8(0x08) = lo;
        u8 hi = RAM8((u16)(0x0180 + x)) & 0xF0;
        u8 sub = RAM8(0x09);
        u8 res;
        if (hi >= sub)                 /* SEC SBC $09 -> BCS (no borrow) */
            res = (u8)((u8)(hi - sub) | lo);
        else
            res = 0x0F;
        RAM8((u16)(0x0180 + x)) = res;
        ++x;
        --y;
    } while (y != 0);
    r->x = x;
    r->y = y;
}
