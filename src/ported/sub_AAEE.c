/* $AAEE — refresh helper: rebuild the scroll/OAM state ($C1D8), run $C2B1,
 * then request one PPU job ($36=1) and frame-sync via $C135.
 * C135 frame-syncs on $28/$36 (sync_clear). */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r);
void sub_C2B1(Regs *r);
void sub_C135(Regs *r);

void sub_AAEE(Regs *r)
{
    sub_C1D8(r);                /* JSR $C1D8 */
    sub_C2B1(r);                /* JSR $C2B1 */
    RAM8(0x36) = 0x01;          /* LDA #$01 / STA $36 */
    sub_C135(r);                /* JSR $C135 */
}
