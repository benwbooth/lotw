/* $CC8F queue_ppu_job_and_wait — submit a VRAM update request and frame-sync.
 *   PHA / wait nmi_vram_req==0 / PLA / STA nmi_vram_req=A (queue) / wait ==0 / RTS
 * The NMI applies the queued job and clears $28. In the flat-memory port the job
 * is applied synchronously, so the net effect is nmi_vram_req($28)=0 with A
 * preserved. (Diff-tested with oracle vram_sync, which models the NMI clearing $28.) */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
#include "ppu.h"         /* nes_frame_wait */
#endif
void queue_ppu_job_and_wait(Regs *r)
{
#ifdef LOTW_SHIM
    /* Shim build, faithful to the asm: PHA / wait $28==0 / PLA / STA $28=A / wait
     * $28==0 / RTS. The LEADING wait (for a prior job to clear) is usually 0 frames
     * but burns one when a job is still pending — needed for frame-exact transition
     * timing. Each wait yields a frame; the vblank hook runs the NMI that applies
     * the job and clears $28. */
    while (RAM8(0x28) != 0) nes_frame_wait(r);    /* wait prior job clear */
    RAM8(0x28) = r->a;                            /* STA nmi_vram_req = A (job type) */
    while (RAM8(0x28) != 0) nes_frame_wait(r);    /* wait this job clear */
#else
    RAM8(0x28) = 0;      /* flat-memory port: job applied synchronously by the oracle */
#endif
}
