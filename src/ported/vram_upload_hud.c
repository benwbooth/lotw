/* $D290 vram_upload_hud — NMI VRAM job: push two 24-byte HUD rows ($0140,$0158)
 * to PPUDATA at vram_dst, then a 11-cell read-modify-write attribute blend from
 * $0170/$0171, then return through nmi_tail. INSPECTION-PORT (NMI context). */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_upload_hud(Regs *r)
{
    int x;
    REG_W(0x2000, (u8)(RAM8(0x23) | 0x04));            /* ppuctrl_shadow|$04 -> PPUCTRL */
    for (x = 0x17; x >= 0; x--)
        REG_W(0x2007, RAM8((u16)(0x0140 + x)));        /* $0140,X */
    REG_W(0x2006, RAM8(0x17));                         /* vram_dst_hi */
    REG_W(0x2006, (u8)(RAM8(0x16) + 1));               /* vram_dst_lo+1 */
    for (x = 0x17; x >= 0; x--)
        REG_W(0x2007, RAM8((u16)(0x0158 + x)));        /* $0158,X */
    for (x = 0x0A; x >= 0; x -= 2) {                   /* attribute blend */
        REG_W(0x2006, RAM8(0x19)); REG_W(0x2006, RAM8((u16)(0x0170 + x)));
        (void)REG_R(0x2007);                            /* buffered dummy read */
        {
            u8 v = (u8)((REG_R(0x2007) & RAM8(0x18)) | RAM8((u16)(0x0171 + x)));
            REG_W(0x2006, RAM8(0x19)); REG_W(0x2006, RAM8((u16)(0x0170 + x)));
            REG_W(0x2007, v);
        }
    }
    nmi_tail(r);
}
