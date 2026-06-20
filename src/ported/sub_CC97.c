/* $CC97 — wait until the queued VRAM job is done (LDA vblank_vram_req / BNE / RTS).
 * After the wait the frame commit has cleared $28, so the port leaves the request at 0. */
#include "ram.h"
#include "regs.h"
void sub_CC97(Regs *r)
{
    RAM8(0x28) = 0;
}
