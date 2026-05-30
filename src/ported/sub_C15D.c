/* $C15D: horizontal scroll follow.
 *   $08 = (scroll_x_tile<<4)|scroll_x_fine
 *   A   = (player_x_tile<<4)|player_x_fine ; SEC ; SBC $08   (player - scroll)
 *   CMP #$60 / BCC L_C19D                  ; player close to left  -> scroll left?
 *   CMP #$91 / BCC L_C1C2                   ; mid window -> no scroll, return C=1
 *   ; player far right:
 *   LDA scroll_x_tile / CMP #$30 / BCS L_C192
 *     scroll_x_tile = player_x_tile - 9 ; scroll_x_fine = player_x_fine ; $7F=$01 ; ->C1BD
 *   L_C192: scroll_x_tile=$30 ; scroll_x_fine=0 ; -> C1C2
 *   L_C19D: A = scroll_x_tile | scroll_x_fine ; BEQ L_C1C2   ; already at left edge
 *     player_x_tile - 6 ; BCC L_C1B7
 *       scroll_x_tile = result ; scroll_x_fine = player_x_fine ; $7F=$FF ; -> C1BD
 *     L_C1B7: scroll_x_fine=0 ; scroll_x_tile=0 ; (falls into C1BD)
 *   L_C1BD: JSR L_C1C7 ; CLC ; RTS     (carry clear)
 *   L_C1C2: JSR L_C1C7 ; SEC ; RTS     (carry set) */
#include "ram.h"
#include "regs.h"

void sub_C1C7(Regs *r);

#define scroll_x_fine RAM8(0x7B)
#define scroll_x_tile RAM8(0x7C)
#define player_x_fine RAM8(0x43)
#define player_x_tile RAM8(0x44)

void sub_C15D(Regs *r)
{
    u8 scrollpos = (u8)((scroll_x_tile << 4) | scroll_x_fine);
    u8 playerpos = (u8)((player_x_tile << 4) | player_x_fine);
    u8 delta = (u8)(playerpos - scrollpos);   /* SEC ; SBC */
    int out_carry;                            /* set per exit (CLC=0, SEC=1) */

    RAM8(0x08) = scrollpos;

    if (delta < 0x60) {
        /* L_C19D */
        if ((scroll_x_tile | scroll_x_fine) == 0) {
            out_carry = 1;                    /* L_C1C2 */
        } else {
            u8 t = (u8)(player_x_tile - 0x06);
            if (player_x_tile < 0x06) {       /* SBC #$06 borrow -> BCC L_C1B7 */
                scroll_x_fine = 0x00;
                scroll_x_tile = 0x00;
                out_carry = 0;                /* falls into L_C1BD */
            } else {
                scroll_x_tile = t;
                scroll_x_fine = player_x_fine;
                RAM8(0x7F) = 0xFF;
                out_carry = 0;                /* L_C1BD */
            }
        }
    } else if (delta < 0x91) {
        out_carry = 1;                        /* L_C1C2 */
    } else {
        /* player far right */
        if (scroll_x_tile >= 0x30) {
            /* L_C192 */
            scroll_x_tile = 0x30;
            scroll_x_fine = 0x00;
            out_carry = 1;                    /* L_C1C2 */
        } else {
            scroll_x_tile = (u8)(player_x_tile - 0x09);
            scroll_x_fine = player_x_fine;
            RAM8(0x7F) = 0x01;
            out_carry = 0;                    /* L_C1BD */
        }
    }

    sub_C1C7(r);                              /* updates $1C/$1D, r->a/r->x */
    r->c = (u8)out_carry;
}
