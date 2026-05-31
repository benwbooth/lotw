/* $A75D — title-screen sprite cycler. Decrement frame counter $3E (wrap to 7
 * when it goes negative). If ($3E & $06) != 0, pull a 4-byte OAM entry from the
 * $0280 table at index ($3E*4) into OAM $0200-$0203 and blank the source's Y
 * byte to #$EF; otherwise do the same from the $0210 table. Pure OAM/data. */
#include "ram.h"
#include "regs.h"

void sub_A75D(Regs *r)
{
    (void)r;
    u8 c = (u8)(RAM8(0x3E) - 1);           /* DEC $3E */
    if (c & 0x80)                          /* BPL skips; else negative */
        c = 0x07;                          /* LDA #$07 / STA $3E */
    RAM8(0x3E) = c;

    u8 x = (u8)(c << 2);                   /* ASL A / ASL A / TAX */
    u16 base = (c & 0x06) ? 0x0280 : 0x0210;  /* AND #$06 / BEQ L_A78E */

    RAM8(0x0200) = RAM8((u16)(base + 0 + x));
    RAM8(0x0201) = RAM8((u16)(base + 1 + x));
    RAM8(0x0202) = RAM8((u16)(base + 2 + x));
    RAM8(0x0203) = RAM8((u16)(base + 3 + x));
    RAM8((u16)(base + x)) = 0xEF;          /* LDA #$EF / STA base,X */
}
