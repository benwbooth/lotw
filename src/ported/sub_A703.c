/* $A703 — draw (or hide) one 2-sprite boss-part metasprite into OAM. X base from
 * $0F (OAM byte offset), Y index from $0E (into the sprite_tables block $0400).
 * If the part is inactive ($0401,Y == 0) or its X-screen pos $040E,Y >= $BF
 * (off-screen), hide both sprites (tile $EF). Otherwise:
 *   - attr byte $0402,Y -> both sprite attr slots ($0202,X / $0206,X)
 *   - if attr bit6 (h-flip) clear: left tile = sprite_tables[Y], right = +2
 *     else (flipped): swap which slot gets base / base+2
 *   - X positions from $040C,Y (and +8); Y screen pos from $040E,Y + $2B.
 * Pure OAM/data, RTS. */
#include "ram.h"
#include "regs.h"

void sub_A703(Regs *r)
{
    u8 x = RAM8(0x0F);                    /* LDX $0F */
    u8 y = RAM8(0x0E);                    /* LDY $0E */

    if (RAM8((u16)(0x0401 + y)) == 0)     /* LDA $0401,Y / BEQ L_A754 */
        goto hide;
    if (RAM8((u16)(0x040E + y)) >= 0xBF)  /* LDA $040E,Y / CMP #$BF / BCS L_A754 */
        goto hide;

    {
        u8 attr = RAM8((u16)(0x0402 + y));        /* LDA $0402,Y */
        RAM8((u16)(0x0202 + x)) = attr;           /* STA $0202,X */
        RAM8((u16)(0x0206 + x)) = attr;           /* STA $0206,X */
        if (attr & 0x40) {                        /* AND #$40 / BNE L_A72F */
            u8 t = RAM8((u16)(0x0400 + y));       /* LDA sprite_tables,Y */
            RAM8((u16)(0x0205 + x)) = t;          /* STA $0205,X */
            RAM8((u16)(0x0201 + x)) = (u8)(t + 2); /* CLC / ADC #$02 / STA $0201,X */
        } else {
            u8 t = RAM8((u16)(0x0400 + y));       /* LDA sprite_tables,Y */
            RAM8((u16)(0x0201 + x)) = t;          /* STA $0201,X */
            RAM8((u16)(0x0205 + x)) = (u8)(t + 2); /* CLC / ADC #$02 / STA $0205,X */
        }
    }
    {   /* L_A73B */
        u8 px = RAM8((u16)(0x040C + y));          /* LDA $040C,Y */
        RAM8((u16)(0x0203 + x)) = px;             /* STA $0203,X */
        RAM8((u16)(0x0207 + x)) = (u8)(px + 8);   /* CLC / ADC #$08 / STA $0207,X */
        {
            u8 py = (u8)(RAM8((u16)(0x040E + y)) + 0x2B); /* LDA $040E,Y / CLC / ADC #$2B */
            RAM8((u16)(0x0200 + x)) = py;         /* STA $0200,X */
            RAM8((u16)(0x0204 + x)) = py;         /* STA $0204,X */
        }
    }
    (void)r;
    return;

hide:                                             /* L_A754 */
    RAM8((u16)(0x0200 + x)) = 0xEF;               /* LDA #$EF / STA $0200,X */
    RAM8((u16)(0x0204 + x)) = 0xEF;               /* STA $0204,X */
    (void)r;
}
