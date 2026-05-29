/* $D0C5: restore live inventory from the save area (inverse of $D0A5).
 *   LDX #$07: loop  LDA $0308,X / STA save_inventory($0300),X / DEX / BPL
 *   LDX #$0F: loop  LDA save_inventory_counts($0310),X / STA inventory_counts($0060),X / DEX / BPL
 *   LDA save_gold($0321) / STA gold($5A)
 *   LDA save_keys($0320) / STA keys($5B)
 * Pure RAM copy. */
#include "ram.h"
#include "regs.h"

void sub_D0C5(Regs *r)
{
    int i;
    for (i = 7; i >= 0; i--)
        RAM8(0x0300 + i) = RAM8(0x0308 + i);   /* $0308,X -> save_inventory,X */
    for (i = 15; i >= 0; i--)
        RAM8(0x0060 + i) = RAM8(0x0310 + i);   /* save_inventory_counts -> inventory_counts */
    RAM8(0x5A) = RAM8(0x0321);                 /* gold = save_gold */
    RAM8(0x5B) = RAM8(0x0320);                 /* keys = save_keys */
    r->x = 0xFF;                               /* X after DEX past 0 / BPL */
}
