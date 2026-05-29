/* $F740 sub_F740 — compute a packed target X/Y position into scratch $0E/$0F/$0A
 * from the player position ($43/$44/$45) plus per-step deltas $F5/$F6/$F7.
 *
 *   $0E = player_x_fine ($43); $0F = player_x_tile ($44); $0A = player_y ($45)
 *   if $F7: A = ($F7<<2) + $0A; $0A = A
 *   if $F5: A = (($F5<<2) & $0F) + $0E   (the "pulled" byte)
 *           $0E = A & $0F
 *           $0F = $0F + $F6 + (bit4 of A)   -- carry left by the dead ASL A x4
 * Note: the 4x ASL A on the pulled value is otherwise dead (LDA $0F overwrites A);
 * its only effect is the carry into ADC $F6, which equals bit 4 of the pulled byte.
 */
#include "ram.h"
#include "regs.h"

void sub_F740(Regs *r)
{
    RAM8(0x0E) = RAM8(0x43);            /* player_x_fine */
    RAM8(0x0F) = RAM8(0x44);            /* player_x_tile */
    RAM8(0x0A) = RAM8(0x45);            /* player_y */

    if (RAM8(0xF7) != 0) {              /* BEQ L_F757 */
        u8 a = (u8)(RAM8(0xF7) << 2);
        a = (u8)(a + RAM8(0x0A));       /* CLC ADC $0A */
        RAM8(0x0A) = a;
    }

    if (RAM8(0xF5) != 0) {              /* BEQ L_F772 */
        u8 pulled = (u8)((u8)((RAM8(0xF5) << 2) & 0x0F) + RAM8(0x0E)); /* CLC ADC $0E */
        RAM8(0x0E) = pulled & 0x0F;
        /* ASL A x4 of pulled leaves carry = bit4 of pulled; LDA $0F discards A */
        RAM8(0x0F) = (u8)(RAM8(0x0F) + RAM8(0xF6) + ((pulled >> 4) & 1));
    }
}
