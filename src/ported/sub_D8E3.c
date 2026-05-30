/* $D8E3: player sprite/animation state selector. Computes a sprite-frame index
 * in $56 (some paths merge it with low bits of the old $56) and a flip flag $57,
 * using flags/state $46,$50,$20,$4B,$4E,$4F,$4A,$49. Scratch $08. */
#include "ram.h"
#include "regs.h"

void sub_D8E3(Regs *r)
{
    u8 x, y, a;

    x = 0x3D;
    if (RAM8(0x46) != 0) { RAM8(0x56) = x; return; }      /* D92E */
    x = 0x09;
    if (RAM8(0x50) != 0) { RAM8(0x56) = x; return; }      /* D92E */
    if ((RAM8(0x20) & 0xBF) == 0x80) { RAM8(0x56) = x; return; } /* D92E, X=$09 */

    a = RAM8(0x4B);
    if (a == 0) goto D913;
    if (a & 0x80) goto D90C;                              /* BMI */

    /* $4B positive nonzero */
    if (RAM8(0x4E) != 0) goto D931;
    if ((RAM8(0x20) & 0x04) == 0) goto D913;
    x = 0x0D;
    RAM8(0x56) = x;                                       /* JMP D92E */
    return;

D90C:
    if (RAM8(0x4F) == 0) { RAM8(0x56) = x; return; }      /* D92E, X=$09 */
    goto D931;

D913:
    x = 0x01;
    y = 0x00;
    if (RAM8(0x4A) & 0x80) goto D921;                     /* BMI */
    if (RAM8(0x49) == 0) return;                          /* D930: RTS, no store */
    y = 0x40;
D921:
    RAM8(0x08) = x;
    RAM8(0x56) = (RAM8(0x56) & 0x07) | RAM8(0x08);
    RAM8(0x57) = y;
    return;

D931:
    x = 0x39;
    y = 0x00;
    a = RAM8(0x4A) | RAM8(0x49);
    if (a & 0x80) goto D941;                              /* BMI */
    if (a != 0) goto D93F;                                /* BNE */
    x = 0x09;
D93F:
    y = 0x40;
D941:
    RAM8(0x08) = x;
    RAM8(0x56) = (RAM8(0x56) & 0x03) | RAM8(0x08);
    RAM8(0x57) = y;
}
