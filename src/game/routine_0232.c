














#include "game_memory.h"
#include "routine_context.h"

#define player_x_tile GAME_MEM8(0x44)
#define player_y      GAME_MEM8(0x45)

void routine_0112(RoutineContext *r);
void routine_0113(RoutineContext *r);

void routine_0232(RoutineContext *r)
{
    u8 x;

    GAME_MEM8(0x0F) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0E) = GAME_MEM8(0xF9);
    GAME_MEM8(0x0A) = GAME_MEM8(0xFB);

    routine_0112(r);
    x = 0x00;
    if (r->carry == 0) {
        u8 d = (u8)(GAME_MEM8(0xFA) - player_x_tile);
        u8 carry = (GAME_MEM8(0xFA) >= player_x_tile) ? 1 : 0;
        x = 0x01;
        if (carry) x = 0x02;
        (void)d;
    }
    GAME_MEM8(0xF4) = x;

    routine_0113(r);
    x = 0x00;
    if (r->carry == 0) {
        u8 carry = (GAME_MEM8(0xFB) >= player_y) ? 1 : 0;
        x = 0x04;
        if (carry) x = 0x08;
    }
    GAME_MEM8(0xF4) = (u8)(x | GAME_MEM8(0xF4));

    GAME_MEM8(0xF3) = 0x00;
}
