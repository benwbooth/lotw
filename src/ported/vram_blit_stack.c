/* $D2E5 vram_blit_stack — NMI VRAM job: blit 4*16 = 64 bytes pulled off the
 * stack page (SP repointed to $FF) straight to PPUDATA, restore SP, return
 * through nmi_tail. INSPECTION-PORT: manipulates the stack pointer (not modelled
 * in the Regs ABI); the pulled bytes feed PPUDATA. */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_blit_stack(Regs *r)
{
    /* TSX/TXA/LDX #$FF/TXS ... PLA x64 -> PPUDATA ... TXS.
     * With SP forced to $FF, the PLAs stream $0100..$013F. The C port does not
     * model the 6502 call stack, but the game-visible stack page bytes are in
     * NES_MEM and are the actual VRAM source for this job. */
    for (int i = 0; i < 0x40; i++)
        REG_W(0x2007, RAM8((u16)(0x0100 + i)));
    nmi_tail(r);
}
