/* $D1C8 ram_state_init — copy initial RAM image from ROM tables into RAM.
 *   $0000-$00FF <- ROM $9B9F (256 bytes, INX/BNE)
 *   $0100-$013F <- ROM $9C9E (64 bytes, DEX/BPL high-to-low)
 *   $0180-$019F <- #$0F      (32 bytes, DEX/BPL)
 *   save_inventory $0300-$03FF <- ROM $9D3E (256 bytes, INX/BNE)
 *   sprite_tables  $0400-$04FF <- ROM $9DC9 (256 bytes, INX/BNE)
 * ROM source addresses are mapped by the harness; read via RAM8().
 */
#include "ram.h"
#include "regs.h"

void ram_state_init(Regs *r)
{
    u8 x;
    signed char i;

    x = 0;                                              /* LDX #$00 */
    do { RAM8(0x0000 + x) = RAM8(0x9B9F + x); } while (++x != 0);

    for (i = 0x3F; i >= 0; i--)                         /* LDX #$3F / DEX BPL */
        RAM8(0x0100 + (u8)i) = RAM8(0x9C9E + (u8)i);

    for (i = 0x1F; i >= 0; i--)                         /* A=#$0F / X=#$1F */
        RAM8(0x0180 + (u8)i) = 0x0F;

    x = 0;                                              /* LDX #$00 */
    do { RAM8(0x0300 + x) = RAM8(0x9D3E + x); } while (++x != 0); /* save_inventory */

    x = 0;                                              /* LDX #$00 */
    do { RAM8(0x0400 + x) = RAM8(0x9DC9 + x); } while (++x != 0); /* sprite_tables */
}
