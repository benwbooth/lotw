/* $EEDA:
 *   LDX $F1 / BNE L_EEE0 / LDX #$0F
 * L_EEE0:
 *   DEX / STX $F1
 *   TXA / LSR A / EOR #$FF / CLC / ADC #$01 / STA $F7   ; $F7 = -(X>>1)
 *   JSR L_F0E1 / BCS L_EEF2 / RTS         ; C clear -> return (carry from F0E1)
 * L_EEF2:
 *   LDA #$00 / STA $F5 / STA $F6
 *   JSR L_F0E1 / BCS L_EEFE / RTS         ; C clear -> return
 * L_EEFE:
 *   INC $F1 / JSR L_F2DA / RTS            ; returns F2DA carry
 */
#include "ram.h"
#include "regs.h"

void sub_F0E1(Regs *r);
void sub_F2DA(Regs *r);

void sub_EEDA(Regs *r)
{
    u8 x = RAM8(0xF1);
    if (x == 0)                /* BNE not taken */
        x = 0x0F;
    x = (u8)(x - 1);           /* DEX */
    RAM8(0xF1) = x;
    r->x = x;
    /* TXA / LSR A / EOR #$FF / CLC / ADC #$01 */
    RAM8(0xF7) = (u8)(((x >> 1) ^ 0xFF) + 1);

    sub_F0E1(r);
    if (!r->c)                 /* BCS L_EEF2 not taken -> RTS */
        return;

    /* L_EEF2 */
    RAM8(0xF5) = 0x00;
    RAM8(0xF6) = 0x00;
    sub_F0E1(r);
    if (!r->c)                 /* BCS L_EEFE not taken -> RTS */
        return;

    /* L_EEFE */
    RAM8(0xF1) = (u8)(RAM8(0xF1) + 1);   /* INC $F1 */
    sub_F2DA(r);
}
