/* $CB69:
 *   LDA $0405 / CMP #$6D / BCC + / LDA #$6D
 * L_CB72: STA $08 / LDA #$00 / STA $09 / LDX #$65 / LDY #$6B / JMP L_CB94
 * L_CB94:
 *   TXA / LDX $09
 *   STA $0259,X / STA $025D,X / STA $0261,X / STA $0265,X / STA $0269,X
 *   TYA
 *   STA $026D,X / STA $0271,X / STA $0275,X / STA $0279,X / STA $027D,X
 *   JSR L_CBFA              ; $08/10 -> Y=count, $08=remainder
 *   LDA $09 / CLC / ADC #$18 / TAX
 * L_CBBF: DEY/BEQ / DEC $0241,X / DEC $0241,X / DEY/BEQ / DEC $0241,X / DEC $0241,X / INX*4 / JMP
 * L_CBD8: LDA $09 / CLC / ADC #$2C / TAX / LDY $08
 * L_CBE0: DEY/BEQ / DEC $0241,X / DEC $0241,X / DEY/BEQ / DEC $0241,X / DEC $0241,X / INX*4 / JMP
 * L_CBF9: RTS
 *
 * Renders a clamped value from $0405 (max $6D) as a meter: stores tile constants
 * $65/$6B into the OAM shadow ($0259.., $026D..) and adjusts attribute/position
 * bytes at $0241+ per the tens($08/10 count) and ones(remainder) digits. */
#include "ram.h"
#include "regs.h"

void sub_CBFA(Regs *r);

void sub_CB69(Regs *r)
{
    u8 a, x, y;

    a = RAM8(0x0405);
    if (a >= 0x6D) a = 0x6D;     /* CMP/BCC clamp */
    RAM8(0x08) = a;
    RAM8(0x09) = 0x00;

    x = 0x65;                    /* LDX #$65 (loaded into A via TXA) */
    y = 0x6B;                    /* LDY #$6B */

    /* L_CB94: TXA / LDX $09 ; store A=$65 at $0259+X stride 4, then A=$6B at $026D+X */
    a = x;
    x = RAM8(0x09);
    RAM8((u16)(0x0259 + x)) = a;
    RAM8((u16)(0x025D + x)) = a;
    RAM8((u16)(0x0261 + x)) = a;
    RAM8((u16)(0x0265 + x)) = a;
    RAM8((u16)(0x0269 + x)) = a;
    a = y;
    RAM8((u16)(0x026D + x)) = a;
    RAM8((u16)(0x0271 + x)) = a;
    RAM8((u16)(0x0275 + x)) = a;
    RAM8((u16)(0x0279 + x)) = a;
    RAM8((u16)(0x027D + x)) = a;

    sub_CBFA(r);                 /* uses $08; sets r->y=count, $08=remainder */
    y = r->y;

    /* L_CBBF: X = $09 + $18 */
    x = (u8)(RAM8(0x09) + 0x18);
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0241 + x))--;
        RAM8((u16)(0x0241 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0241 + x))--;
        RAM8((u16)(0x0241 + x))--;
        x = (u8)(x + 4);
    }

    /* L_CBD8: X = $09 + $2C, Y = $08 (remainder) */
    x = (u8)(RAM8(0x09) + 0x2C);
    y = RAM8(0x08);
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0241 + x))--;
        RAM8((u16)(0x0241 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0241 + x))--;
        RAM8((u16)(0x0241 + x))--;
        x = (u8)(x + 4);
    }

    r->a = a;
    r->x = x;
    r->y = y;
}
