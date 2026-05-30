/* $E7CE:  LDA health / BEQ L_E7D9 / DEC health / JSR L_CAB6 / CLC / RTS
 *         L_E7D9: SEC / RTS
 * If health != 0: decrement it, refresh display (CAB6), return C=0.
 * If health == 0: return C=1 (cannot decrement). */
#include "ram.h"
#include "regs.h"

void sub_CAB6(Regs *r);

void sub_E7CE(Regs *r)
{
    r->a = health;
    if (r->a == 0) {
        r->c = 1;
        return;
    }
    health = (u8)(health - 1);
    sub_CAB6(r);
    r->c = 0;
}
