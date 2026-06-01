/* $B0B1 — screen teardown (reached by JMP from the title/character-select path).
 * Resets state via C461, far-calls $C38B (scene/state reset) through the $CCE4
 * return-home epilogue, then runs the screen-rebuild chain C57A/C375/B631 and the
 * four CAB6/CAF8/CAE2/CAF8 attribute/nametable uploads, sets the 1-frame job count
 * $36=1 and far-calls the frame commit $C135 (via $CCE4) before returning.
 *
 * INSPECTION-PORT (no diff-test spec): far-calls + frame-commit side effects on the
 * PPU job queue / bank shadows; not isolation-testable in flat memory. Integration-
 * verified. Far-calls handled: $C38B and $C135, both via farcall_return_home ($CCE4). */
#include "ram.h"
#include "regs.h"

void sub_C461(Regs *r); void sub_C38B(Regs *r); void sub_C57A(Regs *r);
void sub_C375(Regs *r); void sub_B631(Regs *r); void sub_CAB6(Regs *r);
void sub_CAF8(Regs *r); void sub_CAE2(Regs *r); void sub_C135(Regs *r);

/* $CCE4 farcall_return_home: restore banks from $32/$33, run target, then the
 * $CD08 seed re-maps banks 12/13 with select=$07 (see src/ported/sub_B6A6.c). */
static void farcall_cce4(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33); RAM8(0x25) = 0x06; NES_PRG_SYNC();
    target(r);
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
}

void sub_B0B1(Regs *r)
{
    sub_C461(r);                                /* JSR $C461 */
    farcall_cce4(r, 0x8B, 0xC3, sub_C38B);      /* far-call $C38B */
    sub_C57A(r);                                /* JSR $C57A */
    sub_C375(r);                                /* JSR $C375 */
    sub_B631(r);                                /* JSR L_B631 */
    sub_CAB6(r);                                /* JSR $CAB6 */
    sub_CAF8(r);                                /* JSR $CAF8 */
    sub_CAE2(r);                                /* JSR $CAE2 */
    sub_CAF8(r);                                /* JSR $CAF8 */
    RAM8(0x36) = 0x01;                          /* LDA #$01 / STA $36 */
    farcall_cce4(r, 0x35, 0xC1, sub_C135);      /* far-call $C135 (frame commit) */
}
