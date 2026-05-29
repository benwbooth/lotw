/* $C1D8 sub_C1D8 — build the 2-sprite OAM entries ($0210-$0217) for an object,
 * positioning it relative to the player/scroll. Two sprites at $0210.. and $0214..
 *
 *   if $85 != 0 and ($84 & 1) == 0:  Y=$EF (offscreen) for both sprites; return
 *   else:
 *     Y = player_y($45) + $2B           -> $0210, $0214
 *     world_x = (scroll_x_tile($7C)<<4) | scroll_x_fine($7B)      (camera, $08)
 *     X = ((player_x_tile($44)<<4) | player_x_fine($43)) - world_x -> $0213 (sprite0 X)
 *     $0217 = $0213 + 8                                            (sprite1 X)
 *     attr = $57                         -> $0212, $0216
 *     tile = $56; if (attr bit6 set) sprites swapped:
 *        V clear: $0211 = tile,   $0215 = tile+2
 *        V set:   $0215 = tile,   $0211 = tile+2
 */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r)
{
    u8 a, x, world_x;

    if (RAM8(0x85) != 0 && (RAM8(0x84) & 0x01) == 0) {  /* BEQ/BNE -> fall to here */
        RAM8(0x0210) = 0xEF;
        RAM8(0x0214) = 0xEF;
        return;
    }

    /* Y position (both sprites) */
    a = (u8)(RAM8(0x45) + 0x2B);          /* player_y + #$2B (CLC) */
    RAM8(0x0210) = a;
    RAM8(0x0214) = a;

    /* camera world X into $08 */
    world_x = (u8)((RAM8(0x7C) << 4) | RAM8(0x7B));   /* scroll_x_tile<<4 | scroll_x_fine */
    RAM8(0x08) = world_x;

    /* sprite0 X = player world X - camera */
    a = (u8)((RAM8(0x44) << 4) | RAM8(0x43));         /* player_x_tile<<4 | player_x_fine */
    a = (u8)(a - world_x);                            /* SEC SBC $08 */
    RAM8(0x0213) = a;
    RAM8(0x0217) = (u8)(a + 0x08);                    /* CLC ADC #$08 */

    /* attributes (both sprites) */
    RAM8(0x0212) = RAM8(0x57);
    RAM8(0x0216) = RAM8(0x57);

    /* tiles, ordered by attr bit6 (BIT $57 / BVS) */
    x = RAM8(0x56);
    if (RAM8(0x57) & 0x40) {              /* V = bit6 of $57; BVS L_C22B */
        RAM8(0x0215) = x;
        RAM8(0x0211) = (u8)(x + 2);       /* INX INX */
    } else {
        RAM8(0x0211) = x;
        RAM8(0x0215) = (u8)(x + 2);       /* INX INX */
    }
}
