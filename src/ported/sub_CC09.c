/* $CC09 — read one debounced button press: wait for release (CC17), wait for a
 * press (CC2D, buttons in A/$20), save it, wait for release (CC17), then restore
 * the pressed mask into $20.  INSPECTION-PORT: built on the CC17/CC2D input
 * wait-loops (never terminate in flat memory). Integration-verified. */
#include "ram.h"
#include "regs.h"
void sub_CC17(Regs *r); void sub_CC2D(Regs *r);
void sub_CC09(Regs *r)
{
    sub_CC17(r);                 /* wait release */
    sub_CC2D(r);                 /* wait press (A = buttons) */
    {
        u8 btn = r->a;           /* PHA */
        sub_CC17(r);             /* wait release */
        r->a = btn;              /* PLA */
        RAM8(0x20) = btn;        /* STA $20 */
    }
}
