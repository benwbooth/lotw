/* $D94E sub_D94E — per-frame update of facing/animation state in $56/$57.
 *
 *   if $46 == 0 and $56 < $20:
 *       if (bit6 of $20) set:  $56 |= $10   else  $56 &= $EF
 *   if ($20 & $0F) == 0:                       return
 *   if ($4E | $4F) != 0:                       return
 *   ++$4D
 *   if ($4D & $07) != 0:                       return
 *   if ($56 & $08):  $57 ^= $40   else  $56 ^= $04
 */
#include "ram.h"
#include "regs.h"

void sub_D94E(Regs *r)
{
    if (RAM8(0x46) == 0) {                     /* LDA $46 / BNE L_D967 */
        if (RAM8(0x56) < 0x20) {               /* CMP #$20 / BCS L_D967 */
            /* BIT $20: V = bit6 of $20 */
            if (RAM8(0x20) & 0x40)             /* BVS L_D963 */
                RAM8(0x56) = (u8)(RAM8(0x56) | 0x10);
            else
                RAM8(0x56) = (u8)(RAM8(0x56) & 0xEF);
        }
    }

    if ((RAM8(0x20) & 0x0F) == 0)              /* BEQ L_D990 */
        return;
    if ((RAM8(0x4F) | RAM8(0x4E)) != 0)        /* ORA / BNE L_D990 */
        return;
    RAM8(0x4D) = (u8)(RAM8(0x4D) + 1);         /* INC $4D */
    if ((RAM8(0x4D) & 0x07) != 0)              /* AND #$07 / BNE L_D990 */
        return;
    if (RAM8(0x56) & 0x08)                     /* AND #$08 / BNE L_D98A */
        RAM8(0x57) = (u8)(RAM8(0x57) ^ 0x40);
    else
        RAM8(0x56) = (u8)(RAM8(0x56) ^ 0x04);
}
