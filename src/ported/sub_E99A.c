/* $E99A:  LDY #$0F / loop: LDA $00ED,Y / STA ($E5),Y / DEY / BPL loop / RTS
 * Copies 16 bytes from $00ED..$00FC into buffer pointed to by $E5/$E6. */
#include "ram.h"
#include "regs.h"

void sub_E99A(Regs *r)
{
    u16 ptr = (u16)(RAM8(0xE5) | (RAM8(0xE6) << 8));
    int y;
    for (y = 0x0F; y >= 0; --y)
        RAM8((u16)(ptr + y)) = RAM8((u16)(0x00ED + y));
    r->y = 0xFF;
}
