














#include "game_memory.h"
#include "routine_context.h"

void routine_0060(RoutineContext *r);

#define scroll_x_fine GAME_MEM8(0x7B)
#define scroll_x_tile GAME_MEM8(0x7C)
#define player_x_fine GAME_MEM8(0x43)
#define player_x_tile GAME_MEM8(0x44)

void routine_0059(RoutineContext *r)
{
    u8 scrollpos = (u8)((scroll_x_tile << 4) | scroll_x_fine);
    u8 playerpos = (u8)((player_x_tile << 4) | player_x_fine);
    u8 delta = (u8)(playerpos - scrollpos);
    int out_carry;

    GAME_MEM8(0x08) = scrollpos;

    if (delta < 0x60) {

        if ((scroll_x_tile | scroll_x_fine) == 0) {
            out_carry = 1;
        } else {
            u8 t = (u8)(player_x_tile - 0x06);
            if (player_x_tile < 0x06) {
                scroll_x_fine = 0x00;
                scroll_x_tile = 0x00;
                out_carry = 0;
            } else {
                scroll_x_tile = t;
                scroll_x_fine = player_x_fine;
                GAME_MEM8(0x7F) = 0xFF;
                out_carry = 0;
            }
        }
    } else if (delta < 0x91) {
        out_carry = 1;
    } else {

        if (scroll_x_tile >= 0x30) {

            scroll_x_tile = 0x30;
            scroll_x_fine = 0x00;
            out_carry = 1;
        } else {
            scroll_x_tile = (u8)(player_x_tile - 0x09);
            scroll_x_fine = player_x_fine;
            GAME_MEM8(0x7F) = 0x01;
            out_carry = 0;
        }
    }

    routine_0060(r);
    r->carry = (u8)out_carry;
}
