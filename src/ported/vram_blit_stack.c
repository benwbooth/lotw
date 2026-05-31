/* $D2E5 vram_blit_stack — NMI VRAM job: blit 4*16 = 64 bytes pulled off the
 * stack page (SP repointed to $FF) straight to PPUDATA, restore SP, return
 * through nmi_tail. INSPECTION-PORT: manipulates the stack pointer (not modelled
 * in the Regs ABI); the pulled bytes feed PPUDATA. */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_blit_stack(Regs *r)
{
    /* TSX/TXA/LDX #$FF/TXS ... PLA x64 -> PPUDATA ... TXS : reads the stack page
     * as a 64-byte source. In the flat host build the SP juggling has no RAM
     * effect; the data goes to PPUDATA (a no-op write). */
    nmi_tail(r);
}
