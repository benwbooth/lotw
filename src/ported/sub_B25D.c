/* $B25D (bank13) — advance $0A to the next multiple of 8, queuing a sprite
 * group (sub_B278) for each intermediate step.
 *   loop: INC $0A; if ($0A & 7)==0 break; A=$FF; sub_B278(); repeat (JMP self)
 *   then if $0A == $F0 wrap it back to 0.
 * Net RAM: $0A advanced; $1E and $28=0 from sub_B278's PPU jobs. */
#include "ram.h"
#include "regs.h"

void sub_B278(Regs *r);

void sub_B25D(Regs *r)
{
    for (;;) {
        RAM8(0x0A)++;                       /* INC $0A */
        if ((RAM8(0x0A) & 0x07) == 0)       /* LDA $0A / AND #$07 / BEQ L_B26D */
            break;
        r->a = 0xFF;                        /* LDA #$FF */
        sub_B278(r);                        /* JSR L_B278 */
    }                                       /* JMP L_B25D */

    if (RAM8(0x0A) == 0xF0)                 /* L_B26D: LDA $0A / CMP #$F0 / BNE */
        RAM8(0x0A) = 0x00;                  /* LDA #$00 / STA $0A */
}                                           /* L_B277: RTS */
