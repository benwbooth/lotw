/* $CC97 — wait until the queued VRAM job is done (LDA nmi_vram_req / BNE / RTS).
 * After the wait the NMI has cleared $28, so the port leaves nmi_vram_req=0. */
#include "ram.h"
#include "regs.h"
void sub_CC97(Regs *r)
{
    RAM8(0x28) = 0;
}
