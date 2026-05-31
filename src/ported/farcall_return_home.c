/* $CCE4 farcall_return_home — far-call epilogue: restore the MMC3 bank shadows
 * (mmc3_r6/r7 from the saved $32/$33) and jump back to the saved target $0E:$0F.
 * INSPECTION-PORT: far-call/bank infrastructure (ends in JMP ($000E)); the bank
 * register writes are hardware. integration-only for the control transfer. */
#include "ram.h"
#include "regs.h"
void farcall_return_home(Regs *r)
{
    (void)r;
    RAM8(0x31) = RAM8(0x33);            /* mmc3_r7_shadow <- $33 */
    RAM8(0x30) = RAM8(0x32);            /* mmc3_r6_shadow <- $32 */
    /* push $CD07, write MMC3 regs, JMP ($000E) — integration-only */
}
