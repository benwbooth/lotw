/* $A7E1 (bank13, far-call target) — load the boss-life meter sprites: copy the
 * 64-byte OAM template at $AB3C (bank-13 table) into $0240, then draw the bar
 * (CB53). Pure data copy + CB53; $CB53 is fixed-bank so the call is a direct JSR. */
#include "ram.h"
#include "regs.h"

void sub_CB53(Regs *r);

void sub_A7E1(Regs *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)            /* L_A7E3: copy 64 bytes */
        RAM8((u16)(0x0240 + x)) = RAM8((u16)(0xAB3C + x));
    sub_CB53(r);                            /* JSR $CB53 */
}
