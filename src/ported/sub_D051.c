/* $D051:
 *   LDX equipped_item ($55) / LDA carried_item0,X ($51+X) / CMP #$08 / BNE .else
 *   LDA magic ($59) / BEQ .else
 *   LDA stat_strength ($5D) / ASL / ASL / CLC / RTS    ; A = strength*4, carry clear
 * .else (D063): LDA stat_strength / SEC / RTS          ; A = strength, carry set
 * Returns attack strength in A plus a carry flag callers test. */
#include "ram.h"
#include "regs.h"

void sub_D051(Regs *r)
{
    u8 x = RAM8(0x55);                  /* equipped_item */
    u8 item = RAM8((0x51 + x) & 0xFF);  /* carried_item0,X (zp,X wraps) */

    if (item == 0x08 && RAM8(0x59) != 0) {   /* equipped item 8 and magic > 0 */
        r->a = (u8)(RAM8(0x5D) << 2);        /* strength * 4 */
        r->c = 0;                            /* CLC */
    } else {
        r->a = RAM8(0x5D);                   /* strength */
        r->c = 1;                            /* SEC */
    }
}
