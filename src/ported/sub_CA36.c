/* $CA36:
 *   LDA map_screen_y / LSR A / TAX / INX     ; X = (msy>>1)+1  (rotate count)
 *   LDA #$FF / CLC
 * L_CA3E:
 *   ROR A / DEX / BNE L_CA3E                  ; build a clear-mask in A
 *   PHA
 *   LDA map_screen_y / ASL A / ASL A / AND #$04 / ORA map_screen_x / TAX
 *   PLA / AND save_inventory,X / STA save_inventory,X   ; clear bits in slot
 *   RTS
 * Clears a run of high bits in a save_inventory ($0300) byte. The rotate of $FF
 * right (X = (msy>>1)+1) times, starting carry=0, produces the AND-mask. The
 * target slot index is ((msy<<2)&4) | map_screen_x. */
#include "ram.h"
#include "regs.h"

void sub_CA36(Regs *r)
{
    u8 msy = RAM8(0x48);          /* map_screen_y */
    u8 x   = (u8)((msy >> 1) + 1);
    u8 a   = 0xFF;
    u8 carry = 0;
    u8 idx;

    do {                          /* ROR A / DEX / BNE (runs >=1, X>=1) */
        u8 newcarry = a & 1;
        a = (u8)((carry << 7) | (a >> 1));
        carry = newcarry;
        x--;
    } while (x != 0);

    idx = (u8)((((u8)(msy << 2)) & 0x04) | RAM8(0x47));   /* map_screen_x */
    RAM8((u16)(0x0300 + idx)) &= a;

    r->a = RAM8((u16)(0x0300 + idx));   /* final A = result of AND/STA */
    r->x = idx;
}
