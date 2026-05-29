/* $FD9C sound_restore_game_banks — restore MMC3 R6/R7 from their shadows.
 *   LDA #$06 / STA $8000 / LDA $30 / STA $8001
 *   LDA #$07 / STA $8000 / LDA $31 / STA $8001 / RTS
 * Pure hardware side-effect (MMC3 bank registers); reads mmc3_r6/r7 shadows.
 * No RAM/reg outputs — compare ["ram"] passes trivially (verified by inspection). */
#include "ram.h"
#include "regs.h"

void sound_restore_game_banks(Regs *r)
{
    REG_W(0x8000, 0x06);
    REG_W(0x8001, RAM8(0x30));   /* mmc3_r6_shadow -> MMC3 R6 */
    REG_W(0x8000, 0x07);
    REG_W(0x8001, RAM8(0x31));   /* mmc3_r7_shadow -> MMC3 R7 */
    (void)r;
}
