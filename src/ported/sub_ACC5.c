/* $ACC5 — compute provisional move position from player + per-axis velocity.
 *   $0E = player_x_fine; if x-velocity $49 != 0, $0E += player_x_fine? no:
 *   $0A = player_y; if y-velocity $4B != 0, $0A += $0A (player_y + $4B)
 *   $0E = player_x_fine; if $49 != 0, $0E += player_x_fine + $49
 * Stores the candidate Y in $0A and X in $0E for the collision check (AE41).
 * Pure RAM math, RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_ACC5(Regs *r)
{
    RAM8(0x0E) = RAM8(0x43);            /* player_x_fine -> $0E */
    RAM8(0x0A) = RAM8(0x45);            /* player_y -> $0A */

    if (RAM8(0x4B) != 0) {              /* LDA $4B / BEQ */
        RAM8(0x0A) = (u8)(RAM8(0x4B) + RAM8(0x0A));   /* CLC/ADC $0A */
    }
    if (RAM8(0x49) != 0) {              /* LDA $49 / BEQ */
        RAM8(0x0E) = (u8)(RAM8(0x49) + RAM8(0x0E));   /* CLC/ADC $0E */
    }
}
