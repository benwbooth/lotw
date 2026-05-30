/* $CE90:  SEC / LDA $0F / SBC player_x_tile / BEQ set / CMP #$02 / BCC one
 *         CMP #$FF / BCC clr / SEC / LDA $0E / SBC player_x_fine / BEQ clr / BMI clr / JMP set
 *  one:   LDA $0E / SEC / SBC player_x_fine / BMI set / (fall) clr
 *  clr: CLC RTS    set: SEC RTS
 * Returns carry = "player within +/-1 tile in X (with fine tiebreak)". Output: carry. */
#include "ram.h"
#include "regs.h"

#define player_x_fine RAM8(0x43)
#define player_x_tile RAM8(0x44)

void sub_CE90(Regs *r)
{
    u8 d = (u8)(RAM8(0x0F) - player_x_tile);   /* SEC SBC -> exact */

    if (d == 0) { r->c = 1; return; }          /* BEQ set */
    if (d < 0x02) {                            /* CMP #$02 BCC -> d==1 */
        u8 f = (u8)(RAM8(0x0E) - player_x_fine);
        r->c = (f & 0x80) ? 1 : 0;             /* BMI set, else clr */
        return;
    }
    if (d < 0xFF) { r->c = 0; return; }        /* CMP #$FF BCC -> clr */
    /* d == $FF */
    {
        u8 f = (u8)(RAM8(0x0E) - player_x_fine);
        if (f == 0)        { r->c = 0; return; }   /* BEQ clr */
        if (f & 0x80)      { r->c = 0; return; }   /* BMI clr */
        r->c = 1;                                   /* set */
    }
}
