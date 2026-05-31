/* $D252 vram_fill_run — NMI VRAM job: write vram_src_lo to PPUDATA vram_len
 * times (a run-fill), then return through nmi_tail. INSPECTION-PORT (NMI context,
 * PPU-register writes + RTI tail). */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_fill_run(Regs *r)
{
    u8 x = RAM8(0x1A);                 /* LDX vram_len */
    u8 a = RAM8(0x18);                 /* LDA vram_src_lo */
    do { REG_W(0x2007, a); } while (--x != 0);   /* STA PPUDATA / DEX / BNE */
    nmi_tail(r);                       /* JMP nmi_tail */
}
