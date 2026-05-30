/* $E800:  CLC / ADC health / BCC L_E80A / LDA #$6D / JMP store
 *         L_E80A: CMP #$6E / BCC store / LDA #$6D / store: STA health / JSR L_CAB6 / RTS
 * Adds A to health ($58), clamping to $6D on overflow or when result >= $6E.
 * Then updates health bar via sub_CAB6. */
#include "ram.h"
#include "regs.h"

void sub_CAB6(Regs *r);

void sub_E800(Regs *r)
{
    u16 sum = (u16)r->a + health;     /* CLC then ADC */
    u8 v;

    if (sum > 0xFF)                   /* BCC L_E80A taken when no carry */
        v = 0x6D;
    else if ((u8)sum >= 0x6E)
        v = 0x6D;
    else
        v = (u8)sum;
    health = v;
    sub_CAB6(r);                      /* reads health from RAM */
}
