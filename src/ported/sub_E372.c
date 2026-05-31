/* $E372 — password-entry keypad handler. Computes the selected grid cell
 * (a = $F5*5 + $F7). Cell $20 -> advance cursor ($F9++); $21 -> back ($F9--);
 * $22 -> confirm (E347 validate). Any other cell is a character: store it into
 * the password buffer $0322,X (X from E41E); if the buffer just filled (X=$1F)
 * confirm (E347), else advance the cursor. Redraws via E3D6.
 *
 * INSPECTION-PORT (no diff-test spec): tail-calls E347, whose PLA/PLA non-local
 * return the flat Regs ABI can't model. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_E347(Regs *r); void sub_E41E(Regs *r); void sub_E3D6(Regs *r);

void sub_E372(Regs *r)
{
    u8 f5 = RAM8(0xF5);
    u8 a = (u8)((u8)(f5 << 2) + f5);   /* ASL A / ASL A / ADC $F5  -> $F5*5 */
    a = (u8)(a + RAM8(0xF7));           /* ADC $F7 */

    if (a == 0x20) goto L_E392;         /* CMP #$20 / BEQ */
    if (a == 0x21) goto L_E398;         /* CMP #$21 / BEQ */
    if (a == 0x22) {                    /* CMP #$22 / BEQ L_E347 */
        sub_E347(r);
        return;
    }

    /* character cell: store into the password buffer */
    r->a = a;                           /* PHA */
    sub_E41E(r);                        /* JSR L_E41E -> buffer index in X */
    /* PLA: the char is held in `a` */
    RAM8((u16)(0x0322 + r->x)) = a;     /* STA $0322,X */
    if (r->x == 0x1F) {                 /* CPX #$1F / BEQ L_E347 */
        sub_E347(r);
        return;
    }
    /* fall through to L_E392 */

L_E392:
    RAM8(0xF9)++;                       /* INC $F9 */
    sub_E3D6(r);                        /* JSR L_E3D6 */
    return;
L_E398:
    RAM8(0xF9)--;                       /* DEC $F9 */
    sub_E3D6(r);
}
