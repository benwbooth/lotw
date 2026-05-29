/* $D038:
 *   LDX equipped_item ($55)
 *   LDA $51,X / CMP #$06 / BNE L_D04D   ; B5 = zero-page,X (wraps & 0xFF)
 *   LDA magic ($59) / BEQ L_D04D
 *   LDA stat_jump ($5C) / LSR A / LSR A / CLC / ADC stat_jump / CLC / RTS
 * L_D04D:
 *   LDA stat_jump / SEC / RTS
 * Effective jump stat: if equipped item==6 and magic!=0, jump = (jump>>2)+jump,
 * carry clear; otherwise jump unchanged, carry set. Returns A and carry.
 */
#include "ram.h"
#include "regs.h"

void sub_D038(Regs *r)
{
    u8 x = RAM8(0x55);                   /* equipped_item */
    u8 item = RAM8((0x51 + x) & 0xFF);   /* LDA $51,X zero-page,X (wraps) */
    r->x = x;
    if (item == 0x06 && magic != 0) {
        u8 jump = RAM8(0x5C);            /* stat_jump */
        r->a = (u8)((jump >> 2) + jump);
        r->c = 0;                        /* CLC */
    } else {
        r->a = RAM8(0x5C);               /* stat_jump */
        r->c = 1;                        /* SEC */
    }
}
