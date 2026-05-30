/* $E842:  STA $08 / LDA gold / SEC / SBC $08 / BCC L_E851 / STA gold
 *         JSR L_CAF8 / SEC / L_E851: RTS
 * Subtract A from gold. If it would go negative (borrow), leave gold unchanged
 * and return C=0. Otherwise store result, refresh display (CAF8), return C=1. */
#include "ram.h"
#include "regs.h"

void sub_CAF8(Regs *r);

void sub_E842(Regs *r)
{
    RAM8(0x08) = r->a;
    u16 res = (u16)gold - (u16)RAM8(0x08);
    r->a = (u8)res;
    if (res & 0x100) {
        /* borrow: carry clear, no store */
        r->c = 0;
        return;
    }
    gold = r->a;
    sub_CAF8(r);
    r->c = 1;
}
