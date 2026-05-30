/* $EA4F:
 *   LDA #$1E / JSR rng_update / TAX / BNE L_EA93       ; 1/$1E chance to proceed
 *   LDX #$03 / LDY #$03 / LDA $0402 / AND #$40 / BEQ + / LDY #$13
 * L_EA64: LDA $040C,Y / STA $F9,X / DEY / DEX / BPL L_EA64
 *   LDA #$00 / STA $F1 / STA $F0 / STA $F4
 *   LDY #$04 / LDA ($E7),Y / STA boss_life($F2)
 *   INY / LDA ($E7),Y / STA $F8
 *   LDA #$01 / STA $EE / LDA #$81 / STA $ED
 *   LDA #$04 / JSR rng_update / STA $EF
 *   LDA #$80 / STA $F1
 * L_EA93: RTS
 *
 * Boss-spawn init: with 1/$1E probability, seeds boss state from $040C table
 * (offset 3 or $13 by $0402 bit6), the ($E7) actor record, and the RNG. */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

void sub_EA4F(Regs *r)
{
    u8 x, y;
    u16 e7;

    r->a = 0x1E;                 /* LDA #$1E */
    rng_update(r);               /* result in r->a */
    if (r->a != 0) {             /* TAX / BNE L_EA93 */
        r->x = r->a;
        return;
    }
    r->x = 0;                    /* X = 0 from TAX */

    x = 0x03;
    y = 0x03;
    if (RAM8(0x0402) & 0x40)     /* AND #$40 / BEQ */
        y = 0x13;

    /* copy $040C+Y -> $F9+X for X=3..0 */
    for (;;) {
        RAM8((u16)(0x00F9 + x)) = RAM8((u16)(0x040C + y));
        y = (u8)(y - 1);
        if ((x--) == 0)          /* DEX / BPL: stop after X goes 0 -> -1 */
            break;
    }

    RAM8(0xF1) = 0x00;
    RAM8(0xF0) = 0x00;
    RAM8(0xF4) = 0x00;

    e7 = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
    RAM8(0xF2) = RAM8((u16)(e7 + 4));    /* boss_life */
    RAM8(0xF8) = RAM8((u16)(e7 + 5));

    RAM8(0xEE) = 0x01;
    RAM8(0xED) = 0x81;

    r->a = 0x04;                 /* LDA #$04 */
    rng_update(r);               /* result in r->a */
    RAM8(0xEF) = r->a;

    RAM8(0xF1) = 0x80;

    r->y = y;                    /* Y after the second rng_update is unchanged by it */
    r->x = x;                    /* X = $FF after the BPL exit */
}
