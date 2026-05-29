/* $D8B6: compute a target tile/pixel position from the player position plus
 * deltas $49/$4A (X) and $4B (Y).
 *   LDA $43 / STA $0E         ; player_x_fine
 *   LDA $44 / STA $0F         ; player_x_tile
 *   LDA $45 / STA $0A         ; player_y
 *   LDA $4B / BEQ +           ; if Y delta != 0:
 *     CLC / ADC $0A / STA $0A ;   $0A += $4B
 * + LDA $49 / BEQ ++          ; if X-fine delta != 0:
 *     CLC / ADC $0E / PHA     ;   sum = $49 + $0E
 *     AND #$0F / STA $0E      ;   $0E = sum & $0F  (A is still sum here)
 *     PLA / ASL Ax4           ;   A = sum<<4 (discarded; sets carry = bit4 of sum)
 *     LDA $0F / ADC $4A / STA $0F  ; $0F = $0F + $4A + carry
 * ++ RTS
 * Note: after the 4x ASL the shifted value is overwritten by LDA $0F; only the
 * carry-out (bit4 of sum) survives into the ADC $4A.
 */
#include "ram.h"
#include "regs.h"

void sub_D8B6(Regs *r)
{
    u8 dx, sum, carry;

    RAM8(0x0E) = RAM8(0x43);    /* player_x_fine */
    RAM8(0x0F) = RAM8(0x44);    /* player_x_tile */
    RAM8(0x0A) = RAM8(0x45);    /* player_y */

    if (RAM8(0x4B) != 0)
        RAM8(0x0A) = (u8)(RAM8(0x4B) + RAM8(0x0A));

    dx = RAM8(0x49);
    if (dx != 0) {
        sum = (u8)(dx + RAM8(0x0E));      /* CLC / ADC $0E -> sum */
        RAM8(0x0E) = (u8)(sum & 0x0F);    /* AND #$0F on sum */
        carry = (u8)((sum >> 4) & 1);     /* carry after ASL A x4 */
        RAM8(0x0F) = (u8)(RAM8(0x0F) + RAM8(0x4A) + carry);
    }
}
