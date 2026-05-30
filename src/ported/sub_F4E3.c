/* $F4E3:
 *   LDX $F1 / BNE L_F4E9 / LDX #$19
 * L_F4E9:
 *   DEX / STX $F1
 *   TXA / LSR A / LSR A / EOR #$FF / CLC / ADC #$01 / STA $F7  ; $F7 = -(X>>2)
 *   JSR L_F506            ; sub_F506
 *   BCS L_F4FC / RTS      ; C clear -> return
 * L_F4FC:
 *   LDA #$00 / STA $F5 / STA $F6
 *   JSR L_F506            ; sub_F506
 *   RTS
 */
#include "ram.h"
#include "regs.h"

void sub_F506(Regs *r);

void sub_F4E3(Regs *r)
{
    u8 x = RAM8(0xF1);
    if (x == 0)                /* BNE not taken */
        x = 0x19;
    x = (u8)(x - 1);           /* DEX */
    RAM8(0xF1) = x;
    r->x = x;
    /* TXA / LSR A / LSR A / EOR #$FF / CLC / ADC #$01 */
    RAM8(0xF7) = (u8)(((x >> 2) ^ 0xFF) + 1);

    sub_F506(r);
    if (!r->c)                 /* BCS L_F4FC not taken -> RTS */
        return;

    RAM8(0xF5) = 0x00;
    RAM8(0xF6) = 0x00;
    sub_F506(r);
}
