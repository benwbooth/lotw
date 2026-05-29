/* $E400:
 *   LDA $F5 / ASL A / ASL A / ASL A / ADC #$36 / STA $0297
 *   SEC / SBC #$08 / STA $0293
 *   LDA $F7 / ASL A / ASL A / ASL A / ADC #$81 / STA $0290 / STA $0294
 *   RTS
 * Computes sprite X/Y coords from tile coords $F5/$F7. The three ASLs leave the
 * carry = bit5 of the original byte, which the following ADC folds in. */
#include "ram.h"
#include "regs.h"

/* shift left 3 times; return result and final carry (= bit5 of input) */
static u8 asl3(u8 v, u8 *carry_out)
{
    u8 c = 0;
    int i;
    for (i = 0; i < 3; i++) { c = (v >> 7) & 1; v = (u8)(v << 1); }
    *carry_out = c;
    return v;
}

void sub_E400(Regs *r)
{
    u8 c, a, t;

    t = asl3(RAM8(0xF5), &c);
    a = (u8)(t + 0x36 + c);          /* ADC #$36 with carry from ASL */
    RAM8(0x0297) = a;
    a = (u8)(a - 0x08);              /* SEC / SBC #$08 (no borrow) */
    RAM8(0x0293) = a;

    t = asl3(RAM8(0xF7), &c);
    a = (u8)(t + 0x81 + c);          /* ADC #$81 with carry from ASL */
    RAM8(0x0290) = a;
    RAM8(0x0294) = a;

    r->a = a;
}
