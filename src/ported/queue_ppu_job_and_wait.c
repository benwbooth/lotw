/* $CC8F queue_ppu_job_and_wait — submit a VRAM update request and frame-sync.
 *   PHA / wait nmi_vram_req==0 / PLA / STA nmi_vram_req=A (queue) / wait ==0 / RTS
 * The NMI applies the queued job and clears $28. In the flat-memory port the job
 * is applied synchronously, so the net effect is nmi_vram_req($28)=0 with A
 * preserved. (Diff-tested with oracle vram_sync, which models the NMI clearing $28.) */
#include "ram.h"
#include "regs.h"
void queue_ppu_job_and_wait(Regs *r)
{
    RAM8(0x28) = 0;   /* job submitted (r->a) and processed by the NMI */
}
