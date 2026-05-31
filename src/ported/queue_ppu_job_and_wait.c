/* $CC8F queue_ppu_job_and_wait — submit a VRAM update request and frame-sync.
 *   PHA / wait nmi_vram_req==0 / PLA / STA nmi_vram_req=A (queue) / wait ==0 / RTS
 * The NMI applies the queued job and clears $28. In the flat-memory port the job
 * is applied synchronously, so the net effect is nmi_vram_req($28)=0 with A
 * preserved. (Diff-tested with oracle vram_sync, which models the NMI clearing $28.) */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
void nmi_handler(Regs *r);
#endif
void queue_ppu_job_and_wait(Regs *r)
{
#ifdef LOTW_SHIM
    /* Shim build: actually perform the queued job by running one NMI (which
     * dispatches the vram_* uploader into the software PPU, then clears $28).
     * One queue-and-wait == one frame's NMI, matching hardware. */
    RAM8(0x28) = r->a;   /* STA nmi_vram_req = A (job type) */
    nmi_handler(r);      /* NMI applies it and clears $28 */
#else
    RAM8(0x28) = 0;      /* flat-memory port: job applied synchronously by the oracle */
#endif
}
