/* $E620 — push a 7-byte room checkpoint (player_x_fine/tile, player_y,
 * scroll_x_fine/tile, map_screen_x/y) onto the stack below the caller's return
 * address, after stashing the current level $8E into $FE.
 * INSPECTION-PORT: the checkpoint lives on the 6502 stack until E642 pops it —
 * the flat Regs ABI can't model that. Only the $FE write is plain RAM. */
#include "ram.h"
#include "regs.h"
void sub_E620(Regs *r)
{
    RAM8(0xFE) = RAM8(0x8E);            /* LDA $8E / STA $FE */
    /* PHA player_x_fine,player_x_tile,player_y,scroll_x_fine,scroll_x_tile,
       map_screen_x,map_screen_y (below the return addr); RTS — integration-only */
}
