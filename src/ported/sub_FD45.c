/* $FD45:  animation-frame stepper, X = $02.
 *   LDX $02 / DEC $9E,X / BNE L_FD69            ; countdown timer; not yet zero -> CLC,RTS
 *   LDA $9B,X / AND #$0F / CMP #$0C / BCS L_FD6A ; phase index hit end -> RTS (carry set)
 *   LDA $9B,X / ADC #$04 / TAY / STY $9B,X       ; advance phase by 4 (carry clear from CMP)
 *   LDA $FDCB,Y / STA $9C,X
 *   LDA $FDCC,Y / STA $9D,X
 *   LDA $FDCD,Y / STA $9E,X
 * L_FD69 CLC
 * L_FD6A RTS
 */
#include "ram.h"
#include "regs.h"

void sub_FD45(Regs *r)
{
    u8 x = RAM8(0x02);
    u8 a, y;

    if (--RAM8((0x9E + x) & 0xFF) != 0) {   /* DEC / BNE L_FD69 */
        r->x = x;
        r->c = 0;                           /* CLC */
        return;
    }
    a = RAM8((0x9B + x) & 0xFF) & 0x0F;
    if (a >= 0x0C) {                        /* CMP #$0C / BCS L_FD6A */
        r->x = x;
        r->a = a;
        r->c = 1;
        return;
    }
    /* carry is clear here (BCC not taken at CMP), so ADC #$04 == +4 */
    y = (u8)(RAM8((0x9B + x) & 0xFF) + 0x04);
    RAM8((0x9B + x) & 0xFF) = y;
    RAM8((0x9C + x) & 0xFF) = RAM8((u16)(0xFDCB + y));
    RAM8((0x9D + x) & 0xFF) = RAM8((u16)(0xFDCC + y));
    RAM8((0x9E + x) & 0xFF) = RAM8((u16)(0xFDCD + y));
    r->x = x;
    r->y = y;
    r->c = 0;                              /* CLC */
}
