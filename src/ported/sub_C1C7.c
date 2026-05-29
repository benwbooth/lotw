/* $C1C7:  LDA scroll_x_tile ($7C) / ASL A x4 / ORA scroll_x_fine ($7B) / TAX
 *         LDA #$00 / ROL A / STX $1C / STA $1D / RTS
 * 16-bit scroll-x = (tile<<4 | fine): low byte -> $1C (and X), and the carry
 * out of the 4th ASL (bit 7 of tile<<3, i.e. bit 4 of tile) rolls into $1D. */
#include "ram.h"
#include "regs.h"

void sub_C1C7(Regs *r)
{
    u8 a = RAM8(0x7C);              /* scroll_x_tile */
    u8 carry = 0;
    int i;
    for (i = 0; i < 4; i++) {       /* ASL A x4 */
        carry = (u8)(a >> 7);
        a = (u8)(a << 1);
    }
    a |= RAM8(0x7B);                /* ORA scroll_x_fine */
    r->x = a;                       /* TAX */
    a = (u8)(0x00 << 1) | carry;    /* LDA #$00 / ROL A */
    RAM8(0x1C) = r->x;
    RAM8(0x1D) = a;
    r->a = a;
}
