/* $C4E0:  $09=$40; loop { $36=5; copy 4B ($77)+$E0->$0180; copy 4B ($77)+$F0->$0190;
 *         C520(X=0,Y=4); C520(X=$10,Y=4); C135; $09-=$10; BPL }; C569
 * Two-palette variant of C4B4: refreshes both $0180..$0183 and $0190..$0193 from
 * ($77), dims each block via two C520 calls, 5 passes, commits via C569. */
#include "ram.h"
#include "regs.h"

void sub_C520(Regs *r);
void sub_C135(Regs *r);
void sub_C569(Regs *r);

void sub_C4E0(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    u8 v = 0x40;
    RAM8(0x09) = v;
    do {
        u8 y;
        RAM8(0x36) = 0x05;
        for (y = 0xE0; y < 0xE4; ++y)   /* -> $0180..$0183 */
            RAM8((u16)(0x00A0 + y)) = RAM8((u16)(ptr + y));
        for (y = 0xF0; y < 0xF4; ++y)   /* -> $0190..$0193 */
            RAM8((u16)(0x00A0 + y)) = RAM8((u16)(ptr + y));
        r->x = 0x00; r->y = 0x04;
        sub_C520(r);
        r->x = 0x10; r->y = 0x04;
        sub_C520(r);
        sub_C135(r);
        v = (u8)(RAM8(0x09) - 0x10);
        RAM8(0x09) = v;
    } while ((v & 0x80) == 0);
    sub_C569(r);
}
