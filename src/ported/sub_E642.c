/* $E642 — pop the 7-byte room checkpoint that $E620 saved, restoring
 * map_screen_y/x, scroll_x_tile/fine, and player_y/tile/fine. Called from the
 * character-select resume tail ($E5FD), so the player lands back on the exact
 * room/screen they came from. The original routine pulls the 7 bytes while
 * preserving its own return address.
 *
 * The port keeps the checkpoint in a dedicated LIFO rather than the $0100 page
 * (see src/ported/sub_E620.c for why), so this just pops the most recent entry. */
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

/* The checkpoint LIFO defined in sub_E620.c. */
#define ROOM_CKPT_BYTES 7
extern u8  room_ckpt_stack[][ROOM_CKPT_BYTES];
extern int room_ckpt_sp;

void sub_E642(Regs *r)
{
    (void)r;
    if (room_ckpt_sp > 0) {
        u8 *c = room_ckpt_stack[--room_ckpt_sp];
        RAM8(player_x_fine) = c[0];
        RAM8(player_x_tile) = c[1];
        RAM8(player_y)      = c[2];
        RAM8(scroll_x_fine) = c[3];
        RAM8(scroll_x_tile) = c[4];
        RAM8(map_screen_x)  = c[5];
        RAM8(map_screen_y)  = c[6];
    }
}
