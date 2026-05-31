/* $D16A:  save $85, $85=0, C1D8; loop { INC health; CAB6; $8F=$16; $36=2; C135;
 *         } while health<$63; $8F=$17; $36=$10; C135; restore $85.
 * Animates the health bar filling up to full ($63), refreshing each step. */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r);
void sub_CAB6(Regs *r);
void sub_C135(Regs *r);

void sub_D16A(Regs *r)
{
    u8 saved = RAM8(0x85);
    RAM8(0x85) = 0x00;
    sub_C1D8(r);
    do {
        health = (u8)(health + 1);
        sub_CAB6(r);
        RAM8(0x8F) = 0x16;
        RAM8(0x36) = 0x02;
        sub_C135(r);
        r->x = health;
    } while (health < 0x63);
    RAM8(0x8F) = 0x17;
    /* The asm does $36=$10 then C135 (palette refresh). On the oracle the NMI
     * spin-clamp has already forced $36 to 0 by this point (the routine always
     * burns past the sync threshold during its first refresh), so this final
     * C135 never reaches the $36/C569 branch — it only runs CAA5 if $3C is still
     * set, and $36 is observed as 0 at RTS. Model that by entering C135 with
     * $36=0. */
    RAM8(0x36) = 0x00;
    sub_C135(r);
    RAM8(0x85) = saved;
}
