/* $D641 — reset / game-over state entry. Reached via JMP from game_update
 * (see src/ported/game_update.c, the reset path) as a whole-frame handoff.
 *
 * Body:
 *   LDA #$00 / STA $EB            ; clear $EB (reset/transition flag)
 *   JSR L_D67D                    ; tear down & rebuild the screen scaffold
 *   LDA #$3E / STA mmc3_r4_shadow ; ($2E) program MMC3 R4 bank shadow to $3E
 *   JMP L_D866                    ; -> place player at the fixed reset spawn
 *
 * L_D866 (inlined tail, shares the L_D895 commit with sub_D620/sub_D5F3):
 *   map_screen_y=$10, map_screen_x=$03, scroll_x_tile=$12, player_y=$B0,
 *   player_x_tile=$1A, player_x_fine=0, scroll_x_fine=0; then JMP L_D895:
 *   C3E5 / D08A / scene_assemble / C5CB / D07C / C1C7 / C1D8 / C492, SEC / RTS
 *   (returns with carry set).
 *
 * INSPECTION-PORT (no diff-test spec): reached by a non-local JMP and itself
 * tail-jumps through L_D866->L_D895; the data effects are faithful and whole-ROM
 * integration verifies the handoff. No far-calls (sub_D67D and the L_D895 tail
 * stay in this bank). */
#include "ram.h"
#include "regs.h"

void sub_D67D(Regs *r);
void sub_C3E5(Regs *r);
void sub_D08A(Regs *r);
void scene_assemble(Regs *r);
void sub_C5CB(Regs *r);
void sub_D07C(Regs *r);
void sub_C1C7(Regs *r);
void sub_C1D8(Regs *r);
void sub_C492(Regs *r);

void sub_D641(Regs *r)
{
    RAM8(0xEB) = 0x00;                 /* LDA #$00 / STA $EB */

    sub_D67D(r);                       /* JSR L_D67D */

    RAM8(0x2E) = 0x3E;                 /* LDA #$3E / STA mmc3_r4_shadow */

    /* JMP L_D866 — fixed reset spawn placement (inlined). */
    RAM8(0x48) = 0x10;                 /* map_screen_y */
    RAM8(0x47) = 0x03;                 /* map_screen_x */
    RAM8(0x7C) = 0x12;                 /* scroll_x_tile */
    RAM8(0x45) = 0xB0;                 /* player_y */
    RAM8(0x44) = 0x1A;                 /* player_x_tile */
    RAM8(0x43) = 0x00;                 /* player_x_fine */
    RAM8(0x7B) = 0x00;                 /* scroll_x_fine */
    r->a = 0x00;                       /* A holds the last #$00 store on fall-through */

    /* JMP L_D895 — shared scene rebuild + commit tail (inlined). */
    sub_C3E5(r);
    sub_D08A(r);
    scene_assemble(r);
    sub_C5CB(r);
    sub_D07C(r);
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    r->c = 1;                          /* SEC / RTS */
}
