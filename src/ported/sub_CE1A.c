/* $CE1A sub_CE1A — scan sprite/object slots 10..0 (X = $A0 stepping -$10) for one
 * overlapping the player box at ($0E,$0F,$0A). Returns carry set with $08=slot
 * index (Y) and $09=table offset (X) on the first hit; carry clear (no hit) after
 * all slots checked. Same overlap test as sub_CDB2 but a wider slot range and a
 * simpler "active" check ($0401,X nonzero and non-negative).
 */
#include "ram.h"
#include "regs.h"

void sub_CE1A(Regs *r)
{
    int y = 0x0A;
    u8 x = 0xA0;
    u8 d;

    for (;;) {
        if ((u8)y == RAM8(0xE3))            /* CPY $E3 / BEQ skip */
            goto skip;
        if (RAM8((u16)(0x0401 + x)) == 0)   /* LDA $0401,X / BEQ skip */
            goto skip;
        if (RAM8((u16)(0x0401 + x)) & 0x80) /* BMI skip */
            goto skip;
        if ((RAM8((u16)(0x0400 + x)) & 0xF9) == 0xE1)  /* AND #$F9 / CMP #$E1 / BEQ skip */
            goto skip;
        if (RAM8((u16)(0x0402 + x)) & 0x20)          /* AND #$20 / BNE skip */
            goto skip;

        d = (u8)(RAM8(0x0A) - RAM8((u16)(0x040E + x)));   /* LDA $0A / SBC $040E,X */
        if (!(d < 0x10)) {                  /* CMP #$10 / BCC */
            if (d < 0xF1)                   /* CMP #$F1 / BCC skip */
                goto skip;
        }
        d = (u8)(RAM8(0x0F) - RAM8((u16)(0x040D + x)));   /* LDA $0F / SBC $040D,X */
        if (d == 0)                         /* BEQ L_CE76 (hit) */
            goto hit;
        if (d < 0x02) {                     /* CMP #$02 / BCC L_CE64 */
            d = (u8)(RAM8(0x0E) - RAM8((u16)(0x040C + x)));  /* LDA $0E / SBC $040C,X */
            if (d & 0x80)                   /* BMI hit */
                goto hit;
            goto skip;
        }
        if (d < 0xFF)                       /* CMP #$FF / BCC skip */
            goto skip;
        d = (u8)(RAM8(0x0E) - RAM8((u16)(0x040C + x)));  /* LDA $0E / SBC $040C,X */
        if (d == 0)                         /* BEQ skip */
            goto skip;
        if (d & 0x80)                       /* BMI skip */
            goto skip;
        goto hit;                           /* JMP L_CE76 */

    skip:
        x = (u8)(x - 0x10);                 /* TXA / SBC #$10 / TAX */
        --y;                                /* DEY */
        if (y < 0)                          /* BPL L_CE1E */
            break;
    }
    r->c = 0;                               /* CLC / RTS */
    return;

hit:
    RAM8(0x08) = (u8)y;                     /* STY $08 */
    RAM8(0x09) = x;                         /* STX $09 */
    r->c = 1;                               /* SEC / RTS */
}
