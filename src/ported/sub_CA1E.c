/* $CA1E:
 *   LDA map_screen_y / ASL A / ASL A / AND #$04 / ORA map_screen_x / TAX
 *   LDA save_inventory,X / PHA
 *   LDA map_screen_y / LSR A / TAX / INX / PLA
 *   -: ASL A / DEX / BNE - / RTS
 * Indexes save_inventory ($0300) by ((map_screen_y<<2)&4)|map_screen_x, then
 * returns that byte shifted left ((map_screen_y>>1)+1) times.
 */
#include "ram.h"
#include "regs.h"

void sub_CA1E(Regs *r)
{
    u8 ms_y = RAM8(0x48);                /* map_screen_y */
    u8 ms_x = RAM8(0x47);                /* map_screen_x */
    u8 idx = (u8)(((ms_y << 2) & 0x04) | ms_x);
    u8 a = RAM8((u16)(0x0300 + idx));    /* save_inventory,X */
    u8 cnt = (u8)((ms_y >> 1) + 1);      /* LSR A / TAX / INX */
    do {                                 /* ASL A / DEX / BNE */
        a = (u8)(a << 1);
    } while (--cnt != 0);
    r->a = a;
}
