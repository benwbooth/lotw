/* $D41D ppu_commit_banks:
 *   LDX #$07
 * L_D41F:
 *   LDA mmc3_r0_shadow,X   ; $2A,X
 *   STX MMC3_BANK_SELECT   ; $8000
 *   STA MMC3_BANK_DATA     ; $8001
 *   DEX / BPL L_D41F / RTS
 * Copies the 8 MMC3 bank-shadow bytes ($2A..$31) into the mapper. Pure hardware
 * side-effect (REG_W only); leaves X = $FF. Verified by inspection. */
#include "ram.h"
#include "regs.h"

void ppu_commit_banks(Regs *r)
{
    int x;
    for (x = 7; x >= 0; x--) {
        REG_W(0x8000, (u8)x);
        REG_W(0x8001, RAM8((u16)(0x2A + x)));
    }
    r->x = 0xFF;
}
