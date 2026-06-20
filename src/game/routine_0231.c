










#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

#define player_x_tile GAME_MEM8(0x0044)
#define player_y      GAME_MEM8(0x0045)

void routine_0231(RoutineContext *r)
{
    u8 x = 0x00;
    u16 dx = (u16)((u16)GAME_MEM8(0xFA) - player_x_tile);
    if ((u8)dx != 0) {
        ++x;
        if (!(dx & 0x100))
            ++x;
    }
    GAME_MEM8(0xF4) = x;

    {
        u16 dy = (u16)((u16)GAME_MEM8(0xFB) - player_y);
        if (!(dy & 0x100)) {
            u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
            u8 flag = GAME_MEM8((u16)(ptr + 0x09));
            if (flag != 0) {
                r->value = 0x03;
                rng_update(r);
                r->index = r->value;
                if (r->index == 0)
                    GAME_MEM8(0xF4) = (u8)(GAME_MEM8(0xF4) | 0x80);
            }
        } else {
            r->value = 0x03;
            rng_update(r);
            r->index = r->value;
            if (r->index == 0)
                GAME_MEM8(0xF4) = 0x04;
        }
    }
}
