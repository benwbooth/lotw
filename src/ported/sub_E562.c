/* $E562 (bankfix) — interactive vertical-move loop (variant of $E514).
 * Each frame: $36=1; read controllers; if button bit7 set -> CLC,RTS (cancel).
 * Else move by the direction nibble (sub_CD2C, Y=1), recompute (sub_D8B6).
 * If trial tile-Y ($0A) >= $A1 -> SEC,RTS. If it is in [$20,$A1) and the trial
 * tile-low ($0F&$0F) >= $01 and ( <$0F or $0E==0 ), commit the trial position
 * to player_{x_fine=$43, x_tile=$44, y=$45}. Then redraw and loop.
 *
 * INSPECTION-PORT (no diff-test spec): the loop never terminates under flat host
 * memory ($20=0 always), so every state is skipped — see $E514. Integration-verified. */
#include "ram.h"
#include "regs.h"

void read_controllers(Regs *r);
void sub_CD2C(Regs *r);
void sub_D8B6(Regs *r);
void sub_D8E3(Regs *r);
void sub_D94E(Regs *r);
void sub_C1D8(Regs *r);
void sub_C135(Regs *r);

void sub_E562(Regs *r)
{
    for (;;) {
        RAM8(0x36) = 0x01;                  /* LDA #$01 / STA $36 */
        read_controllers(r);                /* JSR read_controllers -> $20 */
        if (RAM8(0x20) & 0x80) {            /* LDA $20 / AND #$80 / BNE L_E5B0 */
            r->a = 0x80; r->c = 0;          /* L_E5B0: CLC / RTS */
            return;
        }
        r->a = RAM8(0x20) & 0x0F;           /* LDA $20 / AND #$0F */
        r->y = 0x01;                        /* LDY #$01 */
        sub_CD2C(r);                        /* JSR L_CD2C */
        sub_D8B6(r);                        /* JSR L_D8B6 */

        u8 ty = RAM8(0x0A);
        if (ty >= 0xA1) {                   /* CMP #$A1 / BCS L_E5B2 */
            r->a = ty; r->c = 1;            /* L_E5B2: SEC / RTS */
            return;
        }
        if (ty >= 0x20) {                   /* CMP #$20 / BCC L_E5A1 (skip store) */
            u8 lo = RAM8(0x0F) & 0x0F;       /* LDA $0F / AND #$0F */
            int store = 0;
            if (lo >= 0x01) {               /* CMP #$01 / BCC L_E5A1 */
                if (lo < 0x0F) store = 1;    /* CMP #$0F / BCC L_E595 */
                else if (RAM8(0x0E) == 0)    /* LDA $0E / BNE L_E5A1 */
                    store = 1;               /* else fall through to L_E595 */
            }
            if (store) {                    /* L_E595: */
                RAM8(0x43) = RAM8(0x0E);     /* STA player_x_fine */
                RAM8(0x44) = RAM8(0x0F);     /* STA player_x_tile */
                RAM8(0x45) = RAM8(0x0A);     /* STA player_y */
            }
        }
        sub_D8E3(r);                        /* L_E5A1: */
        sub_D94E(r);
        sub_C1D8(r);
        sub_C135(r);
    }                                       /* JMP L_E562 */
}
