/* $E98F:  LDY #$0F / loop: LDA ($E5),Y / STA $00ED,Y / DEY / BPL loop / RTS
 * Copies 16 bytes from buffer pointed to by $E5/$E6 into $00ED..$00FC. */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r)
{
    u16 ptr = (u16)(RAM8(0xE5) | (RAM8(0xE6) << 8));
    int y;
    for (y = 0x0F; y >= 0; --y)
        RAM8((u16)(0x00ED + y)) = RAM8((u16)(ptr + y));
    r->y = 0xFF;
}
