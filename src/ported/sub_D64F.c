/* $D64F — secret-warp trigger check. If the equipped item is $0F and the player
 * stands at map (1,5), scroll ($10,$00), player_y=$A0, set the warp flag $EC=1
 * and DISCARD the caller's return frame (PLA/PLA — non-local return to the
 * grandparent). Otherwise RTS.
 * INSPECTION-PORT: the PLA/PLA non-local return isn't modelled; only $EC is RAM. */
#include "ram.h"
#include "regs.h"
void sub_D64F(Regs *r)
{
    u8 x = RAM8(0x55);                              /* equipped_item */
    if (RAM8((u16)(0x51 + x)) == 0x0F &&            /* carried_item0,X == $0F */
        RAM8(0x47) == 0x01 && RAM8(0x48) == 0x05 && /* map_screen_x/y */
        RAM8(0x7C) == 0x10 && RAM8(0x7B) == 0x00 && /* scroll_x_tile/fine */
        RAM8(0x45) == 0xA0) {                       /* player_y */
        RAM8(0xEC) = 0x01;                          /* L_D676 */
        /* PLA / PLA — non-local return (see header) */
    }
}
