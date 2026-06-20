/* $D344 vram_poke2 — vblank VRAM job: write vram_src_hi then vram_src_lo to PPUDATA,
 * then return through vblank_commit_tail. INSPECTION-PORT. */
#include "ram.h"
#include "regs.h"
void vblank_commit_tail(Regs *r);
void vram_poke2(Regs *r)
{
    REG_W(0x2007, RAM8(0x19));         /* vram_src_hi -> PPUDATA */
    REG_W(0x2007, RAM8(0x18));         /* vram_src_lo -> PPUDATA */
    vblank_commit_tail(r);
}
