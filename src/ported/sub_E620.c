/* $E620 — save a room checkpoint before the character-select / family-house
 * sequence. The real ROM stashes the current level id ($8E) into $FE, then pushes
 * 7 bytes of room state onto the 6502 stack *below* the caller's return address;
 * the matching $E642 pops them back after the player picks a character, restoring
 * the room exactly as it was (map_screen_x/y, scroll, player position). Without it
 * the player resumes on the wrong overworld screen.
 *
 * Why a dedicated store instead of the real $0100 stack: the port doesn't maintain
 * a call-depth-accurate stack pointer (JSR/RTS are C calls), and other routines
 * write fixed $0100-page addresses for diff-test reasons (e.g. sub_CB0E saves X at
 * $01FB). On real hardware E620 is many calls deep, so its checkpoint sits low and
 * far from those slots; in the flat port it would land at the top of the page and
 * collide. So we keep the checkpoint in a small dedicated LIFO — it is still stack
 * *data* (pushed by E620, popped by E642), just held where nothing can clobber it.
 * See src/ported/sub_E642.c. */
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

/* Dedicated checkpoint LIFO shared with sub_E642.c. A depth of 4 covers any
 * realistic E620/E642 nesting (they are called as balanced pairs). */
#define ROOM_CKPT_BYTES  7
#define ROOM_CKPT_DEPTH  4
u8  room_ckpt_stack[ROOM_CKPT_DEPTH][ROOM_CKPT_BYTES];
int room_ckpt_sp = 0;

void sub_E620(Regs *r)
{
    (void)r;
    RAM8(0xFE) = RAM8(0x8E);                 /* LDA $8E / STA $FE — stash level id */

    if (room_ckpt_sp < ROOM_CKPT_DEPTH) {
        u8 *c = room_ckpt_stack[room_ckpt_sp++];
        c[0] = RAM8(player_x_fine);          /* same 7 bytes the asm PHAs */
        c[1] = RAM8(player_x_tile);
        c[2] = RAM8(player_y);
        c[3] = RAM8(scroll_x_fine);
        c[4] = RAM8(scroll_x_tile);
        c[5] = RAM8(map_screen_x);
        c[6] = RAM8(map_screen_y);
    }
}
