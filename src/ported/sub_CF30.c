/* $CF30:  LDX #$0F / loop: TXA/PHA / LDY inventory_counts,X / JSR L_CF3F /
 *         PLA/TAX / DEX / BPL loop / RTS
 * For each X = $0F..0, call sub_CF3F with Y = inventory_counts[X] ($0060+X), X = X. */
#include "ram.h"
#include "regs.h"

void sub_CF3F(Regs *r);

void sub_CF30(Regs *r)
{
    int x;
    for (x = 0x0F; x >= 0; --x) {
        r->x = (u8)x;
        r->y = RAM8((u16)(0x0060 + x));   /* inventory_counts,X */
        sub_CF3F(r);
        r->x = (u8)x;                      /* restored from stack (PLA/TAX) */
    }
    r->x = 0xFF;                           /* X after DEX from 0 / BPL falls through */
}
