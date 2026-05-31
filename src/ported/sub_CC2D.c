/* $CC2D — redraw the scene each frame and wait until ANY button is pressed.
 * INSPECTION-PORT (no diff-test spec): never terminates in flat memory ($20=0).
 * Integration-verified. */
#include "ram.h"
#include "regs.h"
void sub_C1D8(Regs *r); void sub_C2B1(Regs *r); void sub_C234(Regs *r);
void sub_C135(Regs *r); void read_controllers(Regs *r);
void sub_CC2D(Regs *r)
{
    do {
        RAM8(0x36) = 0x01;
        sub_C1D8(r); sub_C2B1(r); sub_C234(r); sub_C135(r);
        read_controllers(r);
    } while (RAM8(0x20) == 0);          /* BEQ L_CC2D */
}
