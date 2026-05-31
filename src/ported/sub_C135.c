/* $C135: dispatches one deferred screen-refresh job, then frame-syncs.
 *   if $3D != 0:  $3D=0; sub_C7FE()      (column refresh)
 *   elif $3C != 0: $3C=0; sub_CAA5()     (status-bar refresh)
 *   elif $36 != 0: sub_C569()            (palette refresh)
 *   L_C158: LDA $36 / BNE L_C158 / RTS   (spin-wait until NMI clears $36)
 * The trailing loop is an NMI spin-wait; the C port runs the body once and
 * leaves $36 alone (oracle's sync_clear treats the poll as satisfied).
 */
#include "ram.h"
#include "regs.h"

void sub_C7FE(Regs *r);
void sub_CAA5(Regs *r);
void sub_C569(Regs *r);

void sub_C135(Regs *r)
{
    if (RAM8(0x3D) != 0) {
        RAM8(0x3D) = 0;
        sub_C7FE(r);
    } else if (RAM8(0x3C) != 0) {
        RAM8(0x3C) = 0;
        sub_CAA5(r);
    } else if (RAM8(0x36) != 0) {
        sub_C569(r);
    }
    /* L_C158: LDA $36 / BNE L_C158 — spin until the NMI clears $36. The wait
     * exits only once $36 reaches 0, so model that terminal state directly. */
    RAM8(0x36) = 0;
}
