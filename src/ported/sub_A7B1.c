/* $A7B1 — scale ROM per-step delta tables ($FE8B/$FE8C indexed by 2*(A&$0F)) by
 * the count Y, storing the accumulated low/high deltas into $F5 and $F7.
 * Entry: A = table selector, Y = step count.
 *   X = (A & $0F) << 1
 *   $F5 = sum of ROM[$FE8B+X], Y times   (DEY/BNE: 256 iters if Y==0 on entry)
 *   $F7 = sum of ROM[$FE8C+X], Y times
 * ROM tables are mapped by the harness; read via RAM8(). Pure, RTS. */
#include "ram.h"
#include "regs.h"

void sub_A7B1(Regs *r)
{
    u8 y, x, a, c;

    RAM8(0x09) = r->y;                          /* STY $09 */
    x = (u8)((r->a & 0x0F) << 1);               /* AND #$0F / ASL A / TAX */

    a = 0x00;                                   /* LDA #$00 */
    y = r->y;
    do {                                        /* L_A7B9: CLC / ADC $FE8B,X / DEY / BNE */
        a = (u8)(a + RAM8((u16)(0xFE8B + x)));
        y--;
    } while (y != 0);
    RAM8(0xF5) = a;                             /* STA a:$00F5 */

    y = RAM8(0x09);                             /* LDY $09 */
    a = 0x00;                                   /* LDA #$00 */
    do {                                        /* L_A7C7: CLC / ADC $FE8C,X / DEY / BNE */
        a = (u8)(a + RAM8((u16)(0xFE8C + x)));
        y--;
    } while (y != 0);
    RAM8(0xF7) = a;                             /* STA a:$00F7 */
    (void)c;
}
