/* $E660:
 *   PHA / JSR C3E5 / JMP E66B
 * (shared body E66B with $E667; see below). Entry A = screen/start index.
 */
#include "ram.h"
#include "regs.h"

void sub_C3E5(Regs *r);
void sub_D07C(Regs *r);
void sub_C8FF(Regs *r);
void sub_C5DC(Regs *r);
void sub_D8E3(Regs *r);
void sub_C1D8(Regs *r);
void sub_C1C7(Regs *r);

void sub_E660(Regs *r)
{
    u8 a = r->a;          /* PHA (saved) */

    sub_C3E5(r);

    /* E66B: PLA / PHA / STA $08 (a still our saved value) */
    RAM8(0x08) = a;
    RAM8(0x47) = (u8)((a & 0x0C) >> 2);          /* map_screen_x */
    RAM8(0x7C) = (u8)((a & 0x03) << 4);          /* scroll_x_tile */
    RAM8(0x44) = (u8)(RAM8(0x7C) + 0x07);        /* player_x_tile */
    RAM8(0x48) = 0x10;                           /* map_screen_y */
    RAM8(0x43) = 0x08;                           /* player_x_fine */
    RAM8(0x45) = 0xA0;                           /* player_y */
    RAM8(0x4F) = 0x00;
    RAM8(0x4E) = 0x00;
    RAM8(0x7B) = 0x00;                           /* scroll_x_fine */

    sub_D07C(r);
    sub_C8FF(r);

    /* PLA / CMP #$04 */
    if (a == 0x04)
        RAM8(0x7A) = (u8)(0x1F + 0xA0);          /* $7A = $BF */

    sub_C5DC(r);
    sub_D8E3(r);
    sub_C1D8(r);
    sub_C1C7(r);
}
