/* $A683 — compute a player-relative point ($0E,$0A) from player position plus
 * optional facing offsets (scaled x4). $0E = player_x_fine ($43), $0A = player_y
 * ($45). If $F7 != 0: $0A += $F7<<2. If $F5 != 0: $0E += $F5<<2. Pure, RTS. */
#include "ram.h"
#include "regs.h"

void sub_A683(Regs *r)
{
    RAM8(0x0E) = RAM8(0x43);                 /* LDA player_x_fine / STA $0E */
    RAM8(0x0A) = RAM8(0x45);                 /* LDA player_y / STA $0A */
    if (RAM8(0xF7) != 0) {                    /* LDA $F7 / BEQ L_A696 */
        u8 a = (u8)(RAM8(0xF7) << 2);        /* ASL A / ASL A */
        RAM8(0x0A) = (u8)(a + RAM8(0x0A));   /* CLC / ADC $0A / STA $0A */
    }
    if (RAM8(0xF5) != 0) {                    /* LDA $F5 / BEQ L_A6A1 */
        u8 a = (u8)(RAM8(0xF5) << 2);        /* ASL A / ASL A */
        RAM8(0x0E) = (u8)(a + RAM8(0x0E));   /* CLC / ADC $0E / STA $0E */
    }
    (void)r;
}
