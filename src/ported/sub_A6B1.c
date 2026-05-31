/* $A6B1 — on-screen bounds test. Given $0A (Y coord) and $0E (X coord), set
 * carry (off-screen) if Y >= $A1, or ($0E >= $F1 and $0E != 0). Clear carry
 * (on-screen) if Y < $A1 and ($0E < $F1 or $0E == 0). No RAM writes; result is
 * the carry flag. */
#include "ram.h"
#include "regs.h"

void sub_A6B1(Regs *r)
{
    if (RAM8(0x0A) >= 0xA1) { r->c = 1; return; }   /* CMP #$A1 / BCS L_A6C1 (SEC) */
    if (RAM8(0x0E) < 0xF1)  { r->c = 0; return; }   /* CMP #$F1 / BCC L_A6C3 (CLC) */
    if (RAM8(0x0E) == 0x00) { r->c = 0; return; }   /* LDA $0E / BEQ L_A6C3 (CLC) */
    r->c = 1;                                        /* L_A6C1: SEC */
}
