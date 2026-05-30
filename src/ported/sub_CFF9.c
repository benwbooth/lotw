/* $CFF9:  LDX #$D0 / STX $19 / loop: CMP #$0A / BCC + / SBC #$0A / INC $19 / JMP loop
 *         then ADC #$D0 / STA $18 / LDA $19 / CMP #$D0 / BNE + / LDA #$C0 / STA $19 / RTS
 * Splits A into ($18 = A%10 + $D0 with wrap to $C0, $19 = $D0 + A/10) — VRAM addr calc.
 * Entry A; carry on entry is irrelevant (first CMP sets it). */
#include "ram.h"
#include "regs.h"

void sub_CFF9(Regs *r)
{
    u8 a = r->a;
    u8 hi = 0xD0;

    while (a >= 0x0A) {          /* CMP #$0A / BCC out  (A>=$0A keeps looping) */
        a = (u8)(a - 0x0A);      /* SBC #$0A, carry set so exact subtract */
        ++hi;                    /* INC vram_src_hi */
    }
    a = (u8)(a + 0xD0);          /* ADC #$D0, carry clear */
    RAM8(0x18) = a;              /* STA vram_src_lo */
    if (hi == 0xD0)
        hi = 0xC0;
    RAM8(0x19) = hi;             /* STA vram_src_hi */
}
