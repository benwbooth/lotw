/* $E347 — "confirm password" action. Far-calls the password DECODE+VALIDATE
 * ($B577); if it returns SEC (invalid) just RTS. If valid (CLC), set the prompt,
 * commit (D0C5/CAE2/CAF8), then PLA/PLA to drop the caller's frame (non-local
 * return to the grandparent) and finish drawing the game screen (scroll, C7B5,
 * C1C7, E7B2).
 *
 * INSPECTION-PORT (no diff-test spec): the PLA/PLA is a non-local return the flat
 * Regs ABI can't model. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_B577(Regs *r); void sub_D0C5(Regs *r); void sub_CAE2(Regs *r);
void sub_CAF8(Regs *r); void sub_C7B5(Regs *r); void sub_C1C7(Regs *r);
void sub_E7B2(Regs *r);

void sub_E347(Regs *r)
{
    RAM8(0x0E) = 0x77;                 /* LDA #$77 / STA $0E */
    RAM8(0x0F) = 0xB5;                 /* LDA #$B5 / STA $0F */
    sub_B577(r);                       /* JSR farcall_bank_0C0D -> $B577 (validate) */
    if (r->c)                          /* BCC L_E355; SEC (bad password) -> RTS */
        return;

    /* L_E355 */
    RAM8(0x8F) = 0x10;
    sub_D0C5(r);
    sub_CAE2(r);
    sub_CAF8(r);
    /* PLA / PLA — drop caller frame (non-local return; see header) */

    /* L_E364 */
    RAM8(0x7C) = 0x20;                 /* scroll_x_tile */
    sub_C7B5(r);
    sub_C1C7(r);
    sub_E7B2(r);
}
