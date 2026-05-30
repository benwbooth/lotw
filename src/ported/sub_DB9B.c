/* $DB9B:
 *   LDA #$13 / STA $008F / LDX #$3C
 *   LDA $88 / BEQ $DBB4
 *   LDA $89 / BEQ $DBB2
 *   LDA $8A / BEQ $DBB0
 *   STX $8B
 * $DBB0: STX $8A
 * $DBB2: STX $89
 * $DBB4: STX $88 / RTS
 * Sets $8F=$13, then cascades $3C into the first zero slot of $88/$89/$8A/$8B. */
#include "ram.h"
#include "regs.h"

void sub_DB9B(Regs *r)
{
    u8 x = 0x3C;
    u8 a;
    RAM8(0x8F) = 0x13;
    a = RAM8(0x88);                 /* LDA $88 */
    if (a != 0) {                   /* BEQ $DBB4 */
        a = RAM8(0x89);             /* LDA $89 */
        if (a != 0) {               /* BEQ $DBB2 */
            a = RAM8(0x8A);         /* LDA $8A */
            if (a != 0) {           /* BEQ $DBB0 */
                RAM8(0x8B) = x;
            }
            RAM8(0x8A) = x;
        }
        RAM8(0x89) = x;
    }
    RAM8(0x88) = x;
    r->a = a;
    r->x = x;
}
