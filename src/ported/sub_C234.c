/* $C234: build the inventory/equipped-item sprite shadow block at $0220-$023F.
 * First lays out the equipped item ($0055): if >=3 it is blanked (tile $EF in
 * $0238/$023C), otherwise a 2x2 sprite at column from equipped<<4. Then loops
 * over carried_item0..2 ($0051..$0053) -> three 2x2 groups at $0220,$0228,$0230. */
#include "ram.h"
#include "regs.h"

#define equipped_item RAM8(0x0055)
#define carried_item0 0x0051

void sub_C234(Regs *r)
{
    u8 a, x, y;

    a = equipped_item;
    x = 0x13;
    if (a >= 0x03) {                      /* BCC failed */
        x = 0xEF;
        RAM8(0x0238) = x;
        RAM8(0x023C) = x;
        /* JMP C26F, X=$EF (unused below) */
    } else {                              /* C247: X=$13 */
        RAM8(0x0238) = x;
        RAM8(0x023C) = x;
        a = (u8)(a << 4);
        a = (u8)(a + 0xC8);  RAM8(0x023B) = a;
        a = (u8)(a + 0x08);  RAM8(0x023F) = a;
        RAM8(0x0239) = 0xFF;
        RAM8(0x023D) = 0xFF;
        RAM8(0x023A) = 0x01;
        RAM8(0x023E) = 0x41;
    }

    /* C26F: loop X=2..0, Y=$10,$08,$00 */
    x = 0x02;
    y = 0x10;
    for (;;) {
        a = RAM8((u16)(carried_item0 + x));
        if (a & 0x80) {                   /* BMI -> C2A0 */
            a = 0xEF;
        } else {
            a = (u8)(a << 2);
            a = (u8)(a + 0xA1);  RAM8((u16)(0x0221 + y)) = a;
            a = (u8)(a + 0x02);  RAM8((u16)(0x0225 + y)) = a;
            a = (u8)(y << 1);
            a = (u8)(a + 0xC8);  RAM8((u16)(0x0223 + y)) = a;
            a = (u8)(a + 0x08);  RAM8((u16)(0x0227 + y)) = a;
            RAM8((u16)(0x0222 + y)) = 0x01;
            RAM8((u16)(0x0226 + y)) = 0x01;
            a = 0x13;
        }
        /* C2A2 */
        RAM8((u16)(0x0220 + y)) = a;
        RAM8((u16)(0x0224 + y)) = a;
        y = (u8)(y - 0x08);
        if (x-- == 0) break;              /* DEX / BPL: stop after X went 0->FF */
    }
}
