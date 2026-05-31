/* $E514 (bankfix) — interactive vertical-move loop (climb-up style).
 * Each frame: $36=1; read controllers; if button bit7 set -> CLC,RTS (cancel).
 * Else move by the direction nibble (sub_CD2C, Y=1), recompute trial position
 * (sub_D8B6). If the trial tile-Y ($0A) >= $A1 -> SEC,RTS (blocked). If it is
 * in [$8C,$A1) and the trial tile-low ($0F&$0F) is in [$02,$0D), commit the
 * trial position to player_{x_fine=$43, x_tile=$44, y=$45}. Then redraw
 * (D8E3/D94E/C1D8/C135) and loop (JMP self).
 *
 * INSPECTION-PORT (no diff-test spec): read_controllers yields $20=0 under flat
 * host memory ($4016 strobe reads 0), so the button-exit is never taken and the
 * loop never terminates — both the host harness (watchdog) and the m6502 oracle
 * (step limit) skip every state. Verified by whole-ROM integration instead. */
#include "ram.h"
#include "regs.h"

void read_controllers(Regs *r);
void sub_CD2C(Regs *r);
void sub_D8B6(Regs *r);
void sub_D8E3(Regs *r);
void sub_D94E(Regs *r);
void sub_C1D8(Regs *r);
void sub_C135(Regs *r);

void sub_E514(Regs *r)
{
    for (;;) {
        RAM8(0x36) = 0x01;                  /* LDA #$01 / STA $36 */
        read_controllers(r);                /* JSR read_controllers -> $20 */
        if (RAM8(0x20) & 0x80) {            /* LDA $20 / AND #$80 / BNE L_E55E */
            r->a = 0x80; r->c = 0;          /* L_E55E: CLC / RTS */
            return;
        }
        r->a = RAM8(0x20) & 0x0F;           /* LDA $20 / AND #$0F */
        r->y = 0x01;                        /* LDY #$01 */
        sub_CD2C(r);                        /* JSR L_CD2C */
        sub_D8B6(r);                        /* JSR L_D8B6 */

        u8 ty = RAM8(0x0A);
        if (ty >= 0xA1) {                   /* (ty>=$8C then) CMP #$A1 / BCS L_E560 */
            r->a = ty; r->c = 1;            /* L_E560: SEC / RTS */
            return;
        }
        if (ty >= 0x8C) {                   /* CMP #$8C / BCC L_E54F (skip store) */
            u8 lo = RAM8(0x0F) & 0x0F;       /* LDA $0F / AND #$0F */
            if (lo >= 0x02 && lo < 0x0D) {   /* CMP #$02 BCC / CMP #$0D BCS -> skip */
                RAM8(0x43) = RAM8(0x0E);     /* STA player_x_fine */
                RAM8(0x44) = RAM8(0x0F);     /* STA player_x_tile */
                RAM8(0x45) = RAM8(0x0A);     /* STA player_y */
            }
        }
        sub_D8E3(r);                        /* L_E54F: */
        sub_D94E(r);
        sub_C1D8(r);
        sub_C135(r);
    }                                       /* JMP L_E514 */
}
