/* $E778: store constants into OAM-ish slots $0250..$0257 (two 4-byte sprites). */
#include "ram.h"
#include "regs.h"

void sub_E778(Regs *r)
{
    RAM8(0x0250) = 0x98;
    RAM8(0x0254) = 0x98;
    RAM8(0x0251) = 0xF1;
    RAM8(0x0255) = 0xF3;
    RAM8(0x0252) = 0x02;
    RAM8(0x0256) = 0x02;
    RAM8(0x0253) = 0x78;
    RAM8(0x0257) = 0x80;
    r->a = 0x80;                 /* last LDA #$80 */
}
