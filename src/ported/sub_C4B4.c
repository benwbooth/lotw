/* $C4B4:  $09=$40; loop { $36=5; copy 4B ($77)+$E0 -> $00A0+$E0; X=0,Y=4,C520;
 *         C135; $09-=$10; BPL }; C569
 * Like C492 but refreshes 4 palette bytes ($0180..$0183) from ($77) each pass
 * instead of C9FB, then dims via C520 (X=0,Y=4) over 5 passes, commits via C569. */
#include "ram.h"
#include "regs.h"

void sub_C520(Regs *r);
void sub_C135(Regs *r);
void sub_C569(Regs *r);

void sub_C4B4(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    u8 v = 0x40;
    RAM8(0x09) = v;
    do {
        u8 y;
        RAM8(0x36) = 0x05;
        for (y = 0xE0; y < 0xE4; ++y)   /* X=4: Y=$E0..$E3 */
            RAM8((u16)(0x00A0 + y)) = RAM8((u16)(ptr + y));
        r->x = 0x00;
        r->y = 0x04;
        sub_C520(r);
        sub_C135(r);
        v = (u8)(RAM8(0x09) - 0x10);
        RAM8(0x09) = v;
    } while ((v & 0x80) == 0);
    sub_C569(r);
}
