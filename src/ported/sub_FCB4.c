/* $FCB4:  LDA #$00 / INY
 * loop: CLC / ADC $00 / DEY / BNE loop
 *       LSR A x4 / STA $00 / RTS
 * Accumulates RAM[$00] (Y+1) times into A (INY then loop until Y wraps to 0),
 * then A >>= 4, stored to $00. Input: Y. Outputs: RAM[$00], A. */
#include "ram.h"
#include "regs.h"

void sub_FCB4(Regs *r)
{
    u8 a = 0x00;
    u8 y = (u8)(r->y + 1);          /* INY */
    do {
        a = (u8)(a + RAM8(0x00));   /* CLC / ADC $00 */
        y = (u8)(y - 1);            /* DEY */
    } while (y != 0);               /* BNE */
    a >>= 4;                        /* LSR A x4 */
    RAM8(0x00) = a;
    r->a = a;
    r->y = 0;
}
