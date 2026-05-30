/* $CAB6:  clamp health ($58) to <=$6D, store; STA $08; LDX #$00;
 * JSR sub_CB0E; LDA #$01 / STA $3C; RTS.
 * Sets up the health-bar tile update via sub_CB0E. */
#include "ram.h"
#include "regs.h"

void sub_CB0E(Regs *r);

void sub_CAB6(Regs *r)
{
    u8 v = health;
    if (v >= 0x6D)
        v = 0x6D;
    health = v;
    RAM8(0x08) = v;
    r->a = v;
    r->x = 0x00;
    sub_CB0E(r);
    r->a = 0x01;
    RAM8(0x3C) = 0x01;
}
