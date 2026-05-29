/* $D408 frame_counters:
 *   DEC $84 / BEQ L_D40D / RTS         ; tick frame counter; act only on wrap
 * L_D40D:
 *   LDX #$07
 * L_D40F: LDA $85,X / BEQ + / DEC $85,X
 *         DEX / BPL L_D40F             ; decrement each nonzero timer $85..$8C
 *   LDA #$3C / STA $84                 ; reload frame counter = 60
 * Output: RAM ($84, $85..$8C).
 */
#include "ram.h"
#include "regs.h"

void frame_counters(Regs *r)
{
    if (--RAM8(0x84) != 0)              /* DEC $84 / BEQ ... else RTS */
        return;
    for (int x = 7; x >= 0; x--) {     /* LDX #$07 ... DEX / BPL */
        if (RAM8((0x85 + x) & 0xFF) != 0)
            --RAM8((0x85 + x) & 0xFF);
    }
    RAM8(0x84) = 0x3C;                 /* reload = 60 frames */
    r->x = 0xFF;                        /* X = $FF after loop falls through (DEX past 0) */
}
