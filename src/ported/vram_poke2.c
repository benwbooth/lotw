/* $D344 vram_poke2 — NMI VRAM job: write vram_src_hi then vram_src_lo to PPUDATA,
 * then return through nmi_tail. INSPECTION-PORT (NMI context). */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_poke2(Regs *r)
{
    REG_W(0x2007, RAM8(0x19));         /* vram_src_hi -> PPUDATA */
    REG_W(0x2007, RAM8(0x18));         /* vram_src_lo -> PPUDATA */
    nmi_tail(r);
}
