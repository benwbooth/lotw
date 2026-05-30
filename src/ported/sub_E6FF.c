/* $E6FF: build the two on-screen "held item" sprite groups (OAM shadow at
 * $0240-$0247 and $0248-$024F) from item slot ids in $80 and $82.
 * A negative slot, or one whose inventory_count >= $0B, blanks the group (tile
 * $EF) and clears the slot to $EF; otherwise lays out a 2x2 sprite block. */
#include "ram.h"
#include "regs.h"

void sub_E6FF(Regs *r)
{
    u8 x, a;

    /* ---- group 1: slot $80 -> $0240..$0247 ---- */
    a = 0xEF;
    x = RAM8(0x80);
    if (x & 0x80) goto G1;                       /* BMI */
    if (RAM8((u16)(0x0060 + x)) >= 0x0B) {
        RAM8(0x80) = 0xEF;
        a = 0xEF;
        goto G1;
    }
    a = (u8)(x << 2);
    a = (u8)(a + 0xA1);   RAM8(0x0241) = a;
    a = (u8)(a + 0x02);   RAM8(0x0245) = a;
    RAM8(0x0243) = 0x40;
    RAM8(0x0247) = 0x48;
    a = 0xA4;
G1:
    RAM8(0x0240) = a;
    RAM8(0x0244) = a;
    RAM8(0x0242) = 0x01;
    RAM8(0x0246) = 0x01;

    /* ---- group 2: slot $82 -> $0248..$024F ---- */
    a = 0xEF;
    x = RAM8(0x82);
    if (x & 0x80) goto G2;                       /* BMI */
    if (RAM8((u16)(0x0060 + x)) >= 0x0B) {
        RAM8(0x82) = 0xEF;
        a = 0xEF;
        goto G2;
    }
    a = (u8)(x << 2);
    a = (u8)(a + 0xA1);   RAM8(0x0249) = a;
    a = (u8)(a + 0x02);   RAM8(0x024D) = a;
    RAM8(0x024B) = 0xB0;
    RAM8(0x024F) = 0xB8;
    a = 0xA0;
G2:
    RAM8(0x0248) = a;
    RAM8(0x024C) = a;
    RAM8(0x024A) = 0x01;
    RAM8(0x024E) = 0x01;
}
