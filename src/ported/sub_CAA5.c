/* $CAA5:
 *   JSR L_CC97              ; wait for prior VRAM job
 *   LDA #$60 / STA vram_dst_lo ($16)
 *   LDA #$23 / STA vram_dst_hi ($17)
 *   LDA #$04 / JSR queue_ppu_job_and_wait
 *   RTS
 * Sets the VRAM destination to $2360 and submits a job of type $04.
 */
#include "ram.h"
#include "regs.h"

void sub_CC97(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_CAA5(Regs *r)
{
    sub_CC97(r);
    RAM8(0x16) = 0x60;          /* vram_dst_lo */
    RAM8(0x17) = 0x23;          /* vram_dst_hi */
    r->a = 0x04;
    queue_ppu_job_and_wait(r);
}
