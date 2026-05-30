/* $B648: upload a nametable (1024 bytes from $9EC9..) to PPU, then load two
 * MMC3 bank shadows from ROM.
 *   LDA ppuctrl_shadow($23) / PHA / AND #$7B / STA PPUCTRL
 *   LDA #$00 / STA $29
 *   LDA $24 / PHA / AND #$E7 / STA PPUMASK
 *   LDA #$20 / STA PPUADDR / LDA #$00 / STA PPUADDR
 *   copy 256 bytes each from $9EC9,$9FC9,$A0C9,$A1C9 -> PPUDATA
 *   LDA $A2E9 / STA mmc3_r0_shadow($2A)
 *   LDA $A2EA / STA mmc3_r1_shadow($2B)
 *   PLA / STA $24 / PLA / STA ppuctrl_shadow / STA PPUCTRL / RTS
 * Net RAM effect: $29=0, $2A=ROM[$A2E9], $2B=ROM[$A2EA]; $23/$24 restored.
 * Tile data goes to PPUDATA (hardware, ignored by harness). */
#include "ram.h"
#include "regs.h"

void sub_B648(Regs *r)
{
    u8 ctrl = RAM8(0x23);
    u8 mask = RAM8(0x24);
    int i;

    REG_W(0x2000, ctrl & 0x7B);   /* PPUCTRL */
    RAM8(0x29) = 0x00;
    REG_W(0x2001, mask & 0xE7);   /* PPUMASK */
    REG_W(0x2006, 0x20);          /* PPUADDR hi */
    REG_W(0x2006, 0x00);          /* PPUADDR lo */

    for (i = 0; i < 0x100; i++) REG_W(0x2007, RAM8((u16)(0x9EC9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, RAM8((u16)(0x9FC9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, RAM8((u16)(0xA0C9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, RAM8((u16)(0xA1C9 + i)));

    RAM8(0x2A) = RAM8(0xA2E9);    /* mmc3_r0_shadow */
    RAM8(0x2B) = RAM8(0xA2EA);    /* mmc3_r1_shadow */

    RAM8(0x24) = mask;            /* PLA -> $24 (restored) */
    RAM8(0x23) = ctrl;            /* PLA -> ppuctrl_shadow (restored) */
    REG_W(0x2000, ctrl);          /* STA PPUCTRL */

    r->a = ctrl;
    r->x = 0;
}
