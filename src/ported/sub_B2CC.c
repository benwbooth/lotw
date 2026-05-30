/* $B2CC:  set palette buffer $0180..$0183 = {$0F,$0C,$10,$30}, then fill
 *         $0184..$019F (X=$1B..0) with $0F, then JSR $C569.
 * Initializes the palette RAM buffer and commits it via sub_C569. */
#include "ram.h"
#include "regs.h"

void sub_C569(Regs *r);

void sub_B2CC(Regs *r)
{
    int x;
    RAM8(0x0180) = 0x0F;
    RAM8(0x0181) = 0x0C;
    RAM8(0x0182) = 0x10;
    RAM8(0x0183) = 0x30;
    for (x = 0x1B; x >= 0; --x)
        RAM8((u16)(0x0184 + x)) = 0x0F;
    r->a = 0x0F;
    sub_C569(r);
}
