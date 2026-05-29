/* $F552:  INC $F3 / LDA $F3 / AND #$0C / ASL A / ORA #$41 / STA $ED / RTS
 * Advances animation counter $F3, derives a tile/attr byte into $ED. */
#include "ram.h"
#include "regs.h"

void sub_F552(Regs *r)
{
    u8 a;
    a = ++RAM8(0xF3);
    a = (u8)(((a & 0x0C) << 1) | 0x41);
    RAM8(0xED) = a;
    r->a = a;
}
