/* $C2B1:
 *   LDA #$10 / STA $0A          ; loop count = 16
 *   LDX $3F / LDY $3E
 * C2B9:
 *   JSR L_C2DB                  ; render sprite pair (uses X,Y as inputs)
 *   TXA / CLC / ADC #$08 / ORA #$80 / TAX
 *   TYA / CLC / ADC #$30 / TAY
 *   DEC $0A / BNE C2B9
 *   TXA / CLC / ADC #$38 / ORA #$80 / STA $3F
 *   TYA / CLC / ADC #$10 / STA $3E
 *   RTS
 * Renders a column of 16 sprites; advances OAM index ($3F) and table index ($3E).
 */
#include "ram.h"
#include "regs.h"

void sub_C2DB(Regs *r);

void sub_C2B1(Regs *r)
{
    u8 x, y;
    RAM8(0x0A) = 0x10;
    x = RAM8(0x3F);
    y = RAM8(0x3E);

    do {
        r->x = x;
        r->y = y;
        sub_C2DB(r);

        x = (u8)(((u8)(x + 0x08)) | 0x80);
        y = (u8)(y + 0x30);

        RAM8(0x0A) = (u8)(RAM8(0x0A) - 1);
    } while (RAM8(0x0A) != 0);

    RAM8(0x3F) = (u8)(((u8)(x + 0x38)) | 0x80);
    RAM8(0x3E) = (u8)(y + 0x10);
}
