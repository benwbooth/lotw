/* $CC17 — redraw the scene each frame and wait until ALL buttons are released.
 * INSPECTION-PORT (no diff-test spec): read_controllers yields $20=0 in flat host
 * memory, so this loop never terminates there. Integration-verified. */
#include "ram.h"
#include "regs.h"
void sub_C1D8(Regs *r); void sub_C2B1(Regs *r); void sub_C234(Regs *r);
void sub_C135(Regs *r); void read_controllers(Regs *r);
void sub_CC17(Regs *r)
{
    do {
        RAM8(0x36) = 0x01;
        sub_C1D8(r); sub_C2B1(r); sub_C234(r); sub_C135(r);
        read_controllers(r);
    } while (RAM8(0x20) != 0);          /* BNE L_CC17 */
}
