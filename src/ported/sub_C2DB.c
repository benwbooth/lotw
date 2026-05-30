/* $C2DB: render one enemy/object as a pair of hardware sprites into the OAM
 * shadow at $0200+X, from the object record at sprite_tables ($0400) +Y.
 * Computes screen X by 16-bit subtracting the scroll origin; blanks the pair
 * (tile $EF) when the object is inactive, off the right edge, or off-screen. */
#include "ram.h"
#include "regs.h"

#define sprite_tables 0x0400
#define scroll_x_fine RAM8(0x007B)
#define scroll_x_tile RAM8(0x007C)

void sub_C2DB(Regs *r)
{
    u16 x = r->x;
    u16 y = r->y;
    u8 a, t;
    u8 carry;

    if (RAM8(sprite_tables + 1 + y) == 0) goto blank;          /* $0401,Y */
    if (RAM8(sprite_tables + 0x0E + y) >= 0xBF) goto blank;    /* $040E,Y */

    a = RAM8(sprite_tables + 2 + y);                           /* $0402,Y */
    RAM8(0x0202 + x) = a;
    RAM8(0x0206 + x) = a;
    if (a & 0x40) {
        a = RAM8(sprite_tables + y);                           /* $0400,Y */
        RAM8(0x0205 + x) = a;
        a = (u8)(a + 0x02);                                    /* C=0 here */
        RAM8(0x0201 + x) = a;
    } else {
        a = RAM8(sprite_tables + y);
        RAM8(0x0201 + x) = a;
        a = (u8)(a + 0x02);                                    /* C=0 here */
        RAM8(0x0205 + x) = a;
    }

    /* C30D: 16-bit screen-X = ($040C/$040D,Y) - (scroll_x_fine/tile) */
    {
        u16 d = (u16)RAM8(sprite_tables + 0x0C + y) + 0x100 - scroll_x_fine; /* SEC,SBC */
        a = (u8)d & 0x0F;
        RAM8(0x08) = a;
        carry = (u8)(d >> 8);                                  /* borrow flag */

        d = (u16)RAM8(sprite_tables + 0x0D + y) + carry
            - scroll_x_tile - 1;                               /* SBC w/ borrow-in */
        a = (u8)d;
        if (a >= 0x10) goto blank;                             /* CMP #$10 / BCS */
        a = (u8)((a << 4) | RAM8(0x08));
        RAM8(0x08) = a;
    }

    if (RAM8(sprite_tables + 1 + y) == 0x01) {                 /* $0401,Y == 1 */
        if (RAM8(sprite_tables + 0x0F + y) != 0) {             /* $040F,Y != 0 */
            RAM8(0x08) = (u8)(RAM8(0x08) + RAM8(sprite_tables + 0x0F + y));
            RAM8(sprite_tables + 0x0F + y) = 0x00;
        }
    }

    /* C33E */
    a = RAM8(0x08);
    if (a >= 0xEF) {                                           /* CMP #$EF / BCS C363 */
        RAM8(0x0203 + x) = a;
        t = (u8)(RAM8(sprite_tables + 0x0E + y) + 0x2B);
        RAM8(0x0200 + x) = t;
        RAM8(0x0204 + x) = 0xEF;
        return;
    }
    RAM8(0x0203 + x) = a;
    a = (u8)(a + 0x08);
    RAM8(0x0207 + x) = a;
    t = (u8)(RAM8(sprite_tables + 0x0E + y) + 0x2B);
    RAM8(0x0200 + x) = t;
    RAM8(0x0204 + x) = t;
    return;

blank:
    RAM8(0x0200 + x) = 0xEF;
    RAM8(0x0204 + x) = 0xEF;
}
