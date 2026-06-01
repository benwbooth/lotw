/* $E642 — pop the 7-byte room checkpoint that $E620 pushed, restoring
 * map_screen_y/x, scroll_x_tile/fine, and player_y/tile/fine. Called from the
 * resume tail ($E5FD) after the character-select, so the player lands back on the
 * exact room/screen they came from. The asm preserves its own return address while
 * pulling the 7 bytes off the stack underneath it.
 *
 * In the flat Regs ABI there is no return address on the $0100 page (JSR/RTS are C
 * calls), so we just PLA the 7 bytes via r->s in the reverse of $E620's push order.
 * This is the counterpart to src/ported/sub_E620.c. */
#include "ram.h"
#include "regs.h"

/* Same zero-page room-state bytes as sub_E620.c. */
#define player_x_fine  0x43
#define player_x_tile  0x44
#define player_y       0x45
#define scroll_x_fine  0x7B
#define scroll_x_tile  0x7C
#define map_screen_x   0x47
#define map_screen_y   0x48

void sub_E642(Regs *r)
{
    /* PLA in reverse of the push order (stack is LIFO): map_screen_y was pushed
     * last by $E620, so it comes off first. */
    RAM8(map_screen_y)  = nes_pull(r);
    RAM8(map_screen_x)  = nes_pull(r);
    RAM8(scroll_x_tile) = nes_pull(r);
    RAM8(scroll_x_fine) = nes_pull(r);
    RAM8(player_y)      = nes_pull(r);
    RAM8(player_x_tile) = nes_pull(r);
    RAM8(player_x_fine) = nes_pull(r);
}
