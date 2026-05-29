/* $D08A:
 *   LDY #$10 / LDX #$00
 * L_D08E:
 *   LDA #$00 / STA $0401,X
 *   LDA #$02 / STA $0406,X
 *   TXA / CLC / ADC #$10 / TAX
 *   DEY / BNE L_D08E
 *   LDA #$00 / STA $E9 / RTS
 * 16 slots of stride $10 starting at $0400: clear byte +1, set byte +6 = 2.
 * Then $E9 = 0. Final A=0, X=0 (240+16 wraps), Y=0.
 */
#include "ram.h"
#include "regs.h"

void sub_D08A(Regs *r)
{
    u8 x = 0x00;
    u8 y = 0x10;
    do {
        RAM8((u16)(0x0401 + x)) = 0x00;   /* STA $0401,X (absolute,X) */
        RAM8((u16)(0x0406 + x)) = 0x02;   /* STA $0406,X */
        x = (u8)(x + 0x10);               /* TXA/CLC/ADC #$10/TAX */
    } while (--y != 0);
    RAM8(0xE9) = 0x00;
    r->a = 0x00;
    r->x = x;                             /* = 0 after 16 steps */
    r->y = 0x00;
}
