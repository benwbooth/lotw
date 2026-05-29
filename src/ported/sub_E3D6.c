/* $E3D6: derive sprite-OAM bytes from $F9.
 *   LDX #$61 / LDA $F9 / AND #$1F / CMP #$10 / BCC + / SBC #$10 / LDX #$69
 * + STX $0280 / STX $0284 / STA $08
 *   LSR A / LSR A / CLC / ADC $08 / ASL A / ASL A / ASL A
 *   ADC #$36 / STA $0287 / SEC / SBC #$08 / STA $0283 / RTS
 * The carry into "ADC #$36" is the carry-out of the third ASL, so the shifts
 * are tracked in 9 bits.
 */
#include "ram.h"
#include "regs.h"

void sub_E3D6(Regs *r)
{
    u8 a = RAM8(0xF9) & 0x1F;
    u8 x = 0x61;
    u8 base;
    u16 v;
    u8 carry, res;

    if (a >= 0x10) {            /* CMP #$10: carry set when A>=$10, BCC not taken */
        a = (u8)(a - 0x10);     /* SBC #$10 with carry set = exact subtract */
        x = 0x69;
    }
    RAM8(0x0280) = x;
    RAM8(0x0284) = x;
    RAM8(0x08) = a;

    base = (u8)((a >> 2) + a);  /* LSR A / LSR A / CLC / ADC $08 */
    v = (u8)(base << 3);        /* ASL A x3 (8-bit result) */
    carry = (u8)((base >> 5) & 1); /* carry-out of third ASL = bit5 of base */
    res = (u8)((u8)v + 0x36 + carry); /* ADC #$36 (carry-in from last ASL) */
    RAM8(0x0287) = res;
    res = (u8)(res - 0x08);     /* SEC / SBC #$08 */
    RAM8(0x0283) = res;

    r->x = x;
    r->a = res;
}
