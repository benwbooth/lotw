/* Uniform ABI for ported routines: every port is `void <name>(Regs *r)`.
 * r carries the 6502 registers + flags in and out; memory is the global address
 * space via RAM8()/REG_W(). This lets the bulk diff-test harness call any
 * routine generically while the bodies stay readable. */
#ifndef LOTW_REGS_H
#define LOTW_REGS_H
#include "nes.h"

typedef struct {
    u8 a, x, y;        /* accumulator / index registers (in & out) */
    u8 c, z, n, v;     /* carry, zero, negative, overflow flags (in & out, 0/1) */
} Regs;

/* NB on the 6502 stack: the port does NOT maintain a real stack pointer. JSR/RTS
 * are modelled as C calls (return addresses live on the C stack, not the $0100
 * page), and PHA/PLA are modelled as balanced C locals. A few routines also write
 * fixed $0100-page addresses directly (e.g. sub_CB0E saves X at $01FB to match the
 * diff-test oracle's stack layout). Because there is no call-depth-accurate S, the
 * $0100 page is NOT a coherent shared stack — so data that must persist across a
 * return (the E620/E642 room checkpoint) is kept in a dedicated store, not on it. */

typedef void (*PortFn)(Regs *r);

#endif /* LOTW_REGS_H */
