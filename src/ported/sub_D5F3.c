/* $D5F3 — tile-$03 "key-door" handler. Reached as a non-local PLA/PLA + JMP
 * tail-transfer out of sub_DCE2's key-door check (see src/ported/sub_DCE2.c
 * hit_D5F3): once the equipped/held-item gate passes, the grandparent frame is
 * discarded and control lands here to teleport the player to the destination
 * stored in the active object/door record pointed to by ($77).
 *
 * Using ($77) it reads the destination fields at offsets +$0C..+$0F:
 *   +$0C -> map_screen_x   ($47)
 *   +$0D -> map_screen_y   ($48)
 *   +$0E -> player_x_tile  ($44); also derives scroll_x_tile: (tile-8)
 *           clamped to [0,$30] (BCS guard floors at 0; CMP #$31/BCC clamps high
 *           to $30).
 *   +$0F -> player_y       ($45)
 * player_x_fine ($43) and scroll_x_fine ($7B) are zeroed.
 * Then JMP L_D895 (the shared "rebuild scene & commit" tail, inlined here as in
 * src/ported/sub_D620.c): C3E5 / D08A / scene_assemble / C5CB / D07C / C1C7 /
 * C1D8 / C492, returning with carry set.
 *
 * INSPECTION-PORT (no diff-test spec): the entry is the *target* of a non-local
 * PLA/PLA+JMP, so its own caller frame is the grandparent's; the flat Regs ABI
 * cannot model that stack surgery. The body's data effects are faithful and
 * whole-ROM integration verifies the transfer. No far-calls. */
#include "ram.h"
#include "regs.h"

void sub_C3E5(Regs *r);
void sub_D08A(Regs *r);
void scene_assemble(Regs *r);
void sub_C5CB(Regs *r);
void sub_D07C(Regs *r);
void sub_C1C7(Regs *r);
void sub_C1D8(Regs *r);
void sub_C492(Regs *r);

void sub_D5F3(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));  /* ($77) record pointer */
    u8 a;

    r->y = 0x0C;                                /* LDY #$0C */
    RAM8(0x47) = RAM8((u16)(ptr + r->y));       /* LDA ($77),Y / STA map_screen_x */
    r->y++;                                     /* INY -> $0D */
    RAM8(0x48) = RAM8((u16)(ptr + r->y));       /* LDA ($77),Y / STA map_screen_y */
    r->y++;                                     /* INY -> $0E */
    a = RAM8((u16)(ptr + r->y));                /* LDA ($77),Y */
    RAM8(0x44) = a;                             /* STA player_x_tile */

    /* SEC / SBC #$08 ; BCS keeps result, else floor to 0 */
    if (a >= 0x08)                              /* (carry set after SBC == a>=8) */
        a = (u8)(a - 0x08);
    else
        a = 0x00;                               /* L_D60A path: LDA #$00 */

    /* CMP #$31 / BCC keep ; else LDA #$30 (clamp high) */
    if (a >= 0x31)
        a = 0x30;
    RAM8(0x7C) = a;                             /* STA scroll_x_tile */

    RAM8(0x43) = 0x00;                          /* LDA #$00 / STA player_x_fine */
    RAM8(0x7B) = 0x00;                          /* STA scroll_x_fine */

    r->y++;                                     /* INY -> $0F */
    r->a = RAM8((u16)(ptr + r->y));             /* LDA ($77),Y */
    RAM8(0x45) = r->a;                          /* STA player_y (A = player_y on fall-through) */

    /* JMP L_D895 — shared scene rebuild + commit tail (inlined). */
    sub_C3E5(r);
    sub_D08A(r);
    scene_assemble(r);
    sub_C5CB(r);
    sub_D07C(r);
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    r->c = 1;                                   /* SEC / RTS */
}
