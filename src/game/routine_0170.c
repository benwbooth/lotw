







#include "game_memory.h"
#include "routine_context.h"

void routine_0170(RoutineContext *r)
{
    u8 a = GAME_MEM8(0x0B);
    u8 y;

    if (a >= 0x0C) {
        a = (u8)(a - 0x0C);
        GAME_MEM8(0x0F)++;
    }
    y = a;
    if (y != 0) {
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x0A) + 0x10);
    }
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A) & 0xF0;
    GAME_MEM8(0xFC) = 0x00;
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xF9) = 0x00;
    r->value = 0x00;
    r->offset = y;
}
