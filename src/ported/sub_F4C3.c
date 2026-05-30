/* $F4C3:
 *   LDA $F0 / LSR A / LSR A / CLC / ADC #$01 / STA $F7
 *   JSR L_F506 / BCS L_F4D2 / RTS
 * L_F4D2:
 *   LDA #$00 / STA $F5 / STA $F6
 *   JSR L_F0E1 / BCS L_F4DE / RTS
 * L_F4DE:
 *   LDA #$00 / STA $F7 / RTS
 *
 * F7 = ($F0 >> 2) + 1; call F506. If C clear -> return.
 * Else clear F5/F6, call F0E1. If C clear -> return.
 * Else F7 = 0 and return.
 */
#include "ram.h"
#include "regs.h"

void sub_F506(Regs *r);
void sub_F0E1(Regs *r);

void sub_F4C3(Regs *r)
{
    u8 a = (u8)(RAM8(0xF0) >> 2);
    a = (u8)(a + 1);          /* CLC / ADC #$01 */
    RAM8(0xF7) = a;
    r->a = a;

    sub_F506(r);
    if (!r->c)               /* BCS L_F4D2 not taken -> RTS */
        return;

    /* L_F4D2 */
    RAM8(0xF5) = 0x00;
    RAM8(0xF6) = 0x00;
    r->a = 0x00;
    sub_F0E1(r);
    if (!r->c)               /* BCS L_F4DE not taken -> RTS */
        return;

    /* L_F4DE */
    RAM8(0xF7) = 0x00;
    r->a = 0x00;
}
