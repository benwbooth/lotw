/* $E620 — save a room checkpoint before the character-select / family-house
 * sequence. The real ROM stashes the current level id ($8E) into $FE, then pushes
 * 7 bytes of room state onto the 6502 stack *below* the caller's return address;
 * the matching $E642 pops them back after the player picks a character, restoring
 * the room exactly as it was. Without this, map_screen_x/y, scroll, and the player
 * position are lost across the select (e.g. you resume on the wrong overworld
 * screen).
 *
 * The flat Regs ABI models JSR/RTS as C calls, so there is no return address on the
 * $0100 page to step "below" — we simply push the 7 checkpoint bytes via r->s onto
 * the real stack page. Nothing else maintains r->s between here and $E642, so the
 * checkpoint survives untouched (the NMI handler doesn't push to $0100 either). */
#include "ram.h"
#include "regs.h"

/* Zero-page room-state bytes that make up the checkpoint. */
#define player_x_fine  0x43
#define player_x_tile  0x44
#define player_y       0x45
#define scroll_x_fine  0x7B
#define scroll_x_tile  0x7C
#define map_screen_x   0x47
#define map_screen_y   0x48

void sub_E620(Regs *r)
{
    RAM8(0xFE) = RAM8(0x8E);                 /* LDA $8E / STA $FE — stash level id */

    /* PHA the 7 room-state bytes (same order as the asm). $E642 pulls them in the
     * reverse order, restoring each to its own address. */
    nes_push(r, RAM8(player_x_fine));
    nes_push(r, RAM8(player_x_tile));
    nes_push(r, RAM8(player_y));
    nes_push(r, RAM8(scroll_x_fine));
    nes_push(r, RAM8(scroll_x_tile));
    nes_push(r, RAM8(map_screen_x));
    nes_push(r, RAM8(map_screen_y));
}
