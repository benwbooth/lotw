/* $D017:
 *   PHA / LDA cur_character / ASL A / TAX / PLA
 *   CMP #$08 / BCC + / INX
 *   +: AND #$07 / TAY / INY
 *   LDA $FFBB,X / -: ASL A / DEY / BNE - / RTS
 * Input A (item index). X = cur_character*2 (+1 if A>=8). Y = (A&7)+1.
 * Returns A = ROM[$FFBB+X] shifted left Y times (bit mask for that slot).
 */
#include "ram.h"
#include "regs.h"

void sub_D017(Regs *r)
{
    u8 in = r->a;
    u8 x = (u8)(cur_character << 1);     /* ASL A / TAX */
    if (in >= 0x08)                      /* CMP #$08 / BCC -> else INX */
        x++;
    u8 y = (u8)((in & 0x07) + 1);        /* AND #$07 / TAY / INY */
    u8 a = RAM8((u16)(0xFFBB + x));      /* LDA $FFBB,X */
    do {                                 /* ASL A / DEY / BNE */
        a = (u8)(a << 1);
    } while (--y != 0);
    r->a = a;
}
