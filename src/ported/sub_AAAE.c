/* $AAAE — patch the three sprite-table rows ($0410/$0420/$0430) with the low 5
 * bits of $56, set the three X-fine columns ($041C/$042C/$043C) to player_x_fine,
 * and derive three column tiles from player_x_tile (+1, -2, -3). Pure data. */
#include "ram.h"
#include "regs.h"

void sub_AAAE(Regs *r)
{
    (void)r;
    u8 v = (u8)(RAM8(0x56) & 0x1F);        /* LDA $56 / AND #$1F / STA $08 */
    RAM8(0x08) = v;

    RAM8(0x0410) = (u8)((RAM8(0x0410) & 0xE0) | v);
    RAM8(0x0420) = (u8)((RAM8(0x0420) & 0xE0) | v);
    RAM8(0x0430) = (u8)((RAM8(0x0430) & 0xE0) | v);

    u8 xf = RAM8(0x43);                    /* player_x_fine */
    RAM8(0x041C) = xf;
    RAM8(0x042C) = xf;
    RAM8(0x043C) = xf;

    u8 x = RAM8(0x44);                     /* LDX player_x_tile */
    x++;                                   /* INX */
    RAM8(0x042D) = x;                      /* STX $042D */
    x -= 3;                                /* DEX/DEX/DEX */
    RAM8(0x043D) = x;                      /* STX $043D */
    x--;                                   /* DEX */
    RAM8(0x041D) = x;                      /* STX $041D */
}
