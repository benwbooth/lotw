/* $EEBB:
 *   LDA $F0 / LSR A / CLC / ADC #$02 / STA $F7
 *   JSR L_F0E1            ; sub_F0E1
 *   BCS L_EEC9 / RTS      ; C clear -> return
 * L_EEC9:
 *   LDA #$00 / STA $F5 / STA $F6
 *   JSR L_F0E1            ; sub_F0E1
 *   BCS L_EED5 / RTS      ; C clear -> return
 * L_EED5:
 *   LDA #$00 / STA $F7 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_F0E1(Regs *r);

void sub_EEBB(Regs *r)
{
    RAM8(0xF7) = (u8)((RAM8(0xF0) >> 1) + 0x02);
    sub_F0E1(r);
    if (!r->c)                 /* BCS L_EEC9 not taken -> RTS */
        return;

    RAM8(0xF5) = 0x00;
    RAM8(0xF6) = 0x00;
    sub_F0E1(r);
    if (!r->c)                 /* BCS L_EED5 not taken -> RTS */
        return;

    RAM8(0xF7) = 0x00;
}
