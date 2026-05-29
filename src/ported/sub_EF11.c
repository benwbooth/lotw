/* $EF11:  LDA #$00 / STA $F5 / STA $F7 / STA $F1 / STA $F0 / RTS
 * Clears scroll/state bytes $F5, $F7, $F1, $F0 to zero. No inputs. */
#include "ram.h"
#include "regs.h"

void sub_EF11(Regs *r)
{
    (void)r;
    RAM8(0xF5) = 0;
    RAM8(0xF7) = 0;
    RAM8(0xF1) = 0;
    RAM8(0xF0) = 0;
}
