/* $C569:
 *   JSR L_CC97              ; wait for prior VRAM job
 *   LDA #$00 / STA vram_dst_lo ($16)
 *   LDA #$3F / STA vram_dst_hi ($17)
 *   LDA #$02 / JSR queue_ppu_job_and_wait
 *   RTS
 * Sets the VRAM destination to $3F00 (palette) and submits a job of type $02.
 */
#include "ram.h"
#include "regs.h"

void sub_CC97(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_C569(Regs *r)
{
    sub_CC97(r);
    RAM8(0x16) = 0x00;          /* vram_dst_lo */
    RAM8(0x17) = 0x3F;          /* vram_dst_hi */
    r->a = 0x02;
    queue_ppu_job_and_wait(r);
}
