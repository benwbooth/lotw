/* $CAE2:  clamp keys ($5B) to <=$6D, store; STA $08; LDX #$0C;
 * JSR sub_CB0E; LDA #$01 / STA $3C; RTS. */
#include "ram.h"
#include "regs.h"

void sub_CB0E(Regs *r);

void sub_CAE2(Regs *r)
{
    u8 v = keys;
    if (v >= 0x6D)
        v = 0x6D;
    keys = v;
    RAM8(0x08) = v;
    r->a = v;
    r->x = 0x0C;
    sub_CB0E(r);
    r->a = 0x01;
    RAM8(0x3C) = 0x01;
}
