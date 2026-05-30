/* $CF82:
 *   LDA #$DE / STA vram_dst_lo ($16) / LDA #$21 / STA vram_dst_hi ($17)
 *   JSR L_D051                 ; A = effective strength
 *   JSR L_CFF9                 ; split A into VRAM src
 *   LDA #$06 / JSR queue_ppu_job_and_wait
 *   LDA #$1E / STA vram_dst_lo / LDA #$22 / STA vram_dst_hi
 *   JSR L_D038                 ; A = effective jump
 *   JSR L_CFF9
 *   LDA #$06 / JSR queue_ppu_job_and_wait
 *   LDA #$5E / STA vram_dst_lo / LDA #$22 / STA vram_dst_hi
 *   JSR L_D067                 ; A = effective shot range
 *   JSR L_CFF9
 *   LDA #$06 / JSR queue_ppu_job_and_wait
 *   RTS
 * Draws strength / jump / shot-range stats to three VRAM destinations.
 */
#include "ram.h"
#include "regs.h"

void sub_CFF9(Regs *r);
void sub_D038(Regs *r);
void sub_D051(Regs *r);
void sub_D067(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_CF82(Regs *r)
{
    RAM8(0x16) = 0xDE;          /* vram_dst_lo */
    RAM8(0x17) = 0x21;          /* vram_dst_hi -> $21DE */
    sub_D051(r);                /* -> r->a = strength */
    sub_CFF9(r);
    r->a = 0x06;
    queue_ppu_job_and_wait(r);

    RAM8(0x16) = 0x1E;
    RAM8(0x17) = 0x22;          /* $221E */
    sub_D038(r);                /* -> r->a = jump */
    sub_CFF9(r);
    r->a = 0x06;
    queue_ppu_job_and_wait(r);

    RAM8(0x16) = 0x5E;
    RAM8(0x17) = 0x22;          /* $225E */
    sub_D067(r);                /* -> r->a = shot range */
    sub_CFF9(r);
    r->a = 0x06;
    queue_ppu_job_and_wait(r);
}
