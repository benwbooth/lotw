/* $D596: "use/consume equipped item" dispatch.
 *   Y = equipped_item ($55); X = carried_item0[Y] ($51+Y) = the item id.
 *   X < 2 (sword/shield-ish): if $86+X != 0 already busy -> RTS; else try
 *     consume magic via E7F0. C=0 (had magic) -> $86+X = 2. C=1 (no magic) ->
 *     if $37 != 0 and $37 not negative: set $37=$FD, $8F=$1A.
 *   X == $0B (potion?): if magic != 0 RTS; else clear slot (carried[equipped]=$FF),
 *     rebuild inventory sprites (C234), refill magic bar (D199).
 *   X == $0D (ladder/warp?): if map_screen_y >= $11 just set equipped_item=3;
 *     else clear slot, C234, $8F=$12, then JMP D866 -> D895 screen rebuild
 *     (C3E5,D08A,scene_assemble,C5CB,D07C,C1C7,C1D8,C492; sets the warp coords).
 *   Otherwise RTS.
 * The JMP D866 tail is inlined here (D866 sets fixed warp coords, falls into D895). */
#include "ram.h"
#include "regs.h"

#define equipped_item RAM8(0x0055)
#define carried_item0 0x0051
#define map_screen_x  RAM8(0x0047)
#define map_screen_y  RAM8(0x0048)
#define scroll_x_tile RAM8(0x007C)
#define scroll_x_fine RAM8(0x007B)
#define player_x_tile RAM8(0x0044)
#define player_x_fine RAM8(0x0043)
#define player_y      RAM8(0x0045)

void sub_E7F0(Regs *r);
void sub_C234(Regs *r);
void sub_D199(Regs *r);
void sub_C3E5(Regs *r);
void sub_D08A(Regs *r);
void scene_assemble(Regs *r);
void sub_C5CB(Regs *r);
void sub_D07C(Regs *r);
void sub_C1C7(Regs *r);
void sub_C1D8(Regs *r);
void sub_C492(Regs *r);

void sub_D596(Regs *r)
{
    u8 y = equipped_item;
    u8 x = RAM8((u16)(carried_item0 + y));

    if (x >= 0x02) {                         /* L_D5BC */
        if (x == 0x0B) {
            if (magic != 0)
                return;
            /* L_D5C5 */
            x = equipped_item;
            RAM8((u16)(carried_item0 + x)) = 0xFF;
            sub_C234(r);
            sub_D199(r);
            return;
        }
        /* L_D5D2 */
        if (x != 0x0D)
            return;
        /* L_D5D7 */
        if (map_screen_y >= 0x11) {
            equipped_item = 0x03;
            return;
        }
        /* L_D5E2 */
        x = equipped_item;
        RAM8((u16)(carried_item0 + x)) = 0xFF;
        sub_C234(r);
        RAM8(0x8F) = 0x12;
        /* JMP L_D866: set fixed warp coords, fall into L_D895 */
        map_screen_y = 0x10;
        map_screen_x = 0x03;
        scroll_x_tile = 0x12;
        player_y = 0xB0;
        player_x_tile = 0x1A;
        player_x_fine = 0x00;
        scroll_x_fine = 0x00;
        /* L_D895 */
        sub_C3E5(r);
        sub_D08A(r);
        scene_assemble(r);
        sub_C5CB(r);
        sub_D07C(r);
        sub_C1C7(r);
        sub_C1D8(r);
        sub_C492(r);
        r->c = 1;                            /* SEC */
        return;
    }

    /* x < 2 */
    if (RAM8((u16)(0x86 + x)) != 0)          /* BEQ L_D5A3 else RTS */
        return;
    /* L_D5A3 */
    r->x = x;
    sub_E7F0(r);
    if (r->c == 0) {                         /* BCC L_D5B7: had magic */
        RAM8((u16)(0x86 + x)) = 0x02;
        return;
    }
    /* no magic: check $37 */
    {
        u8 t = RAM8(0x37);
        if (t == 0 || (t & 0x80))            /* BEQ / BMI L_D5B6 */
            return;
        RAM8(0x37) = 0xFD;
        RAM8(0x8F) = 0x1A;
    }
}
