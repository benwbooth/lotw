/* $E4AA — interactive "swap carried item" menu loop.
 * Each round: run the cursor-move loop (sub_E562). If it returns carry set
 * (player pressed cancel), finalize: if the equipped slot holds item $0D, reset
 * equipped_item to $03 and refresh ($C234); RTS. Otherwise pick the inventory
 * slot under the cursor (index = (player_x_tile>>1) | bank, bank from player_y),
 * and if that slot is non-empty try to use it (sub_D017); on success decrement
 * its count and rotate it into the 3-deep carried_item ring (returning the
 * displaced item to inventory), refresh the HUD ($E6B7/$C234/$CF30/$CF82), and
 * loop. Empty/failed selections just set the $8F prompt and loop.
 *
 * INSPECTION-PORT (no diff-test spec): calls sub_E562, an interactive loop that
 * never terminates under flat host memory (read_controllers yields $20=0), so
 * every state is skipped by the watchdog/oracle — see sub_E562. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_E562(Regs *r);
void sub_D017(Regs *r);
void sub_E6B7(Regs *r);
void sub_C234(Regs *r);
void sub_CF30(Regs *r);
void sub_CF82(Regs *r);

void sub_E4AA(Regs *r)
{
    for (;;) {
        u8 x, py;

        sub_E562(r);                        /* JSR L_E562 (returns carry) */
        if (r->c) {                         /* BCS L_E504 */
            u8 e = RAM8(0x55);              /* LDX equipped_item */
            if (RAM8((u16)(0x51 + e)) == 0x0D) {  /* LDA carried_item0,X / CMP #$0D / BNE */
                RAM8(0x55) = 0x03;          /* STA equipped_item */
                sub_C234(r);
            }
            return;                         /* L_E513: RTS */
        }

        x = 0xFF;                           /* LDX #$FF */
        py = RAM8(0x45);                    /* LDA player_y */
        if (py >= 0x58)                     /* CMP #$58 / BCS L_E4DD */
            goto L_E4DD;
        x = (py < 0x38) ? 0x00 : 0x08;      /* LDX #$00 / CMP #$38 / BCC L_E4BF / LDX #$08 */

        /* L_E4BF */
        RAM8(0x08) = x;                     /* STX $08 (inventory bank) */
        x = (u8)((RAM8(0x44) >> 1) | RAM8(0x08));  /* player_x_tile>>1 | $08 -> X */
        if (RAM8((u16)(0x60 + x)) != 0) {   /* LDA inventory_counts,X / BEQ L_E4D4 */
            r->a = x;                       /* TXA / PHA */
            sub_D017(r);                    /* JSR L_D017 (PLA/TAX restores X = our x) */
            if (r->c) {                     /* BCS L_E4DB */
                RAM8((u16)(0x60 + x))--;    /* L_E4DB: DEC inventory_counts,X */
                goto L_E4DD;
            }
        }
        /* L_E4D4 */
        RAM8(0x8F) = 0x06;
        continue;                           /* JMP L_E4AA */

    L_E4DD:
        RAM8(0x08) = x;                     /* STX $08 (selected item type) */
        {
            u8 ci0 = RAM8(0x51);            /* LDX carried_item0 */
            if (!(ci0 & 0x80))              /* BMI L_E4E5 (skip if no item) */
                RAM8((u16)(0x60 + ci0))++;  /* INC inventory_counts,X */
        }
        /* L_E4E5 — rotate the 3-deep carried ring, push selection in */
        RAM8(0x51) = RAM8(0x52);            /* carried_item1 -> carried_item0 */
        RAM8(0x52) = RAM8(0x53);            /* carried_item2 -> carried_item1 */
        RAM8(0x53) = RAM8(0x08);            /* $08 -> carried_item2 */
        RAM8(0x8F) = 0x12;
        sub_E6B7(r);
        sub_C234(r);
        sub_CF30(r);
        sub_CF82(r);
        /* JMP L_E4AA */
    }
}
