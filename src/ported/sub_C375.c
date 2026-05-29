/* $C375:
 *   LDX #$03 / .loop1 LDA $FF6B,X / STA $0200,X / DEX / BPL .loop1
 *   LDX #$04 / .loop2 LDA #$F8 / STA $0200,X / INX / BNE .loop2 / RTS
 * Copies 4 bytes of ROM ($FF6B..$FF6E) into OAM shadow $0200..$0203, then fills
 * $0204..$02FF with $F8 (X runs 4..255, INX wraps 255->0 to end). X exits $00. */
#include "ram.h"
#include "regs.h"

void sub_C375(Regs *r)
{
    int x;
    for (x = 3; x >= 0; x--)
        RAM8((u16)(0x0200 + x)) = RAM8((u16)(0xFF6B + x));
    for (x = 4; x <= 0xFF; x++)
        RAM8((u16)(0x0200 + x)) = 0xF8;
    r->x = 0x00;  /* INX wrapped 255->0, terminating the loop */
}
