/* $E00F (bankfix) — screen-transition / scroll handler. Reached via JMP from
 * game_update when the "screen-transition" input bit ($20 & $10) is held.
 *
 * Sets the PPU-job pacing byte $8F=$03 and bumps the in-transition depth $8D.
 * If the current room bank (mmc3_r3_shadow) is < $30 (i.e. a non-status screen
 * that needs a full rebuild), it tears down and rebuilds the scene: room init
 * (E620), screen/start index #$08 (E660), object reload (E6B7), CF30/CF82
 * sprite work, sets scroll_x_fine=$08, then C1C7/C1D8/C492 commit/redraw.
 *
 * Then it runs three faithful read_controllers wait-loops:
 *   L_E039: spin until all buttons released ($20==0),
 *   L_E03E: spin until the #$10 (transition) button is pressed,
 *   L_E045: spin again until all buttons released.
 * After the handshake it sets $8F=$04 and, again only if mmc3_r3_shadow < $30,
 * runs the post-transition rebuild: E642, C3E5, E79D, then re-arm the music for
 * the new area (LDA $FE / D02E), C8FF, C5CB, C1D8, C2B1, C1C7, C492.
 * Finally DEC $8D (leave transition) and RTS.
 *
 * INSPECTION-PORT (no diff-test spec): the three read_controllers spin-loops are
 * translated faithfully as while-loops; in flat host memory read_controllers
 * yields $20=0, so they are not isolation-testable. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_E620(Regs *r); void sub_E660(Regs *r); void sub_E6B7(Regs *r);
void sub_CF30(Regs *r); void sub_CF82(Regs *r); void sub_C1C7(Regs *r);
void sub_C1D8(Regs *r); void sub_C492(Regs *r); void read_controllers(Regs *r);
void sub_E642(Regs *r); void sub_C3E5(Regs *r); void sub_E79D(Regs *r);
void sub_D02E(Regs *r); void sub_C8FF(Regs *r); void sub_C5CB(Regs *r);
void sub_C2B1(Regs *r);

void sub_E00F(Regs *r)
{
    RAM8(0x8F) = 0x03;                       /* LDA #$03 / STA $8F */
    RAM8(0x8D)++;                            /* INC $8D */

    if (RAM8(0x2D) /*mmc3_r3_shadow*/ < 0x30) {   /* CMP #$30 / BCS L_E039 */
        sub_E620(r);                         /* JSR L_E620 */
        r->a = 0x08;                         /* LDA #$08 */
        sub_E660(r);                         /* JSR L_E660 */
        sub_E6B7(r);                         /* JSR L_E6B7 */
        sub_CF30(r);                         /* JSR L_CF30 */
        sub_CF82(r);                         /* JSR L_CF82 */
        RAM8(0x7B) /*scroll_x_fine*/ = 0x08; /* LDA #$08 / STA scroll_x_fine */
        sub_C1C7(r);                         /* JSR L_C1C7 */
        sub_C1D8(r);                         /* JSR L_C1D8 */
        sub_C492(r);                         /* JSR L_C492 */
    }

    /* L_E039: spin until all buttons released */
    do { read_controllers(r); } while (RAM8(0x20) != 0);
    /* L_E03E: spin until #$10 (transition) button pressed */
    do { read_controllers(r); } while ((RAM8(0x20) & 0x10) == 0);
    /* L_E045: spin until all buttons released */
    do { read_controllers(r); } while (RAM8(0x20) != 0);

    RAM8(0x8F) = 0x04;                       /* LDA #$04 / STA $8F */

    if (RAM8(0x2D) /*mmc3_r3_shadow*/ < 0x30) {   /* CMP #$30 / BCS L_E074 */
        sub_E642(r);                         /* JSR L_E642 */
        sub_C3E5(r);                         /* JSR L_C3E5 */
        sub_E79D(r);                         /* JSR L_E79D */
        r->a = RAM8(0xFE);                   /* LDA $FE */
        sub_D02E(r);                         /* JSR L_D02E (re-arm music) */
        sub_C8FF(r);                         /* JSR L_C8FF */
        sub_C5CB(r);                         /* JSR L_C5CB */
        sub_C1D8(r);                         /* JSR L_C1D8 */
        sub_C2B1(r);                         /* JSR L_C2B1 */
        sub_C1C7(r);                         /* JSR L_C1C7 */
        sub_C492(r);                         /* JSR L_C492 */
    }

    /* L_E074 */
    RAM8(0x8D)--;                            /* DEC $8D */
}
