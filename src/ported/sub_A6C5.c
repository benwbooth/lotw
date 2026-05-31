/* $A6C5 — compute a boss point ($0E,$0A) from boss origin $F9/$FB plus optional
 * facing offsets. $0E = $F9 (+ $F5 if $F5 != 0); $0A = $FB (+ $F7 if $F7 != 0).
 * Pure compute, RTS. */
#include "ram.h"
#include "regs.h"

void sub_A6C5(Regs *r)
{
    RAM8(0x0E) = RAM8(0xF9);                 /* LDA $F9 / STA $0E */
    RAM8(0x0A) = RAM8(0xFB);                 /* LDA $FB / STA $0A */
    if (RAM8(0xF7) != 0)                      /* LDA $F7 / BEQ L_A6D6 */
        RAM8(0x0A) = (u8)(RAM8(0xF7) + RAM8(0x0A)); /* CLC / ADC $0A / STA $0A */
    if (RAM8(0xF5) != 0)                      /* LDA $F5 / BEQ L_A6DF */
        RAM8(0x0E) = (u8)(RAM8(0xF5) + RAM8(0x0E)); /* CLC / ADC $0E / STA $0E */
    (void)r;
}
