/* $D0A5: snapshot live inventory into the save area.
 *   LDX #$07: loop  LDA save_inventory($0300),X / STA $0308,X / DEX / BPL
 *   LDX #$0F: loop  LDA inventory_counts($0060),X / STA save_inventory_counts($0310),X / DEX / BPL
 *   LDA gold($5A) / STA save_gold($0321)
 *   LDA keys($5B) / STA save_keys($0320)
 * Note: first loop copies $0300..$0307 -> $0308..$030F. Pure RAM copy. */
#include "ram.h"
#include "regs.h"

void sub_D0A5(Regs *r)
{
    int i;
    for (i = 7; i >= 0; i--)
        RAM8(0x0308 + i) = RAM8(0x0300 + i);   /* save_inventory,X -> $0308,X */
    for (i = 15; i >= 0; i--)
        RAM8(0x0310 + i) = RAM8(0x0060 + i);   /* inventory_counts -> save_inventory_counts */
    RAM8(0x0321) = RAM8(0x5A);                 /* save_gold = gold */
    RAM8(0x0320) = RAM8(0x5B);                 /* save_keys = keys */
    r->x = 0xFF;                               /* X after DEX past 0 / BPL */
}
