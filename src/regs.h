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
    u8 s;              /* stack pointer (6502 S). Most routines model JSR/RTS as C
                        * calls and PHA/PLA as balanced C locals, so they ignore s.
                        * It exists for the few routines that genuinely leave data on
                        * the $0100 page across separate calls — e.g. the E620/E642
                        * room-checkpoint push/pop. Set to $FF (TXS) at boot. */
} Regs;

/* 6502 stack primitives over the real $0100-$01FF page, driven by r->s.
 * nes_push stores then post-decrements S; nes_pull pre-increments S then loads. */
static inline void nes_push(Regs *r, u8 val) { RAM8((u16)(0x0100 + r->s--)) = val; }
static inline u8   nes_pull(Regs *r)         { return RAM8((u16)(0x0100 + ++r->s)); }

typedef void (*PortFn)(Regs *r);

#endif /* LOTW_REGS_H */
