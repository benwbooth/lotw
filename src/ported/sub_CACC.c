/* $CACC:  clamp magic ($59) to <=$6D, store; STA $08; LDX #$06;
 * JSR sub_CB0E; LDA #$01 / STA $3C; RTS. */
#include "ram.h"
#include "regs.h"

void sub_CB0E(Regs *r);

void sub_CACC(Regs *r)
{
    u8 v = magic;
    if (v >= 0x6D)
        v = 0x6D;
    magic = v;
    RAM8(0x08) = v;
    r->a = v;
    r->x = 0x06;
    sub_CB0E(r);
    r->a = 0x01;
    RAM8(0x3C) = 0x01;
}
