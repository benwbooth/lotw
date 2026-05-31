/* $E5B4 (bankfix) — interactive vertical-move loop (variant of $E514, no
 * D8E3/D94E redraw, single exit). Each frame: $36=1; read controllers; if
 * button bit7 set -> RTS. Else move by the direction nibble (sub_CD2C, Y=1),
 * recompute (sub_D8B6). If trial tile-Y ($0A) is in [$30,$A1) and the trial
 * tile-low ($0F&$0F) >= $02 and ( <$0D or $0E==0 ), commit the trial position
 * to player_{x_fine=$43, x_tile=$44, y=$45}. Then redraw (C1D8/C135) and loop.
 * ($0A>=$A1 just skips the store and keeps looping — no SEC exit here.)
 *
 * INSPECTION-PORT (no diff-test spec): the loop never terminates under flat host
 * memory ($20=0 always), so every state is skipped — see $E514. Integration-verified. */
#include "ram.h"
#include "regs.h"

void read_controllers(Regs *r);
void sub_CD2C(Regs *r);
void sub_D8B6(Regs *r);
void sub_C1D8(Regs *r);
void sub_C135(Regs *r);

void sub_E5B4(Regs *r)
{
    for (;;) {
        RAM8(0x36) = 0x01;                  /* LDA #$01 / STA $36 */
        read_controllers(r);                /* JSR read_controllers -> $20 */
        if (RAM8(0x20) & 0x80) {            /* LDA $20 / AND #$80 / BNE L_E5FC */
            r->a = 0x80;                     /* L_E5FC: RTS (carry untouched) */
            return;
        }
        r->a = RAM8(0x20) & 0x0F;           /* LDA $20 / AND #$0F */
        r->y = 0x01;                        /* LDY #$01 */
        sub_CD2C(r);                        /* JSR L_CD2C */
        sub_D8B6(r);                        /* JSR L_D8B6 */

        u8 ty = RAM8(0x0A);
        if (ty >= 0x30 && ty < 0xA1) {      /* CMP #$30 BCC / CMP #$A1 BCS -> L_E5F3 */
            u8 lo = RAM8(0x0F) & 0x0F;       /* LDA $0F / AND #$0F */
            if (lo >= 0x02) {               /* CMP #$02 / BCC L_E5F3 */
                int store = (lo < 0x0D)      /* CMP #$0D / BCC L_E5E7 */
                            || (RAM8(0x0E) == 0); /* LDA $0E / BNE L_E5F3 */
                if (store) {                /* L_E5E7: */
                    RAM8(0x43) = RAM8(0x0E); /* STA player_x_fine */
                    RAM8(0x44) = RAM8(0x0F); /* STA player_x_tile */
                    RAM8(0x45) = RAM8(0x0A); /* STA player_y */
                }
            }
        }
        sub_C1D8(r);                        /* L_E5F3: */
        sub_C135(r);
    }                                       /* JMP L_E5B4 */
}
