/* $AD7A — build the 2-tile player sprite OAM entries ($0210-$0217). If a death/
 * stun timer $85 is active and $84 is even, hide the sprites (Y=$EF). Otherwise
 * place both tiles below the player (player_y+$2B), at player_x_fine and +8,
 * apply the flip attribute $57|$20, and assign tile $56 / $56+2 left-to-right
 * (or swapped when the H-flip bit $57 bit6 is set). Pure OAM/RAM, RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_AD7A(Regs *r)
{
    if (RAM8(0x85) != 0) {                          /* LDA $85 / BEQ L_AD8D */
        if ((RAM8(0x84) & 0x01) == 0) {             /* AND #$01 / BNE L_AD8D */
            RAM8(0x0210) = 0xEF;                    /* hide both tiles */
            RAM8(0x0214) = 0xEF;
            return;
        }
    }

    /* L_AD8D */
    RAM8(0x0210) = (u8)(RAM8(0x45) + 0x2B);         /* player_y + $2B */
    RAM8(0x0214) = (u8)(RAM8(0x45) + 0x2B);
    RAM8(0x0213) = RAM8(0x43);                      /* player_x_fine */
    RAM8(0x0217) = (u8)(RAM8(0x43) + 0x08);
    RAM8(0x0212) = (u8)(RAM8(0x57) | 0x20);
    RAM8(0x0216) = (u8)(RAM8(0x57) | 0x20);

    if (RAM8(0x57) & 0x40) {                        /* BIT $57 / BVS L_ADBC (V = bit6) */
        r->x = RAM8(0x56);                          /* L_ADBC */
        RAM8(0x0215) = r->x;
        r->x = (u8)(r->x + 2);                      /* INX / INX */
        RAM8(0x0211) = r->x;
    } else {
        r->x = RAM8(0x56);
        RAM8(0x0211) = r->x;
        r->x = (u8)(r->x + 2);
        RAM8(0x0215) = r->x;
    }
}
