/* $D25F vram_upload_palette — NMI VRAM job: upload 32 bytes from $0180 to the
 * palette ($3F00), reset the VRAM address, return through nmi_tail.
 * INSPECTION-PORT (NMI context). */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_upload_palette(Regs *r)
{
    int y;
    REG_W(0x2006, 0x3F); REG_W(0x2006, 0x00);          /* PPUADDR = $3F00 */
    for (y = 0; y < 0x20; y++)
        REG_W(0x2007, RAM8((u16)(0x0180 + y)));        /* $0180,Y -> PPUDATA */
    REG_W(0x2006, 0x3F); REG_W(0x2006, 0x00);
    REG_W(0x2006, 0x00); REG_W(0x2006, 0x00);
    nmi_tail(r);
}
