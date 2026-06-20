/* Uniform ABI for ported routines: every port is `void <name>(Regs *r)`.
 * r carries the legacy accumulator/index values and condition flags in and out;
 * memory is the global game address space via RAM8()/REG_W(). This lets tests
 * call any routine generically while the bodies stay readable. */
#ifndef LOTW_REGS_H
#define LOTW_REGS_H
#include "nes.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    u8 a, x, y;        /* accumulator / index registers (in & out) */
    u8 c, z, n, v;     /* carry, zero, negative, overflow flags (in & out, 0/1) */
} Regs;

/* The port does not maintain a hardware stack pointer. Calls use the C stack, and
 * old push/pop idioms are represented as balanced C locals. A few routines also
 * write fixed $0100-page addresses directly for game-visible scratch storage.
 * The $0100 page is not a coherent shared call stack, so data that must persist
 * across a return is kept in a dedicated store instead. */

typedef void (*PortFn)(Regs *r);

#ifdef __cplusplus
}
#endif

#endif /* LOTW_REGS_H */
