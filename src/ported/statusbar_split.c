/* $D36E statusbar_split — the mid-frame status-bar split (sprite-0-hit timed
 * PPU/CHR reprogram), called from the NMI. Faithful translation; NOT isolated-
 * diff-tested (the sprite-0 poll needs the PPU flag to toggle across two phases,
 * which a static oracle can't model). Verified by inspection; behaviour will be
 * confirmed by whole-ROM integration testing. On a PC renderer the split itself
 * is handled by the display layer, so the busy-wait/delay are no-ops here.
 *
 * RAM effect: ppuctrl_shadow ($23). All PPU/MMC3 register access via REG_W (real
 * on the NES build, ignored on host). */
#include "ram.h"
#include "regs.h"

void sound_tick(Regs *r);

void statusbar_split(Regs *r)
{
    REG_W(0x2001, RAM8(0x24));                          /* PPUMASK = $24 */
    RAM8(0x23) = (u8)((RAM8(0x23) & 0xFE) | RAM8(0x1D));/* ppuctrl_shadow */
    REG_W(0x2000, RAM8(0x23));
    REG_W(0x2005, RAM8(0x1C));
    REG_W(0x2005, RAM8(0x1E));
    if (RAM8(0x29) != 0) {                              /* split active this frame */
        (void)RAM8(0x2002);                            /* PPUSTATUS read: reset addr latch */
        REG_W(0x2000, RAM8(0x23) & 0xFE);
        REG_W(0x2005, 0x00);
        REG_W(0x2005, 0xC4);
        REG_W(0x8000, 0x01); REG_W(0x8001, 0x16);      /* split-region CHR banks R1/R4/R5 */
        REG_W(0x8000, 0x04); REG_W(0x8001, 0x3E);
        REG_W(0x8000, 0x05); REG_W(0x8001, 0x3F);
    }
    sound_tick(r);
    if (RAM8(0x29) == 0)
        return;
    /* wait for sprite-0 hit, then a short delay (display-layer concern: no-op) */
    REG_W(0x8000, 0x01);                                /* restore game CHR banks */
    REG_W(0x2000, RAM8(0x23));
    REG_W(0x2005, RAM8(0x1C));
    REG_W(0x2005, RAM8(0x1E));
    REG_W(0x8001, RAM8(0x2B));                          /* R1 = mmc3_r1_shadow */
    REG_W(0x8000, 0x04); REG_W(0x8001, RAM8(0x2E));     /* R4 = mmc3_r4_shadow */
    REG_W(0x8000, 0x05); REG_W(0x8001, RAM8(0x2F));     /* R5 = mmc3_r5_shadow */
}
