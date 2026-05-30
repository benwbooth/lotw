/* $CD70 sub_CD70 — scale a 16-bit per-step delta (ROM tables $FE8B/$FE8C indexed
 * by 2*(A&$0F)) by the count Y, splitting the low byte into nibble fields into
 * $F5/$F6/$F7. Mirror of sub_CD2C but with A as the table selector and outputs
 * to $F5..$F7.
 *
 *   $09 = Y
 *   if Y == 0:  $F5 = $F6 = $F7 = 0;  return
 *   X = (A & $0F) << 1
 *   A = sum of ROM[$FE8B+X], Y times
 *     $F5 = A & $0F
 *     $F6 = (A>>4)|($08), where $08 = $F0 if A had bit7 set else $00
 *   A = sum of ROM[$FE8C+X], Y times
 *     $F7 = A
 * ROM tables are mapped by the harness; read via RAM8().
 */
#include "ram.h"
#include "regs.h"

void sub_CD70(Regs *r)
{
    u8 y = r->y;
    u8 x, a, c, sign_fill;

    RAM8(0x09) = y;                     /* STY $09 / LDY $09 */
    if (y == 0) {                       /* BEQ L_CDA9 */
        RAM8(0xF5) = 0;
        RAM8(0xF6) = 0;
        RAM8(0xF7) = 0;
        return;
    }

    x = (u8)((r->a & 0x0F) << 1);       /* AND #$0F / ASL A / TAX */

    a = 0;                              /* LDA #$00 */
    for (c = y; c != 0; c--)            /* DEY / BNE L_CD7C */
        a = (u8)(a + RAM8(0xFE8B + x));
    /* PHA; AND #$0F -> $F5 */
    RAM8(0xF5) = a & 0x0F;
    sign_fill = (a & 0x80) ? 0xF0 : 0x00;   /* PLA / BPL: Y=#$F0 if negative */
    RAM8(0x08) = sign_fill;                  /* STY $08 */
    RAM8(0xF6) = (u8)(((a & 0xF0) >> 4) | sign_fill);

    a = 0;                             /* LDY $09 / LDA #$00 */
    for (c = y; c != 0; c--)           /* DEY / BNE L_CD9F */
        a = (u8)(a + RAM8(0xFE8C + x));
    RAM8(0xF7) = a;
}
