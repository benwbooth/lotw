/* $C38B: clears a nametable region in the PPU.
 *   LDA ppuctrl_shadow($23) / PHA / AND #$7B / STA PPUCTRL
 *   LDA #$00 / STA $29
 *   LDA $24 / PHA / AND #$E7 / STA PPUMASK
 *   LDA #$20 / STA PPUADDR / LDA #$00 / STA PPUADDR
 *   fill PPUDATA: 5*$C0 bytes of $C0, $40 bytes of $00, again 5*$C0 of $C0,
 *                 $40 bytes of $00
 *   PLA / STA $24 / PLA / STA ppuctrl_shadow / STA PPUCTRL / RTS
 * Net RAM effect: $24 and $23 are pushed then restored (unchanged); only $29=0.
 * All tile data goes to PPUDATA (hardware, ignored by harness). */
#include "ram.h"
#include "regs.h"

void sub_C38B(Regs *r)
{
    u8 ctrl = RAM8(0x23);
    u8 mask = RAM8(0x24);
    int i;

    REG_W(0x2000, ctrl & 0x7B);   /* PPUCTRL */
    RAM8(0x29) = 0x00;
    REG_W(0x2001, mask & 0xE7);   /* PPUMASK */
    REG_W(0x2006, 0x20);          /* PPUADDR hi */
    REG_W(0x2006, 0x00);          /* PPUADDR lo */

    for (i = 0; i < 5 * 0xC0; i++) REG_W(0x2007, 0xC0);
    for (i = 0; i < 0x40; i++)     REG_W(0x2007, 0x00);
    for (i = 0; i < 5 * 0xC0; i++) REG_W(0x2007, 0xC0);
    for (i = 0; i < 0x40; i++)     REG_W(0x2007, 0x00);

    RAM8(0x24) = mask;            /* PLA -> $24 (restored) */
    RAM8(0x23) = ctrl;            /* PLA -> ppuctrl_shadow (restored) */
    REG_W(0x2000, ctrl);          /* STA PPUCTRL */

    r->a = ctrl;
    r->x = 0;
    r->y = 0;
}
