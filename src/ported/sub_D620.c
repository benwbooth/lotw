/* $D620:
 *   JSR L_D67D
 *   LDA #$11 / STA $48 (map_screen_y)
 *   LDX $6E / DEX / STX $47 (map_screen_x)
 *   LDA #$12 / STA $7C (scroll_x_tile)
 *   LDA #$10 / STA $45 (player_y)
 *   LDA #$1A / STA $44 (player_x_tile)
 *   LDA #$00 / STA $43 (player_x_fine) / STA $7B (scroll_x_fine)
 *   JMP L_D895
 * L_D895 (inlined tail-call):
 *   JSR sub_C3E5 / JSR sub_D08A / JSR scene_assemble / JSR sub_C5CB /
 *   JSR sub_D07C / JSR sub_C1C7 / JSR sub_C1D8($C1D8) / JSR sub_C492 /
 *   SEC / RTS  ->  carry set on return.
 */
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

void sub_D620(Regs *r)
{
    sub_D67D(r);

    RAM8(0x48) = 0x11;                 /* map_screen_y */
    r->x = (u8)(RAM8(0x6E) - 1);
    RAM8(0x47) = r->x;                 /* map_screen_x */
    RAM8(0x7C) = 0x12;                 /* scroll_x_tile */
    RAM8(0x45) = 0x10;                 /* player_y */
    RAM8(0x44) = 0x1A;                 /* player_x_tile */
    RAM8(0x43) = 0x00;                 /* player_x_fine */
    RAM8(0x7B) = 0x00;                 /* scroll_x_fine */
    r->a = 0x00;

    /* JMP L_D895 */
    sub_C3E5(r);
    sub_D08A(r);
    scene_assemble(r);
    sub_C5CB(r);
    sub_D07C(r);
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    r->c = 1;
}
