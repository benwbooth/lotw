/* $CDB2 sub_CDB2 — scan sprite/object slots 9..0 (X = $90 stepping -$10) for one
 * overlapping the player box at ($0E,$0F,$0A). Returns carry set with $08=slot
 * index (Y) and $09=table offset (X) on the first hit; carry clear (no hit) after
 * all slots checked.
 *
 * sprite_tables = $0400, so $0400+X is the slot's type byte, $0401+X a state byte,
 * $0402+X flags, $040C/$040D/$040E its X/Y/extra coordinates.
 */
#include "ram.h"
#include "regs.h"

void sub_CDB2(Regs *r)
{
    int y = 0x09;
    u8 x = 0x90;
    u8 d;

    for (;;) {
        if ((u8)y == RAM8(0xE3))            /* CPY $E3 / BEQ skip */
            goto skip;
        if (RAM8((u16)(0x0401 + x)) & 0x80) /* LDA $0401,X / BMI skip */
            goto skip;
        if (RAM8((u16)(0x0401 + x)) != 0x01) {       /* CMP #$01 / BEQ proceed */
            if (RAM8((u16)(0x0401 + x)) < 0x1A)      /* CMP #$1A / BCC skip */
                goto skip;
        }
        /* L_CDC7 */
        if ((RAM8((u16)(0x0400 + x)) & 0xF9) == 0xE1)  /* AND #$F9 / CMP #$E1 / BEQ skip */
            goto skip;
        if (RAM8((u16)(0x0402 + x)) & 0x20)          /* AND #$20 / BNE skip */
            goto skip;

        d = (u8)(RAM8(0x0A) - RAM8((u16)(0x040E + x)));   /* LDA $0A / SBC $040E,X */
        if (!(d < 0x10)) {                  /* CMP #$10 / BCC L_CDE5 */
            if (d < 0xF1)                   /* CMP #$F1 / BCC skip */
                goto skip;
        }
        /* L_CDE5 */
        d = (u8)(RAM8(0x0F) - RAM8((u16)(0x040D + x)));   /* LDA $0F / SBC $040D,X */
        if (d == 0)                         /* BEQ L_CE14 (hit) */
            goto hit;
        if (d < 0x02) {                     /* CMP #$02 / BCC L_CE02 */
            d = (u8)(RAM8(0x0E) - RAM8((u16)(0x040C + x)));  /* LDA $0E / SBC $040C,X */
            if (d & 0x80)                   /* BMI L_CE14 (hit) */
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
        goto hit;                           /* JMP L_CE14 */

    skip:
        x = (u8)(x - 0x10);                 /* TXA / SBC #$10 / TAX */
        --y;                                /* DEY */
        if (y < 0)                          /* BPL L_CDB6 */
            break;
    }
    r->c = 0;                               /* CLC / RTS */
    return;

hit:
    RAM8(0x08) = (u8)y;                     /* STY $08 */
    RAM8(0x09) = x;                         /* STX $09 */
    r->c = 1;                               /* SEC / RTS */
}
