/* $CD2C sub_CD2C — scale a 16-bit per-step delta (ROM tables $FE8B/$FE8C indexed
 * by 2*($20&$0F)) by the count Y, splitting the low byte into nibble fields.
 *
 *   $09 = Y
 *   if Y == 0:  $49 = $4A = $4B = 0;  return
 *   X = ($20 & $0F) << 1
 *   A = sum of ROM[$FE8B+X], Y times          (low/fractional accumulation)
 *     $49 = A & $0F
 *     hi-nibble field $4A: (A>>4)|($08), where $08 = $F0 if A had bit7 set else $00
 *   A = sum of ROM[$FE8C+X], Y times          (high byte accumulation)
 *     $4B = A
 * ROM tables are mapped by the harness; read via RAM8().
 */
#include "ram.h"
#include "regs.h"

void sub_CD2C(Regs *r)
{
    u8 y = r->y;
    u8 x, a, c, sign_fill;

    RAM8(0x09) = y;                     /* STY $09 */
    if (y == 0) {                       /* LDY $09 / BEQ L_CD67 */
        RAM8(0x49) = 0;
        RAM8(0x4A) = 0;
        RAM8(0x4B) = 0;
        return;
    }

    x = (u8)((RAM8(0x20) & 0x0F) << 1); /* AND #$0F / ASL A / TAX */

    a = 0;                              /* LDA #$00 */
    for (c = y; c != 0; c--)            /* DEY / BNE L_CD3A */
        a = (u8)(a + RAM8(0xFE8B + x));
    /* PHA; AND #$0F -> $49 */
    RAM8(0x49) = a & 0x0F;
    sign_fill = (a & 0x80) ? 0xF0 : 0x00;   /* PLA / BPL: Y=#$F0 if negative */
    RAM8(0x08) = sign_fill;                  /* STY $08 */
    RAM8(0x4A) = (u8)(((a & 0xF0) >> 4) | sign_fill);

    a = 0;                             /* LDY $09 / LDA #$00 */
    for (c = y; c != 0; c--)           /* DEY / BNE L_CD5D */
        a = (u8)(a + RAM8(0xFE8C + x));
    RAM8(0x4B) = a;
}
