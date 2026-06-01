/* $E27D — show the password screen. Sets the scroll, redraws (C7B5/C1C7),
 * far-calls the password ENCODE ($B4D4), draws the code (D0E5), waits for the
 * player to release then press a button, then restores the scroll and redraws.
 *
 * INSPECTION-PORT (no diff-test spec): the two read_controllers wait-loops never
 * terminate under flat host memory ($4016 reads 0, so $20 stays 0). Integration-verified. */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
#include "ppu.h"          /* nes_input_poll_yield — keep button polls fast-CPU-safe */
#endif

void sub_C7B5(Regs *r); void sub_C1C7(Regs *r); void sub_B4D4(Regs *r);
void sub_D0E5(Regs *r); void read_controllers(Regs *r);

void sub_E27D(Regs *r)
{
    RAM8(0x7C) = 0x10;                 /* LDA #$10 / STA scroll_x_tile */
    sub_C7B5(r);                       /* JSR L_C7B5 */
    sub_C1C7(r);                       /* JSR L_C1C7 */

    RAM8(0x0E) = 0xD4;                 /* LDA #$D4 / STA $0E */
    RAM8(0x0F) = 0xB4;                 /* LDA #$B4 / STA $0F */
    sub_B4D4(r);                       /* JSR farcall_bank_0C0D -> $B4D4 (encode) */
    sub_D0E5(r);                       /* JSR L_D0E5 */

    /* Wait release then wait a press. The per-iteration nes_input_poll_yield
     * advances a frame in the live-input build so the $4016 latch refreshes
     * (the wait-for-press would otherwise hang on a never-pausing CPU); it is a
     * no-op under the per-read lockstep input. */
#ifdef LOTW_SHIM
    do { read_controllers(r); nes_input_poll_yield(r); } while (RAM8(0x20) != 0);
    do { read_controllers(r); nes_input_poll_yield(r); } while (RAM8(0x20) == 0);
#else
    do { read_controllers(r); } while (RAM8(0x20) != 0);   /* L_E295 */
    do { read_controllers(r); } while (RAM8(0x20) == 0);   /* L_E29A */
#endif

    RAM8(0x7C) = 0x20;                 /* LDA #$20 / STA scroll_x_tile */
    sub_C7B5(r);                       /* JSR L_C7B5 */
    sub_C1C7(r);                       /* JSR L_C1C7 */
}
