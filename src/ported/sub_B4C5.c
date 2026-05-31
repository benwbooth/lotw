/* $B4C5 — stash X->$56 and Y->$57, request 8 PPU jobs ($36=8), rebuild scroll/
 * OAM state ($C1D8), then frame-sync via $C135.
 * C135 frame-syncs on $28/$36 (sync_clear). */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r);
void sub_C135(Regs *r);

void sub_B4C5(Regs *r)
{
    RAM8(0x56) = r->x;          /* STX $56 */
    RAM8(0x57) = r->y;          /* STY $57 */
    RAM8(0x36) = 0x08;          /* LDA #$08 / STA $36 */
    sub_C1D8(r);                /* JSR $C1D8 */
    sub_C135(r);                /* JSR $C135 */
}
