




#include "game_memory.h"
#include "routine_context.h"

#define player_x_fine GAME_MEM8(0x43)
#define player_x_tile GAME_MEM8(0x44)

void routine_0112(RoutineContext *r)
{
    u8 d = (u8)(GAME_MEM8(0x0F) - player_x_tile);

    if (d == 0) { r->carry = 1; return; }
    if (d < 0x02) {
        u8 f = (u8)(GAME_MEM8(0x0E) - player_x_fine);
        r->carry = (f & 0x80) ? 1 : 0;
        return;
    }
    if (d < 0xFF) { r->carry = 0; return; }

    {
        u8 f = (u8)(GAME_MEM8(0x0E) - player_x_fine);
        if (f == 0)        { r->carry = 0; return; }
        if (f & 0x80)      { r->carry = 0; return; }
        r->carry = 1;
    }
}
