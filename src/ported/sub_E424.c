/* $E424 (bankfix) — "shop / buy" game-state handler (tile-$04 destination).
 * Reached via a non-local JMP from sub_DCE2 (the player stepped onto a tile
 * whose low-6-bit id is $04). Drives the purchase counter screen.
 *
 * Setup: room init (E620); preserve the four shop-slot bytes $80..$83 across a
 * screen/start-index load (LDA map_screen_x / JSR E660) by stacking and
 * restoring them; then E6FF (object refresh), CFBC, E778, C492 (commit/redraw).
 *
 * Main loop (L_E450): run the interactive horizontal-move loop sub_E514 (returns
 * carry). On C=1 (exit) -> non-local JMP L_E5FD (leave-shop handler). Otherwise
 * map the player's column to a slot index X from the low nibble of player_x_tile:
 *   nibble in {3,4} -> X=0 ; nibble in {$0A,$0B} -> X=2 ; anything else -> reloop.
 * At the chosen slot:
 *   - empty slot ($80,X negative) -> just set pacing $8F=$06.
 *   - else try to buy: subtract the slot's price ($81,X) from gold via E842.
 *       C=1 (paid): clear the slot ($80,X=$FF), E6FF, add item to
 *                   inventory_counts[item], pacing $8F=$10.
 *       C=0 (too poor): if the item id is $0D and the "$37" flag is set, raise
 *                   the $61 request flag; pacing $8F=$06.
 * Then wait for button release (faithful read_controllers loop) and reloop.
 *
 * NEW NON-LOCAL JMP TARGET: L_E5FD (not yet ported) — the BCS-exit from E514
 * tail-jumps there via L_E4A7. Documented + stubbed; no data effects here.
 *
 * INSPECTION-PORT (no diff-test spec): the L_E49F read_controllers release-wait
 * is a faithful while-loop (yields $20=0 in flat host memory), and the loop only
 * leaves via E514's exit -> L_E5FD non-local JMP. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_E5FD(Regs *r);
void sub_E620(Regs *r); void sub_E660(Regs *r); void sub_E6FF(Regs *r);
void sub_CFBC(Regs *r); void sub_E778(Regs *r); void sub_C492(Regs *r);
void sub_E514(Regs *r); void sub_E842(Regs *r); void read_controllers(Regs *r);
/* NEW non-local JMP target (PLA-free JMP via L_E4A7). Not yet ported — see header.
 * void sub_E5FD(Regs *r);   (leave-shop / exit handler) */

void sub_E424(Regs *r)
{
    u8 x, nib, item, pr;

    sub_E620(r);                             /* JSR L_E620 */

    /* Preserve $80..$83 across the E660 screen/index load. */
    {
        u8 s80 = RAM8(0x80), s81 = RAM8(0x81);
        u8 s82 = RAM8(0x82), s83 = RAM8(0x83);
        r->a = RAM8(0x47) /*map_screen_x*/;  /* LDA map_screen_x */
        sub_E660(r);                         /* JSR L_E660 */
        RAM8(0x83) = s83; RAM8(0x82) = s82;
        RAM8(0x81) = s81; RAM8(0x80) = s80;
    }

    sub_E6FF(r);                             /* JSR L_E6FF */
    sub_CFBC(r);                             /* JSR L_CFBC */
    sub_E778(r);                             /* JSR L_E778 */
    sub_C492(r);                             /* JSR L_C492 */

    for (;;) {                               /* L_E450 */
        sub_E514(r);                         /* JSR L_E514 (returns carry) */
        if (r->c) {                          /* BCS L_E4A7 -> JMP L_E5FD */
            sub_E5FD(r);                     /* leave-shop -> level-resume tail */
            return;
        }

        /* Map player column to a slot index X (else reloop). */
        nib = (u8)(RAM8(0x44) /*player_x_tile*/ & 0x0F);  /* AND #$0F */
        if (nib < 0x03) continue;            /* CMP #$03 / BCC L_E450 */
        if (nib < 0x05) {                    /* CMP #$05 / BCC L_E46D */
            x = 0x00;
        } else {
            x = 0x02;                        /* LDX #$02 */
            if (nib < 0x0A) continue;        /* CMP #$0A / BCC L_E450 */
            if (nib >= 0x0C) continue;       /* CMP #$0C / BCS L_E450 */
        }

        /* L_E46D */
        item = RAM8((u16)(0x80 + x));        /* LDA $80,X */
        if (item & 0x80) {                   /* BMI L_E489 (empty slot) */
            RAM8(0x8F) = 0x06;               /* L_E489: LDA #$06 (-> L_E49D STA $8F) */
        } else {
             pr = RAM8((u16)(0x81 + x));      /* LDA $81,X (price) */
            r->a = pr;
            sub_E842(r);                     /* JSR L_E842 (gold -= A, C=1 if paid) */
            if (r->c) {                      /* BCS L_E48E (purchase succeeded) */
                /* L_E48E */
                RAM8((u16)(0x80 + x)) = 0xFF;       /* LDA #$FF / STA $80,X */
                sub_E6FF(r);                        /* JSR L_E6FF */
                RAM8((u16)(0x60 /*inventory_counts*/ + item))++;  /* INC inventory_counts,X(=item) */
                RAM8(0x8F) = 0x10;                  /* LDA #$10 (-> L_E49D) */
            } else {
                /* C=0: too poor. CMP #$0D / BNE L_E489 ; LDA $37 / BEQ L_E489 */
                if (item == 0x0D && RAM8(0x37) != 0)
                    RAM8(0x61) = 0x01;       /* LDA #$01 / STA $61 */
                RAM8(0x8F) = 0x06;           /* L_E489: LDA #$06 (-> L_E49D) */
            }
        }

        /* L_E49F: wait for button release (faithful loop) */
        do { read_controllers(r); } while (RAM8(0x20) != 0);
        /* JMP L_E450 */
    }
}
