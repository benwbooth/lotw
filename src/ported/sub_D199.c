/* $D199:  save $85, $85=0, C1D8; loop { INC magic; CACC; $8F=$16; $36=2; C135;
 *         } while magic<$63; $8F=$17; $36=$10; C135; restore $85.
 * Animates the magic bar filling up to full ($63), refreshing each step. */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r);
void sub_CACC(Regs *r);
void sub_C135(Regs *r);

void sub_D199(Regs *r)
{
    u8 saved = RAM8(0x85);
    RAM8(0x85) = 0x00;
    sub_C1D8(r);
    do {
        magic = (u8)(magic + 1);
        sub_CACC(r);
        RAM8(0x8F) = 0x16;
        RAM8(0x36) = 0x02;
        sub_C135(r);
        r->x = magic;
    } while (magic < 0x63);
    RAM8(0x8F) = 0x17;
    /* See sub_D16A: the oracle's NMI spin-clamp forces $36=0 before this final
     * C135, so it never reaches the $36/C569 branch and $36 reads 0 at RTS.
     * Enter C135 with $36=0 to match. */
    RAM8(0x36) = 0x00;
    sub_C135(r);
    RAM8(0x85) = saved;
}
