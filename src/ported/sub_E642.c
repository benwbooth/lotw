/* $E642 — pop the 7-byte room checkpoint pushed by E620 back into map_screen_y/x,
 * scroll_x_tile/fine, player_y/tile/fine, preserving the return address.
 * INSPECTION-PORT: reads the values off the 6502 stack (not modelled in the Regs
 * ABI); integration-verified. */
#include "ram.h"
#include "regs.h"
void sub_E642(Regs *r)
{
    (void)r;
    /* PLA->map_screen_y / map_screen_x / scroll_x_tile / scroll_x_fine /
       player_y / player_x_tile / player_x_fine ; RTS — integration-only */
}
