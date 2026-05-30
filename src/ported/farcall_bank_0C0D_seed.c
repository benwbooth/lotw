/* $CD08 farcall_bank_0C0D_seed
 * Saves the current MMC3 r6/r7 bank shadows into $32/$33, then programs banks
 * $0C (r6) and $0D (r7), updating the select-shadow ($25) and bank shadows
 * ($30/$31). MMC3 register writes ($8000/$8001) are hardware-only.
 *
 *   LDA $30 / STA $32        save r6 shadow
 *   LDA $31 / STA $33        save r7 shadow
 *   LDY #$06 / STY $25 / STY $8000
 *   LDA #$0C / STA $30 / STA $8001
 *   INY      / STY $25 / STY $8000
 *   LDA #$0D / STA $31 / STA $8001
 *   RTS
 */
#include "ram.h"
#include "regs.h"

void farcall_bank_0C0D_seed(Regs *r)
{
    RAM8(0x32) = RAM8(0x30);            /* save r6 shadow */
    RAM8(0x33) = RAM8(0x31);            /* save r7 shadow */

    RAM8(0x25) = 0x06;                  /* select r6 */
    REG_W(0x8000, 0x06);
    RAM8(0x30) = 0x0C;
    REG_W(0x8001, 0x0C);

    RAM8(0x25) = 0x07;                  /* INY -> select r7 */
    REG_W(0x8000, 0x07);
    RAM8(0x31) = 0x0D;
    REG_W(0x8001, 0x0D);

    r->a = 0x0D;
    r->y = 0x07;
}
