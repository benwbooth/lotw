/* Regression for D991 -> DAAA item pickup dispatch.
 *
 * The source behavior reaches DAAA with A still holding $0401,X. If the ported
 * code calls DAAA with stale A, collectible states do not increment inventory. */
#include "ram.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>

u8 NES_MEM[0x10000];

void sub_D991(Regs *r);

static int expect_u8(const char *name, u16 addr, u8 want)
{
    u8 got = RAM8(addr);
    if (got == want)
        return 0;
    fprintf(stderr, "%s $%04X: got %02X, expected %02X\n", name, addr, got, want);
    return 1;
}

int main(void)
{
    Regs r;
    memset(&r, 0, sizeof r);
    memset(NES_MEM, 0, sizeof NES_MEM);

    RAM8(0x43) = 0x00;      /* player_x_fine */
    RAM8(0x44) = 0x10;      /* player_x_tile */
    RAM8(0x45) = 0x50;      /* player_y */
    RAM8(0x49) = 0x00;      /* scan X delta */
    RAM8(0x4B) = 0x00;      /* scan Y delta */
    RAM8(0xE3) = 0xFF;      /* do not suppress any object slot */

    /* Slot index 8 uses object table offset $80. $0401,X = $0A means
     * dispatcher n=$08, inventory_counts[0]++. Coordinates overlap exactly. */
    RAM8(0x0400 + 0x80) = 0x02;
    RAM8(0x0401 + 0x80) = 0x0A;
    RAM8(0x0402 + 0x80) = 0x00;
    RAM8(0x040C + 0x80) = 0x00;
    RAM8(0x040D + 0x80) = RAM8(0x44);
    RAM8(0x040E + 0x80) = RAM8(0x45);

    r.a = 0x00;             /* stale A must not affect pickup dispatch */
    sub_D991(&r);

    int errors = 0;
    errors |= expect_u8("inventory count", 0x0060, 0x01);
    errors |= expect_u8("object active", 0x0401 + 0x80, 0x00);
    errors |= expect_u8("object y clear", 0x0406 + 0x80, 0xF0);
    errors |= expect_u8("oam clear 0", 0x0200 + ((8 << 3) | 0x80), 0xEF);
    errors |= expect_u8("oam clear 1", 0x0204 + ((8 << 3) | 0x80), 0xEF);
    return errors ? 1 : 0;
}
