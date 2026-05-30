/* $DB85:
 *   LDA #$13 / STA $008F / LDX #$1E
 *   LDA $88 / BEQ $DB98
 *   LDA $89 / BEQ $DB96
 *   STX $8A
 * $DB96: STX $89
 * $DB98: STX $88 / RTS
 * Sets $8F=$13, then cascades $1E into the first zero slot of $88/$89/$8A. */
#include "ram.h"
#include "regs.h"

void sub_DB85(Regs *r)
{
    u8 x = 0x1E;
    u8 a;
    RAM8(0x8F) = 0x13;
    a = RAM8(0x88);                 /* LDA $88 */
    if (a != 0) {                   /* BEQ $DB98 skips when $88==0 */
        a = RAM8(0x89);             /* LDA $89 */
        if (a != 0) {               /* BEQ $DB96 skips when $89==0 */
            RAM8(0x8A) = x;
        }
        RAM8(0x89) = x;
    }
    RAM8(0x88) = x;
    r->a = a;
    r->x = x;
}
