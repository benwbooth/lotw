







#include "game_memory.h"
#include "routine_context.h"

#define player_x_fine GAME_MEM8(0x43)
#define player_x_tile GAME_MEM8(0x44)
#define player_y      GAME_MEM8(0x45)

void routine_0114(RoutineContext *r)
{
    u8 dy, dx;
    GAME_MEM8(0xEA) = 0x00;

    dy = (u8)(GAME_MEM8(0x0A) - player_y);

    if (dy >= 0x10 && dy < 0xE1) {
        r->carry = 0;
        return;
    }

    dx = (u8)(GAME_MEM8(0x0F) - player_x_tile);
    if (dx == 0)    goto set;
    if (dx == 0xFF) goto set;
    if (dx < 0x02) {
        u8 f = (u8)(GAME_MEM8(0x0E) - player_x_fine);
        if (f & 0x80) goto set;
        r->carry = 0;
        return;
    }
    if (dx < 0xFE) { r->carry = 0; return; }

    {
        u8 f = (u8)(GAME_MEM8(0x0E) - player_x_fine);
        if (f == 0)   { r->carry = 0; return; }
        if (f & 0x80) { r->carry = 0; return; }
        goto set;
    }

set:
    GAME_MEM8(0xEA) = 0x01;
    r->carry = 1;
}
