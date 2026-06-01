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
#ifdef LOTW_SHIM
#include "ppu.h"         /* nes_vblank_wait */
#endif

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
    /* L_C158: LDA $36 / BNE L_C158 — spin until the NMI clears $36. */
#ifdef LOTW_SHIM
    /* Shim: this is the per-frame commit. Wait one real frame — the vblank hook
     * runs an NMI (which decrements $36) inline, or yields a frame and resumes. */
    while (RAM8(0x36) != 0) nes_vblank_wait(r);
#else
    /* Flat-memory port: the wait exits only once $36 reaches 0 (the oracle's
     * sync_clear treats the poll as satisfied), so model that terminal state. */
    RAM8(0x36) = 0;
#endif
}
