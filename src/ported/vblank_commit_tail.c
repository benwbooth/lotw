/* $D351 frame/vblank commit tail: commit MMC3 bank shadows, do the sprite-0
 * status-bar split, decrement the PPU-job counter $36, advance frame counters,
 * and restore the bank-select register. */
#include "ram.h"
#include "regs.h"
void ppu_commit_banks(Regs *r); void statusbar_split(Regs *r); void frame_counters(Regs *r);
void vblank_commit_tail(Regs *r)
{
    ppu_commit_banks(r);                 /* JSR ppu_commit_banks */
    /* LDA PPUSTATUS — clears the vblank latch on hardware; no RAM effect */
    statusbar_split(r);                  /* JSR statusbar_split */
    if (RAM8(0x36) != 0)                 /* LDA $36 / BEQ / DEC $36 */
        RAM8(0x36)--;
    frame_counters(r);                   /* JSR frame_counters */
    REG_W(0x8000, RAM8(0x25));           /* mmc3_select_shadow -> MMC3_BANK_SELECT */
}
