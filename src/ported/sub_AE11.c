/* $AE11 — "press START to continue" gate. Sets the prompt ($8F=3), bumps the
 * dialog depth ($8D), waits for all buttons released, waits for the $10 (Start)
 * button, waits released again, then $8F=4 / $8D--.
 * INSPECTION-PORT (no diff-test spec): the read_controllers wait-loops never
 * terminate in flat host memory ($20 reads 0). Integration-verified. */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
#include "ppu.h"          /* nes_input_poll_yield — keep button polls fast-CPU-safe */
#endif
void read_controllers(Regs *r);
void sub_AE11(Regs *r)
{
    RAM8(0x8F) = 0x03;
    RAM8(0x8D)++;                                       /* INC $8D */
    /* wait release / wait Start press / wait release. The per-iteration
     * nes_input_poll_yield advances a frame in the live-input build so the pad
     * latch refreshes (else the wait-for-Start hangs on a never-pausing CPU);
     * no-op under per-read lockstep input. */
#ifdef LOTW_SHIM
    do { read_controllers(r); nes_input_poll_yield(r); } while (RAM8(0x20) != 0);
    do { read_controllers(r); nes_input_poll_yield(r); } while (!(RAM8(0x20) & 0x10));
    do { read_controllers(r); nes_input_poll_yield(r); } while (RAM8(0x20) != 0);
#else
    do { read_controllers(r); } while (RAM8(0x20) != 0);        /* L_AE17 wait release */
    do { read_controllers(r); } while (!(RAM8(0x20) & 0x10));   /* L_AE1C wait Start */
    do { read_controllers(r); } while (RAM8(0x20) != 0);        /* L_AE23 wait release */
#endif
    RAM8(0x8F) = 0x04;
    RAM8(0x8D)--;                                       /* DEC $8D */
}
