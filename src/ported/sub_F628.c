/* $F628: iterate over the projectile/shot entity table and step each one.
 *   $E3 = $0B (record index)
 *   $E5/$E6 = $04B0 (pointer to current record)
 * loop (L_F634):
 *   Y=1; A = ($E5),Y
 *   if A != 0:                JSR F6BB         ; active shot -> step it
 *   else:
 *     if ($20 & $40)==0:      skip             ; BIT $20 / BVC
 *     else if ($FD & $40)!=0: skip             ; BIT $FD / BVS
 *     else:                   JSR F664         ; free slot, fire new shot
 *   $E3++
 *   $E5/$E6 += $10
 *   if (($E3 - $0B) < shots_allowed) loop
 *   RTS
 * Net effect is whatever the callees mutate in RAM, plus $E3/$E5/$E6 final state. */
#include "ram.h"
#include "regs.h"

void sub_F664(Regs *r);
void sub_F6BB(Regs *r);

void sub_F628(Regs *r)
{
    RAM8(0xE3) = 0x0B;
    RAM8(0xE5) = 0xB0;
    RAM8(0xE6) = 0x04;

    do {
        u16 ptr = (u16)(RAM8(0xE5) | (RAM8(0xE6) << 8));
        u8 v = RAM8((u16)(ptr + 1));            /* LDY #$01; LDA ($E5),Y */

        if (v != 0) {                            /* BNE L_F648 */
            r->a = v; r->y = 0x01;
            sub_F6BB(r);
        } else {
            /* BIT $20 -> V = bit6 of $20 */
            if (RAM8(0x20) & 0x40) {             /* BVC L_F64B skips when clear */
                /* BIT $FD -> V = bit6 of $FD; BVS L_F64B skips when set */
                if (!(RAM8(0xFD) & 0x40)) {
                    r->a = 0x00; r->y = 0x01;
                    sub_F664(r);
                }
            }
        }

        RAM8(0xE3)++;                            /* INC $E3 */
        {                                        /* $E5/$E6 += $10 */
            u16 t = (u16)(0x10 + RAM8(0xE5));
            RAM8(0xE5) = (u8)t;
            RAM8(0xE6) = (u8)(RAM8(0xE6) + (t >> 8));
        }
    } while ((u8)(RAM8(0xE3) - 0x0B) < RAM8(0x5E));   /* CMP shots_allowed; BCC */
}
