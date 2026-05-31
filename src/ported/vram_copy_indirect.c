/* $D334 vram_copy_indirect — NMI VRAM job: copy vram_len bytes from (vram_src_lo)
 * to PPUDATA, then return through nmi_tail. INSPECTION-PORT (NMI context). */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_copy_indirect(Regs *r)
{
    u8 x = RAM8(0x1A);                 /* LDX vram_len */
    u16 src = (u16)(RAM8(0x18) | (RAM8(0x19) << 8));   /* (vram_src_lo) */
    u8 y = 0;
    do { REG_W(0x2007, RAM8((u16)(src + y))); y++; } while (--x != 0);
    nmi_tail(r);
}
