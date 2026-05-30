/* $B278 (bank13) — queue an OAM/sprite-clear group.
 *   PHA / A = $0A + 6; if A >= $F0 then A += $10; STA $1E / PLA
 *   JSR queue (A=entry A) ; then LDA #$FF / JSR queue  x3
 * Computes a clamped Y coordinate into $1E, then submits 4 PPU jobs.
 * Net RAM: $1E = adjusted coord, $28 = 0 (queue). Entry A is an input. */
#include "ram.h"
#include "regs.h"

void queue_ppu_job_and_wait(Regs *r);

void sub_B278(Regs *r)
{
    u8 saved_a = r->a;          /* PHA */
    u8 v = (u8)(RAM8(0x0A) + 0x06);  /* LDA $0A / CLC / ADC #$06 */
    if (v >= 0xF0)              /* CMP #$F0 / BCC */
        v = (u8)(v + 0x10);     /* CLC / ADC #$10 */
    RAM8(0x1E) = v;             /* STA $1E */

    r->a = saved_a;             /* PLA */
    queue_ppu_job_and_wait(r);  /* JSR $CC8F */
    r->a = 0xFF;
    queue_ppu_job_and_wait(r);
    r->a = 0xFF;
    queue_ppu_job_and_wait(r);
    r->a = 0xFF;
    queue_ppu_job_and_wait(r);
}
