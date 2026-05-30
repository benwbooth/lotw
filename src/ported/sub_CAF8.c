/* $CAF8:  clamp gold ($5A) to <=$6D, store; STA $08; LDX #$12;
 * JSR sub_CB0E; LDA #$01 / STA $3C; RTS. */
#include "ram.h"
#include "regs.h"

void sub_CB0E(Regs *r);

void sub_CAF8(Regs *r)
{
    u8 v = gold;
    if (v >= 0x6D)
        v = 0x6D;
    gold = v;
    RAM8(0x08) = v;
    r->a = v;
    r->x = 0x12;
    sub_CB0E(r);
    r->a = 0x01;
    RAM8(0x3C) = 0x01;
}
