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

typedef void (*PortFn)(Regs *r);

#endif /* LOTW_REGS_H */
