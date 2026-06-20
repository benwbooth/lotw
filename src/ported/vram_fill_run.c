/* $D252 vram_fill_run — vblank VRAM job: write vram_src_lo to PPUDATA vram_len
 * times (a run-fill), then return through vblank_commit_tail. INSPECTION-PORT,
 * PPU-register writes + RTI tail). */
#include "ram.h"
#include "regs.h"
void vblank_commit_tail(Regs *r);
void vram_fill_run(Regs *r)
{
    u8 x = RAM8(0x1A);                 /* LDX vram_len */
    u8 a = RAM8(0x18);                 /* LDA vram_src_lo */
    do { REG_W(0x2007, a); } while (--x != 0);   /* STA PPUDATA / DEX / BNE */
    vblank_commit_tail(r);             /* tail commit */
}
